//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-verity-verify-sig.h
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
// Copyright (C) 2019 Microsoft Corporation.
//
// Author:  Jaskaran Singh Khurana <jaskarankhurana@linux.microsoft.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_verity_sig_opts {
    pub sig_size: c_uint,
    pub sig: *mut u8,
}

pub const DM_VERITY_ROOT_HASH_VERIFICATION_OPTS: c_int = 2;
extern "C" {
    pub fn verity_verify_is_sig_opt_arg(arg_name: *const c_char) -> bool;
}
extern "C" {
    pub fn verity_verify_sig_opts_cleanup(sig_opts: *mut dm_verity_sig_opts);
}
extern "C" {
    pub fn dm_verity_verify_sig_init() -> int __init;
}
extern "C" {
    pub fn dm_verity_verify_sig_exit();
}

pub const DM_VERITY_ROOT_HASH_VERIFICATION_OPTS: c_int = 0;

