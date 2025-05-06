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