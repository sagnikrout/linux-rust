//! Automatically rewritten from C Header to Rust Module
//! Source: fs/lockd/xdr.h
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
// XDR types for the NLM protocol
//
// Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de>
//

pub const SM_MAXSTRLEN: c_int = 1024;
pub const SM_PRIV_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsm_private {
    pub data: [c_uchar; SM_PRIV_SIZE],
}

pub const NLM_MAXCOOKIELEN: c_int = 32;
pub const NLM_MAXSTRLEN: c_int = 1024;

// Lock info passed via NLM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockd_lock {
    pub caller: *mut *mut c_char,
    pub /: *mut *mut unsigned int len; / length of "caller",
    pub fh: nfs_fh,
    pub oh: xdr_netobj,
    pub svid: u32,
    pub lock_start: u64,
    pub lock_len: u64,
    pub fl: file_lock,
}

//
// NLM cookies. Technically they can be 1K, but Linux only uses 8 bytes.
// FreeBSD uses 16, Apple Mac OS X 10.3 uses 20. Therefore we set it to
// 32 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockd_cookie {
    pub data: [c_uchar; NLM_MAXCOOKIELEN],
    pub len: c_uint,
}

//
// Generic lockd arguments for all but sm_notify
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockd_args {
    pub cookie: lockd_cookie,
    pub lock: lockd_lock,
    pub block: u32,
    pub reclaim: u32,
    pub state: u32,
}

//
// Generic lockd result
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockd_res {
    pub cookie: lockd_cookie,
    pub status: __be32,
    pub lock: lockd_lock,
}

//
// statd callback when client has rebooted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockd_reboot {
    pub mon: *mut c_char,
    pub len: c_uint,
    pub state: u32,
    pub priv: nsm_private,
}
