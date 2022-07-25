#[derive(Clone, Debug)]
pub enum Data {
    Text(String),
    Number(i128),
    Boolean(bool),
    Placeholder,
}
