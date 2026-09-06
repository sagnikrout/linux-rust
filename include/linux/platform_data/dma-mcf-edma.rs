//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/dma-mcf-edma.h
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
// Freescale eDMA platform data, ColdFire SoC's family.
//
// Copyright (c) 2017 Angelo Dureghello <angelo@sysam.it>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
extern "C" {
    pub fn mcf_edma_filter_fn(chan: *mut dma_chan, param: *mut c_void) -> bool;
}

//
// struct mcf_edma_platform_data - platform specific data for eDMA engine
//
// @dma_channels:	The number of eDMA channels.
// @slave_map:		Slave device map
// @slavecnt:		Number of entries in @slave_map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcf_edma_platform_data {
    pub dma_channels: c_int,
    pub slave_map: *const dma_slave_map,
    pub slavecnt: c_int,
}
