//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/slimbus.h
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
// Copyright (c) 2011-2017, The Linux Foundation
//

//
// struct slim_eaddr - Enumeration address for a SLIMbus device
// @instance: Instance value
// @dev_index: Device index
// @prod_code: Product code
// @manf_id: Manufacturer Id for the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_eaddr {
    pub instance: u8,
    pub dev_index: u8,
    pub prod_code: u16,
    pub manf_id: u16,
    pub __packed: },
//
// enum slim_device_status - slim device status
// @SLIM_DEVICE_STATUS_DOWN: Slim device is absent or not reported yet.
// @SLIM_DEVICE_STATUS_UP: Slim device is announced on the bus.
// @SLIM_DEVICE_STATUS_RESERVED: Reserved for future use.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_device_status {
    SLIM_DEVICE_STATUS_DOWN = 0,
    SLIM_DEVICE_STATUS_UP,
    SLIM_DEVICE_STATUS_RESERVED,
}

    pub slim_controller: struct,
//
// struct slim_device - Slim device handle.
// @dev: Driver model representation of the device.
// @e_addr: Enumeration address of this device.
// @status: slim device status
// @ctrl: slim controller instance.
// @laddr: 1-byte Logical address of this device.
// @is_laddr_valid: indicates if the laddr is valid or not
// @stream_list: List of streams on this device
// @stream_list_lock: lock to protect the stream list
//
// This is the client/device handle returned when a SLIMbus
// device is registered with a controller.
// Pointer to this structure is used by client-driver as a handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_device {
    pub dev: device,
    pub e_addr: slim_eaddr,
    pub ctrl: *mut slim_controller,
    pub status: slim_device_status,
    pub laddr: u8,
    pub is_laddr_valid: bool,
    pub stream_list: list_head,
    pub stream_list_lock: spinlock_t,
}

//
// struct slim_driver - SLIMbus 'generic device' (slave) device driver
// (similar to 'spi_device' on SPI)
// @probe: Binds this driver to a SLIMbus device.
// @remove: Unbinds this driver from the SLIMbus device.
// @shutdown: Standard shutdown callback used during powerdown/halt.
// @device_status: This callback is called when
// - The device reports present and gets a laddr assigned
// - The device reports absent, or the bus goes down.
// @driver: SLIMbus device drivers should initialize name and owner field of
// this structure
// @id_table: List of SLIMbus devices supported by this driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_driver {
    pub sl): *mut *mut int (probe)(struct slim_device,
    pub sl): *mut *mut void (remove)(struct slim_device,
    pub sl): *mut *mut void (shutdown)(struct slim_device,
    pub s): slim_device_status,
    pub driver: device_driver,
    pub id_table: *const slim_device_id,
}

//
// struct slim_val_inf - Slimbus value or information element
// @start_offset: Specifies starting offset in information/value element map
// @rbuf: buffer to read the values
// @wbuf: buffer to write
// @num_bytes: upto 16. This ensures that the message will fit the slicesize
// per SLIMbus spec
// @comp: completion for asynchronous operations, valid only if TID is
// required for transaction, like REQUEST operations.
// Rest of the transactions are synchronous anyway.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_val_inf {
    pub start_offset: u16,
    pub num_bytes: u8,
    pub rbuf: *mut u8,
    pub wbuf: *const u8,
    pub comp: *mut completion,
}

pub const SLIM_DEVICE_MAX_CHANNELS: c_int = 256;
// A SLIMBus Device may have frmo 0 to 31 Ports (inclusive)
pub const SLIM_DEVICE_MAX_PORTS: c_int = 32;
//
// struct slim_stream_config - SLIMbus stream configuration
// Configuring a stream is done at hw_params or prepare call
// from audio drivers where they have all the required information
// regarding rate, number of channels and so on.
// There is a 1:1 mapping of channel and ports.
//
// @rate: data rate
// @bps: bits per data sample
// @ch_count: number of channels
// @chs: pointer to list of channel numbers
// @port_mask: port mask of ports to use for this stream
// @direction: direction of the stream, SNDRV_PCM_STREAM_PLAYBACK
// or SNDRV_PCM_STREAM_CAPTURE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_stream_config {
    pub rate: c_uint,
    pub bps: c_uint,
// MAX 256 channels
    pub ch_count: c_uint,
    pub chs: *mut c_uint,
// Max 32 ports per device
    pub port_mask: c_ulong,
    pub direction: c_int,
}

//
// use a macro to avoid include chaining to get THIS_MODULE
//

extern "C" {
    pub fn __slim_driver_register(drv: *mut slim_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn slim_driver_unregister(drv: *mut slim_driver);
}
//
// module_slim_driver() - Helper macro for registering a SLIMbus driver
// @__slim_driver: slimbus_driver struct
//
// Helper macro for SLIMbus drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

extern "C" {
    pub fn dev_get_drvdata(_arg: &dev->dev) -> return;
}
extern "C" {
    pub fn slim_get_logical_addr(sbdev: *mut slim_device) -> c_int;
}
// Information Element management messages
pub const SLIM_MSG_MC_REQUEST_INFORMATION: c_uint = 0x20;
pub const SLIM_MSG_MC_REQUEST_CLEAR_INFORMATION: c_uint = 0x21;
pub const SLIM_MSG_MC_REPLY_INFORMATION: c_uint = 0x24;
pub const SLIM_MSG_MC_CLEAR_INFORMATION: c_uint = 0x28;
pub const SLIM_MSG_MC_REPORT_INFORMATION: c_uint = 0x29;
// Value Element management messages
pub const SLIM_MSG_MC_REQUEST_VALUE: c_uint = 0x60;
pub const SLIM_MSG_MC_REQUEST_CHANGE_VALUE: c_uint = 0x61;
pub const SLIM_MSG_MC_REPLY_VALUE: c_uint = 0x64;
pub const SLIM_MSG_MC_CHANGE_VALUE: c_uint = 0x68;
extern "C" {
    pub fn slim_readb(sdev: *mut slim_device, addr: u32) -> c_int;
}
extern "C" {
    pub fn slim_writeb(sdev: *mut slim_device, addr: u32, value: u8) -> c_int;
}
extern "C" {
    pub fn slim_read(sdev: *mut slim_device, addr: u32, count: usize, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn slim_write(sdev: *mut slim_device, addr: u32, count: usize, val: *mut u8) -> c_int;
}
// SLIMbus Stream apis
extern "C" {
    pub fn slim_stream_enable(stream: *mut slim_stream_runtime) -> c_int;
}
extern "C" {
    pub fn slim_stream_disable(stream: *mut slim_stream_runtime) -> c_int;
}
extern "C" {
    pub fn slim_stream_unprepare(stream: *mut slim_stream_runtime) -> c_int;
}
extern "C" {
    pub fn slim_stream_free(stream: *mut slim_stream_runtime) -> c_int;
}
