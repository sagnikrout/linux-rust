//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/stmmac_pcs.h
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
// stmmac_pcs.h: Physical Coding Sublayer Header File
//
// Copyright (C) 2016 STMicroelectronics (R&D) Limited
// Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
//

// PCS registers (AN/TBI/SGMII/RGMII) offsets

// AN Configuration defines

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_pcs_info {
    pub pcs_offset: c_uint,
    pub rgsmii_offset: c_uint,
    pub rgsmii_status_mask: u32,
    pub int_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_pcs {
    pub priv: *mut stmmac_priv,
    pub base: *mut void __iomem,
    pub rgsmii: *mut void __iomem,
    pub rgsmii_status_mask: u32,
    pub int_mask: u32,
    pub pcs: phylink_pcs,
    pub support_tbi_rtbi: bool,
}

extern "C" {
    pub fn container_of(_arg: pcs, stmmac_pcs: struct, _arg: pcs) -> return;
}
//
// dwmac_ctrl_ane - To program the AN Control Register.
// @ioaddr: IO registers pointer
// @reg: Base address of the AN Control Register.
// @ane: to enable the auto-negotiation
// @srgmi_ral: to manage MAC-2-MAC SGMII connections.
// Description: this is the main function to configure the AN control register
// and init the ANE, select loopback (usually for debugging purpose) and
// configure SGMII RAL.
//
// Enable and restart the Auto-Negotiation
// In case of MAC-2-MAC connection, block is configured to operate
// according to MAC conf register.
//
