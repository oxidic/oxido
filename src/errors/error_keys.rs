pub fn syntax_error_message(expectation: &str) -> String {
    format!("Expected {expectation} here")
}
pub static SYNTAX_ERROR_CODE: i32 = 1;
