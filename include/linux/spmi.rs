//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spmi.h
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
// Copyright (c) 2012-2013, The Linux Foundation. All rights reserved.
//

// Maximum slave identifier
pub const SPMI_MAX_SLAVE_ID: c_int = 16;
// SPMI Commands
pub const SPMI_CMD_EXT_WRITE: c_uint = 0x00;
pub const SPMI_CMD_RESET: c_uint = 0x10;
pub const SPMI_CMD_SLEEP: c_uint = 0x11;
pub const SPMI_CMD_SHUTDOWN: c_uint = 0x12;
pub const SPMI_CMD_WAKEUP: c_uint = 0x13;
pub const SPMI_CMD_AUTHENTICATE: c_uint = 0x14;
pub const SPMI_CMD_MSTR_READ: c_uint = 0x15;
pub const SPMI_CMD_MSTR_WRITE: c_uint = 0x16;
pub const SPMI_CMD_TRANSFER_BUS_OWNERSHIP: c_uint = 0x1A;
pub const SPMI_CMD_DDB_MASTER_READ: c_uint = 0x1B;
pub const SPMI_CMD_DDB_SLAVE_READ: c_uint = 0x1C;
pub const SPMI_CMD_EXT_READ: c_uint = 0x20;
pub const SPMI_CMD_EXT_WRITEL: c_uint = 0x30;
pub const SPMI_CMD_EXT_READL: c_uint = 0x38;
pub const SPMI_CMD_WRITE: c_uint = 0x40;
pub const SPMI_CMD_READ: c_uint = 0x60;
pub const SPMI_CMD_ZERO_WRITE: c_uint = 0x80;
//
// struct spmi_device - Basic representation of an SPMI device
// @dev:	Driver model representation of the device.
// @ctrl:	SPMI controller managing the bus hosting this device.
// @usid:	This devices' Unique Slave IDentifier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spmi_device {
    pub dev: device,
    pub ctrl: *mut spmi_controller,
    pub usid: u8,
}

extern "C" {
    pub fn container_of(_arg: d, spmi_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &sdev->dev) -> return;
}
extern "C" {
    pub fn spmi_device_add(sdev: *mut spmi_device) -> c_int;
}
extern "C" {
    pub fn spmi_device_remove(sdev: *mut spmi_device);
}
//
// struct spmi_controller - interface to the SPMI master controller
// @dev:	Driver model representation of the device.
// @nr:		board-specific number identifier for this controller/bus
// @cmd:	sends a non-data command sequence on the SPMI bus.
// @read_cmd:	sends a register read command sequence on the SPMI bus.
// @write_cmd:	sends a register write command sequence on the SPMI bus.
// @priv:	array of private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spmi_controller {
    pub dev: device,
    pub nr: c_uint,
    pub sid): *mut *mut *mut int (cmd)(struct spmi_controller ctrl, u8 opcode, u8,
    pub len): *mut *mut u8 sid, u16 addr, u8 buf, size_t,
    pub len): *const *const u8 sid, u16 addr, u8 buf, size_t,
    pub priv: [u8; ],
}

extern "C" {
    pub fn container_of(_arg: d, spmi_controller: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &ctrl->dev) -> return;
}
//
// spmi_controller_put() - decrement controller refcount
// @ctrl:	SPMI controller.
//
extern "C" {
    pub fn spmi_controller_add(ctrl: *mut spmi_controller) -> c_int;
}
extern "C" {
    pub fn spmi_controller_remove(ctrl: *mut spmi_controller);
}
extern "C" {
    pub fn devm_spmi_controller_add(parent: *mut device, ctrl: *mut spmi_controller) -> c_int;
}
//
// struct spmi_driver - SPMI slave device driver
// @driver:	SPMI device drivers should initialize name and owner field of
// this structure.
// @probe:	binds this driver to a SPMI device.
// @remove:	unbinds this driver from the SPMI device.
// @shutdown:	shuts down this driver.
//
// If PM runtime support is desired for a slave, a device driver can call
// pm_runtime_put() from their probe() routine (and a balancing
// pm_runtime_get() in remove()).  PM runtime support for a slave is
// implemented by issuing a SLEEP command to the slave on runtime_suspend(),
// transitioning the slave into the SLEEP state.  On runtime_resume(), a WAKEUP
// command is sent to the slave to bring it back to ACTIVE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spmi_driver {
    pub driver: device_driver,
    pub sdev): *mut *mut int (probe)(struct spmi_device,
    pub sdev): *mut *mut void (remove)(struct spmi_device,
    pub sdev): *mut *mut void (shutdown)(struct spmi_device,
}

extern "C" {
    pub fn container_of(_arg: d, spmi_driver: struct, _arg: driver) -> return;
}

extern "C" {
    pub fn __spmi_driver_register(sdrv: *mut spmi_driver, owner: *mut module) -> c_int;
}
//
// spmi_driver_unregister() - unregister an SPMI client driver
// @sdrv:	the driver to unregister
//

extern "C" {
    pub fn spmi_register_read(sdev: *mut spmi_device, addr: u8, buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn spmi_register_write(sdev: *mut spmi_device, addr: u8, data: u8) -> c_int;
}
extern "C" {
    pub fn spmi_register_zero_write(sdev: *mut spmi_device, data: u8) -> c_int;
}
extern "C" {
    pub fn spmi_command_reset(sdev: *mut spmi_device) -> c_int;
}
extern "C" {
    pub fn spmi_command_sleep(sdev: *mut spmi_device) -> c_int;
}
extern "C" {
    pub fn spmi_command_wakeup(sdev: *mut spmi_device) -> c_int;
}
extern "C" {
    pub fn spmi_command_shutdown(sdev: *mut spmi_device) -> c_int;
}
