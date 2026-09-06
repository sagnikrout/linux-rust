//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_phy.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.

pub const IXGBE_I2C_EEPROM_DEV_ADDR: c_uint = 0xA0;
pub const IXGBE_I2C_EEPROM_DEV_ADDR2: c_uint = 0xA2;
// EEPROM byte offsets
pub const IXGBE_SFF_IDENTIFIER: c_uint = 0x0;
pub const IXGBE_SFF_IDENTIFIER_SFP: c_uint = 0x3;
pub const IXGBE_SFF_VENDOR_OUI_BYTE0: c_uint = 0x25;
pub const IXGBE_SFF_VENDOR_OUI_BYTE1: c_uint = 0x26;
pub const IXGBE_SFF_VENDOR_OUI_BYTE2: c_uint = 0x27;
pub const IXGBE_SFF_1GBE_COMP_CODES: c_uint = 0x6;
pub const IXGBE_SFF_10GBE_COMP_CODES: c_uint = 0x3;
pub const IXGBE_SFF_CABLE_TECHNOLOGY: c_uint = 0x8;
pub const IXGBE_SFF_BITRATE_NOMINAL: c_uint = 0xC;
pub const IXGBE_SFF_CABLE_SPEC_COMP: c_uint = 0x3C;
pub const IXGBE_SFF_SFF_8472_SWAP: c_uint = 0x5C;
pub const IXGBE_SFF_SFF_8472_COMP: c_uint = 0x5E;
pub const IXGBE_SFF_SFF_8472_OSCB: c_uint = 0x6E;
pub const IXGBE_SFF_SFF_8472_ESCB: c_uint = 0x76;
pub const IXGBE_SFF_IDENTIFIER_QSFP_PLUS: c_uint = 0xD;
pub const IXGBE_SFF_QSFP_VENDOR_OUI_BYTE0: c_uint = 0xA5;
pub const IXGBE_SFF_QSFP_VENDOR_OUI_BYTE1: c_uint = 0xA6;
pub const IXGBE_SFF_QSFP_VENDOR_OUI_BYTE2: c_uint = 0xA7;
pub const IXGBE_SFF_QSFP_CONNECTOR: c_uint = 0x82;
pub const IXGBE_SFF_QSFP_10GBE_COMP: c_uint = 0x83;
pub const IXGBE_SFF_QSFP_1GBE_COMP: c_uint = 0x86;
pub const IXGBE_SFF_QSFP_CABLE_LENGTH: c_uint = 0x92;
pub const IXGBE_SFF_QSFP_DEVICE_TECH: c_uint = 0x93;
pub const IXGBE_SFF_SM_LENGTH_KM: c_uint = 0xE;
pub const IXGBE_SFF_SM_LENGTH_100M: c_uint = 0xF;
// Bitmasks
pub const IXGBE_SFF_DA_PASSIVE_CABLE: c_uint = 0x4;
pub const IXGBE_SFF_DA_ACTIVE_CABLE: c_uint = 0x8;
pub const IXGBE_SFF_DA_SPEC_ACTIVE_LIMITING: c_uint = 0x4;
pub const IXGBE_SFF_1GBASESX_CAPABLE: c_uint = 0x1;
pub const IXGBE_SFF_1GBASELX_CAPABLE: c_uint = 0x2;
pub const IXGBE_SFF_1GBASET_CAPABLE: c_uint = 0x8;
pub const IXGBE_SFF_BASEBX10_CAPABLE: c_uint = 0x40;
pub const IXGBE_SFF_10GBASESR_CAPABLE: c_uint = 0x10;
pub const IXGBE_SFF_10GBASELR_CAPABLE: c_uint = 0x20;
pub const IXGBE_SFF_SOFT_RS_SELECT_MASK: c_uint = 0x8;
pub const IXGBE_SFF_SOFT_RS_SELECT_10G: c_uint = 0x8;
pub const IXGBE_SFF_SOFT_RS_SELECT_1G: c_uint = 0x0;
pub const IXGBE_SFF_ADDRESSING_MODE: c_uint = 0x4;
pub const IXGBE_SFF_DDM_IMPLEMENTED: c_uint = 0x40;
pub const IXGBE_SFF_QSFP_DA_ACTIVE_CABLE: c_uint = 0x1;
pub const IXGBE_SFF_QSFP_DA_PASSIVE_CABLE: c_uint = 0x8;
pub const IXGBE_SFF_QSFP_CONNECTOR_NOT_SEPARABLE: c_uint = 0x23;
pub const IXGBE_SFF_QSFP_TRANSMITER_850NM_VCSEL: c_uint = 0x0;
pub const IXGBE_I2C_EEPROM_READ_MASK: c_uint = 0x100;
pub const IXGBE_I2C_EEPROM_STATUS_MASK: c_uint = 0x3;
pub const IXGBE_I2C_EEPROM_STATUS_NO_OPERATION: c_uint = 0x0;
pub const IXGBE_I2C_EEPROM_STATUS_PASS: c_uint = 0x1;
pub const IXGBE_I2C_EEPROM_STATUS_FAIL: c_uint = 0x2;
pub const IXGBE_I2C_EEPROM_STATUS_IN_PROGRESS: c_uint = 0x3;
pub const IXGBE_CS4227: c_uint = 0xBE    /* CS4227 address */;
pub const IXGBE_CS4227_GLOBAL_ID_LSB: c_int = 0;
pub const IXGBE_CS4227_GLOBAL_ID_MSB: c_int = 1;
pub const IXGBE_CS4227_SCRATCH: c_int = 2;
pub const IXGBE_CS4227_EFUSE_PDF_SKU: c_uint = 0x19F;
pub const IXGBE_CS4223_SKU_ID: c_uint = 0x0010  /* Quad port */;
pub const IXGBE_CS4227_SKU_ID: c_uint = 0x0014  /* Dual port */;
pub const IXGBE_CS4227_RESET_PENDING: c_uint = 0x1357;
pub const IXGBE_CS4227_RESET_COMPLETE: c_uint = 0x5AA5;
pub const IXGBE_CS4227_RETRIES: c_int = 15;
pub const IXGBE_CS4227_EFUSE_STATUS: c_uint = 0x0181;
pub const IXGBE_CS4227_LINE_SPARE22_MSB: c_uint = 0x12AD	/* Reg to set speed */;
pub const IXGBE_CS4227_LINE_SPARE24_LSB: c_uint = 0x12B0	/* Reg to set EDC */;
pub const IXGBE_CS4227_HOST_SPARE22_MSB: c_uint = 0x1AAD	/* Reg to set speed */;
pub const IXGBE_CS4227_HOST_SPARE24_LSB: c_uint = 0x1AB0	/* Reg to program EDC */;
pub const IXGBE_CS4227_EEPROM_STATUS: c_uint = 0x5001;
pub const IXGBE_CS4227_EEPROM_LOAD_OK: c_uint = 0x0001;
pub const IXGBE_CS4227_SPEED_1G: c_uint = 0x8000;
pub const IXGBE_CS4227_SPEED_10G: c_int = 0;
pub const IXGBE_CS4227_EDC_MODE_CX1: c_uint = 0x0002;
pub const IXGBE_CS4227_EDC_MODE_SR: c_uint = 0x0004;
pub const IXGBE_CS4227_EDC_MODE_DIAG: c_uint = 0x0008;

