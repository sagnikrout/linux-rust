//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/hyperbus.h
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
// Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
//

// HyperBus command bits
pub const HYPERBUS_RW: c_uint = 0x80	/* R/W# */;
pub const HYPERBUS_RW_WRITE: c_int = 0;
pub const HYPERBUS_RW_READ: c_uint = 0x80;
pub const HYPERBUS_AS: c_uint = 0x40	/* Address Space */;
pub const HYPERBUS_AS_MEM: c_int = 0;
pub const HYPERBUS_AS_REG: c_uint = 0x40;
pub const HYPERBUS_BT: c_uint = 0x20	/* Burst Type */;
pub const HYPERBUS_BT_WRAPPED: c_int = 0;
pub const HYPERBUS_BT_LINEAR: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hyperbus_memtype {
    HYPERFLASH,
    HYPERRAM,
}

//
// struct hyperbus_device - struct representing HyperBus slave device
// @map: map_info struct for accessing MMIO HyperBus flash memory
// @np: pointer to HyperBus slave device node
// @mtd: pointer to MTD struct
// @ctlr: pointer to HyperBus controller struct
// @memtype: type of memory device: HyperFlash or HyperRAM
// @priv: pointer to controller specific per device private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyperbus_device {
    pub map: map_info,
    pub np: *mut device_node,
    pub mtd: *mut mtd_info,
    pub ctlr: *mut hyperbus_ctlr,
    pub memtype: hyperbus_memtype,
    pub priv: *mut c_void,
}

//
// struct hyperbus_ops - struct representing custom HyperBus operations
// @read16: read 16 bit of data from flash in a single burst. Used to read
// from non default address space, such as ID/CFI space
// @write16: write 16 bit of data to flash in a single burst. Used to
// send cmd to flash or write single 16 bit word at a time.
// @copy_from: copy data from flash memory
// @copy_to: copy data to flash memory
// @calibrate: calibrate HyperBus controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyperbus_ops {
    pub addr): *mut *mut *mut u16 (read16)(struct hyperbus_device hbdev, unsigned long,
    pub val): unsigned long addr, u16,
    pub len): unsigned long from, ssize_t,
    pub len): *const *const void from, ssize_t,
    pub dev): *mut *mut int (calibrate)(struct hyperbus_device,
}

//
// struct hyperbus_ctlr - struct representing HyperBus controller
// @dev: pointer to HyperBus controller device
// @calibrated: flag to indicate ctlr calibration sequence is complete
// @ops: HyperBus controller ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyperbus_ctlr {
    pub dev: *mut device,
    pub calibrated: bool,
    pub ops: *const hyperbus_ops,
}

//
// hyperbus_register_device - probe and register a HyperBus slave memory device
// @hbdev: hyperbus_device struct with dev, np and ctlr field populated
//
// Return: 0 for success, others for failure.
//
extern "C" {
    pub fn hyperbus_register_device(hbdev: *mut hyperbus_device) -> c_int;
}
//
// hyperbus_unregister_device - deregister HyperBus slave memory device
// @hbdev: hyperbus_device to be unregistered
//
extern "C" {
    pub fn hyperbus_unregister_device(hbdev: *mut hyperbus_device);
}
