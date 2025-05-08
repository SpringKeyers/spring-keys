use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use rand::Rng;
use crossterm::{
    terminal::{size, Clear, ClearType},
    cursor::{Hide, Show, MoveTo},
    execute,
    style::{Color, SetForegroundColor, SetBackgroundColor},
};

const MOOSE_FRAMES: [&str; 2] = [
    r#"
     \   ^__^
      \  (oo)\_______
         (__)\       )\/\
             ||----w |
             ||     ||
    "#,
    r#"
     \   ^__^
      \  (oo)\_______
         (__)\       )\/\
             ||----W |
             ||     ||
    "#
];

#[derive(Debug, Clone)]
struct Moose {
    x: i32,
    y: i32,
    direction: Direction,
    frame: usize,
    quote: String,
    quote_age: f32,
    is_edge_walker: bool,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
    Up,
    UpLeft,
}

impl Direction {
    fn random() -> Self {
        use Direction::*;
        let directions = [Left, DownLeft, Down, DownRight, Right, UpRight, Up, UpLeft];
        directions[rand::thread_rng().gen_range(0..directions.len())]
    }

    fn get_dx_dy(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            Left => (-1, 0),
            DownLeft => (-1, 1),
            Down => (0, 1),
            DownRight => (1, 1),
            Right => (1, 0),
            UpRight => (1, -1),
            Up => (0, -1),
            UpLeft => (-1, -1),
        }
    }
}

impl Moose {
    fn new(x: i32, y: i32, quote: String, is_edge_walker: bool) -> Self {
        Self {
            x,
            y,
            direction: Direction::random(),
            frame: 0,
            quote,
            quote_age: 0.0,
            is_edge_walker,
        }
    }

    fn update(&mut self, width: i32, height: i32) {
        // Update position
        let (dx, dy) = self.direction.get_dx_dy();
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        // Check boundaries and adjust direction if needed
        if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
            self.direction = Direction::random();
        } else {
            self.x = new_x;
            self.y = new_y;
        }

        // Update frame
        self.frame = (self.frame + 1) % MOOSE_FRAMES.len();

        // Update quote age
        self.quote_age += 0.1;
    }

    fn should_change_direction(&self) -> bool {
        rand::thread_rng().gen_bool(0.05) // 5% chance to change direction each update
    }
}

fn create_speech_bubble(text: &str, width: usize) -> String {
    let mut wrapped_lines = Vec::new();
    let mut current_line = String::new();
    
    // Word wrap the text
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 <= width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                wrapped_lines.push(current_line);
            }
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        wrapped_lines.push(current_line);
    }
    
    // Find the longest line to determine bubble width
    let max_length = wrapped_lines.iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);
    
    let mut bubble = String::new();
    
    // Top border
    bubble.push_str(" ");
    bubble.push_str(&"_".repeat(max_length + 2));
    bubble.push_str("\n");
    
    // Text lines
    for (i, line) in wrapped_lines.iter().enumerate() {
        if wrapped_lines.len() == 1 {
            bubble.push_str("< ");
        } else if i == 0 {
            bubble.push_str("/ ");
        } else if i == wrapped_lines.len() - 1 {
            bubble.push_str("\\ ");
        } else {
            bubble.push_str("| ");
        }
        
        bubble.push_str(line);
        bubble.push_str(&" ".repeat(max_length - line.len()));
        
        if wrapped_lines.len() == 1 {
            bubble.push_str(" >\n");
        } else if i == 0 {
            bubble.push_str(" \\\n");
        } else if i == wrapped_lines.len() - 1 {
            bubble.push_str(" /\n");
        } else {
            bubble.push_str(" |\n");
        }
    }
    
    // Bottom border
    bubble.push_str(" ");
    bubble.push_str(&"-".repeat(max_length + 2));
    bubble.push_str("\n");
    
    bubble
}

