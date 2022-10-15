use crate::expression::Data;

mod print;

pub struct Globals;

impl Globals {
    pub fn print(data: Data) {
        print::print(data);
    }

    pub fn println(data: Data) {
        Globals::print(data);
        Globals::print(Data::String("\n".to_string()))
    }
}
