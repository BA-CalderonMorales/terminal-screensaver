# Testing Documentation

## Testing Philosophy

The terminal screensaver project follows a comprehensive testing strategy that ensures reliability, performance, and compatibility across different terminal environments while maintaining the plugin architecture integrity.

## Testing Architecture

### Testing Pyramid Structure

```
                    E2E Tests
                  ┌─────────────┐
                  │   Manual    │
                  │  Integration │
                ┌─┴─────────────┴─┐
                │   Integration   │
                │     Tests       │
              ┌─┴─────────────────┴─┐
              │     Unit Tests      │
            ┌─┴─────────────────────┴─┐
            │    Property Tests       │
          └─────────────────────────────┘
```

### Test Organization

#### Co-located Testing
Each feature maintains its tests alongside implementation:
```
src/feature_alpha/
├── feature_alpha_logic.rs
├── feature_alpha_logic_tests.rs  # Unit tests
└── mod.rs
```

#### Integration Testing
```
tests/
├── integration/
│   ├── cli_integration_tests.rs
│   ├── configuration_tests.rs
│   ├── feature_integration_tests.rs
│   └── terminal_compatibility_tests.rs
├── fixtures/
│   ├── test_configs/
│   └── test_terminals/
└── common/
    └── test_helpers.rs
```

## Unit Testing Strategy

### Core Components Testing

#### Configuration Testing
**Location**: `src/shared/shared_logic_tests.rs`

Tests cover:
- TOML parsing validation
- Default value assignment
- Invalid configuration handling
- Configuration merging logic

```rust
#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn test_default_configuration_loads() {
        let config = ScreensaverConfig::default();
        assert_eq!(config.default_feature, "alpha");
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn test_invalid_feature_falls_back() {
        let toml_content = r#"
        [screensaver]
        default_feature = "nonexistent"
        "#;
        
        let config = ScreensaverConfig::from_str(toml_content)?;
        assert_eq!(config.default_feature, "alpha"); // Fallback
    }
}
```

#### Feature Testing
**Location**: `src/feature_*/feature_*_logic_tests.rs`

Each feature implements standardized tests:
- Rendering with different screen sizes
- Input handling validation
- Animation state management
- Resource cleanup

```rust
#[cfg(test)]
mod alpha_feature_tests {
    use super::*;

    #[test]
    fn test_renders_on_small_screen() {
        let mut feature = AlphaFeature::new();
        let small_area = Rect::new(0, 0, 20, 5);
        
        let result = feature.render(small_area);
        assert!(!result.is_empty());
        // Content should fit in small area
        assert!(result.len() <= 5);
    }

    #[test]
    fn test_handles_escape_key() {
        let mut feature = AlphaFeature::new();
        let action = feature.handle_input(KeyEvent::from(KeyCode::Esc));
        assert_eq!(action, ScreensaverAction::Exit);
    }
}
```

### Shared Utilities Testing
**Location**: `src/shared/shared_logic_tests.rs`

Comprehensive testing of shared functionality:
- Screen dimension calculations
- Text wrapping algorithms  
- Animation timing functions
- Utility functions

### Logger Testing
**Location**: `src/logger/logger_logic_tests.rs`

Validates logging infrastructure:
- Log level filtering
- Output formatting
- File rotation
- Performance impact measurement

## Integration Testing

### CLI Integration Tests
**Location**: `tests/integration/cli_integration_tests.rs`

Tests complete CLI workflows:
- Command parsing and execution
- Configuration file handling
- Error message clarity
- Exit code consistency

```rust
#[test]
fn test_config_init_creates_file() {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("test-config.toml");
    
    let result = Command::new(env!("CARGO_BIN_EXE_terminal-screensaver"))
        .args(&["config", "init", "--config", config_path.to_str().unwrap()])
        .output()?;
    
    assert!(result.status.success());
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path)?;
    assert!(config_content.contains("[screensaver]"));
}
```

### Feature Integration Tests
**Location**: `tests/integration/feature_integration_tests.rs`

Tests feature interaction with the screensaver engine:
- Feature loading and initialization
- Runtime feature switching
- Error handling and recovery
- Resource management

### Configuration Integration Tests
**Location**: `tests/integration/configuration_tests.rs`

End-to-end configuration testing:
- Configuration file discovery
- Environment variable override
- Configuration validation
- Runtime configuration changes

## Terminal Compatibility Testing

### Test Matrix

| Terminal Type | Color Support | Unicode | Size Range |
|---------------|---------------|---------|------------|
| xterm | 256 color | Full | 20x5 to 200x80 |
| gnome-terminal | True color | Full | 40x12 to 160x60 |
| Windows Terminal | True color | Full | 30x8 to 180x70 |
| macOS Terminal | 256 color | Full | 25x7 to 150x50 |
| tmux | Depends | Full | Variable |
| screen | Limited | Basic | Variable |

### Automated Terminal Testing
**Location**: `tests/integration/terminal_compatibility_tests.rs`

```rust
#[test]
fn test_renders_correctly_across_terminals() {
    let test_cases = vec![
        ("xterm", 80, 24, ColorCapability::Color256),
        ("screen", 40, 12, ColorCapability::Basic),
        ("tmux", 120, 40, ColorCapability::TrueColor),
    ];
    
    for (terminal_type, width, height, color_cap) in test_cases {
        let mut screensaver = setup_test_screensaver(terminal_type);
        screensaver.set_dimensions(width, height);
        screensaver.set_color_capability(color_cap);
        
        let result = screensaver.render_frame();
        assert_rendering_valid(&result, width, height);
    }
}
```

