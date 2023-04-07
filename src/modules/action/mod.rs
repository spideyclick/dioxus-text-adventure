use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Action {
    pub id: String,
    pub title: String,
    pub command: ActionType,
    pub hp: i32,
    pub targets: TargetType,
    pub consumes_item: bool,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionType {
    Heal,
    Harm,
}
impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionType::Heal => write!(f, "Heal"),
            ActionType::Harm => write!(f, "Harm"),
        }
    }
}
#[derive(Clone, PartialEq)]
pub enum TargetType {
    YourSelf,
    Anyone,
    Friendly,
    Hostile,
    AnyoneElse,
}
