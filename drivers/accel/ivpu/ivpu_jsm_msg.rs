//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_jsm_msg.h
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
// Copyright (C) 2020-2026 Intel Corporation
//

extern "C" {
    pub fn ivpu_jsm_unregister_db(vdev: *mut ivpu_device, db_id: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_get_heartbeat(vdev: *mut ivpu_device, engine: u32, heartbeat: *mut u64) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_reset_engine(vdev: *mut ivpu_device, engine: u32, response: *mut vpu_jsm_msg) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_preempt_engine(vdev: *mut ivpu_device, engine: u32, preempt_id: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_dyndbg_control(vdev: *mut ivpu_device, command: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_context_release(vdev: *mut ivpu_device, host_ssid: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_pwr_d0i3_enter(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_hws_destroy_cmdq(vdev: *mut ivpu_device, ctx_id: u32, cmdq_id: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_hws_resume_engine(vdev: *mut ivpu_device, engine: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_hws_setup_priority_bands(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_metric_streamer_stop(vdev: *mut ivpu_device, metric_group_mask: u64) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_dct_enable(vdev: *mut ivpu_device, active_us: u32, inactive_us: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_dct_disable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_state_dump(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_state_dump_no_reply(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_jsm_msg_freq_config(vdev: *mut ivpu_device, min_ratio: u16, pn_ratio: u16, max_ratio: u16) -> c_int;
}
