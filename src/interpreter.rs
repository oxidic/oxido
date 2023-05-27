use std::{collections::HashMap, ops::Range, process};

use crate::{
    ast::{Ast, AstNode, Expression},
    data::{Data, DataType, Function, Variable},
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
    std: StandardLibrary<'a>,
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
            std: StandardLibrary::new(name, file),
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
                let data = self.parse_expression(expression, datatype.clone(), &node.1);
                let datatype = if let Some(d) = datatype {
                    d
                } else {
                    data.r#type()
                };
                if datatype != data.r#type() {
                    error(
                        self.name,
                        self.file,
                        "E00011",
                        "incorrect data type",
                        &format!(
                            "mismatched data types expected {} found {}",
                            datatype,
                            data
                        ),
                        &node.1,
                    )
                }
                self.variables.insert(ident, Variable::new(datatype, data));
            }
            AstNode::ReAssignment(ident, expression) => {
                if !self.variables.contains_key(&ident) {
                    error(
                        self.name,
                        self.file,
                        "0005",
                        "undeclared variable",
                        "attempted to access value of undeclared variable",
                        &node.1,
                    );
                }
                let datatype = self.variables.get(&ident).unwrap().datatype.clone();
                let data = self.parse_expression(expression, Some(datatype.clone()), &node.1);
                if datatype != data.r#type() {
                    error(
                        self.name,
                        self.file,
                        "E00011",
                        "incorrect data type",
                        &format!(
                            "mismatched data types expected {} found {}",
                            datatype,
                            data
                        ),
                        &node.1,
                    )
                }
                self.variables.insert(ident, Variable::new(datatype, data));
            }
            AstNode::VecReAssignment(ident, index, expression) => {
                let data = self.parse_expression(expression, None, &node.1);
                let index = self.parse_expression(index, None, &node.1);
                let mut variable = self.variables.get(&ident).unwrap().clone();

                if let Data::Vector(mut vec, datatype) = variable.data {
                    if let Data::Int(index) = index {
                        if index as usize > vec.len() {
                            error(
                                self.name,
                                self.file,
                                "E0006",
                                "index out of bounds",
                                "index out of bounds",
                                &node.1,
                            );
                        }

                        if datatype != DataType::Vector(Box::new(data.r#type())) {
                            error(
                                self.name,
                                self.file,
                                "E00011",
                                "incorrect data type",
                                &format!(
                                    "mismatched data types expected {} found {}",
                                    datatype,
                                    data
                                ),
                                &node.1,
                            )
                        }

                        if vec.len() == index.try_into().unwrap() {
                            vec.push(data);
                        } else {
                            vec[index as usize] = data;
                        }

                        variable.data = Data::Vector(vec, datatype);

                        self.variables.insert(ident, variable.clone());
                    } else {
                        error(
                            self.name,
                            self.file,
                            "0002",
                            &format!(
                                "mismatched data types, expected `int` found {}",
                                index
                            ),
                            "a value of type `int` was expected",
                            &node.1,
                        );
                    }
                } else {
                    error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            variable.data
                        ),
                        "a value of type `vector` was expected",
                        &node.1,
                    );
                }
            }
            AstNode::If(condition, statements) => {
                let data = self.parse_expression(condition, None, &node.1);

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
                            data
                        ),
                        "a value of type `bool` was expected",
                        &node.1,
                    );
                }
            }
            AstNode::IfElse(condition, then, otherwise) => {
                let data = self.parse_expression(condition, None, &node.1);

                if let Data::Bool(bool) = data {
                    let mut stream = if bool {
                        then.into_iter().peekable()
                    } else {
                        otherwise.into_iter().peekable()
                    };

                    loop {
                        if stream.peek().is_none() {
                            break;
                        }

                        self.match_node(stream.next().unwrap());
                    }
                } else {
                    error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `bool` found {}",
                            data
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
                    args.push(self.parse_expression(param, None, &node.1))
                }

                if self.std.contains(&name) {
                    self.std.call(&name, &node.1, args);
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
                                    arg
                                ),
                                &node.1,
                            )
                        }

                        self.variables.insert(
                            param.name.clone(),
                            Variable::new(param.datatype.clone(), arg.to_owned()),
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
                self.functions.insert(
                    name.clone(),
                    Function::new(name, params, datatype, statements),
                );
            }
            AstNode::Break => {
                self.stop = true;
            }
            AstNode::Return(expr) => {
                self.returned = Some(self.parse_expression(expr, None, &node.1))
            }
            AstNode::Exit(expr) => {
                let data = self.parse_expression(expr, None, &node.1);

                match data {
                    Data::Int(n) => process::exit(n.try_into().unwrap()),
                    _ => {
                        error(
                            self.name,
                            self.file,
                            "0002",
                            &format!(
                                "mismatched data types, expected `int` found {}",
                                data
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
        if self.std.contains(&f) {
            let args = args
                .iter()
                .map(|f| self.parse_expression(f.clone(), None, pos))
                .collect::<Vec<_>>();
            return match self.std.call(&f, pos, args) {
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
        let function = self.functions.get(&*f).unwrap().clone();
        let datatype = &function.datatype;

        if datatype.is_none() {
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

            let data = self.parse_expression(arg.to_owned(), None, pos);

            if param.datatype != data.r#type() {
                error(
                    self.name,
                    self.file,
                    "E00011",
                    &format!(
                        "mismatched data types expected {} found {}",
                        param.datatype,
                        data
                    ),
                    "incorrect data type",
                    pos,
                )
            }

            self.variables.insert(
                param.name.clone(),
                Variable::new(param.datatype.clone(), data),
            );
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

                if data.r#type() != datatype.clone().unwrap() {
                    error(
                        self.name,
                        self.file,
                        "0004",
                        &format!(
                            "mismatched data types expected {} found {}",
                            datatype.clone().unwrap(),
                            data
                        ),
                        "incorrect data type",
                        pos,
                    );
                }
                break data;
            }
        }
    }

    pub fn parse_expression(
        &mut self,
        expr: Expression,
        datatype: Option<DataType>,
        pos: &Range<usize>,
    ) -> Data {
        match expr {
            Expression::BinaryOperation(lhs, op, rhs) => {
                self.parse_binary_operation(*lhs, op, *rhs, pos)
            }
            Expression::Int(i) => Data::Int(i),
            Expression::Identifier(i) => self.variables.get(&*i).unwrap().to_owned().data,
            Expression::Bool(b) => Data::Bool(b),
            Expression::Str(s) => Data::Str(s),
            Expression::FunctionCall(f, args) => self.parse_function(f, args, pos),
            Expression::Vector(vector, d) => {
                let mut data = Vec::new();
                let mut datatype = if d.is_some() {
                    d
                } else {
                    datatype
                };
                for expr in vector {
                    let d = self.parse_expression(expr, None, pos);

                    if datatype.is_none() {
                        datatype = Some(d.r#type());
                    } else if datatype.clone().unwrap() != d.r#type() {
                        error(
                            self.name,
                            self.file,
                            "0004",
                            &format!(
                                "mismatched data types expected {} found {}",
                                datatype.unwrap(),
                                d
                            ),
                            "incorrect data type",
                            pos,
                        );
                    }

                    data.push(d);
                }
                Data::Vector(data, datatype.unwrap())
            }
            Expression::VecIndex(ident, index) => {
                let index = self.parse_expression(*index, None, pos);
                let data = self.variables.get(&ident).unwrap().to_owned().data;

                match data {
                    Data::Vector(vec, _) => match index {
                        Data::Int(i) => {
                            if i < 0 {
                                error(
                                    self.name,
                                    self.file,
                                    "E0004",
                                    "index cannot be negative",
                                    "index cannot be negative",
                                    pos,
                                );
                            }
                            match vec.get(i as usize) {
                                Some(data) => data.to_owned(),
                                None => error(
                                    self.name,
                                    self.file,
                                    "E0004",
                                    &format!("index out of bounds, index {} is out of bounds for vector of length {}", i, vec.len()),
                                    "index out of bounds",
                                    pos,
                                )
                            }
                        }
                        data => error(
                            self.name,
                            self.file,
                            "E0004",
                            &format!(
                                "mismatched data types, expected `int` found {}",
                                data
                            ),
                            "a value of type `int` was expected",
                            pos,
                        ),
                    },
                    data => error(
                        self.name,
                        self.file,
                        "0004",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            data
                        ),
                        "a value of type `vector` was expected",
                        pos,
                    ),
                }
            }
        }
    }

    pub fn parse_binary_operation(
        &mut self,
        lhs: Expression,
        op: Token,
        rhs: Expression,
        pos: &Range<usize>,
    ) -> Data {
        let lhs = self.parse_expression(lhs, None, pos);
        let operator = op;
        let rhs = self.parse_expression(rhs, None, pos);
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
                            data
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
                            data
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
                        data
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
                            data
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
                        data
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
                            data
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
                        data
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
                            data
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
                        data
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
                            data
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
                        data
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
                            data
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
                            data
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
                            data
                        ),
                        "a value of type `bool` was expected",
                        pos,
                    ),
                },
                Data::Vector(v1, _) => match rhs {
                    Data::Vector(v2, _) => Data::Bool(v1 == v2),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            data
                        ),
                        "a value of type `vector` was expected",
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
                            data
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
                            data
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
                            data
                        ),
                        "a value of type `bool` was expected",
                        pos,
                    ),
                },
                Data::Vector(v1, _) => match rhs {
                    Data::Vector(v2, _) => Data::Bool(v1 != v2),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            data
                        ),
                        "a value of type `vector` was expected",
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
                            data
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
                            data
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
                            data
                        ),
                        "a value of type `bool` was expected",
                        pos,
                    ),
                },
                Data::Vector(v1, _) => match rhs {
                    Data::Vector(v2, _) => Data::Bool(v1 > v2),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            data
                        ),
                        "a value of type `vector` was expected",
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
                            data
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
                            data
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
                            data
                        ),
                        "a value of type `bool` was expected",
                        pos,
                    ),
                },
                Data::Vector(v1, _) => match rhs {
                    Data::Vector(v2, _) => Data::Bool(v1 < v2),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            data
                        ),
                        "a value of type `vector` was expected",
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
                            data
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
                            data
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
                            data
                        ),
                        "a value of type `bool` was expected",
                        pos,
                    ),
                },
                Data::Vector(v1, _) => match rhs {
                    Data::Vector(v2, _) => Data::Bool(v1 >= v2),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            data
                        ),
                        "a value of type `vector` was expected",
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
                            data
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
                            data
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
                            data
                        ),
                        "a value of type `bool` was expected",
                        pos,
                    ),
                },
                Data::Vector(v1, _) => match rhs {
                    Data::Vector(v2, _) => Data::Bool(v1 <= v2),
                    data => error(
                        self.name,
                        self.file,
                        "0002",
                        &format!(
                            "mismatched data types, expected `vector` found {}",
                            data
                        ),
                        "a value of type `vector` was expected",
                        pos,
                    ),
                },
            },
            _ => unreachable!(),
        }
    }
}
