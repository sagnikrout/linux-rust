//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dax/bus.h
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
// Copyright(c) 2016 - 2018 Intel Corporation. All rights reserved.

// dax bus specific ioresource flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_dax_data {
    pub dax_region: *mut dax_region,
    pub pgmap: *mut dev_pagemap,
    pub size: resource_size_t,
    pub id: c_int,
    pub memmap_on_memory: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dax_driver_type {
    DAXDRV_KMEM_TYPE,
    DAXDRV_DEVICE_TYPE,
    DAXDRV_FSDEV_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dax_device_driver {
    pub drv: device_driver,
    pub ids: list_head,
    pub type: dax_driver_type,
    pub dev): *mut *mut int (probe)(struct dev_dax,
    pub dev): *mut *mut void (remove)(struct dev_dax,
}

extern "C" {
    pub fn dax_driver_unregister(dax_drv: *mut dax_device_driver);
}
extern "C" {
    pub fn kill_dev_dax(dev_dax: *mut dev_dax);
}
extern "C" {
    pub fn static_dev_dax(dev_dax: *mut dev_dax) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmem_platform_device {
    pub pdev: platform_device,
    pub work: work_struct,
    pub did_probe: bool,
}

extern "C" {
    pub fn container_of(_arg: pdev, hmem_platform_device: struct, _arg: pdev) -> return;
}

extern "C" {
    pub fn dax_hmem_flush_work();
}

