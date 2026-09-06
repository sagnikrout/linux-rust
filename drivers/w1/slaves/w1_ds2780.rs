//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/w1/slaves/w1_ds2780.h
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
// 1-Wire implementation for the ds2780 chip
//
// Copyright (C) 2010 Indesign, LLC
//
// Author: Clifton Barnes <cabarnes@indesign-llc.com>
//
// Based on w1-ds2760 driver
//
// Function commands
pub const W1_DS2780_READ_DATA: c_uint = 0x69;
pub const W1_DS2780_WRITE_DATA: c_uint = 0x6C;
pub const W1_DS2780_COPY_DATA: c_uint = 0x48;
pub const W1_DS2780_RECALL_DATA: c_uint = 0xB8;
pub const W1_DS2780_LOCK: c_uint = 0x6A;
// Register map
// Register 0x00 Reserved
pub const DS2780_STATUS_REG: c_uint = 0x01;
pub const DS2780_RAAC_MSB_REG: c_uint = 0x02;
pub const DS2780_RAAC_LSB_REG: c_uint = 0x03;
pub const DS2780_RSAC_MSB_REG: c_uint = 0x04;
pub const DS2780_RSAC_LSB_REG: c_uint = 0x05;
pub const DS2780_RARC_REG: c_uint = 0x06;
pub const DS2780_RSRC_REG: c_uint = 0x07;
pub const DS2780_IAVG_MSB_REG: c_uint = 0x08;
pub const DS2780_IAVG_LSB_REG: c_uint = 0x09;
pub const DS2780_TEMP_MSB_REG: c_uint = 0x0A;
pub const DS2780_TEMP_LSB_REG: c_uint = 0x0B;
pub const DS2780_VOLT_MSB_REG: c_uint = 0x0C;
pub const DS2780_VOLT_LSB_REG: c_uint = 0x0D;
pub const DS2780_CURRENT_MSB_REG: c_uint = 0x0E;
pub const DS2780_CURRENT_LSB_REG: c_uint = 0x0F;
pub const DS2780_ACR_MSB_REG: c_uint = 0x10;
pub const DS2780_ACR_LSB_REG: c_uint = 0x11;
pub const DS2780_ACRL_MSB_REG: c_uint = 0x12;
pub const DS2780_ACRL_LSB_REG: c_uint = 0x13;
pub const DS2780_AS_REG: c_uint = 0x14;
pub const DS2780_SFR_REG: c_uint = 0x15;
pub const DS2780_FULL_MSB_REG: c_uint = 0x16;
pub const DS2780_FULL_LSB_REG: c_uint = 0x17;
pub const DS2780_AE_MSB_REG: c_uint = 0x18;
pub const DS2780_AE_LSB_REG: c_uint = 0x19;
pub const DS2780_SE_MSB_REG: c_uint = 0x1A;
pub const DS2780_SE_LSB_REG: c_uint = 0x1B;
// Register 0x1C - 0x1E Reserved
pub const DS2780_EEPROM_REG: c_uint = 0x1F;
pub const DS2780_EEPROM_BLOCK0_START: c_uint = 0x20;
// Register 0x20 - 0x2F User EEPROM
pub const DS2780_EEPROM_BLOCK0_END: c_uint = 0x2F;
// Register 0x30 - 0x5F Reserved
pub const DS2780_EEPROM_BLOCK1_START: c_uint = 0x60;
pub const DS2780_CONTROL_REG: c_uint = 0x60;
pub const DS2780_AB_REG: c_uint = 0x61;
pub const DS2780_AC_MSB_REG: c_uint = 0x62;
pub const DS2780_AC_LSB_REG: c_uint = 0x63;
pub const DS2780_VCHG_REG: c_uint = 0x64;
pub const DS2780_IMIN_REG: c_uint = 0x65;
pub const DS2780_VAE_REG: c_uint = 0x66;
pub const DS2780_IAE_REG: c_uint = 0x67;
pub const DS2780_AE_40_REG: c_uint = 0x68;
pub const DS2780_RSNSP_REG: c_uint = 0x69;
pub const DS2780_FULL_40_MSB_REG: c_uint = 0x6A;
pub const DS2780_FULL_40_LSB_REG: c_uint = 0x6B;
pub const DS2780_FULL_3040_SLOPE_REG: c_uint = 0x6C;
pub const DS2780_FULL_2030_SLOPE_REG: c_uint = 0x6D;
pub const DS2780_FULL_1020_SLOPE_REG: c_uint = 0x6E;
pub const DS2780_FULL_0010_SLOPE_REG: c_uint = 0x6F;
pub const DS2780_AE_3040_SLOPE_REG: c_uint = 0x70;
pub const DS2780_AE_2030_SLOPE_REG: c_uint = 0x71;
pub const DS2780_AE_1020_SLOPE_REG: c_uint = 0x72;
pub const DS2780_AE_0010_SLOPE_REG: c_uint = 0x73;
pub const DS2780_SE_3040_SLOPE_REG: c_uint = 0x74;
pub const DS2780_SE_2030_SLOPE_REG: c_uint = 0x75;
pub const DS2780_SE_1020_SLOPE_REG: c_uint = 0x76;
pub const DS2780_SE_0010_SLOPE_REG: c_uint = 0x77;
pub const DS2780_RSGAIN_MSB_REG: c_uint = 0x78;
pub const DS2780_RSGAIN_LSB_REG: c_uint = 0x79;
pub const DS2780_RSTC_REG: c_uint = 0x7A;
pub const DS2780_FRSGAIN_MSB_REG: c_uint = 0x7B;
pub const DS2780_FRSGAIN_LSB_REG: c_uint = 0x7C;
pub const DS2780_EEPROM_BLOCK1_END: c_uint = 0x7C;
// Register 0x7D - 0xFF Reserved
// Number of valid register addresses
pub const DS2780_DATA_SIZE: c_uint = 0x80;
// Status register bits

// Bit 3 Reserved

// Bit 0 Reserved
// Control register bits
// Bit 7 Reserved

// Bit 0 - 3 Reserved
// Special feature register bits
// Bit 1 - 7 Reserved

// EEPROM register bits

// Bit 2 - 6 Reserved

extern "C" {
    pub fn w1_ds2780_eeprom_cmd(dev: *mut device, addr: c_int, cmd: c_int) -> c_int;
}
