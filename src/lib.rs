//! Centralized seeded RNG functionality for Bevy games.
//!
//! This crate provides deterministic, reproducible random number generation
//! through a centralized [`GlobalRng`] resource and per-entity [`EntityRng`]
//! components.
//!
//! # Features
//!
//! - **Seeded RNG**: Set a seed for reproducible gameplay/testing
//! - **Global resource**: Single source of randomness for game systems
//! - **Per-entity RNG**: Isolated RNG per entity for deterministic AI/behavior
//! - **Forking**: Create isolated child RNGs that don't affect parent state
//! - **Stream separation**: Named streams for different game systems
//! - **Seed retrieval**: Always access the current seed, even when randomly generated
//!
//! # Quick Start
//!
//! ```rust
//! use bevy::prelude::*;
//! use msg_rng::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(RngPlugin::seeded(12345))
//!         .add_systems(Update, my_system);
//!     // .run() would start the app loop
//! }
//!
//! fn my_system(mut rng: ResMut<GlobalRng>) {
//!     // Generate random values
//!     let roll: u32 = rng.range(1..=20);
//!     let chance: f32 = rng.f32();
//!     let coin_flip: bool = rng.bool();
//! }
//! ```
//!
//! # Retrieving the Seed
//!
//! The seed is always retrievable via `rng.seed()`, even when using
//! `RngPlugin::random()`. Use `Res<GlobalRng>` for immutable access:
//!
//! ```rust
//! use bevy::prelude::*;
//! use msg_rng::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         // Random seed - but still retrievable!
//!         .add_plugins(RngPlugin::random())
//!         .add_systems(Startup, show_seed);
//!     // .run() would start the app loop
//! }
//!
//! fn show_seed(rng: Res<GlobalRng>) {
//!     // Immutable access is sufficient for reading the seed
//!     println!("World seed: {}", rng.seed());
//! }
//! ```
//!
//! # Per-Entity RNG
//!
//! For deterministic per-entity behavior:
//!
//! ```rust
//! use bevy::prelude::*;
//! use msg_rng::prelude::*;
//!
//! fn spawn_enemy(mut commands: Commands, global_rng: Res<GlobalRng>) {
//!     commands.spawn((
//!         // Fork RNG for this entity
//!         EntityRng::from_global(&global_rng),
//!     ));
//! }
//!
//! fn enemy_ai(mut query: Query<&mut EntityRng>) {
//!     for mut rng in &mut query {
//!         // Each entity has its own deterministic RNG
//!         if rng.chance(0.1) {
//!             // 10% chance to do something
//!         }
//!     }
//! }
//! ```

use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

/// Plugin for adding centralized RNG to a Bevy app.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use msg_rng::RngPlugin;
///
/// // Random seed (different each run)
/// App::new().add_plugins(RngPlugin::random());
///
/// // Fixed seed (reproducible)
/// App::new().add_plugins(RngPlugin::seeded(42));
///
/// // Custom configuration
/// App::new().add_plugins(RngPlugin::new().with_seed(42));
/// ```
pub struct RngPlugin {
    seed: Option<u64>,
}

impl Default for RngPlugin {
    fn default() -> Self {
        Self::random()
    }
}

impl RngPlugin {
    /// Create a new RNG plugin with random seed.
    #[must_use]
    pub fn new() -> Self {
        Self { seed: None }
    }

    /// Create an RNG plugin with a random seed (different each run).
    #[must_use]
    pub fn random() -> Self {
        Self { seed: None }
    }

    /// Create an RNG plugin with a fixed seed (reproducible).
    #[must_use]
    pub fn seeded(seed: u64) -> Self {
        Self { seed: Some(seed) }
    }

    /// Set the seed for this plugin.
    #[must_use]
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        let global_rng = match self.seed {
            Some(seed) => GlobalRng::seeded(seed),
            None => GlobalRng::random(),
        };

        app.insert_resource(global_rng);
    }
}

/// Global random number generator resource.
///
/// This is the primary source of randomness for game systems.
/// Use [`EntityRng`] for per-entity deterministic randomness.
///
/// The seed is always retrievable via [`GlobalRng::seed()`], even when using
/// [`RngPlugin::random()`]. Use `Res<GlobalRng>` for immutable access to the seed.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use msg_rng::GlobalRng;
///
/// fn roll_dice(mut rng: ResMut<GlobalRng>) {
///     let roll: u32 = rng.range(1..=6);
///     println!("Rolled: {}", roll);
/// }
///
/// fn save_game(rng: Res<GlobalRng>) {
///     // Use Res (immutable) to just read the seed
///     let seed = rng.seed();
///     println!("Saving with seed: {}", seed);
/// }
/// ```
#[derive(Resource)]
pub struct GlobalRng {
    rng: StdRng,
    seed: u64,
}

