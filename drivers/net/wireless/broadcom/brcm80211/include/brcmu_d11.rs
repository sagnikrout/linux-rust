//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/include/brcmu_d11.h
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
// Copyright (c) 2010 Broadcom Corporation
//
// d11 io type
pub const BRCMU_D11N_IOTYPE: c_int = 1;
pub const BRCMU_D11AC_IOTYPE: c_int = 2;
// A chanspec (channel specification) holds the channel number, band,
// bandwidth and control sideband
//
// chanspec binary format
pub const BRCMU_CHSPEC_INVALID: c_int = 255;
// bit 0~7 channel number
// for 80+80 channels: bit 0~3 low channel id, bit 4~7 high channel id
//
pub const BRCMU_CHSPEC_CH_MASK: c_uint = 0x00ff;
pub const BRCMU_CHSPEC_CH_SHIFT: c_int = 0;
pub const BRCMU_CHSPEC_CHL_MASK: c_uint = 0x000f;
pub const BRCMU_CHSPEC_CHL_SHIFT: c_int = 0;
pub const BRCMU_CHSPEC_CHH_MASK: c_uint = 0x00f0;
pub const BRCMU_CHSPEC_CHH_SHIFT: c_int = 4;
// bit 8~16 for dot 11n IO types
// bit 8~9 sideband
// bit 10~11 bandwidth
// bit 12~13 spectral band
// bit 14~15 not used
//
pub const BRCMU_CHSPEC_D11N_SB_MASK: c_uint = 0x0300;
pub const BRCMU_CHSPEC_D11N_SB_SHIFT: c_int = 8;
pub const BRCMU_CHSPEC_D11N_SB_L: c_uint = 0x0100	/* control lower */;
pub const BRCMU_CHSPEC_D11N_SB_U: c_uint = 0x0200	/* control upper */;
pub const BRCMU_CHSPEC_D11N_SB_N: c_uint = 0x0300	/* none */;
pub const BRCMU_CHSPEC_D11N_BW_MASK: c_uint = 0x0c00;
pub const BRCMU_CHSPEC_D11N_BW_SHIFT: c_int = 10;
pub const BRCMU_CHSPEC_D11N_BW_10: c_uint = 0x0400;
pub const BRCMU_CHSPEC_D11N_BW_20: c_uint = 0x0800;
pub const BRCMU_CHSPEC_D11N_BW_40: c_uint = 0x0c00;
pub const BRCMU_CHSPEC_D11N_BND_MASK: c_uint = 0x3000;
pub const BRCMU_CHSPEC_D11N_BND_SHIFT: c_int = 12;
pub const BRCMU_CHSPEC_D11N_BND_5G: c_uint = 0x1000;
pub const BRCMU_CHSPEC_D11N_BND_2G: c_uint = 0x2000;
// bit 8~16 for dot 11ac IO types
// bit 8~10 sideband
// bit 11~13 bandwidth
// bit 14~15 spectral band
//
pub const BRCMU_CHSPEC_D11AC_SB_MASK: c_uint = 0x0700;
pub const BRCMU_CHSPEC_D11AC_SB_SHIFT: c_int = 8;
pub const BRCMU_CHSPEC_D11AC_SB_LLL: c_uint = 0x0000;
pub const BRCMU_CHSPEC_D11AC_SB_LLU: c_uint = 0x0100;
pub const BRCMU_CHSPEC_D11AC_SB_LUL: c_uint = 0x0200;
pub const BRCMU_CHSPEC_D11AC_SB_LUU: c_uint = 0x0300;
pub const BRCMU_CHSPEC_D11AC_SB_ULL: c_uint = 0x0400;
pub const BRCMU_CHSPEC_D11AC_SB_ULU: c_uint = 0x0500;
pub const BRCMU_CHSPEC_D11AC_SB_UUL: c_uint = 0x0600;
pub const BRCMU_CHSPEC_D11AC_SB_UUU: c_uint = 0x0700;

