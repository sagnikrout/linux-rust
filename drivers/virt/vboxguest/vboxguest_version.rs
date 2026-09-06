//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virt/vboxguest/vboxguest_version.h
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


// SPDX-License-Identifier: (GPL-2.0 OR CDDL-1.0)
//
// VBox Guest additions version info, this is used by the host to determine
// supported guest-addition features in some cases. So this will need to be
// synced with vbox upstreams versioning scheme when we implement / port
// new features from the upstream out-of-tree vboxguest driver.
//
pub const VBG_VERSION_MAJOR: c_int = 6;
pub const VBG_VERSION_MINOR: c_int = 0;
pub const VBG_VERSION_BUILD: c_int = 0;
pub const VBG_SVN_REV: c_int = 127566;

