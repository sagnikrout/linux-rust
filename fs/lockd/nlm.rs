//! Automatically rewritten from C Header to Rust Module
//! Source: fs/lockd/nlm.h
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
// Declarations for the Network Lock Manager protocol.
//
// Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
//
// Maximum file offset in file_lock.fl_end

// Return states for NLM

pub const NLM_PROGRAM: c_int = 100021;
pub const NLMPROC_NULL: c_int = 0;
pub const NLMPROC_TEST: c_int = 1;
pub const NLMPROC_LOCK: c_int = 2;
pub const NLMPROC_CANCEL: c_int = 3;
pub const NLMPROC_UNLOCK: c_int = 4;
pub const NLMPROC_GRANTED: c_int = 5;
pub const NLMPROC_TEST_MSG: c_int = 6;
pub const NLMPROC_LOCK_MSG: c_int = 7;
pub const NLMPROC_CANCEL_MSG: c_int = 8;
pub const NLMPROC_UNLOCK_MSG: c_int = 9;
pub const NLMPROC_GRANTED_MSG: c_int = 10;
pub const NLMPROC_TEST_RES: c_int = 11;
pub const NLMPROC_LOCK_RES: c_int = 12;
pub const NLMPROC_CANCEL_RES: c_int = 13;
pub const NLMPROC_UNLOCK_RES: c_int = 14;
pub const NLMPROC_GRANTED_RES: c_int = 15;

pub const NLMPROC_SHARE: c_int = 20;
pub const NLMPROC_UNSHARE: c_int = 21;
pub const NLMPROC_NM_LOCK: c_int = 22;
pub const NLMPROC_FREE_ALL: c_int = 23;
