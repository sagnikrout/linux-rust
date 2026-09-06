//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192ce/def.h
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
pub const PHY_RSSI_SLID_WIN_MAX: c_int = 100;
pub const PHY_LINKQUALITY_SLID_WIN_MAX: c_int = 20;
pub const PHY_BEACON_RSSI_SLID_WIN_MAX: c_int = 10;
pub const RX_SMOOTH_FACTOR: c_int = 20;
pub const HAL_PRIME_CHNL_OFFSET_DONT_CARE: c_int = 0;
pub const HAL_PRIME_CHNL_OFFSET_LOWER: c_int = 1;
pub const HAL_PRIME_CHNL_OFFSET_UPPER: c_int = 2;
pub const RX_MPDU_QUEUE: c_int = 0;
pub const RX_CMD_QUEUE: c_int = 1;

pub const CHIP_BONDING_92C_1T2R: c_uint = 0x1;

pub const CHIP_92C_1T2R: c_uint = 0x03;
pub const CHIP_92C: c_uint = 0x01;
pub const CHIP_88C: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum version_8192c {
    VERSION_A_CHIP_92C = 0x01,
    VERSION_A_CHIP_88C = 0x00,
    VERSION_B_CHIP_92C = 0x11,
    VERSION_B_CHIP_88C = 0x10,
    VERSION_TEST_CHIP_88C = 0x00,
    VERSION_TEST_CHIP_92C = 0x01,
    VERSION_NORMAL_TSMC_CHIP_88C = 0x10,
    VERSION_NORMAL_TSMC_CHIP_92C = 0x11,
    VERSION_NORMAL_TSMC_CHIP_92C_1T2R = 0x13,
    VERSION_NORMAL_UMC_CHIP_88C_A_CUT = 0x30,
    VERSION_NORMAL_UMC_CHIP_92C_A_CUT = 0x31,
    VERSION_NORMAL_UMC_CHIP_92C_1T2R_A_CUT = 0x33,
    VERSION_NORMA_UMC_CHIP_8723_1T1R_A_CUT = 0x34,
    VERSION_NORMA_UMC_CHIP_8723_1T1R_B_CUT = 0x3c,
    VERSION_NORMAL_UMC_CHIP_88C_B_CUT = 0x70,
    VERSION_NORMAL_UMC_CHIP_92C_B_CUT = 0x71,
    VERSION_NORMAL_UMC_CHIP_92C_1T2R_B_CUT = 0x73,
    VERSION_UNKNOWN = 0x88,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl819x_loopback_e {
    RTL819X_NO_LOOPBACK = 0,
    RTL819X_MAC_LOOPBACK = 1,
    RTL819X_DMA_LOOPBACK = 2,
    RTL819X_CCK_LOOPBACK = 3,
}

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
pub enum power_polocy_config {
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
#[derive(Copy, Clone)]
pub struct phy_sts_cck_8192s_t {
    pub adc_pwdb_X: [u8; 4],
    pub sq_rpt: u8,
    pub cck_agc_rpt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_cmd_8192c {
    pub element_id: u8,
    pub cmd_len: u32,
    pub p_cmdbuffer: *mut u8,
}
