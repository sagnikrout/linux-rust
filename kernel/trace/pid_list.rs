//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/pid_list.h
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
// Do not include this file directly.
//
// In order to keep track of what pids to trace, a tree is created much
// like page tables are used. This creates a sparse bit map, where
// the tree is filled in when needed. A PID is at most 30 bits (see
// linux/thread.h), and is broken up into 3 sections based on the bit map
// of the bits. The 8 MSB is the "upper1" section. The next 8 MSB is the
// "upper2" section and the 14 LSB is the "lower" section.
//
// A trace_pid_list structure holds the "upper1" section, in an
// array of 256 pointers (1 or 2K in size) to "upper_chunk" unions, where
// each has an array of 256 pointers (1 or 2K in size) to the "lower_chunk"
// structures, where each has an array of size 2K bytes representing a bitmask
// of the 14 LSB of the PID (256 * 8 = 2048)
//
// When a trace_pid_list is allocated, it includes the 256 pointer array
// of the upper1 unions. Then a "cache" of upper and lower is allocated
// where these will be assigned as needed.
//
// When a bit is set in the pid_list bitmask, the pid to use has
// the 8 MSB masked, and this is used to index the array in the
// pid_list to find the next upper union. If the element is NULL,
// then one is retrieved from the upper_list cache. If none is
// available, then -ENOMEM is returned.
//
// The next 8 MSB is used to index into the "upper2" section. If this
// element is NULL, then it is retrieved from the lower_list cache.
// Again, if one is not available -ENOMEM is returned.
//
// Finally the 14 LSB of the PID is used to set the bit in the 16384
// bitmask (made up of 2K bytes).
//
// When the second upper section or the lower section has their last
// bit cleared, they are added back to the free list to be reused
// when needed.
//
pub const UPPER_BITS: c_int = 8;

pub const LOWER_BITS: c_int = 14;

// According to linux/thread.h pids can not be bigger than or equal to 1 << 30

// Just keep 6 chunks of both upper and lower in the cache on alloc
pub const CHUNK_ALLOC: c_int = 6;
// Have 2 chunks free, trigger a refill of the cache
pub const CHUNK_REALLOC: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub union lower_chunk {
    pub next: *mut lower_chunk,
    pub size: unsigned long data[LOWER_SIZE]; // 2K in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union upper_chunk {
    pub next: *mut upper_chunk,
    pub size: *mut *mut lower_chunk data[UPPER2_SIZE]; // 1 or 2K in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_pid_list {
    pub seqcount: seqcount_raw_spinlock_t,
    pub lock: raw_spinlock_t,
    pub refill_irqwork: irq_work,
    pub size: *mut *mut upper_chunk upper[UPPER1_SIZE]; // 1 or 2K in,
    pub upper_list: *mut upper_chunk,
    pub lower_list: *mut lower_chunk,
    pub free_upper_chunks: c_int,
    pub free_lower_chunks: c_int,
}
