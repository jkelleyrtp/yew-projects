pub struct RunnerSlice {
    pub running: bool,
    pub steps_left: u32,
}

pub enum RunnerAction {
    Play,
    Pause,
    Step,
}

impl RunnerAction {
    fn dispatch(self, slice: &mut RunnerSlice) {
        match self {
            RunnerAction::Play => {
                slice.running = true;
            }
            RunnerAction::Pause => {
                slice.running = false;
            }
            RunnerAction::Step => {
                slice.steps_left = 1;
            }
        }
    }
}
