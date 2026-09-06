//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mei_aux.h
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
// Copyright (c) 2022, Intel Corporation. All rights reserved.
//

//
// struct mei_aux_device - mei auxiliary device
// @aux_dev: - auxiliary device object
// @irq: interrupt driving the mei auxiliary device
// @bar: mmio resource bar reserved to mei auxiliary device
// @ext_op_mem: resource for extend operational memory
// used in graphics PXP mode.
// @slow_firmware: The device has slow underlying firmware.
// Such firmware will require to use larger operation timeouts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_aux_device {
    pub aux_dev: auxiliary_device,
    pub irq: c_int,
    pub bar: resource,
    pub ext_op_mem: resource,
    pub slow_firmware: bool,
}

