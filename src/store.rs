use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeManager {
    pub _loop: i128,
    pub _if: i128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub arguments: Vec<String>,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineManager {
    pub at: i128,
    pub text: String,
    pub total: i128,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateManager {
    pub stack: Vec<String>,
    pub loops: LoopState,
    pub functions: FunctionState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopState {
    pub loop_line: i128,
    pub stack: Vec<String>,
    pub looping: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionState {
    pub capturing: bool,
    pub current: String,
    pub args: HashMap<String, String>,
    pub running: bool,
}

/// # Store
///
/// The Store acts as the Storage for the interpreter
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub file_name: String,
    pub variables: HashMap<String, String>,
    pub functions: HashMap<String, Function>,
    pub lines: LineManager,
    pub scopes: ScopeManager,
    pub states: StateManager,
}

impl Store {
    pub fn new(file_name: String, lines: Vec<String>) -> Self {
        Store {
            file_name,
            variables: HashMap::new(),
            functions: HashMap::new(),
            lines: LineManager {
                at: 0,
                text: String::new(),
                total: lines.len().try_into().unwrap(),
                lines,
            },
            scopes: ScopeManager { _loop: 0, _if: 0 },
            states: StateManager {
                stack: vec![],
                loops: LoopState {
                    loop_line: 0,
                    stack: vec![],
                    looping: false,
                },
                functions: FunctionState {
                    capturing: false,
                    current: String::new(),
                    args: HashMap::new(),
                    running: false,
                },
            },
        }
    }

    pub fn increment_line(&mut self, line: String) {
        self.lines.at += 1;
        self.lines.text = line;
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#?}", self)
    }
}

impl Display for ScopeManager {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#?}", self)
    }
}
