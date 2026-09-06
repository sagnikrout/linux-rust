//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/server.h
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
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

//
// Server state type
//
// Server global config string index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_server_config {
    pub flags: c_uint,
    pub state: c_uint,
    pub signing: c_short,
    pub enforced_signing: c_short,
    pub min_protocol: c_short,
    pub max_protocol: c_short,
    pub tcp_port: c_ushort,
    pub ipc_timeout: c_ushort,
    pub ipc_last_active: c_ulong,
    pub deadtime: c_ulong,
    pub share_fake_fscaps: c_uint,
    pub domain_sid: smb_sid,
    pub auth_mechs: c_uint,
    pub max_connections: c_uint,
    pub max_inflight_req: c_uint,
    pub max_ip_connections: c_uint,
    pub 1]: *mut *mut char conf[SERVER_CONF_WORK_GROUP +,
    pub dh_task: *mut task_struct,
    pub bind_interfaces_only: bool,
// AAPL model string for Finder icon, e.g. "Xserve"
    pub aapl_model: [c_char; 32],
}

extern "C" {
    pub fn ksmbd_set_netbios_name(v: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ksmbd_set_server_string(v: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ksmbd_set_work_group(v: *mut c_char) -> c_int;
}
extern "C" {
    pub fn server_queue_ctrl_init_work() -> c_int;
}
extern "C" {
    pub fn server_queue_ctrl_reset_work() -> c_int;
}
