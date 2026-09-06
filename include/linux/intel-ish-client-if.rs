//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/intel-ish-client-if.h
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
// Intel ISH client Interface definitions
//
// Copyright (c) 2019, Intel Corporation.
//

// Client state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cl_state {
    ISHTP_CL_INITIALIZING = 0,
    ISHTP_CL_CONNECTING,
    ISHTP_CL_CONNECTED,
    ISHTP_CL_DISCONNECTING,
    ISHTP_CL_DISCONNECTED
}

//
// struct ishtp_cl_device - ISHTP device handle
// @driver:	driver instance on a bus
// @name:	Name of the device for probe
// @probe:	driver callback for device probe
// @remove:	driver callback on device removal
//
// Client drivers defines to get probed/removed for ISHTP client device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_cl_driver {
    pub driver: device_driver,
    pub name: *const c_char,
    pub id: *const ishtp_device_id,
    pub dev): *mut *mut int (probe)(struct ishtp_cl_device,
    pub dev): *mut *mut void (remove)(struct ishtp_cl_device,
    pub dev): *mut *mut int (reset)(struct ishtp_cl_device,
    pub pm: *const dev_pm_ops,
}

//
// struct ishtp_msg_data - ISHTP message data struct
// @size:	Size of data in the *data
// @data:	Pointer to data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_msg_data {
    pub size: u32,
    pub data: *mut c_uchar,
}

//
// struct ishtp_cl_rb - request block structure
// @list:	Link to list members
// @cl:		ISHTP client instance
// @buffer:	message header
// @buf_idx:	Index into buffer
// @read_time:	 unused at this time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_cl_rb {
    pub list: list_head,
    pub cl: *mut ishtp_cl,
    pub buffer: ishtp_msg_data,
    pub buf_idx: c_ulong,
    pub read_time: c_ulong,
}

extern "C" {
    pub fn ishtp_cl_driver_unregister(driver: *mut ishtp_cl_driver);
}
// Get the device * from ishtp device instance
// wait for IPC resume
extern "C" {
    pub fn ishtp_wait_resume(dev: *mut ishtp_device) -> bool;
}
// Trace interface for clients
extern "C" {
    pub fn ishtp_trace_callback(cl_device: *mut ishtp_cl_device) -> ishtp_print_log;
}
// Get device pointer of PCI device for DMA acces
// Get the ISHTP workqueue
extern "C" {
    pub fn ishtp_cl_free(cl: *mut ishtp_cl);
}
extern "C" {
    pub fn ishtp_cl_link(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_unlink(cl: *mut ishtp_cl);
}
extern "C" {
    pub fn ishtp_cl_disconnect(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_connect(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_destroy_connection(cl: *mut ishtp_cl, reset: bool);
}
extern "C" {
    pub fn ishtp_cl_send(cl: *mut ishtp_cl, buf: *mut u8, length: usize) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_flush_queues(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_io_rb_recycle(rb: *mut ishtp_cl_rb) -> c_int;
}
extern "C" {
    pub fn ishtp_set_client_data(cl: *mut ishtp_cl, data: *mut c_void);
}
extern "C" {
    pub fn ishtp_set_tx_ring_size(cl: *mut ishtp_cl, size: c_int);
}
extern "C" {
    pub fn ishtp_set_rx_ring_size(cl: *mut ishtp_cl, size: c_int);
}
extern "C" {
    pub fn ishtp_set_connection_state(cl: *mut ishtp_cl, state: c_int);
}
extern "C" {
    pub fn ishtp_get_connection_state(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_set_fw_client_id(cl: *mut ishtp_cl, fw_client_id: c_int);
}
extern "C" {
    pub fn ishtp_put_device(cl_dev: *mut ishtp_cl_device);
}
extern "C" {
    pub fn ishtp_get_device(cl_dev: *mut ishtp_cl_device);
}
extern "C" {
    pub fn ishtp_set_drvdata(cl_device: *mut ishtp_cl_device, data: *mut c_void);
}
extern "C" {
    pub fn ishtp_get_fw_client_id(fw_client: *mut ishtp_fw_client) -> c_int;
}
extern "C" {
    pub fn ish_hw_reset(dev: *mut ishtp_device) -> c_int;
}
