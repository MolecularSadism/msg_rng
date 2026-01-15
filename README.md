# msg_rng

Centralized seeded RNG functionality for Bevy games.

## Features

- **Seeded RNG**: Set a seed for reproducible gameplay/testing
- **Global resource**: Single source of randomness for game systems
- **Per-entity RNG**: Isolated RNG per entity for deterministic AI/behavior
- **Forking**: Create isolated child RNGs that don't affect parent state
- **Stream separation**: Named streams for different game systems
- **Seed retrieval**: Always access the current seed, even when randomly generated

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
msg_rng = { git = "https://github.com/MolecularSadism/msg_rng", tag = "v0.1.0" }
```

## Quick Start

```rust
use bevy::prelude::*;
use msg_rng::prelude::*;

fn main() {
    App::new()
        .add_plugins(RngPlugin::seeded(12345))
        .add_systems(Update, my_system)
        .run();
}

fn my_system(mut rng: ResMut<GlobalRng>) {
    let roll: u32 = rng.range(1..=20);
    let chance: f32 = rng.f32();
    let coin_flip: bool = rng.bool();
}
```

## Retrieving the Seed

The seed is always retrievable via `rng.seed()`, even when using `RngPlugin::random()`:

```rust
fn show_seed(rng: Res<GlobalRng>) {
    println!("World seed: {}", rng.seed());
}
```

## Per-Entity RNG

For deterministic per-entity behavior:

```rust
fn spawn_enemy(mut commands: Commands, global_rng: Res<GlobalRng>) {
    commands.spawn(EntityRng::from_global(&global_rng));
}

fn enemy_ai(mut query: Query<&mut EntityRng>) {
    for mut rng in &mut query {
        if rng.chance(0.1) {
            // 10% chance to do something
        }
    }
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
