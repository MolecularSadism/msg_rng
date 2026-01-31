//! Basic example demonstrating msg_rng usage with Bevy.
//!
//! This example shows how to use GlobalRng and EntityRng in a simple Bevy app.

use bevy::prelude::*;
use msg_rng::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RngPlugin::seeded(42))
        .add_systems(Startup, (setup, print_seed))
        .add_systems(Update, (use_global_rng, use_entity_rng))
        .run();
}

fn setup(mut commands: Commands, global_rng: Res<GlobalRng>) {
    // Spawn an entity with its own RNG
    commands.spawn(EntityRng::from_global(&global_rng));

    println!("Example started with seed: {}", global_rng.seed());
}

fn print_seed(rng: Res<GlobalRng>) {
    println!("Current seed: {}", rng.seed());
}

fn use_global_rng(mut rng: ResMut<GlobalRng>) {
    // Generate some random values
    let roll: u32 = rng.range(1..=20);
    let chance: f32 = rng.f32();

    if rng.chance(0.1) {
        println!("Global RNG - Roll: {}, Chance: {:.2}", roll, chance);
    }
}

fn use_entity_rng(mut query: Query<&mut EntityRng>) {
    for mut entity_rng in &mut query {
        if entity_rng.chance(0.05) {
            let value: u32 = entity_rng.range(0..100);
            println!("Entity RNG - Generated: {}", value);
        }
    }
}
