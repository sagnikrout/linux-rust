//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rpmsg.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Remote processor messaging
//
// Copyright (C) 2011 Texas Instruments, Inc.
// Copyright (C) 2011 Google, Inc.
// All rights reserved.
//

//
// struct rpmsg_channel_info - channel info representation
// @name: name of service
// @src: local address
// @dst: destination address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_channel_info {
    pub name: [c_char; RPMSG_NAME_SIZE],
    pub src: u32,
    pub dst: u32,
}

//
// rpmsg_device - device that belong to the rpmsg bus
// @dev: the device struct
// @id: device id (used to match between rpmsg drivers and devices)
// @src: local address
// @dst: destination address
// @ept: the rpmsg endpoint of this channel
// @announce: if set, rpmsg will announce the creation/removal of this channel
// @little_endian: True if transport is using little endian byte representation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_device {
    pub dev: device,
    pub id: rpmsg_device_id,
    pub src: u32,
    pub dst: u32,
    pub ept: *mut rpmsg_endpoint,
    pub announce: bool,
    pub little_endian: bool,
    pub ops: *const rpmsg_device_ops,
}

extern "C" {
    pub fn int(: *mut *mut rpmsg_rx_cb_t)(struct rpmsg_device, : *mut c_void, _arg: c_int, : *mut c_void, _arg: u32) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut rpmsg_flowcontrol_cb_t)(struct rpmsg_device, : *mut c_void, _arg: bool) -> typedef;
}
//
// struct rpmsg_endpoint - binds a local rpmsg address to its user
// @rpdev: rpmsg channel device
// @refcount: when this drops to zero, the ept is deallocated
// @cb: rx callback handler
// @flow_cb: remote flow control callback handler
// @cb_lock: must be taken before accessing/changing @cb
// @addr: local rpmsg address
// @priv: private data for the driver's use
//
// In essence, an rpmsg endpoint represents a listener on the rpmsg bus, as
// it binds an rpmsg address with an rx callback handler.
//
// Simple rpmsg drivers shouldn't use this struct directly, because
// things just work: every rpmsg driver provides an rx callback upon
// registering to the bus, and that callback is then bound to its rpmsg
// address when the driver is probed. When relevant inbound messages arrive
// (i.e. messages which their dst address equals to the src address of
// the rpmsg channel), the driver's handler is invoked to process it.
//
// More complicated drivers though, that do need to allocate additional rpmsg
// addresses, and bind them to different rx callbacks, must explicitly
// create additional endpoints by themselves (see rpmsg_create_ept()).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_endpoint {
    pub rpdev: *mut rpmsg_device,
    pub refcount: kref,
    pub cb: rpmsg_rx_cb_t,
    pub flow_cb: rpmsg_flowcontrol_cb_t,
    pub cb_lock: mutex,
    pub addr: u32,
    pub priv: *mut c_void,
    pub ops: *const rpmsg_endpoint_ops,
}

//
// struct rpmsg_driver - rpmsg driver struct
// @drv: underlying device driver
// @id_table: rpmsg ids serviced by this driver
// @probe: invoked when a matching rpmsg channel (i.e. device) is found
// @remove: invoked when the rpmsg channel is removed
// @callback: invoked when an inbound message is received on the channel
// @flowcontrol: invoked when remote side flow control request is received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_driver {
    pub drv: device_driver,
    pub id_table: *const rpmsg_device_id,
    pub dev): *mut *mut int (probe)(struct rpmsg_device,
    pub dev): *mut *mut void (remove)(struct rpmsg_device,
    pub u32): *mut *mut *mut *mut *mut int (callback)(struct rpmsg_device , void , int, void ,,
    pub bool): *mut *mut *mut *mut int (flowcontrol)(struct rpmsg_device , void ,,
}

extern "C" {
    pub fn __rpmsg16_to_cpu(_arg: rpmsg_is_little_endian(), _arg: val) -> return;
}
extern "C" {
    pub fn __rpmsg16_to_cpu(_arg: rpdev->little_endian, _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_rpmsg16(_arg: rpmsg_is_little_endian(), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_rpmsg16(_arg: rpdev->little_endian, _arg: val) -> return;
}
extern "C" {
    pub fn __rpmsg32_to_cpu(_arg: rpmsg_is_little_endian(), _arg: val) -> return;
}
extern "C" {
    pub fn __rpmsg32_to_cpu(_arg: rpdev->little_endian, _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_rpmsg32(_arg: rpmsg_is_little_endian(), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_rpmsg32(_arg: rpdev->little_endian, _arg: val) -> return;
}
extern "C" {
    pub fn __rpmsg64_to_cpu(_arg: rpmsg_is_little_endian(), _arg: val) -> return;
}
extern "C" {
    pub fn __rpmsg64_to_cpu(_arg: rpdev->little_endian, _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_rpmsg64(_arg: rpmsg_is_little_endian(), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_rpmsg64(_arg: rpdev->little_endian, _arg: val) -> return;
}

extern "C" {
    pub fn rpmsg_register_device(rpdev: *mut rpmsg_device) -> c_int;
}
extern "C" {
    pub fn __register_rpmsg_driver(drv: *mut rpmsg_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn unregister_rpmsg_driver(drv: *mut rpmsg_driver);
}
extern "C" {
    pub fn rpmsg_destroy_ept(: *mut rpmsg_endpoint);
}
extern "C" {
    pub fn rpmsg_send(ept: *mut rpmsg_endpoint, data: *const c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn rpmsg_sendto(ept: *mut rpmsg_endpoint, data: *const c_void, len: c_int, dst: u32) -> c_int;
}
extern "C" {
    pub fn rpmsg_trysend(ept: *mut rpmsg_endpoint, data: *const c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn rpmsg_trysendto(ept: *mut rpmsg_endpoint, data: *const c_void, len: c_int, dst: u32) -> c_int;
}
extern "C" {
    pub fn rpmsg_get_mtu(ept: *mut rpmsg_endpoint) -> isize;
}
extern "C" {
    pub fn rpmsg_set_flow_control(ept: *mut rpmsg_endpoint, pause: bool, dst: u32) -> c_int;
}

// This shouldn't be possible

// use a macro to avoid include chaining to get THIS_MODULE

//
// module_rpmsg_driver() - Helper macro for registering an rpmsg driver
// @__rpmsg_driver: rpmsg_driver struct
//
// Helper macro for rpmsg drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate.  Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

