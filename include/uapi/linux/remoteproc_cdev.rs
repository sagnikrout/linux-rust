//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/remoteproc_cdev.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// IOCTLs for Remoteproc's character device interface.
//
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
//

pub const RPROC_MAGIC: c_uint = 0xB7;
//
// The RPROC_SET_SHUTDOWN_ON_RELEASE ioctl allows to enable/disable the shutdown of a remote
// processor automatically when the controlling userpsace closes the char device interface.
//
// input parameter: integer
// 0		: disable automatic shutdown
// other	: enable automatic shutdown
//

//
// The RPROC_GET_SHUTDOWN_ON_RELEASE ioctl gets information about whether the automatic shutdown of
// a remote processor is enabled or disabled when the controlling userspace closes the char device
// interface.
//
// output parameter: integer
// 0		: automatic shutdown disable
// other	: automatic shutdown enable
//

