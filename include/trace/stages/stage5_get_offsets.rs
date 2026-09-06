//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/stages/stage5_get_offsets.h
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
// Stage 5 definitions for creating trace events
//
// remember the offset of each array from the beginning of the event.
//

//
// Fields should never declare an array: i.e. __field(int, arr[5])
// If they do, it will cause issues in parsing and possibly corrupt the
// events. To prevent that from happening, test the sizeof() a fictitious
// type called "struct _test_no_array_##item" which will fail if "item"
// contains array elements (like "arr[5]").
//
// If you hit this, use __array(int, arr, 5) instead.
//

//
// __bitmask_size_in_bytes_raw is the number of bytes needed to hold
// num_possible_cpus().
//

//
// __bitmask_size_in_bytes is the number of bytes needed to hold
// num_possible_cpus() padded out to the nearest long. This is what
// is saved in the buffer, just to be consistent.
//

