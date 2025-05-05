use std::io::{self, Write, stdout};
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, size, enable_raw_mode, disable_raw_mode},
    cursor::{Hide, Show, MoveTo},
    style::{Color, SetForegroundColor, SetBackgroundColor, ResetColor},
    event::{poll, read, Event},
};

const BOX_SIZE: u16 = 30;
const FRAME_TIME: u64 = 10; // Much faster updates

const SYMBOLS: &[char] = &[
    // Block Elements
    '█', '▀', '▄', '▌', '▐', '░', '▒', '▓', '■', '□', '▢', '▣', '▤', '▥', '▦', '▧',
    '▨', '▩', '▪', '▫', '▬', '▭', '▮', '▯', '▰', '▱', '▲', '△', '▴', '▵', '▶', '▷',
    '▸', '▹', '►', '▻', '▼', '▽', '▾', '▿', '◀', '◁', '◂', '◃', '◄', '◅', '◆', '◇',
    '◈', '◉', '◊', '○', '◌', '◍', '◎', '●', '◐', '◑', '◒', '◓', '◔', '◕', '◖', '◗',
    // ASCII Art Blocks
    '╔', '╗', '╚', '╝', '║', '═', '╒', '╓', '╕', '╖', '╘', '╙', '╛', '╜', '╞', '╟',
    '╠', '╡', '╢', '╣', '╤', '╥', '╦', '╧', '╨', '╩', '╪', '╫', '╬', '╭', '╮', '╯',
    '╰', '╱', '╲', '╳', '╴', '╵', '╶', '╷', '│', '┌', '┐', '└', '┘', '├', '┤', '┬',
    '┴', '┼', '┃', '┄', '┅', '┆', '┇', '┈', '┉', '┊', '┋', '┌', '┍', '┎', '┏', '┐',
    // Braille Patterns
    '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⠐',
];

// Color palette with RGB values for smooth transitions
const COLORS: &[(u8, u8, u8)] = &[
    (255, 0, 0),     // Red
    (255, 127, 0),   // Orange
    (255, 255, 0),   // Yellow
    (127, 255, 0),   // Chartreuse
    (0, 255, 0),     // Green
    (0, 255, 127),   // Spring Green
    (0, 255, 255),   // Cyan
    (0, 127, 255),   // Azure
    (0, 0, 255),     // Blue
    (127, 0, 255),   // Violet
    (255, 0, 255),   // Magenta
    (255, 0, 127),   // Rose
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

impl Pattern {
    fn new() -> Self {
        Self {
            x_offset: 0.0,
            y_offset: 0.0,
            direction: Direction::Left,
            speed: 1.0,
        }
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Left => self.x_offset -= self.speed,
            Direction::Up => self.y_offset -= self.speed,
            Direction::Down => self.y_offset += self.speed,
            Direction::Right => self.x_offset += self.speed,
        }
    }

    fn get_offset(&self, x: u16, y: u16) -> (usize, usize) {
        let offset_x = (x as f64 + self.x_offset).floor() as usize;
        let offset_y = (y as f64 + self.y_offset).floor() as usize;
        (offset_x, offset_y)
    }
}

