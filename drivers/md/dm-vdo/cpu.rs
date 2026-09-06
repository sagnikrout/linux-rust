//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/cpu.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2023 Red Hat
//

//
// uds_prefetch_address() - Minimize cache-miss latency by attempting to move data into a CPU cache
// before it is accessed.
//
// @address: the address to fetch (may be invalid)
// @for_write: must be constant at compile time--false if for reading, true if for writing
//
// for_write won't be a constant if we are compiled with optimization turned off, in which
// case prefetching really doesn't matter. clang can't figure out that if for_write is a
// constant, it can be passed as the second, mandatorily constant argument to prefetch(),
// at least currently on llvm 12.
//
// uds_prefetch_range() - Minimize cache-miss latency by attempting to move a range of addresses
// into a CPU cache before they are accessed.
//
// @start: the starting address to fetch (may be invalid)
// @size: the number of bytes in the address range
// @for_write: must be constant at compile time--false if for reading, true if for writing
//
// Count the number of cache lines to fetch, allowing for the address range to span an
// extra cache line boundary due to address alignment.
//
