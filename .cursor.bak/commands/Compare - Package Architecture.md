# Generate Architecture Comparison Command

## Purpose

Generates comprehensive architectural comparison documents for specified package groups using Deep Package Comprehension (DPC) analysis.

## Usage

```
/Generate-Architecture-Comparison <package-group> [options]
```

## Package Groups

- `all-core` - All core packages (`packages/*/core`)
- `all-ext` - All extension packages (`packages/*/ext`)
- `all-libs` - All library packages (`libs/*`)
- `all-tools` - All tool packages (`libs/tools/*`)
- `all-shared` - All shared packages (`libs/shared`)
- `all-packages` - All packages in the workspace
- `custom:<pattern>` - Custom package pattern (e.g., `custom:packages/ghost-writer/*`)

## Options

- `--dimensions <count>` - Number of architectural dimensions to analyze (default: 20)
- `--output <path>` - Custom output path (default: `docs/analysis/`)
- `--no-dpc` - Skip DPC analysis (default: DPC enabled)
- `--format <format>` - Output format: `markdown`, `json`, `html` (default: `markdown`)

## Examples

```
/Generate-Architecture-Comparison all-core
/Generate-Architecture-Comparison all-libs --dimensions 8
/Generate-Architecture-Comparison custom:packages/ghost-writer/* --output docs/ghost-writer/
/Generate-Architecture-Comparison all-packages --format json
/Generate-Architecture-Comparison all-core --no-dpc
```

## Output

Creates a comprehensive comparison document with:

- Package analysis matrix across 20 architectural dimensions
- Detailed compliance assessment with specific evidence
- DPC-enhanced findings (dependency analysis, code complexity, patterns)
- Architectural violations with file-specific evidence
- Recommendations and compliance rankings
- Performance and quality metrics

## DPC Integration

Automatically performs Deep Package Comprehension analysis for each package to enhance the comparison with:

- **Dependency Analysis**: Import tracing, external packages, circular dependencies
- **Code Complexity Assessment**: Service organization, pattern recognition
- **Architectural Pattern Recognition**: Compliance with established patterns
- **Performance Optimization Insights**: Caching strategies, resource management
- **Documentation Compliance**: README, API documentation, code comments
- **Mock Strategy Analysis**: Enhanced mock strategy implementation, global vs package-level mocks
- **Test Coverage Assessment**: Functional vs coverage test separation, 100% coverage achievement
- **Anti-Pattern Detection**: VSCode value imports, business logic in extensions, DI container violations
- **Security & Validation**: Input validation, error handling, security patterns
- **Process Management**: Process isolation, cleanup patterns, test environment management

## Generated Files

- `{package-group}-architecture-comparison.md` - Main comparison document
- `{package-group}-violations.md` - Critical violations summary
- `{package-group}-dpc-findings.json` - DPC analysis results (if DPC enabled)

## Implementation

- **Tool Location**: `libs/tools/architecture-comparison-generator/`
- **Entry Point**: `src/cli.ts`
- **Main Class**: `ArchitectureComparisonGenerator`
- **Dependencies**: Node.js built-ins only (fs, path, child_process)

## Architecture Dimensions Analyzed

1. Build Configuration
2. Package.json Structure
3. Dependency Aggregation
4. Complex Orchestration
5. VSCode Import Patterns
6. Testing Configuration
7. Service Architecture
8. Error Handling Strategy
9. Configuration Management
10. Code Organization
11. Performance Patterns
12. Documentation Compliance
13. Mock Strategy Implementation
14. Test Coverage Patterns
15. Anti-Pattern Compliance
16. Security & Validation Patterns
17. Caching & Optimization Strategies
18. PAE Alias Compliance
19. Process Isolation & Cleanup
20. Asset Processing Patterns
