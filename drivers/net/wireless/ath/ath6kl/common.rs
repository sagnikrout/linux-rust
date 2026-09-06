//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/common.h
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
// Copyright (c) 2010-2011 Atheros Communications Inc.
// Copyright (c) 2011-2012 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const ATH6KL_MAX_IE: c_int = 256;
//
// Reflects the version of binary interface exposed by ATH6KL target
// firmware. Needs to be incremented by 1 for any change in the firmware
// that requires upgrade of the driver on the host side for the change to
// work correctly
//
pub const ATH6KL_ABI_VERSION: c_int = 1;
pub const SIGNAL_QUALITY_METRICS_NUM_MAX: c_int = 2;
//
// Data Path
//

// An AMSDU frame */ /* The MAX AMSDU length of AR6003 is 3839

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_llc_snap_hdr {
    pub dsap: u8,
    pub ssap: u8,
    pub cntl: u8,
    pub org_code: [u8; 3],
    pub eth_type: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_crypto_type {
    NONE_CRYPT          = 0x01,
    WEP_CRYPT           = 0x02,
    TKIP_CRYPT          = 0x04,
    AES_CRYPT           = 0x08,
    WAPI_CRYPT          = 0x10,
}

    pub htc_endpoint_credit_dist: struct,
    pub ath6kl: struct,
    pub ath6kl_htcap: struct,
    pub htc_credit_dist_reason: enum,
    pub ath6kl_htc_credit_info: struct,
    pub size): *mut *mut sk_buff ath6kl_buf_alloc(int,
