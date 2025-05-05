use std::io::{self, Write, stdout};
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    cursor::{Hide, Show, MoveTo},
    style::{Color, SetForegroundColor, SetBackgroundColor, ResetColor},
    event::{poll, read, Event},
};

const FRAME_TIME: u64 = 10; // Animation frame time in milliseconds

// Reduced symbol set that works well at any size
const SYMBOLS: &[char] = &[
    '█', '▀', '▄', '▌',
];

// 8-bit rainbow colors for diagonal transition
const COLORS: &[(u8, u8, u8)] = &[
    (255, 0, 0),     // Red
    (255, 127, 0),   // Orange
    (255, 255, 0),   // Yellow
    (0, 255, 0),     // Green
    (0, 0, 255),     // Blue
    (75, 0, 130),    // Indigo
    (148, 0, 211),   // Violet
];

#[derive(Copy, Clone)]
enum Direction {
    Left,
}

struct Pattern {
    x_offset: f64,
    y_offset: f64,
    direction: Direction,
    speed: f64,
}

impl Pattern {
    fn new(_box_size: u16) -> Self {
        Self {
            x_offset: 0.0,
            y_offset: 0.0,
            direction: Direction::Left,
            speed: 1.0,
        }
    }

    fn update(&mut self, box_size: u16) {
        let max_offset = box_size.saturating_sub(2) as f64;
        match self.direction {
            Direction::Left => {
                self.x_offset = (self.x_offset - self.speed).rem_euclid(max_offset)
            },
        }
    }

    fn get_offset(&self, x: u16, y: u16, box_size: u16) -> (usize, usize) {
        let max_offset = box_size.saturating_sub(2) as f64;
        let x_pos = ((x as f64 + self.x_offset).rem_euclid(max_offset)).max(0.0);
        let y_pos = ((y as f64 + self.y_offset).rem_euclid(max_offset)).max(0.0);
        (x_pos.floor() as usize, y_pos.floor() as usize)
    }
}

fn draw_pattern(
    stdout: &mut io::Stdout,
    term_width: u16,
    term_height: u16,
    pattern: &Pattern,
    color_phase: f64,
    box_size: u16,
) -> io::Result<()> {
    let start_x = (term_width.saturating_sub(box_size)) / 2;
    let start_y = (term_height.saturating_sub(box_size)) / 2;
    
    // Draw a border around the animation box
    let border_color = Color::Rgb { r: 150, g: 150, b: 150 }; // Brighter border
    
    // Only draw title if there's enough space
    if term_height > box_size + 1 {
        let title = format!("VGA {}x{}", box_size, box_size);
        let title_x = if title.len() as u16 > term_width {
            0
        } else {
            (term_width.saturating_sub(title.len() as u16)) / 2
        };
        execute!(stdout, MoveTo(title_x, start_y.saturating_sub(1)), SetForegroundColor(border_color))?;
        write!(stdout, "{}", title)?;
    }
    
    // Draw borders
    for x in start_x..start_x.saturating_add(box_size) {
        execute!(stdout, MoveTo(x, start_y), SetForegroundColor(border_color))?;
        write!(stdout, "-")?;
        execute!(stdout, MoveTo(x, start_y.saturating_add(box_size.saturating_sub(1))), SetForegroundColor(border_color))?;
        write!(stdout, "-")?;
    }
    
    for y in start_y..start_y.saturating_add(box_size) {
        execute!(stdout, MoveTo(start_x, y), SetForegroundColor(border_color))?;
        write!(stdout, "|")?;
        execute!(stdout, MoveTo(start_x.saturating_add(box_size.saturating_sub(1)), y), SetForegroundColor(border_color))?;
        write!(stdout, "|")?;
    }
    
    // Draw the animated pattern inside the box
    for y in 0..box_size.saturating_sub(2) {
        for x in 0..box_size.saturating_sub(2) {
            let (offset_x, offset_y) = pattern.get_offset(x, y, box_size);
            let symbol = SYMBOLS[(offset_x + offset_y) % SYMBOLS.len()];
            
            let position_t = ((x as f64 / box_size as f64) + (y as f64 / box_size as f64)) / 2.0;
            let color_shift = (position_t * 0.2) - 0.1;
            let adjusted_t = (color_phase + color_shift).max(0.0).min(1.0);
            
            let color_index = (adjusted_t * (COLORS.len() - 1) as f64).floor() as usize;
            let next_color_index = (color_index + 1) % COLORS.len();
            let t = (adjusted_t * (COLORS.len() - 1) as f64).fract();
            
            let color = interpolate_color(COLORS[color_index], COLORS[next_color_index], t);
            
            execute!(
                stdout,
                MoveTo(start_x.saturating_add(x.saturating_add(1)), start_y.saturating_add(y.saturating_add(1))),
                SetForegroundColor(color),
                SetBackgroundColor(Color::Black)
            )?;
            
            write!(stdout, "{}", symbol)?;
        }
    }
    
    stdout.flush()?;
    Ok(())
}

fn interpolate_color(color1: (u8, u8, u8), color2: (u8, u8, u8), t: f64) -> Color {
    let r = (color1.0 as f64 * (1.0 - t) + color2.0 as f64 * t) as u8;
    let g = (color1.1 as f64 * (1.0 - t) + color2.1 as f64 * t) as u8;
    let b = (color1.2 as f64 * (1.0 - t) + color2.2 as f64 * t) as u8;
    Color::Rgb { r, g, b }
}

pub fn run_test_screen() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    
    // Use fixed dimensions for testing
    let term_width = 40;
    let term_height = 20;
    let box_size = 16; // Fixed box size for consistent display

    execute!(stdout, Hide)?;
    execute!(stdout, Clear(ClearType::All))?;
    
    let mut pattern = Pattern::new(box_size);
    let mut color_phase = 0.0;
    let color_speed = 0.02;
    
    // Animation sequence - 200 frames = 2 seconds at 10ms per frame
    for _ in 0..200 {
        if poll(Duration::from_millis(FRAME_TIME))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
        
        draw_pattern(&mut stdout, term_width, term_height, &pattern, color_phase, box_size)?;
        pattern.update(box_size);
        color_phase = (color_phase + color_speed) % COLORS.len() as f64;
    }
    
    // Reset terminal
    execute!(stdout, Clear(ClearType::All), ResetColor, Show, MoveTo(0, 0))?;
    disable_raw_mode()?;
    Ok(())
} 