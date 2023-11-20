use crate::transformer::Transformer;
use anyhow::anyhow;
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use scale_info::{form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeDefPrimitive};
use scale_typegen::{TypeGenerator, TypeGeneratorSettings};
use std::{borrow::BorrowMut, cell::RefCell};

type CodeTransformer<'a> = Transformer<'a, TokenStream, CodeTransformerState<'a>>;

struct CodeTransformerState<'a> {
    rng: RefCell<rand_chacha::ChaCha8Rng>,
    type_generator: TypeGenerator<'a>,
    /// `ty_middleware` allows you to return a different value on a case by case basis.
    /// For example, when a type with that looks like our AccountId type is seen,
    /// instead of `AccountId([255,203,130,..])` a middleware could return just `dev::alice`.
    ty_middleware: Option<TyMiddleware>,
    /// `ty_path_middleware` can be used to convert a type path like `::std::vec::Vec<T>` into just `Vec<T>`.
    ty_path_middleware: Option<TyPathMiddleware>,
}

type TyMiddleware = Box<dyn Fn(&Type<PortableForm>) -> Option<anyhow::Result<TokenStream>>>;

type TyPathMiddleware = Box<dyn Fn(TokenStream) -> TokenStream>;

impl<'a> CodeTransformer<'a> {
    /// resolves a type path, removes the generic bits, e.g. `Foo<T, R>` becomes `Foo`,
    /// and, if the correct ty_path_middleware is set, prunes the resulting type path.
    fn resolve_type_path_omit_generics(&self, type_id: u32) -> anyhow::Result<TokenStream> {
        let mut type_path = self
            .state()
            .type_generator
            .resolve_type_path(type_id)
            .map_err(|e| anyhow!("{e}"))?
            .to_token_stream();

        // apply ty path middleware pruning/replacing paths:
        if let Some(ty_path_middleware) = &self.state().ty_path_middleware {
            type_path = ty_path_middleware(type_path);
        };

        // omit generics:
        return Ok(omit_generics(type_path));

        fn omit_generics(input: TokenStream) -> TokenStream {
            input
                .into_iter()
                .take_while(|t| match t {
                    TokenTree::Punct(p) => p.as_char() != '<',
                    _ => true,
                })
                .collect()
        }
    }

    fn has_unused_type_params(&self, ty: &Type<PortableForm>) -> anyhow::Result<bool> {
        let has_unused_type_params = self
            .state
            .type_generator
            .create_type_ir(ty)
            .map_err(|e| anyhow!("{e}"))?
            .map(|e| e.type_params.has_unused_type_params())
            .unwrap_or(false);
        Ok(has_unused_type_params)
    }

    fn resolve_type(&self, type_id: u32) -> anyhow::Result<&Type<PortableForm>> {
        self.state
            .type_generator
            .resolve_type(type_id)
            .map_err(|e| anyhow!("{e}"))
    }

    /// Simple Heuristics. Just makes array initialization shorter if is `Copy`.
    fn type_def_is_copy(&self, ty: &TypeDef<PortableForm>) -> anyhow::Result<bool> {
        let tf = match ty {
            TypeDef::Primitive(def) => !matches!(def, TypeDefPrimitive::Str),
            scale_info::TypeDef::Array(def) => {
                let item_type = self.resolve_type(def.type_param.id)?;
                def.len <= 32 && self.type_def_is_copy(&item_type.type_def)?
            }
            scale_info::TypeDef::Tuple(def) => {
                for f in def.fields.iter() {
                    let ty = self.resolve_type(f.id)?;
                    if !self.type_def_is_copy(&ty.type_def)? {
                        return Ok(false);
                    }
                }
                true
            }

            scale_info::TypeDef::Compact(def) => {
                let ty = self.resolve_type(def.type_param.id)?;
                self.type_def_is_copy(&ty.type_def)?
            }
            _ => false,
        };
        Ok(tf)
    }
}

pub fn rust_type_example(
    type_id: u32,
    types: &PortableRegistry,
    settings_for_path_resolver: TypeGeneratorSettings,
) -> anyhow::Result<TokenStream> {
    rust_type_example_from_seed(type_id, types, settings_for_path_resolver, 42, None, None)
}

