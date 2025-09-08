# Quick Start Guide

## Installation

### From Source (Current)

```bash
# Clone the repository
git clone https://github.com/yourusername/terminal-screensaver.git
cd terminal-screensaver

# Build and run
cargo build --release
./target/release/terminal-screensaver
```

### Pre-built Binaries (Coming in v0.0.2)

```bash
# macOS (Homebrew)
brew install terminal-screensaver

# Windows (Chocolatey) 
choco install terminal-screensaver

# Linux (Package managers)
# Ubuntu/Debian
sudo apt install terminal-screensaver

# Arch Linux
yay -S terminal-screensaver
```

## Basic Usage

### As a Standalone Application

```bash
# Run with default settings
terminal-screensaver

# Run with custom configuration
terminal-screensaver -c my-config.toml

# Show help
terminal-screensaver --help
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
terminal-screensaver = "0.0.1"
```

Use in your Rust code:

```rust
use terminal_screensaver::{Config, run_screensaver};

fn main() {
    let config = Config {
        text: "My Custom Screensaver".to_string(),
        style: "default".to_string(),
        actions: vec![],
    };
    
    // Run the screensaver (this will block until user exits)
    run_screensaver(config);
}
```

## First Configuration

Create a `terminal-screensaver.toml` file:

```toml
# Basic configuration
text = "Welcome to My Terminal"
style = "default"
```

Run the screensaver:

```bash
terminal-screensaver -c terminal-screensaver.toml
```

## Adding Script Integration

### 1. Create Your First Script

Create `scripts/hello.sh`:

```bash
#!/bin/bash
echo "Hello from Terminal Screensaver!"
echo "Current time: $(date)"
echo ""
read -p "Press Enter to return to screensaver..."
```

Make it executable:

```bash
chmod +x scripts/hello.sh
```

### 2. Update Configuration

Add to your `terminal-screensaver.toml`:

```toml
text = "My Terminal Screensaver"
style = "default"

[[actions]]
key = "h"
description = "Say Hello"
command = "./scripts/hello.sh"
```

### 3. Test Your Setup

```bash
terminal-screensaver -c terminal-screensaver.toml
```

- Press `h` to run your script
- Press `ESC` to exit
- Press `ENTER` to see all available actions

## Common Use Cases

### Development Workflow

```toml
text = "Development Environment"
style = "green"

[[actions]]
key = "s"
description = "Git Status"
command = "git status && read -p 'Press Enter...'"

[[actions]]
key = "l"
description = "Git Log"
command = "git log --oneline -10 && read -p 'Press Enter...'"

[[actions]]
key = "t"
description = "Run Tests"
command = "cargo test && read -p 'Press Enter...'"
```

### System Monitoring

```toml
text = "System Monitor"
style = "blue"

[[actions]]
key = "c"
description = "CPU Usage"
command = "top -b -n1 | head -10 && read -p 'Press Enter...'"

[[actions]]
key = "m"
description = "Memory Usage"
command = "free -h && read -p 'Press Enter...'"

[[actions]]
key = "d"
description = "Disk Usage"
command = "df -h && read -p 'Press Enter...'"
```

### Quick Notes

```toml
text = "Quick Notes Terminal"
style = "yellow"

[[actions]]
key = "n"
description = "Add Note"
command = "echo 'Note:' && read -p '' note && echo \"$(date): $note\" >> notes.txt && echo 'Note saved!' && read -p 'Press Enter...'"

[[actions]]
key = "v"
description = "View Notes"
command = "cat notes.txt 2>/dev/null || echo 'No notes yet' && read -p 'Press Enter...'"
```

## Controls Reference

| Key | Action |
|-----|--------|
| `ESC` | Exit screensaver |
| `ENTER` | Show action menu |
| `a-z, A-Z, 0-9` | Execute configured action (if defined) |

## Keyboard Shortcuts

- **Always Available**:
  - `ESC` - Exit screensaver
  - `ENTER` - Show action menu
  
- **Configurable**: Any single character can be assigned to custom actions via the configuration file

## Troubleshooting Quick Fixes

### Script Not Running

```bash
# Make sure script is executable
chmod +x your-script.sh

# Test script directly
./your-script.sh
```

### Configuration Issues

```bash
# Validate configuration
terminal-screensaver --validate-config -c your-config.toml

# Check for TOML syntax errors
# Use online TOML validators if needed
```

### Permission Errors

```bash
# Run with debug logging
TERMINAL_SCREENSAVER_LOG_LEVEL=debug terminal-screensaver -c config.toml
```

## Next Steps

1. **Explore Examples**: Check out `demo-config.toml` for more examples
2. **Read Documentation**: See `docs/configuration.md` for comprehensive configuration options
3. **Create Scripts**: Build custom scripts for your workflow
4. **Share Configurations**: Share your configurations with the community

## Getting Help

- **Documentation**: Check the `docs/` folder for comprehensive guides
- **Issues**: Report bugs or request features on GitHub
- **Discussions**: Join community discussions for tips and tricks
- **Examples**: Browse the `scripts/demo/` folder for inspiration

## What's Next?

- Explore advanced configuration options in [`docs/configuration.md`](configuration.md)
- Learn about the project architecture in [`docs/architecture.md`](architecture.md)
- Check out upcoming features in [`docs/roadmap.md`](roadmap.md)
- Dive into development with [`docs/backend.md`](backend.md)
