use crate::{token::Token, Error};

pub fn check_syntax(file: &str, line: &str, expectation: Token, reality: Token) {
    if expectation != reality {
        Error::throw(
            file,
            line,
            SYNTAX_ERROR_CODE,
            &format!("Expected {expectation} here"),
            true,
        );
    }
}
pub static SYNTAX_ERROR_CODE: i32 = 1;
