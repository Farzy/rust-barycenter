//! # Body
//!
//! Here we define the [`Body`] structure and some functions to calculate weighted averages.
//!
//! # Example
//! ```
//! use barycenter::*
//!
//! let b1 = Body {
//!     x: 1.0,
//!     y: 10.0,
//!     z: 4.0,
//!     mass: 100.0
//! };
//!
//! let b2 = Body {
//!     x: 2.0,
//!     y: 5.0,
//!     z: 2.0,
//!     mass: 300.0
//! };
//!
//! assert_eq!(average(1.0, 2.0), 1.5);
//! ```

extern crate rayon;
extern crate itertools;
use rayon::prelude::*;
use itertools::Itertools;

/// Body structure with 3D coordinates and mass
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub mass: f64,
}

/// Compute the average of two floating numbers
#[allow(dead_code)]
pub fn average(a: f64, b: f64) -> f64 {
    (a + b)/2.0
}

/// Compute the weighted average of two objects with mass
pub fn average_with_mass(a: f64, b: f64, amass: f64, bmass: f64) -> f64 {
    (a * amass + b * bmass) / (amass + bmass)
}

/// Merge two Body and compute their barycenter
pub fn merge_two_bodies(a: Body, b: Body) -> Body {
    Body {
        x: average_with_mass(a.x, b.x, a.mass, b.mass),
        y: average_with_mass(a.y, b.y, a.mass, b.mass),
        z: average_with_mass(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass,
    }
}

/// Merge an array of Body and compute their barycenter
pub fn merge_all_bodies_iter(bodies: &[Body]) -> Body {
    let barycenter = bodies[0];
    bodies.iter().skip(1).fold(barycenter, |sum, body| {
        merge_two_bodies(sum, *body)
    })
}

/// Merge an array of Body and compute their barycenter using recursion and parallelization
pub fn merge_all_bodies_recursive(bodies: &[Body]) -> Body {
    println!("Bodies: {}", bodies.len());

    if bodies.len() == 1 {
        return bodies[0];
    }

    let tuples: Vec<(&Body, &Body)> = bodies.iter().tuples().collect();
    let mut merged_bodies: Vec<_> = tuples.into_par_iter().map(|(a, b)| { merge_two_bodies(*a, *b)}).collect();

    if bodies.len() % 2 != 0 {
        merged_bodies.push(bodies[bodies.len() - 1]);
    }

    merge_all_bodies_recursive(&merged_bodies)
}

// ------------------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use crate::body::*;

    #[test]
    fn test_average() {
        assert_eq!(average(2.0, 4.0), 3.0);
    }

    #[test]
    fn test_weighted_average() {
        assert_eq!(average_with_mass(2.0, 4.0, 1.0, 3.0), 3.5);
    }

    #[test]
    fn test_merge_two_bodies() {
        let b1 = Body {
            x: 1.0,
            y: 10.0,
            z: 4.0,
            mass: 100.0
        };
        let b2 = Body {
            x: 2.0,
            y: 5.0,
            z: 2.0,
            mass: 300.0
        };
        let body_merged = Body {
            x: 1.75,
            y: 6.25,
            z: 2.5,
            mass: 400.0
        };

        let result = merge_two_bodies(b1, b2);
        assert_eq!(result, body_merged);
    }

    #[test]
    fn test_merge_all_bodies_iter() {
        let b1 = Body {
            x: 1.0,
            y: 10.0,
            z: 4.0,
            mass: 100.0
        };
        let b2 = Body {
            x: 2.0,
            y: 5.0,
            z: 2.0,
            mass: 300.0
        };
        let b3 = Body {
            x: 2.0,
            y: 5.0,
            z: 2.0,
            mass: 400.0
        };
        let bodies = vec![b1, b2, b3];
        let body_merged = Body {
            x: 1.875,
            y: 5.625,
            z: 2.25,
            mass: 800.0
        };

        let result = merge_all_bodies_iter(&bodies);
        assert_eq!(result, body_merged);
    }

    #[test]
    fn test_merge_all_bodies_recursive() {
        let b1 = Body {
            x: 1.0,
            y: 10.0,
            z: 4.0,
            mass: 100.0
        };
        let b2 = Body {
            x: 2.0,
            y: 5.0,
            z: 2.0,
            mass: 300.0
        };
        let b3 = Body {
            x: 2.0,
            y: 5.0,
            z: 2.0,
            mass: 400.0
        };
        let bodies = vec![b1, b2, b3];
        let body_merged = Body {
            x: 1.875,
            y: 5.625,
            z: 2.25,
            mass: 800.0
        };

        let result = merge_all_bodies_recursive(&bodies);
        assert_eq!(result, body_merged);
    }
}