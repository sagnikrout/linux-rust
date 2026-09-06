//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mlxcpld.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Mellanox I2C multiplexer support in CPLD
//
// Copyright (C) 2016-2020 Mellanox Technologies
//
// Platform data for the CPLD I2C multiplexers
// mlxcpld_mux_plat_data - per mux data, used with i2c_register_board_info
// @chan_ids - channels array
// @num_adaps - number of adapters
// @sel_reg_addr - mux select register offset in CPLD space
// @reg_size: register size in bytes
// @handle: handle to be passed by callback
// @completion_notify: callback to notify when all the adapters are created
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxcpld_mux_plat_data {
    pub chan_ids: *mut c_int,
    pub num_adaps: c_int,
    pub sel_reg_addr: c_int,
    pub reg_size: u8,
    pub handle: *mut c_void,
    pub adapters[]): *mut i2c_adapter,
}
