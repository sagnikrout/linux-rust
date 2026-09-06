//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i3c/device.h
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
// Copyright (C) 2018 Cadence Design Systems Inc.
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//

//
// enum i3c_error_code - I3C error codes
//
// @I3C_ERROR_UNKNOWN: unknown error, usually means the error is not I3C
// related
// @I3C_ERROR_M0: M0 error
// @I3C_ERROR_M1: M1 error
// @I3C_ERROR_M2: M2 error
//
// These are the standard error codes as defined by the I3C specification.
// When -EIO is returned by the i3c_device_do_i3c_xfers() or
// i3c_device_send_hdr_cmds() one can check the error code in
// &struct_i3c_xfer.err or &struct i3c_hdr_cmd.err to get a better idea of
// what went wrong.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_error_code {
    I3C_ERROR_UNKNOWN = 0,
    I3C_ERROR_M0 = 1,
    I3C_ERROR_M1,
    I3C_ERROR_M2,
}

//
// enum i3c_xfer_mode - I3C xfer mode ids
// @I3C_HDR_DDR: DDR mode
// @I3C_HDR_TSP: TSP mode
// @I3C_HDR_TSL: TSL mode
// @I3C_SDR: SDR mode (NOT HDR mode)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_xfer_mode {
// The below 3 value (I3C_HDR*) must match GETCAP1 Byte bit position
    I3C_HDR_DDR = 0,
    I3C_HDR_TSP = 1,
    I3C_HDR_TSL = 2,
// Use for default SDR transfer mode
    I3C_SDR = 31,
}

//
// struct i3c_xfer - I3C data transfer
// @rnw: encodes the transfer direction. true for a read, false for a write
// @cmd: Read/Write command in HDR mode, read: 0x80 - 0xff, write: 0x00 - 0x7f
// @len: transfer length in bytes of the transfer
// @actual_len: actual length in bytes are transferred by the controller
// @data: input/output buffer
// @data.in: input buffer. Must point to a DMA-able buffer
// @data.out: output buffer. Must point to a DMA-able buffer
// @err: I3C error code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_xfer {
    pub rnw: u8,
    pub cmd: u8,
}

//
// enum i3c_dcr - I3C DCR values
// @I3C_DCR_GENERIC_DEVICE: generic I3C device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_dcr {
    I3C_DCR_GENERIC_DEVICE = 0,
}

//
// struct i3c_device_info - I3C device information
// @pid: Provisioned ID
// @bcr: Bus Characteristic Register
// @dcr: Device Characteristic Register
// @static_addr: static/I2C address
// @dyn_addr: dynamic address
// @hdr_cap: supported HDR modes
// @max_read_ds: max read speed information
// @max_write_ds: max write speed information
// @max_ibi_len: max IBI payload length
// @max_read_turnaround: max read turn-around time in micro-seconds
// @max_read_len: max private SDR read length in bytes
// @max_write_len: max private SDR write length in bytes
//
// These are all basic information that should be advertised by an I3C device.
// Some of them are optional depending on the device type and device
// capabilities.
// For each I3C slave attached to a master with
// i3c_master_add_i3c_dev_locked(), the core will send the relevant CCC command
// to retrieve these data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_device_info {
    pub pid: u64,
    pub bcr: u8,
    pub dcr: u8,
    pub static_addr: u8,
    pub dyn_addr: u8,
    pub hdr_cap: u8,
    pub max_read_ds: u8,
    pub max_write_ds: u8,
    pub max_ibi_len: u8,
    pub max_read_turnaround: u32,
    pub max_read_len: u16,
    pub max_write_len: u16,
}

//
// I3C device internals are kept hidden from I3C device users. It's just
// simpler to refactor things when everything goes through getter/setters, and
// I3C device drivers should not have to worry about internal representation
// anyway.
//
// These macros should be used to i3c_device_id entries.

