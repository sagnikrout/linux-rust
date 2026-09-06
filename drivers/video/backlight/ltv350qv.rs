//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/backlight/ltv350qv.h
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
// Register definitions for Samsung LTV350QV Quarter VGA LCD Panel
//
// Copyright (C) 2006, 2007 Atmel Corporation
//
pub const LTV_OPC_INDEX: c_uint = 0x74;
pub const LTV_OPC_DATA: c_uint = 0x76;
pub const LTV_ID: c_uint = 0x00		/* ID Read */;
pub const LTV_IFCTL: c_uint = 0x01		/* Display Interface Control */;
pub const LTV_DATACTL: c_uint = 0x02		/* Display Data Control */;
pub const LTV_ENTRY_MODE: c_uint = 0x03		/* Entry Mode */;
pub const LTV_GATECTL1: c_uint = 0x04		/* Gate Control 1 */;
pub const LTV_GATECTL2: c_uint = 0x05		/* Gate Control 2 */;
pub const LTV_VBP: c_uint = 0x06		/* Vertical Back Porch */;
pub const LTV_HBP: c_uint = 0x07		/* Horizontal Back Porch */;
pub const LTV_SOTCTL: c_uint = 0x08		/* Source Output Timing Control */;
pub const LTV_PWRCTL1: c_uint = 0x09		/* Power Control 1 */;
pub const LTV_PWRCTL2: c_uint = 0x0a		/* Power Control 2 */;

// Bit definitions for LTV_IFCTL

// Bit definitions for LTV_DATACTL

// Bit definitions for LTV_ENTRY_MODE

// Bit definitions for LTV_GATECTL1

// Bit definitions for LTV_GATECTL2

// Bit definitions for LTV_SOTCTL

// Bit definitions for LTV_PWRCTL1

// Bit definitions for LTV_PWRCTL2

