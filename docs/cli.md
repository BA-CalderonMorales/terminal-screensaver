# Command Line Interface Documentation

## CLI Design Philosophy

The terminal screensaver CLI follows the principle of "simple by default, powerful when needed." It provides essential functionality with minimal complexity while offering advanced configuration options for power users.

## Command Structure

### Basic Usage
```bash
terminal-screensaver [OPTIONS] [SUBCOMMAND]
```

### Core Commands

#### Start Screensaver
```bash
terminal-screensaver
terminal-screensaver start
terminal-screensaver --config ./custom-config.toml
```

#### Configuration Management
```bash
terminal-screensaver config init          # Create default config
terminal-screensaver config validate     # Validate configuration
terminal-screensaver config show         # Display current config
terminal-screensaver config edit         # Open config in default editor
```

#### Feature Management
```bash
terminal-screensaver features list       # List available features
terminal-screensaver features test alpha # Test specific feature
terminal-screensaver features benchmark  # Performance testing
```

#### Diagnostics
```bash
terminal-screensaver doctor              # System compatibility check
terminal-screensaver info                # Display system information
terminal-screensaver logs                # View recent logs
```

## Command Line Arguments

### Global Options

| Flag | Long Form | Description | Default |
|------|-----------|-------------|---------|
| `-c` | `--config` | Configuration file path | `./terminal-screensaver.toml` |
| `-v` | `--verbose` | Enable verbose logging | `false` |
| `-q` | `--quiet` | Suppress non-error output | `false` |
| `-d` | `--debug` | Enable debug mode | `false` |
| `-h` | `--help` | Display help information | - |
| `-V` | `--version` | Display version information | - |

### Feature-Specific Options

| Flag | Description | Example |
|------|-------------|---------|
| `--feature` | Select specific feature | `--feature matrix` |
| `--timeout` | Override timeout setting | `--timeout 120` |
| `--no-animation` | Disable animations | `--no-animation` |
| `--test-mode` | Run in test mode | `--test-mode` |

## Subcommands

### Configuration Commands

#### `config init`
Creates a default configuration file with comprehensive comments.

```bash
terminal-screensaver config init [--force] [--template TEMPLATE]
```

Options:
- `--force`: Overwrite existing configuration
- `--template`: Use specific configuration template

Example output:
```
Configuration file created: ./terminal-screensaver.toml
Edit this file to customize your screensaver settings.
```

#### `config validate`
Validates configuration syntax and settings.

```bash
terminal-screensaver config validate [CONFIG_PATH]
```

Example output:
```
Configuration validation: PASSED
- Default feature 'alpha' found and valid
- All custom actions properly defined
- No deprecated settings detected
```

#### `config show`
Displays current configuration in human-readable format.

```bash
terminal-screensaver config show [--raw] [--section SECTION]
```

Options:
- `--raw`: Display raw TOML format
- `--section`: Show specific configuration section only

### Feature Commands

#### `features list`
Lists all available screensaver features.

```bash
terminal-screensaver features list [--detailed]
```

Example output:
```
Available Features:
  alpha     - Default animated text screensaver
  matrix    - Matrix-style falling characters
  clock     - Digital clock display
  custom    - User-defined content display

Use 'terminal-screensaver features test FEATURE' to preview.
```

#### `features test`
Tests a specific feature without full screensaver mode.

```bash
terminal-screensaver features test FEATURE [--duration SECONDS]
```

Options:
- `--duration`: Test duration in seconds (default: 10)

### Diagnostic Commands

#### `doctor`
Performs system compatibility check and provides recommendations.

```bash
terminal-screensaver doctor [--fix] [--export-report]
```

Options:
- `--fix`: Attempt to fix detected issues automatically
- `--export-report`: Save diagnostic report to file

Example output:
```
Terminal Screensaver Doctor Report
================================

✓ Terminal compatibility: PASSED
✓ Color support: 256 colors detected
✓ Configuration file: Valid
⚠ Performance: Consider reducing animation speed for better performance
✗ Dependencies: cargo-audit not found (optional)

Overall Status: GOOD (1 warning, 1 optional issue)
```

#### `info`
Displays system and environment information.

```bash
terminal-screensaver info [--json] [--clipboard]
```

Options:
- `--json`: Output in JSON format
- `--clipboard`: Copy output to clipboard (if supported)

Example output:
```
Terminal Screensaver Information
==============================

Version: 0.0.1
Build Date: 2025-09-07
Rust Version: 1.70.0

Terminal Information:
- Type: xterm-256color
- Dimensions: 120x40
- Color Support: TrueColor
- Unicode Support: Full

Configuration:
- Config Path: ./terminal-screensaver.toml
- Default Feature: alpha
- Custom Actions: 2 defined

System:
- OS: Linux 5.15.0
- Shell: bash 5.1.8
- CPU: 8 cores
- Memory: 16GB
```

