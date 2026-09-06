//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_connector.h
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
// Copyright 2026 Advanced Micro Devices, Inc.
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
extern "C" {
    pub fn amdgpu_dm_connector_funcs_reset(connector: *mut drm_connector);
}
extern "C" {
    pub fn amdgpu_dm_hdmi_cec_set_edid(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn amdgpu_dm_initialize_hdmi_connector(aconnector: *mut amdgpu_dm_connector) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_convert_dc_color_depth_into_bpc(display_color_depth: dc_color_depth) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_s3_handle_hdmi_cec(ddev: *mut drm_device, suspend: bool);
}
extern "C" {
    pub fn amdgpu_dm_detect_mst_link_for_all_connectors(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_set_panel_orientation(connector: *mut drm_connector);
}
pub const DDC_MANUFACTURERNAME_SAMSUNG: c_uint = 0x2D4C;
// Encoder functions
extern "C" {
    pub fn amdgpu_dm_get_encoder_crtc_mask(adev: *mut amdgpu_device) -> c_int;
}

extern "C" {
    pub fn amdgpu_dm_i2c_func(adap: *mut i2c_adapter) -> u32;
}
extern "C" {
    pub fn amdgpu_dm_connector_funcs_force(connector: *mut drm_connector);
}
extern "C" {
    pub fn amdgpu_dm_get_native_mode(connector: *mut drm_connector);
}
extern "C" {
    pub fn add_fs_modes(aconnector: *mut amdgpu_dm_connector) -> c_uint;
}
extern "C" {
    pub fn hdmi_cec_unset_edid(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn create_eml_sink(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn handle_edid_mgmt(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn dm_encoder_helper_disable(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn get_subconnector_type(link: *mut dc_link) -> drm_mode_subconnector;
}
extern "C" {
    pub fn update_subconnector_property(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn amdgpu_dm_fbc_init(connector: *mut drm_connector);
}
extern "C" {
    pub fn amdgpu_dm_set_panel_type(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn amdgpu_dm_update_cacp_caps(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn amdgpu_dm_connector_unregister(connector: *mut drm_connector);
}
extern "C" {
    pub fn amdgpu_dm_connector_late_register(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_connector_destroy(connector: *mut drm_connector);
}
extern "C" {
    pub fn to_drm_connector_type(st: signal_type, connector_id: u32) -> c_int;
}
extern "C" {
    pub fn is_duplicate_mode(aconnector: *mut amdgpu_dm_connector, mode: *mut drm_display_mode) -> bool;
}
extern "C" {
    pub fn get_aspect_ratio(mode_in: *const drm_display_mode) -> dc_aspect_ratio;
}
extern "C" {
    pub fn amdgpu_dm_set_panel_type(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn amdgpu_dm_update_cacp_caps(aconnector: *mut amdgpu_dm_connector);
}

