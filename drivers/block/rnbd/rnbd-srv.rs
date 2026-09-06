//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/rnbd/rnbd-srv.h
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
// RDMA Network Block Driver
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_srv_session {
// Entry inside global sess_list
    pub list: list_head,
    pub rtrs: *mut rtrs_srv_sess,
    pub sessname: [c_char; NAME_MAX],
    pub queue_depth: c_int,
    pub index_idr: xarray,
    pub lock: mutex,
    pub ver: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_srv_dev {
// Entry inside global dev_list
    pub list: list_head,
    pub dev_kobj: kobject,
    pub dev_sessions_kobj: *mut kobject,
    pub kref: kref,
    pub name: [c_char; NAME_MAX],
// List of rnbd_srv_sess_dev structs
    pub sess_dev_list: list_head,
    pub lock: mutex,
    pub open_write_cnt: c_int,
}

// Structure which binds N devices and N sessions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_srv_sess_dev {
// Entry inside rnbd_srv_dev struct
    pub dev_list: list_head,
    pub bdev_file: *mut file,
    pub sess: *mut rnbd_srv_session,
    pub dev: *mut rnbd_srv_dev,
    pub kobj: kobject,
    pub device_id: u32,
    pub keep_id: bool,
    pub readonly: bool,
    pub kref: kref,
    pub destroy_comp: *mut completion,
    pub pathname: [c_char; NAME_MAX],
    pub access_mode: rnbd_access_mode,
}

// rnbd-srv-sysfs.c
extern "C" {
    pub fn rnbd_srv_destroy_dev_sysfs(dev: *mut rnbd_srv_dev);
}
extern "C" {
    pub fn rnbd_srv_create_dev_session_sysfs(sess_dev: *mut rnbd_srv_sess_dev) -> c_int;
}
extern "C" {
    pub fn rnbd_srv_destroy_dev_session_sysfs(sess_dev: *mut rnbd_srv_sess_dev);
}
extern "C" {
    pub fn rnbd_srv_create_sysfs_files() -> c_int;
}
extern "C" {
    pub fn rnbd_srv_destroy_sysfs_files();
}
extern "C" {
    pub fn rnbd_destroy_sess_dev(sess_dev: *mut rnbd_srv_sess_dev, keep_id: bool);
}
