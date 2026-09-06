//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla4xxx/ql4_nvram.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// QLogic iSCSI HBA Driver
// Copyright (c)  2003-2013 QLogic Corporation
//
// AM29LV Flash definitions
//
pub const FM93C56A_SIZE_8: c_uint = 0x100;
pub const FM93C56A_SIZE_16: c_uint = 0x80;
pub const FM93C66A_SIZE_8: c_uint = 0x200;
pub const FM93C66A_SIZE_16: c_uint = 0x100/* 4010 */;
pub const FM93C86A_SIZE_16: c_uint = 0x400/* 4022 */;
pub const FM93C56A_START: c_uint = 0x1;
// Commands
pub const FM93C56A_READ: c_uint = 0x2;
pub const FM93C56A_WEN: c_uint = 0x0;
pub const FM93C56A_WRITE: c_uint = 0x1;
pub const FM93C56A_WRITE_ALL: c_uint = 0x0;
pub const FM93C56A_WDS: c_uint = 0x0;
pub const FM93C56A_ERASE: c_uint = 0x3;
pub const FM93C56A_ERASE_ALL: c_uint = 0x0;
// Command Extensions
pub const FM93C56A_WEN_EXT: c_uint = 0x3;
pub const FM93C56A_WRITE_ALL_EXT: c_uint = 0x1;
pub const FM93C56A_WDS_EXT: c_uint = 0x0;
pub const FM93C56A_ERASE_ALL_EXT: c_uint = 0x2;
// Address Bits

