//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nospec.h
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
// Copyright(c) 2018 Linus Torvalds. All rights reserved.
// Copyright(c) 2018 Alexei Starovoitov. All rights reserved.
// Copyright(c) 2018 Intel Corporation. All rights reserved.

//
// array_index_mask_nospec() - generate a ~0 mask when index < size, 0 otherwise
// @index: array element index
// @size: number of elements in array
//
// When @index is out of bounds (@index >= @size), the sign bit will be
// set.  Extend the sign bit to all bits and invert, giving a result of
// zero for an out of bounds index, or ~0 if within bounds [0, @size).
//

//
// Always calculate and emit the mask even if the compiler
// thinks the mask is not needed. The compiler does not take
// into account the value of @index under speculation.
//

//
// array_index_nospec - sanitize an array index after a bounds check
//
// For a code sequence like:
//
// if (index < size) {
// index = array_index_nospec(index, size);
// val = array[index];
// }
//
// ...if the CPU speculates past the bounds check then
// array_index_nospec() will clamp the index within the range of [0,
// size).
//

// Speculation control prctl
extern "C" {
    pub fn arch_prctl_spec_ctrl_get(task: *mut task_struct, which: c_ulong) -> c_int;
}
// Speculation control for seccomp enforced mitigation
extern "C" {
    pub fn arch_seccomp_spec_mitigate(task: *mut task_struct);
}
