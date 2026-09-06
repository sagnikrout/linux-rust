//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/ipmi/kcs_bmc_device.h
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
// Copyright (c) 2021, IBM Corp.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcs_bmc_device_ops {
    pub enable): *mut *mut *mut void (irq_mask_update)(struct kcs_bmc_device kcs_bmc, u8 mask, u8,
    pub reg): *mut *mut *mut u8 (io_inputb)(struct kcs_bmc_device kcs_bmc, u32,
    pub b): *mut *mut *mut void (io_outputb)(struct kcs_bmc_device kcs_bmc, u32 reg, u8,
    pub b): *mut *mut *mut void (io_updateb)(struct kcs_bmc_device kcs_bmc, u32 reg, u8 mask, u8,
}

extern "C" {
    pub fn kcs_bmc_handle_event(kcs_bmc: *mut kcs_bmc_device) -> irqreturn_t;
}
extern "C" {
    pub fn kcs_bmc_add_device(kcs_bmc: *mut kcs_bmc_device) -> c_int;
}
extern "C" {
    pub fn kcs_bmc_remove_device(kcs_bmc: *mut kcs_bmc_device);
}
