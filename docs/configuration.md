# Configuration Guide

## Overview

Terminal Screensaver uses TOML configuration files for all customization. This allows for powerful, flexible configuration without requiring code changes.

## Configuration File Structure

### Basic Configuration

```toml
# Basic screensaver text and style
text = "Welcome to Terminal Screensaver"
style = "default"
```

### Script Integration Configuration

```toml
text = "Professional Terminal Screensaver"
style = "default"

# Define custom actions with keyboard shortcuts
[[actions]]
key = "d"
description = "Development Environment Status"
command = "./scripts/dev-status.sh"

[[actions]]
key = "g"
description = "Git Quick Actions"
command = "./scripts/git-quick.sh"

[[actions]]
key = "h"
description = "Help Information"
command = "./scripts/help.sh"
```

## Configuration Options

### Text Display

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `text` | String | `"Terminal Screensaver"` | Main text to display |
| `style` | String | `"default"` | Color scheme to use |

### Available Styles

- **`default`**: Standard white text with cyan borders
- **`red`**: Red-themed color scheme
- **`blue`**: Blue-themed color scheme
- **`green`**: Green-themed color scheme

### Action Configuration

Actions are defined using the `[[actions]]` array syntax:

```toml
[[actions]]
key = "x"                    # Single character shortcut key
description = "My Action"    # Description shown in help panel
command = "./my-script.sh"   # Command or script to execute
```

#### Action Properties

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `key` | String | Yes | Single character keyboard shortcut |
| `description` | String | Yes | Human-readable description for help display |
| `command` | String | Yes | Shell command or script path to execute |

#### Command Examples

```toml
# Shell script
[[actions]]
key = "s"
description = "System Status"
command = "./scripts/system-status.sh"

# Python script
[[actions]]
key = "p"
description = "Python Info"
command = "python3 -c 'import sys; print(sys.version); input()'"

# Node.js script
[[actions]]
key = "n"
description = "Node Version"
command = "node -e 'console.log(process.version); process.stdin.resume();'"

# Direct shell commands
[[actions]]
key = "t"
description = "Current Time"
command = "date && read -p 'Press Enter to continue...'"
```

## Advanced Configuration

### Future Features (v0.0.3+)

```toml
# Example of planned advanced features
text = "Advanced Terminal Screensaver"
style = "matrix"

[features.matrix_rain]
enabled = true
charset = "katakana"
speed = 5
density = 0.8
color_fade = true

[features.clock_display]
enabled = true
format = "24hour"
show_seconds = true
timezone = "UTC"
position = "bottom-right"

[features.system_monitor]
enabled = true
show_cpu = true
show_memory = true
show_disk = false
update_interval = 1000

[animation]
fps = 30
smooth_transitions = true
```

## Configuration File Locations

### Default Locations

1. **Current Directory**: `./terminal-screensaver.toml`
2. **User Config**: `~/.config/terminal-screensaver/config.toml`
3. **System Config**: `/etc/terminal-screensaver/config.toml`

### Custom Configuration

```bash
# Use custom configuration file
terminal-screensaver -c /path/to/custom-config.toml
terminal-screensaver --config /path/to/custom-config.toml
```

## Script Integration Best Practices

### Script Requirements

- **Exit Handling**: Scripts should handle cleanup properly
- **User Input**: Use `read` or `input()` to pause for user interaction
- **Error Handling**: Include proper error handling and status codes
- **Platform Compatibility**: Consider cross-platform compatibility

### Example Script Template

```bash
#!/bin/bash
# example-script.sh

echo "=== My Custom Action ==="
echo ""

# Your script logic here
echo "Performing custom action..."

# Handle errors
if [ $? -ne 0 ]; then
    echo "Error occurred!"
    read -p "Press Enter to continue..."
    exit 1
fi

echo "Action completed successfully!"
read -p "Press Enter to return to screensaver..."
```

### Python Script Template

```python
#!/usr/bin/env python3
# example-script.py

def main():
    print("=== My Python Action ===")
    print()
    
    try:
        # Your Python logic here
        print("Performing Python action...")
        
        # Simulate some work
        import time
        time.sleep(1)
        
        print("Action completed successfully!")
        
    except Exception as e:
        print(f"Error: {e}")
    
    finally:
        input("Press Enter to return to screensaver...")

if __name__ == "__main__":
    main()
```

## Configuration Validation

The screensaver automatically validates configuration files and provides helpful error messages:

- **TOML Syntax**: Validates proper TOML format
- **Required Fields**: Ensures all required fields are present
- **Key Conflicts**: Detects duplicate keyboard shortcuts
- **Command Validation**: Checks if scripts/commands are accessible

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `TERMINAL_SCREENSAVER_CONFIG` | Override config file path | None |
| `TERMINAL_SCREENSAVER_LOG_LEVEL` | Set log level (error, warn, info, debug) | `info` |
| `NO_COLOR` | Disable color output | None |

## Troubleshooting

### Common Issues

1. **Script Not Found**: Ensure script paths are correct and executable
2. **Permission Denied**: Check script permissions (`chmod +x script.sh`)
3. **TOML Parse Error**: Validate TOML syntax using online validators
4. **Key Conflicts**: Ensure each action has a unique key

### Debug Mode

```bash
# Enable debug logging
TERMINAL_SCREENSAVER_LOG_LEVEL=debug terminal-screensaver -c config.toml

# Check configuration parsing
terminal-screensaver --validate-config -c config.toml
```

## Migration Guide

When updating between versions, configuration migration is handled automatically. However, you may want to review new features and options available in newer versions.

For breaking changes (rare), migration guides will be provided in release notes.

---

*For more configuration examples, see the `demo-config.toml` file in the project repository.*
