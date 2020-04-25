//! # Barycenter
//!
//! This is a test program written using the O'Reilly "Learning Rust" tutorial by Leo Tindall.
//!

#![feature(test)]

mod values;
use crate::values::get_values;
mod body;
use crate::body::{merge_all_bodies_iter, merge_all_bodies_recursive};

extern crate test;

fn main() {
    let vals = get_values(1000);

    let barycenter = merge_all_bodies_iter(&vals);
    println!("Barycenter iterative: {:?}", barycenter);

    let barycenter = merge_all_bodies_recursive(&vals);
    println!("Barycenter recursive: {:?}", barycenter);
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const SIZE: usize = 1000;

    #[bench]
    fn bench_merge_iterative(b: &mut Bencher) {
        let vals = get_values(SIZE);

        b.iter(|| merge_all_bodies_iter(&vals));
    }

    #[bench]
    fn bench_merge_recursive(b: &mut Bencher) {
        let vals = get_values(SIZE);

        b.iter(|| merge_all_bodies_recursive(&vals));
    }
}