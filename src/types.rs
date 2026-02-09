use crate::resources::Resources;

#[derive(Debug)]
pub enum Phase {
    Selection,
    Game,
    Results,
    Skip,
    Quit,
}

pub enum Command {
    Normal(String),
    ChangePhase(String),
}

pub struct GameState {
    pub filepath: String,
    pub resources: Resources,
    pub translations: Vec<(String, String, f64)>, // (og, trans, word_score)
    pub settings: (String, String), // (mode, order)
    pub results: (i32, i32), // (tested, passed)
}
