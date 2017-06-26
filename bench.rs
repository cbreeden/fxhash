#![feature(test)]
extern crate test;
use test::{Bencher, black_box};

extern crate fxhash;

macro_rules! generate_benches {
    ($($name:ident, $s:expr),* $(,)*) => (
        $(
            #[bench]
            fn $name(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    fxhash::hash(&s)
                })
            }
        )*
    )
}

generate_benches!(
    bench_003_chars, "123",
    bench_004_chars, "1234",
    bench_011_chars, "12345678901",
    bench_012_chars, "123456789012",
    bench_023_chars, "12345678901234567890123",
    bench_024_chars, "123456789012345678901234",
    bench_068_chars, "11234567890123456789012345678901234567890123456789012345678901234567",
    bench_132_chars, "112345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901",
);