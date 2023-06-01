use std::ops::Range;

use crate::data::Data;

#[derive(Debug, Clone)]
pub struct StandardLibrary<'a> {
    name: &'a str,
    file: String,
}

impl<'a> StandardLibrary<'a> {
    pub fn new(name: &'a str, file: String) -> Self {
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
                &self.file,
                range,
                params.first()?.to_owned(),
            )),
            "bool" => Some(types::bool(
                self.name,
                &self.file,
                range,
                params.first()?.to_owned(),
            )),
            "str" => Some(types::str(
                self.name,
                &self.file,
                range,
                params.first()?.to_owned(),
            )),
            "vec" => Some(types::vec(
                self.name,
                &self.file,
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
                &format!("mismatched data types expected `vector` found {}", data),
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
                    data
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
                    data
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
                    data
                ),
                range,
            ),
        }
    }
}

mod io {
    use crate::data::Data;
    use std::io::{stdin, stdout, Write};

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen::prelude::wasm_bindgen;

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

    #[cfg(target_arch = "wasm32")]
    pub fn p(data: &str) {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
        }
        macro_rules! console_log {
            ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
        }
        console_log!("{data}");
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn p(data: &str) {
        print!("{data}");
    }

    pub fn print(datas: Vec<Data>) {
        for data in datas {
            match data {
                Data::Int(i) => p(&i.to_string()),
                Data::Bool(b) => p(&b.to_string()),
                Data::Str(s) => p(&s),
                Data::Vector(vec, _) => {
                    p("[");
                    for (i, v) in vec.iter().enumerate() {
                        print(vec![v.clone()]);
                        if i != vec.len() - 1 {
                            p(", ");
                        }
                    }
                    p("]")
                }
            }
        }

        stdout().flush().unwrap();
    }

    pub fn println(datas: Vec<Data>) {
        if datas.is_empty() {
            p("\n");
            return;
        }
        for data in datas {
            match data {
                Data::Int(i) => p(&format!("{i}\n")),
                Data::Bool(b) => p(&format!("{b}\n")),
                Data::Str(s) => p(&format!("{s}\n")),
                Data::Vector(vec, _) => {
                    p("[");
                    for (i, v) in vec.iter().enumerate() {
                        print(vec![v.clone()]);
                        if i != vec.len() - 1 {
                            p(", ");
                        }
                    }
                    p("]")
                }
            }
        }
    }
}
