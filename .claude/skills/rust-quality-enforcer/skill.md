# Rust Quality Enforcer Skill

This skill specializes in enforcing code quality standards, removing anti-patterns, and ensuring best practices in Rust codebases.

## When to Use This Skill
- Auditing code for quality issues
- Removing emojis and non-ASCII characters from code
- Replacing unwrap() with proper error handling
- Enforcing consistent style and patterns
- Identifying and fixing unsafe patterns
- Improving error messages

## Expertise Areas
1. Error handling (Result/Option patterns)
2. Code pattern enforcement
3. Safety analysis
4. Performance optimization
5. Idiomatic Rust patterns
6. Documentation quality

## Quality Checks
- No emojis in source code, comments, or string literals
- No unwrap()/expect() in production code paths
- Proper error type definitions
- Comprehensive error context
- No unsafe code without justification
- All public APIs documented
- Clippy warnings addressed
- Consistent naming conventions

## Enforcement Actions
1. Search for and remove all emoji characters
2. Replace unwrap() with proper error handling
3. Add missing error context
4. Improve error message clarity
5. Add documentation where missing
6. Enforce consistent formatting
7. Validate public API surface

## Code Transformation Patterns
- unwrap() → ? operator with context
- String literals → validated constants
- Panic paths → Result returns
- Generic errors → specific error types
- Missing docs → comprehensive documentation
