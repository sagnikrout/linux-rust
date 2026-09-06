//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dfl.h
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
// Header file for DFL driver and device API
//
// Copyright (C) 2020 Intel Corporation, Inc.
//

//
// enum dfl_id_type - define the DFL FIU types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dfl_id_type {
    FME_ID = 0,
    PORT_ID = 1,
    DFL_ID_MAX,
}

//
// struct dfl_device - represent an dfl device on dfl bus
//
// @dev: generic device interface.
// @id: id of the dfl device.
// @type: type of DFL FIU of the device. See enum dfl_id_type.
// @feature_id: feature identifier local to its DFL FIU type.
// @revision: revision of this dfl device feature.
// @mmio_res: mmio resource of this dfl device.
// @irqs: list of Linux IRQ numbers of this dfl device.
// @num_irqs: number of IRQs supported by this dfl device.
// @cdev: pointer to DFL FPGA container device this dfl device belongs to.
// @id_entry: matched id entry in dfl driver's id table.
// @dfh_version: version of DFH for the device
// @param_size: size of the block parameters in bytes
// @params: pointer to block of parameters copied memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_device {
    pub dev: device,
    pub id: c_int,
    pub type: u16,
    pub feature_id: u16,
    pub revision: u8,
    pub mmio_res: resource,
    pub irqs: *mut c_int,
    pub num_irqs: c_uint,
    pub cdev: *mut dfl_fpga_cdev,
    pub id_entry: *const dfl_device_id,
    pub dfh_version: u8,
    pub param_size: c_uint,
    pub params: *mut c_void,
}

//
// struct dfl_driver - represent an dfl device driver
//
// @drv: driver model structure.
// @id_table: pointer to table of device IDs the driver is interested in.
// { } member terminated.
// @probe: mandatory callback for device binding.
// @remove: callback for device unbinding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_driver {
    pub drv: device_driver,
    pub id_table: *const dfl_device_id,
    pub dfl_dev): *mut *mut int (probe)(struct dfl_device,
    pub dfl_dev): *mut *mut void (remove)(struct dfl_device,
}

//
// use a macro to avoid include chaining to get THIS_MODULE.
//

extern "C" {
    pub fn __dfl_driver_register(dfl_drv: *mut dfl_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn dfl_driver_unregister(dfl_drv: *mut dfl_driver);
}
//
// module_dfl_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit().
//

