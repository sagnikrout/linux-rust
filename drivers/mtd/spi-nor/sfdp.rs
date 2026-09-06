//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/spi-nor/sfdp.h
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
// Copyright (C) 2005, Intec Automation Inc.
// Copyright (C) 2014, Freescale Semiconductor, Inc.
//
// SFDP revisions
pub const SFDP_JESD216_MAJOR: c_int = 1;
pub const SFDP_JESD216_MINOR: c_int = 0;
pub const SFDP_JESD216A_MINOR: c_int = 5;
pub const SFDP_JESD216B_MINOR: c_int = 6;
// SFDP DWORDS are indexed from 1 but C arrays are indexed from 0.

// Basic Flash Parameter Table
// JESD216 rev D defines a Basic Flash Parameter Table of 20 DWORDs.
pub const BFPT_DWORD_MAX: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfdp_bfpt {
    pub dwords: [u32; BFPT_DWORD_MAX],
}

// The first version of JESD216 defined only 9 DWORDs.
pub const BFPT_DWORD_MAX_JESD216: c_int = 9;
pub const BFPT_DWORD_MAX_JESD216B: c_int = 16;
// 1st DWORD.

// 5th DWORD.

// 11th DWORD.
pub const BFPT_DWORD11_PAGE_SIZE_SHIFT: c_int = 4;

// 15th DWORD.
//
// (from JESD216 rev B)
// Quad Enable Requirements (QER):
// - 000b: Device does not have a QE bit. Device detects 1-1-4 and 1-4-4
// reads based on instruction. DQ3/HOLD# functions are hold during
// instruction phase.
// - 001b: QE is bit 1 of status register 2. It is set via Write Status with
// two data bytes where bit 1 of the second byte is one.
// [...]
// Writing only one byte to the status register has the side-effect of
// clearing status register 2, including the QE bit. The 100b code is
// used if writing one byte to the status register does not modify
// status register 2.
// - 010b: QE is bit 6 of status register 1. It is set via Write Status with
// one data byte where bit 6 is one.
// [...]
// - 011b: QE is bit 7 of status register 2. It is set via Write status
// register 2 instruction 3Eh with one data byte where bit 7 is one.
// [...]
// The status register 2 is read using instruction 3Fh.
// - 100b: QE is bit 1 of status register 2. It is set via Write Status with
// two data bytes where bit 1 of the second byte is one.
// [...]
// In contrast to the 001b code, writing one byte to the status
// register does not modify status register 2.
// - 101b: QE is bit 1 of status register 2. Status register 1 is read using
// Read Status instruction 05h. Status register2 is read using
// instruction 35h. QE is set via Write Status instruction 01h with
// two data bytes where bit 1 of the second byte is one.
// [...]
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfdp_parameter_header {
    pub id_lsb: u8,
    pub minor: u8,
    pub major: u8,
    pub /: *mut *mut u8 length; / in double words,
    pub /: *mut *mut u8 parameter_table_pointer[3]; / byte address,
    pub id_msb: u8,
}
