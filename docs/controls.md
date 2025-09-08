# Controls & Keyboard Shortcuts

## System Controls

These controls are always available regardless of configuration:

| Key | Action | Description |
|-----|--------|-------------|
| `ESC` | Exit Screensaver | Immediately exit the screensaver and return to terminal |
| `ENTER` | Show Action Menu | Display interactive menu of all configured actions |

## Configurable Actions

All other keys can be customized through the TOML configuration file. The screensaver supports any single character as a keyboard shortcut.

### Supported Key Types

- **Letters**: `a-z`, `A-Z` (case-sensitive)
- **Numbers**: `0-9`  
- **Symbols**: Most printable symbols (`,`, `.`, `/`, `;`, `'`, `[`, `]`, etc.)

### Configuration Example

```toml
# Define custom keyboard shortcuts
[[actions]]
key = "d"                           # Lowercase 'd'
description = "Development Status"   
command = "./scripts/dev-status.sh" 

[[actions]]
key = "G"                           # Uppercase 'G' (different from lowercase)
description = "Git Actions"         
command = "./scripts/git-quick.sh"  

[[actions]]
key = "1"                           # Number key
description = "Quick Command"       
command = "echo 'Hello!' && read"   

[[actions]]
key = "/"                           # Symbol key
description = "Help"                
command = "./scripts/help.sh"       
```

## Interactive Help Display

The screensaver includes a built-in help panel that shows all available keyboard shortcuts.

### Help Panel Features

- **Always Visible**: Help panel appears in the top-right corner by default
- **Dynamic Content**: Automatically updates based on your configuration
- **Professional Styling**: Unicode borders with clean, readable layout
- **Smart Layout**: Automatically avoids overlapping with main content

### Help Panel Contents

The help panel displays:
1. **System Controls**: ESC and ENTER shortcuts
2. **Custom Actions**: All configured action shortcuts with descriptions
3. **Action Menu**: Information about the ENTER menu option

## Action Menu

Pressing `ENTER` opens an interactive action menu that lists all configured actions:

```
=== Available Actions ===
1. Development Environment Status (Press 'd')
2. Git Quick Actions (Press 'g')
3. Help Information (Press 'h')
ESC. Return to screensaver

Press any key to continue...
```

### Menu Features

- **Numbered List**: All actions with their descriptions
- **Key Reminders**: Shows which key to press for each action
- **Easy Navigation**: Press any key to return to screensaver
- **Consistent Styling**: Matches overall screensaver aesthetic

## Best Practices for Key Assignment

### Recommended Conventions

1. **Mnemonic Keys**: Use first letter of action name when possible
   - `d` for Development, `g` for Git, `h` for Help

2. **Frequency-Based**: Assign easily-typed keys to frequently-used actions
   - Use `a-z` for common actions
   - Use `1-9` for numbered workflows
   - Use symbols for special or rare actions

3. **Avoid Conflicts**: Each key can only be assigned to one action
   - Configuration validation will catch duplicate keys
   - Case-sensitive: `d` and `D` are different keys

### Example Key Schemes

#### Development Workflow
```toml
[[actions]]
key = "s"
description = "Git Status"
command = "git status && read -p 'Press Enter...'"

[[actions]]
key = "l"  
description = "Git Log"
command = "git log --oneline -5 && read -p 'Press Enter...'"

[[actions]]
key = "t"
description = "Run Tests"
command = "cargo test && read -p 'Press Enter...'"

[[actions]]
key = "b"
description = "Build Project"
command = "cargo build && read -p 'Press Enter...'"
```

#### System Administration
```toml
[[actions]]
key = "c"
description = "CPU Usage"
command = "top -b -n1 | head -15 && read -p 'Press Enter...'"

[[actions]]
key = "m"
description = "Memory Usage"  
command = "free -h && read -p 'Press Enter...'"

[[actions]]
key = "d"
description = "Disk Usage"
command = "df -h && read -p 'Press Enter...'"

[[actions]]
key = "p"
description = "Process List"
command = "ps aux | head -20 && read -p 'Press Enter...'"
```

#### Quick Access Numbers
```toml
[[actions]]
key = "1"
description = "Quick Status"
command = "./scripts/quick-status.sh"

[[actions]]
key = "2"  
description = "Deploy Staging"
command = "./scripts/deploy-staging.sh"

[[actions]]
key = "3"
description = "Run Backup"
command = "./scripts/backup.sh"
```

## Advanced Control Features

### Script Integration

When you press a configured key:
1. **Terminal Preparation**: Screensaver cleanly exits raw mode
2. **Script Execution**: Your script runs with full terminal access
3. **User Interaction**: Script can use input, colors, and full terminal features  
4. **Clean Return**: After script completion, screensaver resumes seamlessly

### Error Handling

- **Invalid Keys**: Unrecognized keys are ignored (no action)
- **Script Errors**: Failed scripts show error message and return to screensaver
- **Permissions**: Permission errors are logged and displayed to user
- **Missing Scripts**: Clear error messages for missing or inaccessible scripts

### Cross-Platform Compatibility

The control system works consistently across all supported platforms:

- **Windows**: Full support in Command Prompt, PowerShell, and Windows Terminal
- **macOS**: Compatible with Terminal.app, iTerm2, and other terminal emulators  
- **Linux**: Works with all major terminal emulators and distributions

## Accessibility

### Keyboard-Only Operation

- **No Mouse Required**: All functionality accessible via keyboard
- **Clear Visual Feedback**: Help panel shows all available options
- **Consistent Behavior**: Predictable key responses across all features

### Screen Reader Compatibility

- **Text-Based Interface**: All content is plain text, compatible with screen readers
- **Logical Layout**: Help panel and menus use logical reading order
- **Clear Descriptions**: All actions have descriptive names and purposes

## Troubleshooting Controls

### Common Issues

1. **Key Not Responding**:
   - Check if key is defined in configuration
   - Verify TOML syntax is correct
   - Ensure no duplicate key assignments

2. **Script Not Running**:
   - Verify script path is correct
   - Check script permissions (`chmod +x script.sh`)
   - Test script independently

3. **Help Panel Missing**:
   - Help panel is always visible in current version
   - Check terminal size (very small terminals may not show help)
   - Verify configuration file is loading correctly

### Debug Mode

Enable debug logging to troubleshoot control issues:

```bash
TERMINAL_SCREENSAVER_LOG_LEVEL=debug terminal-screensaver -c config.toml
```

This will show detailed information about:
- Configuration parsing
- Key press detection
- Script execution
- Error conditions

---

*For more information about configuration options, see [`docs/configuration.md`](configuration.md).*
