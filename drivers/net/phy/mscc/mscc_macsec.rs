//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mscc/mscc_macsec.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Driver for Microsemi VSC85xx PHYs
//
// Copyright (c) 2020 Microsemi Corporation
//

pub const MSCC_MS_MAX_FLOWS: c_int = 16;
pub const CONTROL_TYPE_EGRESS: c_uint = 0x6;
pub const CONTROL_TYPE_INGRESS: c_uint = 0xf;

pub const CTRYPTO_ALG_AES_CTR_128: c_uint = 0x5;
pub const CTRYPTO_ALG_AES_CTR_192: c_uint = 0x6;
pub const CTRYPTO_ALG_AES_CTR_256: c_uint = 0x7;

pub const AUTH_ALG_AES_GHAS: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mscc_macsec_destination_ports {
    MSCC_MS_PORT_COMMON		= 0,
    MSCC_MS_PORT_RSVD		= 1,
    MSCC_MS_PORT_CONTROLLED		= 2,
    MSCC_MS_PORT_UNCONTROLLED	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mscc_macsec_drop_actions {
    MSCC_MS_ACTION_BYPASS_CRC	= 0,
    MSCC_MS_ACTION_BYPASS_BAD	= 1,
    MSCC_MS_ACTION_DROP		= 2,
    MSCC_MS_ACTION_BYPASS		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mscc_macsec_flow_types {
    MSCC_MS_FLOW_BYPASS		= 0,
    MSCC_MS_FLOW_DROP		= 1,
    MSCC_MS_FLOW_INGRESS		= 2,
    MSCC_MS_FLOW_EGRESS		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mscc_macsec_validate_levels {
    MSCC_MS_VALIDATE_DISABLED	= 0,
    MSCC_MS_VALIDATE_CHECK		= 1,
    MSCC_MS_VALIDATE_STRICT		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_bank {
    FC_BUFFER   = 0x04,
    HOST_MAC    = 0x05,
    LINE_MAC    = 0x06,
    PROC_0      = 0x0e,
    PROC_2      = 0x0f,
    MACSEC_INGR = 0x38,
    MACSEC_EGR  = 0x3c,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_flow {
    pub list: list_head,
    pub port: mscc_macsec_destination_ports,
    pub bank: macsec_bank,
    pub index: u32,
    pub assoc_num: c_int,
    pub has_transformation: bool,
// Highest takes precedence [0..15]
    pub priority: u8,
    pub rx_sa: *mut macsec_rx_sa,
    pub tx_sa: *mut macsec_tx_sa,
}

// Matching
// Action
pub const MSCC_EXT_PAGE_MACSEC_17: c_int = 17;
pub const MSCC_EXT_PAGE_MACSEC_18: c_int = 18;
pub const MSCC_EXT_PAGE_MACSEC_19: c_int = 19;

pub const MSCC_EXT_PAGE_MACSEC_20: c_int = 20;

pub const MSCC_MS_ENA_CFG: c_uint = 0x800;
pub const MSCC_MS_FC_CFG: c_uint = 0x804;

pub const MSCC_MS_SAM_ENTRY_SET1: c_uint = 0x1808;
pub const MSCC_MS_SAM_ENTRY_CLEAR1: c_uint = 0x180c;

pub const MSCC_MS_SAM_CP_TAG: c_uint = 0x1e40;
pub const MSCC_MS_SAM_NM_FLOW_NCP: c_uint = 0x1e51;
pub const MSCC_MS_SAM_NM_FLOW_CP: c_uint = 0x1e52;
pub const MSCC_MS_MISC_CONTROL: c_uint = 0x1e5f;
pub const MSCC_MS_COUNT_CONTROL: c_uint = 0x3204;
pub const MSCC_MS_PARAMS2_IG_CC_CONTROL: c_uint = 0x3a10;
pub const MSCC_MS_PARAMS2_IG_CP_TAG: c_uint = 0x3a14;

pub const MSCC_MS_NON_VLAN_MTU_CHECK: c_uint = 0x3c48;
pub const MSCC_MS_PP_CTRL: c_uint = 0x3c4b;
pub const MSCC_MS_STATUS_CONTEXT_CTRL: c_uint = 0x3d02;
pub const MSCC_MS_INTR_CTRL_STATUS: c_uint = 0x3d04;
pub const MSCC_MS_BLOCK_CTX_UPDATE: c_uint = 0x3d0c;
pub const MSCC_MS_AIC_CTRL: c_uint = 0x3e02;
// MACSEC_ENA_CFG

// MACSEC_FC_CFG

// MSCC_MS_SAM_MAC_SA_MATCH_HI

// MACSEC_SAM_MISC_MATCH

// MACSEC_SAM_MASK

// MACSEC_SAM_FLOW_CTRL_EGR

// MACSEC_SAM_CP_TAG

// MACSEC_SAM_NM_FLOW_NCP

// MACSEC_SAM_NM_FLOW_CP

// MACSEC_MISC_CONTROL

// MACSEC_COUNT_CONTROL

// MACSEC_PARAMS2_IG_CC_CONTROL

// MACSEC_PARAMS2_IG_CP_TAG

// MACSEC_VLAN_MTU_CHECK

// MACSEC_NON_VLAN_MTU_CHECK

// MACSEC_PP_CTRL

// MACSEC_INTR_CTRL_STATUS

