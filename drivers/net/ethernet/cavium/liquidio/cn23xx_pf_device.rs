//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/cn23xx_pf_device.h
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
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// ! \file  cn23xx_device.h
// \brief Host Driver: Routines that perform CN23XX specific operations.
//

// Register address and configuration for a CN23XX devices.
// If device specific changes need to be made then add a struct to include
// device specific fields as shown in the commented section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_cn23xx_pf {
// PCI interrupt summary register
    pub intr_sum_reg64: *mut u8 __iomem,
// PCI interrupt enable register
    pub intr_enb_reg64: *mut u8 __iomem,
// The PCI interrupt mask used by interrupt handler
    pub intr_mask64: u64,
    pub conf: *mut octeon_config,
}

pub const CN23XX_SLI_DEF_BP: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_vf_stats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub broadcast: u64,
    pub multicast: u64,
}

extern "C" {
    pub fn setup_cn23xx_octeon_pf_device(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn cn23xx_pf_get_oq_ticks(oct: *mut octeon_device, time_intr_in_us: u32) -> u32;
}
extern "C" {
    pub fn cn23xx_sriov_config(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn cn23xx_fw_loaded(oct: *mut octeon_device) -> c_int;
}
