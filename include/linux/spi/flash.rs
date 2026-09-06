//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/flash.h
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
// struct flash_platform_data: board-specific flash data
// @name: optional flash device name (eg, as used with mtdparts=)
// @parts: optional array of mtd_partitions for static partitioning
// @nr_parts: number of mtd_partitions for static partitioning
// @type: optional flash device type (e.g. m25p80 vs m25p64), for use
// with chips that can't be queried for JEDEC or other IDs
//
// Board init code (in arch/.../mach-xxx/board-yyy.c files) can
// provide information about SPI flash parts (such as DataFlash) to
// help set up the device and its appropriate default partitioning.
//
// Note that for DataFlash, sizes for pages, blocks, and sectors are
// rarely powers of two; and partitions should be sector-aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_platform_data {
    pub name: *mut c_char,
    pub parts: *mut mtd_partition,
    pub nr_parts: c_uint,
    pub type: *mut c_char,
// we'll likely add more ... use JEDEC IDs, etc
}