## Interactive Mode

### Configuration Wizard
```bash
terminal-screensaver config init --interactive
```

Provides guided configuration setup:
1. Feature selection with previews
2. Timeout configuration
3. Custom action setup
4. Appearance customization
5. Performance optimization

### Feature Selection Menu
When multiple features are available, interactive selection:
```
Select Screensaver Feature:
  1) Alpha - Animated text display
  2) Matrix - Character rain effect
  3) Clock - Digital clock
  4) Custom - User content
  
Enter selection (1-4): 
```

## Error Handling and Messages

### Error Categories

#### Configuration Errors
```
Error: Invalid configuration file
  ├─ File: ./terminal-screensaver.toml
  ├─ Line: 15, Column: 3
  └─ Issue: Unknown feature 'invalid_feature'

Suggestion: Run 'terminal-screensaver features list' to see available features.
```

#### Runtime Errors
```
Error: Terminal too small
  ├─ Current: 20x5
  ├─ Minimum: 40x12
  └─ Feature: alpha

Suggestion: Resize terminal or use '--feature minimal' for small terminals.
```

#### Permission Errors
```
Error: Cannot write configuration file
  ├─ Path: ./terminal-screensaver.toml
  └─ Cause: Permission denied

Suggestion: Run with appropriate permissions or specify writable path with --config.
```

### Warning Messages
```
Warning: Performance impact detected
  ├─ Cause: High CPU usage during animation
  └─ Impact: Battery life may be reduced

Suggestion: Use '--no-animation' flag or reduce animation speed in configuration.
```

## Integration with Host Applications

### Library Integration
When used as a library, CLI functionality is available programmatically:

```rust
use terminal_screensaver::cli::CliConfig;

let config = CliConfig::from_args(args)?;
let screensaver = config.build_screensaver()?;
screensaver.run()?;
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `TERMINAL_SCREENSAVER_CONFIG` | Configuration file path | `./terminal-screensaver.toml` |
| `TERMINAL_SCREENSAVER_LOG_LEVEL` | Logging level | `info` |
| `TERMINAL_SCREENSAVER_NO_COLOR` | Disable colored output | `false` |
| `TERMINAL_SCREENSAVER_DATA_DIR` | Data directory | `~/.local/share/terminal-screensaver` |

## Shell Integration

### Bash Completion
```bash
# Enable bash completion
eval "$(terminal-screensaver completion bash)"

# Or add to ~/.bashrc
echo 'eval "$(terminal-screensaver completion bash)"' >> ~/.bashrc
```

### Zsh Completion
```zsh
# Enable zsh completion
eval "$(terminal-screensaver completion zsh)"

# Or add to ~/.zshrc
echo 'eval "$(terminal-screensaver completion zsh)"' >> ~/.zshrc
```

### Fish Completion
```fish
# Enable fish completion
terminal-screensaver completion fish | source

# Or save to completions
terminal-screensaver completion fish > ~/.config/fish/completions/terminal-screensaver.fish
```

## Advanced Usage

### Scripting Integration
```bash
# Start screensaver with timeout
terminal-screensaver --timeout 300

# Run diagnostic and capture report
terminal-screensaver doctor --export-report > diagnostics.txt

# Validate configuration in CI/CD
if ! terminal-screensaver config validate --quiet; then
    echo "Configuration validation failed"
    exit 1
fi
```

### Performance Optimization
```bash
# Minimal resource usage
terminal-screensaver --feature minimal --no-animation --quiet

# High performance mode
terminal-screensaver --feature alpha --debug --verbose
```

## Troubleshooting Guide

### Common Issues

#### "Configuration file not found"
```bash
# Solution: Initialize configuration
terminal-screensaver config init
```

#### "Feature not available"
```bash
# Solution: List available features
terminal-screensaver features list
```

#### "Terminal compatibility issues"
```bash
# Solution: Run compatibility check
terminal-screensaver doctor
```

### Debug Mode
```bash
# Enable debug logging
terminal-screensaver --debug --verbose
```

Debug mode provides:
- Detailed timing information
- Configuration parsing details
- Terminal capability detection
- Performance metrics
- Error stack traces

## Future CLI Enhancements

### Planned Features
- Plugin management commands
- Theme marketplace integration
- Configuration import/export
- Remote configuration management
- Telemetry and analytics (opt-in)

### Enhanced Interactivity
- TUI-based configuration editor
- Real-time feature previews
- Interactive troubleshooting
- Guided performance optimization