## Performance Testing

### Benchmarking Suite
**Location**: `benches/`

Performance benchmarks for:
- Rendering performance across screen sizes
- Memory usage during long-running sessions
- CPU utilization during animations
- Startup time measurement

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_rendering_performance(c: &mut Criterion) {
    c.bench_function("alpha_feature_render_large", |b| {
        let mut feature = AlphaFeature::new();
        let large_area = Rect::new(0, 0, 200, 80);
        
        b.iter(|| {
            feature.render(large_area)
        });
    });
}

criterion_group!(benches, benchmark_rendering_performance);
criterion_main!(benches);
```

### Performance Regression Testing
Automated detection of performance regressions:
- Rendering time increases
- Memory leak detection
- CPU usage spikes
- Battery impact measurement

## Property-Based Testing

### Hypothesis Testing
**Location**: Throughout test files using `proptest`

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_text_wrapping_never_exceeds_width(
        text in ".*",
        width in 10u16..200u16
    ) {
        let wrapped = wrap_text(&text, width);
        for line in wrapped {
            assert!(line.len() <= width as usize);
        }
    }

    #[test] 
    fn test_screen_resize_preserves_content(
        initial_width in 20u16..100u16,
        initial_height in 5u16..50u16,
        new_width in 20u16..100u16,
        new_height in 5u16..50u16
    ) {
        let mut feature = AlphaFeature::new();
        let initial_area = Rect::new(0, 0, initial_width, initial_height);
        let new_area = Rect::new(0, 0, new_width, new_height);
        
        let initial_render = feature.render(initial_area);
        feature.resize(new_area);
        let new_render = feature.render(new_area);
        
        // Core content should be preserved
        assert_content_preserved(&initial_render, &new_render);
    }
}
```

## Test Data Management

### Test Fixtures
**Location**: `tests/fixtures/`

Standardized test data:
- Configuration file templates
- Terminal capability profiles
- Expected output samples
- Performance baseline data

### Mock Objects
Terminal interaction mocking:
```rust
pub struct MockTerminal {
    size: (u16, u16),
    color_capability: ColorCapability,
    input_buffer: VecDeque<KeyEvent>,
}

impl MockTerminal {
    pub fn simulate_resize(&mut self, width: u16, height: u16) {
        self.size = (width, height);
    }
    
    pub fn simulate_input(&mut self, key: KeyEvent) {
        self.input_buffer.push_back(key);
    }
}
```

## Test Automation

### Local CI Integration
The `local-ci.sh` script includes comprehensive testing:
```bash
# Run all test categories
./local-ci.sh

# Test phases included:
# 1. Unit tests
# 2. Integration tests  
# 3. Performance benchmarks
# 4. Compatibility tests
# 5. Documentation tests
```

### Test Categories in CI

#### Fast Tests (< 5 seconds)
- Unit tests
- Basic integration tests
- Configuration validation

#### Medium Tests (< 30 seconds)
- Feature integration tests
- CLI integration tests
- Performance regression tests

#### Slow Tests (> 30 seconds)
- Full compatibility matrix
- Long-running stability tests
- Memory leak detection
- Battery impact testing

## Coverage Requirements

### Minimum Coverage Thresholds
- Unit tests: 90% line coverage
- Integration tests: 80% feature coverage
- CLI tests: 100% command coverage
- Error handling: 95% error path coverage

### Coverage Reporting
```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# View coverage report
open coverage/tarpaulin-report.html
```

## Quality Gates

### Pre-commit Requirements
All tests must pass:
- Unit tests: ✓
- Integration tests: ✓
- Performance benchmarks: Within thresholds
- Security audit: No vulnerabilities
- Documentation tests: ✓

### Release Requirements
Additional requirements for releases:
- Full compatibility matrix testing
- Performance regression analysis
- Memory leak verification
- Security penetration testing
- Documentation completeness audit

## Testing Best Practices

### Test Structure
1. **Arrange**: Set up test conditions
2. **Act**: Execute the behavior being tested
3. **Assert**: Verify expected outcomes
4. **Cleanup**: Restore initial state

### Test Naming Convention
```rust
#[test]
fn test_[component]_[scenario]_[expected_result]() {
    // Example: test_alpha_feature_small_screen_renders_content
}
```

### Error Testing
Always test error conditions:
```rust
#[test]
fn test_invalid_configuration_returns_error() {
    let result = ScreensaverConfig::from_invalid_toml();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}
```

## Continuous Testing Strategy

### Development Workflow Testing
1. **Feature branch**: Unit tests + basic integration
2. **Pull request**: Full test suite + performance regression
3. **Main branch**: Complete compatibility matrix
4. **Release**: Extended stability and security testing

### Test Environment Management
- Isolated test environments for each feature
- Consistent test data across environments
- Automated test environment provisioning
- Test result archiving and analysis

## Future Testing Enhancements

### Planned Improvements
- Visual regression testing for terminal output
- Automated accessibility testing
- Cross-platform compatibility automation
- Performance monitoring integration
- Fuzz testing for input handling