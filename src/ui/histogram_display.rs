use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use crate::core::histogram::HistogramStats;

#[allow(dead_code)]
const HIST_WIDTH: usize = 4; // Width of each histogram bar

/// Render a histogram in the terminal
#[allow(dead_code)]
pub fn render_histogram<B: Backend>(
    frame: &mut Frame<B>,
    stats: &HistogramStats,
    area: Rect,
    title: &str,
    is_wpm: bool,
) {
    let max_count = stats.total_distribution.iter()
        .chain(stats.current_distribution.iter())
        .chain(stats.avg_10s_distribution.iter())
        .chain(stats.avg_60s_distribution.iter())
        .max()
        .copied()
        .unwrap_or(1)
        .max(1);

    let mut display_text = Vec::new();

    // Header
    let header = format!("{} Distribution:", title);
    display_text.push(Spans::from(vec![
        Span::styled(header, Style::default().add_modifier(Modifier::BOLD))
    ]));

    // Running Averages
    let unit = if is_wpm { "WPM" } else { "ms" };
    let running_stats = format!(
        "10s Avg: {:.1}{} | 60s Avg: {:.1}{} | Geo Avg: {:.1}{}",
        stats.running_10s_avg, unit,
        stats.running_60s_avg, unit,
        stats.running_geo_avg, unit
    );
    display_text.push(Spans::from(vec![
        Span::styled(running_stats, Style::default().fg(Color::Yellow))
    ]));

    // Range header
    let mut range_header = String::from("Range   │");
    for range in &stats.ranges {
        let range_text = if range.max.is_infinite() {
            format!("{:>4}+  ", range.min as usize)
        } else {
            format!("{:>2}-{:<3}", range.min as usize, range.max as usize)
        };
        range_header.push_str(&format!("{:^width$}", range_text, width = HIST_WIDTH + 3));
    }
    display_text.push(Spans::from(vec![Span::raw(range_header.clone())]));

    // Separator
    let separator = "─".repeat(range_header.len());
    display_text.push(Spans::from(vec![Span::raw(separator)]));

    // Distribution rows
    let distributions = [
        ("Total  ", &stats.total_distribution, Color::Green),
        ("Current", &stats.current_distribution, Color::Yellow),
        ("10s Avg", &stats.avg_10s_distribution, Color::Blue),
        ("60s Avg", &stats.avg_60s_distribution, Color::Cyan),
    ];

    for (label, dist, color) in distributions {
        let mut row = format!("{} │", label);
        for &count in dist {
            let bar_height = (count as f64 / max_count as f64 * HIST_WIDTH as f64) as usize;
            let bar = "█".repeat(bar_height).pad_right(HIST_WIDTH);
            row.push_str(&format!(" {:>width$} ", bar, width = HIST_WIDTH + 1));
        }
        display_text.push(Spans::from(vec![
            Span::styled(row, Style::default().fg(color))
        ]));
    }

    // Stats summary
    let stats_text = format!(
        "Min: {:.1}{} | Max: {:.1}{} | Art Mean: {:.1}{}",
        stats.min_value, unit,
        stats.max_value, unit,
        stats.arithmetic_mean, unit
    );
    display_text.push(Spans::from(vec![
        Span::styled(stats_text, Style::default().add_modifier(Modifier::BOLD))
    ]));

    let paragraph = Paragraph::new(display_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());

    frame.render_widget(paragraph, area);
}

#[allow(dead_code)]
trait StrPadding {
    fn pad_right(self, width: usize) -> String;
}

impl StrPadding for String {
    fn pad_right(self, width: usize) -> String {
        if self.len() >= width {
            self
        } else {
            let padding = " ".repeat(width - self.len());
            format!("{}{}", self, padding)
        }
    }
} 