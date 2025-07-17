//! Player entities and state management

/// Internal state of the player
#[derive(Debug, Default)]
pub struct PlayerState {
    pub volume: f32,
    pub current_track: Option<String>,
    pub queue: Vec<String>,
    pub is_playing: bool,
    pub position: f64,
}
