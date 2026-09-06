//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/w1/w1_internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net>
//

pub const W1_SLAVE_ACTIVE: c_int = 0;
pub const W1_SLAVE_DETACH: c_int = 1;
//
// struct w1_async_cmd - execute callback from the w1_process kthread
// @async_entry: link entry
// @cb: callback function, must list_del and destroy this list before
// returning
//
// When inserted into the w1_master async_list, w1_process will execute
// the callback.  Embed this into the structure with the command details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_async_cmd {
    pub async_entry: list_head,
    pub async_cmd): *mut *mut *mut void (cb)(struct w1_master dev, struct w1_async_cmd,
}

extern "C" {
    pub fn w1_create_master_attributes(master: *mut w1_master) -> c_int;
}
extern "C" {
    pub fn w1_destroy_master_attributes(master: *mut w1_master);
}
// call w1_unref_slave to release the reference counts w1_search_slave added
//
// decrements the reference on sl->master and sl, and cleans up if zero
// returns the reference count after it has been decremented
//
extern "C" {
    pub fn w1_unref_slave(sl: *mut w1_slave) -> c_int;
}
extern "C" {
    pub fn w1_slave_found(dev: *mut w1_master, rn: u64);
}
// Disconnect and reconnect devices in the given family.  Used for finding
// unclaimed devices after a family has been registered or releasing devices
// after a family has been unregistered.  Set attach to 1 when a new family
// has just been registered, to 0 when it has been unregistered.
//
extern "C" {
    pub fn w1_reconnect_slaves(f: *mut w1_family, attach: c_int);
}
extern "C" {
    pub fn w1_attach_slave_device(dev: *mut w1_master, rn: *mut w1_reg_num) -> c_int;
}
// 0 success, otherwise EBUSY
extern "C" {
    pub fn w1_slave_detach(sl: *mut w1_slave) -> c_int;
}
extern "C" {
    pub fn __w1_remove_master_device(dev: *mut w1_master);
}
extern "C" {
    pub fn w1_family_put(f: *mut w1_family);
}
extern "C" {
    pub fn __w1_family_get(f: *mut w1_family);
}
extern "C" {
    pub fn w1_process_callbacks(dev: *mut w1_master) -> c_int;
}
extern "C" {
    pub fn w1_process(data: *mut c_void) -> c_int;
}
