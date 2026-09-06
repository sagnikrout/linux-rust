//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/usb/extron-da-hd-4k-plus/cec-splitter.h
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
// Copyright 2021-2024 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
pub const STATE_CHANGE_MAX_REPEATS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_splitter_port {
    pub splitter: *mut cec_splitter,
    pub adap: *mut cec_adapter,
    pub port: c_uint,
    pub is_active_source: bool,
    pub found_sink: bool,
    pub lost_sink_ts: ktime_t,
    pub out_request_current_latency_seq: u32,
    pub out_request_current_latency_ts: ktime_t,
    pub video_latency: u8,
    pub out_give_device_power_status_seq: u32,
    pub out_give_device_power_status_ts: ktime_t,
    pub power_status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_splitter {
    pub dev: *mut device,
    pub num_out_ports: c_uint,
    pub ports: *mut cec_splitter_port,
// High-level splitter state
    pub request_current_latency_dest: u8,
    pub give_device_power_status_dest: u8,
    pub is_standby: bool,
}

extern "C" {
    pub fn cec_splitter_unconfigured_output(port: *mut cec_splitter_port);
}
extern "C" {
    pub fn cec_splitter_configured_output(port: *mut cec_splitter_port);
}
extern "C" {
    pub fn cec_splitter_received_input(port: *mut cec_splitter_port, msg: *mut cec_msg) -> c_int;
}
