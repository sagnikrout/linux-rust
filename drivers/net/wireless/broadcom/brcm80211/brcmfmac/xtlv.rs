//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/xtlv.h
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
// Copyright (c) 2019 Broadcom
//

// bcm type(id), length, value with w/16 bit id/len. The structure below
// is nominal, and is used to support variable length id and type. See
// xtlv options below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_xtlv {
    pub id: u16,
    pub len: u16,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_xtlv_option {
    BRCMF_XTLV_OPTION_ALIGN32 = BIT(0),
    BRCMF_XTLV_OPTION_IDU8 = BIT(1),
    BRCMF_XTLV_OPTION_LENU8 = BIT(2),
}

extern "C" {
    pub fn brcmf_xtlv_data_size(dlen: c_int, opts: u16) -> c_int;
}
