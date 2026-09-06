//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/cluster/heartbeat.h
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
// heartbeat.h
//
// Function prototypes
//
// Copyright (C) 2004 Oracle.  All rights reserved.
//

pub const O2HB_REGION_TIMEOUT_MS: c_int = 2000;
pub const O2HB_MAX_REGION_NAME_LEN: c_int = 32;
// number of changes to be seen as live
pub const O2HB_LIVE_THRESHOLD: c_int = 2;
// number of equal samples to be seen as dead
pub const O2HB_DEFAULT_DEAD_THRESHOLD: c_int = 31;
// Otherwise MAX_WRITE_TIMEOUT will be zero...
pub const O2HB_MIN_DEAD_THRESHOLD: c_int = 2;

pub const O2HB_CB_MAGIC: c_uint = 0x51d1e4ec;
// callback stuff
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum o2hb_callback_type {
    O2HB_NODE_DOWN_CB = 0,
    O2HB_NODE_UP_CB,
    O2HB_NUM_CB
}

extern "C" {
    pub fn void(: *mut o2hb_cb_func)(struct o2nm_node, _arg: c_int, : *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2hb_callback_func {
    pub hc_magic: u32,
    pub hc_item: list_head,
    pub hc_func: *mut o2hb_cb_func,
    pub hc_data: *mut c_void,
    pub hc_priority: c_int,
    pub hc_type: o2hb_callback_type,
}

extern "C" {
    pub fn o2hb_free_hb_set(group: *mut config_group);
}
extern "C" {
    pub fn o2hb_callback_read_lock();
}
extern "C" {
    pub fn o2hb_callback_read_unlock();
}
extern "C" {
    pub fn o2hb_synchronize_callbacks();
}
extern "C" {
    pub fn o2hb_exit();
}
extern "C" {
    pub fn o2hb_init();
}
extern "C" {
    pub fn o2hb_check_node_heartbeating_no_sem(node_num: u8) -> c_int;
}
extern "C" {
    pub fn o2hb_check_node_heartbeating_from_callback(node_num: u8) -> c_int;
}
extern "C" {
    pub fn o2hb_stop_all_regions();
}
extern "C" {
    pub fn o2hb_get_all_regions(region_uuids: *mut c_char, numregions: u8) -> c_int;
}
extern "C" {
    pub fn o2hb_global_heartbeat_active() -> c_int;
}
