pub fn print_help() {
    println!("SpringKeys - Typing Tutor with Spring-based Physics");
    println!("====================================================");
    println!();
    println!("USAGE:");
    println!("  spring-keys [OPTIONS] [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("  practice              Practice typing with quotes");
    println!("  game                  Play typing mini-games");
    println!("  stats                 View your typing statistics");
    println!("  config                Edit configuration");
    println!("  test                  Display test pattern (VGA-style test)");
    println!("  single [QUOTE]        Run a single quote test with automated exit");
    println!("  consume [INPUT]       Process input and visualize typing results in UI");
    println!();
    println!("OPTIONS:");
    println!("  -h, --help            Show this help message");
    println!("  -v, --version         Show version information");
    println!("  -d, --difficulty      Set difficulty level (easy, medium, hard)");
    println!("  -q, --quiet           Quiet mode (minimal logging)");
    println!();
    println!("SINGLE MODE OPTIONS:");
    println!("  --preset=NAME         Use a preset quote (e.g., \"foxjump\")");
    println!("  --input=TEXT          Input sequence for automation");
    println!("  --timeout=MS          Timeout in milliseconds (default: 1000)");
    println!();
    println!("CONSUME MODE OPTIONS:");
    println!("  --input=TEXT          Input sequence to process (space-separated tokens)");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("  SPRING_KEYS_TEST_MODE Set to '1' or 'true' to enable single mode for automated testing");
    println!("  SPRING_KEYS_ENV_INFO  Set to '1' or 'true' to display environment information");
    println!("  SPRING_KEYS_DEMO_HEATMAP Enable color spectrum visualization for keyboard heatmap");
    println!();
    println!("HEADLESS DETECTION:");
    println!("  SpringKeys automatically detects headless environments (CI systems, non-interactive");
    println!("  terminals) and enables test mode automatically. This can be overridden by explicitly");
    println!("  setting SPRING_KEYS_TEST_MODE=0 if needed.");
    println!();
    println!("EXAMPLES:");
    println!("  spring-keys practice -d medium");
    println!("  spring-keys game");
    println!("  spring-keys test");
    println!("  spring-keys single \"Custom test quote.\"");
    println!("  spring-keys single --preset=foxjump");
    println!("  spring-keys consume \"T h e <space> q u i c k\"");
    println!("  SPRING_KEYS_TEST_MODE=1 spring-keys --input \"T h e <space> q u i c k\"");
    println!();
    println!("For more information, see the documentation.");
}

// Commented out as it's not currently used
/*
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
*/ 