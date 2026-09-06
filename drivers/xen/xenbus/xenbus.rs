//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/xen/xenbus/xenbus.h
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


//
// Private include for xenbus communications.
//
// Copyright (C) 2005 Rusty Russell, IBM Corporation
// Copyright (C) 2005 XenSource Ltd.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

pub const XEN_BUS_ID_SIZE: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_bus_type {
    pub root: *mut c_char,
    pub levels: c_uint,
    pub nodename): *const *const int (get_bus_id)(char bus_id[XEN_BUS_ID_SIZE], char,
    pub dir): *const c_char,
    pub token): *const *const char path, char,
    pub token): *const c_char,
    pub bus: bus_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xenstore_init {
    XS_UNKNOWN,
    XS_PV,
    XS_HVM,
    XS_LOCAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xs_watch_event {
    pub list: list_head,
    pub len: c_uint,
    pub handle: *mut xenbus_watch,
    pub path: *const c_char,
    pub token: *const c_char,
    pub body: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xb_req_state {
    xb_req_state_queued,
    xb_req_state_wait_reply,
    xb_req_state_got_reply,
    xb_req_state_aborted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xb_req_data {
    pub list: list_head,
    pub wq: wait_queue_head_t,
    pub kref: kref,
    pub msg: xsd_sockmsg,
    pub caller_req_id: u32,
    pub type: xsd_sockmsg_type,
    pub body: *mut c_char,
    pub vec: *const kvec,
    pub num_vecs: c_int,
    pub err: c_int,
    pub state: xb_req_state,
    pub user_req: bool,
    pub ): *mut *mut void (cb)(struct xb_req_data,
    pub par: *mut c_void,
}

extern "C" {
    pub fn xs_init() -> c_int;
}
extern "C" {
    pub fn xb_init_comms() -> c_int;
}
extern "C" {
    pub fn xb_deinit_comms();
}
extern "C" {
    pub fn xs_watch_msg(event: *mut xs_watch_event) -> c_int;
}
extern "C" {
    pub fn xs_request_exit(req: *mut xb_req_data);
}
extern "C" {
    pub fn xs_free_req(kref: *mut kref);
}
extern "C" {
    pub fn xenbus_match(_dev: *mut device, _drv: *const device_driver) -> c_int;
}
extern "C" {
    pub fn xenbus_dev_probe(_dev: *mut device) -> c_int;
}
extern "C" {
    pub fn xenbus_dev_remove(_dev: *mut device);
}
extern "C" {
    pub fn xenbus_probe_devices(bus: *mut xen_bus_type) -> c_int;
}
extern "C" {
    pub fn xenbus_dev_changed(node: *const c_char, bus: *mut xen_bus_type);
}
extern "C" {
    pub fn xenbus_dev_freeze(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn xenbus_dev_restore(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn xenbus_dev_thaw(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn xenbus_ring_ops_init();
}
extern "C" {
    pub fn xenbus_dev_request_and_reply(msg: *mut xsd_sockmsg, par: *mut c_void) -> c_int;
}
extern "C" {
    pub fn xenbus_dev_queue_reply(req: *mut xb_req_data);
}
