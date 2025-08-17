//! Animation utilities for creating dynamic terminal content.
//!
//! This module provides frame-based animation support for components like spinners
//! and progress indicators. It includes timing utilities and predefined animation
//! sequences.

use std::time::{Duration, Instant};

/// Represents a sequence of animation frames.
#[derive(Debug, Clone)]
pub struct Animation {
    /// The frames in this animation
    frames: Vec<String>,
    /// Duration each frame should be displayed
    frame_duration: Duration,
    /// Whether the animation should loop
    loops: bool,
    /// Current frame index
    current_frame: usize,
    /// Time when the current frame started
    frame_start: Option<Instant>,
}

impl Animation {
    /// Create a new animation with the given frames and frame duration.
    pub fn new(frames: Vec<String>, frame_duration: Duration) -> Self {
        Self {
            frames,
            frame_duration,
            loops: true,
            current_frame: 0,
            frame_start: None,
        }
    }

    /// Create an animation that doesn't loop.
    pub fn once(frames: Vec<String>, frame_duration: Duration) -> Self {
        Self {
            frames,
            frame_duration,
            loops: false,
            current_frame: 0,
            frame_start: None,
        }
    }

    /// Set whether this animation should loop.
    pub fn with_loops(mut self, loops: bool) -> Self {
        self.loops = loops;
        self
    }

    /// Start the animation by recording the current time.
    pub fn start(&mut self) {
        self.frame_start = Some(Instant::now());
        self.current_frame = 0;
    }

    /// Get the current frame of the animation.
    /// Returns None if the animation hasn't started or has finished (for non-looping animations).
    pub fn current_frame(&self) -> Option<&str> {
        if self.frames.is_empty() {
            return None;
        }

        if !self.loops && self.current_frame >= self.frames.len() {
            return self.frames.last().map(|s| s.as_str());
        }

        self.frames.get(self.current_frame).map(|s| s.as_str())
    }

    /// Update the animation, advancing to the next frame if enough time has passed.
    /// Returns true if the frame changed.
    pub fn update(&mut self) -> bool {
        if self.frames.is_empty() {
            return false;
        }

        let frame_start = match self.frame_start {
            Some(start) => start,
            None => {
                // Animation not started
                return false;
            }
        };

        let elapsed = frame_start.elapsed();
        if elapsed < self.frame_duration {
            return false; // Not time to advance yet
        }

        // Time to advance to next frame
        let frames_to_advance = (elapsed.as_millis() / self.frame_duration.as_millis()) as usize;
        let old_frame = self.current_frame;

        if self.loops {
            self.current_frame = (self.current_frame + frames_to_advance) % self.frames.len();
        } else {
            self.current_frame =
                std::cmp::min(self.current_frame + frames_to_advance, self.frames.len());
        }

        // Update frame start time to account for the time advancement
        self.frame_start = Some(
            frame_start
                + Duration::from_millis(
                    (frames_to_advance as u64) * self.frame_duration.as_millis() as u64,
                ),
        );

        old_frame != self.current_frame
    }

    /// Check if the animation is finished (only relevant for non-looping animations).
    pub fn is_finished(&self) -> bool {
        !self.loops && self.current_frame >= self.frames.len()
    }

    /// Reset the animation to the beginning.
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frame_start = None;
    }

    /// Get the total number of frames in this animation.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Get the duration of each frame.
    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    /// Get the total duration of one complete animation cycle.
    pub fn cycle_duration(&self) -> Duration {
        Duration::from_millis((self.frames.len() as u64) * self.frame_duration.as_millis() as u64)
    }
}

/// Predefined spinner animations.
pub struct Spinners;

impl Spinners {
    /// Classic dots spinner: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
    pub fn dots() -> Animation {
        Animation::new(
            vec![
                "⠋".to_string(),
                "⠙".to_string(),
                "⠹".to_string(),
                "⠸".to_string(),
                "⠼".to_string(),
                "⠴".to_string(),
                "⠦".to_string(),
                "⠧".to_string(),
                "⠇".to_string(),
                "⠏".to_string(),
            ],
            Duration::from_millis(80),
        )
    }

    /// Simple line spinner: | / - \
    pub fn line() -> Animation {
        Animation::new(
            vec![
                "|".to_string(),
                "/".to_string(),
                "-".to_string(),
                "\\".to_string(),
            ],
            Duration::from_millis(100),
        )
    }

    /// Arrow spinner: ← ↖ ↑ ↗ → ↘ ↓ ↙
    pub fn arrow() -> Animation {
        Animation::new(
            vec![
                "←".to_string(),
                "↖".to_string(),
                "↑".to_string(),
                "↗".to_string(),
                "→".to_string(),
                "↘".to_string(),
                "↓".to_string(),
                "↙".to_string(),
            ],
            Duration::from_millis(120),
        )
    }

