//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/mt6351.h
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


// SPDX-License-Identifier: GPL-2.0
//
// mt6351.h  --  mt6351 ALSA SoC audio codec driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//

pub const MT6351_TOP_CKPDN_CON0: c_uint = 0x023A;
pub const MT6351_TOP_CKPDN_CON0_SET: c_uint = 0x023C;
pub const MT6351_TOP_CKPDN_CON0_CLR: c_uint = 0x023E;
pub const MT6351_TOP_CLKSQ: c_uint = 0x029A;
pub const MT6351_TOP_CLKSQ_SET: c_uint = 0x029C;
pub const MT6351_TOP_CLKSQ_CLR: c_uint = 0x029E;
pub const MT6351_ZCD_CON0: c_uint = 0x0800;
pub const MT6351_ZCD_CON1: c_uint = 0x0802;
pub const MT6351_ZCD_CON2: c_uint = 0x0804;
pub const MT6351_ZCD_CON3: c_uint = 0x0806;
pub const MT6351_ZCD_CON4: c_uint = 0x0808;
pub const MT6351_ZCD_CON5: c_uint = 0x080A;
pub const MT6351_LDO_VA18_CON0: c_uint = 0x0A00;
pub const MT6351_LDO_VA18_CON1: c_uint = 0x0A02;
pub const MT6351_LDO_VUSB33_CON0: c_uint = 0x0A16;
pub const MT6351_LDO_VUSB33_CON1: c_uint = 0x0A18;
pub const MT6351_AUDDEC_ANA_CON0: c_uint = 0x0CF2;
pub const MT6351_AUDDEC_ANA_CON1: c_uint = 0x0CF4;
pub const MT6351_AUDDEC_ANA_CON2: c_uint = 0x0CF6;
pub const MT6351_AUDDEC_ANA_CON3: c_uint = 0x0CF8;
pub const MT6351_AUDDEC_ANA_CON4: c_uint = 0x0CFA;
pub const MT6351_AUDDEC_ANA_CON5: c_uint = 0x0CFC;
pub const MT6351_AUDDEC_ANA_CON6: c_uint = 0x0CFE;
pub const MT6351_AUDDEC_ANA_CON7: c_uint = 0x0D00;
pub const MT6351_AUDDEC_ANA_CON8: c_uint = 0x0D02;
pub const MT6351_AUDDEC_ANA_CON9: c_uint = 0x0D04;
pub const MT6351_AUDDEC_ANA_CON10: c_uint = 0x0D06;
pub const MT6351_AUDENC_ANA_CON0: c_uint = 0x0D08;
pub const MT6351_AUDENC_ANA_CON1: c_uint = 0x0D0A;
pub const MT6351_AUDENC_ANA_CON2: c_uint = 0x0D0C;
pub const MT6351_AUDENC_ANA_CON3: c_uint = 0x0D0E;
pub const MT6351_AUDENC_ANA_CON4: c_uint = 0x0D10;
pub const MT6351_AUDENC_ANA_CON5: c_uint = 0x0D12;
pub const MT6351_AUDENC_ANA_CON6: c_uint = 0x0D14;
pub const MT6351_AUDENC_ANA_CON7: c_uint = 0x0D16;
pub const MT6351_AUDENC_ANA_CON8: c_uint = 0x0D18;
pub const MT6351_AUDENC_ANA_CON9: c_uint = 0x0D1A;
pub const MT6351_AUDENC_ANA_CON10: c_uint = 0x0D1C;
pub const MT6351_AUDENC_ANA_CON11: c_uint = 0x0D1E;
pub const MT6351_AUDENC_ANA_CON12: c_uint = 0x0D20;
pub const MT6351_AUDENC_ANA_CON13: c_uint = 0x0D22;
pub const MT6351_AUDENC_ANA_CON14: c_uint = 0x0D24;
pub const MT6351_AUDENC_ANA_CON15: c_uint = 0x0D26;
pub const MT6351_AUDENC_ANA_CON16: c_uint = 0x0D28;