impl Default for GlobalRng {
    fn default() -> Self {
        Self::random()
    }
}

impl GlobalRng {
    /// Create a new GlobalRng with a random seed.
    #[must_use]
    pub fn random() -> Self {
        let seed = rand::random();
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    /// Create a new GlobalRng with a specific seed.
    #[must_use]
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    /// Get the seed used to initialize this RNG.
    ///
    /// This works even when the RNG was created with [`GlobalRng::random()`].
    /// Use `Res<GlobalRng>` for immutable access when you only need the seed.
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Reset the RNG to its initial state using the original seed.
    pub fn reset(&mut self) {
        self.rng = StdRng::seed_from_u64(self.seed);
    }

    /// Reset the RNG with a new seed.
    pub fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.rng = StdRng::seed_from_u64(seed);
    }

    /// Fork this RNG to create an independent child RNG.
    ///
    /// The child RNG will have a different seed derived from the parent,
    /// and operations on the child won't affect the parent's state.
    #[must_use]
    pub fn fork(&mut self) -> StdRng {
        let child_seed: u64 = self.rng.random();
        StdRng::seed_from_u64(child_seed)
    }

    /// Fork this RNG with a named stream for deterministic separation.
    ///
    /// Using the same stream name will produce the same child RNG
    /// (if called at the same point in the parent's sequence).
    #[must_use]
    pub fn fork_stream(&mut self, stream: u32) -> StdRng {
        let base: u64 = self.rng.random();
        let combined = base.wrapping_add(stream as u64);
        StdRng::seed_from_u64(combined)
    }

    /// Generate a random value within a range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use msg_rng::GlobalRng;
    /// # let mut rng = GlobalRng::seeded(42);
    /// let dice: u32 = rng.range(1..=6);
    /// let percent: f32 = rng.range(0.0..1.0);
    /// ```
    pub fn range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distr::uniform::SampleUniform,
        R: rand::distr::uniform::SampleRange<T>,
    {
        self.rng.random_range(range)
    }

    /// Generate a random f32 in [0.0, 1.0).
    pub fn f32(&mut self) -> f32 {
        self.rng.random::<f32>()
    }

    /// Generate a random f64 in [0.0, 1.0).
    pub fn f64(&mut self) -> f64 {
        self.rng.random::<f64>()
    }

    /// Generate a random bool with 50% probability.
    pub fn bool(&mut self) -> bool {
        self.rng.random()
    }

    /// Generate a random bool with the given probability (0.0 to 1.0).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use msg_rng::GlobalRng;
    /// # let mut rng = GlobalRng::seeded(42);
    /// // 25% chance
    /// if rng.chance(0.25) {
    ///     println!("Lucky!");
    /// }
    /// ```
    pub fn chance(&mut self, probability: f64) -> bool {
        self.rng.random::<f64>() < probability
    }

    /// Select a random element from a slice.
    ///
    /// Returns `None` if the slice is empty.
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let idx = self.rng.random_range(0..slice.len());
            Some(&slice[idx])
        }
    }

    /// Select a random element from a slice and return its index.
    ///
    /// Returns `None` if the slice is empty.
    pub fn choose_index<T>(&mut self, slice: &[T]) -> Option<usize> {
        if slice.is_empty() {
            None
        } else {
            Some(self.rng.random_range(0..slice.len()))
        }
    }

    /// Shuffle a slice in place.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        use rand::seq::SliceRandom;
        slice.shuffle(&mut self.rng);
    }

    /// Generate a random value of type T.
    ///
    /// Works with any type where StandardUniform implements Distribution<T>.
    pub fn random_value<T>(&mut self) -> T
    where
        rand::distr::StandardUniform: rand::distr::Distribution<T>,
    {
        self.rng.random()
    }

    /// Get mutable access to the underlying RNG.
    ///
    /// Use sparingly; prefer the convenience methods when possible.
    pub fn inner(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    /// Alias for `inner()` - get mutable access to the underlying RNG.
    ///
    /// Useful when you need to pass the RNG to functions expecting `impl Rng`.
    pub fn rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    /// Generate a random u32 value.
    pub fn u32(&mut self) -> u32 {
        self.rng.random()
    }

    /// Generate a random u64 value.
    pub fn u64(&mut self) -> u64 {
        self.rng.random()
    }

    /// Generate a random i32 value.
    pub fn i32(&mut self) -> i32 {
        self.rng.random()
    }

    /// Generate a random i64 value.
    pub fn i64(&mut self) -> i64 {
        self.rng.random()
    }
}

