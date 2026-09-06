//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_mem_ops.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more
// details.
//
// !  \file octeon_mem_ops.h
// \brief Host Driver: Routines used to read/write Octeon memory.
//
// Read a 64-bit value from a BAR1 mapped core memory address.
// @param  oct        -  pointer to the octeon device.
// @param  core_addr  -  the address to read from.
//
// The range_idx gives the BAR1 index register for the range of address
// in which core_addr is mapped.
//
// @return  64-bit value read from Core memory
//
extern "C" {
    pub fn octeon_read_device_mem64(oct: *mut octeon_device, core_addr: u64) -> u64;
}
// Read a 32-bit value from a BAR1 mapped core memory address.
// @param  oct        -  pointer to the octeon device.
// @param  core_addr  -  the address to read from.
//
// @return  32-bit value read from Core memory
//
extern "C" {
    pub fn octeon_read_device_mem32(oct: *mut octeon_device, core_addr: u64) -> u32;
}
// Write a 32-bit value to a BAR1 mapped core memory address.
// @param  oct        -  pointer to the octeon device.
// @param  core_addr  -  the address to write to.
// @param  val        -  32-bit value to write.
//
// Read multiple bytes from Octeon memory.
//
// Write multiple bytes into Octeon memory.
//
