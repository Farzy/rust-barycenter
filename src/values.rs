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

/// Return a list of `size` random [`Body`]
///
/// The Random generator always uses the same seed in order to make testing possible.
///
/// # Examples
/// ```
/// use body::Body
///
/// let values = get_values(100);
/// ```
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

#[cfg(test)]
mod tests {
    use super::get_values;

    #[test]
    fn test_pseudo_random_values() {
        let values = get_values(10);

        assert_eq!(values.len(), 10);
        assert_eq!(values[0].x, 119.74106605089537);
        assert_eq!(values[0].y, -983.1379236571887);
        assert_eq!(values[0].z, 77.87151041898233);
        assert_eq!(values[0].mass, 875.9898531258021);
    }
}