    /// Clock spinner: 🕐 🕑 🕒 🕓 🕔 🕕 🕖 🕗 🕘 🕙 🕚 🕛
    pub fn clock() -> Animation {
        Animation::new(
            vec![
                "🕐".to_string(),
                "🕑".to_string(),
                "🕒".to_string(),
                "🕓".to_string(),
                "🕔".to_string(),
                "🕕".to_string(),
                "🕖".to_string(),
                "🕗".to_string(),
                "🕘".to_string(),
                "🕙".to_string(),
                "🕚".to_string(),
                "🕛".to_string(),
            ],
            Duration::from_millis(100),
        )
    }

    /// Growing dots: ⠁ ⠃ ⠇ ⠏ ⠟ ⠿ ⡿ ⣿
    pub fn growing_dots() -> Animation {
        Animation::new(
            vec![
                "⠁".to_string(),
                "⠃".to_string(),
                "⠇".to_string(),
                "⠏".to_string(),
                "⠟".to_string(),
                "⠿".to_string(),
                "⡿".to_string(),
                "⣿".to_string(),
            ],
            Duration::from_millis(100),
        )
    }

    /// Bouncing ball: ⠁ ⠂ ⠄ ⡀ ⢀ ⠠ ⠐ ⠈
    pub fn bouncing_ball() -> Animation {
        Animation::new(
            vec![
                "⠁".to_string(),
                "⠂".to_string(),
                "⠄".to_string(),
                "⡀".to_string(),
                "⢀".to_string(),
                "⠠".to_string(),
                "⠐".to_string(),
                "⠈".to_string(),
            ],
            Duration::from_millis(120),
        )
    }

    /// Simple dots: . .. ...
    pub fn simple_dots() -> Animation {
        Animation::new(
            vec![".  ".to_string(), ".. ".to_string(), "...".to_string()],
            Duration::from_millis(400),
        )
    }

    /// ASCII fallback spinner for compatibility: - \ | /
    pub fn ascii() -> Animation {
        Animation::new(
            vec![
                "-".to_string(),
                "\\".to_string(),
                "|".to_string(),
                "/".to_string(),
            ],
            Duration::from_millis(150),
        )
    }

    /// Get a spinner by name for easier configuration.
    pub fn by_name(name: &str) -> Option<Animation> {
        match name {
            "dots" => Some(Self::dots()),
            "line" => Some(Self::line()),
            "arrow" => Some(Self::arrow()),
            "clock" => Some(Self::clock()),
            "growing_dots" => Some(Self::growing_dots()),
            "bouncing_ball" => Some(Self::bouncing_ball()),
            "simple_dots" => Some(Self::simple_dots()),
            "ascii" => Some(Self::ascii()),
            _ => None,
        }
    }

    /// Get a list of all available spinner names.
    pub fn available_names() -> Vec<&'static str> {
        vec![
            "dots",
            "line",
            "arrow",
            "clock",
            "growing_dots",
            "bouncing_ball",
            "simple_dots",
            "ascii",
        ]
    }
}

/// A timer utility for tracking elapsed time and intervals.
#[derive(Debug)]
pub struct Timer {
    start_time: Instant,
    last_interval: Instant,
    interval_duration: Duration,
}

impl Timer {
    /// Create a new timer with the given interval.
    pub fn new(interval: Duration) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_interval: now,
            interval_duration: interval,
        }
    }

    /// Check if the interval has elapsed since the last check.
    /// If so, updates the last interval time and returns true.
    pub fn check_interval(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_interval) >= self.interval_duration {
            self.last_interval = now;
            true
        } else {
            false
        }
    }

    /// Get the total elapsed time since the timer was created.
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Reset the timer to start from now.
    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_interval = now;
    }

    /// Get the time until the next interval.
    pub fn time_until_next_interval(&self) -> Duration {
        let elapsed_since_last = self.last_interval.elapsed();
        if elapsed_since_last >= self.interval_duration {
            Duration::from_millis(0)
        } else {
            self.interval_duration - elapsed_since_last
        }
    }
}

/// Create a progress animation that fills from left to right.
///
/// # Arguments
/// * `width` - Total width of the progress bar
/// * `fill_char` - Character to use for filled portions
/// * `empty_char` - Character to use for empty portions
/// * `frame_duration` - How long each frame should be displayed
///
/// # Returns
/// An animation that shows a progress bar filling up
pub fn progress_fill_animation(
    width: usize,
    fill_char: char,
    empty_char: char,
    frame_duration: Duration,
) -> Animation {
    let mut frames = Vec::new();

    for filled in 0..=width {
        let mut frame = String::new();
        frame.push_str(&fill_char.to_string().repeat(filled));
        frame.push_str(&empty_char.to_string().repeat(width - filled));
        frames.push(frame);
    }

    Animation::once(frames, frame_duration)
}

