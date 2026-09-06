//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/eeprom_93cx6.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

//
// EEPROM operation defines.
//
pub const PCI_EEPROM_WIDTH_93C46: c_int = 6;
pub const PCI_EEPROM_WIDTH_93C56: c_int = 8;
pub const PCI_EEPROM_WIDTH_93C66: c_int = 8;
pub const PCI_EEPROM_WIDTH_93C86: c_int = 8;
pub const PCI_EEPROM_WIDTH_OPCODE: c_int = 3;
pub const PCI_EEPROM_WRITE_OPCODE: c_uint = 0x05;
pub const PCI_EEPROM_ERASE_OPCODE: c_uint = 0x07;
pub const PCI_EEPROM_READ_OPCODE: c_uint = 0x06;
pub const PCI_EEPROM_EWDS_OPCODE: c_uint = 0x10;
pub const PCI_EEPROM_EWEN_OPCODE: c_uint = 0x13;
//
// struct eeprom_93cx6 - control structure for setting the commands
// for reading the eeprom data.
// @data: private pointer for the driver.
// @register_read: handler to read the eeprom register;
// this function should set all reg_* fields.
// @register_write: handler to write to the eeprom register by using
// all reg_* fields.
// @width: eeprom width, should be one of the PCI_EEPROM_WIDTH_* defines
// @quirks: eeprom or controller quirks
// @drive_data: Set if we're driving the data line.
// @reg_data_in: register field to indicate data input
// @reg_data_out: register field to indicate data output
// @reg_data_clock: register field to set the data clock
// @reg_chip_select: register field to set the chip select
//
// This structure is used for the communication between the driver
// and the eeprom_93cx6 handlers for reading the eeprom.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_93cx6 {
    pub data: *mut c_void,
    pub eeprom): *mut *mut void (register_read)(struct eeprom_93cx6,
    pub eeprom): *mut *mut void (register_write)(struct eeprom_93cx6,
    pub width: c_int,
    pub quirks: c_uint,
// Some EEPROMs require an extra clock cycle before reading

    pub drive_data: c_char,
    pub reg_data_in: c_char,
    pub reg_data_out: c_char,
    pub reg_data_clock: c_char,
    pub reg_chip_select: c_char,
}

extern "C" {
    pub fn eeprom_93cx6_wren(eeprom: *mut eeprom_93cx6, enable: bool);
}
