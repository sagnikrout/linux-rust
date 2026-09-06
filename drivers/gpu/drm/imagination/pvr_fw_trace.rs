//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_fw_trace.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declaration from pvr_device.h.
// Forward declaration from pvr_gem.h.
// Forward declarations from pvr_rogue_fwif.h
//
// struct pvr_fw_trace_buffer - Structure representing a trace buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_trace_buffer {
// @buf_obj: FW buffer object representing trace buffer.
    pub buf_obj: *mut pvr_fw_object,
// @buf: Pointer to CPU mapping of trace buffer.
    pub buf: *mut u32,
//
// @tracebuf_space: Pointer to FW tracebuf_space structure for this
// trace buffer.
//
    pub tracebuf_space: *mut rogue_fwif_tracebuf_space,
}

//
// struct pvr_fw_trace - Device firmware trace data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_trace {
//
// @tracebuf_ctrl_obj: Object representing FW trace buffer control
// structure.
//
    pub tracebuf_ctrl_obj: *mut pvr_fw_object,
//
// @tracebuf_ctrl: Pointer to CPU mapping of FW trace buffer control
// structure.
//
    pub tracebuf_ctrl: *mut rogue_fwif_tracebuf,
//
// @buffers: Array representing the actual trace buffers owned by this
// device.
//
    pub buffers: [pvr_fw_trace_buffer; ROGUE_FW_THREAD_MAX],
// @group_mask: Mask of enabled trace groups.
    pub group_mask: u32,
}

extern "C" {
    pub fn pvr_fw_trace_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_fw_trace_fini(pvr_dev: *mut pvr_device);
}
// Forward declaration from <linux/dcache.h>.
extern "C" {
    pub fn pvr_fw_trace_debugfs_init(pvr_dev: *mut pvr_device, dir: *mut dentry);
}
