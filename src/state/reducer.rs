use crate::command::command::Command;
use crate::state::app_state::AppState;

pub fn reduce(state: &mut AppState, cmd: Command) {
    match cmd {
        Command::Increment => state.value += 1.0,
        Command::Decrement => state.value -= 1.0,
    }
}
