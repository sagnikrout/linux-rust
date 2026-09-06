//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/spi-omap2-mcspi.h
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
pub const OMAP4_MCSPI_REG_OFFSET: c_uint = 0x100;
pub const MCSPI_PINDIR_D0_IN_D1_OUT: c_int = 0;
pub const MCSPI_PINDIR_D0_OUT_D1_IN: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap2_mcspi_platform_config {
    pub num_cs: c_ushort,
    pub regs_offset: c_uint,
    pub pin_dir:1: c_uint,
    pub max_xfer_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap2_mcspi_device_config {
    pub turbo_mode:1: unsigned,
}
