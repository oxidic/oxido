use std::ops::Range;

use crate::data::Data;

pub struct StandardLibrary<'a> {
    name: &'a str,
    file: &'a str,
}

impl<'a> StandardLibrary<'a> {
    pub fn new(name: &'a str, file: &'a str) -> Self {
        Self { name, file }
    }
    pub fn contains(&self, x: &str) -> bool {
        ["print", "println", "read", "int", "bool", "str", "vec"].contains(&x)
    }

    pub fn call(&self, x: &str, range: &Range<usize>, params: Vec<Data>) -> Option<Data> {
        match x {
            "print" => {
                io::print(params);
                None
            }
            "println" => {
                io::println(params);
                None
            }
            "read" => Some(io::read()),
            "int" => Some(types::int(
                self.name,
                self.file,
                range,
                params.first()?.to_owned(),
            )),
            "bool" => Some(types::bool(
                self.name,
                self.file,
                range,
                params.first()?.to_owned(),
            )),
            "str" => Some(types::str(
                self.name,
                self.file,
                range,
                params.first()?.to_owned(),
            )),
            "vec" => Some(types::vec(
                self.name,
                self.file,
                range,
                params.first()?.to_owned(),
            )),
            _ => panic!("not a global function"),
        }
    }
}

mod types {
    use crate::{
        data::{Data, DataType},
        error::error,
    };
    use std::ops::Range;

    pub fn vec(name: &str, file: &str, range: &Range<usize>, data: Data) -> Data {
        match data {
            Data::Vector(_, _) => data,
            Data::Str(str) => Data::Vector(
                str.chars().map(|ch| Data::Str(ch.to_string())).collect(),
                DataType::Str,
            ),
            _ => error(
                name,
                file,
                "E00011",
                "incorrect data type",
                &format!(
                    "mismatched data types expected `vector` found {}",
                    data.to_string()
                ),
                range,
            ),
        }
    }

    pub fn int(name: &str, file: &str, range: &Range<usize>, data: Data) -> Data {
        match data {
            Data::Int(_) => data,
            Data::Bool(b) => Data::Int(b as i64),
            Data::Str(s) => Data::Int(s.parse::<i64>().unwrap()),
            _ => error(
                name,
                file,
                "E00011",
                "incorrect data type",
                &format!(
                    "mismatched data types expected `int | bool | str` found {}",
                    data.to_string()
                ),
                range,
            ),
        }
    }

    pub fn bool(name: &str, file: &str, range: &Range<usize>, data: Data) -> Data {
        match data {
            Data::Int(i) => Data::Bool(i != 0),
            Data::Bool(_) => data,
            Data::Str(s) => Data::Bool(s.parse::<bool>().unwrap()),
            _ => error(
                name,
                file,
                "E00011",
                "incorrect data type",
                &format!(
                    "mismatched data types expected `int | bool | str` found {}",
                    data.to_string()
                ),
                range,
            ),
        }
    }

    pub fn str(name: &str, file: &str, range: &Range<usize>, data: Data) -> Data {
        match data {
            Data::Int(i) => Data::Str(i.to_string()),
            Data::Bool(b) => Data::Str(b.to_string()),
            Data::Str(_) => data,
            _ => error(
                name,
                file,
                "E00011",
                "incorrect data type",
                &format!(
                    "mismatched data types expected `int | bool | str` found {}",
                    data.to_string()
                ),
                range,
            ),
        }
    }
}

mod io {
    use crate::data::Data;
    use std::io::{stdin, Write, stdout};

    pub fn read() -> Data {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        Data::Str(s)
    }

    pub fn print(datas: Vec<Data>) {
        for data in datas {
            match data {
                Data::Int(i) => print!("{i}"),
                Data::Bool(b) => print!("{b}"),
                Data::Str(s) => print!("{s}"),
                Data::Vector(vec, _) => {
                    print!("[");
                    for (i, v) in vec.iter().enumerate() {
                        print(vec![v.clone()]);
                        if i != vec.len() - 1 {
                            print!(", ");
                        }
                    }
                    print!("]")
                }
            }
        }

        stdout().flush().unwrap();
    }

    pub fn println(datas: Vec<Data>) {
        for data in datas {
            match data {
                Data::Int(i) => println!("{i}"),
                Data::Bool(b) => println!("{b}"),
                Data::Str(s) => println!("{s}"),
                Data::Vector(vec, _) => {
                    print!("[");
                    for (i, v) in vec.iter().enumerate() {
                        print(vec![v.clone()]);
                        if i != vec.len() - 1 {
                            print!(", ");
                        }
                    }
                    print!("]")
                }
            }
        }
    }
}
