//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dtpm.h
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
// Copyright (C) 2020 Linaro Ltd
//
// Author: Daniel Lezcano <daniel.lezcano@linaro.org>
//

pub const MAX_DTPM_DESCR: c_int = 8;
pub const MAX_DTPM_CONSTRAINTS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtpm {
    pub zone: powercap_zone,
    pub parent: *mut dtpm,
    pub sibling: list_head,
    pub children: list_head,
    pub ops: *mut dtpm_ops,
    pub flags: c_ulong,
    pub power_limit: u64,
    pub power_max: u64,
    pub power_min: u64,
    pub weight: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtpm_ops {
    pub u64): *mut *mut *mut u64 (set_power_uw)(struct dtpm ,,
    pub ): *mut *mut u64 (get_power_uw)(struct dtpm,
    pub ): *mut *mut int (update_power_uw)(struct dtpm,
    pub ): *mut *mut void (release)(struct dtpm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtpm_subsys_ops {
    pub name: *const c_char,
    pub (*init)(void): *mut c_int,
    pub (*exit)(void): *mut c_void,
    pub ): *mut *mut *mut int (setup)(struct dtpm , struct device_node,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DTPM_NODE_TYPE {
    DTPM_NODE_VIRTUAL = 0,
    DTPM_NODE_DT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtpm_node {
    pub type: DTPM_NODE_TYPE,
    pub name: *const c_char,
    pub parent: *mut dtpm_node,
}

extern "C" {
    pub fn container_of(_arg: zone, dtpm: struct, _arg: zone) -> return;
}
extern "C" {
    pub fn dtpm_update_power(dtpm: *mut dtpm) -> c_int;
}
extern "C" {
    pub fn dtpm_release_zone(pcz: *mut powercap_zone) -> c_int;
}
extern "C" {
    pub fn dtpm_init(dtpm: *mut dtpm, ops: *mut dtpm_ops);
}
extern "C" {
    pub fn dtpm_unregister(dtpm: *mut dtpm);
}
extern "C" {
    pub fn dtpm_register(name: *const c_char, dtpm: *mut dtpm, parent: *mut dtpm) -> c_int;
}
extern "C" {
    pub fn dtpm_create_hierarchy(dtpm_match_table: *mut of_device_id) -> c_int;
}
extern "C" {
    pub fn dtpm_destroy_hierarchy();
}
