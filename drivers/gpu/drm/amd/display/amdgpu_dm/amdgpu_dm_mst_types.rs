//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_mst_types.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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
pub const DP_BRANCH_DEVICE_ID_90CC24: c_uint = 0x90CC24;
pub const SYNAPTICS_RC_COMMAND: c_uint = 0x4B2;
pub const SYNAPTICS_RC_RESULT: c_uint = 0x4B3;
pub const SYNAPTICS_RC_LENGTH: c_uint = 0x4B8;
pub const SYNAPTICS_RC_OFFSET: c_uint = 0x4BC;
pub const SYNAPTICS_RC_DATA: c_uint = 0x4C0;
pub const DP_BRANCH_VENDOR_SPECIFIC_START: c_uint = 0x50C;
//
// Panamera MST Hub detection
// Offset DPCD 050Eh == 0x5A indicates cascaded MST hub case
// Check from beginning of branch device vendor specific field (050Ch)
//

pub const BRANCH_HW_REVISION_PANAMERA_A2: c_uint = 0x10;
pub const SYNAPTICS_CASCADED_HUB_ID: c_uint = 0x5A;

pub const PBN_FEC_OVERHEAD_MULTIPLIER_8B_10B: c_int = 1031;
pub const PBN_FEC_OVERHEAD_MULTIPLIER_128B_132B: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mst_msg_ready_type {
    NONE_MSG_RDY_EVENT = 0,
    DOWN_REP_MSG_RDY_EVENT = 1,
    UP_REQ_MSG_RDY_EVENT = 2,
    DOWN_OR_UP_MSG_RDY_EVENT = 3
}

extern "C" {
    pub fn dm_mst_get_pbn_divider(link: *mut dc_link) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_mst_fairness_vars {
    pub pbn: c_int,
    pub dsc_enabled: bool,
    pub bpp_x16: c_int,
    pub aconnector: *mut amdgpu_dm_connector,
}

extern "C" {
    pub fn needs_dsc_aux_workaround(link: *mut dc_link) -> bool;
}

extern "C" {
    pub fn amdgpu_dm_mst_reset_mst_connector_setting(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn retrieve_downstream_port_device(aconnector: *mut amdgpu_dm_connector) -> bool;
}
extern "C" {
    pub fn retrieve_branch_specific_data(aconnector: *mut amdgpu_dm_connector) -> bool;
}
extern "C" {
    pub fn dm_dp_aux_fill_payload_flags(request: u8, payload: *mut aux_payload);
}
extern "C" {
    pub fn dm_dp_aux_transfer(aux: *mut drm_dp_aux, msg: *mut drm_dp_aux_msg) -> isize;
}
extern "C" {
    pub fn dm_mst_msg_ready_mask(msg_rdy_type: mst_msg_ready_type) -> u8;
}
extern "C" {
    pub fn dm_mst_select_esi_dpcd(dpcd_rev: u8, dpcd_addr: *mut c_int, dpcd_bytes_to_read: *mut u8);
}
extern "C" {
    pub fn dm_handle_mst_down_rep_msg_ready(mgr: *mut drm_dp_mst_topology_mgr);
}

