use std::clone::Clone;

#[derive(Clone, PartialEq)]
pub struct Turn {
    pub initiator: String,
    pub item: String,
    pub action: String,
    pub targets: Vec<String>,
}
