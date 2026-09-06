//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/pmc.h
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


// SPDX-License-Identifier: ISC
// Copyright (c) 2012-2015 Qualcomm Atheros, Inc.

extern "C" {
    pub fn wil_pmc_init(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_pmc_free(wil: *mut wil6210_priv, send_pmc_cmd: c_int);
}
extern "C" {
    pub fn wil_pmc_last_cmd_status(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_pmc_read(: *mut file, : *mut char __user, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn wil_pmc_llseek(filp: *mut file, off: loff_t, whence: c_int) -> loff_t;
}
extern "C" {
    pub fn wil_pmcring_read(s: *mut seq_file, data: *mut c_void) -> c_int;
}
