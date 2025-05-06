use crossterm::{
    style::{Color, Print, SetForegroundColor, ResetColor},
    cursor::MoveTo,
    queue,
};
use std::io::{self, Write};
use crate::TypingMetrics;

pub fn draw_row_performance(
    stdout: &mut impl Write,
    metrics: &TypingMetrics,
    y_offset: u16
) -> io::Result<()> {
    // Draw header
    queue!(
        stdout,
        MoveTo(0, y_offset),
        SetForegroundColor(Color::White),
        Print("Row Performance:"),
        ResetColor
    )?;

    // Draw row metrics
    let rows = [
        ("Numbers", &metrics.number_metrics),
        ("Top Row", &metrics.top_row_metrics),
        ("Home Row", &metrics.home_row_metrics),
        ("Bottom Row", &metrics.bottom_row_metrics),
    ];

    for (i, (label, metrics)) in rows.iter().enumerate() {
        let y = y_offset + 1 + i as u16;
        
        let color = if metrics.avg_time_ms < 150.0 {
            Color::Green
        } else if metrics.avg_time_ms < 250.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        queue!(
            stdout,
            MoveTo(0, y),
            Print(label),
            SetForegroundColor(color),
            Print(format!(" {:3.0}ms", metrics.avg_time_ms)),
            ResetColor
        )?;
    }

    Ok(())
} 