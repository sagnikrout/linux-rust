//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/reg.h
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
// Copyright (c) 2008-2009 Atheros Communications Inc.
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
pub const AR_MIBC: c_uint = 0x0040;
pub const AR_MIBC_COW: c_uint = 0x00000001;
pub const AR_MIBC_FMC: c_uint = 0x00000002;
pub const AR_MIBC_CMC: c_uint = 0x00000004;
pub const AR_MIBC_MCS: c_uint = 0x00000008;
pub const AR_STA_ID0: c_uint = 0x8000;
pub const AR_STA_ID1: c_uint = 0x8004;
pub const AR_STA_ID1_SADH_MASK: c_uint = 0x0000ffff;
//
// BSSID mask registers. See ath_hw_set_bssid_mask()
// for detailed documentation about these registers.
//
pub const AR_BSSMSKL: c_uint = 0x80e0;
pub const AR_BSSMSKU: c_uint = 0x80e4;
pub const AR_TFCNT: c_uint = 0x80ec;
pub const AR_RFCNT: c_uint = 0x80f0;
pub const AR_RCCNT: c_uint = 0x80f4;
pub const AR_CCCNT: c_uint = 0x80f8;
pub const AR_KEYTABLE_0: c_uint = 0x8800;

pub const AR_KEY_CACHE_SIZE: c_int = 128;
pub const AR_RSVD_KEYTABLE_ENTRIES: c_int = 4;
pub const AR_KEY_TYPE: c_uint = 0x00000007;
pub const AR_KEYTABLE_TYPE_40: c_uint = 0x00000000;
pub const AR_KEYTABLE_TYPE_104: c_uint = 0x00000001;
pub const AR_KEYTABLE_TYPE_128: c_uint = 0x00000003;
pub const AR_KEYTABLE_TYPE_TKIP: c_uint = 0x00000004;
pub const AR_KEYTABLE_TYPE_AES: c_uint = 0x00000005;
pub const AR_KEYTABLE_TYPE_CCM: c_uint = 0x00000006;
pub const AR_KEYTABLE_TYPE_CLR: c_uint = 0x00000007;
pub const AR_KEYTABLE_ANT: c_uint = 0x00000008;
pub const AR_KEYTABLE_VALID: c_uint = 0x00008000;