fn draw_pattern(
    stdout: &mut io::Stdout,
    term_width: u16,
    term_height: u16,
    pattern: &Pattern,
    color_phase: f64,
) -> io::Result<()> {
    let start_x = (term_width - BOX_SIZE) / 2;
    let start_y = (term_height - BOX_SIZE) / 2;
    
    // Draw a border around the animation box
    let border_color = Color::Rgb { r: 100, g: 100, b: 100 };
    
    // Top and bottom borders
    for x in start_x..start_x + BOX_SIZE {
        execute!(stdout, MoveTo(x, start_y - 1), SetForegroundColor(border_color))?;
        write!(stdout, "─")?;
        execute!(stdout, MoveTo(x, start_y + BOX_SIZE), SetForegroundColor(border_color))?;
        write!(stdout, "─")?;
    }
    
    // Left and right borders
    for y in start_y..start_y + BOX_SIZE {
        execute!(stdout, MoveTo(start_x - 1, y), SetForegroundColor(border_color))?;
        write!(stdout, "│")?;
        execute!(stdout, MoveTo(start_x + BOX_SIZE, y), SetForegroundColor(border_color))?;
        write!(stdout, "│")?;
    }
    
    // Corners
    execute!(stdout, MoveTo(start_x - 1, start_y - 1), SetForegroundColor(border_color))?;
    write!(stdout, "┌")?;
    execute!(stdout, MoveTo(start_x + BOX_SIZE, start_y - 1), SetForegroundColor(border_color))?;
    write!(stdout, "┐")?;
    execute!(stdout, MoveTo(start_x - 1, start_y + BOX_SIZE), SetForegroundColor(border_color))?;
    write!(stdout, "└")?;
    execute!(stdout, MoveTo(start_x + BOX_SIZE, start_y + BOX_SIZE), SetForegroundColor(border_color))?;
    write!(stdout, "┘")?;
    
    let color_index = (color_phase.floor() as usize) % COLORS.len();
    let next_color_index = (color_index + 1) % COLORS.len();
    let color_t = color_phase.fract();
    
    let current_color = COLORS[color_index];
    let next_color = COLORS[next_color_index];
    
    // Draw the animated pattern inside the box
    for y in 0..BOX_SIZE {
        for x in 0..BOX_SIZE {
            let (offset_x, offset_y) = pattern.get_offset(x, y);
            let symbol = SYMBOLS[(offset_x + offset_y) % SYMBOLS.len()];
            
            let position_t = ((x as f64 / BOX_SIZE as f64) + (y as f64 / BOX_SIZE as f64)) / 2.0;
            let color_shift = (position_t * 0.2) - 0.1;
            let adjusted_t = (color_t + color_shift).max(0.0).min(1.0);
            
            let color = interpolate_color(current_color, next_color, adjusted_t);
            
            execute!(
                stdout,
                MoveTo(start_x + x, start_y + y),
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
    
    // Get terminal size
    let (term_width, term_height) = size()?;
    
    // Check if terminal is large enough
    if term_width < BOX_SIZE + 2 || term_height < BOX_SIZE + 2 {
        execute!(stdout, ResetColor)?;
        println!("Terminal window too small. Minimum size: {}x{}", BOX_SIZE + 2, BOX_SIZE + 2);
        return Ok(());
    }
    
    // Hide cursor and clear screen
    execute!(stdout, Hide, Clear(ClearType::All))?;
    
    let mut pattern = Pattern::new();
    let mut color_phase = 0.0;
    let color_speed = 0.1;
    
    // Animation sequence
    for frame in 0..400 { // 4 seconds total at 10ms per frame
        if poll(Duration::from_millis(FRAME_TIME))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
        
        // Change direction every 100 frames (1 second)
        if frame % 100 == 0 {
            pattern.direction = match pattern.direction {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Left,
            };
        }
        
        draw_pattern(&mut stdout, term_width, term_height, &pattern, color_phase)?;
        pattern.update();
        color_phase += color_speed;
    }
    
    // Quick fade to black
    for i in 0..10 {
        let fade = 1.0 - (i as f64 / 10.0);
        let start_x = (term_width - BOX_SIZE) / 2;
        let start_y = (term_height - BOX_SIZE) / 2;
        
        for y in 0..BOX_SIZE {
            for x in 0..BOX_SIZE {
                let color = Color::Rgb {
                    r: (fade * 255.0) as u8,
                    g: (fade * 255.0) as u8,
                    b: (fade * 255.0) as u8,
                };
                execute!(
                    stdout,
                    MoveTo(start_x + x, start_y + y),
                    SetForegroundColor(color),
                )?;
                write!(stdout, "█")?;
            }
        }
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(FRAME_TIME));
    }
    
    // Reset terminal
    execute!(
        stdout,
        Clear(ClearType::All),
        ResetColor,
        Show,
        MoveTo(0, 0)
    )?;
    
    disable_raw_mode()?;
    Ok(())
} 