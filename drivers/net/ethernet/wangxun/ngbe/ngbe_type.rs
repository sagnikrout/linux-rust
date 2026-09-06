//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/ngbe/ngbe_type.h
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
// Copyright (c) 2019 - 2022 Beijing WangXun Technology Co., Ltd.

// NGBE_register.h
// Device IDs
pub const NGBE_DEV_ID_EM_WX1860AL_W: c_uint = 0x0100;
pub const NGBE_DEV_ID_EM_WX1860A2: c_uint = 0x0101;
pub const NGBE_DEV_ID_EM_WX1860A2S: c_uint = 0x0102;
pub const NGBE_DEV_ID_EM_WX1860A4: c_uint = 0x0103;
pub const NGBE_DEV_ID_EM_WX1860A4S: c_uint = 0x0104;
pub const NGBE_DEV_ID_EM_WX1860AL2: c_uint = 0x0105;
pub const NGBE_DEV_ID_EM_WX1860AL2S: c_uint = 0x0106;
pub const NGBE_DEV_ID_EM_WX1860AL4: c_uint = 0x0107;
pub const NGBE_DEV_ID_EM_WX1860AL4S: c_uint = 0x0108;
pub const NGBE_DEV_ID_EM_WX1860LC: c_uint = 0x0109;
pub const NGBE_DEV_ID_EM_WX1860A1: c_uint = 0x010a;
pub const NGBE_DEV_ID_EM_WX1860A1L: c_uint = 0x010b;
// Subsystem ID
pub const NGBE_SUBID_M88E1512_SFP: c_uint = 0x0003;
pub const NGBE_SUBID_OCP_CARD: c_uint = 0x0040;
pub const NGBE_SUBID_LY_M88E1512_SFP: c_uint = 0x0050;
pub const NGBE_SUBID_M88E1512_RJ45: c_uint = 0x0051;
pub const NGBE_SUBID_M88E1512_MIX: c_uint = 0x0052;
pub const NGBE_SUBID_YT8521S_SFP: c_uint = 0x0060;
pub const NGBE_SUBID_INTERNAL_YT8521S_SFP: c_uint = 0x0061;
pub const NGBE_SUBID_YT8521S_SFP_GPIO: c_uint = 0x0062;
pub const NGBE_SUBID_INTERNAL_YT8521S_SFP_GPIO: c_uint = 0x0064;
pub const NGBE_SUBID_LY_YT8521S_SFP: c_uint = 0x0070;
pub const NGBE_SUBID_RGMII_FPGA: c_uint = 0x0080;
pub const NGBE_OEM_MASK: c_uint = 0x00FF;
// EM Registers
// chip control Registers
pub const NGBE_MIS_PRB_CTL: c_uint = 0x10010;
// FMGR Registers
pub const NGBE_SPI_ILDR_STATUS: c_uint = 0x10120;

// Checksum and EEPROM pointers
pub const NGBE_CALSUM_COMMAND: c_uint = 0xE9;
pub const NGBE_CALSUM_CAP_STATUS: c_uint = 0x10224;
pub const NGBE_EEPROM_VERSION_STORE_REG: c_uint = 0x1022C;
pub const NGBE_SAN_MAC_ADDR_PTR: c_uint = 0x18;
pub const NGBE_DEVICE_CAPS: c_uint = 0x1C;
pub const NGBE_EEPROM_VERSION_L: c_uint = 0x1D;
pub const NGBE_EEPROM_VERSION_H: c_uint = 0x1E;
// GPIO Registers
pub const NGBE_GPIO_DR: c_uint = 0x14800;
pub const NGBE_GPIO_DDR: c_uint = 0x14804;
// GPIO bit

// Extended Interrupt Enable Set

// Extended Interrupt Cause Read

pub const NGBE_INTR_ALL: c_uint = 0x1FF;

pub const NGBE_CFG_LAN_SPEED: c_uint = 0x14440;
pub const NGBE_CFG_PORT_ST: c_uint = 0x14404;
pub const NGBE_FW_EEPROM_CHECKSUM_CMD: c_uint = 0xE9;
pub const NGBE_FW_NVM_DATA_OFFSET: c_int = 3;
pub const NGBE_FW_CMD_DEFAULT_CHECKSUM: c_uint = 0xFF /* checksum always 0xFF */;
pub const NGBE_FW_CMD_ST_PASS: c_uint = 0x80658383;
pub const NGBE_FW_CMD_ST_FAIL: c_uint = 0x70657376;
pub const NGBE_MAX_FDIR_INDICES: c_int = 7;
pub const NGBE_MAX_RSS_INDICES: c_int = 8;

pub const NGBE_ETH_LENGTH_OF_ADDRESS: c_int = 6;
pub const NGBE_MAX_MSIX_VECTORS: c_uint = 0x09;
pub const NGBE_RAR_ENTRIES: c_int = 32;
pub const NGBE_RX_PB_SIZE: c_int = 42;
pub const NGBE_MC_TBL_SIZE: c_int = 128;
pub const NGBE_SP_VFT_TBL_SIZE: c_int = 128;

// TX/RX descriptor defines

pub const NGBE_DEFAULT_TX_WORK: c_int = 256;
pub const NGBE_MAX_TXD: c_int = 8192;
pub const NGBE_MIN_TXD: c_int = 128;

pub const NGBE_DEFAULT_RX_WORK: c_int = 256;
pub const NGBE_MAX_RXD: c_int = 8192;
pub const NGBE_MIN_RXD: c_int = 128;
pub const NGBE_MAX_VFS_DRV_LIMIT: c_int = 7;
extern "C" {
    pub fn ngbe_down(wx: *mut wx);
}
extern "C" {
    pub fn ngbe_up(wx: *mut wx);
}
extern "C" {
    pub fn ngbe_setup_tc(dev: *mut net_device, tc: u8) -> c_int;
}
extern "C" {
    pub fn ngbe_do_reset(netdev: *mut net_device, reinit: bool);
}
