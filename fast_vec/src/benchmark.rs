// Times your vectors and draws two plots, benchmark_slow.png and benchmark_growth.png.
//
// Run it in release mode, or it will take far longer and the numbers will mislead you:
//
//     cargo run --release --bin benchmark
//
// main() is yours to change: it picks what gets compared. The plotting code below
// it you can leave alone.

use fast_vec::timing::{time_fast_pushes, time_slow_pushes};
use fast_vec::{FastVec, Growth};

use plotters::prelude::*;
use plotters::style::full_palette::{GREEN, ORANGE, PURPLE};

fn main() {
    // Plot 1: SlowVec against FastVec. SlowVec is slow enough that a small n
    // is all it can manage.
    let n = 5_000;
    let every = 100;
    let slow = time_slow_pushes(n, every);
    let fast = time_fast_pushes(FastVec::new(), n, every);
    plot(
        "benchmark_slow.png",
        "SlowVec vs FastVec",
        vec![("SlowVec", slow), ("FastVec, Double", fast)],
    );

    // Plot 2: growth strategies against each other, at a much bigger n.
    // Which strategies, and which knob settings, is your choice. Up to four.
    let n = 1_000_000;
    let every = 10_000;
    let mut series = Vec::new();
    series.push(("Double", time_fast_pushes(FastVec::with_growth(Growth::Double), n, every)));
    // YOUR CHOICES GO HERE.

    plot("benchmark_growth.png", "Growth strategies", series);
}

// ---------------------------------------------------------------------------
// Plotting. You won't need to edit anything below this line.
// ---------------------------------------------------------------------------

fn plot(filename: &str, title: &str, series: Vec<(&str, Vec<(usize, f64)>)>) {
    let mut max_x = 1.0f32;
    let mut max_y = 1.0f32;
    for (_, points) in &series {
        for (x, y) in points {
            max_x = max_x.max(*x as f32);
            max_y = max_y.max(*y as f32);
        }
    }

    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    let root = root.margin(10, 10, 10, 10);
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(0f32..max_x * 1.05, 0f32..max_y * 1.1).unwrap();

    chart
        .configure_mesh()
        .x_desc("elements pushed")
        .y_desc("total milliseconds")
        .x_labels(8)
        .y_labels(10)
        .x_label_formatter(&|x| format!("{}", *x as u64))
        .label_style(("sans-serif", 18))
        .draw().unwrap();

    let colors = [BLUE, ORANGE, GREEN, PURPLE];
    for (i, (name, points)) in series.into_iter().enumerate() {
        let style = ShapeStyle::from(&colors[i % colors.len()]).stroke_width(2).filled();
        let points: Vec<(f32, f32)> = points.iter().map(|(x, y)| (*x as f32, *y as f32)).collect();
        chart.draw_series(LineSeries::new(points, style.clone()).point_size(2)).unwrap()
            .label(name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style));
    }

    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(&BLACK)
        .label_font(("sans-serif", 20))
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    root.present().unwrap();
    println!("Wrote {}", filename);
}
