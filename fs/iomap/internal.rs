//! Automatically rewritten from C Header to Rust Module
//! Source: fs/iomap/internal.h
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
pub const _IOMAP_INTERNAL_H: c_int = 1;
pub const IOEND_BATCH_SIZE: c_int = 4096;
//
// Normally we can build bios as big as the data structure supports.
//
// But for integrity protected I/O we need to respect the maximum size of the
// single contiguous allocation for the integrity buffer.
//
extern "C" {
    pub fn max_integrity_io_size(_arg: bdev_limits(iomap->bdev)) -> return;
}
extern "C" {
    pub fn iomap_finish_ioend_buffered_read(ioend: *mut iomap_ioend) -> u32;
}
extern "C" {
    pub fn iomap_finish_ioend_direct(ioend: *mut iomap_ioend) -> u32;
}

