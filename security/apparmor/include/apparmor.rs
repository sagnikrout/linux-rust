//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/apparmor.h
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
// AppArmor security module
//
// This file contains AppArmor basic global
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.
//

//
// Class of mediation types in the AppArmor policy db
//
pub const AA_CLASS_NONE: c_int = 0;
pub const AA_CLASS_UNKNOWN: c_int = 1;
pub const AA_CLASS_FILE: c_int = 2;
pub const AA_CLASS_CAP: c_int = 3;
pub const AA_CLASS_DEPRECATED: c_int = 4;
pub const AA_CLASS_RLIMITS: c_int = 5;
pub const AA_CLASS_DOMAIN: c_int = 6;
pub const AA_CLASS_MOUNT: c_int = 7;
pub const AA_CLASS_PTRACE: c_int = 9;
pub const AA_CLASS_SIGNAL: c_int = 10;
pub const AA_CLASS_XMATCH: c_int = 11;
pub const AA_CLASS_NET: c_int = 14;
pub const AA_CLASS_NETV9: c_int = 15;
pub const AA_CLASS_LABEL: c_int = 16;
pub const AA_CLASS_POSIX_MQUEUE: c_int = 17;
pub const AA_CLASS_MODULE: c_int = 19;
pub const AA_CLASS_DISPLAY_LSM: c_int = 20;
pub const AA_CLASS_NS: c_int = 21;
pub const AA_CLASS_IO_URING: c_int = 22;
pub const AA_CLASS_NETV9_SKB: c_int = 30;
pub const AA_CLASS_X: c_int = 31;
pub const AA_CLASS_DBUS: c_int = 32;
// NOTE: if AA_CLASS_LAST > 63 need to update label->mediates

// Control parameters settable through module/boot flags

pub const AA_MIN_CLEVEL: c_int = 0;
pub const AA_MAX_CLEVEL: c_int = 0;
pub const AA_DEFAULT_CLEVEL: c_int = 0;

