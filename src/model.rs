struct Model {
    dictionary: crate::dictionary::Dictionary,
    running_state: RunningState
}


enum RunningState {
    Running,
    Done
}