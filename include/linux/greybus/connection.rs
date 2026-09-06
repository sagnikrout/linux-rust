//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/connection.h
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
// Greybus connections
//
// Copyright 2014 Google Inc.
// Copyright 2014 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_connection_state {
    GB_CONNECTION_STATE_DISABLED		= 0,
    GB_CONNECTION_STATE_ENABLED_TX		= 1,
    GB_CONNECTION_STATE_ENABLED		= 2,
    GB_CONNECTION_STATE_DISCONNECTING	= 3,
}

extern "C" {
    pub fn int(: *mut *mut gb_request_handler_t)(struct gb_operation) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_connection {
    pub hd: *mut gb_host_device,
    pub intf: *mut gb_interface,
    pub bundle: *mut gb_bundle,
    pub kref: kref,
    pub hd_cport_id: u16,
    pub intf_cport_id: u16,
    pub hd_links: list_head,
    pub bundle_links: list_head,
    pub handler: gb_request_handler_t,
    pub flags: c_ulong,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub state: gb_connection_state,
    pub operations: list_head,
    pub name: [c_char; 16],
    pub wq: *mut workqueue_struct,
    pub op_cycle: core::sync::atomic::AtomicI32,
    pub private: *mut c_void,
    pub mode_switch: bool,
}

extern "C" {
    pub fn gb_connection_destroy(connection: *mut gb_connection);
}
extern "C" {
    pub fn gb_connection_enable(connection: *mut gb_connection) -> c_int;
}
extern "C" {
    pub fn gb_connection_enable_tx(connection: *mut gb_connection) -> c_int;
}
extern "C" {
    pub fn gb_connection_disable_rx(connection: *mut gb_connection);
}
extern "C" {
    pub fn gb_connection_disable(connection: *mut gb_connection);
}
extern "C" {
    pub fn gb_connection_disable_forced(connection: *mut gb_connection);
}
extern "C" {
    pub fn gb_connection_mode_switch_prepare(connection: *mut gb_connection);
}
extern "C" {
    pub fn gb_connection_mode_switch_complete(connection: *mut gb_connection);
}
extern "C" {
    pub fn gb_connection_latency_tag_enable(connection: *mut gb_connection);
}
extern "C" {
    pub fn gb_connection_latency_tag_disable(connection: *mut gb_connection);
}
