# CLI Test Builder Skill

This skill specializes in building robust, testable CLI applications with minimal dependencies.

## When to Use This Skill
- Building or refactoring CLI argument parsers
- Creating integration tests for CLI applications
- Implementing test fixtures and helpers for CLI testing
- Validating CLI input/output behaviors
- Setting up CLI test harnesses

## Expertise Areas
1. CLI argument parsing and validation
2. Command-line integration testing
3. Test fixture creation
4. Input/output mocking and capturing
5. Exit code validation
6. Configuration file testing

## Implementation Guidelines
- Prefer minimal dependencies (stdlib where possible)
- Create reusable test helpers
- Implement comprehensive error handling
- Support both unit and integration tests
- Design for extensibility
- Zero unwrap() calls in production code
- Proper Result/Option error propagation

## Testing Patterns
- Test both success and failure paths
- Validate error messages
- Test edge cases (empty input, invalid flags)
- Test configuration loading from various sources
- Mock external dependencies
- Capture and validate stdout/stderr

## Code Quality Standards
- No emojis in code or output
- Clear, descriptive error messages
- Comprehensive doc comments
- Example-driven documentation
- Minimal external dependencies
