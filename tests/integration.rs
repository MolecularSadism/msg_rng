//! Integration tests for `msg_rng` with Bevy 0.18

// Bevy system parameters are passed by value through dependency injection.
// The pedantic lint `needless_pass_by_value` is a false positive for Bevy systems.
#![allow(clippy::needless_pass_by_value)]

use bevy::prelude::*;
use msg_rng::prelude::*;
use rand::Rng;

// System functions for integration tests
fn test_rng_range_system(mut rng: ResMut<GlobalRng>) {
    let value: u32 = rng.range(0..100);
    assert!(value < 100);
}

fn test_rng_immutable_access_system(rng: Res<GlobalRng>) {
    assert_eq!(rng.seed(), 99999);
}

fn spawn_entity_rng_system(mut commands: Commands, global_rng: Res<GlobalRng>) {
    commands.spawn(EntityRng::from_global(&global_rng));
}

fn query_entity_rng_system(mut query: Query<&mut EntityRng>) {
    for mut rng in &mut query {
        let value: u32 = rng.range(0..100);
        assert!(value < 100);
    }
}

fn spawn_multiple_entities_system(mut commands: Commands, global_rng: Res<GlobalRng>) {
    for i in 0..5 {
        commands.spawn(EntityRng::from_global_and_id(global_rng.seed(), i));
    }
}

fn check_independent_seeds_system(query: Query<&EntityRng>) {
    let seeds: Vec<u64> = query.iter().map(EntityRng::seed).collect();

    for i in 0..seeds.len() {
        for j in (i + 1)..seeds.len() {
            assert_ne!(seeds[i], seeds[j]);
        }
    }
}

fn consume_random_value_system(mut rng: ResMut<GlobalRng>) {
    let _: u32 = rng.range(0..100);
}

fn test_forking_system(mut rng: ResMut<GlobalRng>) {
    let parent_val1: u32 = rng.range(0..1000);

    let mut child = rng.fork();
    let _child_vals: Vec<u32> = (0..100).map(|_| child.random_range(0..1000)).collect();

    let parent_val2: u32 = rng.range(0..1000);

    assert_ne!(parent_val1, parent_val2);
}

fn test_stream_forking_system(mut rng: ResMut<GlobalRng>) {
    let mut child1 = rng.fork_stream(1);
    rng.reset();
    let mut child2 = rng.fork_stream(1);

    let val1: u32 = child1.random_range(0..1000);
    let val2: u32 = child2.random_range(0..1000);

    assert_eq!(val1, val2);
}

fn test_all_rng_methods_system(mut rng: ResMut<GlobalRng>) {
    let _range: u32 = rng.range(0..100);
    let _f32_val = rng.f32();
    let _f64_val = rng.f64();
    let _bool_val = rng.bool();
    let _chance = rng.chance(0.5);

    let slice = [1, 2, 3, 4, 5];
    let _choice = rng.choose(&slice);
    let _idx = rng.choose_index(&slice);

    let mut to_shuffle = vec![1, 2, 3, 4, 5];
    rng.shuffle(&mut to_shuffle);

    let _u32_val = rng.u32();
    let _u64_val = rng.u64();
    let _i32_val = rng.i32();
    let _i64_val = rng.i64();

    let _inner = rng.inner();
    let _rng_ref = rng.rng();
}

fn test_all_entity_rng_methods_system(mut query: Query<&mut EntityRng>) {
    for mut rng in &mut query {
        let _range: u32 = rng.range(0..100);
        let _f32_val = rng.f32();
        let _f64_val = rng.f64();
        let _bool_val = rng.bool();
        let _chance = rng.chance(0.5);

        let slice = [1, 2, 3, 4, 5];
        let _choice = rng.choose(&slice);

        let mut to_shuffle = vec![1, 2, 3, 4, 5];
        rng.shuffle(&mut to_shuffle);

        let _inner = rng.inner();

        let seed = rng.seed();
        rng.reset();
        assert_eq!(rng.seed(), seed);
    }
}

fn test_reseed_system(mut rng: ResMut<GlobalRng>) {
    assert_eq!(rng.seed(), 800);
    rng.reseed(900);
    assert_eq!(rng.seed(), 900);
}

fn test_type_alias_system(mut rng: GlobalRngMut) {
    let _val: u32 = rng.range(0..100);
    assert_eq!(rng.seed(), 999);
}

// Tests
#[test]
fn plugin_initializes_with_seeded_rng() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(42));

    let rng = app.world().resource::<GlobalRng>();
    assert_eq!(rng.seed(), 42);
}

#[test]
fn plugin_initializes_with_random_rng() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::random());

    let rng = app.world().resource::<GlobalRng>();
    assert_ne!(rng.seed(), 0);
}

#[test]
fn plugin_initializes_with_default() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::default());

    let rng = app.world().resource::<GlobalRng>();
    assert_ne!(rng.seed(), 0);
}

#[test]
fn global_rng_accessible_in_system() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(12345));

    app.add_systems(Update, test_rng_range_system);
    app.update();
}

#[test]
fn global_rng_immutable_access() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(99999));

    app.add_systems(Update, test_rng_immutable_access_system);
    app.update();
}

#[test]
fn entity_rng_component_works() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(100));

    app.add_systems(Startup, spawn_entity_rng_system);
    app.add_systems(Update, query_entity_rng_system);
    app.update();
    app.update();
}

#[test]
fn multiple_entities_have_independent_rngs() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(200));

    app.add_systems(Startup, spawn_multiple_entities_system);
    app.add_systems(Update, check_independent_seeds_system);
    app.update();
    app.update();
}

#[test]
fn rng_state_persists_across_updates() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(300));

    app.add_systems(Update, consume_random_value_system);
    app.update();
    let first_value = app.world_mut().resource_mut::<GlobalRng>().range(0..100);

    app.update();
    let second_value = app.world_mut().resource_mut::<GlobalRng>().range(0..100);

    assert_ne!(first_value, second_value);
}

#[test]
fn forking_creates_independent_sequences() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(400));

    app.add_systems(Update, test_forking_system);
    app.update();
}

#[test]
fn stream_forking_is_deterministic() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(500));

    app.add_systems(Update, test_stream_forking_system);
    app.update();
}

#[test]
fn all_rng_methods_work() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(600));

    app.add_systems(Update, test_all_rng_methods_system);
    app.update();
}

#[test]
fn entity_rng_all_methods_work() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(700));

    app.add_systems(Startup, spawn_entity_rng_system);
    app.add_systems(Update, test_all_entity_rng_methods_system);
    app.update();
    app.update();
}

#[test]
fn reseed_changes_seed() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(800));

    app.add_systems(Update, test_reseed_system);
    app.update();
}

#[test]
fn plugin_builder_pattern_works() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::new().with_seed(1234));

    let rng = app.world().resource::<GlobalRng>();
    assert_eq!(rng.seed(), 1234);
}

#[test]
fn global_rng_mut_type_alias_works() {
    let mut app = App::new();
    app.add_plugins(RngPlugin::seeded(999));

    app.add_systems(Update, test_type_alias_system);
    app.update();
}
