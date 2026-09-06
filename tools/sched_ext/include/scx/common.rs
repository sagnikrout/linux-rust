//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/common.h
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2023 Tejun Heo <tj@kernel.org>
// Copyright (c) 2023 David Vernet <dvernet@meta.com>
//

pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;

//
// RESIZE_ARRAY - Convenience macro for resizing a BPF array
// @__skel: the skeleton containing the array
// @elfsec: the data section of the BPF program in which the array exists
// @arr: the name of the array
// @n: the desired array element count
//
// For BPF arrays declared with RESIZABLE_ARRAY(), this macro performs two
// operations. It resizes the map which corresponds to the custom data
// section that contains the target array. As a side effect, the BTF info for
// the array is adjusted so that the array length is sized to cover the new
// data section size. The second operation is reassigning the skeleton pointer
// for that custom data section so that it points to the newly memory mapped
// region.
//

