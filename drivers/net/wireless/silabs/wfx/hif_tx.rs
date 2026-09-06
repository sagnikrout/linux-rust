//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/hif_tx.h
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
// Implementation of the host-to-chip commands (aka request/confirmation) of the
// hardware API.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
// Copyright (C) 2010, ST-Ericsson SA
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cmd {
    pub lock: mutex,
    pub ready: completion,
    pub done: completion,
    pub buf_send: *mut wfx_hif_msg,
    pub buf_recv: *mut c_void,
    pub len_recv: usize,
    pub ret: c_int,
}

extern "C" {
    pub fn wfx_init_hif_cmd(wfx_hif_cmd: *mut wfx_hif_cmd);
}
extern "C" {
    pub fn wfx_hif_read_mib(wdev: *mut wfx_dev, vif_id: c_int, mib_id: u16, buf: *mut c_void, buf_size: usize) -> c_int;
}
extern "C" {
    pub fn wfx_hif_write_mib(wdev: *mut wfx_dev, vif_id: c_int, mib_id: u16, buf: *mut c_void, buf_size: usize) -> c_int;
}
extern "C" {
    pub fn wfx_hif_reset(wvif: *mut wfx_vif, reset_stat: bool) -> c_int;
}
extern "C" {
    pub fn wfx_hif_map_link(wvif: *mut wfx_vif, unmap: bool, mac_addr: *mut u8, sta_id: c_int, mfp: bool) -> c_int;
}
extern "C" {
    pub fn wfx_hif_add_key(wdev: *mut wfx_dev, arg: *const wfx_hif_req_add_key) -> c_int;
}
extern "C" {
    pub fn wfx_hif_remove_key(wdev: *mut wfx_dev, idx: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_pm(wvif: *mut wfx_vif, ps: bool, dynamic_ps_timeout: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_set_bss_params(wvif: *mut wfx_vif, aid: c_int, beacon_lost_count: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_beacon_transmit(wvif: *mut wfx_vif, enable: bool) -> c_int;
}
extern "C" {
    pub fn wfx_hif_update_ie_beacon(wvif: *mut wfx_vif, ies: *const u8, ies_len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_hif_scan_uniq(wvif: *mut wfx_vif, chan: *mut ieee80211_channel, duration: c_int) -> c_int;
}
extern "C" {
    pub fn wfx_hif_stop_scan(wvif: *mut wfx_vif) -> c_int;
}
extern "C" {
    pub fn wfx_hif_configuration(wdev: *mut wfx_dev, conf: *const u8, len: usize) -> c_int;
}
extern "C" {
    pub fn wfx_hif_shutdown(wdev: *mut wfx_dev) -> c_int;
}
