//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mtd-nand-pxa3xx.h
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
// Current pxa3xx_nand controller has two chip select which both be workable but
// historically all platforms remaining on platform data used only one. Switch
// to device tree if you need more.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa3xx_nand_platform_data {
// Keep OBM/bootloader NFC timing configuration
    pub keep_config: bool,
// Use a flash-based bad block table
    pub flash_bbt: bool,
// Requested ECC strength and ECC step size
    pub ecc_step_size: int ecc_strength,,
// Partitions
    pub parts: *const mtd_partition,
    pub nr_parts: c_uint,
}

extern "C" {
    pub fn pxa3xx_set_nand_info(info: *mut pxa3xx_nand_platform_data);
}
