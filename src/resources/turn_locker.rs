#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnLockMode {
    PlayerInput,
    PlayerTurn,
    WorldTurn,
}

pub struct TurnLocker {
    pub lock: TurnLockMode,
    pub end_turn: bool,
    pub next_sys: bool,
}

impl Default for TurnLocker {
    fn default() -> Self {
        TurnLocker {
            lock: TurnLockMode::PlayerInput,
            end_turn: true,
            next_sys: false,
        }
    }
}

impl TurnLocker {
    pub fn trigger_next_sys(&mut self) {
        self.lock = match self.lock {
            TurnLockMode::PlayerInput => TurnLockMode::PlayerTurn,
            TurnLockMode::PlayerTurn => TurnLockMode::WorldTurn,
            TurnLockMode::WorldTurn => TurnLockMode::PlayerInput,
        };
        self.next_sys = false;
    }
}
