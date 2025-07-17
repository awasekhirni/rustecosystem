//! Player traits defining interfaces for audio playback

/// Interface for basic player controls
pub trait PlayerControls {
    /// Play a specific track by ID
    fn play(&self, track_id: &str);

    /// Pause the currently playing track
    fn pause(&self);

    /// Skip to the next track in the queue
    fn skip(&self);

    /// Get the current playback status
    fn status(&self) -> PlaybackStatus;
}

/// Interface for playback status information
pub trait PlaybackStatusProvider {
    /// Get current playback position in seconds
    fn current_position(&self) -> f64;

    /// Get current track duration in seconds
    fn current_duration(&self) -> f64;

    /// Check if player is currently playing
    fn is_playing(&self) -> bool;
}

/// Interface for volume control
pub trait VolumeControl {
    /// Set volume level (0.0 to 1.0)
    fn set_volume(&mut self, level: f32);

    /// Get current volume level
    fn get_volume(&self) -> f32;
}

/// Playback status information
#[derive(Debug, Clone)]
pub enum PlaybackStatus {
    Playing { position: f64, duration: f64 },
    Paused { position: f64, duration: f64 },
    Stopped,
}
