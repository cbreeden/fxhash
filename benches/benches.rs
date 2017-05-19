#![feature(test)]
extern crate test;
extern crate fxhash;

use test::Bencher;
use test::black_box;

#[bench]
fn bench_3chars(b: &mut Bencher) {
    let s = &black_box("abc");
    b.iter(|| {
        fxhash::hash(s)
    })
}

#[bench]
fn bench_4chars(b: &mut Bencher) {
    let s = &black_box("abcd");
    b.iter(|| {
        fxhash::hash(s)
    })
}

#[bench]
fn bench_11chars(b: &mut Bencher) {
    let s = &black_box("abcdefghijk");
    b.iter(|| {
        fxhash::hash(s)
    })
}

#[bench]
fn bench_12chars(b: &mut Bencher) {
    let s = &black_box("abcdefghijkl");
    b.iter(|| {
        fxhash::hash(s)
    })
}

#[bench]
fn bench_23chars(b: &mut Bencher) {
    let s = &black_box("abcdefghijklabcdefghijk");
    b.iter(|| {
        fxhash::hash(s)
    })
}

#[bench]
fn bench_24chars(b: &mut Bencher) {
    let s = &black_box("abcdefghijklabcdefghijkl");
    b.iter(|| {
        fxhash::hash(s)
    })
}