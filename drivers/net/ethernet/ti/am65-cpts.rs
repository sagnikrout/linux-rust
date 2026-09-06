//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/am65-cpts.h
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


// SPDX-License-Identifier: GPL-2.0+
// TI K3 AM65 CPTS driver interface
//
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpts_estf_cfg {
    pub ns_period: u64,
    pub ns_start: u64,
}

extern "C" {
    pub fn am65_cpts_release(cpts: *mut am65_cpts);
}
extern "C" {
    pub fn am65_cpts_phc_index(cpts: *mut am65_cpts) -> c_int;
}
extern "C" {
    pub fn am65_cpts_tx_timestamp(cpts: *mut am65_cpts, skb: *mut sk_buff);
}
extern "C" {
    pub fn am65_cpts_ns_gettime(cpts: *mut am65_cpts) -> u64;
}
extern "C" {
    pub fn am65_cpts_estf_disable(cpts: *mut am65_cpts, idx: c_int);
}
extern "C" {
    pub fn am65_cpts_suspend(cpts: *mut am65_cpts);
}
extern "C" {
    pub fn am65_cpts_resume(cpts: *mut am65_cpts);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

