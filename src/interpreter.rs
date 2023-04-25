use std::{collections::HashMap, process};

use crate::{
    ast::{AstNode, Expression},
    datatype::{Data, Function},
    error::error,
    standardlibrary::StandardLibrary,
    token::Token,
};

pub struct Interpreter<'a> {
    name: &'a str,
    file: &'a str,
    ast: Vec<(AstNode, usize)>,
    stop: bool,
    returned: Option<Data>,
    variables: HashMap<String, Data>,
    functions: HashMap<String, Function>,
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: Vec<(AstNode, usize)>, name: &'a str, file: &'a str) -> Self {
        Self {
            name,
            file,
            ast,
            stop: false,
            returned: None,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut stream = self.ast.clone().into_iter().peekable();
        loop {
            if stream.peek().is_none() {
                break;
            }

            self.match_node(stream.next().unwrap());
        }
    }

    pub fn match_node(&mut self, node: (AstNode, usize)) {
        if self.stop || self.returned.is_some() {
            return;
        }
        match node.0 {
            AstNode::Assignment(ident, expression) => {
                let data = self.parse_expression(expression, node.1);
                self.variables.insert(ident, data);
            }
            AstNode::ReAssignment(ident, expression) => {
                if !self.variables.contains_key(&*ident) {
                    error(
                        self.name,
                        self.file,
                        "0005",
                        "token was not expected here",
                        "unexpected token",
                        node.1..node.1,
                    );
                }
                let data = self.parse_expression(expression, node.1);
                self.variables.insert(ident, data);
            }
            AstNode::If(condition, statements) => {
                let data = self.parse_expression(condition, node.1);

                if let Data::Bool(bool) = data {
                    if bool {
                        let mut stream = statements.into_iter().peekable();
                        loop {
                            if stream.peek().is_none() {
                                break;
                            }

                            self.match_node(stream.next().unwrap());
                        }
                    }
                } else {
                    error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        node.1..node.1,
                    );
                }
            }
            AstNode::Loop(statements) => {
                let mut stream = statements.clone().into_iter().peekable();
                loop {
                    if stream.peek().is_none() {
                        stream = statements.clone().into_iter().peekable();
                    }

                    if self.stop {
                        self.stop = false;
                        break;
                    }

                    if self.returned.is_some() {
                        break;
                    }

                    self.match_node(stream.next().unwrap());
                }
            }
            AstNode::FunctionCall(name, params) => {
                let mut args = vec![];

                for param in params {
                    args.push(self.parse_expression(param, node.1))
                }

                if StandardLibrary::contains(&name) {
                    StandardLibrary::call(&name, args);
                } else if self.functions.contains_key(&*name) {
                    let function = self.functions.get(&*name).unwrap().to_owned();

                    if args.len() != function.params.len() {
                        error(
                            self.name,
                            self.file,
                            "0004",
                            "not enough arguments were passed",
                            &format!(
                                "{} arguments were expected but {} were passed",
                                function.params.len(),
                                args.len()
                            ),
                            node.1..node.1,
                        );
                    }

                    for (i, arg) in args.iter().enumerate() {
                        let param = function.params.get(i).unwrap();

                        self.variables.insert(param.to_string(), arg.to_owned());
                    }

                    let mut stream = function.statements.into_iter().peekable();
                    loop {
                        if stream.peek().is_none() {
                            break;
                        }

                        if self.returned.is_some() {
                            break;
                        }

                        self.match_node(stream.next().unwrap());
                    }
                }
            }
            AstNode::FunctionDeclaration(name, params, statements) => {
                self.functions
                    .insert(name.clone(), Function::new(name, params, statements));
            }
            AstNode::Break => {
                self.stop = true;
            }
            AstNode::Return(expr) => self.returned = Some(self.parse_expression(expr, node.1)),
            AstNode::Exit => process::exit(0),
        }
    }

    pub fn parse_expression(&mut self, expr: Expression, pos: usize) -> Data {
        match expr {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&*i).unwrap().to_owned(),
            Expression::Bool(b) => Data::Bool(b),
            Expression::Str(s) => Data::Str(s),
            Expression::FunctionCall(f, args) => {
                if StandardLibrary::contains(&f) {
                    return match StandardLibrary::call(
                        &f,
                        args.iter()
                            .map(|f| self.parse_expression(f.clone(), pos))
                            .collect::<Vec<_>>(),
                    ) {
                        Some(data) => data,
                        None => error(
                            self.name,
                            self.file,
                            "0004",
                            "function does not return a value",
                            "function does not a return a value",
                            pos..pos,
                        ),
                    };
                }
                let function = self.functions.get(&*f).unwrap().to_owned();

                if args.len() != function.params.len() {
                    error(
                        self.name,
                        self.file,
                        "0004",
                        "not enough arguments were passed",
                        &format!(
                            "{} arguments were expected but {} were passed",
                            function.params.len(),
                            args.len()
                        ),
                        pos..pos,
                    );
                }

                for (i, arg) in args.iter().enumerate() {
                    let param = function.params.get(i).unwrap();

                    let data = self.parse_expression(arg.to_owned(), pos);

                    self.variables.insert(param.to_string(), data);
                }

                let mut stream = function.statements.into_iter().peekable();
                loop {
                    if stream.peek().is_none() {
                        error(
                            self.name,
                            self.file,
                            "0004",
                            &format!("function {f} did not return a value"),
                            "expected function to return a value",
                            pos..pos,
                        );
                    }

                    self.match_node(stream.next().unwrap());

                    if self.returned.is_some() {
                        let data = self.returned.clone().unwrap();
                        self.returned = None;
                        break data;
                    }
                }
            }
        }
    }

    pub fn parse_binary_operation(
        &mut self,
        lhs: Expression,
        op: Token,
        rhs: Expression,
        pos: usize,
    ) -> Data {
        let lhs = match lhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => {
                if !self.variables.contains_key(&*i) {
                    error(
                        self.name,
                        self.file,
                        "0006",
                        "attempt to access value of undeclared variable",
                        "declare the value of the variable before using it",
                        pos..pos + i.len(),
                    );
                };
                self.variables.get(&*i).unwrap().to_owned()
            }
            Expression::Str(s) => Data::Str(s),
            Expression::Bool(b) => Data::Bool(b),
            Expression::FunctionCall(f, args) => {
                if StandardLibrary::contains(&f) {
                    return match StandardLibrary::call(
                        &f,
                        args.iter()
                            .map(|f| self.parse_expression(f.clone(), pos))
                            .collect::<Vec<_>>(),
                    ) {
                        Some(data) => data,
                        None => error(
                            self.name,
                            self.file,
                            "0004",
                            "function does not return a value",
                            "function does not a return a value",
                            pos..pos,
                        ),
                    };
                }
                let function = self.functions.get(&*f).unwrap().to_owned();

                if args.len() != function.params.len() {
                    error(
                        self.name,
                        self.file,
                        "0004",
                        "not enough arguments were passed",
                        &format!(
                            "{} arguments were expected but {} were passed",
                            function.params.len(),
                            args.len()
                        ),
                        pos..pos,
                    );
                }

                for (i, arg) in args.iter().enumerate() {
                    let param = function.params.get(i).unwrap();

                    let data = self.parse_expression(arg.to_owned(), pos);

                    self.variables.insert(param.to_string(), data);
                }

                let mut stream = function.statements.into_iter().peekable();

                loop {
                    if stream.peek().is_none() {
                        error(
                            self.name,
                            self.file,
                            "0004",
                            &format!("function {f} did not return a value"),
                            "expected function to return a value",
                            pos..pos,
                        );
                    }

                    self.match_node(stream.next().unwrap());

                    if self.returned.is_some() {
                        let data = self.returned.clone().unwrap();
                        self.returned = None;
                        break data;
                    }
                }
            }
        };
        let operator = op;
        let rhs = match rhs {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Integer(i) => Data::Integer(i),
            Expression::Identifier(i) => self.variables.get(&*i).unwrap().to_owned(),
            Expression::Str(s) => Data::Str(s),
            Expression::Bool(b) => Data::Bool(b),
            Expression::FunctionCall(f, args) => {
                if StandardLibrary::contains(&f) {
                    return match StandardLibrary::call(
                        &f,
                        args.iter()
                            .map(|f| self.parse_expression(f.clone(), pos))
                            .collect::<Vec<_>>(),
                    ) {
                        Some(data) => data,
                        None => error(
                            self.name,
                            self.file,
                            "0004",
                            "function does not return a value",
                            "function does not a return a value",
                            pos..pos,
                        ),
                    };
                }
                let function = self.functions.get(&*f).unwrap().to_owned();

                if args.len() != function.params.len() {
                    error(
                        self.name,
                        self.file,
                        "0004",
                        "not enough arguments were passed",
                        &format!(
                            "{} arguments were expected but {} were passed",
                            function.params.len(),
                            args.len()
                        ),
                        pos..pos,
                    );
                }

                for (i, arg) in args.iter().enumerate() {
                    let param = function.params.get(i).unwrap();

                    let data = self.parse_expression(arg.to_owned(), pos);

                    self.variables.insert(param.to_string(), data);
                }

                let mut stream = function.statements.into_iter().peekable();
                loop {
                    if stream.peek().is_none() {
                        error(
                            self.name,
                            self.file,
                            "0004",
                            &format!("function {f} did not return a value"),
                            "expected function to return a value",
                            pos..pos,
                        );
                    }

                    self.match_node(stream.next().unwrap());

                    if self.returned.is_some() {
                        let data = self.returned.clone().unwrap();
                        self.returned = None;
                        break data;
                    }
                }
            }
        };
        match operator {
            Token::Addition => match lhs {
                Data::Str(str) => match rhs {
                    Data::Str(s) => Data::Str(str + &s),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n + m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `String` or `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `String` or `int` was expected",
                    pos..pos,
                ),
            },
            Token::Subtraction => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n - m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::Multiplication => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n * m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::Division => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n / m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::Power => match lhs {
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Integer(n.pow(m as u32)),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.type_as_str()
                    ),
                    "a value of type `int` was expected",
                    pos..pos,
                ),
            },
            Token::IsEqual => match lhs {
                Data::Str(str) => match rhs {
                    Data::Str(s) => Data::Bool(str == s),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n == m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b == d),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsNotEqual => match lhs {
                Data::Str(str) => match rhs {
                    Data::Str(s) => Data::Bool(str != s),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n != m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b != d),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsGreater => match lhs {
                Data::Str(str) => match rhs {
                    Data::Str(s) => Data::Bool(str > s),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n > m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b & !d),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsLesser => match lhs {
                Data::Str(str) => match rhs {
                    Data::Str(s) => Data::Bool(str < s),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n < m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(!b & d),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsGreaterEqual => match lhs {
                Data::Str(str) => match rhs {
                    Data::Str(s) => Data::Bool(str >= s),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n >= m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b >= d),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            Token::IsLesserEqual => match lhs {
                Data::Str(str) => match rhs {
                    Data::Str(s) => Data::Bool(str <= s),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `String` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `String` was expected",
                        pos..pos,
                    ),
                },
                Data::Integer(n) => match rhs {
                    Data::Integer(m) => Data::Bool(n <= m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `int` was expected",
                        pos..pos,
                    ),
                },
                Data::Bool(b) => match rhs {
                    Data::Bool(d) => Data::Bool(b <= d),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data.type_as_str()
                        ),
                        "a value of type `bool` was expected",
                        pos..pos,
                    ),
                },
            },
            _ => unreachable!(),
        }
    }
}
