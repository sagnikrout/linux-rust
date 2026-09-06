//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mmc-pxamci.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxamci_platform_data {
    pub /: *mut *mut unsigned int ocr_mask; / available voltages,
    pub /: *mut *mut unsigned long detect_delay_ms; / delay in millisecond before detecting cards after interrupt,
    pub ): *mut *mut *mut int (init)(struct device , irq_handler_t , void,
    pub ): *mut *mut int (get_ro)(struct device,
    pub int): *mut *mut *mut int (setpower)(struct device , unsigned,
    pub ): *mut *mut *mut void (exit)(struct device , void,
    pub /: *mut *mut bool gpio_card_ro_invert; / gpio ro is inverted,
}

extern "C" {
    pub fn pxa3xx_set_mci2_info(info: *mut pxamci_platform_data);
}
extern "C" {
    pub fn pxa3xx_set_mci3_info(info: *mut pxamci_platform_data);
}
