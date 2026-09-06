//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amba/mmci.h
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
// include/linux/amba/mmci.h
//

//
// struct mmci_platform_data - platform configuration for the MMCI
// (also known as PL180) block.
// @ocr_mask: available voltages on the 4 pins from the block, this
// is ignored if a regulator is used, see the MMC_VDD_* masks in
// mmc/host.h
// @status: if no GPIO line was given to the block in this function will
// be called to determine whether a card is present in the MMC slot or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmci_platform_data {
    pub ocr_mask: c_uint,
    pub ): *mut *mut unsigned int (status)(struct device,
}
