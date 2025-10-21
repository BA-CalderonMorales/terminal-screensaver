# Integration Test Architect Skill

This skill specializes in designing and implementing comprehensive integration test suites for CLI applications and libraries.

## When to Use This Skill
- Designing test architecture
- Creating integration test suites
- Building test fixtures and helpers
- Implementing test harnesses
- Setting up CI/CD test pipelines
- Creating test documentation

## Expertise Areas
1. Test pyramid design
2. Integration test patterns
3. Test fixture management
4. Mock and stub creation
5. Test data generation
6. Test isolation strategies

## Test Architecture Principles
- Follow test pyramid (many unit, some integration, few e2e)
- Isolate tests from each other
- Make tests deterministic
- Fast feedback loops
- Clear test naming
- Comprehensive coverage of critical paths

## Integration Test Patterns
1. **Command Execution Tests**
   - Launch binary with arguments
   - Capture stdout/stderr
   - Validate exit codes
   - Test timeout handling

2. **Configuration Tests**
   - Load configs from files
   - Test config validation
   - Test default values
   - Test config merging

3. **Feature Tests**
   - Test plugin loading
   - Test feature registration
   - Test runtime behavior
   - Test error conditions

4. **End-to-End Tests**
   - Full workflow scenarios
   - Multi-step interactions
   - State management validation

## Test Utilities to Provide
- TempDir helpers for isolated test environments
- Config file builders
- Output capture utilities
- Process execution wrappers
- Assertion helpers for CLI output
- Mock filesystem implementations

## Test Documentation Standards
- Each test clearly describes what it validates
- Test organization follows code structure
- README explains how to run tests
- Document test data requirements
- Explain test fixtures and helpers
