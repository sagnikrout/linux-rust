//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_connector.h
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
// Copyright © 2019 Intel Corporation
//

extern "C" {
    pub fn intel_connector_free(connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_connector_destroy(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_connector_register(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn intel_connector_unregister(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_connector_get_hw_state(connector: *mut intel_connector) -> bool;
}
extern "C" {
    pub fn intel_connector_get_pipe(connector: *mut intel_connector) -> pipe;
}
extern "C" {
    pub fn intel_ddc_get_modes(c: *mut drm_connector, ddc: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn intel_attach_force_audio_property(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_attach_broadcast_rgb_property(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_attach_aspect_ratio_property(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_attach_hdmi_colorspace_property(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_attach_dp_colorspace_property(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_attach_scaling_mode_property(connector: *mut drm_connector);
}
extern "C" {
    pub fn intel_connector_queue_modeset_retry_work(connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_connector_cancel_modeset_retry_work(connector: *mut intel_connector);
}
