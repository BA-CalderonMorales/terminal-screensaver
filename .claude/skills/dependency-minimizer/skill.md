# Dependency Minimizer Skill

This skill specializes in analyzing and reducing external dependencies while maintaining or improving functionality.

## When to Use This Skill
- Auditing project dependencies
- Replacing heavy dependencies with lightweight alternatives
- Implementing stdlib-only solutions
- Analyzing dependency trees
- Identifying unnecessary dependencies
- Creating zero-dependency abstractions

## Expertise Areas
1. Dependency analysis and visualization
2. Stdlib API knowledge
3. Minimal implementation patterns
4. Feature flag optimization
5. Optional dependency design
6. Transitive dependency reduction

## Analysis Process
1. Identify all direct dependencies
2. Map actual usage of each dependency
3. Evaluate stdlib alternatives
4. Assess transitive dependency impact
5. Calculate dependency weight (compile time, binary size)
6. Propose replacement strategies

## Replacement Strategies
- Heavy logging → simple macros or conditional compilation
- Complex parsers → focused implementations
- Large frameworks → targeted utilities
- Convenience wrappers → direct implementations

## Minimal Dependency Principles
- Use stdlib first, dependencies only when necessary
- Prefer feature-gated optional dependencies
- Implement simple functionality directly
- Avoid dependency cascades
- Minimize transitive dependencies
- Consider compilation time impact
- Evaluate maintenance burden

## Zero-Dependency Patterns
- Simple logging via eprintln! + macros
- Basic TOML parsing (if truly needed, keep minimal parser)
- Direct terminal I/O via std::io
- Command-line parsing via std::env::args
