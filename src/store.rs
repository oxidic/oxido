use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    variables: HashMap<String, String>,
    pub current_line: usize,
    pub line_text: String,
    pub file_name: String,
    pub total_lines: usize,
    pub lines: Vec<String>,
    pub scope: i128,
    pub r#loop: i128,
    pub is_looping: bool,
    pub loop_line: usize,
    pub loop_stack: Vec<String>,
    pub bracket_stack: Vec<String>,
}

impl Store {
    pub fn new(file_name: String, lines: Vec<String>) -> Self {
        Store {
            variables: HashMap::new(),
            current_line: 0,
            line_text: String::new(),
            file_name,
            total_lines: lines.len(),
            lines,
            is_looping: false,
            scope: 0,
            r#loop: 0,
            loop_line: 0,
            loop_stack: vec![],
            bracket_stack: vec![],
        }
    }

    pub fn increment_line(&mut self, line: String) {
        self.current_line += 1;
        self.line_text = line;
    }

    pub fn set_variable(&mut self, ident: String, value: String) -> Option<String> {
        self.variables.insert(ident, value)
    }

    pub fn get_variable(&self, ident: & str) -> Option<&String> {
        self.variables.get(ident)
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#?}", self)
    }
}
