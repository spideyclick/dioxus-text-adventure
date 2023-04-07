use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Alert {
    pub type_: AlertType,
    pub content: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlertType {
    Info,
    Warning,
    Error,
}
impl fmt::Display for AlertType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlertType::Info => write!(f, "Info"),
            AlertType::Warning => write!(f, "Warning"),
            AlertType::Error => write!(f, "Error"),
        }
    }
}
