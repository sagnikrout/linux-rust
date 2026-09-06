//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/smartpqi/smartpqi_sis.h
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


// SPDX-License-Identifier: GPL-2.0
//
// driver for Microchip PQI-based storage controllers
// Copyright (c) 2019-2023 Microchip Technology Inc. and its subsidiaries
// Copyright (c) 2016-2018 Microsemi Corporation
// Copyright (c) 2016 PMC-Sierra, Inc.
//
// Questions/Comments/Bugfixes to storagedev@microchip.com
//

extern "C" {
    pub fn sis_verify_structures();
}
extern "C" {
    pub fn sis_wait_for_ctrl_ready(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_wait_for_ctrl_ready_resume(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_is_firmware_running(ctrl_info: *mut pqi_ctrl_info) -> bool;
}
extern "C" {
    pub fn sis_is_kernel_up(ctrl_info: *mut pqi_ctrl_info) -> bool;
}
extern "C" {
    pub fn sis_get_ctrl_properties(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_get_pqi_capabilities(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_init_base_struct_addr(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_enable_msix(ctrl_info: *mut pqi_ctrl_info);
}
extern "C" {
    pub fn sis_enable_intx(ctrl_info: *mut pqi_ctrl_info);
}
extern "C" {
    pub fn sis_pqi_reset_quiesce(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_reenable_sis_mode(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_write_driver_scratch(ctrl_info: *mut pqi_ctrl_info, value: u32);
}
extern "C" {
    pub fn sis_read_driver_scratch(ctrl_info: *mut pqi_ctrl_info) -> u32;
}
extern "C" {
    pub fn sis_soft_reset(ctrl_info: *mut pqi_ctrl_info);
}
extern "C" {
    pub fn sis_get_product_id(ctrl_info: *mut pqi_ctrl_info) -> u32;
}
extern "C" {
    pub fn sis_wait_for_fw_triage_completion(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn sis_is_ctrl_logging_supported(ctrl_info: *mut pqi_ctrl_info) -> bool;
}
extern "C" {
    pub fn sis_notify_kdump(ctrl_info: *mut pqi_ctrl_info);
}
extern "C" {
    pub fn sis_wait_for_ctrl_logging_completion(ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
