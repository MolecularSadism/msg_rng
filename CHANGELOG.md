# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-01-31

### Changed
- **BREAKING**: Updated to Bevy 0.18.0
- Updated dependency: `bevy = "0.18"`
- Improved documentation with proper backtick formatting for type names
- Changed cast operations to use `u64::from()` for better type safety
- Migrated CI to reusable workflow template

### Added
- Comprehensive integration tests covering all public API functionality
- Tests for plugin initialization, system integration, and entity RNG behavior
- Tests for RNG forking, stream separation, and state persistence
- Basic example demonstrating `GlobalRng` and `EntityRng` usage
- Bevy version compatibility table in README
- CI, License, Bevy version, and Rust edition badges to README

### Fixed
- Fixed clippy pedantic warnings regarding documentation formatting
- Fixed clippy lossless cast warnings
- Updated CI configuration for bevy_lint compatibility with Bevy 0.18

## [0.2.0] - 2026-01-20

### Changed
- **BREAKING**: Updated to Bevy 0.17.0
- Updated dependency: `bevy = "0.17"`

## [0.1.0] - 2026-01-15

### Added

- `RngPlugin` for easy Bevy integration with seeded or random initialization
- `GlobalRng` resource for centralized game-wide randomness
- `EntityRng` component for per-entity deterministic RNG
- Seed retrieval via `seed()` method, even when using random initialization
- RNG forking for isolated child RNGs
- Stream separation with `fork_stream()` for named RNG streams
- Convenience methods: `range()`, `f32()`, `f64()`, `bool()`, `chance()`, `choose()`, `shuffle()`
- `RngFork` trait for unified forking interface
- Full documentation with examples
