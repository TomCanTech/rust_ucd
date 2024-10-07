use crate::{dictionary::Dictionary, message::Message};

pub struct Model {
    pub dictionary: Dictionary,
    pub running_state: RunningState
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    None
}

#[derive(PartialEq)]
pub enum RunningState {
    Running,
    Done
}