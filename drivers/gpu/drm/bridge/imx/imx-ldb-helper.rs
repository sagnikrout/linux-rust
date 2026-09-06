//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/imx/imx-ldb-helper.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019,2020,2022 NXP
//

pub const MAX_LDB_CHAN_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ldb_channel_link_type {
    LDB_CH_SINGLE_LINK,
    LDB_CH_DUAL_LINK_EVEN_ODD_PIXELS,
    LDB_CH_DUAL_LINK_ODD_EVEN_PIXELS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldb_channel {
    pub ldb: *mut ldb,
    pub bridge: drm_bridge,
    pub next_bridge: *mut drm_bridge,
    pub np: *mut device_node,
    pub chno: u32,
    pub is_available: bool,
    pub in_bus_format: u32,
    pub out_bus_format: u32,
    pub link_type: ldb_channel_link_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldb {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub channel: [*mut ldb_channel; MAX_LDB_CHAN_NUM],
    pub ctrl_reg: c_uint,
    pub ldb_ctrl: u32,
    pub available_ch_cnt: c_uint,
}

extern "C" {
    pub fn ldb_channel_is_single_link(ldb_ch: *mut ldb_channel) -> bool;
}
extern "C" {
    pub fn ldb_channel_is_split_link(ldb_ch: *mut ldb_channel) -> bool;
}
extern "C" {
    pub fn ldb_bridge_enable_helper(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn ldb_bridge_disable_helper(bridge: *mut drm_bridge);
}
extern "C" {
    pub fn ldb_init_helper(ldb: *mut ldb) -> c_int;
}
extern "C" {
    pub fn ldb_find_next_bridge_helper(ldb: *mut ldb) -> c_int;
}
extern "C" {
    pub fn ldb_add_bridge_helper(ldb: *mut ldb);
}
extern "C" {
    pub fn ldb_remove_bridge_helper(ldb: *mut ldb);
}
