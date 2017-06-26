// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]

//! # Fx Hash
//!
//! This hashing algorithm was extracted from the Rustc compiler.  This is the same hashing
//! algoirthm used for some internal operations in FireFox.  The strength of this algorithm
//! is in hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm works on one
//! byte at a time.
//!
//! ## Disclaimer
//!
//! It is **not a cryptographically secure** hash, so it is strongly recommended that you do
//! not use this hash for cryptographic purproses.  Furthermore, this hashing algorithm was
//! not designed to prevent any attacks for determining collisions which could be used to
//! potentially cause quadratic behavior in `HashMap`s.  So it is not recommended to expose
//! this hash in places where collissions or DDOS attacks may be a concern.

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

/// This hashing algorithm was extracted from the Rustc compiler.
/// This is the same hashing algoirthm used for some internal operations in FireFox.
/// The strength of this algorithm is in hashing 8 bytes at a time on 64-bit platforms,
/// where the FNV algorithm works on one byte at a time.
///
/// This hashing algorithm should not be used for cryptographic, or in scenarios where
/// DOS attacks are a concern.
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
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        let ptr_size = std::mem::size_of::<usize>();
        while bytes.len() >= ptr_size {
            let n = NativeEndian::read_u32(bytes) as usize;
            self.add_to_hash(n as usize);
            bytes = bytes.split_at(ptr_size).1;
        }

        for byte in bytes {
            let i = *byte;
            self.add_to_hash(i as usize);
        }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        let ptr_size = std::mem::size_of::<usize>();
        while bytes.len() >= ptr_size {
            let n = NativeEndian::read_u64(bytes);
            self.add_to_hash(n as usize);
            bytes = bytes.split_at(ptr_size).1;
        }

        while bytes.len() >= 4 {
            let n = NativeEndian::read_u32(bytes);
            self.add_to_hash(n as usize);
            bytes = bytes.split_at(4).1;
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

/// A convenience function for when you need a quick hash.
#[inline]
pub fn hash<T: Hash + ?Sized>(v: &T) -> u64 {
    let mut state = FxHasher::default();
    v.hash(&mut state);
    state.finish()
}