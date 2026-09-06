//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192d/def.h
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
// Min Spacing related settings.
pub const MAX_MSS_DENSITY_2T: c_uint = 0x13;
pub const MAX_MSS_DENSITY_1T: c_uint = 0x0A;
pub const RF6052_MAX_TX_PWR: c_uint = 0x3F;
pub const RF6052_MAX_PATH: c_int = 2;
pub const PHY_RSSI_SLID_WIN_MAX: c_int = 100;
pub const PHY_LINKQUALITY_SLID_WIN_MAX: c_int = 20;
pub const PHY_BEACON_RSSI_SLID_WIN_MAX: c_int = 10;

pub const RX_SMOOTH_FACTOR: c_int = 20;
pub const HAL_PRIME_CHNL_OFFSET_DONT_CARE: c_int = 0;
pub const HAL_PRIME_CHNL_OFFSET_LOWER: c_int = 1;
pub const HAL_PRIME_CHNL_OFFSET_UPPER: c_int = 2;
pub const RX_MPDU_QUEUE: c_int = 0;
pub const RX_CMD_QUEUE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum version_8192d {
    VERSION_TEST_CHIP_88C = 0x0000,
    VERSION_TEST_CHIP_92C = 0x0020,
    VERSION_TEST_UMC_CHIP_8723 = 0x0081,
    VERSION_NORMAL_TSMC_CHIP_88C = 0x0008,
    VERSION_NORMAL_TSMC_CHIP_92C = 0x0028,
    VERSION_NORMAL_TSMC_CHIP_92C_1T2R = 0x0018,
    VERSION_NORMAL_UMC_CHIP_88C_A_CUT = 0x0088,
    VERSION_NORMAL_UMC_CHIP_92C_A_CUT = 0x00a8,
    VERSION_NORMAL_UMC_CHIP_92C_1T2R_A_CUT = 0x0098,
    VERSION_NORMAL_UMC_CHIP_8723_1T1R_A_CUT = 0x0089,
    VERSION_NORMAL_UMC_CHIP_8723_1T1R_B_CUT = 0x1089,
    VERSION_NORMAL_UMC_CHIP_88C_B_CUT = 0x1088,
    VERSION_NORMAL_UMC_CHIP_92C_B_CUT = 0x10a8,
    VERSION_NORMAL_UMC_CHIP_92C_1T2R_B_CUT = 0x1090,
    VERSION_TEST_CHIP_92D_SINGLEPHY = 0x0022,
    VERSION_TEST_CHIP_92D_DUALPHY = 0x0002,
    VERSION_NORMAL_CHIP_92D_SINGLEPHY = 0x002a,
    VERSION_NORMAL_CHIP_92D_DUALPHY = 0x000a,
    VERSION_NORMAL_CHIP_92D_C_CUT_SINGLEPHY = 0x202a,
    VERSION_NORMAL_CHIP_92D_C_CUT_DUALPHY = 0x200a,
    VERSION_NORMAL_CHIP_92D_D_CUT_SINGLEPHY = 0x302a,
    VERSION_NORMAL_CHIP_92D_D_CUT_DUALPHY = 0x300a,
    VERSION_NORMAL_CHIP_92D_E_CUT_SINGLEPHY = 0x402a,
    VERSION_NORMAL_CHIP_92D_E_CUT_DUALPHY = 0x400a,
}

// for 92D

// Chip specific

pub const CHIP_BONDING_92C_1T2R: c_uint = 0x1;
pub const CHIP_BONDING_88C_USB_MCARD: c_uint = 0x2;
pub const CHIP_BONDING_88C_USB_HP: c_uint = 0x1;
// [15:12] IC version(CUT): A-cut=0, B-cut=1, C-cut=2, D-cut=3
// [7] Manufacturer: TSMC=0, UMC=1
// [6:4] RF type: 1T1R=0, 1T2R=1, 2T2R=2
// [3] Chip type: TEST=0, NORMAL=1
// [2:0] IC type: 81xxC=0, 8723=1, 92D=2

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
pub enum channel_plan {
    CHPL_FCC	= 0,
    CHPL_IC		= 1,
    CHPL_ETSI	= 2,
    CHPL_SPAIN	= 3,
    CHPL_FRANCE	= 4,
    CHPL_MKK	= 5,
    CHPL_MKK1	= 6,
    CHPL_ISRAEL	= 7,
    CHPL_TELEC	= 8,
    CHPL_GLOBAL	= 9,
    CHPL_WORLD	= 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_sts_cck_8192d {
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txpower_info {
    pub cck_index: [u8; RF6052_MAX_PATH][CHANNEL_GROUP_MAX],
    pub ht40_1sindex: [u8; RF6052_MAX_PATH][CHANNEL_GROUP_MAX],
    pub ht40_2sindexdiff: [u8; RF6052_MAX_PATH][CHANNEL_GROUP_MAX],
    pub ht20indexdiff: [u8; RF6052_MAX_PATH][CHANNEL_GROUP_MAX],
    pub ofdmindexdiff: [u8; RF6052_MAX_PATH][CHANNEL_GROUP_MAX],
    pub ht40maxoffset: [u8; RF6052_MAX_PATH][CHANNEL_GROUP_MAX],
    pub ht20maxoffset: [u8; RF6052_MAX_PATH][CHANNEL_GROUP_MAX],
    pub /: *mut *mut u8 tssi_a[3]; / 5GL/5GM/5GH,
    pub tssi_b: [u8; 3],
}
