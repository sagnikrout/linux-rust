//! Automatically rewritten from C Header to Rust Module
//! Source: fs/lockd/share.h
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
// DOS share management for lockd.
//
// Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
//

// Synthetic svid for lockowner lookup during share operations

// One bit per (access, deny) pair; index = (access << 2) | deny

//
// DOS share for a specific file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockd_share {
    pub /: *mut *mut *mut lockd_share  s_next; / linked list,
    pub /: *mut *mut *mut nlm_host  s_host; / client host,
    pub /: *mut *mut *mut nlm_file  s_file; / shared file,
    pub /: *mut *mut xdr_netobj s_owner; / owner handle,
    pub /: *mut *mut u32 s_access; / access mode,
    pub /: *mut *mut u32 s_mode; / deny mode,
    pub /: *mut *mut u16 s_access_deny_bmap; / held (access, deny) pairs,
}
