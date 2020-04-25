//! # Body generator
//!
//! The video tutorial does not provide a link to the `values.rs` file that is supposed to
//! contain an array of celestial bodies.
//!
//! So I wrote a Body array generator using the random generator seeded from a fixed value so
//! that values are always the same, whatever the running platform is.

extern crate rand;
extern crate rand_chacha;

use rand::{Rng, SeedableRng};
use crate::body::Body;

/// Return a list of `size` random [body::Body]
///
/// The Random generator always uses the same seed in order to make testing possible.
pub fn get_values(size: usize) -> Vec<Body> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let mut bodies: Vec<Body> = Vec::with_capacity(size);

    for _ in 1..=size {
        bodies.push(Body {
            x: rng.gen_range(-1000.0, 1000.0),
            y: rng.gen_range(-1000.0, 1000.0),
            z: rng.gen_range(-1000.0, 1000.0),
            mass: rng.gen_range(100.0, 1000.0)
        })
    }
    // bodies.truncate(0);
    // bodies.push(Body {
    //     x: 1.0,
    //     y: 2.0,
    //     z: 3.0,
    //     mass: 100.0
    // });
    // bodies.push(Body {
    //     x: 2.0,
    //     y: 2.0,
    //     z: -1.0,
    //     mass: 50.0
    // });
    bodies
}
