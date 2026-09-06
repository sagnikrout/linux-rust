//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_vfr.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2016-2017 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

pub const MAX_CFA_CODE: c_int = 65536;
extern "C" {
    pub fn bnxt_vf_reps_create(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_vf_reps_destroy(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_vf_reps_close(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_vf_reps_open(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_vf_rep_rx(bp: *mut bnxt, skb: *mut sk_buff);
}
extern "C" {
    pub fn bnxt_vf_reps_alloc(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_vf_reps_free(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_dev_is_vf_rep(dev: *mut net_device) -> bool;
}
extern "C" {
    pub fn bnxt_dl_eswitch_mode_get(devlink: *mut devlink, mode: *mut u16) -> c_int;
}

