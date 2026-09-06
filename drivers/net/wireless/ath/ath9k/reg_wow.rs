//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/reg_wow.h
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
// Copyright (c) 2015 Qualcomm Atheros Inc.
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
pub const AR_WOW_PATTERN: c_uint = 0x825C;
pub const AR_WOW_COUNT: c_uint = 0x8260;
pub const AR_WOW_BCN_EN: c_uint = 0x8270;
pub const AR_WOW_BCN_TIMO: c_uint = 0x8274;
pub const AR_WOW_KEEP_ALIVE_TIMO: c_uint = 0x8278;
pub const AR_WOW_KEEP_ALIVE: c_uint = 0x827c;
pub const AR_WOW_KEEP_ALIVE_DELAY: c_uint = 0x8288;
pub const AR_WOW_PATTERN_MATCH: c_uint = 0x828c;
//
// AR_WOW_LENGTH1
// bit 31:24 pattern 0 length
// bit 23:16 pattern 1 length
// bit 15:8 pattern 2 length
// bit 7:0 pattern 3 length
//
// AR_WOW_LENGTH2
// bit 31:24 pattern 4 length
// bit 23:16 pattern 5 length
// bit 15:8 pattern 6 length
// bit 7:0 pattern 7 length
//
// AR_WOW_LENGTH3
// bit 31:24 pattern 8 length
// bit 23:16 pattern 9 length
// bit 15:8 pattern 10 length
// bit 7:0 pattern 11 length
//
// AR_WOW_LENGTH4
// bit 31:24 pattern 12 length
// bit 23:16 pattern 13 length
// bit 15:8 pattern 14 length
// bit 7:0 pattern 15 length
//
pub const AR_WOW_LENGTH1: c_uint = 0x8360;

pub const AR_WOW_PATTERN_MATCH_LT_256B: c_uint = 0x8368;
pub const AR_MAC_PCU_WOW4: c_uint = 0x8370;
pub const AR_SW_WOW_CONTROL: c_uint = 0x20018;
pub const AR_SW_WOW_ENABLE: c_uint = 0x1;
pub const AR_SWITCH_TO_REFCLK: c_uint = 0x2;
pub const AR_RESET_CONTROL: c_uint = 0x4;
pub const AR_RESET_VALUE_MASK: c_uint = 0x8;
pub const AR_HW_WOW_DISABLE: c_uint = 0x10;
pub const AR_CLR_MAC_INTERRUPT: c_uint = 0x20;
pub const AR_CLR_KA_INTERRUPT: c_uint = 0x40;

pub const AR_WOW_MAC_INTR_EN: c_uint = 0x00040000;
pub const AR_WOW_MAGIC_EN: c_uint = 0x00010000;

pub const AR_WOW_PAT_FOUND_SHIFT: c_int = 8;

pub const AR_WOW_MAGIC_PAT_FOUND: c_uint = 0x00020000;
pub const AR_WOW_MAC_INTR: c_uint = 0x00080000;
pub const AR_WOW_KEEP_ALIVE_FAIL: c_uint = 0x00100000;
pub const AR_WOW_BEACON_FAIL: c_uint = 0x00200000;

pub const AR_WOW2_PATTERN_FOUND_SHIFT: c_int = 8;

pub const AR_WOW_BEACON_FAIL_EN: c_uint = 0x00000001;
pub const AR_WOW_BEACON_TIMO: c_uint = 0x40000000;
pub const AR_WOW_KEEP_ALIVE_NEVER: c_uint = 0xffffffff;
pub const AR_WOW_KEEP_ALIVE_AUTO_DIS: c_uint = 0x00000001;
pub const AR_WOW_KEEP_ALIVE_FAIL_DIS: c_uint = 0x00000002;
pub const AR_WOW_KEEP_ALIVE_DELAY_VALUE: c_uint = 0x000003e8 /* 1 msec */;
pub const AR_WOW_BMISSTHRESHOLD: c_uint = 0x20;

pub const AR_WOW_PAT_BACKOFF: c_uint = 0x00000004;
pub const AR_WOW_CNT_AIFS_CNT: c_uint = 0x00000022;
pub const AR_WOW_CNT_SLOT_CNT: c_uint = 0x00000009;
pub const AR_WOW_CNT_KA_CNT: c_uint = 0x00000008;
pub const AR_WOW_TRANSMIT_BUFFER: c_uint = 0xe000;

pub const AR_WOW_KA_DESC_WORD2: c_uint = 0xe000;

pub const AR_WOW_PATTERN_SUPPORTED_LEGACY: c_uint = 0xff;
pub const AR_WOW_PATTERN_SUPPORTED: c_uint = 0xffff;
pub const AR_WOW_LENGTH_MAX: c_uint = 0xff;

