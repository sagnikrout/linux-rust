//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/vendor.h
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

// Macro flag: #define _vendor_h_
pub const BROADCOM_OUI: c_uint = 0x001018;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_vndr_cmds {
    BRCMF_VNDR_CMDS_UNSPEC,
    BRCMF_VNDR_CMDS_DCMD,
    BRCMF_VNDR_CMDS_LAST
}

//
// enum brcmf_nlattrs - nl80211 message attributes
//
// @BRCMF_NLATTR_LEN: message body length
// @BRCMF_NLATTR_DATA: message body
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_nlattrs {
    BRCMF_NLATTR_UNSPEC,

    BRCMF_NLATTR_LEN,
    BRCMF_NLATTR_DATA,

    __BRCMF_NLATTR_AFTER_LAST,
    BRCMF_NLATTR_MAX = __BRCMF_NLATTR_AFTER_LAST - 1
}

//
// struct brcmf_vndr_dcmd_hdr - message header for cfg80211 vendor command dcmd
// support
//
// @cmd: common dongle cmd definition
// @len: length of expecting return buffer
// @offset: offset of data buffer
// @set: get or set request(optional)
// @magic: magic number for verification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_vndr_dcmd_hdr {
    pub cmd: c_uint,
    pub len: c_int,
    pub offset: c_uint,
    pub set: c_uint,
    pub magic: c_uint,
}
