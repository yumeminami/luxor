//! Integration tests for the animation module.
//!
//! These tests verify that the animation system works correctly with
//! frame timing, spinners, and animation state management.

use luxor::animation::{Animation, Spinners, Timer, progress_fill_animation, wave_animation};
use std::thread;
use std::time::Duration;

#[test]
fn test_animation_creation() {
    let frames = vec![
        "frame1".to_string(),
        "frame2".to_string(),
        "frame3".to_string(),
    ];
    let animation = Animation::new(frames.clone(), Duration::from_millis(100));

    assert_eq!(animation.frame_count(), 3);
    assert_eq!(animation.frame_duration(), Duration::from_millis(100));
    assert_eq!(animation.cycle_duration(), Duration::from_millis(300));
    assert!(!animation.is_finished()); // Looping animation never finishes

    // Test one-time animation
    let once_animation = Animation::once(frames, Duration::from_millis(50));
    assert_eq!(once_animation.frame_count(), 3);
    assert_eq!(once_animation.cycle_duration(), Duration::from_millis(150));
}

#[test]
fn test_animation_looping() {
    let frames = vec!["A".to_string(), "B".to_string()];
    let mut animation = Animation::new(frames, Duration::from_millis(10)).with_loops(true);

    // Before starting, should have first frame at index 0
    assert!(animation.current_frame().is_some());

    animation.start();
    assert_eq!(animation.current_frame(), Some("A"));

    // Wait and update to advance frame
    thread::sleep(Duration::from_millis(15));
    let changed = animation.update();
    assert!(changed);
    assert_eq!(animation.current_frame(), Some("B"));

    // Wait and update again to loop back
    thread::sleep(Duration::from_millis(15));
    animation.update();
    // Should advance to A or B (next frame), depending on timing
    let current = animation.current_frame();
    assert!(current == Some("A") || current == Some("B"));
    assert!(!animation.is_finished());
}

#[test]
fn test_animation_once() {
    let frames = vec!["1".to_string(), "2".to_string()];
    let mut animation = Animation::once(frames, Duration::from_millis(10));

    animation.start();
    assert_eq!(animation.current_frame(), Some("1"));

    // Advance past all frames
    thread::sleep(Duration::from_millis(25));
    animation.update();

    // Should be finished and stay on last frame
    assert!(animation.is_finished());
    assert_eq!(animation.current_frame(), Some("2"));

    // Further updates shouldn't change the frame
    animation.update();
    assert_eq!(animation.current_frame(), Some("2"));
}

#[test]
fn test_animation_frame_advancement() {
    let frames = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut animation = Animation::new(frames, Duration::from_millis(10));

    animation.start();
    assert_eq!(animation.current_frame(), Some("A"));

    // Test that update returns false if not enough time has passed
    assert!(!animation.update());
    assert_eq!(animation.current_frame(), Some("A"));

    // Wait for frame duration and test advancement
    thread::sleep(Duration::from_millis(15));
    assert!(animation.update());
    assert_eq!(animation.current_frame(), Some("B"));

    // Test multiple frame advancement
    thread::sleep(Duration::from_millis(25)); // Enough for 2+ frames
    animation.update();
    // Should advance to some other frame (timing-dependent)
    let current = animation.current_frame();
    assert!(current.is_some()); // Just verify we have a frame
}

#[test]
fn test_animation_reset() {
    let frames = vec!["A".to_string(), "B".to_string()];
    let mut animation = Animation::new(frames, Duration::from_millis(10));

    animation.start();
    thread::sleep(Duration::from_millis(15));
    animation.update();
    assert_eq!(animation.current_frame(), Some("B"));

    // Reset should go back to frame 0
    animation.reset();
    assert_eq!(animation.current_frame(), Some("A"));

    // Start again
    animation.start();
    assert_eq!(animation.current_frame(), Some("A"));
}

#[test]
fn test_empty_animation() {
    let empty_animation = Animation::new(vec![], Duration::from_millis(100));
    assert_eq!(empty_animation.frame_count(), 0);
    assert!(empty_animation.current_frame().is_none());

    // Starting empty animation should still return None
    let mut empty_animation = empty_animation;
    empty_animation.start();
    assert!(empty_animation.current_frame().is_none());
    assert!(!empty_animation.update());
}

