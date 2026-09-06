//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_late_bind_fw_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

//
// enum xe_late_bind_fw_id - enum to determine late binding fw index
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_late_bind_fw_id {
// @XE_LB_FW_FAN_CONTROL: Fan control
    XE_LB_FW_FAN_CONTROL = 0,
// @XE_LB_FW_MAX_ID: Number of IDs
    XE_LB_FW_MAX_ID
}

//
// struct xe_late_bind_fw
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_late_bind_fw {
// @id: firmware index
    pub id: u32,
// @blob_path: firmware binary path
    pub blob_path: [c_char; PATH_MAX],
// @type: firmware type
    pub type: u32,
// @flags: firmware flags
    pub flags: u32,
// @payload: to store the late binding blob
    pub payload: *const u8,
// @payload_size: late binding blob payload_size
    pub payload_size: usize,
// @work: worker to upload latebind blob
    pub work: work_struct,
// @version: late binding blob manifest version
    pub version: gsc_version,
}

//
// struct xe_late_bind_component - Late Binding services component
// @mei_dev: device that provide Late Binding service.
// @ops: Ops implemented by Late Binding driver, used by Xe driver.
//
// Communication between Xe and MEI drivers for Late Binding services
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_late_bind_component {
    pub mei_dev: *mut device,
    pub ops: *const intel_lb_component_ops,
}

//
// struct xe_late_bind
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_late_bind {
// @component: struct for communication with mei component
    pub component: xe_late_bind_component,
// @late_bind_fw: late binding firmware array
    pub late_bind_fw: [xe_late_bind_fw; XE_LB_FW_MAX_ID],
// @wq: workqueue to submit request to download late bind blob
    pub wq: *mut workqueue_struct,
// @component_added: whether the component has been added
    pub component_added: bool,
// @disable: to block late binding reload during pm resume flow
    pub disable: bool,
}
