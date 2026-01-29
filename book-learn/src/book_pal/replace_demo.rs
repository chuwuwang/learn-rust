use std::mem;

enum State {
    Running(Vec<i32>),
    Stopped,
}

struct Machine {
    state: State,
}

impl Machine {

    fn stop(&mut self) {
        let old_data = mem::replace(&mut self.state, State::Stopped);
        match old_data {
            State::Running(data) => {
                tracing::info!("Stopping machine with data: {:?}", data);
            }
            State::Stopped => {
                tracing::info!("Machine is already stopped.");
            }
        }
    }

}
