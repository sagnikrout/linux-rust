//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/ucode_loader.h
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


//
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const MAX_FW_SIZE: c_int = 150000;
pub const UCODE_LOADER_API_VER: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_ucode {
    pub d11lcn0bsinitvals24: *mut d11init,
    pub d11lcn0initvals24: *mut d11init,
    pub d11lcn1bsinitvals24: *mut d11init,
    pub d11lcn1initvals24: *mut d11init,
    pub d11lcn2bsinitvals24: *mut d11init,
    pub d11lcn2initvals24: *mut d11init,
    pub d11n0absinitvals16: *mut d11init,
    pub d11n0bsinitvals16: *mut d11init,
    pub d11n0initvals16: *mut d11init,
    pub bcm43xx_16_mimo: *mut __le32,
    pub bcm43xx_16_mimosz: usize,
    pub bcm43xx_24_lcn: *mut __le32,
    pub bcm43xx_24_lcnsz: usize,
    pub bcm43xx_bommajor: *mut u32,
    pub bcm43xx_bomminor: *mut u32,
}

extern "C" {
    pub fn brcms_ucode_data_init(wl: *mut brcms_info, ucode: *mut brcms_ucode) -> c_int;
}
extern "C" {
    pub fn brcms_ucode_data_free(ucode: *mut brcms_ucode);
}
extern "C" {
    pub fn brcms_ucode_init_buf(wl: *mut brcms_info, pbuf: *mut c_void, idx: c_uint) -> c_int;
}
extern "C" {
    pub fn brcms_ucode_free_buf(: *mut c_void);
}
extern "C" {
    pub fn brcms_check_firmwares(wl: *mut brcms_info) -> c_int;
}
