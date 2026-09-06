//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_diag.h
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
// Copyright (c)  2020 Intel Corporation
extern "C" {
    pub fn igc_reg_test(adapter: *mut igc_adapter, data: *mut u64) -> bool;
}
extern "C" {
    pub fn igc_eeprom_test(adapter: *mut igc_adapter, data: *mut u64) -> bool;
}
extern "C" {
    pub fn igc_link_test(adapter: *mut igc_adapter, data: *mut u64) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_reg_test {
    pub reg: u16,
    pub array_len: u8,
    pub test_type: u8,
    pub mask: u32,
    pub write: u32,
}

// In the hardware, registers are laid out either singly, in arrays
// spaced 0x40 bytes apart, or in contiguous tables.  We assume
// most tests take place on arrays or single registers (handled
// as a single-element array) and special-case the tables.
// Table tests are always pattern tests.
//
// We also make provision for some required setup steps by specifying
// registers to be written without any read-back testing.
//
pub const PATTERN_TEST: c_int = 1;
pub const SET_READ_TEST: c_int = 2;
pub const TABLE32_TEST: c_int = 3;
pub const TABLE64_TEST_LO: c_int = 4;
pub const TABLE64_TEST_HI: c_int = 5;
