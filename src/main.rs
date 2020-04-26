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

    const SIZE_SMALL: usize = 1_000;
    const SIZE_LARGE: usize = 1_000_000;

    #[bench]
    fn bench_merge_iterative_small(b: &mut Bencher) {
        let vals = get_values(SIZE_SMALL);

        b.iter(|| merge_all_bodies_iter(&vals));
    }

    #[bench]
    fn bench_merge_recursive_small(b: &mut Bencher) {
        let vals = get_values(SIZE_SMALL);

        b.iter(|| merge_all_bodies_recursive(&vals));
    }

    #[bench]
    fn bench_merge_iterative_large(b: &mut Bencher) {
        let vals = get_values(SIZE_LARGE);

        b.iter(|| merge_all_bodies_iter(&vals));
    }

    #[bench]
    fn bench_merge_recursive_large(b: &mut Bencher) {
        let vals = get_values(SIZE_LARGE);

        b.iter(|| merge_all_bodies_recursive(&vals));
    }
}