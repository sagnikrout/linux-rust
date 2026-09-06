//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/w1/slaves/w1_ds2781.h
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
// Author: Renata Sayakhova <renata@oktetlabs.ru>
//
// Based on w1-ds2760 driver
//
// Function commands
pub const W1_DS2781_READ_DATA: c_uint = 0x69;
pub const W1_DS2781_WRITE_DATA: c_uint = 0x6C;
pub const W1_DS2781_COPY_DATA: c_uint = 0x48;
pub const W1_DS2781_RECALL_DATA: c_uint = 0xB8;
pub const W1_DS2781_LOCK: c_uint = 0x6A;
// Register map
// Register 0x00 Reserved
pub const DS2781_STATUS: c_uint = 0x01;
pub const DS2781_RAAC_MSB: c_uint = 0x02;
pub const DS2781_RAAC_LSB: c_uint = 0x03;
pub const DS2781_RSAC_MSB: c_uint = 0x04;
pub const DS2781_RSAC_LSB: c_uint = 0x05;
pub const DS2781_RARC: c_uint = 0x06;
pub const DS2781_RSRC: c_uint = 0x07;
pub const DS2781_IAVG_MSB: c_uint = 0x08;
pub const DS2781_IAVG_LSB: c_uint = 0x09;
pub const DS2781_TEMP_MSB: c_uint = 0x0A;
pub const DS2781_TEMP_LSB: c_uint = 0x0B;
pub const DS2781_VOLT_MSB: c_uint = 0x0C;
pub const DS2781_VOLT_LSB: c_uint = 0x0D;
pub const DS2781_CURRENT_MSB: c_uint = 0x0E;
pub const DS2781_CURRENT_LSB: c_uint = 0x0F;
pub const DS2781_ACR_MSB: c_uint = 0x10;
pub const DS2781_ACR_LSB: c_uint = 0x11;
pub const DS2781_ACRL_MSB: c_uint = 0x12;
pub const DS2781_ACRL_LSB: c_uint = 0x13;
pub const DS2781_AS: c_uint = 0x14;
pub const DS2781_SFR: c_uint = 0x15;
pub const DS2781_FULL_MSB: c_uint = 0x16;
pub const DS2781_FULL_LSB: c_uint = 0x17;
pub const DS2781_AE_MSB: c_uint = 0x18;
pub const DS2781_AE_LSB: c_uint = 0x19;
pub const DS2781_SE_MSB: c_uint = 0x1A;
pub const DS2781_SE_LSB: c_uint = 0x1B;
// Register 0x1C - 0x1E Reserved
pub const DS2781_EEPROM: c_uint = 0x1F;
pub const DS2781_EEPROM_BLOCK0_START: c_uint = 0x20;
// Register 0x20 - 0x2F User EEPROM
pub const DS2781_EEPROM_BLOCK0_END: c_uint = 0x2F;
// Register 0x30 - 0x5F Reserved
pub const DS2781_EEPROM_BLOCK1_START: c_uint = 0x60;
pub const DS2781_CONTROL: c_uint = 0x60;
pub const DS2781_AB: c_uint = 0x61;
pub const DS2781_AC_MSB: c_uint = 0x62;
pub const DS2781_AC_LSB: c_uint = 0x63;
pub const DS2781_VCHG: c_uint = 0x64;
pub const DS2781_IMIN: c_uint = 0x65;
pub const DS2781_VAE: c_uint = 0x66;
pub const DS2781_IAE: c_uint = 0x67;
pub const DS2781_AE_40: c_uint = 0x68;
pub const DS2781_RSNSP: c_uint = 0x69;
pub const DS2781_FULL_40_MSB: c_uint = 0x6A;
pub const DS2781_FULL_40_LSB: c_uint = 0x6B;
pub const DS2781_FULL_4_SLOPE: c_uint = 0x6C;
pub const DS2781_FULL_3_SLOPE: c_uint = 0x6D;
pub const DS2781_FULL_2_SLOPE: c_uint = 0x6E;
pub const DS2781_FULL_1_SLOPE: c_uint = 0x6F;
pub const DS2781_AE_4_SLOPE: c_uint = 0x70;
pub const DS2781_AE_3_SLOPE: c_uint = 0x71;
pub const DS2781_AE_2_SLOPE: c_uint = 0x72;
pub const DS2781_AE_1_SLOPE: c_uint = 0x73;
pub const DS2781_SE_4_SLOPE: c_uint = 0x74;
pub const DS2781_SE_3_SLOPE: c_uint = 0x75;
pub const DS2781_SE_2_SLOPE: c_uint = 0x76;
pub const DS2781_SE_1_SLOPE: c_uint = 0x77;
pub const DS2781_RSGAIN_MSB: c_uint = 0x78;
pub const DS2781_RSGAIN_LSB: c_uint = 0x79;
pub const DS2781_RSTC: c_uint = 0x7A;
pub const DS2781_COB: c_uint = 0x7B;
pub const DS2781_TBP34: c_uint = 0x7C;
pub const DS2781_TBP23: c_uint = 0x7D;
pub const DS2781_TBP12: c_uint = 0x7E;
pub const DS2781_EEPROM_BLOCK1_END: c_uint = 0x7F;
// Register 0x7D - 0xFF Reserved
pub const DS2781_FSGAIN_MSB: c_uint = 0xB0;
pub const DS2781_FSGAIN_LSB: c_uint = 0xB1;
// Number of valid register addresses
pub const DS2781_DATA_SIZE: c_uint = 0xB2;
// Status register bits

// Bit 3 Reserved

// Bit 0 Reserved
// Control register bits
// Bit 7 Reserved

// Bit 0 - 2 Reserved
// Special feature register bits
// Bit 1 - 7 Reserved

// EEPROM register bits

// Bit 2 - 6 Reserved

extern "C" {
    pub fn w1_ds2781_eeprom_cmd(dev: *mut device, addr: c_int, cmd: c_int) -> c_int;
}
