#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum GamePhase {
    #[default]
    Init = 0,
    Pregame = 1,
    StartGamePhase = 2,
    TeamSideSwitch = 3,
    GameHalfEnded = 4,
    GameEnded = 5,
    StaleMate = 6,
    GameOver = 7,
}

impl std::fmt::Display for GamePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | GamePhase::Init => "Init",
                | GamePhase::Pregame => "Pregame",
                | GamePhase::StartGamePhase => "Start game phase",
                | GamePhase::TeamSideSwitch => "Team side switch",
                | GamePhase::GameHalfEnded => "Game half ended",
                | GamePhase::GameEnded => "Game ended",
                | GamePhase::StaleMate => "StaleMate",
                | GamePhase::GameOver => "GameOver",
            }
        )
    }
}
