//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/enic_clsf.h
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

pub const ENIC_CLSF_EXPIRE_COUNT: c_int = 128;
extern "C" {
    pub fn enic_addfltr_5t(enic: *mut enic, keys: *mut flow_keys, rq: u16) -> c_int;
}
extern "C" {
    pub fn enic_delfltr(enic: *mut enic, filter_id: u16) -> c_int;
}
extern "C" {
    pub fn enic_rfs_flw_tbl_init(enic: *mut enic);
}
extern "C" {
    pub fn enic_rfs_flw_tbl_free(enic: *mut enic);
}

extern "C" {
    pub fn enic_flow_may_expire(t: *mut timer_list);
}