// Data Bits
pub const FM93C56A_DATA_BITS_16: c_int = 16;
pub const FM93C56A_DATA_BITS_8: c_int = 8;
// Special Bits
pub const FM93C56A_READ_DUMMY_BITS: c_int = 1;
pub const FM93C56A_READY: c_int = 0;
pub const FM93C56A_BUSY: c_int = 1;
pub const FM93C56A_CMD_BITS: c_int = 2;
// Auburn Bits
pub const AUBURN_EEPROM_DI: c_uint = 0x8;
pub const AUBURN_EEPROM_DI_0: c_uint = 0x0;
pub const AUBURN_EEPROM_DI_1: c_uint = 0x8;
pub const AUBURN_EEPROM_DO: c_uint = 0x4;
pub const AUBURN_EEPROM_DO_0: c_uint = 0x0;
pub const AUBURN_EEPROM_DO_1: c_uint = 0x4;
pub const AUBURN_EEPROM_CS: c_uint = 0x2;
pub const AUBURN_EEPROM_CS_0: c_uint = 0x0;
pub const AUBURN_EEPROM_CS_1: c_uint = 0x2;
pub const AUBURN_EEPROM_CLK_RISE: c_uint = 0x1;
pub const AUBURN_EEPROM_CLK_FALL: c_uint = 0x0;
//
// EEPROM format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_params {
    pub SpinUpDelay:1: u16,
    pub BIOSDisable:1: u16,
    pub MMAPEnable:1: u16,
    pub BootEnable:1: u16,
    pub Reserved0:12: u16,
    pub bootID0:7: u8,
    pub bootID0Valid:1: u8,
    pub bootLUN0: [u8; 8],
    pub bootID1:7: u8,
    pub bootID1Valid:1: u8,
    pub bootLUN1: [u8; 8],
    pub MaxLunsPerTarget: u16,
    pub Reserved1: [u8; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_port_cfg {
// MTU MAC 0
    pub etherMtu_mac: u16,
// Flow Control MAC 0
    pub pauseThreshold_mac: u16,
    pub resumeThreshold_mac: u16,
    pub reserved: [u16; 13],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_function_cfg {
    pub reserved: [u8; 30],
// MAC ADDR
    pub macAddress: [u8; 6],
    pub macAddressSecondary: [u8; 6],
    pub subsysVendorId: u16,
    pub subsysDeviceId: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_data {
    pub /: *mut *mut u8 asic_id[4]; / x00,
    pub /: *mut *mut u8 version; / x04,
    pub /: *mut *mut u8 reserved; / x05,
    pub /: *mut *mut u16 board_id; / x06,
pub const EEPROM_BOARDID_ELDORADO: c_int = 1;
pub const EEPROM_BOARDID_PLACER: c_int = 2;
pub const EEPROM_SERIAL_NUM_SIZE: c_int = 16;
    pub /: *mut *mut u8 serial_number[EEPROM_SERIAL_NUM_SIZE]; / x08,
// ExtHwConfig:
// Offset = 24bytes
//
// | SSRAM Size|     |ST|PD|SDRAM SZ| W| B| SP	|  |
// |15|14|13|12|11|10| 9| 8| 7| 6| 5| 4| 3| 2| 1| 0|
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//
    pub /: *mut *mut u16 ext_hw_conf; / x18,
    pub /: *mut *mut u8 mac0[6]; / x1A,
    pub /: *mut *mut u8 mac1[6]; / x20,
    pub /: *mut *mut u8 mac2[6]; / x26,
    pub /: *mut *mut u8 mac3[6]; / x2C,
    pub /: *mut *mut u16 etherMtu; / x32,
    pub /: *mut *mut u16 macConfig; / x34,
pub const MAC_CONFIG_ENABLE_ANEG: c_uint = 0x0001;
pub const MAC_CONFIG_ENABLE_PAUSE: c_uint = 0x0002;
    pub /: *mut *mut u16 phyConfig; / x36,
pub const PHY_CONFIG_PHY_ADDR_MASK: c_uint = 0x1f;
pub const PHY_CONFIG_ENABLE_FW_MANAGEMENT_MASK: c_uint = 0x20;
    pub /: *mut *mut u16 reserved_56; / x38,
pub const EEPROM_UNUSED_1_SIZE: c_int = 2;
    pub /: *mut *mut u8 unused_1[EEPROM_UNUSED_1_SIZE]; / x3A,
    pub /: *mut *mut u16 bufletSize; / x3C,
    pub /: *mut *mut u16 bufletCount; / x3E,
    pub /: *mut *mut u16 bufletPauseThreshold; / x40,
    pub /: *mut *mut u16 tcpWindowThreshold50; / x42,
    pub /: *mut *mut u16 tcpWindowThreshold25; / x44,
    pub /: *mut *mut u16 tcpWindowThreshold0; / x46,
    pub /: *mut *mut u16 ipHashTableBaseHi; / x48,
    pub /: *mut *mut u16 ipHashTableBaseLo; / x4A,
    pub /: *mut *mut u16 ipHashTableSize; / x4C,
    pub /: *mut *mut u16 tcpHashTableBaseHi; / x4E,
    pub /: *mut *mut u16 tcpHashTableBaseLo; / x50,
    pub /: *mut *mut u16 tcpHashTableSize; / x52,
    pub /: *mut *mut u16 ncbTableBaseHi; / x54,
    pub /: *mut *mut u16 ncbTableBaseLo; / x56,
    pub /: *mut *mut u16 ncbTableSize; / x58,
    pub /: *mut *mut u16 drbTableBaseHi; / x5A,
    pub /: *mut *mut u16 drbTableBaseLo; / x5C,
    pub /: *mut *mut u16 drbTableSize; / x5E,
pub const EEPROM_UNUSED_2_SIZE: c_int = 4;
    pub /: *mut *mut u8 unused_2[EEPROM_UNUSED_2_SIZE]; / x60,
    pub /: *mut *mut u16 ipReassemblyTimeout; / x64,
    pub /: *mut *mut u16 tcpMaxWindowSizeHi; / x66,
    pub /: *mut *mut u16 tcpMaxWindowSizeLo; / x68,
    pub TOE: *mut *mut u32 net_ip_addr0; / x6A Added for,
// functionality.
    pub /: *mut *mut u32 net_ip_addr1; / x6E,
    pub /: *mut *mut u32 scsi_ip_addr0; / x72,
    pub /: *mut *mut u32 scsi_ip_addr1; / x76,

// for ip addresses
    pub /: *mut *mut u8 unused_3[EEPROM_UNUSED_3_SIZE]; / x7A,
    pub /: *mut *mut u16 subsysVendorId_f0; / xFA,
    pub /: *mut *mut u16 subsysDeviceId_f0; / xFC,
// Address = 0x7F
pub const FM93C56A_SIGNATURE: c_uint = 0x9356;
pub const FM93C66A_SIGNATURE: c_uint = 0x9366;
    pub /: *mut *mut u16 signature; / xFE,
pub const EEPROM_UNUSED_4_SIZE: c_int = 250;
    pub /: *mut *mut u8 unused_4[EEPROM_UNUSED_4_SIZE]; / x100,
    pub /: *mut *mut u16 subsysVendorId_f1; / x1FA,
    pub /: *mut *mut u16 subsysDeviceId_f1; / x1FC,
    pub /: *mut *mut u16 checksum; / x1FE,
// C attribute field omitted
    pub /: *mut *mut u8 asicId[4]; / x00,
    pub /: *mut *mut u8 version; / x04,
    pub /: *mut *mut u8 reserved_5; / x05,
    pub /: *mut *mut u16 boardId; / x06,
    pub /: *mut *mut u8 boardIdStr[16]; / x08,
    pub /: *mut *mut u8 serialNumber[16]; / x18,
// External Hardware Configuration
    pub /: *mut *mut u16 ext_hw_conf; / x28,
// MAC 0 CONFIGURATION
    pub /: *mut *mut eeprom_port_cfg macCfg_port0; / x2A,
// MAC 1 CONFIGURATION
    pub /: *mut *mut eeprom_port_cfg macCfg_port1; / x4A,
// DDR SDRAM Configuration
    pub /: *mut *mut u16 bufletSize; / x6A,
    pub /: *mut *mut u16 bufletCount; / x6C,
    pub /: *mut *mut u16 tcpWindowThreshold50; / x6E,
    pub /: *mut *mut u16 tcpWindowThreshold25; / x70,
    pub /: *mut *mut u16 tcpWindowThreshold0; / x72,
    pub /: *mut *mut u16 ipHashTableBaseHi; / x74,
    pub /: *mut *mut u16 ipHashTableBaseLo; / x76,
    pub /: *mut *mut u16 ipHashTableSize; / x78,
    pub /: *mut *mut u16 tcpHashTableBaseHi; / x7A,
    pub /: *mut *mut u16 tcpHashTableBaseLo; / x7C,
    pub /: *mut *mut u16 tcpHashTableSize; / x7E,
    pub /: *mut *mut u16 ncbTableBaseHi; / x80,
    pub /: *mut *mut u16 ncbTableBaseLo; / x82,
    pub /: *mut *mut u16 ncbTableSize; / x84,
    pub /: *mut *mut u16 drbTableBaseHi; / x86,
    pub /: *mut *mut u16 drbTableBaseLo; / x88,
    pub /: *mut *mut u16 drbTableSize; / x8A,
    pub /: *mut *mut u16 reserved_142[4]; / x8C,
// TCP/IP Parameters
    pub /: *mut *mut u16 ipReassemblyTimeout; / x94,
    pub /: *mut *mut u16 tcpMaxWindowSize; / x96,
    pub /: *mut *mut u16 ipSecurity; / x98,
    pub /: *mut *mut u8 reserved_156[294]; / x9A,
    pub /: *mut *mut u16 qDebug[8]; / QLOGIC USE ONLY x1C0,
    pub /: *mut *mut eeprom_function_cfg funcCfg_fn0; / x1D0,
    pub /: *mut *mut u16 reserved_510; / x1FE,
// Address = 512
    pub /: *mut *mut u8 oemSpace[432]; / x200,
    pub /: *mut *mut bios_params sBIOSParams_fn1; / x3B0,
    pub /: *mut *mut eeprom_function_cfg funcCfg_fn1; / x3D0,
    pub /: *mut *mut u16 reserved_1022; / x3FE,
// Address = 1024
    pub /: *mut *mut u8 reserved_1024[464]; / x400,
    pub /: *mut *mut eeprom_function_cfg funcCfg_fn2; / x5D0,
    pub /: *mut *mut u16 reserved_1534; / x5FE,
// Address = 1536
    pub /: *mut *mut u8 reserved_1536[432]; / x600,
    pub /: *mut *mut bios_params sBIOSParams_fn3; / x7B0,
    pub /: *mut *mut eeprom_function_cfg funcCfg_fn3; / x7D0,
    pub /: *mut *mut u16 checksum; / x7FE,
// C attribute field omitted
}
