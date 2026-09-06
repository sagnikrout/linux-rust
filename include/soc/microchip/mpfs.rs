//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/microchip/mpfs.h
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
// Microchip PolarFire SoC (MPFS)
//
// Copyright (c) 2020 Microchip Corporation. All rights reserved.
//
// Author: Conor Dooley <conor.dooley@microchip.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_mss_msg {
    pub cmd_opcode: u8,
    pub cmd_data_size: u16,
    pub response: *mut mpfs_mss_response,
    pub cmd_data: *mut u8,
    pub mbox_offset: u16,
    pub resp_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_mss_response {
    pub resp_status: u32,
    pub resp_msg: *mut u32,
    pub resp_size: u16,
}

extern "C" {
    pub fn mpfs_blocking_transaction(mpfs_client: *mut mpfs_sys_controller, msg: *mut mpfs_mss_msg) -> c_int;
}

extern "C" {
    pub fn mpfs_reset_controller_register(clk_dev: *mut device, map: *mut regmap) -> c_int;
}