pub const BRCMU_CHSPEC_D11AC_BW_MASK: c_uint = 0x3800;
pub const BRCMU_CHSPEC_D11AC_BW_SHIFT: c_int = 11;
pub const BRCMU_CHSPEC_D11AC_BW_5: c_uint = 0x0000;
pub const BRCMU_CHSPEC_D11AC_BW_10: c_uint = 0x0800;
pub const BRCMU_CHSPEC_D11AC_BW_20: c_uint = 0x1000;
pub const BRCMU_CHSPEC_D11AC_BW_40: c_uint = 0x1800;
pub const BRCMU_CHSPEC_D11AC_BW_80: c_uint = 0x2000;
pub const BRCMU_CHSPEC_D11AC_BW_160: c_uint = 0x2800;
pub const BRCMU_CHSPEC_D11AC_BW_8080: c_uint = 0x3000;
pub const BRCMU_CHSPEC_D11AC_BND_MASK: c_uint = 0xc000;
pub const BRCMU_CHSPEC_D11AC_BND_SHIFT: c_int = 14;
pub const BRCMU_CHSPEC_D11AC_BND_2G: c_uint = 0x0000;
pub const BRCMU_CHSPEC_D11AC_BND_3G: c_uint = 0x4000;
pub const BRCMU_CHSPEC_D11AC_BND_4G: c_uint = 0x8000;
pub const BRCMU_CHSPEC_D11AC_BND_5G: c_uint = 0xc000;
pub const BRCMU_CHAN_BAND_2G: c_int = 0;
pub const BRCMU_CHAN_BAND_5G: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmu_chan_bw {
    BRCMU_CHAN_BW_20,
    BRCMU_CHAN_BW_40,
    BRCMU_CHAN_BW_80,
    BRCMU_CHAN_BW_80P80,
    BRCMU_CHAN_BW_160,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmu_chan_sb {
    BRCMU_CHAN_SB_NONE = -1,
    BRCMU_CHAN_SB_LLL,
    BRCMU_CHAN_SB_LLU,
    BRCMU_CHAN_SB_LUL,
    BRCMU_CHAN_SB_LUU,
    BRCMU_CHAN_SB_ULL,
    BRCMU_CHAN_SB_ULU,
    BRCMU_CHAN_SB_UUL,
    BRCMU_CHAN_SB_UUU,
    BRCMU_CHAN_SB_L = BRCMU_CHAN_SB_LLL,
    BRCMU_CHAN_SB_U = BRCMU_CHAN_SB_LLU,
    BRCMU_CHAN_SB_LL = BRCMU_CHAN_SB_LLL,
    BRCMU_CHAN_SB_LU = BRCMU_CHAN_SB_LLU,
    BRCMU_CHAN_SB_UL = BRCMU_CHAN_SB_LUL,
    BRCMU_CHAN_SB_UU = BRCMU_CHAN_SB_LUU,
}

//
// struct brcmu_chan - stores channel formats
//
// This structure can be used with functions translating chanspec into generic
// channel info and the other way.
//
// @chspec: firmware specific format
// @chnum: center channel number
// @control_ch_num: control channel number
// @band: frequency band
// @bw: channel width
// @sb: control sideband (location of control channel against the center one)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmu_chan {
    pub chspec: u16,
    pub chnum: u8,
    pub control_ch_num: u8,
    pub band: u8,
    pub bw: brcmu_chan_bw,
    pub sb: brcmu_chan_sb,
}

//
// struct brcmu_d11inf - provides functions translating channel format
//
// @io_type: determines version of channel format used by firmware
// @encchspec: encodes channel info into a chanspec, requires center channel
// number, ignores control one
// @decchspec: decodes chanspec into generic info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmu_d11inf {
    pub io_type: u8,
    pub ch): *mut *mut void (encchspec)(struct brcmu_chan,
    pub ch): *mut *mut void (decchspec)(struct brcmu_chan,
}

extern "C" {
    pub fn brcmu_d11_attach(d11inf: *mut brcmu_d11inf);
}
