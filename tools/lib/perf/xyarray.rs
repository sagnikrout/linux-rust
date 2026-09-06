//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/xyarray.c
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

    struct xyarray *xyarray__new(int xlen, int ylen, size_t entry_size)
    {
    let mut row_size: usize = ylen * entry_size;
    struct xyarray *xy = zalloc(sizeof(*xy) + xlen * row_size);
    if (xy != core::ptr::null_mut()) {
    xy.entry_size = entry_size;
    xy.row_size   = row_size;
    xy.entries    = xlen * ylen;
    xy.max_x      = xlen;
    xy.max_y      = ylen;
    }
    return xy;
    }
#[no_mangle]
pub unsafe extern "C" fn xyarray__reset(xy: *mut xyarray) {
    void xyarray__reset(struct xyarray *xy)
    {
    let mut n: usize = xy.entries * xy.entry_size;
    memset(xy.contents, 0, n);
    }
#[no_mangle]
pub unsafe extern "C" fn xyarray__delete(xy: *mut xyarray) {
    void xyarray__delete(struct xyarray *xy)
    {
    free(xy);
    }
