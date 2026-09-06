//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/commonring.h
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
//
// Copyright (c) 2014 Broadcom Corporation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_commonring {
    pub r_ptr: u16,
    pub w_ptr: u16,
    pub f_ptr: u16,
    pub depth: u16,
    pub item_len: u16,
    pub buf_addr: *mut c_void,
    pub ctx): *mut *mut int (cr_ring_bell)(void,
    pub ctx): *mut *mut int (cr_update_rptr)(void,
    pub ctx): *mut *mut int (cr_update_wptr)(void,
    pub ctx): *mut *mut int (cr_write_rptr)(void,
    pub ctx): *mut *mut int (cr_write_wptr)(void,
    pub cr_ctx: *mut c_void,
    pub lock: spinlock_t,
    pub flags: c_ulong,
    pub inited: bool,
    pub was_full: bool,
    pub outstanding_tx: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn brcmf_commonring_lock(commonring: *mut brcmf_commonring);
}
extern "C" {
    pub fn brcmf_commonring_unlock(commonring: *mut brcmf_commonring);
}
extern "C" {
    pub fn brcmf_commonring_write_available(commonring: *mut brcmf_commonring) -> bool;
}
extern "C" {
    pub fn brcmf_commonring_write_complete(commonring: *mut brcmf_commonring) -> c_int;
}

