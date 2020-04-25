//! # Body
//!
//! Here we define the body structure and some functions to calculae averages.

extern crate rayon;
extern crate itertools;
use rayon::prelude::*;
use itertools::Itertools;

/// Body structure with 3D coordinates and mass
#[derive(Debug, Copy, Clone)]
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