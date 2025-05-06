# SpringKeys Phase 9: Single Mode Implementation

## Overview
This phase focuses on implementing a new "single" mode for SpringKeys that enables automated testing, benchmarking, and scripted interaction with the application. The single mode will accept input via command-line arguments or standard input, and will exit automatically based on specific conditions.

## Core Features

### 1. Single Mode Execution
- [ ] Command-line argument for single mode (`--single` or `-s`)
- [ ] Support for predefined test quotes (e.g., "The quick brown fox jumps over the lazy dog.")
- [ ] Custom quote input via command-line arguments
- [ ] Support for reading input from stdin for automation
- [ ] Automatic application exit with appropriate return codes

### 2. Input Processing
- [ ] Space-separated input tokens to simulate sequential keystrokes
- [ ] Special sequence handling for control keys and modifiers
- [ ] Properly timed input simulation to match realistic typing
- [ ] Buffer pre-loading for immediate testing start

### 3. Exit Conditions
- [ ] Exit with code 0 on successful completion (period + Enter or ".<Enter>")
- [ ] Exit with code 1 if exit sequence not detected within 1 second of last input
- [ ] Custom timeout configuration via command-line arguments

### 4. Test Integration
- [ ] Integration with automated test framework
- [ ] Benchmark mode for performance measurement
- [ ] Repeatable test scenarios with consistent output
- [ ] Statistics collection and export

## Technical Implementation

### Command Line Interface
```
springkeys single [OPTIONS] [QUOTE]
```

Options:
- `--input=FILE`: Read input sequences from file
- `--timeout=MS`: Custom timeout in milliseconds (default: 1000)
- `--preset=NAME`: Use a preset quote (e.g., "foxjump" for "The quick brown fox...")
- `--output=FILE`: Write statistics to file

### Input Format
Input will be processed as space-separated tokens where:
- Regular characters represent keypresses
- Special sequences represent control keys:
  - `<enter>`: Enter key
  - `<bs>`: Backspace
  - `<tab>`: Tab key
  - `<ctrl+X>`: Control + X
  - `<shift+X>`: Shift + X

### Exit Code Logic
```rust
fn process_single_mode(app: &mut SpringKeys, input: &str, timeout: Duration) -> i32 {
    // Process input sequence
    for token in input.split_whitespace() {
        app.process_token(token);
    }
    
    // Start timeout timer
    let start = Instant::now();
    
    // Wait for period + Enter or until timeout
    while start.elapsed() < timeout {
        if let Some(event) = poll_keyboard_events() {
            if is_exit_sequence(event) {
                return 0; // Success
            }
            
            // Process any additional user input
            app.process_event(event);
        }
    }
    
    // Timeout reached without exit sequence
    return 1;
}
```

### Integration with Main
```rust
match args.command {
    Command::Practice => run_practice_mode(app),
    Command::Single { quote, input_file, timeout } => {
        let input = get_input(quote, input_file);
        let timeout_duration = Duration::from_millis(timeout);
        process_single_mode(app, &input, timeout_duration)
    },
    // Other commands...
}
```

## Test Cases

### Basic Test: Fox Jump
```rust
#[test]
fn test_single_mode_fox_jump() {
    let output = Command::new(env!("CARGO_BIN_EXE_spring-keys"))
        .arg("single")
        .arg("--preset=foxjump")
        .arg("--input=T h e <space> q u i c k <space> b r o w n <space> f o x <space> j u m p s <space> o v e r <space> t h e <space> l a z y <space> d o g . <enter>")
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(0));
}
```

### Timeout Test
```rust
#[test]
fn test_single_mode_timeout() {
    let output = Command::new(env!("CARGO_BIN_EXE_spring-keys"))
        .arg("single")
        .arg("--preset=foxjump")
        .arg("--input=T h e <space> q u i c k")  // Incomplete input
        .output()
        .expect("Failed to execute command");
    
    assert_eq!(output.status.code(), Some(1));
}
```

## Implementation Steps

### 1. Command Line Parsing
- [ ] Update cli argument parsing to support single mode
- [ ] Add option handling for input, timeout, presets
- [ ] Configure appropriate defaults
- [ ] Add help documentation

### 2. Input Processing
- [ ] Implement token parser for special sequences
- [ ] Create input buffer preloading mechanism
- [ ] Develop timed input simulator
- [ ] Implement tracking for exit conditions