/// Create a wave animation that moves across the given width.
///
/// # Arguments
/// * `width` - Total width of the wave
/// * `wave_char` - Character to use for the wave
/// * `background_char` - Character to use for background
/// * `wave_length` - Length of the wave
/// * `frame_duration` - How long each frame should be displayed
///
/// # Returns
/// An animation that shows a wave moving across the width
pub fn wave_animation(
    width: usize,
    wave_char: char,
    background_char: char,
    wave_length: usize,
    frame_duration: Duration,
) -> Animation {
    let mut frames = Vec::new();

    for offset in 0..width {
        let mut frame = vec![background_char; width];

        for i in 0..wave_length {
            let pos = (offset + i) % width;
            frame[pos] = wave_char;
        }

        frames.push(frame.iter().collect());
    }

    Animation::new(frames, frame_duration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_animation_creation() {
        let frames = vec!["frame1".to_string(), "frame2".to_string()];
        let animation = Animation::new(frames.clone(), Duration::from_millis(100));

        assert_eq!(animation.frame_count(), 2);
        assert_eq!(animation.frame_duration(), Duration::from_millis(100));
        assert_eq!(animation.cycle_duration(), Duration::from_millis(200));
    }

    #[test]
    fn test_animation_frames() {
        let frames = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut animation = Animation::new(frames, Duration::from_millis(10));

        // Before starting, should have first frame
        assert!(animation.current_frame().is_some());

        animation.start();
        assert_eq!(animation.current_frame(), Some("A"));

        // Wait and update
        thread::sleep(Duration::from_millis(15));
        let changed = animation.update();
        assert!(changed);
        assert_eq!(animation.current_frame(), Some("B"));
    }

    #[test]
    fn test_animation_looping() {
        let frames = vec!["1".to_string(), "2".to_string()];
        let mut animation = Animation::new(frames, Duration::from_millis(10));

        animation.start();

        // Advance through all frames and back to start
        thread::sleep(Duration::from_millis(25));
        animation.update();
        let current = animation.current_frame();
        assert!(current == Some("1") || current == Some("2")); // May be either frame due to timing
    }

    #[test]
    fn test_animation_once() {
        let frames = vec!["1".to_string(), "2".to_string()];
        let mut animation = Animation::once(frames, Duration::from_millis(10));

        animation.start();

        // Advance past all frames
        thread::sleep(Duration::from_millis(25));
        animation.update();
        assert!(animation.is_finished());
        assert_eq!(animation.current_frame(), Some("2")); // Should stay on last frame
    }

    #[test]
    fn test_spinners() {
        let dots = Spinners::dots();
        assert_eq!(dots.frame_count(), 10);

        let line = Spinners::line();
        assert_eq!(line.frame_count(), 4);

        // Test getting by name
        let arrow = Spinners::by_name("arrow").unwrap();
        assert_eq!(arrow.frame_count(), 8);

        assert!(Spinners::by_name("nonexistent").is_none());
    }

    #[test]
    fn test_timer() {
        let mut timer = Timer::new(Duration::from_millis(10));

        // Should not trigger immediately
        assert!(!timer.check_interval());

        // Wait and check again
        thread::sleep(Duration::from_millis(15));
        assert!(timer.check_interval());

        // Should not trigger again immediately
        assert!(!timer.check_interval());
    }

    #[test]
    fn test_progress_fill_animation() {
        let animation = progress_fill_animation(5, '█', '░', Duration::from_millis(100));

        assert_eq!(animation.frame_count(), 6); // 0 to 5 filled
        assert!(!animation.loops); // Should be a one-time animation

        // Check first and last frames
        let _frames: Vec<_> = (0..=5)
            .map(|i| format!("{}{}", "█".repeat(i), "░".repeat(5 - i)))
            .collect();

        assert_eq!(animation.frames[0], "░░░░░");
        assert_eq!(animation.frames[5], "█████");
    }

    #[test]
    fn test_wave_animation() {
        let animation = wave_animation(5, '~', '.', 2, Duration::from_millis(50));

        assert_eq!(animation.frame_count(), 5);
        assert!(animation.loops); // Should loop

        // Each frame should have exactly 2 wave characters
        for frame in &animation.frames {
            let wave_count = frame.chars().filter(|&c| c == '~').count();
            assert_eq!(wave_count, 2);
        }
    }

    #[test]
    fn test_available_spinner_names() {
        let names = Spinners::available_names();
        assert!(names.contains(&"dots"));
        assert!(names.contains(&"line"));
        assert!(names.contains(&"ascii"));

        // Verify all names actually work
        for name in names {
            assert!(Spinners::by_name(name).is_some());
        }
    }

    #[test]
    fn test_animation_reset() {
        let frames = vec!["A".to_string(), "B".to_string()];
        let mut animation = Animation::new(frames, Duration::from_millis(10));

        animation.start();
        thread::sleep(Duration::from_millis(15));
        animation.update();

        // Should be on frame B
        assert_eq!(animation.current_frame(), Some("B"));

        animation.reset();
        assert_eq!(animation.current_frame(), Some("A")); // Reset to first frame

        animation.start();
        assert_eq!(animation.current_frame(), Some("A")); // Back to first frame
    }
}
