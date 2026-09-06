//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/jhash.h
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


// jhash.h: Jenkins hash support.
//
// Copyright (C) 2006. Bob Jenkins (bob_jenkins@burtleburtle.net)
//
// https://burtleburtle.net/bob/hash
//
// These are the credits from Bob's sources:
//
// lookup3.c, by Bob Jenkins, May 2006, Public Domain.
//
// These are functions for producing 32-bit hashes for hash table lookup.
// hashword(), hashlittle(), hashlittle2(), hashbig(), mix(), and final()
// are externally useful functions.  Routines to test the hash are included
// if SELF_TEST is defined.  You can use this free for any purpose.  It's in
// the public domain.  It has no warranty.
//
// Copyright (C) 2009-2010 Jozsef Kadlecsik (kadlec@netfilter.org)
//
// I've modified Bob's hash to be useful in the Linux kernel, and
// any bugs present are my fault.
// Jozsef
//

// Best hash sizes are of power of two

// Mask the hash value, i.e (value & jhash_mask(n)) instead of (value % n)

// __jhash_mix - mix 3 32-bit values reversibly.

// __jhash_final - final mixing of 3 32-bit values (a,b,c) into c

// An arbitrary initial parameter
pub const JHASH_INITVAL: c_uint = 0xdeadbeef;
// jhash - hash an arbitrary key
// @k: sequence of bytes as key
// @length: the length of the key
// @initval: the previous hash, or an arbitrary value
//
// The generic version, hashes an arbitrary sequence of bytes.
// No alignment or length assumptions are made about the input key.
//
// Returns the hash value of the key. The result depends on endianness.
//
// Set up the internal state
// All but the last block: affect some 32 bits of (a,b,c)
// Last block: affect all 32 bits of (c)
// jhash2 - hash an array of u32's
// @k: the key which must be an array of u32's
// @length: the number of u32's in the key
// @initval: the previous hash, or an arbitrary value
//
// Returns the hash value of the key.
//
// Set up the internal state
// Handle most of the key
// Handle the last 3 u32's
// __jhash_nwords - hash exactly 3, 2 or 1 word(s)
extern "C" {
    pub fn __jhash_nwords(_arg: a, _arg: b, _arg: c, 2): initval + JHASH_INITVAL + (3 <<) -> return;
}
extern "C" {
    pub fn __jhash_nwords(_arg: a, _arg: b, _arg: 0, 2): initval + JHASH_INITVAL + (2 <<) -> return;
}
extern "C" {
    pub fn __jhash_nwords(_arg: a, _arg: 0, _arg: 0, 2): initval + JHASH_INITVAL + (1 <<) -> return;
}
