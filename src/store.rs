use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

/// # Scopes
/// - `_loop` Stores whether the loop is capturing input in number
/// - `_if` Stores if scope, if more than 0, skip, else run
/// - `_function` Stores which function is capturing input
/// - `_looping` Stores whether the loop has started running and not capturing input anymore
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
/// The Store acts as the _Storage_ for the interpreter
///
/// - `at` The line at which the interpreter is running
/// - `text` The at text of line at which the interpreter is running
/// - `file_name` The name of the file
/// - `total` The number of lines in file
/// - `lines` A vector of all lines
/// - `scopes` The scopes object storing various values
/// - `loop_line` The line at which the loop is running at
/// - `stack` Stores the lines of loop
/// - `bracket_stack` Stores the scopes of IF LOOP and FUNCTION
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
