use crate::quotes::QuoteDatabase;

pub fn print_help() {
    let mut quote_db = QuoteDatabase::new_silent();
    let total_quotes = quote_db.total_quotes();

    println!("SpringKeys - Typing Tutor with Spring-based Physics");
    println!("====================================================\n");

    println!("USAGE:");
    println!("  spring-keys [OPTIONS] [COMMAND]");
    println!("  spring-keys -- [COMMAND]  # Force non-interactive mode\n");

    println!("COMMANDS:");
    println!("  practice              Practice typing with quotes");
    println!("  consume [INPUT]       Process input and visualize typing results in UI");
    println!("  config                Edit configuration");
    println!("  test                  Display test pattern (VGA-style test)");
    println!("  quote                 Output a random quote and exit");
    println!("  moosesay              Display an animated moose with a random quote");
    println!("  screensaver [SECONDS] Display animated moose screensaver for specified duration\n");

    println!("OPTIONS:");
    println!("  -h, --help            Show this help message");
    println!("  -v, --version         Show version information");
    println!("  -d, --difficulty      Set difficulty level (easy, medium, hard)");
    println!("  -q, --quiet           Quiet mode (minimal logging)");
    println!("  --verbose             Verbose mode (show final screen buffer)");
    println!("  --                    Force non-interactive mode (no animations)\n");

    println!("CONSUME MODE OPTIONS:");
    println!("  --input=TEXT          Input sequence to process (space-separated tokens)\n");

    println!("ENVIRONMENT VARIABLES:");
    println!("  SPRING_KEYS_ENV_INFO  Set to '1' or 'true' to display environment information");
    println!("  SPRING_KEYS_DEMO_HEATMAP Enable color spectrum visualization for keyboard heatmap\n");

    println!("EXAMPLES:");
    println!("  spring-keys practice -d medium");
    println!("  spring-keys consume \"T h e <space> q u i c k\"");
    println!("  spring-keys test");
    println!("  spring-keys quote     # Get a random quote");
    println!("  spring-keys moosesay  # Get a random quote with an animated moose");
    println!("  spring-keys screensaver 10  # Run moose screensaver for 10 seconds");
    println!("  spring-keys -- quote  # Get a quote without animation");
    println!("  spring-keys -q --verbose screensaver 1  # Run quietly and show final buffer\n");

    println!("Successfully loaded {} quotes from JSON files\n", total_quotes);

    println!("For more information, see the documentation.");
}

pub fn print_help_quiet() {
    println!("SpringKeys - Typing Tutor with Spring-based Physics");
    println!("====================================================\n");

    println!("USAGE:");
    println!("  spring-keys [OPTIONS] [COMMAND]");
    println!("  spring-keys -- [COMMAND]  # Force non-interactive mode\n");

    println!("COMMANDS:");
    println!("  practice              Practice typing with quotes");
    println!("  consume [INPUT]       Process input and visualize typing results in UI");
    println!("  config                Edit configuration");
    println!("  test                  Display test pattern (VGA-style test)");
    println!("  quote                 Output a random quote and exit");
    println!("  moosesay              Display an animated moose with a random quote");
    println!("  screensaver [SECONDS] Display animated moose screensaver for specified duration\n");

    println!("OPTIONS:");
    println!("  -h, --help            Show this help message");
    println!("  -v, --version         Show version information");
    println!("  -d, --difficulty      Set difficulty level (easy, medium, hard)");
    println!("  -q, --quiet           Quiet mode (minimal logging)");
    println!("  --verbose             Verbose mode (show final screen buffer)");
    println!("  --                    Force non-interactive mode (no animations)\n");
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