//
// struct i3c_driver - I3C device driver
// @driver: inherit from device_driver
// @probe: I3C device probe method
// @remove: I3C device remove method
// @id_table: I3C device match table. Will be used by the framework to decide
// which device to bind to this driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_driver {
    pub driver: device_driver,
    pub dev): *mut *mut int (probe)(struct i3c_device,
    pub dev): *mut *mut void (remove)(struct i3c_device,
    pub id_table: *const i3c_device_id,
}

//
// dev_to_i3cdev() - Returns the I3C device containing @dev
// @__dev: device object
//
// Return: a pointer to an I3C device object.
//

extern "C" {
    pub fn dev_get_drvdata(_arg: dev) -> return;
}
extern "C" {
    pub fn i3c_driver_unregister(drv: *mut i3c_driver);
}

//
// module_i3c_driver() - Register a module providing an I3C driver
// @__drv: the I3C driver to register
//
// Provide generic init/exit functions that simply register/unregister an I3C
// driver.
// Should be used by any driver that does not require extra init/cleanup steps.
//

//
// i3c_i2c_driver_register() - Register an i2c and an i3c driver
// @i3cdrv: the I3C driver to register
// @i2cdrv: the I2C driver to register
//
// This function registers both @i2cdev and @i3cdev, and fails if one of these
// registrations fails. This is mainly useful for devices that support both I2C
// and I3C modes.
// Note that when CONFIG_I3C is not enabled, this function only registers the
// I2C driver.
//
// Return: 0 if both registrations succeeds, a negative error code otherwise.
//
// i3c_i2c_driver_unregister() - Unregister an i2c and an i3c driver
// @i3cdrv: the I3C driver to register
// @i2cdrv: the I2C driver to register
//
// This function unregisters both @i3cdrv and @i2cdrv.
// Note that when CONFIG_I3C is not enabled, this function only unregisters the
// @i2cdrv.
//
// module_i3c_i2c_driver() - Register a module providing an I3C and an I2C
// driver
// @__i3cdrv: the I3C driver to register
// @__i2cdrv: the I2C driver to register
//
// Provide generic init/exit functions that simply register/unregister an I3C
// and an I2C driver.
// This macro can be used even if CONFIG_I3C is disabled, in this case, only
// the I2C driver will be registered.
// Should be used by any driver that does not require extra init/cleanup steps.
//

extern "C" {
    pub fn i3c_device_get_supported_xfer_mode(dev: *mut i3c_device) -> u32;
}

extern "C" {
    pub fn i3c_device_do_setdasa(dev: *mut i3c_device) -> c_int;
}
extern "C" {
    pub fn i3c_device_get_info(dev: *const i3c_device, info: *mut i3c_device_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ibi_payload {
    pub len: c_uint,
    pub data: *const c_void,
}

//
// struct i3c_ibi_setup - IBI setup object
// @max_payload_len: maximum length of the payload associated to an IBI. If one
// IBI appears to have a payload that is bigger than this
// number, the IBI will be rejected.
// @num_slots: number of pre-allocated IBI slots. This should be chosen so that
// the system never runs out of IBI slots, otherwise you'll lose
// IBIs.
// @handler: IBI handler, every time an IBI is received. This handler is called
// in a workqueue context. It is allowed to sleep and send new
// messages on the bus, though it's recommended to keep the
// processing done there as fast as possible to avoid delaying
// processing of other queued on the same workqueue.
//
// Temporary structure used to pass information to i3c_device_request_ibi().
// This object can be allocated on the stack since i3c_device_request_ibi()
// copies every bit of information and do not use it after
// i3c_device_request_ibi() has returned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ibi_setup {
    pub max_payload_len: c_uint,
    pub num_slots: c_uint,
    pub payload): *const i3c_ibi_payload,
}

extern "C" {
    pub fn i3c_device_free_ibi(dev: *mut i3c_device);
}
extern "C" {
    pub fn i3c_device_enable_ibi(dev: *mut i3c_device) -> c_int;
}
extern "C" {
    pub fn i3c_device_disable_ibi(dev: *mut i3c_device) -> c_int;
}
