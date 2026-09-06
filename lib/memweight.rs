//! Automatically rewritten from C to Rust
//! Source: lib/memweight.c
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
// memweight - count the total number of bits set in memory area
// @ptr: pointer to the start of the area
// @bytes: the size of the area
//
#[no_mangle]
pub unsafe extern "C" fn memweight(ptr: *const c_void, bytes: usize) -> usize {
    size_t memweight(const void *ptr, size_t bytes)
    {
    let mut ret: usize = 0;
    size_t longs;
    const unsigned char *bitmap = ptr;
    for (; bytes > 0 && ((unsigned long)bitmap) % sizeof(long);
    bytes--, bitmap++)
    ret += hweight8(*bitmap);
    longs = bytes / sizeof(long);
    if (longs) {
    BUG_ON(longs >= INT_MAX / BITS_PER_LONG);
    ret += bitmap_weight((unsigned long *)bitmap,
    longs * BITS_PER_LONG);
    bytes -= longs * sizeof(long);
    bitmap += longs * sizeof(long);
    }
//
// The reason that this last loop is distinct from the preceding
// bitmap_weight() call is to compute 1-bits in the last region smaller
// than sizeof(long) properly on big-endian systems.
//
    for (; bytes > 0; bytes--, bitmap++)
    ret += hweight8(*bitmap);
    return ret;
    }
    EXPORT_SYMBOL(memweight);
