use std::{collections::HashMap, ops::Range, process};

use crate::{
    ast::{Ast, AstNode, Expression},
    data::{Data, Function, Variable},
    error::error,
    standardlibrary::StandardLibrary,
    token::Token,
};

pub struct Interpreter<'a> {
    name: &'a str,
    file: &'a str,
    stop: bool,
    returned: Option<Data>,
    variables: HashMap<String, Variable>,
    functions: HashMap<String, Function>,
}

impl<'a> Interpreter<'a> {
    pub fn new(name: &'a str, file: &'a str) -> Self {
        Self {
            name,
            file,
            stop: false,
            returned: None,
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn run(&mut self, ast: Ast) {
        let mut stream = ast.into_iter().peekable();
        loop {
            if stream.peek().is_none() {
                break;
            }

            self.match_node(stream.next().unwrap());
        }
    }

    pub fn match_node(&mut self, node: (AstNode, Range<usize>)) {
        if self.stop || self.returned.is_some() {
            return;
        }
        match node.0 {
            AstNode::Assignment(ident, datatype, expression) => {
                let data = self.parse_expression(expression, &node.1);
                self.variables.insert(ident, Variable::new(datatype, data));
            }
            AstNode::ReAssignment(ident, expression) => {
                if !self.variables.contains_key(&ident) {
                    error(
                        self.name,
                        self.file,
                        "0005",
                        "token was not expected here",
                        "unexpected token",
                        &node.1,
                    );
                }
                let datatype = self.variables.get(&ident).unwrap().datatype;
                let data = self.parse_expression(expression, &node.1);
                if datatype != data.r#type() {
                    error(
                        self.name,
                        self.file,
                        "E00011",
                        "incorrect data type",
                        &format!(
                            "mismatched data types expected {} found {}",
                            datatype,
                            data.to_string()
                        ),
                        &node.1,
                    )
                }
                self.variables.insert(ident, Variable::new(datatype, data));
            }
            AstNode::If(condition, statements) => {
                let data = self.parse_expression(condition, &node.1);

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
                            data.to_string()
                        ),
                        "a value of type `bool` was expected",
                        &node.1,
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
                    args.push(self.parse_expression(param, &node.1))
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
                            &node.1,
                        );
                    }

                    for (i, arg) in args.iter().enumerate() {
                        let param = function.params.get(i).unwrap();

                        if param.datatype != arg.r#type() {
                            error(
                                self.name,
                                self.file,
                                "E00011",
                                "incorrect data type",
                                &format!(
                                    "mismatched data types expected {} found {}",
                                    param.datatype,
                                    arg.to_string()
                                ),
                                &node.1,
                            )
                        }

                        self.variables.insert(
                            param.name.clone(),
                            Variable::new(param.datatype, arg.to_owned()),
                        );
                    }

                    let mut stream = function.statements.into_iter().peekable();

