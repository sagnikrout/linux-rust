//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/jz4780-nemc.h
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
// JZ4780 NAND/external memory controller (NEMC)
//
// Copyright (c) 2015 Imagination Technologies
// Author: Alex Smith <alex@alex-smith.me.uk>
//

//
// Number of NEMC banks. Note that there are actually 6, but they are numbered
// from 1.
//
pub const JZ4780_NEMC_NUM_BANKS: c_int = 7;
//
// enum jz4780_nemc_bank_type - device types which can be connected to a bank
// @JZ4780_NEMC_BANK_SRAM: SRAM
// @JZ4780_NEMC_BANK_NAND: NAND
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jz4780_nemc_bank_type {
    JZ4780_NEMC_BANK_SRAM,
    JZ4780_NEMC_BANK_NAND,
}

extern "C" {
    pub fn jz4780_nemc_num_banks(dev: *mut device) -> c_uint;
}
