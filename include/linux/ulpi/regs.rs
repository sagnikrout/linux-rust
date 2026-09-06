//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ulpi/regs.h
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
//
// Macros for Set and Clear
// See ULPI 1.1 specification to find the registers with Set and Clear offsets
//

//
// Register Map
//
pub const ULPI_VENDOR_ID_LOW: c_uint = 0x00;
pub const ULPI_VENDOR_ID_HIGH: c_uint = 0x01;
pub const ULPI_PRODUCT_ID_LOW: c_uint = 0x02;
pub const ULPI_PRODUCT_ID_HIGH: c_uint = 0x03;
pub const ULPI_FUNC_CTRL: c_uint = 0x04;
pub const ULPI_IFC_CTRL: c_uint = 0x07;
pub const ULPI_OTG_CTRL: c_uint = 0x0a;
pub const ULPI_USB_INT_EN_RISE: c_uint = 0x0d;
pub const ULPI_USB_INT_EN_FALL: c_uint = 0x10;
pub const ULPI_USB_INT_STS: c_uint = 0x13;
pub const ULPI_USB_INT_LATCH: c_uint = 0x14;
pub const ULPI_DEBUG: c_uint = 0x15;
pub const ULPI_SCRATCH: c_uint = 0x16;
// Optional Carkit Registers
pub const ULPI_CARKIT_CTRL: c_uint = 0x19;
pub const ULPI_CARKIT_INT_DELAY: c_uint = 0x1c;
pub const ULPI_CARKIT_INT_EN: c_uint = 0x1d;
pub const ULPI_CARKIT_INT_STS: c_uint = 0x20;
pub const ULPI_CARKIT_INT_LATCH: c_uint = 0x21;
pub const ULPI_CARKIT_PLS_CTRL: c_uint = 0x22;
// Other Optional Registers
pub const ULPI_TX_POS_WIDTH: c_uint = 0x25;
pub const ULPI_TX_NEG_WIDTH: c_uint = 0x26;
pub const ULPI_POLARITY_RECOVERY: c_uint = 0x27;
// Access Extended Register Set
pub const ULPI_ACCESS_EXTENDED: c_uint = 0x2f;
// Vendor Specific
pub const ULPI_VENDOR_SPECIFIC: c_uint = 0x30;
// Extended Registers
pub const ULPI_EXT_VENDOR_SPECIFIC: c_uint = 0x80;
//
// Register Bits
//
// Function Control

pub const ULPI_FUNC_CTRL_XCVRSEL_MASK: c_uint = 0x3;
pub const ULPI_FUNC_CTRL_HIGH_SPEED: c_uint = 0x0;
pub const ULPI_FUNC_CTRL_FULL_SPEED: c_uint = 0x1;
pub const ULPI_FUNC_CTRL_LOW_SPEED: c_uint = 0x2;
pub const ULPI_FUNC_CTRL_FS4LS: c_uint = 0x3;

// Interface Control

// OTG Control

// USB Interrupt Enable Rising,
// USB Interrupt Enable Falling,
// USB Interrupt Status and
// USB Interrupt Latch
//

// Debug

// Carkit Control

// Carkit Interrupt Enable

// Carkit Interrupt Status and
// Carkit Interrupt Latch
//

// Carkit Pulse Control

