//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/util.h
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
// Simple array based allocator - preallocates a number of elements and you can
// never allocate more than that, also has no locking.
//
// Handy because if you know you only need a fixed number of elements you don't
// have to worry about memory allocation failure, and sometimes a mempool isn't
// what you want.
//
// We treat the free elements as entries in a singly linked list, and the
// freelist as a stack - allocating and freeing push and pop off the freelist.
//

// ((typeof((array)->freelist) *) _ptr) = (array)->freelist;	\

extern "C" {
    pub fn bch_strtoint_h(cp: *const c_char, res: *mut c_int) -> c_int;
}
extern "C" {
    pub fn bch_strtouint_h(cp: *const c_char, res: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn bch_strtoll_h(cp: *const c_char, res: *mut c_longlong) -> c_int;
}
extern "C" {
    pub fn bch_strtoull_h(cp: *const c_char, res: *mut c_ulonglong) -> c_int;
}

extern "C" {
    pub fn bch_strtoint_h(_arg: cp, res: *mut *mut (int )) -> return;
}

extern "C" {
    pub fn bch_strtoll_h(_arg: cp, res: *mut *mut (long long )) -> return;
}

extern "C" {
    pub fn bch_strtouint_h(_arg: cp, res: *mut *mut (unsigned int )) -> return;
}

extern "C" {
    pub fn bch_strtoull_h(_arg: cp, res: *mut *mut (unsigned long long )) -> return;
}

extern "C" {
    pub fn bch_hprint(buf: *mut c_char, v: i64) -> isize;
}
extern "C" {
    pub fn bch_is_zero(p: *const c_char, n: usize) -> bool;
}
extern "C" {
    pub fn bch_parse_uuid(s: *const c_char, uuid: *mut c_char) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_stats {
    pub lock: spinlock_t,
//
// all fields are in nanoseconds, averages are ewmas stored left shifted
// by 8
//
    pub max_duration: u64,
    pub average_duration: u64,
    pub average_frequency: u64,
    pub last: u64,
}

extern "C" {
    pub fn bch_time_stats_update(stats: *mut time_stats, time: u64);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bch_ratelimit {
// Next time we want to do some work, in nanoseconds
    pub next: u64,
//
// Rate at which we want to do work, in units per second
// The units here correspond to the units passed to bch_next_delay()
//
    pub rate: atomic_long_t,
}

extern "C" {
    pub fn bch_next_delay(d: *mut bch_ratelimit, done: u64) -> u64;
}

//
// A stepwise-linear pseudo-exponential.  This returns 1 << (x >>
// frac_bits), with the less-significant bits filled in by linear
// interpolation.
//
// This can also be interpreted as a floating-point number format,
// where the low frac_bits are the mantissa (with implicit leading
// 1 bit), and the more significant bits are the exponent.
// The return value is 1.mantissa * 2^exponent.
//
// The way this is used, fract_bits is 6 and the largest possible
// input is CONGESTED_MAX-1 = 1023 (exponent 16, mantissa 0x1.fc),
// so the maximum output is 0x1fc00.
//
// Largest intermediate value 0x7f0000
extern "C" {
    pub fn bch_bio_map(bio: *mut bio, base: *mut c_void);
}
extern "C" {
    pub fn bch_bio_alloc_pages(bio: *mut bio, gfp_mask: gfp_t) -> c_int;
}
