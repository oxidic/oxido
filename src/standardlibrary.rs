use crate::datatype::Data;

pub struct StandardLibrary;

impl StandardLibrary {
    pub fn contains(x: &str) -> bool {
        ["print", "println", "read", "int", "bool", "str"].contains(&x)
    }

    pub fn call(x: &str, params: Vec<Data>) -> Option<Data> {
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
            "int" => Some(types::int(params.first()?.to_owned())),
            "bool" => Some(types::bool(params.first()?.to_owned())),
            "str" => Some(types::str(params.first()?.to_owned())),
            _ => panic!("not a global function"),
        }
    }
}

mod types {
    use crate::datatype::Data;

    pub fn int(data: Data) -> Data {
        match data {
            Data::Integer(_) => data,
            Data::Bool(b) => Data::Integer(b as i64),
            Data::Str(s) => Data::Integer(s.parse::<i64>().unwrap()),
        }
    }

    pub fn bool(data: Data) -> Data {
        match data {
            Data::Integer(i) => Data::Bool(i != 0),
            Data::Bool(_) => data,
            Data::Str(s) => Data::Bool(s.parse::<bool>().unwrap()),
        }
    }

    pub fn str(data: Data) -> Data {
        match data {
            Data::Integer(i) => Data::Str(i.to_string()),
            Data::Bool(b) => Data::Str(b.to_string()),
            Data::Str(_) => data,
        }
    }
}

mod io {
    use crate::datatype::Data;
    use std::io::stdin;

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
                Data::Integer(i) => print!("{i}"),
                Data::Bool(b) => print!("{b}"),
                Data::Str(s) => print!("{s}"),
            }
        }
    }

    pub fn println(datas: Vec<Data>) {
        self::print(datas);
        println!();
    }
}
