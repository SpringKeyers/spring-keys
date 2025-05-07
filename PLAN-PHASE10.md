# Phase 10: Song Mode with Music Playback

## Overview
This phase introduces a new song mode that synchronizes typing practice with music playback. When a song quote is selected and has an associated music file, the application will play the music while the user types the lyrics, creating an immersive typing experience.

## Design Goals
1. Create an engaging typing experience by combining lyrics with music
2. Support various audio formats (MP3, WAV, OGG)
3. Ensure smooth synchronization between typing and music playback
4. Maintain accessibility and performance
5. Provide clear visual feedback for timing and progress

## Technical Specifications

### Audio System
```rust
// New module: src/audio/mod.rs
pub struct AudioPlayer {
    current_track: Option<AudioTrack>,
    volume: f32,
    is_playing: bool,
}

pub struct AudioTrack {
    file_path: PathBuf,
    duration: Duration,
    format: AudioFormat,
}

pub enum AudioFormat {
    MP3,
    WAV,
    OGG,
}
```

### Song Mode Configuration
```rust
// New struct in src/config/mod.rs
pub struct SongModeConfig {
    pub enabled: bool,
    pub auto_play: bool,
    pub volume: f32,
    pub sync_tolerance: Duration,
    pub supported_formats: Vec<AudioFormat>,
}
```

### Quote Structure Enhancement
```rust
// Enhanced Quote struct in src/quotes.rs
pub struct Quote {
    // ... existing fields ...
    pub audio_path: Option<PathBuf>,  // Path to associated music file
    pub bpm: Option<u32>,            // Beats per minute for timing
    pub sections: Vec<LyricSection>, // Timed sections of lyrics
}

pub struct LyricSection {
    pub start_time: Duration,
    pub end_time: Duration,
    pub text: String,
}
```

## Implementation Plan

### 1. Audio System Integration
- [ ] Add audio playback dependencies (e.g., rodio, symphonia)
- [ ] Implement basic audio player with volume control
- [ ] Add support for multiple audio formats
- [ ] Create audio file management system

### 2. Song Mode UI
- [ ] Add song mode toggle in settings
- [ ] Create music player controls (play, pause, volume)
- [ ] Design visual progress indicator
- [ ] Add timing feedback display

### 3. Synchronization System
- [ ] Implement lyric timing system
- [ ] Create section-based progress tracking
- [ ] Add visual cues for upcoming sections
- [ ] Implement timing tolerance system

### 4. File Management
- [ ] Create data/music directory structure
- [ ] Implement music file validation
- [ ] Add file format detection
- [ ] Create music file indexing system

### 5. Performance Optimization
- [ ] Implement audio streaming
- [ ] Add caching system for frequently used tracks
- [ ] Optimize memory usage
- [ ] Add background loading

## Directory Structure
```
data/
  music/
    songs/
      here_comes_the_sun.mp3
      dont_stop_believing.mp3
      sweet_dreams.mp3
      ...
    metadata/
      song_timing.json
      bpm_data.json
```

## User Experience

### Song Mode Activation
1. User selects a song quote
2. System checks for associated music file
3. If found, song mode is automatically enabled
4. Music player controls appear
5. User can start typing with music

### Visual Feedback
- Progress bar showing current position in song
- Highlighted current section of lyrics
- Timing indicators for upcoming sections
- Volume and playback controls

### Controls
- Space: Start/Resume typing
- Esc: Pause music and typing
- +/-: Adjust volume
- M: Mute/Unmute
- R: Restart song

## Configuration Options
```toml
[song_mode]
enabled = true
auto_play = true
volume = 0.8
sync_tolerance = "100ms"
supported_formats = ["mp3", "wav", "ogg"]
```

## Error Handling
1. Missing audio file
   - Graceful fallback to normal typing mode
   - Clear user notification
   - Option to locate file

2. Format issues
   - Automatic format detection
   - Conversion suggestions
   - Format compatibility warnings

3. Playback issues
   - Automatic retry mechanism
   - Fallback audio system
   - Error logging and reporting

## Testing Strategy
1. Unit Tests
   - Audio system functionality
   - File format handling
   - Synchronization accuracy

2. Integration Tests
   - End-to-end song mode
   - UI responsiveness
   - Performance benchmarks

3. User Testing
   - Timing accuracy
   - User experience
   - Performance feedback

## Future Enhancements
1. Multiple audio tracks support
2. Custom timing adjustments
3. Karaoke-style highlighting
4. Recording and playback
5. Online music integration
6. Collaborative typing sessions

## Dependencies
```toml
[dependencies]
rodio = "0.17"      # Audio playback
symphonia = "0.5"   # Audio format support
hound = "3.5"       # WAV file handling
```

## Timeline
1. Week 1: Audio system implementation
2. Week 2: UI and synchronization
3. Week 3: Testing and optimization
4. Week 4: User feedback and refinement

## Success Metrics
1. Audio playback latency < 50ms
2. Synchronization accuracy > 95%
3. Memory usage < 100MB
4. User satisfaction rating > 4.5/5

## Documentation
1. User guide for song mode
2. Audio file preparation guide
3. Troubleshooting guide
4. API documentation
5. Performance tuning guide 