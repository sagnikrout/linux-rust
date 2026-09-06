//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/proto.h
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
// Copyright (c) 2013 Broadcom Corporation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum proto_addr_mode {
    ADDR_INDIRECT	= 0,
    ADDR_DIRECT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_skb_reorder_data {
    pub reorder: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_proto {
    pub ifp): *mut *mut sk_buff skb, brcmf_if,
    pub fwerr): *mut *mut void buf, uint len, int,
    pub fwerr): *mut uint len, int,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub addr_mode): proto_addr_mode,
    pub peer[ETH_ALEN]): u8,
    pub peer[ETH_ALEN]): u8,
    pub skb): *mut *mut *mut void (rxreorder)(struct brcmf_if ifp, struct sk_buff,
    pub ifp): *mut *mut void (add_if)(struct brcmf_if,
    pub ifp): *mut *mut void (del_if)(struct brcmf_if,
    pub ifp): *mut *mut void (reset_if)(struct brcmf_if,
    pub drvr): *mut *mut int (init_done)(struct brcmf_pub,
    pub drvr): *mut *mut void (debugfs_create)(struct brcmf_pub,
    pub pd: *mut c_void,
}

extern "C" {
    pub fn brcmf_proto_attach(drvr: *mut brcmf_pub) -> c_int;
}
extern "C" {
    pub fn brcmf_proto_detach(drvr: *mut brcmf_pub);
}
// assure protocol is always called with
// non-null initialized pointer.
//
// ifp = NULL;
