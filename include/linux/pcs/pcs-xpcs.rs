//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pcs/pcs-xpcs.h
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
// Copyright (c) 2020 Synopsys, Inc. and/or its affiliates.
// Synopsys DesignWare XPCS helpers
//

// AN mode
pub const DW_AN_C73: c_int = 1;
pub const DW_AN_C37_SGMII: c_int = 2;
pub const DW_2500BASEX: c_int = 3;
pub const DW_AN_C37_1000BASEX: c_int = 4;
pub const DW_10GBASER: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_xpcs_pcs_id {
    DW_XPCS_ID_NATIVE = 0,
    NXP_SJA1105_XPCS_ID = 0x00000010,
    NXP_SJA1110_XPCS_ID = 0x00000020,
    DW_XPCS_ID = 0x7996ced0,
    DW_XPCS_ID_MASK = 0xffffffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_xpcs_pma_id {
    DW_XPCS_PMA_ID_NATIVE = 0,
    DW_XPCS_PMA_GEN1_3G_ID,
    DW_XPCS_PMA_GEN2_3G_ID,
    DW_XPCS_PMA_GEN2_6G_ID,
    DW_XPCS_PMA_GEN4_3G_ID,
    DW_XPCS_PMA_GEN4_6G_ID,
    DW_XPCS_PMA_GEN5_10G_ID,
    DW_XPCS_PMA_GEN5_12G_ID,
    WX_TXGBE_XPCS_PMA_10G_ID = 0xfc806000,
// Meta Platforms OUI 88:25:08, model 0, revision 0
    MP_FBNIC_XPCS_PMA_100G_ID = 0x46904000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_xpcs_info {
    pub pcs: u32,
    pub pma: u32,
}

extern "C" {
    pub fn xpcs_get_an_mode(xpcs: *mut dw_xpcs, interface: phy_interface_t) -> c_int;
}
extern "C" {
    pub fn xpcs_config_eee_mult_fact(xpcs: *mut dw_xpcs, mult_fact: u8);
}
extern "C" {
    pub fn xpcs_destroy(xpcs: *mut dw_xpcs);
}
extern "C" {
    pub fn xpcs_destroy_pcs(pcs: *mut phylink_pcs);
}
