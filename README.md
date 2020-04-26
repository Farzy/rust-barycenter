# Rust Barycenter

## Synopsis

Test program for the "**Learning Rust**" tutorial by Leo Tindall
on [O'Reilly Learning](https://learning.oreilly.com).

I use this project to learn various Rust aspects:
* Modules
* Struct
* [Random](https://rust-random.github.io/rand/rand/index.html)
* [Vec allocation](https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation)
* [Rustdoc](https://doc.rust-lang.org/rustdoc/index.html)

## Building

Because of the experimental [benchmark code](https://doc.rust-lang.org/1.7.0/book/benchmark-tests.html)
you must use the *nightly* version of the `Cargo`toolchain.

```bash
cargo +nightly build
```

## Benchmarking

In order to benchmark the two merging functions run:

```bash
cargo +nightly bench
```

In my tests, the optimized **parallel recursive** is actually 3 to 30 times slower
than the simple **iterative** version:

```text
$ > cargo +nightly bench
   Compiling rust-barycenter v0.1.0 (/Users/ffarid/src/RUST/rust-barycenter)
    Finished bench [optimized] target(s) in 1.25s
     Running target/release/deps/rust_barycenter-df6957fa89931341

running 4 tests
test tests::bench_merge_iterative_large ... bench:   6,134,557 ns/iter (+/- 1,874,839)
test tests::bench_merge_iterative_small ... bench:       5,378 ns/iter (+/- 214)
test tests::bench_merge_recursive_large ... bench:  18,598,300 ns/iter (+/- 10,618,726)
test tests::bench_merge_recursive_small ... bench:     165,415 ns/iter (+/- 78,220)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
```

## Authors

* [Leo Tindall](https://learning.oreilly.com/search/?query=author%3A%22Leo%20Tindall%22&extended_publisher_data=true&highlight=true&include_assessments=false&include_case_studies=true&include_courses=true&include_orioles=true&include_playlists=true&include_collections=true&include_notebooks=true&is_academic_institution_account=false&source=user&sort=relevance&facet_json=true&page=0)
* Farzad FARID

## References

* "Learning Rust" tutorial: https://www.packtpub.com/eu/application-development/learning-rust-video
