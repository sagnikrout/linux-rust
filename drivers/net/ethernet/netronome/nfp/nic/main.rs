//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nic/main.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2023 Corigine, Inc.
pub const __NFP_NIC_H__: c_int = 1;

// DCB feature definitions
pub const NFP_NET_MAX_DSCP: c_int = 64;

pub const NFP_NET_MAX_PRIO: c_int = 8;
pub const NFP_DCB_CFG_STRIDE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_dcb {
    pub dscp2prio: [u8; NFP_NET_MAX_DSCP],
    pub prio2tc: [u8; NFP_NET_MAX_PRIO],
    pub tc2idx: [u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_maxrate: [u64; IEEE_8021QAZ_MAX_TCS],
    pub tc_tx_pct: [u8; IEEE_8021QAZ_MAX_TCS],
    pub tc_tsa: [u8; IEEE_8021QAZ_MAX_TCS],
    pub dscp_cnt: u8,
    pub trust_status: u8,
    pub rate_init: bool,
    pub ets_init: bool,
    pub dcbcfg_tbl_area: *mut nfp_cpp_area,
    pub dcbcfg_tbl: *mut u8 __iomem,
    pub cfg_offset: u32,
}

extern "C" {
    pub fn nfp_nic_dcb_init(nn: *mut nfp_net) -> c_int;
}
extern "C" {
    pub fn nfp_nic_dcb_clean(nn: *mut nfp_net);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_app_nic_private {

    pub dcb: nfp_dcb,

}
