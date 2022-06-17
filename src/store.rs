use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Store<'a> {
    variables: HashMap<&'a str, String>,
    line_number: i128,
    line_text: String,
    file_name: String,
    scope: i128,
    r#loop: i128,
    pub loop_stack: Vec<String>,
    pub bracket_stack: Vec<String>,
}

impl<'a> Store<'a> {
    pub fn new(file_name: String) -> Self {
        Store {
            variables: HashMap::new(),
            line_number: 0,
            line_text: String::new(),
            file_name,
            scope: 0,
            r#loop: 0,
            loop_stack: vec![],
            bracket_stack: vec![],
        }
    }

    pub fn line_text(&self) -> String {
        self.line_text.clone()
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn increment_scope(&mut self) {
        self.scope += 1;
    }

    pub fn decrement_scope(&mut self) {
        self.scope -= 1;
    }

    pub fn get_scope(&self) -> i128 {
        self.scope
    }

    pub fn increment_loop(&mut self) {
        self.r#loop += 1;
    }

    pub fn decrement_loop(&mut self) {
        self.r#loop -= 1;
    }

    pub fn get_loop(&self) -> i128 {
        self.r#loop
    }

    pub fn line_number(&self) -> i128 {
        self.line_number
    }

    pub fn increment_line(&mut self, line: String) -> i128 {
        self.line_number += 1;
        self.line_text = line;

        self.line_number
    }

    pub fn set_variable(&mut self, ident: &'a str, value: String) -> Option<String> {
        self.variables.insert(ident, value)
    }

    pub fn get_variable(&self, ident: &'a str) -> Option<&String> {
        self.variables.get(ident)
    }
}

impl Display for Store<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#?}", self)
    }
}
