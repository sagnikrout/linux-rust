//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/cn23xx_vf_device.h
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
pub struct octeon_cn23xx_vf {
    pub conf: *mut octeon_config,
}

pub const BUSY_READING_REG_VF_LOOP_COUNT: c_int = 10000;
pub const CN23XX_MAILBOX_MSGPARAM_SIZE: c_int = 6;
extern "C" {
    pub fn cn23xx_vf_ask_pf_to_do_flr(oct: *mut octeon_device);
}
extern "C" {
    pub fn cn23xx_octeon_pfvf_handshake(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn cn23xx_setup_octeon_vf_device(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn cn23xx_vf_get_oq_ticks(oct: *mut octeon_device, time_intr_in_us: u32) -> u32;
}
