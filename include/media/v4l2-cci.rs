//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-cci.h
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
// MIPI Camera Control Interface (CCI) register access helpers.
//
// Copyright (C) 2023 Hans de Goede <hansg@kernel.org>
//

//
// struct cci_reg_sequence - An individual write from a sequence of CCI writes
//
// @reg: Register address, use CCI_REG#() macros to encode reg width
// @val: Register value
//
// Register/value pairs for sequences of writes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cci_reg_sequence {
    pub reg: u32,
    pub val: u64,
}

//
// Macros to define register address with the register width encoded
// into the higher bits.
//

pub const CCI_REG_WIDTH_SHIFT: c_int = 16;

//
// Private CCI register flags, for the use of drivers.
//

//
// cci_read() - Read a value from a single CCI register
//
// @map: Register map to read from
// @reg: Register address to read, use CCI_REG#() macros to encode reg width
// @val: Pointer to store read value
// @err: Optional pointer to store errors, if a previous error is set
// then the read will be skipped
//
// Return: %0 on success or a negative error code on failure.
//
extern "C" {
    pub fn cci_read(map: *mut regmap, reg: u32, val: *mut u64, err: *mut c_int) -> c_int;
}
//
// cci_write() - Write a value to a single CCI register
//
// @map: Register map to write to
// @reg: Register address to write, use CCI_REG#() macros to encode reg width
// @val: Value to be written
// @err: Optional pointer to store errors, if a previous error is set
// then the write will be skipped
//
// Return: %0 on success or a negative error code on failure.
//
extern "C" {
    pub fn cci_write(map: *mut regmap, reg: u32, val: u64, err: *mut c_int) -> c_int;
}
//
// cci_update_bits() - Perform a read/modify/write cycle on
// a single CCI register
//
// @map: Register map to update
// @reg: Register address to update, use CCI_REG#() macros to encode reg width
// @mask: Bitmask to change
// @val: New value for bitmask
// @err: Optional pointer to store errors, if a previous error is set
// then the update will be skipped
//
// Note this uses read-modify-write to update the bits, atomicity with regards
// to other cci_*() register access functions is NOT guaranteed.
//
// Return: %0 on success or a negative error code on failure.
//
extern "C" {
    pub fn cci_update_bits(map: *mut regmap, reg: u32, mask: u64, val: u64, err: *mut c_int) -> c_int;
}
//
// cci_multi_reg_write() - Write multiple registers to the device
//
// @map: Register map to write to
// @regs: Array of structures containing register-address, -value pairs to be
// written, register-addresses use CCI_REG#() macros to encode reg width
// @num_regs: Number of registers to write
// @err: Optional pointer to store errors, if a previous error is set
// then the write will be skipped
//
// Write multiple registers to the device where the set of register, value
// pairs are supplied in any order, possibly not all in a single range.
//
// Use of the CCI_REG#() macros to encode reg width is mandatory.
//
// For raw lists of register-address, -value pairs with only 8 bit
// wide writes regmap_multi_reg_write() can be used instead.
//
// Return: %0 on success or a negative error code on failure.
//

//
// devm_cci_regmap_init_i2c() - Create regmap to use with cci_*() register
// access functions
//
// @client: i2c_client to create the regmap for
// @reg_addr_bits: register address width to use (8 or 16)
//
// Note the memory for the created regmap is devm() managed, tied to the client.
//
// Return: %0 on success or a negative error code on failure.
//

