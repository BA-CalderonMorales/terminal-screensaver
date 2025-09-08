# Backend Architecture Documentation

## Core Architecture

The terminal screensaver backend follows a plugin architecture pattern that enables dynamic screensaver content while maintaining clear separation between the screensaver engine and host applications.

## System Design Principles

### Screaming Architecture
The codebase structure immediately reveals the business intent:
- Feature directories contain all related functionality
- Shared utilities remain decoupled from specific features
- Clear boundaries between concerns

### Plugin Architecture
```
Host Application <-> Terminal Screensaver <-> Configuration
                              |
                         Feature Plugins
```

## Core Components

### Screensaver Engine
**Location**: `src/lib.rs`
**Responsibility**: Orchestrates the screensaver lifecycle

Key functions:
- Screen size detection and adaptation
- Input handling (ESC, ENTER, custom keys)
- Plugin lifecycle management
- Configuration loading and validation

### Feature System
**Location**: `src/feature_*/`
**Responsibility**: Individual screensaver implementations

Each feature must implement:
```rust
pub trait ScreensaverFeature {
    fn render(&self, area: Rect) -> Vec<Span>;
    fn handle_input(&mut self, key: KeyEvent) -> ScreensaverAction;
    fn resize(&mut self, new_area: Rect);
}
```

### Configuration System
**Location**: `src/shared/`
**Responsibility**: Handle `terminal-screensaver.toml` parsing

Configuration structure:
```toml
[screensaver]
default_feature = "alpha"
timeout_seconds = 60

[features.alpha]
enabled = true
text = "Custom screensaver text"
animation_speed = 1000

[custom_actions]
# Host application defines these
action_1 = "custom_command"
```

## Data Flow

1. **Initialization**
   - Load configuration from `terminal-screensaver.toml`
   - Initialize selected screensaver feature
   - Set up terminal interface

2. **Runtime Loop**
   - Capture screen dimensions
   - Render current feature content
   - Handle user input
   - Process resize events

3. **Action Delegation**
   - Capture custom key combinations
   - Pass actions back to host application
   - Maintain screensaver state

## Screen Adaptation Strategy

### Dynamic Sizing
The backend automatically handles screen resizing through:

1. **Terminal Dimension Detection**
   ```rust
   let (width, height) = crossterm::terminal::size()?;
   let area = Rect::new(0, 0, width, height);
   ```

2. **Content Scaling**
   - Text content scales proportionally
   - Animations adjust to available space
   - Layouts recalculate on resize events

3. **Responsive Design**
   - Minimum viable dimensions enforced
   - Graceful degradation for small terminals
   - Content truncation with indicators

## Error Handling

### Graceful Degradation
- Invalid configuration falls back to defaults
- Missing features disable gracefully
- Terminal errors logged but don't crash

### Logging Strategy
All backend operations log through the centralized logger:
```rust
use crate::logger::log_info;

log_info("Screensaver initialized", &[
    ("feature", &config.default_feature),
    ("dimensions", &format!("{}x{}", width, height))
]);
```

## Performance Considerations

### Rendering Optimization
- Differential rendering to minimize screen updates
- Buffer management for smooth animations
- Input handling with minimal latency

### Memory Management
- Feature instances created once and reused
- Configuration loaded at startup
- Cleanup on exit or feature switching

## Integration Points

### Host Application Interface
```rust
pub struct ScreensaverBuilder {
    config_path: Option<PathBuf>,
    custom_features: Vec<Box<dyn ScreensaverFeature>>,
}

impl ScreensaverBuilder {
    pub fn with_config(mut self, path: PathBuf) -> Self;
    pub fn add_feature(mut self, feature: Box<dyn ScreensaverFeature>) -> Self;
    pub fn build(self) -> Result<Screensaver, ScreensaverError>;
}
```

### Plugin Registration
Host applications can register custom screensaver features:
```rust
let screensaver = ScreensaverBuilder::new()
    .with_config("./screensaver.toml".into())
    .add_feature(Box::new(CustomFeature::new()))
    .build()?;
```

## Testing Strategy

### Unit Testing
Each backend component maintains isolated tests:
- Configuration parsing validation
- Feature rendering logic
- Screen adaptation algorithms

### Integration Testing
End-to-end scenarios verify:
- Complete screensaver lifecycle
- Host application integration
- Configuration changes

## Security Considerations

### Input Validation
- All user input sanitized before processing
- Configuration file validation against schema
- No execution of arbitrary commands from config

### Resource Management
- Memory usage bounded by configuration
- File system access restricted to config directory
- No network operations without explicit configuration

## Future Extensibility

### Plugin Discovery
Future versions may support:
- Dynamic feature loading
- Plugin marketplace integration
- Runtime feature installation

### Advanced Features
Planned enhancements:
- Multi-monitor support
- Color scheme customization
- Audio integration hooks