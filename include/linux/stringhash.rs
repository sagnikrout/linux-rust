//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stringhash.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

//
// Routines for hashing strings of bytes to a 32-bit hash value.
//
// These hash functions are NOT GUARANTEED STABLE between kernel
// versions, architectures, or even repeated boots of the same kernel.
// (E.g. they may depend on boot-time hardware detection or be
// deliberately randomized.)
//
// They are also not intended to be secure against collisions caused by
// malicious inputs; much slower hash functions are required for that.
//
// They are optimized for pathname components, meaning short strings.
// Even if a majority of files have longer names, the dynamic profile of
// pathname components skews short due to short directory names.
// (E.g. /usr/lib/libsesquipedalianism.so.3.141.)
//
// Version 1: one byte at a time.  Example of use:
//
// unsigned long hash = init_name_hash;
// while (*p)
// hash = partial_name_hash(tolower(*p++), hash);
// hash = end_name_hash(hash);
//
// Although this is designed for bytes, fs/hfsplus/unicode.c
// abuses it to hash 16-bit values.
//
// Hash courtesy of the R5 hash in reiserfs modulo sign bits

// partial hash update function. Assume roughly 4 bits per character
//
// Finally: cut down the number of bits to a int value (and try to avoid
// losing bits).  This also has the property (wanted by the dcache)
// that the msbits make a good hash table index.
//
extern "C" {
    pub fn hash_long(_arg: hash, _arg: 32) -> return;
}
//
// Version 2: One word (32 or 64 bits) at a time.
// If CONFIG_DCACHE_WORD_ACCESS is defined (meaning <asm/word-at-a-time.h>
// exists, which describes major Linux platforms like x86 and ARM), then
// this computes a different hash function much faster.
//
// If not set, this falls back to a wrapper around the preceding.
//
extern "C" {
    pub fn full_name_hash(salt: *const c_void, : *const c_char, int: unsigned) -> unsigned int __pure;
}
//
// A hash_len is a u64 with the hash of a string in the low
// half and the length in the high half.
//

// Return the "hash_len" (hash and length) of a null-terminated string
extern "C" {
    pub fn hashlen_string(salt: *const c_void, name: *const c_char) -> u64 __pure;
}
