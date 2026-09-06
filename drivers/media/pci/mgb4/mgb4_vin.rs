//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mgb4/mgb4_vin.h
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
// Copyright (C) 2021-2023 Digiteq Automotive
// author: Martin Tuma <martin.tuma@digiteqautomotive.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgb4_vin_regs {
    pub address: u32,
    pub config: u32,
    pub status: u32,
    pub resolution: u32,
    pub frame_period: u32,
    pub sync: u32,
    pub pclk: u32,
    pub hsync: u32,
    pub vsync: u32,
    pub padding: u32,
    pub timer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgb4_vin_config {
    pub id: c_int,
    pub dma_channel: c_int,
    pub vin_irq: c_int,
    pub err_irq: c_int,
    pub regs: mgb4_vin_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgb4_vin_dev {
    pub mgbdev: *mut mgb4_dev,
    pub v4l2dev: v4l2_device,
    pub vdev: video_device,
    pub queue: vb2_queue,
    pub /: *mut *mut mutex lock; / vdev lock,
    pub /: *mut *mut spinlock_t qlock; / video buffer queue lock,
    pub buf_list: list_head,
    pub err_work: work_dma_work,,
    pub sequence: c_uint,
    pub timings: v4l2_dv_timings,
    pub freq_range: u32,
    pub padding: u32,
    pub deser: mgb4_i2c_client,
    pub config: *const mgb4_vin_config,

    pub regset: debugfs_regset32,
    pub 4]: debugfs_reg32 regs[sizeof(mgb4_vin_regs) /,

}

extern "C" {
    pub fn mgb4_vin_free(vindev: *mut mgb4_vin_dev);
}
