//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_hdcp.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

//
// Minimal declarations needed by this header.
// Full amdgpu/DM definitions come from amdgpu_dm.h included by each .c file.
//
pub const AMDGPU_DM_MAX_DISPLAY_COUNT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp_workqueue {
    pub cpirq_work: work_struct,
    pub property_update_work: work_struct,
    pub callback_dwork: delayed_work,
    pub watchdog_timer_dwork: delayed_work,
    pub property_validate_dwork: delayed_work,
    pub aconnector: [*mut amdgpu_dm_connector; AMDGPU_DM_MAX_DISPLAY_COUNT],
    pub mutex: mutex,
    pub hdcp: mod_hdcp,
    pub output: mod_hdcp_output,
    pub display: mod_hdcp_display,
    pub link: mod_hdcp_link,
    pub encryption_status: [mod_hdcp_encryption_status; AMDGPU_DM_MAX_DISPLAY_COUNT],
// when display is unplugged from mst hub, connctor will be
// destroyed within dm_dp_mst_connector_destroy. connector
// hdcp perperties, like type, undesired, desired, enabled,
// will be lost. So, save hdcp properties into hdcp_work within
// amdgpu_dm_atomic_commit_tail. if the same display is
// plugged back with same display index, its hdcp properties
// will be retrieved from hdcp_work within dm_dp_mst_get_modes
//
// un-desired, desired, enabled
    pub content_protection: [c_uint; AMDGPU_DM_MAX_DISPLAY_COUNT],
// hdcp1.x, hdcp2.x
    pub hdcp_content_type: [c_uint; AMDGPU_DM_MAX_DISPLAY_COUNT],
    pub max_link: u8,
    pub srm: *mut u8,
    pub srm_temp: *mut u8,
    pub srm_version: u32,
    pub srm_size: u32,
    pub attr: bin_attribute,
}

extern "C" {
    pub fn hdcp_reset_display(work: *mut hdcp_workqueue, link_index: c_uint);
}
extern "C" {
    pub fn hdcp_handle_cpirq(work: *mut hdcp_workqueue, link_index: c_uint);
}
extern "C" {
    pub fn hdcp_destroy(kobj: *mut kobject, work: *mut hdcp_workqueue);
}

extern "C" {
    pub fn process_output(hdcp_work: *mut hdcp_workqueue);
}
extern "C" {
    pub fn event_property_update(work: *mut work_struct);
}
extern "C" {
    pub fn event_property_validate(work: *mut work_struct);
}
extern "C" {
    pub fn event_callback(work: *mut work_struct);
}
extern "C" {
    pub fn event_watchdog_timer(work: *mut work_struct);
}
extern "C" {
    pub fn event_cpirq(work: *mut work_struct);
}
extern "C" {
    pub fn link_lock(work: *mut hdcp_workqueue, lock: bool);
}
extern "C" {
    pub fn psp_set_srm(psp: *mut psp_context, srm: *mut u8, srm_size: u32, srm_version: *mut u32) -> c_int;
}
extern "C" {
    pub fn enable_assr(handle: *mut c_void, link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn update_config(handle: *mut c_void, config: *mut cp_psp_stream_config);
}
extern "C" {
    pub fn lp_write_i2c(handle: *mut c_void, address: u32, data: *const u8, size: u32) -> bool;
}
extern "C" {
    pub fn lp_read_i2c(handle: *mut c_void, address: u32, offset: u8, data: *mut u8, size: u32) -> bool;
}
extern "C" {
    pub fn lp_write_dpcd(handle: *mut c_void, address: u32, data: *const u8, size: u32) -> bool;
}
extern "C" {
    pub fn lp_read_dpcd(handle: *mut c_void, address: u32, data: *mut u8, size: u32) -> bool;
}

