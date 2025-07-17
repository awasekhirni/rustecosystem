use super::entities::PlayerState;
use super::traits::PlayerControls;

/// Concrete player implementation
#[derive(Debug)]
pub struct Player {
    state: PlayerState,
}

impl Player {
    /// Create a new player instance
    pub fn new() -> Self {
        Player {
            state: PlayerState::default(),
        }
    }
}

impl PlayerControls for Player {
    fn play(&self, track_id: &str) {
        println!("Playing track: {}", track_id);
    }

    fn pause(&self) {
        println!("Playback paused");
    }

    fn skip(&self) {
        println!("Skipping to next track");
    }

    fn status(&self) -> super::traits::PlaybackStatus {
        super::traits::PlaybackStatus::Stopped
    }
}
