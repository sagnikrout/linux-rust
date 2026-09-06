//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igbvf/regs.h
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
// Copyright(c) 2009 - 2018 Intel Corporation.
pub const E1000_CTRL: c_uint = 0x00000 /* Device Control - RW */;
pub const E1000_STATUS: c_uint = 0x00008 /* Device Status - RO */;
pub const E1000_ITR: c_uint = 0x000C4 /* Interrupt Throttling Rate - RW */;
pub const E1000_EICR: c_uint = 0x01580 /* Ext. Interrupt Cause Read - R/clr */;

pub const E1000_EICS: c_uint = 0x01520 /* Ext. Interrupt Cause Set - W0 */;
pub const E1000_EIMS: c_uint = 0x01524 /* Ext. Interrupt Mask Set/Read - RW */;
pub const E1000_EIMC: c_uint = 0x01528 /* Ext. Interrupt Mask Clear - WO */;
pub const E1000_EIAC: c_uint = 0x0152C /* Ext. Interrupt Auto Clear - RW */;
pub const E1000_EIAM: c_uint = 0x01530 /* Ext. Interrupt Ack Auto Clear Mask - RW */;
pub const E1000_IVAR0: c_uint = 0x01700 /* Interrupt Vector Allocation (array) - RW */;
pub const E1000_IVAR_MISC: c_uint = 0x01740 /* IVAR for "other" causes - RW */;
// Convenience macros
//
// Note: "_n" is the queue number of the register to be written to.
//
// Example usage:
// E1000_RDBAL_REG(current_rx_queue)
//

// Statistics registers
pub const E1000_VFGPRC: c_uint = 0x00F10;
pub const E1000_VFGORC: c_uint = 0x00F18;
pub const E1000_VFMPRC: c_uint = 0x00F3C;
pub const E1000_VFGPTC: c_uint = 0x00F14;
pub const E1000_VFGOTC: c_uint = 0x00F34;
pub const E1000_VFGOTLBC: c_uint = 0x00F50;
pub const E1000_VFGPTLBC: c_uint = 0x00F44;
pub const E1000_VFGORLBC: c_uint = 0x00F48;
pub const E1000_VFGPRLBC: c_uint = 0x00F40;
// These act per VF so an array friendly macro is used

// Define macros for handling registers

