//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_sram_mgr.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).

pub const CC_CC_SRAM_SIZE: c_int = 4096;

//
// cc_sram_mgr_init() - Initializes SRAM pool.
// The first X bytes of SRAM are reserved for ROM usage, hence, pool
// starts right after X bytes.
//
// @drvdata: Associated device driver context
//
// Return:
// Zero for success, negative value otherwise.
//
extern "C" {
    pub fn cc_sram_mgr_init(drvdata: *mut cc_drvdata) -> c_int;
}
//
// cc_sram_alloc() - Allocate buffer from SRAM pool.
//
// @drvdata: Associated device driver context
// @size: The requested bytes to allocate
//
// Return:
// Address offset in SRAM or NULL_SRAM_ADDR for failure.
//
extern "C" {
    pub fn cc_sram_alloc(drvdata: *mut cc_drvdata, size: u32) -> u32;
}
//
// cc_set_sram_desc() - Create const descriptors sequence to
// set values in given array into SRAM.
// Note: each const value can't exceed word size.
//
// @src:	  A pointer to array of words to set as consts.
// @dst:	  The target SRAM buffer to set into
// @nelement:	  The number of words in "src" array
// @seq:	  A pointer to the given IN/OUT descriptor sequence
// @seq_len:	  A pointer to the given IN/OUT sequence length
//
