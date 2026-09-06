//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_fw.h
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
// Copyright (C) 2020-2025 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_fw_info {
    pub file: *const firmware,
    pub name: *const c_char,
    pub version: [c_char; FW_VERSION_STR_SIZE],
    pub mem_bp: *mut ivpu_bo,
    pub mem_fw_ver: *mut ivpu_bo,
    pub mem: *mut ivpu_bo,
    pub mem_shave_nn: *mut ivpu_bo,
    pub mem_log_crit: *mut ivpu_bo,
    pub mem_log_verb: *mut ivpu_bo,
    pub boot_params_addr: u64,
    pub boot_params_size: u64,
    pub fw_version_addr: u64,
    pub fw_version_size: u64,
    pub runtime_addr: u64,
    pub runtime_size: u32,
    pub image_load_offset: u64,
    pub image_size: u32,
    pub shave_nn_size: u32,
    pub warm_boot_entry_point: u64,
    pub cold_boot_entry_point: u64,
    pub last_boot_mode: u8,
    pub next_boot_mode: u8,
    pub trace_level: u32,
    pub trace_destination_mask: u32,
    pub trace_hw_component_mask: u64,
    pub dvfs_mode: u32,
    pub primary_preempt_buf_size: u32,
    pub secondary_preempt_buf_size: u32,
    pub read_only_addr: u64,
    pub read_only_size: u32,
    pub sched_mode: u32,
    pub last_heartbeat: u64,
}

extern "C" {
    pub fn ivpu_is_within_range(addr: u64, size: usize, range: *mut ivpu_addr_range) -> bool;
}
extern "C" {
    pub fn ivpu_fw_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_fw_fini(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_fw_load(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_fw_boot_params_setup(vdev: *mut ivpu_device, boot_params: *mut vpu_boot_params);
}
