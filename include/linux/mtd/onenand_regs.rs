//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/onenand_regs.h
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
// linux/include/linux/mtd/onenand_regs.h
//
// OneNAND Register header file
//
// Copyright (C) 2005-2007 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
//
// Memory Address Map Translation (Word order)

//
// External BufferRAM area
//

//
// OneNAND Registers
//

//
// Device ID Register F001h (R)
//

//
// Version ID Register F002h (R)
//

//
// Technology Register F006h (R)
//

//
// Start Address 1 F100h (R/W) & Start Address 2 F101h (R/W)
//

//
// Start Address 8 F107h (R/W)
//
// Note: It's actually 0x3f in case of SLC

//
// Start Buffer Register F200h (R/W)
//

// Note: It's actually 0x03 in case of SLC

//
// Command Register F220h (R/W)
//

// NOTE: Those are not *REAL* commands

//
// System Configuration 1 Register F221h (R, R/W)
//

//
// Controller Status Register F240h (R)
//

//
// Interrupt Status Register F241h (R)
//

//
// NAND Flash Write Protection Status Register F24Eh (R)
//

//
// ECC Status Reigser FF00h (R)
//

//
// One-Time Programmable (OTP)
//

