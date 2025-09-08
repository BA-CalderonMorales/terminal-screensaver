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
speed = 5

[features.clock_display]
enabled = true
format = "24hour"
show_seconds = true
```

Available styles: `default`, `red`, `blue`, `green`

## Controls

- **ESC**: Exit the screen saver
- **Enter**: Trigger additional options (extensible)

## Architecture

The project follows a screaming architecture with clear separation of concerns:

- `src/cli/`: Command-line interface and configuration
- `src/feature_alpha/`: Core screen saver implementation
- `src/logger/`: Centralized logging
- `src/shared/`: Shared utilities
- `src/styles/`: UI styling system

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
