//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/xenbus.h
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
// xenbus.h
//
// Talks to Xen Store to figure out what devices we have.
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

pub const XENBUS_MAX_RING_GRANT_ORDER: c_int = 4;

// Register callback to watch this node.
// Path being watched.
//
// Called just before enqueing new event while a spinlock is held.
// The event will be discarded if this callback returns false.
//
// Callback (executed in a process context with no locks held).
// A xenbus device.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenbus_device {
    pub devicetype: *const c_char,
    pub nodename: *const c_char,
    pub otherend: *const c_char,
    pub vanished: bool,
    pub otherend_id: c_int,
    pub otherend_watch: xenbus_watch,
    pub dev: device,
    pub state: xenbus_state,
    pub down: completion,
    pub work: work_struct,
    pub reclaim_sem: semaphore,
// Event channel based statistics and settings.
    pub event_channels: core::sync::atomic::AtomicI32,
    pub events: core::sync::atomic::AtomicI32,
    pub spurious_events: core::sync::atomic::AtomicI32,
    pub jiffies_eoi_delayed: core::sync::atomic::AtomicI32,
    pub spurious_threshold: c_uint,
}

// .../device/<device_type>/<identifier>
// A xenbus driver.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenbus_driver {
    pub /: *const *const *const char name; / defaults to ids[0].devicetype,
    pub ids: *const xenbus_device_id,
    pub /: *mut *mut bool allow_rebind; / avoid setting xenstore closed during remove,
    pub /: *mut *mut bool not_essential; / is not mandatory for boot progress,
    pub id): *const xenbus_device_id,
    pub backend_state): xenbus_state,
    pub dev): *mut *mut void (remove)(struct xenbus_device,
    pub dev): *mut *mut int (suspend)(struct xenbus_device,
    pub dev): *mut *mut int (resume)(struct xenbus_device,
    pub ): *const *const *const int (uevent)(struct xenbus_device , struct kobj_uevent_env,
    pub driver: device_driver,
    pub dev): *mut *mut int (read_otherend_details)(struct xenbus_device,
    pub dev): *mut *mut int (is_ready)(struct xenbus_device,
    pub dev): *mut *mut void (reclaim_memory)(struct xenbus_device,
}

extern "C" {
    pub fn xenbus_unregister_driver(drv: *mut xenbus_driver);
}
// Nil transaction ID.

extern "C" {
    pub fn xenbus_rm(t: xenbus_transaction, dir: *const c_char, node: *const c_char) -> c_int;
}
extern "C" {
    pub fn xenbus_transaction_start(t: *mut xenbus_transaction) -> c_int;
}
extern "C" {
    pub fn xenbus_transaction_end(t: xenbus_transaction, abort: bool) -> c_int;
}
// Single read and scanf: returns -errno or num scanned if > 0.
// Read an (optional) unsigned value.
// Single printf and write: returns -errno or 0.
// Generic read function: NULL-terminated triples of name,
// sprintf-style type string, and pointer. Returns 0 or errno.
extern "C" {
    pub fn xenbus_gather(t: xenbus_transaction, dir: *const c_char, ...) -> c_int;
}
// notifier routines for when the xenstore comes up
extern "C" {
    pub fn register_xenstore_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_xenstore_notifier(nb: *mut notifier_block);
}
extern "C" {
    pub fn register_xenbus_watch(watch: *mut xenbus_watch) -> c_int;
}
extern "C" {
    pub fn unregister_xenbus_watch(watch: *mut xenbus_watch);
}
extern "C" {
    pub fn xs_suspend();
}
extern "C" {
    pub fn xs_resume();
}
extern "C" {
    pub fn xs_suspend_cancel();
}

extern "C" {
    pub fn xenbus_switch_state(dev: *mut xenbus_device, new_state: xenbus_state) -> c_int;
}
extern "C" {
    pub fn xenbus_unmap_ring_vfree(dev: *mut xenbus_device, vaddr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn xenbus_alloc_evtchn(dev: *mut xenbus_device, port: *mut evtchn_port_t) -> c_int;
}
extern "C" {
    pub fn xenbus_free_evtchn(dev: *mut xenbus_device, port: evtchn_port_t) -> c_int;
}
extern "C" {
    pub fn xenbus_dev_error(dev: *mut xenbus_device, err: c_int, fmt: *const c_char, ...);
}
extern "C" {
    pub fn xenbus_dev_fatal(dev: *mut xenbus_device, err: c_int, fmt: *const c_char, ...);
}
extern "C" {
    pub fn xenbus_dev_is_online(dev: *mut xenbus_device) -> c_int;
}
extern "C" {
    pub fn xenbus_frontend_closed(dev: *mut xenbus_device) -> c_int;
}
