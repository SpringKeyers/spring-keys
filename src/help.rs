use colored::*;

pub fn print_help() {
    println!("{}", "SpringKeys - A Modern Terminal-based Typing Tutor".bright_cyan().bold());
    println!("{}", format!("Version {}", env!("CARGO_PKG_VERSION")).bright_black());
    println!();
    
    println!("{}", "DESCRIPTION:".yellow());
    println!("  A modern typing tutor with real-time feedback, statistics tracking,");
    println!("  and mini-games to help improve your typing speed and accuracy.");
    println!();
    
    println!("{}", "USAGE:".yellow());
    println!("  spring-keys [OPTIONS] [COMMAND]");
    println!();
    
    println!("{}", "OPTIONS:".yellow());
    println!("  -h, --help        Print this help message");
    println!("  -v, --version     Print version information");
    println!("  -d, --difficulty  Set difficulty level (easy|medium|hard)");
    println!("  -q, --quiet       Suppress non-error output");
    println!();
    
    println!("{}", "COMMANDS:".yellow());
    println!("  practice          Start a typing practice session");
    println!("  stats            View your typing statistics");
    println!("  config           Edit configuration settings");
    println!("  game             Start a typing mini-game");
    println!("  test             Run VGA-style test screen (default if no command)");
    println!();
    
    println!("{}", "EXAMPLES:".yellow());
    println!("  spring-keys                    Run VGA test screen");
    println!("  spring-keys practice           Start a practice session");
    println!("  spring-keys -d easy practice   Practice with easy difficulty");
    println!("  spring-keys stats              View your statistics");
    println!("  spring-keys game               Start a typing mini-game");
    println!("  spring-keys test               Run VGA test screen");
    println!();
    
    println!("{}", "KEYBOARD SHORTCUTS:".yellow());
    println!("  ESC              Exit the current session");
    println!("  F5               Get a new random quote");
    println!("  Ctrl+C           Force quit the application");
    println!("  Any key          Skip VGA test animation");
    println!();
    
    println!("{}", "AUTHOR:".yellow());
    println!("  {}", env!("CARGO_PKG_AUTHORS"));
    println!();
    
    println!("{}", "LICENSE:".yellow());
    println!("{}", MIT_LICENSE.bright_black());
}

const MIT_LICENSE: &str = r#"MIT License

Copyright (c) 2024 microuser

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#; 