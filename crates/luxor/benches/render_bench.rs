use criterion::{Criterion, criterion_group, criterion_main};
use luxor::{Color, Console, ConsoleOptions, Style, Text};

fn benchmark_text_rendering(c: &mut Criterion) {
    let console = Console::new();
    let options = ConsoleOptions::new();

    c.bench_function("render simple text", |b| {
        let text = Text::new("Hello, world!");
        b.iter(|| {
            let _segments = text.render(&console, &options).unwrap();
        });
    });

    c.bench_function("render styled text", |b| {
        let text =
            Text::new("Hello, world!").style(Style::new().bold().color(Color::rgb(255, 0, 0)));
        b.iter(|| {
            let _segments = text.render(&console, &options).unwrap();
        });
    });
}

fn benchmark_style_operations(c: &mut Criterion) {
    c.bench_function("style creation", |b| {
        b.iter(|| {
            let _style = Style::new()
                .bold()
                .italic()
                .color(Color::rgb(255, 0, 0))
                .background(Color::rgb(0, 255, 0));
        });
    });

    c.bench_function("style combination", |b| {
        let base = Style::new().bold().color(Color::rgb(255, 0, 0));
        let overlay = Style::new().italic().background(Color::rgb(0, 255, 0));
        b.iter(|| {
            let _combined = base.clone().combine(overlay.clone());
        });
    });
}

criterion_group!(
    benches,
    benchmark_text_rendering,
    benchmark_style_operations
);
criterion_main!(benches);