pub const IXGBE_PE: c_uint = 0xE0	/* Port expander addr */;

// Flow control defines
pub const IXGBE_TAF_SYM_PAUSE: c_uint = 0x400;
pub const IXGBE_TAF_ASM_PAUSE: c_uint = 0x800;
// Bit-shift macros
pub const IXGBE_SFF_VENDOR_OUI_BYTE0_SHIFT: c_int = 24;
pub const IXGBE_SFF_VENDOR_OUI_BYTE1_SHIFT: c_int = 16;
pub const IXGBE_SFF_VENDOR_OUI_BYTE2_SHIFT: c_int = 8;
// Vendor OUIs: format of OUI is 0x[byte0][byte1][byte2][00]
pub const IXGBE_SFF_VENDOR_OUI_TYCO: c_uint = 0x00407600;
pub const IXGBE_SFF_VENDOR_OUI_FTL: c_uint = 0x00906500;
pub const IXGBE_SFF_VENDOR_OUI_AVAGO: c_uint = 0x00176A00;
pub const IXGBE_SFF_VENDOR_OUI_INTEL: c_uint = 0x001B2100;
// I2C SDA and SCL timing parameters for standard mode
pub const IXGBE_I2C_T_HD_STA: c_int = 4;
pub const IXGBE_I2C_T_LOW: c_int = 5;
pub const IXGBE_I2C_T_HIGH: c_int = 4;
pub const IXGBE_I2C_T_SU_STA: c_int = 5;
pub const IXGBE_I2C_T_HD_DATA: c_int = 5;
pub const IXGBE_I2C_T_SU_DATA: c_int = 1;
pub const IXGBE_I2C_T_RISE: c_int = 1;
pub const IXGBE_I2C_T_FALL: c_int = 1;
pub const IXGBE_I2C_T_SU_STO: c_int = 4;
pub const IXGBE_I2C_T_BUF: c_int = 5;
pub const IXGBE_SFP_DETECT_RETRIES: c_int = 2;
pub const IXGBE_TN_LASI_STATUS_REG: c_uint = 0x9005;
pub const IXGBE_TN_LASI_STATUS_TEMP_ALARM: c_uint = 0x0008;
// SFP+ SFF-8472 Compliance code
pub const IXGBE_SFF_SFF_8472_UNSUP: c_uint = 0x00;
extern "C" {
    pub fn ixgbe_mii_bus_init(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_identify_phy_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_reset_phy_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_setup_phy_link_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_check_reset_blocked(hw: *mut ixgbe_hw) -> bool;
}
// PHY specific
extern "C" {
    pub fn ixgbe_setup_phy_link_tnx(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_reset_phy_nl(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_copper_phy_power(hw: *mut ixgbe_hw, on: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_identify_module_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_identify_sfp_module_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_tn_check_overtemp(hw: *mut ixgbe_hw) -> bool;
}
