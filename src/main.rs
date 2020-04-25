//! # Barycenter
//!
//! This is a test program written using the O'Reilly "Learning Rust" tutorial by Leo Tindall.
//!


mod values;
use crate::values::get_values;
mod body;
use crate::body::{merge_all_bodies_iter, merge_all_bodies_recursive};

fn main() {
    let vals = get_values(1000);

    let barycenter = merge_all_bodies_iter(&vals);
    println!("Barycenter iterative: {:?}", barycenter);

    let barycenter = merge_all_bodies_recursive(&vals);
    println!("Barycenter recursive: {:?}", barycenter);
}
