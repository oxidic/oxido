use crate::expression::Data;

pub fn print(data: Data) {
    match data {
        Data::Integer(i) => print!("{i}"),
        Data::Bool(b) => print!("{b}"),
        Data::String(s) => print!("{s}"),
        Data::Placeholder => todo!(),
    }
}