//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/fwvid.h
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
// Copyright (c) 2022 Broadcom Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fwvid_ops {
    pub ifp): *mut *mut void (feat_attach)(struct brcmf_if,
    pub crypto): *mut *mut *mut int (set_sae_password)(struct brcmf_if ifp, struct cfg80211_crypto_settings,
    pub drvr): *mut *mut int (alloc_fweh_info)(struct brcmf_pub,
    pub ifp): *mut *mut int (activate_events)(struct brcmf_if,
    pub drvr): *mut *mut void (get_cfg80211_ops)(struct brcmf_pub,
    pub drvr): *mut *mut void (register_event_handlers)(struct brcmf_pub,
}

// exported functions
extern "C" {
    pub fn brcmf_fwvid_unregister_vendor(fwvid: brcmf_fwvendor, mod: *mut module) -> c_int;
}
// core driver functions
extern "C" {
    pub fn brcmf_fwvid_attach(drvr: *mut brcmf_pub) -> c_int;
}
extern "C" {
    pub fn brcmf_fwvid_detach(drvr: *mut brcmf_pub);
}