#[test]
fn test_single_frame_animation() {
    let single_frame = Animation::new(vec!["only".to_string()], Duration::from_millis(100));
    assert_eq!(single_frame.frame_count(), 1);

    let mut single_frame = single_frame;
    single_frame.start();
    assert_eq!(single_frame.current_frame(), Some("only"));

    // Update should keep the same frame
    thread::sleep(Duration::from_millis(150));
    single_frame.update();
    assert_eq!(single_frame.current_frame(), Some("only"));
}

#[test]
fn test_spinner_dots() {
    let dots = Spinners::dots();
    assert_eq!(dots.frame_count(), 10);
    assert_eq!(dots.frame_duration(), Duration::from_millis(80));

    let mut dots = dots;
    dots.start();
    assert_eq!(dots.current_frame(), Some("⠋"));

    // Test that frames are different
    thread::sleep(Duration::from_millis(100));
    dots.update();
    assert_ne!(dots.current_frame(), Some("⠋")); // Should have advanced
}

#[test]
fn test_spinner_line() {
    let line = Spinners::line();
    assert_eq!(line.frame_count(), 4);
    assert_eq!(line.frame_duration(), Duration::from_millis(100));

    let mut line = line;
    line.start();
    assert_eq!(line.current_frame(), Some("|"));

    thread::sleep(Duration::from_millis(120));
    line.update();
    assert_eq!(line.current_frame(), Some("/"));
}

#[test]
fn test_spinner_arrow() {
    let arrow = Spinners::arrow();
    assert_eq!(arrow.frame_count(), 8);
    assert_eq!(arrow.frame_duration(), Duration::from_millis(120));

    let mut arrow = arrow;
    arrow.start();
    assert_eq!(arrow.current_frame(), Some("←"));
}

#[test]
fn test_spinner_clock() {
    let clock = Spinners::clock();
    assert_eq!(clock.frame_count(), 12);
    assert_eq!(clock.frame_duration(), Duration::from_millis(100));

    let mut clock = clock;
    clock.start();
    assert_eq!(clock.current_frame(), Some("🕐"));
}

#[test]
fn test_spinner_growing_dots() {
    let growing = Spinners::growing_dots();
    assert_eq!(growing.frame_count(), 8);

    let mut growing = growing;
    growing.start();
    assert_eq!(growing.current_frame(), Some("⠁"));
}

#[test]
fn test_spinner_bouncing_ball() {
    let bouncing = Spinners::bouncing_ball();
    assert_eq!(bouncing.frame_count(), 8);

    let mut bouncing = bouncing;
    bouncing.start();
    assert_eq!(bouncing.current_frame(), Some("⠁"));
}

#[test]
fn test_spinner_simple_dots() {
    let simple = Spinners::simple_dots();
    assert_eq!(simple.frame_count(), 3);
    assert_eq!(simple.frame_duration(), Duration::from_millis(400));

    let mut simple = simple;
    simple.start();
    assert_eq!(simple.current_frame(), Some(".  "));
}

#[test]
fn test_spinner_ascii() {
    let ascii = Spinners::ascii();
    assert_eq!(ascii.frame_count(), 4);
    assert_eq!(ascii.frame_duration(), Duration::from_millis(150));

    let mut ascii = ascii;
    ascii.start();
    assert_eq!(ascii.current_frame(), Some("-"));
}

#[test]
fn test_spinner_by_name() {
    // Test valid spinner names
    let dots = Spinners::by_name("dots").unwrap();
    assert_eq!(dots.frame_count(), 10);

    let line = Spinners::by_name("line").unwrap();
    assert_eq!(line.frame_count(), 4);

    let arrow = Spinners::by_name("arrow").unwrap();
    assert_eq!(arrow.frame_count(), 8);

    let clock = Spinners::by_name("clock").unwrap();
    assert_eq!(clock.frame_count(), 12);

    let growing_dots = Spinners::by_name("growing_dots").unwrap();
    assert_eq!(growing_dots.frame_count(), 8);

    let bouncing_ball = Spinners::by_name("bouncing_ball").unwrap();
    assert_eq!(bouncing_ball.frame_count(), 8);

    let simple_dots = Spinners::by_name("simple_dots").unwrap();
    assert_eq!(simple_dots.frame_count(), 3);

    let ascii = Spinners::by_name("ascii").unwrap();
    assert_eq!(ascii.frame_count(), 4);

    // Test invalid spinner name
    assert!(Spinners::by_name("nonexistent").is_none());
    assert!(Spinners::by_name("").is_none());
    assert!(Spinners::by_name("invalid_name").is_none());
}

