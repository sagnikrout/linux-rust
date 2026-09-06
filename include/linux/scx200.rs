//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/scx200.h
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
// linux/include/linux/scx200.h
//
// Interesting stuff for the National Semiconductor SCx200 CPU

// F0 PCI Header/Bridge Configuration Registers
pub const SCx200_DOCCS_BASE: c_uint = 0x78	/* DOCCS Base Address Register */;
pub const SCx200_DOCCS_CTRL: c_uint = 0x7c	/* DOCCS Control Register */;
// GPIO Register Block
pub const SCx200_GPIO_SIZE: c_uint = 0x2c	/* Size of GPIO register block */;
// General Configuration Block
pub const SCx200_CB_BASE_FIXED: c_uint = 0x9000	/* Base fixed at 0x9000 according to errata? */;
// Watchdog Timer
pub const SCx200_WDT_OFFSET: c_uint = 0x00	/* offset within configuration block */;
pub const SCx200_WDT_SIZE: c_uint = 0x05	/* size */;
pub const SCx200_WDT_WDTO: c_uint = 0x00	/* Time-Out Register */;
pub const SCx200_WDT_WDCNFG: c_uint = 0x02	/* Configuration Register */;
pub const SCx200_WDT_WDSTS: c_uint = 0x04	/* Status Register */;

// High Resolution Timer
pub const SCx200_TIMER_OFFSET: c_uint = 0x08;
pub const SCx200_TIMER_SIZE: c_uint = 0x06;
// Clock Generators
pub const SCx200_CLOCKGEN_OFFSET: c_uint = 0x10;
pub const SCx200_CLOCKGEN_SIZE: c_uint = 0x10;
// Pin Multiplexing and Miscellaneous Configuration Registers
pub const SCx200_MISC_OFFSET: c_uint = 0x30;
pub const SCx200_MISC_SIZE: c_uint = 0x10;
pub const SCx200_PMR: c_uint = 0x30		/* Pin Multiplexing Register */;
pub const SCx200_MCR: c_uint = 0x34		/* Miscellaneous Configuration Register */;
pub const SCx200_INTSEL: c_uint = 0x38	/* Interrupt Selection Register */;
pub const SCx200_IID: c_uint = 0x3c		/* IA On a Chip Identification Number Reg */;
pub const SCx200_REV: c_uint = 0x3d		/* Revision Register */;
pub const SCx200_CBA: c_uint = 0x3e		/* Configuration Base Address Register */;
pub const SCx200_CBA_SCRATCH: c_uint = 0x64	/* Configuration Base Address Scratchpad */;
