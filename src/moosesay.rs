use std::io::{self, Write};
use std::thread;
use std::time::Duration;

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

pub fn animate_moose_quote(text: &str) -> io::Result<()> {
    let bubble = create_speech_bubble(text, 40);
    let mut stdout = io::stdout();
    let mut frame = 0;
    
    // Clear screen and hide cursor
    print!("\x1B[2J\x1B[1;1H\x1B[?25l");
    stdout.flush()?;
    
    // Animate for about 10 seconds
    for _ in 0..40 {
        // Move cursor to top
        print!("\x1B[1;1H");
        
        // Print speech bubble
        print!("{}", bubble);
        
        // Print current moose frame
        print!("{}", MOOSE_FRAMES[frame]);
        stdout.flush()?;
        
        // Switch frames
        frame = (frame + 1) % MOOSE_FRAMES.len();
        
        thread::sleep(Duration::from_millis(250));
    }
    
    // Show cursor again
    print!("\x1B[?25h");
    stdout.flush()?;
    
    Ok(())
} 