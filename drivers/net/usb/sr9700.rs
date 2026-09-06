//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/usb/sr9700.h
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
// CoreChip-sz SR9700 one chip USB 1.1 Ethernet Devices
//
// Author : Liu Junliang <liujunliang_ljl@163.com>
//
// sr9700 spec. register table on Linux platform
// Network Control Reg
pub const SR_NCR: c_uint = 0x00;

// Network Status Reg
pub const SR_NSR: c_uint = 0x01;

// Tx Control Reg
pub const SR_TCR: c_uint = 0x02;

// Tx Status Reg for Packet Index 1
pub const SR_TSR1: c_uint = 0x03;

// Tx Status Reg for Packet Index 2
pub const SR_TSR2: c_uint = 0x04;

// Rx Control Reg
pub const SR_RCR: c_uint = 0x05;

// Rx Status Reg
pub const SR_RSR: c_uint = 0x06;

// Rx Overflow Counter Reg
pub const SR_ROCR: c_uint = 0x07;

// Back Pressure Threshold Reg
pub const SR_BPTR: c_uint = 0x08;

// Flow Control Threshold Reg
pub const SR_FCTR: c_uint = 0x09;

// rx/tx Flow Control Reg
pub const SR_FCR: c_uint = 0x0A;

// Eeprom Control Reg
pub const SR_EPCR: c_uint = 0x0B;

// Eeprom Address Reg
pub const SR_EPAR: c_uint = 0x0C;

// Eeprom Data Reg
pub const SR_EPDR: c_uint = 0x0D	/* 0x0D ~ 0x0E for Data Reg Low & High */;
// Wakeup Control Reg
pub const SR_WCR: c_uint = 0x0F;

// Physical Address Reg
pub const SR_PAR: c_uint = 0x10	/* 0x10 ~ 0x15 6 bytes for PAR */;
// 0x16 --> 0x1E unused
// Phy Reset Reg
pub const SR_PRR: c_uint = 0x1F;

// Tx sdram Write Pointer Address Low
pub const SR_TWPAL: c_uint = 0x20;
// Tx sdram Write Pointer Address High
pub const SR_TWPAH: c_uint = 0x21;
// Tx sdram Read Pointer Address Low
pub const SR_TRPAL: c_uint = 0x22;
// Tx sdram Read Pointer Address High
pub const SR_TRPAH: c_uint = 0x23;
// Rx sdram Write Pointer Address Low
pub const SR_RWPAL: c_uint = 0x24;
// Rx sdram Write Pointer Address High
pub const SR_RWPAH: c_uint = 0x25;
// Rx sdram Read Pointer Address Low
pub const SR_RRPAL: c_uint = 0x26;
// Rx sdram Read Pointer Address High
pub const SR_RRPAH: c_uint = 0x27;
// Vendor ID register
pub const SR_VID: c_uint = 0x28	/* 0x28 ~ 0x29 2 bytes for VID */;
// Product ID register
pub const SR_PID: c_uint = 0x2A	/* 0x2A ~ 0x2B 2 bytes for PID */;
// CHIP Revision register
pub const SR_CHIPR: c_uint = 0x2C;
// 0x2D --> 0xEF unused
// USB Device Address
pub const SR_USBDA: c_uint = 0xF0;

// RX packet Counter Reg
pub const SR_RXC: c_uint = 0xF1;
// Tx packet Counter & USB Status Reg
pub const SR_TXC_USBS: c_uint = 0xF2;

// USB Control register
pub const SR_USBC: c_uint = 0xF4;

// Register access commands and flags
pub const SR_RD_REGS: c_uint = 0x00;
pub const SR_WR_MULTIPLE_REGS: c_uint = 0x01;
pub const SR_WR_SINGLE_REG: c_uint = 0x03;

// parameters
pub const SR_EEPROM_TIMEOUT: c_int = 1000;
pub const SR_EEPROM_LEN: c_int = 256;

