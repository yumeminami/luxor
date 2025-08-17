//! Comprehensive benchmarks for Luxor library.

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use luxor::{
    Color, ColorSystem, Console, ConsoleOptions, Measurable, Measurement, Renderable, Segment,
    Style, Text,
};

/// Benchmark text rendering performance.
fn benchmark_text_rendering(c: &mut Criterion) {
    let console = Console::new();
    let options = ConsoleOptions::new();

    // Benchmark simple text rendering
    c.bench_function("render simple text", |b| {
        let text = Text::new("Hello, world!");
        b.iter(|| {
            let _segments = text.render(&console, &options).unwrap();
        });
    });

    // Benchmark styled text rendering
    c.bench_function("render styled text", |b| {
        let text =
            Text::new("Hello, world!").with_style(Style::new().bold().color(Color::rgb(255, 0, 0)));
        b.iter(|| {
            let _segments = text.render(&console, &options).unwrap();
        });
    });

    // Benchmark complex styled text
    c.bench_function("render complex styled text", |b| {
        let text = Text::new("Complex styled text with many attributes").with_style(
            Style::new()
                .bold()
                .italic()
                .underline()
                .color(Color::rgb(255, 128, 64))
                .background(Color::rgb(64, 128, 255)),
        );
        b.iter(|| {
            let _segments = text.render(&console, &options).unwrap();
        });
    });

    // Benchmark large text rendering
    let large_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(100);
    c.bench_function("render large text", |b| {
        let text = Text::new(&large_text);
        b.iter(|| {
            let _segments = text.render(&console, &options).unwrap();
        });
    });
}

/// Benchmark style operations.
fn benchmark_style_operations(c: &mut Criterion) {
    // Benchmark style creation
    c.bench_function("create simple style", |b| {
        b.iter(|| {
            let _style = Style::new().bold().color(Color::rgb(255, 0, 0));
        });
    });

    c.bench_function("create complex style", |b| {
        b.iter(|| {
            let _style = Style::new()
                .bold()
                .italic()
                .underline()
                .strikethrough()
                .dim()
                .reverse()
                .blink()
                .hidden()
                .color(Color::rgb(255, 128, 64))
                .background(Color::rgb(64, 128, 255));
        });
    });

    // Benchmark style combination
    c.bench_function("combine styles", |b| {
        let base = Style::new().bold().color(Color::rgb(255, 0, 0));
        let overlay = Style::new().italic().background(Color::rgb(0, 255, 0));
        b.iter(|| {
            let _combined = base.clone().combine(overlay.clone());
        });
    });

    // Benchmark style parsing
    c.bench_function("parse simple style", |b| {
        b.iter(|| {
            let _style = Style::parse("bold red").unwrap();
        });
    });

    c.bench_function("parse complex style", |b| {
        b.iter(|| {
            let _style = Style::parse("bold italic underline red on blue").unwrap();
        });
    });
}

/// Benchmark color operations.
fn benchmark_color_operations(c: &mut Criterion) {
    // Benchmark color creation
    c.bench_function("create rgb color", |b| {
        b.iter(|| {
            let _color = Color::rgb(255, 128, 64);
        });
    });

    // Benchmark color conversion
    c.bench_function("rgb to 8bit conversion", |b| {
        b.iter(|| {
            let _index = Color::rgb_to_eight_bit(255, 128, 64);
        });
    });

    // Benchmark color downgrading
    c.bench_function("color downgrade to standard", |b| {
        let color = Color::rgb(255, 128, 64);
        b.iter(|| {
            let _downgraded = color.downgrade(ColorSystem::Standard);
        });
    });

    c.bench_function("color downgrade to 8bit", |b| {
        let color = Color::rgb(255, 128, 64);
        b.iter(|| {
            let _downgraded = color.downgrade(ColorSystem::EightBit);
        });
    });

    // Benchmark hex parsing
    c.bench_function("parse hex color", |b| {
        b.iter(|| {
            let _color = Color::from_hex("#FF8040").unwrap();
        });
    });
}

/// Benchmark segment operations.
fn benchmark_segment_operations(c: &mut Criterion) {
    let style = Style::new().bold().color(Color::rgb(255, 0, 0));

    // Benchmark segment creation
    c.bench_function("create segment", |b| {
        b.iter(|| {
            let _segment = Segment::new("Hello, world!".to_string(), style.clone());
        });
    });

    // Benchmark segment splitting
    c.bench_function("split segment at char", |b| {
        let segment = Segment::new("Hello, world!".to_string(), style.clone());
        b.iter(|| {
            let _result = segment.clone().split_at_char(5);
        });
    });

    c.bench_function("split segment at width", |b| {
        let segment = Segment::new("Hello, world!".to_string(), style.clone());
        b.iter(|| {
            let _result = segment.clone().split_at_width(5);
        });
    });

    // Benchmark segment rendering
    c.bench_function("render segment", |b| {
        let segment = Segment::new("Hello, world!".to_string(), style.clone());
        b.iter(|| {
            let _output = segment.render(ColorSystem::TrueColor);
        });
    });

    // Benchmark cell length calculation
    c.bench_function("calculate cell length", |b| {
        let segment = Segment::new("Hello, 世界! 👋".to_string(), style.clone());
        b.iter(|| {
            let _length = segment.cell_length();
        });
    });
}

