//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/altera-a10sr.h
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
// Copyright Intel Corporation (C) 2014-2016. All Rights Reserved
//
// Declarations for Altera Arria10 MAX5 System Resource Chip
//
// Adapted from DA9052
//

// Write registers are always on even addresses
pub const WRITE_REG_MASK: c_uint = 0xFE;
// Odd registers are always on odd addresses
pub const READ_REG_MASK: c_uint = 0x01;
pub const ALTR_A10SR_BITS_PER_REGISTER: c_int = 8;
//
// To find the correct register, we divide the input GPIO by
// the number of GPIO in each register. We then need to multiply
// by 2 because the reads are at odd addresses.
//

// Arria10 System Controller Register Defines
pub const ALTR_A10SR_NOP: c_uint = 0x00    /* No Change */;
pub const ALTR_A10SR_VERSION_READ: c_uint = 0x00    /* MAX5 Version Read */;
pub const ALTR_A10SR_LED_REG: c_uint = 0x02    /* LED - Upper 4 bits */;
// LED register Bit Definitions

pub const ALTR_A10SR_OUT_VALID_RANGE_HI: c_int = 7;
pub const ALTR_A10SR_PBDSW_REG: c_uint = 0x04    /* PB & DIP SW - Input only */;
pub const ALTR_A10SR_PBDSW_IRQ_REG: c_uint = 0x06    /* PB & DIP SW Flag Clear */;
// Pushbutton & DIP Switch Bit Definitions
pub const ALTR_A10SR_IN_VALID_RANGE_LO: c_int = 8;
pub const ALTR_A10SR_IN_VALID_RANGE_HI: c_int = 15;
pub const ALTR_A10SR_PWR_GOOD1_REG: c_uint = 0x08    /* Power Good1 Read */;
pub const ALTR_A10SR_PWR_GOOD2_REG: c_uint = 0x0A    /* Power Good2 Read */;
pub const ALTR_A10SR_PWR_GOOD3_REG: c_uint = 0x0C    /* Power Good3 Read */;
pub const ALTR_A10SR_FMCAB_REG: c_uint = 0x0E    /* FMCA/B & PCIe Pwr Enable */;
pub const ALTR_A10SR_HPS_RST_REG: c_uint = 0x10    /* HPS Reset */;
pub const ALTR_A10SR_USB_QSPI_REG: c_uint = 0x12    /* USB, BQSPI, FILE Reset */;
pub const ALTR_A10SR_SFPA_REG: c_uint = 0x14    /* SFPA Control Reg */;
pub const ALTR_A10SR_SFPB_REG: c_uint = 0x16    /* SFPB Control Reg */;
pub const ALTR_A10SR_I2C_M_REG: c_uint = 0x18    /* I2C Master Select */;
pub const ALTR_A10SR_WARM_RST_REG: c_uint = 0x1A    /* HPS Warm Reset */;
pub const ALTR_A10SR_WR_KEY_REG: c_uint = 0x1C    /* HPS Warm Reset Key */;
pub const ALTR_A10SR_PMBUS_REG: c_uint = 0x1E    /* HPS PM Bus */;
//
// struct altr_a10sr - Altera Max5 MFD device private data structure
// @dev:  : this device
// @regmap: the regmap assigned to the parent device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altr_a10sr {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}
