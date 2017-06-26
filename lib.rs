// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//#![deny(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::hash::{Hasher, Hash, BuildHasherDefault};
use std::ops::BitXor;

extern crate byteorder;
use byteorder::{ByteOrder, NativeEndian};

/// A builder for default Fx hashers.
pub type FxBuildHasher = BuildHasherDefault<FxHasher>;

/// A `HashMap` using a default Fx hasher.
pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

/// A `HashSet` using a default Fx hasher.
pub type FxHashSet<V> = HashSet<V, FxBuildHasher>;

/// A speedy hash algorithm used within rustc. The hashmap in libcollections
/// by default uses SipHash which isn't quite as speedy as we want. In the
/// compiler we're not really worried about DOS attempts, so we use a fast
/// non-cryptographic hash.
///
/// This is the same as the algorithm used by Firefox -- which is a homespun
/// one not based on any widely-known algorithm -- though modified to produce
/// 64-bit hash values instead of 32-bit hash values. It consistently
/// out-performs an FNV-based hash within rustc itself -- the collision rate is
/// similar or slightly worse than FNV, but the speed of the hash function
/// itself is much higher because it works on up to 8 bytes at a time.
///
/// TODO: Reword this to be less rustc specific.
#[derive(Debug, Clone)]
pub struct FxHasher {
    hash: usize,
}

impl Default for FxHasher {
    #[inline]
    fn default() -> FxHasher {
        FxHasher { hash: 0 }
    }
}

#[cfg(target_pointer_width = "32")]
const K: usize = 0x9e3779b9;
#[cfg(target_pointer_width = "64")]
const K: usize = 0x517cc1b727220a95;

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: usize) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        #[cfg(target_pointer_width = "32")]
        fn read(buf: &[u8]) -> usize {
            NativeEndian::read_u32(buf) as usize
        }

        #[cfg(target_pointer_width = "64")]
        fn read(buf: &[u8]) -> usize {
            NativeEndian::read_u64(buf) as usize
        }

        let ptr_size: usize = ::std::mem::size_of::<usize>();
        while bytes.len() >= ptr_size {
            let i = read(bytes);
            self.add_to_hash(i as usize);
            bytes = bytes.split_at(ptr_size).1;
        }

        for byte in bytes {
            let i = *byte;
            self.add_to_hash(i as usize);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as usize);
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i as usize);
        self.add_to_hash((i >> 32) as usize);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }
}

/// A helper function.
#[inline]
pub fn hash<T: Hash + ?Sized>(v: &T) -> u64 {
    let mut state = FxHasher::default();
    v.hash(&mut state);
    state.finish()
}

fn test(s: &str) {
    s.lines_any();
}