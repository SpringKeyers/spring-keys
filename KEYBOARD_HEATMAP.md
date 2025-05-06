# SpringKeys Keyboard Heatmap

The SpringKeys application features an enhanced keyboard heatmap visualization that provides detailed feedback on typing performance through a color-coded display.

## Keyboard Visualization Features

### Multi-Row Key Display

Each key on the keyboard is displayed with multiple rows of information:

1. **First Row**: Shows the character with a background color based on the geometric average speed
2. **Second Row**: Shows the last typing speed with the purple-white-red color spectrum
3. **Third Row**: Shows the geometric average speed with the background color from row 1
4. **Fourth Row**: Empty row to complete the key visual

### Color Spectrum Implementation

The second row of each key uses a special color spectrum to represent the speed of the last keystroke:

- **Purple (Minimum)**: Represents slower typing speeds (≥300ms)
- **White (Middle)**: Represents medium typing speeds (around 200ms)
- **Red (Maximum)**: Represents fastest typing speeds (≤100ms)

This provides an intuitive visual gradient where:
- Better performance (faster typing) is shown with red shades
- Average performance is shown with white/light shades
- Slower performance is shown with purple shades

### Speed Mapping

The keyboard visualization maps typing speeds to colors using the following formula:

```rust
// Map speed to a 0-100 scale for the color spectrum
// 100ms = 100 (fastest), 300ms+ = 0 (slowest)
let spectrum_value = if recent_speed <= 0.0 {
    0 // No data
} else if recent_speed <= 100.0 {
    100 // Maximum (fastest)
} else if recent_speed >= 300.0 {
    0 // Minimum (slowest)
} else {
    // Linear mapping from 100-300ms to 100-0
    ((300.0 - recent_speed) / 2.0) as u8
};
```

This creates a scale where:
- 100ms or faster = 100 (maximum/red)
- 200ms = 50 (middle/white)
- 300ms or slower = 0 (minimum/purple)

## Example Visual

```
┌────────┬────────┬────────┬────────┬────────┬────────┬────────┬────┐
│   a    │   s    │   d    │   f    │   g    │   h    │   j    │ .. │
│ 215ms  │ 156ms  │ 178ms  │ 135ms  │ 246ms  │ 195ms  │ 98ms   │ .. │
│ 230ms  │ 160ms  │ 180ms  │ 140ms  │ 250ms  │ 200ms  │ 100ms  │ .. │
└────────┴────────┴────────┴────────┴────────┴────────┴────────┴────┘
```

In this example:
- The 'j' key's second row would be red (98ms is very fast)
- The 'f' key's second row would be light red/pink
- The 's' and 'd' keys would have whitish backgrounds
- The 'a' and 'g' keys would have purple tints

## Implementation Details

The implementation combines traditional keyboard heatmap visualization with a color spectrum that transitions from purple to white to red:

1. **Traditional Heatmap**: Uses blue-green-yellow-red colors based on absolute speed thresholds
2. **Color Spectrum**: Uses purple-white-red based on a relative 0-100 scale

This dual approach allows users to see both absolute performance (traditional colors) and relative performance (spectrum colors) simultaneously.

## How to Use

The keyboard heatmap is displayed automatically during typing sessions, providing real-time feedback as you type. Watch for:

- Red in the second row of keys you type quickly
- White in the second row of keys with average speed
- Purple in the second row of keys that are challenging

Use this visual feedback to identify keys that need practice and track improvements over time. 