//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8821ae/def.h
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
// Copyright(c) 2009-2010  Realtek Corporation.
// --------------------------Define -------------------------------------------
pub const USE_SPECIFIC_FW_TO_SUPPORT_WOWLAN: c_int = 1;
// BIT 7 HT Rate
// TxHT = 0
pub const MGN_1M: c_uint = 0x02;
pub const MGN_2M: c_uint = 0x04;
pub const MGN_5_5M: c_uint = 0x0b;
pub const MGN_11M: c_uint = 0x16;
pub const MGN_6M: c_uint = 0x0c;
pub const MGN_9M: c_uint = 0x12;
pub const MGN_12M: c_uint = 0x18;
pub const MGN_18M: c_uint = 0x24;
pub const MGN_24M: c_uint = 0x30;
pub const MGN_36M: c_uint = 0x48;
pub const MGN_48M: c_uint = 0x60;
pub const MGN_54M: c_uint = 0x6c;
// TxHT = 1
pub const MGN_MCS0: c_uint = 0x80;
pub const MGN_MCS1: c_uint = 0x81;
pub const MGN_MCS2: c_uint = 0x82;
pub const MGN_MCS3: c_uint = 0x83;
pub const MGN_MCS4: c_uint = 0x84;
pub const MGN_MCS5: c_uint = 0x85;
pub const MGN_MCS6: c_uint = 0x86;
pub const MGN_MCS7: c_uint = 0x87;
pub const MGN_MCS8: c_uint = 0x88;
pub const MGN_MCS9: c_uint = 0x89;
pub const MGN_MCS10: c_uint = 0x8a;
pub const MGN_MCS11: c_uint = 0x8b;
pub const MGN_MCS12: c_uint = 0x8c;
pub const MGN_MCS13: c_uint = 0x8d;
pub const MGN_MCS14: c_uint = 0x8e;
pub const MGN_MCS15: c_uint = 0x8f;
// VHT rate
pub const MGN_VHT1SS_MCS0: c_uint = 0x90;
pub const MGN_VHT1SS_MCS1: c_uint = 0x91;
pub const MGN_VHT1SS_MCS2: c_uint = 0x92;
pub const MGN_VHT1SS_MCS3: c_uint = 0x93;
pub const MGN_VHT1SS_MCS4: c_uint = 0x94;
pub const MGN_VHT1SS_MCS5: c_uint = 0x95;
pub const MGN_VHT1SS_MCS6: c_uint = 0x96;
pub const MGN_VHT1SS_MCS7: c_uint = 0x97;
pub const MGN_VHT1SS_MCS8: c_uint = 0x98;
pub const MGN_VHT1SS_MCS9: c_uint = 0x99;
pub const MGN_VHT2SS_MCS0: c_uint = 0x9a;
pub const MGN_VHT2SS_MCS1: c_uint = 0x9b;
pub const MGN_VHT2SS_MCS2: c_uint = 0x9c;
pub const MGN_VHT2SS_MCS3: c_uint = 0x9d;
pub const MGN_VHT2SS_MCS4: c_uint = 0x9e;
pub const MGN_VHT2SS_MCS5: c_uint = 0x9f;
pub const MGN_VHT2SS_MCS6: c_uint = 0xa0;
pub const MGN_VHT2SS_MCS7: c_uint = 0xa1;
pub const MGN_VHT2SS_MCS8: c_uint = 0xa2;
pub const MGN_VHT2SS_MCS9: c_uint = 0xa3;
pub const MGN_VHT3SS_MCS0: c_uint = 0xa4;
pub const MGN_VHT3SS_MCS1: c_uint = 0xa5;
pub const MGN_VHT3SS_MCS2: c_uint = 0xa6;
pub const MGN_VHT3SS_MCS3: c_uint = 0xa7;
pub const MGN_VHT3SS_MCS4: c_uint = 0xa8;
pub const MGN_VHT3SS_MCS5: c_uint = 0xa9;
pub const MGN_VHT3SS_MCS6: c_uint = 0xaa;
pub const MGN_VHT3SS_MCS7: c_uint = 0xab;
pub const MGN_VHT3SS_MCS8: c_uint = 0xac;
pub const MGN_VHT3SS_MCS9: c_uint = 0xad;
pub const MGN_MCS0_SG: c_uint = 0xc0;
pub const MGN_MCS1_SG: c_uint = 0xc1;
pub const MGN_MCS2_SG: c_uint = 0xc2;
pub const MGN_MCS3_SG: c_uint = 0xc3;
pub const MGN_MCS4_SG: c_uint = 0xc4;
pub const MGN_MCS5_SG: c_uint = 0xc5;
pub const MGN_MCS6_SG: c_uint = 0xc6;
pub const MGN_MCS7_SG: c_uint = 0xc7;
pub const MGN_MCS8_SG: c_uint = 0xc8;
pub const MGN_MCS9_SG: c_uint = 0xc9;
pub const MGN_MCS10_SG: c_uint = 0xca;
pub const MGN_MCS11_SG: c_uint = 0xcb;
pub const MGN_MCS12_SG: c_uint = 0xcc;
pub const MGN_MCS13_SG: c_uint = 0xcd;
pub const MGN_MCS14_SG: c_uint = 0xce;
pub const MGN_MCS15_SG: c_uint = 0xcf;
pub const MGN_UNKNOWN: c_uint = 0xff;
// 30 ms
pub const WIFI_NAV_UPPER_US: c_int = 30000;
pub const HAL_92C_NAV_UPPER_UNIT: c_int = 128;
pub const MAX_RX_DMA_BUFFER_SIZE: c_uint = 0x3E80;
pub const HAL_PRIME_CHNL_OFFSET_DONT_CARE: c_int = 0;
pub const HAL_PRIME_CHNL_OFFSET_LOWER: c_int = 1;
pub const HAL_PRIME_CHNL_OFFSET_UPPER: c_int = 2;
pub const RX_MPDU_QUEUE: c_int = 0;
pub const RX_CMD_QUEUE: c_int = 1;
pub const MAX_RX_DMA_BUFFER_SIZE_8812: c_uint = 0x3E80;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum version_8821ae {
    VERSION_TEST_CHIP_1T1R_8812 = 0x0004,
    VERSION_TEST_CHIP_2T2R_8812 = 0x0024,
    VERSION_NORMAL_TSMC_CHIP_1T1R_8812 = 0x100c,
    VERSION_NORMAL_TSMC_CHIP_2T2R_8812 = 0x102c,
    VERSION_NORMAL_TSMC_CHIP_1T1R_8812_C_CUT = 0x200c,
    VERSION_NORMAL_TSMC_CHIP_2T2R_8812_C_CUT = 0x202c,
    VERSION_TEST_CHIP_8821 = 0x0005,
    VERSION_NORMAL_TSMC_CHIP_8821 = 0x000d,
    VERSION_NORMAL_TSMC_CHIP_8821_B_CUT = 0x100d,
    VERSION_UNKNOWN = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vht_data_sc {
    VHT_DATA_SC_DONOT_CARE = 0,
    VHT_DATA_SC_20_UPPER_OF_80MHZ = 1,
    VHT_DATA_SC_20_LOWER_OF_80MHZ = 2,
    VHT_DATA_SC_20_UPPERST_OF_80MHZ = 3,
    VHT_DATA_SC_20_LOWEST_OF_80MHZ = 4,
    VHT_DATA_SC_20_RECV1 = 5,
    VHT_DATA_SC_20_RECV2 = 6,
    VHT_DATA_SC_20_RECV3 = 7,
    VHT_DATA_SC_20_RECV4 = 8,
    VHT_DATA_SC_40_UPPER_OF_80MHZ = 9,
    VHT_DATA_SC_40_LOWER_OF_80MHZ = 10,
}

// MASK

// Get element

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum board_type {
    ODM_BOARD_DEFAULT = 0,	  /* The DEFAULT case. */
    ODM_BOARD_MINICARD = BIT(0), /* 0 = non-mini card, 1 = mini card. */
    ODM_BOARD_SLIM = BIT(1), /* 0 = non-slim card, 1 = slim card */
    ODM_BOARD_BT = BIT(2), /* 0 = without BT card, 1 = with BT */
    ODM_BOARD_EXT_PA = BIT(3), /* 1 = existing 2G ext-PA */
    ODM_BOARD_EXT_LNA = BIT(4), /* 1 = existing 2G ext-LNA */
    ODM_BOARD_EXT_TRSW = BIT(5), /* 1 = existing ext-TRSW */
    ODM_BOARD_EXT_PA_5G = BIT(6), /* 1 = existing 5G ext-PA */
    ODM_BOARD_EXT_LNA_5G = BIT(7), /* 1 = existing 5G ext-LNA */
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
pub struct phy_sts_cck_8821ae_t {
    pub adc_pwdb_X: [u8; 4],
    pub sq_rpt: u8,
    pub cck_agc_rpt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_cmd_8821ae {
    pub element_id: u8,
    pub cmd_len: u32,
    pub p_cmdbuffer: *mut u8,
}