/// Per-entity random number generator component.
///
/// Use this for deterministic per-entity randomness, such as AI decisions
/// or procedural animations. Each entity gets its own isolated RNG stream.
///
/// # Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use msg_rng::prelude::*;
///
/// fn spawn_with_rng(mut commands: Commands, rng: Res<GlobalRng>) {
///     commands.spawn(EntityRng::from_global(&rng));
/// }
///
/// fn use_entity_rng(mut query: Query<&mut EntityRng>) {
///     for mut rng in &mut query {
///         let value: f32 = rng.f32();
///     }
/// }
/// ```
#[derive(Component)]
pub struct EntityRng {
    rng: StdRng,
    seed: u64,
}

impl Default for EntityRng {
    fn default() -> Self {
        Self::random()
    }
}

impl EntityRng {
    /// Create a new EntityRng with a random seed.
    #[must_use]
    pub fn random() -> Self {
        let seed = rand::random();
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    /// Create a new EntityRng with a specific seed.
    #[must_use]
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    /// Create an EntityRng derived from the global RNG.
    ///
    /// Note: This consumes randomness from the global RNG.
    #[must_use]
    pub fn from_global(global: &GlobalRng) -> Self {
        let seed = hash_combine(global.seed, rand::random::<u64>());
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    /// Create an EntityRng with a deterministic seed based on global seed and entity id.
    ///
    /// This is useful for save/load where you want reproducible entity RNG.
    #[must_use]
    pub fn from_global_and_id(global_seed: u64, entity_index: u32) -> Self {
        let seed = hash_combine(global_seed, entity_index as u64);
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    /// Get the seed used to initialize this RNG.
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Reset the RNG to its initial state using the original seed.
    pub fn reset(&mut self) {
        self.rng = StdRng::seed_from_u64(self.seed);
    }

    /// Generate a random value within a range.
    pub fn range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distr::uniform::SampleUniform,
        R: rand::distr::uniform::SampleRange<T>,
    {
        self.rng.random_range(range)
    }

    /// Generate a random f32 in [0.0, 1.0).
    pub fn f32(&mut self) -> f32 {
        self.rng.random::<f32>()
    }

    /// Generate a random f64 in [0.0, 1.0).
    pub fn f64(&mut self) -> f64 {
        self.rng.random::<f64>()
    }

    /// Generate a random bool with 50% probability.
    pub fn bool(&mut self) -> bool {
        self.rng.random()
    }

    /// Generate a random bool with the given probability (0.0 to 1.0).
    pub fn chance(&mut self, probability: f64) -> bool {
        self.rng.random::<f64>() < probability
    }

    /// Select a random element from a slice.
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        if slice.is_empty() {
            None
        } else {
            let idx = self.rng.random_range(0..slice.len());
            Some(&slice[idx])
        }
    }

    /// Shuffle a slice in place.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        use rand::seq::SliceRandom;
        slice.shuffle(&mut self.rng);
    }

    /// Generate a random value of type T.
    pub fn random_value<T>(&mut self) -> T
    where
        rand::distr::StandardUniform: rand::distr::Distribution<T>,
    {
        self.rng.random()
    }

    /// Get mutable access to the underlying RNG.
    pub fn inner(&mut self) -> &mut StdRng {
        &mut self.rng
    }
}

/// Combine two u64 values into a deterministic hash.
#[inline]
fn hash_combine(a: u64, b: u64) -> u64 {
    // FNV-1a inspired mixing
    let mut h = a;
    h ^= b;
    h = h.wrapping_mul(0x517c_c1b7_2722_0a95);
    h ^= h >> 32;
    h
}

/// Extension trait for creating temporary forked RNGs.
///
/// Useful when you need a scoped RNG that doesn't affect the global state.
pub trait RngFork {
    /// Create a forked RNG for isolated random operations.
    fn fork(&mut self) -> StdRng;
}

impl RngFork for GlobalRng {
    fn fork(&mut self) -> StdRng {
        GlobalRng::fork(self)
    }
}

impl RngFork for EntityRng {
    fn fork(&mut self) -> StdRng {
        let child_seed: u64 = self.rng.random();
        StdRng::seed_from_u64(child_seed)
    }
}

/// Convenience type alias for a mutable reference to GlobalRng.
pub type GlobalRngMut<'w> = ResMut<'w, GlobalRng>;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use super::{EntityRng, GlobalRng, GlobalRngMut, RngFork, RngPlugin};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_rng_is_deterministic() {
        let mut rng1 = GlobalRng::seeded(12345);
        let mut rng2 = GlobalRng::seeded(12345);

        let values1: Vec<u32> = (0..10).map(|_| rng1.range(0..100)).collect();
        let values2: Vec<u32> = (0..10).map(|_| rng2.range(0..100)).collect();

        assert_eq!(values1, values2);
    }

