# SpringKeys Color Spectrum

This module provides a customizable color spectrum that maps values from 0 to 100 onto a color gradient with:
- Purple (minimum values)
- White (mid-range values)
- Red (maximum values)

## Color Spectrum Overview

The spectrum is designed for intuitive visualization of performance metrics where:
- Purple represents the minimum or starting point (0-33%)
- White represents the middle range (34-66%)
- Red represents the maximum or optimal performance (67-100%)

All colors are generated with appropriate background (darker) and foreground (lighter) variants of the same hue, ensuring good contrast and readability.

## Implementation Details

### Color Ranges

1. **Minimum Range (0-33)**
   - Gradient: Purple → White
   - Background: Dark purple → Light gray
   - Foreground: Light purple → White

2. **Mid Range (34-66)**
   - Color: White/Light gray
   - Background: Light gray
   - Foreground: White

3. **Maximum Range (67-100)**
   - Gradient: White → Red
   - Background: Light gray → Dark red
   - Foreground: White → Bright red

### Usage Example

```rust
use spring_keys::ui::color_spectrum::{value_to_spectrum, percentage_to_colors};
use crossterm::style::{SetBackgroundColor, SetForegroundColor, ResetColor};

// Get colors for a specific value (0-100)
let colors = value_to_spectrum(75); // Value in maximum range (red)

// Apply colors to terminal output
execute!(
    stdout,
    SetBackgroundColor(colors.background),
    SetForegroundColor(colors.foreground),
    Print("Sample Text"),
    ResetColor
)?;

// Or use the convenience function for crossterm
let crossterm_colors = percentage_to_colors(75);
```

## Demo Application

The included demo application `color_spectrum_demo` visualizes the entire color spectrum:

```bash
cargo run --bin color_spectrum_demo
```

This will display:
- The full gradient from 0 to 100
- Color band examples for key points in the spectrum
- Background and foreground color variations

## Integration with SpringKeys

This color spectrum is designed for integrating with performance metrics in SpringKeys:
- Typing speed visualization
- Accuracy display
- Key performance indicators

The consistent color scheme helps users intuitively understand their performance at a glance. 