pub fn rust_type_example_from_seed(
    type_id: u32,
    types: &PortableRegistry,
    settings_for_path_resolver: TypeGeneratorSettings,
    seed: u64,
    ty_middleware: Option<TyMiddleware>,
    ty_path_middleware: Option<TyPathMiddleware>,
) -> anyhow::Result<TokenStream> {
    fn error_on_recurse(
        _type_id: u32,
        ty: &Type<PortableForm>,
        _transformer: &CodeTransformer,
    ) -> anyhow::Result<TokenStream> {
        Err(anyhow!(
            "Cannot generate rust type example for recursive type: {ty:?}"
        ))
    }

    let state = CodeTransformerState {
        rng: RefCell::new(rand_chacha::ChaCha8Rng::seed_from_u64(seed)),
        type_generator: TypeGenerator::new(types, &settings_for_path_resolver),
        ty_middleware,
        ty_path_middleware,
    };

    let transformer = CodeTransformer::new(ty_example, error_on_recurse, state, types);
    transformer.resolve(type_id)
}

fn ty_example(
    type_id: u32,
    ty: &Type<PortableForm>,
    transformer: &CodeTransformer,
) -> anyhow::Result<TokenStream> {
    // if middleware wants to intersect and return something else, it can:
    if let Some(middleware) = &transformer.state.ty_middleware {
        if let Some(intersected) = middleware(ty) {
            return intersected;
        }
    }

    //  general handling of type definitions
    match &ty.type_def {
        scale_info::TypeDef::Composite(composite) => {
            let struct_path = transformer.resolve_type_path_omit_generics(type_id)?;
            let has_unused_type_params = transformer.has_unused_type_params(ty)?;

            let fields: TokenStream =
                fields_example(&composite.fields, has_unused_type_params, transformer)?;
            Ok(quote!(#struct_path #fields))
        }
        scale_info::TypeDef::Variant(variant) => {
            let enum_path = transformer.resolve_type_path_omit_generics(type_id)?;
            let random_variant = variant
                .variants
                .choose(&mut *transformer.state.rng.borrow_mut())
                .ok_or_else(|| anyhow!("Variant type should have at least one variant"))?;
            let variant_ident = format_ident!("{}", &random_variant.name);
            // never needs phantom data, because phantom data is generated as a separate variant.
            let fields = fields_example(&random_variant.fields, false, transformer)?;
            let mut example = quote!(#enum_path::#variant_ident #fields);

            // Note: this makes it such that Option::None is just shown as None.
            if example.to_string() == "Option :: None" {
                example = quote!(None);
            };
            Ok(example)
        }
        scale_info::TypeDef::Sequence(def) => {
            // return a Vec with 2 elements:
            let inner_ty = transformer.resolve_type(def.type_param.id)?;
            let item_code = ty_example(def.type_param.id, inner_ty, transformer)?; // todo!("Might need CompactMode::Expl")
            let vec_code = quote!(vec![#item_code, #item_code, #item_code]);
            Ok(vec_code)
        }
        scale_info::TypeDef::Array(def) => {
            let inner_ty = transformer.resolve_type(def.type_param.id)?;
            let item_code = ty_example(def.type_param.id, inner_ty, transformer)?; //todo!("Might need CompactMode::Expl")
            let inner_is_copy = transformer.type_def_is_copy(&inner_ty.type_def)?;
            let len = def.len as usize;
            let arr_code = if inner_is_copy {
                // if the item_code is an expression that is `Copy` we can use short init syntax:
                quote!([#item_code;#len])
            } else {
                // otherwise we need to duplicate the item_code `len` times:
                let item_iter = (0..len).map(|_| &item_code);
                quote!([#(#item_iter),*])
            };
            Ok(arr_code)
        }
        scale_info::TypeDef::Tuple(def) => {
            let mut fields: Vec<TokenStream> = vec![];
            for f in &def.fields {
                let value = transformer.resolve(f.id)?; //todo!("Might need CompactMode::Expl")
                fields.push(value)
            }
            Ok(quote!(( #(#fields),* )))
        }
        scale_info::TypeDef::Primitive(def) => Ok(primitive_example(
            def,
            &mut *transformer.state.rng.borrow_mut(),
        )),
        scale_info::TypeDef::Compact(def) => {
            // there are actually two possibilities here:
            // 1. the value is not actually compact but just tagged with { #[codec(compact)] number: u8 } in the type definition.
            // --> give a normal primitive as a type example, e.g. 8
            // 2. the value is actually like (Compact<u8>, String) in the type definition.
            // --> give compact type example, e.g. Compact(8)

            // How to find out? In structs, we are gonna be in case 1, otherwise (inside a tuple, array or vec) where the #[codec(compact)] is not possible, we are in case 2.
            // `explicit_compact` flag is used to indicate we are in case 2.

            let inner_code = transformer.resolve(def.type_param.id)?; //todo!("Might need CompactMode::Expl")

            // I used this originally, but it turns out the compact part should be omitted:

            // todo!("Revisit this code and figure out a better way to handle compact types")
            // let code = match compact_mode {
            //     CompactMode::Expl => {
            //         let compact_path = resolve_type_path_omit_generics(type_gen, id);
            //         quote!(#compact_path(#inner_code))
            //     }
            //     CompactMode::Attr => inner_code,
            // };
            let code = inner_code;

            Ok(code)
        }
        scale_info::TypeDef::BitSequence(_def) => {
            Ok(quote!(subxt::utils::bits::DecodedBits::from_iter([
                true, false, false
            ])))
        }
    }
}

fn fields_example(
    fields: &[Field<PortableForm>],
    needs_phantom_data: bool,
    transformer: &CodeTransformer,
) -> anyhow::Result<TokenStream> {
    let all_named = fields.iter().all(|f| f.name.is_some());
    let all_unnamed = fields.iter().all(|f| f.name.is_none());
    match (all_named, all_unnamed) {
        (true, false) => {
            // all fields named
            let mut field_idents_and_values: Vec<TokenStream> = vec![];
            for f in fields {
                let name = f.name.as_ref().expect("safe because of check above; qed");
                let ident = format_ident!("{name}");
                let value_code = transformer.resolve(f.ty.id)?; // todo!("Check if Compact attribute needed")
                field_idents_and_values.push(quote!(#ident : #value_code));
            }
            // maybe add phantom data to struct / named composite enum
            let maybe_phantom = if needs_phantom_data {
                quote!( __subxt_unused_type_params: ::core::marker::PhantomData )
            } else {
                quote!()
            };
            Ok(quote!({ #(#field_idents_and_values ,)* #maybe_phantom }))
        }
        (false, true) => {
            // all fields unnamed
            let mut field_values: Vec<TokenStream> = vec![];
            for f in fields {
                let value_code = transformer.resolve(f.ty.id)?; // todo!("Check if Compact attribute needed")
                field_values.push(value_code);
            }
            // maybe add phantom data to struct / named composite enum
            let maybe_phantom = if needs_phantom_data {
                quote!(::core::marker::PhantomData)
            } else {
                quote!()
            };
            Ok(quote!(( #(#field_values ,)* #maybe_phantom )))
        }
        (true, true) => {
            // no fields
            Ok(quote!())
        }
        (false, false) => {
            // mixed fields
            Err(anyhow!("mixed fields in struct def"))
        }
    }
}

fn primitive_example(def: &TypeDefPrimitive, rng: &mut impl rand::Rng) -> TokenStream {
    match def {
        TypeDefPrimitive::Bool => {
            let b: bool = rng.gen();
            quote!(#b)
        }
        TypeDefPrimitive::Char => {
            let c = *['a', 'b', 'c', 'd', 'e', 'f', 'g'].choose(rng).unwrap();
            quote!(#c)
        }
        TypeDefPrimitive::Str => {
            let str = *["Foo", "Bar", "Fizz", "Buzz"].choose(rng).unwrap();
            quote!(#str.into())
        }
        TypeDefPrimitive::U8 => {
            let n = rng.gen::<u8>();
            quote!(#n)
        }
        TypeDefPrimitive::U16 => {
            let _n = rng.gen::<u16>();
            quote!(n)
        }
        TypeDefPrimitive::U32 => {
            let n = rng.gen::<u32>();
            quote!(#n)
        }
        TypeDefPrimitive::U64 => {
            let n = rng.gen::<u64>();
            quote!(#n)
        }
        TypeDefPrimitive::U128 => {
            let n = rng.gen::<u128>();
            quote!(#n)
        }
        TypeDefPrimitive::U256 => {
            let n = rng.gen::<[u8; 32]>().into_iter();
            quote!([#(#n),*])
        }
        TypeDefPrimitive::I8 => {
            let n = rng.gen::<i8>();
            quote!(#n)
        }
        TypeDefPrimitive::I16 => {
            let n = rng.gen::<i16>();
            quote!(#n)
        }
        TypeDefPrimitive::I32 => {
            let n = rng.gen::<i32>();
            quote!(#n)
        }
        TypeDefPrimitive::I64 => {
            let n = rng.gen::<i64>();
            quote!(#n)
        }
        TypeDefPrimitive::I128 => {
            let n = rng.gen::<i128>();
            quote!(#n)
        }
        TypeDefPrimitive::I256 => {
            let n = rng.gen::<[u8; 32]>().into_iter();
            quote!([#(#n),*])
        }
    }
}