                    loop {
                        if stream.peek().is_none() {
                            break;
                        }

                        self.match_node(stream.next().unwrap());

                        if self.returned.is_some() {
                            self.returned = None;
                            break;
                        }
                    }
                } else {
                    error(
                        self.name,
                        self.file,
                        "0004",
                        "function does not exist",
                        "tried to call a function which does not exist",
                        &node.1,
                    );
                }
            }
            AstNode::FunctionDeclaration(name, params, datatype, statements) => {
                self.functions
                    .insert(name.clone(), Function::new(name, params, datatype, statements));
            }
            AstNode::Break => {
                self.stop = true;
            }
            AstNode::Return(expr) => self.returned = Some(self.parse_expression(expr, &node.1)),
            AstNode::Exit(expr) => {
                let data = self.parse_expression(expr, &node.1);

                match data {
                    Data::Int(n) => process::exit(n.try_into().unwrap()),
                    _ => {
                        error(
                            self.name,
                            self.file,
                            "0002",
                            &format!(
                                "mismatched data types, expected `int` found {}",
                                data.to_string()
                            ),
                            "a value of type `String` was expected",
                            &node.1,
                        );
                    }
                };
            }
        }
    }

    pub fn parse_function(&mut self, f: String, args: Vec<Expression>, pos: &Range<usize>) -> Data {
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
                    pos,
                ),
            };
        }
        let function = self.functions.get(&*f).unwrap().to_owned();

        if function.datatype.is_none() {
            error(
                self.name,
                self.file,
                "0004",
                "function does not return a value",
                "function does not a return a value",
                pos,
            );
        }

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
                pos,
            );
        }

        for (i, arg) in args.iter().enumerate() {
            let param = function.params.get(i).unwrap();

            let data = self.parse_expression(arg.to_owned(), pos);

            if param.datatype != data.r#type() {
                error(
                    self.name,
                    self.file,
                    "E00011",
                    &format!(
                        "mismatched data types expected {} found {}",
                        param.datatype,
                        data.to_string()
                    ),
                    "incorrect data type",
                    pos,
                )
            }

            self.variables
                .insert(param.name.clone(), Variable::new(param.datatype, data));
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
                    pos,
                );
            }

            self.match_node(stream.next().unwrap());

            if self.returned.is_some() {
                let data = self.returned.clone().unwrap();
                self.returned = None;

                if data.r#type() != function.datatype.unwrap() {
                    error(
                        self.name,
                        self.file,
                        "0004",
                        &format!(
                            "mismatched data types expected {} found {}",
                            function.datatype.unwrap(),
                            data.to_string()
                        ),
                        "incorrect data type",
                        pos,
                    );
                }
                break data;
            }
        }
    }

    pub fn parse_expression(&mut self, expr: Expression, pos: &Range<usize>) -> Data {
        match expr {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Int(i) => Data::Int(i),
            Expression::Identifier(i) => self.variables.get(&*i).unwrap().to_owned().data,
            Expression::Bool(b) => Data::Bool(b),
            Expression::Str(s) => Data::Str(s),
            Expression::FunctionCall(f, args) => self.parse_function(f, args, pos),
        }
    }

    pub fn parse_binary_operation(
        &mut self,
        lhs: Expression,
        op: Token,
        rhs: Expression,
        pos: &Range<usize>,
    ) -> Data {
        let lhs = self.parse_expression(lhs, pos);
        let operator = op;
        let rhs = self.parse_expression(rhs, pos);
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
                            data.to_string()
                        ),
                        "a value of type `String` was expected",
                        pos,
                    ),
                },
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Int(n + m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `String` or `int` found {}",
                        data.to_string()
                    ),
                    "a value of type `String` or `int` was expected",
                    pos,
                ),
            },
            Token::Subtraction => match lhs {
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Int(n - m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.to_string()
                    ),
                    "a value of type `int` was expected",
                    pos,
                ),
            },
            Token::Multiplication => match lhs {
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Int(n * m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.to_string()
                    ),
                    "a value of type `int` was expected",
                    pos,
                ),
            },
            Token::Division => match lhs {
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Int(n / m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.to_string()
                    ),
                    "a value of type `int` was expected",
                    pos,
                ),
            },
            Token::Power => match lhs {
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Int(n.pow(m as u32)),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
                    ),
                },
                data => error(
                    self.name,
                    self.file,
                    "0002",
                    &format!(
                        "mismatched data types, expected `int` found {}",
                        data.to_string()
                    ),
                    "a value of type `int` was expected",
                    pos,
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
                            data.to_string()
                        ),
                        "a value of type `String` was expected",
                        pos,
                    ),
                },
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Bool(n == m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `bool` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `String` was expected",
                        pos,
                    ),
                },
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Bool(n != m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `bool` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `String` was expected",
                        pos,
                    ),
                },
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Bool(n > m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `bool` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `String` was expected",
                        pos,
                    ),
                },
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Bool(n < m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `bool` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `String` was expected",
                        pos,
                    ),
                },
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Bool(n >= m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `bool` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `String` was expected",
                        pos,
                    ),
                },
                Data::Int(n) => match rhs {
                    Data::Int(m) => Data::Bool(n <= m),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `int` found {}",
                            data.to_string()
                        ),
                        "a value of type `int` was expected",
                        pos,
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
                            data.to_string()
                        ),
                        "a value of type `bool` was expected",
                        pos,
                    ),
                },
            },
            _ => unreachable!(),
        }
    }
}