fn draw_fence(width: i32, height: i32) -> io::Result<()> {
    let mut stdout = io::stdout();
    
    // Draw horizontal fence
    for x in 0..width {
        execute!(
            stdout,
            MoveTo(x as u16, 0),
            SetForegroundColor(Color::Yellow),
            Print("=")
        )?;
        execute!(
            stdout,
            MoveTo(x as u16, height as u16 - 1),
            SetForegroundColor(Color::Yellow),
            Print("=")
        )?;
    }
    
    // Draw vertical fence
    for y in 0..height {
        execute!(
            stdout,
            MoveTo(0, y as u16),
            SetForegroundColor(Color::Yellow),
            Print("|")
        )?;
        execute!(
            stdout,
            MoveTo(width as u16 - 1, y as u16),
            SetForegroundColor(Color::Yellow),
            Print("|")
        )?;
    }
    
    Ok(())
}

pub fn animate_moose_quote(text: &str, duration_seconds: Option<u64>) -> io::Result<()> {
    let (width, height) = size()?;
    let width = width as i32;
    let height = height as i32;
    
    let mut stdout = io::stdout();
    execute!(stdout, Hide)?;
    execute!(stdout, Clear(ClearType::All))?;
    
    // Create multiple moose with different movement patterns
    let mut moose = vec![
        Moose::new(width / 4, height / 4, text.to_string(), false), // Center wanderer
        Moose::new(width / 2, height / 2, text.to_string(), true),  // Edge wanderer
    ];
    
    let mut rng = rand::thread_rng();
    let mut frame_count = 0;
    let start_time = std::time::Instant::now();
    
    loop {
        // Check if we've exceeded the duration
        if let Some(duration) = duration_seconds {
            if start_time.elapsed().as_secs() >= duration {
                break;
            }
        }
        
        // Clear screen
        execute!(stdout, Clear(ClearType::All))?;
        
        // Draw fence
        draw_fence(width, height)?;
        
        // Update and draw each moose
        for moose in &mut moose {
            // Update moose
            moose.update(width, height);
            
            // Randomly change direction
            if moose.should_change_direction() {
                moose.direction = Direction::random();
            }
            
            // Draw speech bubble if quote is active
            if moose.quote_age < 10.0 {
                let opacity = 1.0 - (moose.quote_age / 10.0);
                let bubble = create_speech_bubble(&moose.quote, 40);
                let lines: Vec<&str> = bubble.lines().collect();
                
                for (i, line) in lines.iter().enumerate() {
                    execute!(
                        stdout,
                        MoveTo(moose.x as u16, (moose.y - lines.len() as i32 + i as i32) as u16),
                        SetForegroundColor(Color::Rgb {
                            r: (255.0 * opacity) as u8,
                            g: (255.0 * opacity) as u8,
                            b: (255.0 * opacity) as u8,
                        }),
                        Print(line)
                    )?;
                }
            }
            
            // Draw moose
            let moose_lines: Vec<&str> = MOOSE_FRAMES[moose.frame].lines().collect();
            for (i, line) in moose_lines.iter().enumerate() {
                execute!(
                    stdout,
                    MoveTo(moose.x as u16, (moose.y + i as i32) as u16),
                    SetForegroundColor(Color::Green),
                    Print(line)
                )?;
            }
            
            // Reset quote if it's time
            if moose.quote_age >= 20.0 {
                moose.quote_age = 0.0;
            }
        }
        
        // Occasionally add a third moose that moves left to right
        if frame_count % 100 == 0 && moose.len() < 3 {
            moose.push(Moose::new(
                0,
                rng.gen_range(1..height-1),
                text.to_string(),
                false
            ));
        }
        
        // Remove moose that have moved off screen
        moose.retain(|m| m.x >= 0 && m.x < width && m.y >= 0 && m.y < height);
        
        stdout.flush()?;
        thread::sleep(Duration::from_millis(100));
        frame_count += 1;
    }
    
    // Cleanup
    execute!(stdout, Show)?;
    execute!(stdout, Clear(ClearType::All))?;
    
    Ok(())
} 