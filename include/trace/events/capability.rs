//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/capability.h
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
// cap_capable - called after it's determined if a task has a particular
// effective capability
//
// @cred: The credentials used
// @target_ns: The user namespace of the resource being accessed
// @capable_ns: The user namespace in which the credential provides the
// capability to access the targeted resource.
// This will be NULL if ret is not 0.
// @cap: The capability to check for
// @ret: The return value of the check: 0 if it does, -ve if it does not
//
// Allows to trace calls to cap_capable in commoncap.c
//

// This part must be outside protection
