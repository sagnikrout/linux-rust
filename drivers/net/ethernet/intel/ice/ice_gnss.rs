//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_gnss.h
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
// Copyright (C) 2021-2022, Intel Corporation.
pub const ICE_E810T_GNSS_I2C_BUS: c_uint = 0x2;

pub const ICE_GNSS_TTY_WRITE_BUF: c_int = 250;

pub const ICE_MAX_I2C_WRITE_BYTES: c_int = 4;
// u-blox ZED-F9T specific definitions
pub const ICE_GNSS_UBX_I2C_BUS_ADDR: c_uint = 0x42;
// Data length register is big endian
pub const ICE_GNSS_UBX_DATA_LEN_H: c_uint = 0xFD;
pub const ICE_GNSS_UBX_DATA_LEN_WIDTH: c_int = 2;
pub const ICE_GNSS_UBX_EMPTY_DATA: c_uint = 0xFF;
// For u-blox writes are performed without address so the first byte to write is
// passed as I2C addr parameter.
//

//
// struct gnss_serial - data used to initialize GNSS TTY port
// @back: back pointer to PF
// @kworker: kwork thread for handling periodic work
// @read_work: read_work function for handling GNSS reads
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnss_serial {
    pub back: *mut ice_pf,
    pub kworker: *mut kthread_worker,
    pub read_work: kthread_delayed_work,
}

extern "C" {
    pub fn ice_gnss_init(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_gnss_exit(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_gnss_is_module_present(hw: *mut ice_hw) -> bool;
}

