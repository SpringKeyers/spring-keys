use std::io::{self, Write, stdout};
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, size, enable_raw_mode, disable_raw_mode},
    cursor::{Hide, Show, MoveTo},
    style::{Color, SetForegroundColor, SetBackgroundColor, ResetColor},
    event::{poll, read, Event},
};

// Terminal size constants
const MIN_BOX_SIZE: u16 = 4;
const MIN_TERM_WIDTH: u16 = 4;
const MIN_TERM_HEIGHT: u16 = 4;
const BORDER_SPACE: u16 = 2; // Space for borders and padding

// Reduced symbol set that works well at any size
const SYMBOLS: &[char] = &[
    '█', '▀', '▄', '▌',
];

// Brighter colors for better visibility
const COLORS: &[(u8, u8, u8)] = &[
    (255, 0, 0),     // Bright Red
    (0, 255, 0),     // Bright Green
    (0, 0, 255),     // Blue
    (255, 255, 0),   // Yellow
];

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

struct Pattern {
    x_offset: f64,
    y_offset: f64,
    direction: Direction,
    speed: f64,
}

fn calculate_box_size(term_width: u16, term_height: u16) -> u16 {
    // Calculate maximum possible box size while maintaining aspect ratio
    let max_width = term_width.saturating_sub(BORDER_SPACE);
    let max_height = term_height.saturating_sub(BORDER_SPACE);
    let size = max_width.min(max_height);
    
    // Ensure size is at least MIN_BOX_SIZE
    size.max(MIN_BOX_SIZE)
}

impl Pattern {
    fn new(box_size: u16) -> Self {
        Self {
            x_offset: 0.0,
            y_offset: 0.0,
            direction: Direction::Left,
            speed: 0.25, // Consistent speed regardless of size
        }
    }

    fn update(&mut self, box_size: u16) {
        let max_offset = box_size.saturating_sub(2) as f64;
        match self.direction {
            Direction::Left => {
                self.x_offset = (self.x_offset - self.speed).rem_euclid(max_offset)
            },
            Direction::Up => {
                self.y_offset = (self.y_offset - self.speed).rem_euclid(max_offset)
            },
            Direction::Down => {
                self.y_offset = (self.y_offset + self.speed).rem_euclid(max_offset)
            },
            Direction::Right => {
                self.x_offset = (self.x_offset + self.speed).rem_euclid(max_offset)
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
    
    // Get terminal size immediately
    let (term_width, term_height) = size()?;
    
    // Check terminal size and exit if too small
    if term_width < MIN_TERM_WIDTH || term_height < MIN_TERM_HEIGHT {
        disable_raw_mode()?;
        println!("Exiting due to insufficient terminal size ({}x{}, need {}x{})",
            term_width, term_height, MIN_TERM_WIDTH, MIN_TERM_HEIGHT);
        return Ok(());
    }

    execute!(stdout, Hide)?;
    execute!(stdout, Clear(ClearType::All))?;
    
    // Calculate box size based on terminal dimensions
    let box_size = calculate_box_size(term_width, term_height);
    
    let mut pattern = Pattern::new(box_size);
    let mut color_phase = 0.0;
    let color_speed = 0.05; // Slower color changes for better visibility
    
    // Animation sequence
    for frame in 0..200 { // 2 seconds at 10ms per frame
        if poll(Duration::from_millis(FRAME_TIME))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
        
        // Change direction every 50 frames (0.5 second)
        if frame % 50 == 0 {
            pattern.direction = match pattern.direction {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Left,
            };
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