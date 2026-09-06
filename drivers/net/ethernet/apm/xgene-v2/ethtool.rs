//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene-v2/ethtool.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Applied Micro X-Gene SoC Ethernet v2 Driver
//
// Copyright (c) 2017, Applied Micro Circuits Corporation
// Author(s): Iyappan Subramanian <isubramanian@apm.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xge_gstrings_stats {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xge_gstrings_extd_stats {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub addr: u32,
    pub value: u32,
}

pub const TR64: c_uint = 0xa080;
pub const TR127: c_uint = 0xa084;
pub const TR255: c_uint = 0xa088;
pub const TR511: c_uint = 0xa08c;
pub const TR1K: c_uint = 0xa090;
pub const TRMAX: c_uint = 0xa094;
pub const TRMGV: c_uint = 0xa098;
pub const RFCS: c_uint = 0xa0a4;
pub const RMCA: c_uint = 0xa0a8;
pub const RBCA: c_uint = 0xa0ac;
pub const RXCF: c_uint = 0xa0b0;
pub const RXPF: c_uint = 0xa0b4;
pub const RXUO: c_uint = 0xa0b8;
pub const RALN: c_uint = 0xa0bc;
pub const RFLR: c_uint = 0xa0c0;
pub const RCDE: c_uint = 0xa0c4;
pub const RCSE: c_uint = 0xa0c8;
pub const RUND: c_uint = 0xa0cc;
pub const ROVR: c_uint = 0xa0d0;
pub const RFRG: c_uint = 0xa0d4;
pub const RJBR: c_uint = 0xa0d8;
pub const RDRP: c_uint = 0xa0dc;
pub const TMCA: c_uint = 0xa0e8;
pub const TBCA: c_uint = 0xa0ec;
pub const TXPF: c_uint = 0xa0f0;
pub const TDFR: c_uint = 0xa0f4;
pub const TEDF: c_uint = 0xa0f8;
pub const TSCL: c_uint = 0xa0fc;
pub const TMCL: c_uint = 0xa100;
pub const TLCL: c_uint = 0xa104;
pub const TXCL: c_uint = 0xa108;
pub const TNCL: c_uint = 0xa10c;
pub const TPFH: c_uint = 0xa110;
pub const TDRP: c_uint = 0xa114;
pub const TJBR: c_uint = 0xa118;
pub const TFCS: c_uint = 0xa11c;
pub const TXCF: c_uint = 0xa120;
pub const TOVR: c_uint = 0xa124;
pub const TUND: c_uint = 0xa128;
pub const TFRG: c_uint = 0xa12c;
extern "C" {
    pub fn xge_set_ethtool_ops(ndev: *mut net_device);
}
