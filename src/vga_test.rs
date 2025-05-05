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
const FRAME_TIME: u64 = 33; // ~30 FPS for smoother animation

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
    scale: f64,
}

impl Pattern {
    fn new() -> Self {
        Self {
            x_offset: 0.0,
            y_offset: 0.0,
            angle: std::f64::consts::PI / 4.0,
            speed: 0.3, // Reduced speed for smoother movement
            scale: 1.0,
        }
    }

    fn update(&mut self, delta_time: f64) {
        self.x_offset += self.speed * self.angle.cos() * delta_time;
        self.y_offset += self.speed * self.angle.sin() * delta_time;
    }

    fn get_offset(&self, x: u16, y: u16) -> (usize, usize) {
        let scaled_x = x as f64 * self.scale;
        let scaled_y = y as f64 * self.scale;
        let offset_x = (scaled_x + self.x_offset).floor() as usize;
        let offset_y = (scaled_y + self.y_offset).floor() as usize;
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
    term_width: u16,
    term_height: u16,
    pattern: &Pattern,
    color_phase: f64,
) -> io::Result<()> {
    // Calculate the starting position to center the box
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
            
            // Create a subtle color variation based on position
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
    let color_speed = 0.03; // Slower color transitions
    let mut last_frame = std::time::Instant::now();
    
    // Animation sequence
    for frame in 0..180 { // 180 frames = 6 seconds at 30 FPS
        if poll(Duration::from_millis(FRAME_TIME))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
        
        let delta_time = last_frame.elapsed().as_secs_f64();
        last_frame = std::time::Instant::now();
        
        let phase_type = frame / 45; // Changes every 1.5 seconds
        match phase_type {
            0 => { // Smooth diagonal
                pattern.angle = std::f64::consts::PI / 4.0;
                pattern.speed = 0.3;
            },
            1 => { // Gentle wave
                pattern.angle = (frame as f64 * 0.05).sin() * std::f64::consts::PI / 3.0;
                pattern.speed = 0.25;
            },
            2 => { // Slow spiral
                pattern.angle += 0.02;
                pattern.speed = 0.2;
            },
            _ => { // Smooth figure-eight
                let t = frame as f64 * 0.05;
                pattern.angle = t.sin() * std::f64::consts::PI / 2.0;
                pattern.speed = 0.3 * t.cos();
            }
        }
        
        draw_pattern(&mut stdout, term_width, term_height, &pattern, color_phase)?;
        pattern.update(delta_time);
        color_phase += color_speed * delta_time;
        
        std::thread::sleep(Duration::from_millis(FRAME_TIME));
    }
    
    // Fade to black
    for i in 0..30 {
        let fade = 1.0 - (i as f64 / 30.0);
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