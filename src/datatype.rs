#[derive(Clone, Debug)]
pub enum Data {
    String(String),
    Integer(i64),
    Bool(bool),
}