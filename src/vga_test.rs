use std::io::{self, Write, stdout};
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, size, enable_raw_mode, disable_raw_mode},
    cursor::{Hide, Show, MoveTo},
    style::{Color, SetForegroundColor, SetBackgroundColor, ResetColor},
    event::{poll, read, Event},
};

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

struct Pattern {
    x_offset: f64,
    y_offset: f64,
    angle: f64,
    speed: f64,
}

impl Pattern {
    fn new() -> Self {
        Self {
            x_offset: 0.0,
            y_offset: 0.0,
            angle: std::f64::consts::PI / 4.0, // 45 degrees for diagonal movement
            speed: 0.5,
        }
    }

    fn update(&mut self) {
        self.x_offset += self.speed * self.angle.cos();
        self.y_offset += self.speed * self.angle.sin();
    }

    fn get_offset(&self, x: u16, y: u16) -> (usize, usize) {
        let offset_x = (x as f64 + self.x_offset).floor() as usize;
        let offset_y = (y as f64 + self.y_offset).floor() as usize;
        (offset_x, offset_y)
    }
}

fn interpolate_color(color1: (u8, u8, u8), color2: (u8, u8, u8), t: f64) -> Color {
    let r = (color1.0 as f64 * (1.0 - t) + color2.0 as f64 * t) as u8;
    let g = (color1.1 as f64 * (1.0 - t) + color2.1 as f64 * t) as u8;
    let b = (color1.2 as f64 * (1.0 - t) + color2.2 as f64 * t) as u8;
    Color::Rgb { r, g, b }
}

fn draw_pattern(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    pattern: &Pattern,
    color_phase: f64,
) -> io::Result<()> {
    let color_index = (color_phase.floor() as usize) % COLORS.len();
    let next_color_index = (color_index + 1) % COLORS.len();
    let color_t = color_phase.fract();
    
    let current_color = COLORS[color_index];
    let next_color = COLORS[next_color_index];
    
    for y in 0..height {
        for x in 0..width {
            let (offset_x, offset_y) = pattern.get_offset(x, y);
            let symbol = SYMBOLS[(offset_x + offset_y) % SYMBOLS.len()];
            
            // Create a subtle color variation based on position
            let position_t = ((x as f64 / width as f64) + (y as f64 / height as f64)) / 2.0;
            let color_shift = (position_t * 0.2) - 0.1; // -0.1 to 0.1 range
            let adjusted_t = (color_t + color_shift).max(0.0).min(1.0);
            
            let color = interpolate_color(current_color, next_color, adjusted_t);
            
            execute!(
                stdout,
                MoveTo(x, y),
                SetForegroundColor(color),
                SetBackgroundColor(Color::Black)
            )?;
            
            write!(stdout, "{}", symbol)?;
        }
    }
    stdout.flush()?;
    Ok(())
}

pub fn run_test_screen() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    
    // Get terminal size
    let (width, height) = size()?;
    
    // Hide cursor and clear screen
    execute!(stdout, Hide, Clear(ClearType::All))?;
    
    let mut pattern = Pattern::new();
    let mut color_phase = 0.0;
    let color_speed = 0.05;
    
    // Animation sequence
    for frame in 0..120 { // 120 frames = 6 seconds at 20 FPS
        if poll(Duration::from_millis(50))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
        
        let phase_type = frame / 30; // Changes every 1.5 seconds
        match phase_type {
            0 => { // Initial pattern with color cycling
                pattern.angle = std::f64::consts::PI / 4.0;
                pattern.speed = 0.5;
            },
            1 => { // Reverse diagonal
                pattern.angle = -std::f64::consts::PI / 4.0;
                pattern.speed = 0.7;
            },
            2 => { // Horizontal wave
                pattern.angle = (frame as f64 * 0.1).sin() * std::f64::consts::PI / 2.0;
                pattern.speed = 0.6;
            },
            _ => { // Spiral
                pattern.angle += 0.1;
                pattern.speed = 0.4;
            }
        }
        
        draw_pattern(&mut stdout, width, height, &pattern, color_phase)?;
        pattern.update();
        color_phase += color_speed;
    }
    
    // Fade to black
    for i in 0..20 {
        let fade = 1.0 - (i as f64 / 20.0);
        for y in 0..height {
            for x in 0..width {
                let color = Color::Rgb {
                    r: (fade * 255.0) as u8,
                    g: (fade * 255.0) as u8,
                    b: (fade * 255.0) as u8,
                };
                execute!(
                    stdout,
                    MoveTo(x, y),
                    SetForegroundColor(color),
                )?;
                write!(stdout, "█")?;
            }
        }
        stdout.flush()?;
        if poll(Duration::from_millis(50))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
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