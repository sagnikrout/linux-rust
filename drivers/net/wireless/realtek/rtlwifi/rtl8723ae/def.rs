//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723ae/def.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const HAL_PRIME_CHNL_OFFSET_DONT_CARE: c_int = 0;
pub const HAL_PRIME_CHNL_OFFSET_LOWER: c_int = 1;
pub const HAL_PRIME_CHNL_OFFSET_UPPER: c_int = 2;
pub const RX_MPDU_QUEUE: c_int = 0;
pub const RX_CMD_QUEUE: c_int = 1;

pub const CHIP_BONDING_92C_1T2R: c_uint = 0x1;

// MASK

// Get element

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_optype {
    RF_OP_BY_SW_3WIRE = 0,
    RF_OP_BY_FW,
    RF_OP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_power_state {
    RF_ON,
    RF_OFF,
    RF_SLEEP,
    RF_SHUT_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_save_mode {
    POWER_SAVE_MODE_ACTIVE,
    POWER_SAVE_MODE_SAVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_policy_config {
    POWERCFG_MAX_POWER_SAVINGS,
    POWERCFG_GLOBAL_POWER_SAVINGS,
    POWERCFG_LOCAL_POWER_SAVINGS,
    POWERCFG_LENOVO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum interface_select_pci {
    INTF_SEL1_MINICARD = 0,
    INTF_SEL0_PCIE = 1,
    INTF_SEL2_RSV = 2,
    INTF_SEL3_RSV = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_desc_qsel {
    QSLT_BK = 0x2,
    QSLT_BE = 0x0,
    QSLT_VI = 0x5,
    QSLT_VO = 0x7,
    QSLT_BEACON = 0x10,
    QSLT_HIGH = 0x11,
    QSLT_MGNT = 0x12,
    QSLT_CMD = 0x13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_desc8723e_rate {
    DESC92C_RATE1M = 0x00,
    DESC92C_RATE2M = 0x01,
    DESC92C_RATE5_5M = 0x02,
    DESC92C_RATE11M = 0x03,

    DESC92C_RATE6M = 0x04,
    DESC92C_RATE9M = 0x05,
    DESC92C_RATE12M = 0x06,
    DESC92C_RATE18M = 0x07,
    DESC92C_RATE24M = 0x08,
    DESC92C_RATE36M = 0x09,
    DESC92C_RATE48M = 0x0a,
    DESC92C_RATE54M = 0x0b,

    DESC92C_RATEMCS0 = 0x0c,
    DESC92C_RATEMCS1 = 0x0d,
    DESC92C_RATEMCS2 = 0x0e,
    DESC92C_RATEMCS3 = 0x0f,
    DESC92C_RATEMCS4 = 0x10,
    DESC92C_RATEMCS5 = 0x11,
    DESC92C_RATEMCS6 = 0x12,
    DESC92C_RATEMCS7 = 0x13,
    DESC92C_RATEMCS8 = 0x14,
    DESC92C_RATEMCS9 = 0x15,
    DESC92C_RATEMCS10 = 0x16,
    DESC92C_RATEMCS11 = 0x17,
    DESC92C_RATEMCS12 = 0x18,
    DESC92C_RATEMCS13 = 0x19,
    DESC92C_RATEMCS14 = 0x1a,
    DESC92C_RATEMCS15 = 0x1b,
    DESC92C_RATEMCS15_SG = 0x1c,
    DESC92C_RATEMCS32 = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_sts_cck_8723e_t {
    pub adc_pwdb_X: [u8; 4],
    pub sq_rpt: u8,
    pub cck_agc_rpt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_cmd_8723e {
    pub element_id: u8,
    pub cmd_len: u32,
    pub p_cmdbuffer: *mut u8,
}
