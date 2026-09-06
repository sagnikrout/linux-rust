//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-bus.h
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
// Copyright (C) 2013 - 2024 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_bus_device {
    pub auxdev: auxiliary_device,
    pub auxdrv: *const auxiliary_driver,
    pub auxdrv_data: *const ipu6_auxdrv_data,
    pub list: list_head,
    pub pdata: *mut c_void,
    pub mmu: *mut ipu6_mmu,
    pub isp: *mut ipu6_device,
    pub ctrl: *const ipu6_buttress_ctrl,
    pub fw: *const firmware,
    pub fw_sgt: sg_table,
    pub pkg_dir: *mut u64,
    pub pkg_dir_dma_addr: dma_addr_t,
    pub pkg_dir_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_auxdrv_data {
    pub adev): *mut *mut irqreturn_t (isr)(struct ipu6_bus_device,
    pub adev): *mut *mut irqreturn_t (isr_threaded)(struct ipu6_bus_device,
    pub wake_isr_thread: bool,
}

extern "C" {
    pub fn ipu6_bus_add_device(adev: *mut ipu6_bus_device) -> c_int;
}
extern "C" {
    pub fn ipu6_bus_del_devices(pdev: *mut pci_dev);
}
