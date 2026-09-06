//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/libeth/cache.h
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
// Copyright (C) 2024 Intel Corporation

//
// libeth_cacheline_group_assert - make sure cacheline group size is expected
// @type: type of the structure containing the group
// @grp: group name inside the struct
// @sz: expected group size
//

//
// libeth_cacheline_struct_assert - make sure CL-based struct size is expected
// @type: type of the struct
// @...: from 1 to 3 CL group sizes (read-mostly, read-write, cold)
//
// When a struct contains several CL groups, it's difficult to predict its size
// on different architectures. The macro instead takes sizes of all of the
// groups the structure contains and generates the final struct size.
//

//
// libeth_cacheline_set_assert - make sure CL-based struct layout is expected
// @type: type of the struct
// @ro: expected size of the read-mostly group
// @rw: expected size of the read-write group
// @c: expected size of the cold group
//
// Check that each group size is expected and then do final struct size check.
//

