//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/i2c-ocores.h
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
// i2c-ocores.h - definitions for the i2c-ocores interface
//
// Peter Korsgaard <peter@korsgaard.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocores_i2c_platform_data {
    pub /: *mut *mut u32 reg_shift; / register offset shift value,
    pub /: *mut *mut u32 reg_io_width; / register io read/write width,
    pub /: *mut *mut u32 clock_khz; / input clock in kHz,
    pub /: *mut *mut u32 bus_khz; / bus clock in kHz,
    pub /: *mut *mut bool big_endian; / registers are big endian,
    pub /: *mut *mut u8 num_devices; / number of devices in the devices list,
    pub /: *const *const *const i2c_board_info devices; / devices connected to the bus,
}
