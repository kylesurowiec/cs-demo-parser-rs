#[derive(Default, Clone)]
pub struct GameState {
    ingame_tick: i32,
}

impl GameState {
    pub fn ingame_tick(&self) -> i32 {
        self.ingame_tick
    }

    pub fn set_ingame_tick(&mut self, tick: i32) {
        self.ingame_tick = tick;
    }
}
