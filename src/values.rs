extern crate rand;
extern crate rand_chacha;

use rand::{Rng, SeedableRng};
use crate::body::Body;

pub fn get_values() -> Vec<Body> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let mut bodies: Vec<Body> = Vec::new();

    for _ in 1..=100 {
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
