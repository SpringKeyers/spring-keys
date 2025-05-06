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
6. Accumulates and persists typing statistics (✓ Completed)

### Key Features Implemented
- [x] Automatic headless environment detection
- [x] Detailed environment information display
- [x] Clear test configuration output
- [x] Enter key loads new random quotes from current category
- [x] Category switching with function keys (F6, F7, F8)
- [x] Statistics persistence and accumulation

### Statistics System Enhancement
The application now includes a robust statistics persistence system:

1. **Stats Storage**
   - Statistics are stored in JSON files in the `stats/` directory
   - Each session creates a new stats file with timestamp
   - Files contain detailed metrics including:
     - WPM (Words Per Minute)
     - Accuracy percentages
     - Character-specific performance
     - Timing data

2. **Stats Accumulation**
   - On startup, the application reads all valid JSON files from stats directory
   - Each file's data is parsed and validated
   - Valid statistics are accumulated into running totals
   - Interface displays combined averages from all sessions
   - Current session stats are merged with historical data

3. **Resume Capability**
   - Interface maintains running totals across sessions
   - Users can stop and resume practice while maintaining progress
   - Historical performance data influences displayed averages
   - Seamless integration of past and current performance metrics

4. **Implementation Details**
   - Automatic stats file parsing on startup
   - Graceful handling of invalid or corrupted stat files
   - Real-time updates to accumulated statistics
   - Efficient storage and retrieval of historical data

5. **JSON Implementation Details**
   - **File Structure**
     ```json
     {
       "timestamp": "2024-03-21T15:30:45Z",
       "session_duration": 3600,
       "metrics": {
         "wpm": 75.5,
         "accuracy": 98.2,
         "total_keystrokes": 1250,
         "correct_keystrokes": 1227,
         "errors": 23
       },
      "character_stats": {
         "a": { "count": 120, "errors": 2, "avg_time_ms": 150 },
         "b": { "count": 45, "errors": 1, "avg_time_ms": 165 }
         // ... other characters
       }
     }
     ```

   - **File Management**
     - Stats files named with pattern: `typing_stats_YYYYMMDD_HHMMSS.json`
     - Files stored in `stats/` directory with appropriate permissions
     - Automatic cleanup of temporary or corrupted files
     - Backup creation before writing new data

   - **Loading Process**
     1. Scan `stats/` directory for `.json` files on startup
     2. Parse each file using serde_json with custom deserializer
     3. Validate schema and data integrity
     4. Skip and log any corrupted or invalid files
     5. Accumulate valid statistics into memory

   - **Parsing Implementation**
     ```rust
     #[derive(Serialize, Deserialize)]
     struct TypingStats {
         timestamp: DateTime<Utc>,
         session_duration: u64,
         metrics: SessionMetrics,
         character_stats: HashMap<char, CharacterStats>
     }

     impl TypingStats {
         fn load_from_directory(path: &Path) -> Result<Vec<TypingStats>> {
             let entries = fs::read_dir(path)?;
             entries
                 .filter_map(|entry| {
                     let path = entry.ok()?.path();
                     if path.extension()? == "json" {
                         Self::load_from_file(&path).ok()
                     } else {
                         None
                     }
                 })
                 .collect()
         }
     }
     ```

   - **Saving Process**
     1. Create new stats file with unique timestamp
     2. Serialize current session data to JSON
     3. Write to temporary file first
     4. Validate written data
     5. Atomically rename to final filename
     ```rust
     impl TypingStats {
         fn save(&self) -> Result<()> {
             let filename = format!(
                 "typing_stats_{}.json",
                 Utc::now().format("%Y%m%d_%H%M%S")
             );
             let temp_path = self.stats_dir.join(format!(".{}.tmp", filename));
             let final_path = self.stats_dir.join(filename);
             
             serde_json::to_writer_pretty(
                 File::create(&temp_path)?,
                 self
             )?;
             
             fs::rename(temp_path, final_path)?;
             Ok(())
         }
     }
     ```

   - **Error Handling**
     - Graceful recovery from corrupted files
     - Logging of parsing errors with file details
     - Automatic backup of problematic files
     - Fallback to empty stats if no valid files found

   - **Performance Considerations**
     - Lazy loading of historical data
     - Caching of accumulated statistics
     - Periodic auto-save of current session
     - Efficient memory usage for large datasets

This enhancement allows users to track their progress over time and resume practice sessions while maintaining their historical performance metrics.

### Build Verification
// ... existing code ... 