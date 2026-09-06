//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/tpm/st33zp24/st33zp24.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// STMicroelectronics TPM Linux driver for TPM ST33ZP24
// Copyright (C) 2009 - 2016  STMicroelectronics
//

pub const TPM_WRITE_DIRECTION: c_uint = 0x80;
pub const ST33ZP24_BUFSIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st33zp24_dev {
    pub chip: *mut tpm_chip,
    pub phy_id: *mut c_void,
    pub ops: *const st33zp24_phy_ops,
    pub locality: c_int,
    pub irq: c_int,
    pub intrs: u32,
    pub io_lpcpd: *mut gpio_desc,
    pub read_queue: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st33zp24_phy_ops {
    pub tpm_size): *mut *mut *mut *mut int (send)(void phy_id, u8 tpm_register, u8 tpm_data, int,
    pub tpm_size): *mut *mut *mut *mut int (recv)(void phy_id, u8 tpm_register, u8 tpm_data, int,
}

extern "C" {
    pub fn st33zp24_pm_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn st33zp24_pm_resume(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn st33zp24_remove(chip: *mut tpm_chip);
}