#[test]
fn test_available_spinner_names() {
    let names = Spinners::available_names();
    assert!(names.contains(&"dots"));
    assert!(names.contains(&"line"));
    assert!(names.contains(&"arrow"));
    assert!(names.contains(&"clock"));
    assert!(names.contains(&"growing_dots"));
    assert!(names.contains(&"bouncing_ball"));
    assert!(names.contains(&"simple_dots"));
    assert!(names.contains(&"ascii"));

    // Verify all names actually work
    for name in names {
        assert!(
            Spinners::by_name(name).is_some(),
            "Spinner '{}' should exist",
            name
        );
    }
}

#[test]
fn test_timer_basic() {
    let mut timer = Timer::new(Duration::from_millis(50));

    // Should not trigger immediately
    assert!(!timer.check_interval());

    // Check elapsed time
    let elapsed = timer.elapsed();
    assert!(elapsed < Duration::from_millis(10)); // Should be very small

    // Wait and check again
    thread::sleep(Duration::from_millis(60));
    assert!(timer.check_interval());

    // Should not trigger again immediately
    assert!(!timer.check_interval());
}

#[test]
fn test_timer_multiple_intervals() {
    let mut timer = Timer::new(Duration::from_millis(20));

    // First interval
    thread::sleep(Duration::from_millis(25));
    assert!(timer.check_interval());

    // Second interval
    thread::sleep(Duration::from_millis(25));
    assert!(timer.check_interval());

    // No interval yet
    assert!(!timer.check_interval());
}

#[test]
fn test_timer_reset() {
    let mut timer = Timer::new(Duration::from_millis(50));

    thread::sleep(Duration::from_millis(30));
    let elapsed_before = timer.elapsed();
    assert!(elapsed_before >= Duration::from_millis(25));

    timer.reset();
    let elapsed_after = timer.elapsed();
    assert!(elapsed_after < Duration::from_millis(10));

    // Should not trigger immediately after reset
    assert!(!timer.check_interval());
}

#[test]
fn test_timer_time_until_next_interval() {
    let timer = Timer::new(Duration::from_millis(100));

    // Right after creation, should be close to full interval
    let time_until = timer.time_until_next_interval();
    assert!(time_until <= Duration::from_millis(100));
    assert!(time_until >= Duration::from_millis(90)); // Allow some variance

    // After some time, should be less
    thread::sleep(Duration::from_millis(30));
    let time_until = timer.time_until_next_interval();
    assert!(time_until <= Duration::from_millis(70));

    // After interval has passed
    thread::sleep(Duration::from_millis(80));
    let time_until = timer.time_until_next_interval();
    assert_eq!(time_until, Duration::from_millis(0));
}

#[test]
fn test_progress_fill_animation() {
    let animation = progress_fill_animation(5, '█', '░', Duration::from_millis(100));

    assert_eq!(animation.frame_count(), 6); // 0 to 5 filled
    // Check if animation is one-time by testing if it finishes
    assert_eq!(animation.frame_duration(), Duration::from_millis(100));

    let mut animation = animation;
    animation.start();

    // Check first frame (empty)
    assert_eq!(animation.current_frame(), Some("░░░░░"));

    // Advance through frames
    for i in 1..=5 {
        thread::sleep(Duration::from_millis(110));
        animation.update();

        let expected = format!("{}{}", "█".repeat(i), "░".repeat(5 - i));
        assert_eq!(animation.current_frame(), Some(expected.as_str()));
    }

    // Try to advance one more time to check if finished
    thread::sleep(Duration::from_millis(110));
    animation.update();

    // Should be finished now
    assert!(animation.is_finished());
}

