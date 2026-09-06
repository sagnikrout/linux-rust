//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/smsc/smsc9420.h
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
// Copyright (C) 2007,2008  SMSC
//

// interrupt deassertion in multiples of 10us

// Register set is duplicated for BE at an offset of 0x200

//
// DMA Controller Control and Status Registers
//

// Transmit Descriptor Bit Defs

pub const TDES1_IC_: c_uint = 0x80000000;
pub const TDES1_LS_: c_uint = 0x40000000;
pub const TDES1_FS_: c_uint = 0x20000000;
pub const TDES1_TXCSEN_: c_uint = 0x08000000;

pub const TDES1_TCH_: c_uint = 0x01000000;
// Receive Descriptor 0 Bit Defs

// Receive Descriptor 1 Bit Defs

//
// MAC Control and Status Registers
//

//
// System Control and Status Registers
//