/// Benchmark measurement operations.
fn benchmark_measurement_operations(c: &mut Criterion) {
    // Benchmark measurement creation
    c.bench_function("create measurement", |b| {
        b.iter(|| {
            let _measurement = Measurement::new(10, 50);
        });
    });

    // Benchmark measurement operations
    c.bench_function("measurement add_with", |b| {
        let m1 = Measurement::new(10, 30);
        let m2 = Measurement::new(5, 20);
        b.iter(|| {
            let _result = m1.add_with(m2);
        });
    });

    c.bench_function("measurement max_with", |b| {
        let m1 = Measurement::new(10, 30);
        let m2 = Measurement::new(15, 25);
        b.iter(|| {
            let _result = m1.max_with(m2);
        });
    });

    c.bench_function("measurement clamp", |b| {
        let measurement = Measurement::new(10, 50);
        b.iter(|| {
            let _result = measurement.clamp(20, 40);
        });
    });
}

/// Benchmark ANSI operations.
fn benchmark_ansi_operations(c: &mut Criterion) {
    use luxor::ansi::{strip_ansi, style_to_ansi, text_width};

    // Benchmark ANSI escape generation
    c.bench_function("generate ansi escape", |b| {
        let style = Style::new().bold().color(Color::rgb(255, 0, 0));
        b.iter(|| {
            let _ansi = style_to_ansi(&style, ColorSystem::TrueColor);
        });
    });

    // Benchmark ANSI stripping
    let ansi_text = "\x1b[1;31mHello\x1b[0m \x1b[32mWorld\x1b[0m";
    c.bench_function("strip ansi sequences", |b| {
        b.iter(|| {
            let _stripped = strip_ansi(ansi_text);
        });
    });

    // Benchmark text width calculation
    c.bench_function("calculate text width", |b| {
        b.iter(|| {
            let _width = text_width(ansi_text);
        });
    });

    // Benchmark with complex ANSI sequences
    let complex_ansi = "\x1b[1;31;42m\x1b[4mComplex\x1b[24m\x1b[0m \x1b[3;36mANSI\x1b[23;39m";
    c.bench_function("strip complex ansi", |b| {
        b.iter(|| {
            let _stripped = strip_ansi(complex_ansi);
        });
    });
}

/// Benchmark different color systems.
fn benchmark_color_systems(c: &mut Criterion) {
    let console = Console::new();
    let text =
        Text::new("Benchmark text").with_style(Style::new().bold().color(Color::rgb(255, 128, 64)));

    let mut group = c.benchmark_group("color_systems");

    for &color_system in &[
        ColorSystem::Standard,
        ColorSystem::EightBit,
        ColorSystem::TrueColor,
    ] {
        let options = ConsoleOptions::new().with_color_system(color_system);

        group.bench_with_input(
            BenchmarkId::new("render", format!("{:?}", color_system)),
            &color_system,
            |b, _| {
                b.iter(|| {
                    let _segments = text.render(&console, &options).unwrap();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark scaling with text size.
fn benchmark_text_scaling(c: &mut Criterion) {
    let console = Console::new();
    let options = ConsoleOptions::new();

    let mut group = c.benchmark_group("text_scaling");

    for &size in &[10, 100, 1000, 10000] {
        let text_content = "a".repeat(size);
        let text = Text::new(&text_content);

        group.bench_with_input(BenchmarkId::new("render", size), &size, |b, _| {
            b.iter(|| {
                let _segments = text.render(&console, &options).unwrap();
            });
        });

        group.bench_with_input(BenchmarkId::new("measure", size), &size, |b, _| {
            b.iter(|| {
                let _measurement = text.measure(&console, &options).unwrap();
            });
        });
    }

    group.finish();
}

/// Benchmark memory allocation patterns.
fn benchmark_allocation_patterns(c: &mut Criterion) {
    // Benchmark creating many styles (tests allocation patterns)
    c.bench_function("create many styles", |b| {
        b.iter_batched(
            || (), // Setup
            |_| {
                let _styles: Vec<Style> = (0..1000)
                    .map(|i| {
                        Style::new().bold().color(Color::rgb(
                            (i % 256) as u8,
                            ((i / 256) % 256) as u8,
                            ((i / 65536) % 256) as u8,
                        ))
                    })
                    .collect();
            },
            BatchSize::SmallInput,
        );
    });

    // Benchmark creating many segments
    c.bench_function("create many segments", |b| {
        let style = Style::new().bold();
        b.iter_batched(
            || (), // Setup
            |_| {
                let _segments: Vec<Segment> = (0..1000)
                    .map(|i| Segment::new(format!("Segment {}", i), style.clone()))
                    .collect();
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    benchmark_text_rendering,
    benchmark_style_operations,
    benchmark_color_operations,
    benchmark_segment_operations,
    benchmark_measurement_operations,
    benchmark_ansi_operations,
    benchmark_color_systems,
    benchmark_text_scaling,
    benchmark_allocation_patterns
);

criterion_main!(benches);
