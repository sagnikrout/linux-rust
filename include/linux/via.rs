//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/via.h
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
// Miscellaneous definitions for VIA chipsets
// Values for SuperIO function select configuration register
pub const VIA_FUNCTION_PARPORT_SPP: c_uint = 0x00;
pub const VIA_FUNCTION_PARPORT_ECP: c_uint = 0x01;
pub const VIA_FUNCTION_PARPORT_EPP: c_uint = 0x02;
pub const VIA_FUNCTION_PARPORT_DISABLE: c_uint = 0x03;
pub const VIA_FUNCTION_PROBE: c_uint = 0xFF /* Special magic value to be used in code, not to be written into chip */;
// Bits for parallel port mode configuration register

pub const VIA_PARPORT_BIDIR: c_uint = 0x80;
// VIA configuration registers
pub const VIA_CONFIG_INDEX: c_uint = 0x3F0;
pub const VIA_CONFIG_DATA: c_uint = 0x3F1;
// Mask for parallel port IRQ bits (in ISA PnP IRQ routing register 1)
pub const VIA_IRQCONTROL_PARALLEL: c_uint = 0xF0;
// Mask for parallel port DMA bits (in ISA PnP DMA routing register)
pub const VIA_DMACONTROL_PARALLEL: c_uint = 0x0C;
