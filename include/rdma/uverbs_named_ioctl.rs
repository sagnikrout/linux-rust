//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/uverbs_named_ioctl.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2018, Mellanox Technologies inc.  All rights reserved.
//

// These are static so they do not need to be qualified

// Create a standard destroy method using the default handler. The handle_attr
// argument must be the attribute specifying the handle to destroy, the
// default handler does not support any other attributes.
//

//
// Declare global methods. These still have a unique object_id because we
// identify all uapi methods with a (object,method) tuple. However, they have
// no type pointer.
//

// Used by drivers to declare a complete parsing tree for new methods
//

// Used by drivers to declare a complete parsing tree for a single method that
// differs only in having additional driver specific attributes.
//

