# Terminal Screensaver

A dynamic terminal screen saver crate with plugin architecture that automatically resizes with any screen size.

## Demo

<div style="border-radius: 8px; overflow: hidden; display: inline-block;">
    <img src="screenshots_and_demo/terminal-screensaver-ezgif.gif" alt="Terminal Screensaver Demo" style="display: block;">
</div>

## Features

- **Dynamic Sizing**: Automatically adapts to any terminal size
- **Multiple Screensaver Types**: 7 built-in screensaver features
- **Plugin Architecture**: Extensible event handling for custom actions
- **TOML Configuration**: Easy customization without code changes
- **Cross-platform**: Works on Windows, Linux, and macOS
- **Library & Binary**: Use as a library or standalone application
- **Interactive Help Display**: Built-in help panel with keyboard shortcuts

## Built-in Screensaver Features

| Feature | Description |
|---------|-------------|
| **text_display** | Static and animated text display with customizable content |
| **matrix_rain** | Matrix-style falling character rain animation |
| **clock_display** | Digital clock display with multiple format options |
| **bouncing_logo** | Animated bouncing text or logo with physics |
| **system_info** | Live system information and runtime statistics |
| **wave_animation** | Smooth ASCII wave animations with physics simulation |
| **starfield** | 3D starfield simulation with depth and movement |

## Quick Start

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
terminal-screensaver = "0.0.1"
```

Use in your code:

```rust
use terminal_screensaver::{Config, TextDisplayFeature, ScreensaverAction};

// Create a screensaver feature
let mut feature = TextDisplayFeature::new();

// Or use the high-level API
let config = Config {
    text: "Welcome to My App".to_string(),
    style: "default".to_string(),
};

// This will show a screen saver until ESC is pressed
run_screensaver(config);
```

### As a Binary

```bash
cargo install terminal-screensaver
terminal-screensaver --config my-config.toml
```

## Configuration

Create a `terminal-screensaver.toml` file:

```toml
[screensaver]
default_feature = "text_display"
timeout_seconds = 60

[features.text_display]
enabled = true
text = "Welcome to Terminal Screensaver v0.0.1"
style = "default"

[features.matrix_rain]
enabled = true
charset = "katakana"
```

### Script Integration

For simple text display with script integration, use this configuration format:

```toml
text = "Professional Terminal Screensaver"
style = "default"

# Execute any script or command with custom keyboard shortcuts
[[actions]]
key = "d"
description = "Development Status"
command = "./scripts/dev-status.sh"

[[actions]]
key = "g"
description = "Git Actions"
command = "./scripts/git-quick.sh"

[[actions]]
key = "s"
description = "System Info"
command = "python3 -c 'import platform; print(platform.system()); input()'"
```

**Script Integration Features:**
- Execute any script (Bash, Python, Node.js, etc.)
- Custom keyboard shortcuts
- Development workflow integration
- Professional terminal handling
- Extensible action system
speed = 5

[features.clock_display]
enabled = true
format = "24hour"
show_seconds = true
```

Available styles: `default`, `red`, `blue`, `green`

## Controls

- **ESC**: Exit the screen saver
- **ENTER**: Trigger additional options (extensible)
- **H**: Toggle help display panel

## Architecture

The project follows a screaming architecture with clear separation of concerns:

- `src/cli/`: Command-line interface and configuration
- `src/feature_alpha/`: Core screen saver implementation
- `src/logger/`: Centralized logging
- `src/shared/`: Shared utilities
- `src/styles/`: UI styling system

## Roadmap

### v0.0.2 - Multi-Platform & Distribution (Planned)

**Multi-Platform Build System**
- Cross-compilation support for all major platforms
- Automated build scripts for Windows, macOS (Intel & ARM), and Linux (x64 & ARM64)
- Platform-specific optimizations and binary packaging
- GitHub Actions integration for automated releases
- Docker-based cross-compilation environments

**Enhanced Distribution**
- Pre-built binaries for all platforms
- Package manager integration (Homebrew, Chocolatey, APT)
- Installation scripts and platform-specific installers
- Comprehensive platform testing and validation

**Platform Features**
- Native Windows console integration
- macOS Terminal.app optimizations
- Linux distribution compatibility testing
- ARM64 performance optimizations

### v0.0.3 - Advanced Features (Future)

**Enhanced Screensavers**
- Matrix rain animation restoration
- Clock display with multiple time zones
- Bouncing logo physics simulation
- System resource monitoring displays
- Wave animation with audio visualization
- 3D starfield with configurable parameters

**Plugin System**
- Hot-reloadable plugin architecture
- Custom animation framework
- Theme engine with color customization
- Performance monitoring and profiling tools
- External API integration capabilities

**Developer Experience**
- VS Code extension for configuration editing
- Live preview and testing tools
- Documentation website with interactive examples
- Community plugin marketplace
- Comprehensive API documentation

### Long-term Vision

- Network-aware screensavers with remote data
- AI-powered adaptive animations
- Multi-monitor support and synchronization
- Corporate deployment and management tools
- Integration with terminal multiplexers (tmux, screen)

## License

Licensed under MIT license.