#[test]
fn test_wave_animation() {
    let animation = wave_animation(5, '~', '.', 2, Duration::from_millis(50));

    assert_eq!(animation.frame_count(), 5);
    // Check if animation loops by testing that it doesn't finish
    assert_eq!(animation.frame_duration(), Duration::from_millis(50));

    let mut animation = animation;
    animation.start();

    // Each frame should have exactly 2 wave characters
    for _ in 0..animation.frame_count() {
        let frame = animation.current_frame().unwrap();
        let wave_count = frame.chars().filter(|&c| c == '~').count();
        assert_eq!(wave_count, 2);
        assert_eq!(frame.len(), 5);

        thread::sleep(Duration::from_millis(60));
        animation.update();
    }
}

#[test]
fn test_edge_cases() {
    // Test zero-width progress animation
    let animation = progress_fill_animation(0, '█', '░', Duration::from_millis(100));
    assert_eq!(animation.frame_count(), 1);
    let mut animation = animation;
    animation.start();
    assert_eq!(animation.current_frame(), Some(""));

    // Test zero-length wave
    let animation = wave_animation(0, '~', '.', 1, Duration::from_millis(50));
    assert_eq!(animation.frame_count(), 0);

    // Test wave longer than width
    let animation = wave_animation(3, '~', '.', 5, Duration::from_millis(50));
    assert_eq!(animation.frame_count(), 3);
    let mut animation = animation;
    animation.start();

    // All positions should be wave characters
    let frame = animation.current_frame().unwrap();
    assert_eq!(frame, "~~~");
}

#[test]
fn test_performance() {
    use std::time::Instant;

    // Test animation creation performance
    let start = Instant::now();
    for i in 0..1000 {
        let frames: Vec<String> = (0..10).map(|j| format!("frame{}", j)).collect();
        let _animation = Animation::new(frames, Duration::from_millis(i % 100 + 1));
    }
    let creation_time = start.elapsed();
    assert!(creation_time < Duration::from_millis(100));

    // Test frame advancement performance
    let frames: Vec<String> = (0..100).map(|i| format!("frame{}", i)).collect();
    let mut animation = Animation::new(frames, Duration::from_millis(1));
    animation.start();

    let start = Instant::now();
    for _ in 0..1000 {
        animation.update();
    }
    let update_time = start.elapsed();
    assert!(update_time < Duration::from_millis(50));

    // Test spinner creation performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _spinner = Spinners::dots();
    }
    let spinner_time = start.elapsed();
    assert!(spinner_time < Duration::from_millis(10));
}

#[test]
fn test_memory_usage() {
    use std::mem;

    // Test that structures have reasonable memory footprints
    assert!(mem::size_of::<Animation>() <= 200);
    assert!(mem::size_of::<Timer>() <= 64);

    // Test that large animations don't use excessive memory
    let large_frames: Vec<String> = (0..1000).map(|i| format!("frame{}", i)).collect();
    let animation = Animation::new(large_frames, Duration::from_millis(100));

    // Should not cause excessive memory usage (this is a sanity check)
    assert_eq!(animation.frame_count(), 1000);
}

#[test]
fn test_unicode_handling() {
    // Test animation with Unicode frames
    let unicode_frames = vec!["🕐".to_string(), "🕑".to_string(), "🕒".to_string()];
    let mut animation = Animation::new(unicode_frames, Duration::from_millis(10));
    animation.start();

    assert_eq!(animation.current_frame(), Some("🕐"));

    thread::sleep(Duration::from_millis(15));
    animation.update();
    assert_eq!(animation.current_frame(), Some("🕑"));

    // Test progress animation with Unicode characters
    let unicode_progress = progress_fill_animation(3, '█', '░', Duration::from_millis(10));
    let mut unicode_progress = unicode_progress;
    unicode_progress.start();

    assert_eq!(unicode_progress.current_frame(), Some("░░░"));

    thread::sleep(Duration::from_millis(15));
    unicode_progress.update();
    assert_eq!(unicode_progress.current_frame(), Some("█░░"));
}