### 3. Test Framework
- [ ] Create automated test suite for single mode
- [ ] Implement benchmark framework
- [ ] Develop statistics collection and reporting
- [ ] Add CI integration

### 4. UI Integration
- [ ] Update UI to support single mode display
- [ ] Prevent normal UI interactions in single mode
- [ ] Add single mode indicators to screen
- [ ] Implement graceful exit handling

## Success Criteria
- [ ] Single mode can be executed with command-line arguments
- [ ] Input is correctly processed from arguments or files
- [ ] Application exits with code 0 on successful completion
- [ ] Application exits with code 1 on timeout
- [ ] Automated tests successfully validate functionality
- [ ] Performance benchmarks show consistent results

## Example Usage

### Basic Single Mode
```bash
# Test with the fox jump phrase
springkeys single --preset=foxjump

# Custom quote
springkeys single "This is a test quote."

# Custom input sequence
springkeys single --preset=foxjump --input="T h e <space> q u i c k <space> b r o w n <space> f o x ."
```

### Automation Integration
```bash
# Pipe input from file
cat input.txt | springkeys single

# Use in shell script with exit code
springkeys single --preset=foxjump --input="$input_sequence"
if [ $? -eq 0 ]; then
    echo "Test passed!"
else
    echo "Test failed!"
fi
```

## Future Enhancements
1. Detailed performance metrics export
2. Record and replay functionality
3. Interactive test builder
4. Visual replay of test execution with speed controls
5. Integration with continuous benchmarking systems

## Implementation Schedule
- Week 1: Command line argument parsing and basic single mode infrastructure
- Week 2: Input processing and exit condition logic
- Week 3: Test framework integration and automation support
- Week 4: Final testing, documentation, and release

The single mode will significantly enhance the testability and benchmarking capabilities of SpringKeys, making it easier to maintain quality and performance as the application evolves.

## Test Fixes and Improvements

### Color Spectrum Tests
- Updated color spectrum tests to match the actual purple-blue-green-orange-red spectrum implementation
- Fixed test assertions to check for exact RGB values at each point in the spectrum:
  - 0.0: Purple (128, 0, 128)
  - 0.25: Blue (0, 0, 255)
  - 0.5: Green (0, 255, 0)
  - 0.75: Orange (255, 165, 0)
  - 1.0: Red (255, 0, 0)

### Input Processing Improvements
1. Fixed position tracking in TypingSession:
   - Updated `record_keystroke` to correctly increment position when typing the last character
   - Changed condition from `self.current_position < self.text.len() - 1` to `self.current_position < self.text.len()`

2. Enhanced Input Validation:
   - Modified `validate_input` in InputProcessor to properly handle partial matches
   - Added support for validating incomplete but correct input
   - Improved error reporting with position tracking

3. Case Sensitivity Handling:
   - Added proper case sensitivity support in `process_token`
   - Implemented SHIFT modifier handling for uppercase characters
   - Fixed token sequence processing to maintain proper capitalization

### Test Output and Debugging
- Added comprehensive debug output to tests
- Improved test failure messages with detailed state information
- Fixed test visibility using proper cargo test flags (--show-output)

### Dead Code Cleanup (Pending)
Identified unused code that should be reviewed:
- `KeyAnimation` struct and related functions
- `KEY_ANIMATIONS` and `PREVIOUS_FRAME` static variables
- `get_animations`, `should_redraw_key`, and `get_speed_color` functions

### Next Steps
1. Review and clean up unused code in heatmap implementation
2. Add more test cases for edge cases in input processing
3. Consider adding performance tests for typing metrics
4. Document the color spectrum implementation and its use cases 

## Implementation Status Update

The headless auto-detection and environment information features have been successfully implemented and tested. The application now:

1. Automatically detects headless environments
2. Provides detailed environment information when requested
3. Shows clear test configuration during execution
4. Works correctly in both interactive and non-interactive environments
5. Loads new random quotes on Enter key press (✓ Completed)

### Key Features Implemented
- [x] Automatic headless environment detection
- [x] Detailed environment information display
- [x] Clear test configuration output
- [x] Enter key loads new random quotes from current category
- [x] Category switching with function keys (F6, F7, F8)

### Build Verification
// ... existing code ... 