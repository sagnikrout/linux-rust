//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/ctl.h
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
// Thunderbolt driver - control channel and configuration commands
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2018, Intel Corporation
//

// control channel
extern "C" {
    pub fn tb_ctl_start(ctl: *mut tb_ctl);
}
extern "C" {
    pub fn tb_ctl_stop(ctl: *mut tb_ctl);
}
extern "C" {
    pub fn tb_ctl_free(ctl: *mut tb_ctl);
}
// configuration commands
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cfg_result {
    pub response_route: u64,
    pub /*: *mut u32 response_port;,
// If err = 1 then this is the port that send the
// error.
// If err = 0 and if this was a cfg_read/write then
// this is the upstream port of the responding
// switch.
// Otherwise the field is set to zero.
//
    pub /: *mut *mut int err; / negative errors, 0 for success, 1 for tb errors,
    pub /: *mut *mut tb_cfg_error tb_error; / valid if err == 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_pkg {
    pub ctl: *mut tb_ctl,
    pub buffer: *mut c_void,
    pub frame: ring_frame,
}

//
// struct tb_cfg_request - Control channel request
// @kref: Reference count
// @ctl: Pointer to the control channel structure. Only set when the
// request is queued.
// @request: Request is stored here
// @request_size: Size of the request packet (in bytes)
// @request_type: Type of the request packet
// @response: Response is stored here
// @response_size: Maximum size of one response packet
// @response_type: Expected type of the response packet
// @npackets: Number of packets expected to be returned with this request
// @match: Function used to match the incoming packet
// @copy: Function used to copy the incoming packet to @response
// @callback: Callback called when the request is finished successfully
// @callback_data: Data to be passed to @callback
// @flags: Flags for the request
// @work: Work item used to complete the request
// @result: Result after the request has been completed
// @list: Requests are queued using this field
//
// An arbitrary request over Thunderbolt control channel. For standard
// control channel message, one should use tb_cfg_read/write() and
// friends if possible.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cfg_request {
    pub kref: kref,
    pub ctl: *mut tb_ctl,
    pub request: *const c_void,
    pub request_size: usize,
    pub request_type: tb_cfg_pkg_type,
    pub response: *mut c_void,
    pub response_size: usize,
    pub response_type: tb_cfg_pkg_type,
    pub npackets: usize,
    pub pkg): *const ctl_pkg,
    pub pkg): *const *const *const bool (copy)(struct tb_cfg_request req, struct ctl_pkg,
    pub callback_data): *mut *mut void (callback)(void,
    pub callback_data: *mut c_void,
    pub flags: c_ulong,
    pub work: work_struct,
    pub result: tb_cfg_result,
    pub list: list_head,
}

pub const TB_CFG_REQUEST_ACTIVE: c_int = 0;
pub const TB_CFG_REQUEST_CANCELED: c_int = 1;
extern "C" {
    pub fn tb_cfg_request_get(req: *mut tb_cfg_request);
}
extern "C" {
    pub fn tb_cfg_request_put(req: *mut tb_cfg_request);
}
extern "C" {
    pub fn tb_cfg_request_cancel(req: *mut tb_cfg_request, err: c_int);
}
// check for overflow, route_hi is not 32 bits!
extern "C" {
    pub fn tb_cfg_ack_plug(ctl: *mut tb_ctl, route: u64, port: u32, unplug: bool) -> c_int;
}
extern "C" {
    pub fn tb_cfg_reset(ctl: *mut tb_ctl, route: u64) -> tb_cfg_result;
}
extern "C" {
    pub fn tb_cfg_get_upstream_port(ctl: *mut tb_ctl, route: u64) -> c_int;
}
