use derivative::Derivative;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum HistoryEntryType {
    Move,
    Action,
    Generic,
}
impl fmt::Display for HistoryEntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HistoryEntryType::Move => write!(f, "Move"),
            HistoryEntryType::Action => write!(f, "Action"),
            HistoryEntryType::Generic => write!(f, "Generic"),
        }
    }
}

#[derive(Derivative, Clone, PartialEq)]
pub struct HistoryEntry {
    pub type_: HistoryEntryType,
    pub content: String,
}
