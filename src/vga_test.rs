use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};
use crossterm::{
    execute,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    cursor::{Hide, Show, MoveTo},
    style::{Color, SetForegroundColor, SetBackgroundColor},
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

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
}

#[derive(Debug)]
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

    fn update(&mut self) {
        match self.direction {
            Direction::Left => {
                let new_offset = self.x_offset - self.speed;
                if new_offset <= -1.0 {
                    self.x_offset = 0.0;
                } else {
                    self.x_offset = new_offset;
                }
            }
        }
    }
}

pub fn run_test_screen() -> io::Result<()> {
    run_test_screen_with_duration(None)
}

fn run_test_screen_with_duration(max_duration: Option<Duration>) -> io::Result<()> {
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
    
    let start_time = Instant::now();
    
    // Animation sequence
    loop {
        // Check if we've exceeded max duration (for tests)
        if let Some(max_dur) = max_duration {
            if start_time.elapsed() >= max_dur {
                break;
            }
        }

        if poll(Duration::from_millis(FRAME_TIME))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
        
        draw_pattern(&mut stdout, term_width, term_height, &pattern, color_phase, box_size)?;
        
        // Update pattern position
        pattern.update();
        
        // Update color phase
        color_phase = (color_phase + color_speed) % (2.0 * std::f64::consts::PI);
    }
    
    // Cleanup
    execute!(stdout, Show)?;
    execute!(stdout, Clear(ClearType::All))?;
    disable_raw_mode()?;
    
    Ok(())
}

fn draw_pattern(stdout: &mut io::Stdout, width: u16, height: u16, pattern: &Pattern, color_phase: f64, box_size: u16) -> io::Result<()> {
    // Calculate pattern dimensions
    let pattern_width = box_size;
    let pattern_height = box_size;
    
    // Calculate starting position to center the pattern
    let start_x = (width - pattern_width) / 2;
    let start_y = (height - pattern_height) / 2;
    
    // Draw the pattern
    for y in 0..pattern_height {
        for x in 0..pattern_width {
            let pos_x = start_x + x;
            let pos_y = start_y + y;
            
            // Calculate pattern offset
            let offset_x = x as f64 + pattern.x_offset;
            let offset_y = y as f64 + pattern.y_offset;
            
            // Calculate color based on position and phase
            let color_idx = ((offset_x + offset_y + color_phase * 10.0) / 4.0).sin();
            let color_idx = ((color_idx + 1.0) / 2.0 * (COLORS.len() - 1) as f64) as usize;
            let (r, g, b) = COLORS[color_idx];
            
            // Calculate symbol based on position
            let symbol_idx = ((offset_x + offset_y) / 2.0).floor() as usize % SYMBOLS.len();
            let symbol = SYMBOLS[symbol_idx];
            
            // Draw the character
            execute!(
                stdout,
                MoveTo(pos_x, pos_y),
                SetBackgroundColor(Color::Rgb { r, g, b }),
                SetForegroundColor(Color::Black)
            )?;
            write!(stdout, "{}", symbol)?;
        }
    }
    
    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_vga_animation_can_run_and_exit() {
        // Run the test screen with a short duration
        let result = run_test_screen_with_duration(Some(Duration::from_millis(500)));
        assert!(result.is_ok(), "VGA test screen should run and exit cleanly");
    }

    #[test]
    fn test_pattern_movement() {
        let mut pattern = Pattern::new(16);
        
        // Initial state
        assert!(pattern.x_offset >= 0.0 && pattern.x_offset < 1.0, "Pattern should start within bounds");
        assert_eq!(pattern.speed, 1.0);
        assert!(matches!(pattern.direction, Direction::Left));
        
        // Track movement over multiple updates
        let mut updates = 0;
        let mut saw_wrap = false;
        
        for _ in 0..10 {
            pattern.update();
            updates += 1;
            
            // Pattern should always be within [-1.0, 0.0]
            assert!(pattern.x_offset >= -1.0 && pattern.x_offset <= 0.0, 
                "Pattern should stay within bounds (got {})", pattern.x_offset);
            
            // Check if we've seen a wrap
            if pattern.x_offset == 0.0 {
                saw_wrap = true;
            }
        }
        
        assert!(updates > 0, "Pattern should update");
        assert!(saw_wrap, "Pattern should wrap around at least once");
    }
} 