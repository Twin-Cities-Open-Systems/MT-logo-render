# Versioning (Semantic Versioning for HEE Components)

## Overview

MT-logo-render follows [Semantic Versioning 2.0.0](https://semver.org/) as adapted for Human Execution Engine (HEE) platform components. This ensures predictable API evolution while maintaining compatibility within the broader HEE ecosystem.

## Semantic Versioning Format

Versions follow the format: **MAJOR.MINOR.PATCH**

- **MAJOR**: Breaking changes that require HEE platform coordination
- **MINOR**: New features that maintain backward compatibility
- **PATCH**: Bug fixes and improvements that maintain full compatibility

### Pre-release and Build Metadata

- **Pre-releases**: `1.0.0-alpha.1`, `1.0.0-beta.2`, `1.0.0-rc.1`
- **Build metadata**: `1.0.0+20230101120000` (timestamp-based)

## Version Increment Rules

### MAJOR Version (X.0.0)

Increment MAJOR version when:

- **Breaking API changes** that affect HEE workflow integration
- **Fundamental architecture changes** that impact HEE orchestration
- **Removal of deprecated features** used by other HEE components
- **Changes to core HEE execution semantics**

**HEE Platform Coordination Required**: Major version bumps require coordination across all HEE components to ensure platform compatibility.

### MINOR Version (x.Y.0)

Increment MINOR version when:

- **New features** that don't break existing functionality
- **New HEE workflow capabilities** that enhance human execution
- **Additive API changes** (new optional parameters, new endpoints)
- **Performance improvements** that don't change behavior
- **New recipe formats** or rendering capabilities

### PATCH Version (x.y.Z)

Increment PATCH version when:

- **Bug fixes** that maintain full backward compatibility
- **Security fixes** that don't change APIs
- **Documentation improvements**
- **Internal optimizations** that don't affect external behavior
- **Dependency updates** with no functional changes

## HEE Platform Integration

### Platform Compatibility Matrix

| MT-logo-render | HEE Platform | Compatibility |
|----------------|---------------|---------------|
| 1.x.x         | 1.x.x        | ✅ Full compatibility |
| 2.x.x         | 1.x.x        | ⚠️  Requires platform update |
| 1.x.x         | 2.x.x        | ❌ Incompatible |

### Breaking Change Protocol

When a MAJOR version is required:

1. **Announce breaking changes** in advance with migration guide
2. **Coordinate with HEE platform maintainers**
3. **Provide upgrade tooling** for HEE workflow migration
4. **Maintain backward compatibility** in HEE orchestrator if possible

## Release Process

### Release Candidates

- **Alpha releases**: `x.y.z-alpha.N` - Early testing, APIs may change
- **Beta releases**: `x.y.z-beta.N` - Feature complete, API stable
- **Release candidates**: `x.y.z-rc.N` - Final testing before release

### Release Checklist

**Pre-release:**
- [ ] All tests pass (unit, integration, HEE workflow tests)
- [ ] Security audit completed
- [ ] Performance benchmarks meet requirements
- [ ] Documentation updated
- [ ] HEE integration tested
- [ ] Changelog finalized

**Release:**
- [ ] Git tag created: `git tag -a v1.2.3 -m "Release v1.2.3"`
- [ ] GitHub release created with changelog
- [ ] HEE platform notified of new version
- [ ] Documentation published

## Changelog Format

Changelogs follow [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format:

```markdown
## [1.2.3] - 2024-01-15

### Added
- New HEE workflow integration feature
- Support for custom recipe schemas

### Changed
- Improved error messages for HEE orchestration

### Fixed
- Memory leak in PNG generation
- Race condition in cache operations

### Security
- Updated dependencies to address CVE-2024-XXXX
```

## Deprecation Policy

### Feature Deprecation

1. **Mark as deprecated** in MINOR release with warnings
2. **Remove in next MAJOR release** (minimum 6 months deprecation period)
3. **Document migration path** in deprecation warnings
4. **HEE platform coordination** for workflow-impacting deprecations

### API Deprecation Headers

```rust
#[deprecated(since = "1.2.0", note = "Use new_recipe_api instead")]
pub fn old_recipe_function() {
    // Implementation with deprecation warning
}
```

## Compatibility Guarantees

### Backward Compatibility

- **PATCH versions**: 100% backward compatible
- **MINOR versions**: Backward compatible for documented APIs
- **MAJOR versions**: May break compatibility (coordinated with HEE platform)

### Forward Compatibility

- **HEE orchestrator** designed to handle component version differences
- **Graceful degradation** for newer component features
- **Version negotiation** during HEE workflow initialization

## Development Versioning

### Development Builds

During development between releases:

- **Dirty builds**: `1.2.3.dev0+g1234567` (uncommitted changes)
- **Development releases**: `1.2.3.dev1` (development snapshots)

### Branch Versioning

- **main branch**: Always points to latest stable release
- **feature branches**: Version based on target release
- **hotfix branches**: Patch version increments

## Version File Management

### Cargo.toml Version

```toml
[package]
name = "mt-logo-render"
version = "1.2.3"
```

### Git Tags

- **Release tags**: `v1.2.3`
- **Annotated tags**: Include release notes
- **Signed tags**: GPG signed for security

### Release Artifacts

- **Binary releases**: `mt-logo-render-v1.2.3-x86_64-linux.tar.gz`
- **Source archives**: `mt-logo-render-1.2.3.tar.gz`
- **Checksums**: SHA256 hashes for all artifacts

## HEE Ecosystem Coordination

### Platform Version Alignment

HEE components should align major versions when possible:

```
HEE Platform: v2.0.0
├── MT-logo-render: v2.1.0
├── tick-task: v2.0.3
├── market-thesis: v2.0.1
└── FIN-tools: v2.0.2
```

### Cross-Component Dependencies

- **Version ranges** in HEE orchestrator: `"mt-logo-render": "^2.0.0"`
- **Minimum version requirements** documented in HEE platform spec
- **Compatibility matrices** maintained for complex interactions

## Migration Guides

### Major Version Migrations

For each MAJOR version, provide:

1. **Migration guide** with step-by-step instructions
2. **Breaking changes list** with rationale
3. **HEE workflow migration examples**
4. **Rollback procedures** if needed

### Example Migration

```markdown
## Migrating from v1.x to v2.0

### Breaking Changes
- Recipe API now requires explicit schema validation
- Cache format changed for better HEE orchestration

### Migration Steps
1. Update recipes to include schema declarations
2. Clear cache directory (automatic migration available)
3. Update HEE orchestrator configuration
```

## Security Updates

### Security Patch Releases

- **PATCH releases** for security fixes
- **Out-of-band releases** for critical vulnerabilities
- **HEE platform alerts** for security-impacting issues

### Vulnerability Disclosure

Follow responsible disclosure:
1. **Private coordination** with HEE platform maintainers
2. **Fix development** in security branch
3. **Coordinated release** across all affected components
4. **Public disclosure** after fixes deployed

## Long-term Support

### LTS Versions

- **LTS releases**: Selected MINOR versions with extended support
- **Security patches** for LTS versions (minimum 2 years)
- **HEE platform LTS alignment** for ecosystem stability

### End of Life

- **Deprecation notice** 6 months before EOL
- **Migration support** provided during transition
- **Archive availability** for historical reference

---

*This versioning scheme ensures MT-logo-render evolves predictably while maintaining HEE platform compatibility and human-centric workflow stability.*
