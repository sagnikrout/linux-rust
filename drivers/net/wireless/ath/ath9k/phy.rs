//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/phy.h
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
// Copyright (c) 2008-2011 Atheros Communications Inc.
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
pub const CHANSEL_DIV: c_int = 15;

pub const AR_PHY_BASE: c_uint = 0x9800;

pub const AR_PHY_TX_PWRCTRL_TX_GAIN_TAB_MAX: c_uint = 0x0007E000;
pub const AR_PHY_TX_PWRCTRL_TX_GAIN_TAB_MAX_S: c_int = 13;
pub const AR_PHY_TX_GAIN_CLC: c_uint = 0x0000001E;
pub const AR_PHY_TX_GAIN_CLC_S: c_int = 1;
pub const AR_PHY_TX_GAIN: c_uint = 0x0007F000;
pub const AR_PHY_TX_GAIN_S: c_int = 12;
pub const AR_PHY_CLC_TBL1: c_uint = 0xa35c;
pub const AR_PHY_CLC_I0: c_uint = 0x07ff0000;
pub const AR_PHY_CLC_I0_S: c_int = 16;
pub const AR_PHY_CLC_Q0: c_uint = 0x0000ffd0;
pub const AR_PHY_CLC_Q0_S: c_int = 5;
pub const ANTSWAP_AB: c_uint = 0x0001;
pub const REDUCE_CHAIN_0: c_uint = 0x00000050;
pub const REDUCE_CHAIN_1: c_uint = 0x00000051;
pub const AR_PHY_CHIP_ID: c_uint = 0x9818;
pub const AR_PHY_TIMING11_SPUR_FREQ_SD: c_uint = 0x3FF00000;
pub const AR_PHY_TIMING11_SPUR_FREQ_SD_S: c_int = 20;
pub const AR_PHY_PLL_CONTROL: c_uint = 0x16180;
pub const AR_PHY_PLL_MODE: c_uint = 0x16184;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_ant_div_comb_lna_conf {
    ATH_ANT_DIV_COMB_LNA1_MINUS_LNA2,
    ATH_ANT_DIV_COMB_LNA2,
    ATH_ANT_DIV_COMB_LNA1,
    ATH_ANT_DIV_COMB_LNA1_PLUS_LNA2,
}
