use std::{collections::HashMap, fmt::{Display, Formatter, Error}};

#[derive(Debug, Clone, PartialEq)]
pub struct Store<'a> {
    variables: HashMap<&'a str, String>,
    line_number: i128,
    line_text: String,
}

impl<'a> Store<'a> {
    pub fn new() -> Self {
        Store { variables: HashMap::new(), line_number: 0, line_text: String::new() }
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