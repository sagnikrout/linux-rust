//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/usb/sr9800.h
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


// CoreChip-sz SR9800 one chip USB 2.0 Ethernet Devices
//
// Author : Liu Junliang <liujunliang_ljl@163.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
// SR9800 spec. command table on Linux Platform
// command : Software Station Management Control Reg
pub const SR_CMD_SET_SW_MII: c_uint = 0x06;
// command : PHY Read Reg
pub const SR_CMD_READ_MII_REG: c_uint = 0x07;
// command : PHY Write Reg
pub const SR_CMD_WRITE_MII_REG: c_uint = 0x08;
// command : Hardware Station Management Control Reg
pub const SR_CMD_SET_HW_MII: c_uint = 0x0a;
// command : SROM Read Reg
pub const SR_CMD_READ_EEPROM: c_uint = 0x0b;
// command : SROM Write Reg
pub const SR_CMD_WRITE_EEPROM: c_uint = 0x0c;
// command : SROM Write Enable Reg
pub const SR_CMD_WRITE_ENABLE: c_uint = 0x0d;
// command : SROM Write Disable Reg
pub const SR_CMD_WRITE_DISABLE: c_uint = 0x0e;
// command : RX Control Read Reg
pub const SR_CMD_READ_RX_CTL: c_uint = 0x0f;

// command : RX Control Write Reg
pub const SR_CMD_WRITE_RX_CTL: c_uint = 0x10;
// command : IPG0/IPG1/IPG2 Control Read Reg
pub const SR_CMD_READ_IPG012: c_uint = 0x11;
// command : IPG0/IPG1/IPG2 Control Write Reg
pub const SR_CMD_WRITE_IPG012: c_uint = 0x12;
// command : Node ID Read Reg
pub const SR_CMD_READ_NODE_ID: c_uint = 0x13;
// command : Node ID Write Reg
pub const SR_CMD_WRITE_NODE_ID: c_uint = 0x14;
// command : Multicast Filter Array Read Reg
pub const SR_CMD_READ_MULTI_FILTER: c_uint = 0x15;
// command : Multicast Filter Array Write Reg
pub const SR_CMD_WRITE_MULTI_FILTER: c_uint = 0x16;
// command : Eth/HomePNA PHY Address Reg
pub const SR_CMD_READ_PHY_ID: c_uint = 0x19;
// command : Medium Status Read Reg
pub const SR_CMD_READ_MEDIUM_STATUS: c_uint = 0x1a;

// command : Medium Status Write Reg
pub const SR_CMD_WRITE_MEDIUM_MODE: c_uint = 0x1b;

// command : Monitor Mode Status Read Reg
pub const SR_CMD_READ_MONITOR_MODE: c_uint = 0x1c;
// command : Monitor Mode Status Write Reg
pub const SR_CMD_WRITE_MONITOR_MODE: c_uint = 0x1d;
// command : GPIO Status Read Reg
pub const SR_CMD_READ_GPIOS: c_uint = 0x1e;

// command : GPIO Status Write Reg
pub const SR_CMD_WRITE_GPIOS: c_uint = 0x1f;
// command : Eth PHY Power and Reset Control Reg
pub const SR_CMD_SW_RESET: c_uint = 0x20;
pub const SR_SWRESET_CLEAR: c_uint = 0x00;

// command : Software Interface Selection Status Read Reg
pub const SR_CMD_SW_PHY_STATUS: c_uint = 0x21;
// command : Software Interface Selection Status Write Reg
pub const SR_CMD_SW_PHY_SELECT: c_uint = 0x22;
// command : BULK in Buffer Size Reg
pub const SR_CMD_BULKIN_SIZE: c_uint = 0x2A;
// command : LED_MUX Control Reg
pub const SR_CMD_LED_MUX: c_uint = 0x70;

// Register Access Flags

// Multicast Filter Array size & Max Number
pub const SR_MCAST_FILTER_SIZE: c_int = 8;
pub const SR_MAX_MCAST: c_int = 64;
// IPG0/1/2 Default Value
pub const SR9800_IPG0_DEFAULT: c_uint = 0x15;
pub const SR9800_IPG1_DEFAULT: c_uint = 0x0c;
pub const SR9800_IPG2_DEFAULT: c_uint = 0x12;
// Medium Status Default Mode

// RX Control Default Setting

// EEPROM Magic Number & EEPROM Size
pub const SR_EEPROM_MAGIC: c_uint = 0xdeadbeef;
pub const SR9800_EEPROM_LEN: c_uint = 0xff;
// SR9800 Driver Name and Flags

// SR9800 BULKIN Buffer Size
pub const SR9800_MAX_BULKIN_2K: c_int = 0;
pub const SR9800_MAX_BULKIN_4K: c_int = 1;
pub const SR9800_MAX_BULKIN_6K: c_int = 2;
pub const SR9800_MAX_BULKIN_8K: c_int = 3;
pub const SR9800_MAX_BULKIN_16K: c_int = 4;
pub const SR9800_MAX_BULKIN_20K: c_int = 5;
pub const SR9800_MAX_BULKIN_24K: c_int = 6;
pub const SR9800_MAX_BULKIN_32K: c_int = 7;
// 2k
// 4k
// 6k
// 8k
// 16
// 20k
// 24k
// 32k
// This structure cannot exceed sizeof(unsigned long [5]) AKA 20 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sr_data {
    pub multi_filter: [u8; SR_MCAST_FILTER_SIZE],
    pub mac_addr: [u8; ETH_ALEN],
    pub phymode: u8,
    pub ledmode: u8,
    pub eeprom_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sr9800_int_data {
    pub res1: __le16,
    pub link: u8,
    pub res2: __le16,
    pub status: u8,
    pub res3: __le16,
    pub __packed: },
