//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/usb/extron-da-hd-4k-plus/extron-da-hd-4k-plus.h
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

pub const DATA_SIZE: c_int = 256;

pub const MAX_EDID_BLOCKS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extron_port {
    pub port: cec_splitter_port,
    pub dev: *mut device,
    pub adap: *mut cec_adapter,
    pub vdev: video_device,
    pub hdl: v4l2_ctrl_handler,
    pub ctrl_rx_power_present: *mut v4l2_ctrl,
    pub ctrl_tx_hotplug: *mut v4l2_ctrl,
    pub ctrl_tx_edid_present: *mut v4l2_ctrl,
    pub is_input: bool,
    pub direction: c_char,
    pub name: [c_char; 26],
    pub 128]: *mut *mut unsigned char edid[MAX_EDID_BLOCKS,
    pub 128]: *mut *mut unsigned char edid_tmp[MAX_EDID_BLOCKS,
    pub edid_blocks: c_uint,
    pub read_edid: bool,
    pub extron: *mut extron,
    pub irq_work: work_struct,
    pub cmd_done: completion,
    pub response: *const c_char,
    pub cmd_error: c_uint,
    pub rx_msg: [cec_msg; NUM_MSGS],
    pub rx_msg_num: unsigned int rx_msg_cur_idx,,
// protect rx_msg_cur_idx and rx_msg_num
    pub msg_lock: spinlock_t,
    pub tx_done_status: u32,
    pub update_phys_addr: bool,
    pub phys_addr: u16,
    pub cec_was_registered: bool,
    pub disconnected: bool,
    pub update_has_signal: bool,
    pub has_signal: bool,
    pub update_has_edid: bool,
    pub has_edid: bool,
    pub has_4kp30: bool,
    pub has_4kp60: bool,
    pub has_qy: bool,
    pub has_qs: bool,
    pub est_ii: u8 est_i,,
// locks access to the video_device
    pub video_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extron {
    pub splitter: cec_splitter,
    pub dev: *mut device,
    pub serio: *mut serio,
// locks access to serio
    pub serio_lock: mutex,
    pub num_ports: c_uint,
    pub num_in_ports: c_uint,
    pub num_out_ports: c_uint,
    pub unit_name: [c_char; 32],
    pub unit_type: [c_char; 64],
    pub unit_fw_version: [c_char; 32],
    pub unit_cec_engine_version: [c_char; 32],
    pub ports: [*mut extron_port; MAX_PORTS],
    pub splitter_ports: [*mut cec_splitter_port; MAX_PORTS],
    pub v4l2_dev: v4l2_device,
    pub hpd_never_low: bool,
    pub kthread_setup: *mut task_struct,
    pub work_update_edid: delayed_work,
// serializes EDID reading
    pub edid_lock: mutex,
    pub edid_bytes_read: c_uint,
    pub edid_port: *mut extron_port,
    pub edid_completion: completion,
    pub edid_reading: bool,
    pub is_ready: bool,
    pub cmd_done: completion,
    pub response: *const c_char,
    pub cmd_error: c_uint,
    pub data: [c_char; DATA_SIZE],
    pub len: c_uint,
    pub reply: [c_char; DATA_SIZE],
    pub buf: [c_char; DATA_SIZE],
    pub idx: c_uint,
}
