use std::str::Chars;

use peekmore::{PeekMore, PeekMoreIterator};
use smallvec::SmallVec;

/// Formats a type description string to have nice indents.
pub fn format_type_description(input: &str) -> String {
    /// Big scope means we want to spread out items over multiple lines.
    /// Small scope means, we want to keep it compact (on one line).
    #[derive(Debug, Clone, PartialEq)]
    enum Scope {
        Big,
        Small,
    }

    const SMALL_SCOPE_MAX_TOKENS: usize = 32;
    /// should be called on the chars iterator shortly after open_token was encountered.
    ///
    /// scope should never be considered small if any curly braces are encountered inbetween.
    fn scope_is_small(
        chars: &mut PeekMoreIterator<Chars>,
        open_token: char,
        close_token: char,
    ) -> bool {
        // 1 because starting assuming bracket is open
        let mut open_close_balance = 1;
        for ch in chars.peek_amount(SMALL_SCOPE_MAX_TOKENS) {
            let Some(ch) = ch else {
                break;
            };
            if *ch == open_token {
                open_close_balance += 1;
            }
            if *ch == close_token {
                open_close_balance -= 1;
                if open_close_balance == 0 {
                    return true;
                }
            }
            if *ch == '{' {
                return false;
            }
        }
        false
    }

    fn add_indentation(output: &mut String, indent_level: i32) {
        for _ in 0..indent_level {
            output.push_str("    ");
        }
    }

    let mut output = String::new();
    let mut indent_level: i32 = 0;

    let mut tuple_level: SmallVec<[Scope; 8]> = SmallVec::new();
    let mut angle_level: SmallVec<[Scope; 8]> = SmallVec::new();

    let mut chars_peekable = input.chars().peekmore();

    while let Some(ch) = chars_peekable.next() {
        match ch {
            '{' => {
                indent_level += 1;
                output.push(' ');
                output.push(ch);
                output.push('\n');
                add_indentation(&mut output, indent_level);
            }
            '}' => {
                indent_level -= 1;
                output.push('\n');
                add_indentation(&mut output, indent_level);
                output.push(ch);
            }
            ',' => {
                output.push(ch);

                if tuple_level.last() == Some(&Scope::Small) {
                    output.push(' ');
                } else {
                    output.push('\n');
                    add_indentation(&mut output, indent_level);
                }
            }
            '(' => {
                output.push(ch);

                if scope_is_small(&mut chars_peekable, '(', ')') {
                    tuple_level.push(Scope::Small)
                } else {
                    tuple_level.push(Scope::Big);
                    indent_level += 1;
                    output.push('\n');
                    add_indentation(&mut output, indent_level);
                }
            }
            ')' => {
                if let Some(Scope::Big) = tuple_level.pop() {
                    indent_level -= 1;
                    output.push('\n');
                    add_indentation(&mut output, indent_level);
                }
                output.push(ch);
            }
            '<' => {
                output.push(ch);
                if scope_is_small(&mut chars_peekable, '<', '>') {
                    angle_level.push(Scope::Small)
                } else {
                    angle_level.push(Scope::Big);
                    indent_level += 1;
                    output.push('\n');
                    add_indentation(&mut output, indent_level);
                }
            }
            '>' => {
                if let Some(Scope::Big) = angle_level.pop() {
                    indent_level -= 1;
                    output.push('\n');
                    add_indentation(&mut output, indent_level);
                }

                output.push(ch);
            }
            _ => {
                output.push(ch);
            }
        }
    }
    output
}
