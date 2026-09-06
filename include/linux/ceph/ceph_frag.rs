//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/ceph_frag.h
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
// "Frags" are a way to describe a subset of a 32-bit number space,
// using a mask and a value to match against that mask.  Any given frag
// (subset of the number space) can be partitioned into 2^n sub-frags.
//
// Frags are encoded into a 32-bit word:
// 8 upper bits = "bits"
// 24 lower bits = "value"
// (We could go to 5+27 bits, but who cares.)
//
// We use the _most_ significant bits of the 24 bit value.  This makes
// values logically sort.
//
// Unfortunately, because the "bits" field is still in the high bits, we
// can't sort encoded frags numerically.  However, it does allow you
// to feed encoded frags as values into frag_contains_value.
//
extern "C" {
    pub fn ceph_frag_value(ceph_frag_mask(f: f) ==) -> return;
}
//
// comparator to sort frags logically, as when traversing the
// number space in ascending order...
//
extern "C" {
    pub fn ceph_frag_compare(a: __u32, b: __u32) -> c_int;
}
