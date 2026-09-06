//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/client.h
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
// Copyright (c) 2003-2018, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

//
// reference counting base function
//
extern "C" {
    pub fn mei_me_cl_init(me_cl: *mut mei_me_client);
}
extern "C" {
    pub fn mei_me_cl_put(me_cl: *mut mei_me_client);
}
extern "C" {
    pub fn mei_me_cl_add(dev: *mut mei_device, me_cl: *mut mei_me_client);
}
extern "C" {
    pub fn mei_me_cl_del(dev: *mut mei_device, me_cl: *mut mei_me_client);
}
extern "C" {
    pub fn mei_me_cl_rm_by_uuid(dev: *mut mei_device, uuid: *const uuid_le);
}
extern "C" {
    pub fn mei_me_cl_rm_all(dev: *mut mei_device);
}
//
// mei_me_cl_is_active - check whether me client is active in the fw
//
// @me_cl: me client
//
// Return: true if the me client is active in the firmware
//
// mei_me_cl_uuid - return me client protocol name (uuid)
//
// @me_cl: me client
//
// Return: me client protocol name
//
// mei_me_cl_ver - return me client protocol version
//
// @me_cl: me client
//
// Return: me client protocol version
//
// mei_me_cl_max_conn - return me client max number of connections
//
// @me_cl: me client
//
// Return: me client max number of connections
//
// mei_me_cl_fixed - return me client fixed address, if any
//
// @me_cl: me client
//
// Return: me client fixed address
//
// mei_me_cl_vt - return me client vtag supported status
//
// @me_cl: me client
//
// Return: true if me client supports vt tagging
//
// mei_me_cl_max_len - return me client max msg length
//
// @me_cl: me client
//
// Return: me client max msg length
//
// MEI IO Functions
//
extern "C" {
    pub fn mei_io_cb_free(priv_cb: *mut mei_cl_cb);
}
//
// MEI Host Client Functions
//
extern "C" {
    pub fn mei_cl_link(cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_cl_unlink(cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_cl_add_rd_completed(cl: *mut mei_cl, cb: *mut mei_cl_cb);
}
extern "C" {
    pub fn mei_cl_del_rd_completed(cl: *mut mei_cl, cb: *mut mei_cl_cb);
}
extern "C" {
    pub fn mei_cl_flush_queues(cl: *mut mei_cl, fp: *const file) -> c_int;
}
extern "C" {
    pub fn mei_cl_vt_support_check(cl: *const mei_cl) -> c_int;
}
//
// MEI input output function prototype
//
// mei_cl_is_connected - host client is connected
//
// @cl: host client
//
// Return: true if the host client is connected
//
// mei_cl_me_id - me client id
//
// @cl: host client
//
// Return: me client id or 0 if client is not connected
//
// mei_cl_mtu - maximal message that client can send and receive
//
// @cl: host client
//
// Return: mtu or 0 if client is not connected
//
// mei_cl_is_fixed_address - check whether the me client uses fixed address
//
// @cl: host client
//
// Return: true if the client is connected and it has fixed me address
//
// mei_cl_is_single_recv_buf- check whether the me client
// uses single receiving buffer
//
// @cl: host client
//
// Return: true if single_recv_buf == 1; 0 otherwise
//
// mei_cl_uuid -  client's uuid
//
// @cl: host client
//
// Return: return uuid of connected me client
//
extern "C" {
    pub fn mei_me_cl_uuid(_arg: cl->me_cl) -> return;
}
//
// mei_cl_host_addr - client's host address
//
// @cl: host client
//
// Return: 0 for fixed address client, host address for dynamic client
//
extern "C" {
    pub fn mei_cl_disconnect(cl: *mut mei_cl) -> c_int;
}
extern "C" {
    pub fn mei_cl_read_start(cl: *mut mei_cl, length: usize, fp: *const file) -> c_int;
}
extern "C" {
    pub fn mei_cl_write(cl: *mut mei_cl, cb: *mut mei_cl_cb, timeout: c_ulong) -> isize;
}
extern "C" {
    pub fn mei_cl_complete(cl: *mut mei_cl, cb: *mut mei_cl_cb);
}
extern "C" {
    pub fn mei_host_client_init(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_cl_notify_fop2req(fop: mei_cb_file_ops) -> u8;
}
extern "C" {
    pub fn mei_cl_notify_req2fop(request: u8) -> mei_cb_file_ops;
}
extern "C" {
    pub fn mei_cl_notify_get(cl: *mut mei_cl, block: bool, notify_ev: *mut bool) -> c_int;
}
extern "C" {
    pub fn mei_cl_notify(cl: *mut mei_cl);
}
extern "C" {
    pub fn mei_cl_all_disconnect(dev: *mut mei_device);
}
extern "C" {
    pub fn mei_cl_dma_unmap(cl: *mut mei_cl, fp: *const file) -> c_int;
}

