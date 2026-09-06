//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ishtp/bus.h
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
// ISHTP bus definitions
//
// Copyright (c) 2014-2016, Intel Corporation.
//

//
// struct ishtp_cl_device - ISHTP device handle
// @dev:	device pointer
// @ishtp_dev:	pointer to ishtp device structure to primarily to access
// hw device operation callbacks and properties
// @fw_client:	fw_client pointer to get fw information like protocol name
// max message length etc.
// @device_link: Link to next client in the list on a bus
// @event_work:	Used to schedule rx event for client
// @driver_data: Storage driver private data
// @reference_count:	Used for get/put device
// @event_cb:	Callback to driver to send events
//
// An ishtp_cl_device pointer is returned from ishtp_add_device()
// and links ISHTP bus clients to their actual host client pointer.
// Drivers for ISHTP devices will get an ishtp_cl_device pointer
// when being probed and shall use it for doing bus I/O.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_cl_device {
    pub dev: device,
    pub ishtp_dev: *mut ishtp_device,
    pub fw_client: *mut ishtp_fw_client,
    pub device_link: list_head,
    pub event_work: work_struct,
    pub driver_data: *mut c_void,
    pub reference_count: c_int,
    pub device): *mut *mut void (event_cb)(struct ishtp_cl_device,
}

extern "C" {
    pub fn ishtp_bus_new_client(dev: *mut ishtp_device) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_device_bind(cl: *mut ishtp_cl) -> c_int;
}
extern "C" {
    pub fn ishtp_cl_bus_rx_event(device: *mut ishtp_cl_device);
}
// Write a multi-fragment message
// Write a single-fragment message
// Use DMA to send/receive messages
extern "C" {
    pub fn ishtp_use_dma_transfer() -> c_int;
}
// Exported functions
extern "C" {
    pub fn ishtp_recv(dev: *mut ishtp_device);
}
extern "C" {
    pub fn ishtp_reset_handler(dev: *mut ishtp_device);
}
extern "C" {
    pub fn ishtp_reset_compl_handler(dev: *mut ishtp_device);
}
extern "C" {
    pub fn ishtp_fw_cl_by_uuid(dev: *mut ishtp_device, cuuid: *const guid_t) -> c_int;
}
