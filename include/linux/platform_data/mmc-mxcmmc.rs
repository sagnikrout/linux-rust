//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mmc-mxcmmc.h
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

// board specific SDHC data, optional.
// If not present, a writable card with 3,3V is assumed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imxmmc_platform_data {
// Return values for the get_ro callback should be:
// 0 for a read/write card
// 1 for a read-only card
// -ENOSYS when not supported (equal to NULL callback)
// or a negative errno value when something bad happened
//
    pub ): *mut *mut int (get_ro)(struct device,
// board specific hook to (de)initialize the SD slot.
// The board code can call 'handler' on a card detection
// change giving data as argument.
//
    pub data): *mut *mut *mut int (init)(struct device dev, irq_handler_t handler, void,
    pub data): *mut *mut *mut void (exit)(struct device dev, void,
// available voltages. If not given, assume
// MMC_VDD_32_33 | MMC_VDD_33_34
//
    pub ocr_avail: c_uint,
// adjust slot voltage
    pub vdd): *mut *mut *mut void (setpower)(struct device , unsigned int,
// enable card detect using DAT3
    pub dat3_card_detect: c_int,
}