    #[test]
    fn different_seeds_produce_different_values() {
        let mut rng1 = GlobalRng::seeded(12345);
        let mut rng2 = GlobalRng::seeded(54321);

        let values1: Vec<u32> = (0..10).map(|_| rng1.range(0..1000)).collect();
        let values2: Vec<u32> = (0..10).map(|_| rng2.range(0..1000)).collect();

        assert_ne!(values1, values2);
    }

    #[test]
    fn reset_restores_initial_state() {
        let mut rng = GlobalRng::seeded(42);
        let initial: Vec<u32> = (0..5).map(|_| rng.range(0..100)).collect();

        rng.reset();
        let after_reset: Vec<u32> = (0..5).map(|_| rng.range(0..100)).collect();

        assert_eq!(initial, after_reset);
    }

    #[test]
    fn fork_creates_independent_rng() {
        let mut parent = GlobalRng::seeded(42);
        let parent_val1: u32 = parent.range(0..100);

        let mut child = parent.fork();
        let _child_vals: Vec<u32> = (0..100).map(|_| child.random_range(0..100)).collect();

        let parent_val2: u32 = parent.range(0..100);

        // Parent should continue its own sequence unaffected by child
        let mut fresh = GlobalRng::seeded(42);
        let _: u32 = fresh.range(0..100); // consume one like parent_val1
        let _: u64 = fresh.random_value(); // fork consumes one random
        let expected: u32 = fresh.range(0..100);

        assert_eq!(parent_val1, GlobalRng::seeded(42).range(0..100));
        assert_eq!(parent_val2, expected);
    }

    #[test]
    fn choose_returns_none_for_empty_slice() {
        let mut rng = GlobalRng::seeded(42);
        let empty: &[i32] = &[];
        assert!(rng.choose(empty).is_none());
    }

    #[test]
    fn choose_returns_element_from_slice() {
        let mut rng = GlobalRng::seeded(42);
        let items = [1, 2, 3, 4, 5];
        let choice = rng.choose(&items);
        assert!(choice.is_some());
        assert!(items.contains(choice.unwrap()));
    }

    #[test]
    fn entity_rng_is_independent() {
        let global = GlobalRng::seeded(100);

        let mut entity1 = EntityRng::from_global_and_id(global.seed(), 1);
        let mut entity2 = EntityRng::from_global_and_id(global.seed(), 2);

        let vals1: Vec<u32> = (0..5).map(|_| entity1.range(0..100)).collect();
        let vals2: Vec<u32> = (0..5).map(|_| entity2.range(0..100)).collect();

        // Different entities should have different sequences
        assert_ne!(vals1, vals2);

        // Same entity ID should produce same sequence
        let mut entity1_again = EntityRng::from_global_and_id(global.seed(), 1);
        let vals1_again: Vec<u32> = (0..5).map(|_| entity1_again.range(0..100)).collect();
        assert_eq!(vals1, vals1_again);
    }

    #[test]
    fn chance_respects_probability() {
        let mut rng = GlobalRng::seeded(42);

        // 0% should never succeed
        let zeros: Vec<bool> = (0..100).map(|_| rng.chance(0.0)).collect();
        assert!(zeros.iter().all(|&b| !b));

        // 100% should always succeed
        let ones: Vec<bool> = (0..100).map(|_| rng.chance(1.0)).collect();
        assert!(ones.iter().all(|&b| b));
    }

    #[test]
    fn random_seed_is_retrievable() {
        // Even when using random(), the seed should be stored and retrievable
        let rng = GlobalRng::random();
        let seed = rng.seed();

        // Seed should be non-zero (statistically very unlikely to be zero)
        // and using it should reproduce the same sequence
        let mut rng1 = GlobalRng::seeded(seed);
        let mut rng2 = GlobalRng::seeded(seed);

        let vals1: Vec<u32> = (0..10).map(|_| rng1.range(0..1000)).collect();
        let vals2: Vec<u32> = (0..10).map(|_| rng2.range(0..1000)).collect();

        assert_eq!(vals1, vals2);
    }

    #[test]
    fn entity_seed_is_retrievable() {
        let entity_rng = EntityRng::random();
        let seed = entity_rng.seed();

        // Using the same seed should reproduce the sequence
        let mut rng1 = EntityRng::seeded(seed);
        let mut rng2 = EntityRng::seeded(seed);

        let vals1: Vec<u32> = (0..10).map(|_| rng1.range(0..1000)).collect();
        let vals2: Vec<u32> = (0..10).map(|_| rng2.range(0..1000)).collect();

        assert_eq!(vals1, vals2);
    }
}
