# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
