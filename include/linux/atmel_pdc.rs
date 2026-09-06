//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/atmel_pdc.h
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
// include/linux/atmel_pdc.h
//
// Copyright (C) 2005 Ivan Kokshaysky
// Copyright (C) SAN People
//
// Peripheral Data Controller (PDC) registers.
// Based on AT91RM9200 datasheet revision E.
//
pub const ATMEL_PDC_RPR: c_uint = 0x100	/* Receive Pointer Register */;
pub const ATMEL_PDC_RCR: c_uint = 0x104	/* Receive Counter Register */;
pub const ATMEL_PDC_TPR: c_uint = 0x108	/* Transmit Pointer Register */;
pub const ATMEL_PDC_TCR: c_uint = 0x10c	/* Transmit Counter Register */;
pub const ATMEL_PDC_RNPR: c_uint = 0x110	/* Receive Next Pointer Register */;
pub const ATMEL_PDC_RNCR: c_uint = 0x114	/* Receive Next Counter Register */;
pub const ATMEL_PDC_TNPR: c_uint = 0x118	/* Transmit Next Pointer Register */;
pub const ATMEL_PDC_TNCR: c_uint = 0x11c	/* Transmit Next Counter Register */;
pub const ATMEL_PDC_PTCR: c_uint = 0x120	/* Transfer Control Register */;

pub const ATMEL_PDC_PTSR: c_uint = 0x124	/* Transfer Status Register */;
pub const ATMEL_PDC_SCND_BUF_OFF: c_uint = 0x10	/* Offset between first and second buffer registers */;
