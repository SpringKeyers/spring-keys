# Phase 11: Quiet Mode Implementation

## Overview
This phase implements a quiet mode for the SpringKeys application, allowing it to run without any output or logging messages. This is particularly useful for running the screensaver in non-interactive environments or when output needs to be suppressed.

## Requirements

### Command Line Interface
- Add `-q` or `--quiet` flag to suppress all output
- Maintain existing functionality while running silently
- Support quiet mode for all commands, especially screensaver
- Allow ESC or Ctrl-D to exit screensaver mode
- Redraw entire screen buffer at exit

### Output Suppression
1. Logging
   - Set log level to `Error` in quiet mode
   - Suppress all `info!` and `println!` statements
   - Only show critical errors

2. Screensaver Mode
   - Run animation without any text output
   - Skip quote display in quiet mode
   - Maintain visual elements only
   - Support 1-second minimum duration
   - Final screen redraw before exit

3. Quote Display
   - Suppress quote text and source in quiet mode
   - Skip attribution lines
   - Maintain core functionality

### Implementation Details

#### Main Function Changes
```rust
// Set up logging based on quiet mode
let log_level = if quiet_mode { LevelFilter::Error } else { LevelFilter::Info };
let _ = logger::init_logger(log_level, None::<PathBuf>);

if !quiet_mode {
    info!("Starting SpringKeys application");
}
```

#### Screensaver Exit Handling
```rust
// Check for exit conditions
if poll(Duration::from_millis(0))? {
    if let Event::Key(key_event) = read()? {
        match key_event.code {
            KeyCode::Esc => {
                should_exit = true;
            }
            KeyCode::Char('d') if key_event.modifiers == KeyModifiers::CONTROL => {
                should_exit = true;
            }
            _ => {}
        }
    }
}

// Final screen redraw before exit
execute!(stdout, Clear(ClearType::All))?;
update_and_draw_trees(&mut stdout, &mut trees, &mut seeds, width, height)?;
update_and_draw_animals(&mut stdout, &mut animals, &seeds, width, height)?;
update_and_draw_moose(&mut stdout, &mut moose, &mut trees, width, height)?;
stdout.flush()?;
```

#### UI Module Changes
- Remove debug print statements
- Suppress demo mode messages
- Maintain core functionality without output
- Handle exit conditions gracefully

#### Moosesay Module Changes
- Streamline animation code
- Remove unnecessary debug output
- Optimize frame rate control
- Add exit handling
- Implement final screen redraw

## Usage Examples

### Basic Usage
```bash
# Run screensaver quietly for 1 second
spring-keys -q screensaver 1

# Get a quote without any output
spring-keys -q quote

# Run moosesay without text
spring-keys -q moosesay
```

### Combined Options
```bash
# Run in quiet mode with specific difficulty
spring-keys -q -d medium screensaver 30

# Force non-interactive mode with quiet mode
spring-keys -q -- screensaver 15
```

## Testing Requirements

1. Verify quiet mode functionality:
   - No output in quiet mode
   - Proper error handling
   - Maintained core functionality
   - Exit handling (ESC/Ctrl-D)
   - Final screen redraw

2. Test combinations:
   - Quiet mode with different commands
   - Quiet mode with other flags
   - Non-interactive mode with quiet mode
   - Minimum duration (1 second)
   - Exit key combinations

3. Performance testing:
   - Ensure no performance impact
   - Verify memory usage
   - Check CPU utilization
   - Screen redraw performance

## Future Considerations

1. Additional quiet mode features:
   - Configurable output levels
   - Selective output suppression
   - Custom quiet mode profiles
   - Additional exit key combinations

2. Integration improvements:
   - Better error handling in quiet mode
   - Enhanced logging control
   - More granular output control
   - Improved screen redraw mechanism

3. Documentation updates:
   - Update help text
   - Add quiet mode examples
   - Document best practices
   - Document exit key combinations 