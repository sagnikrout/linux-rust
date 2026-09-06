//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/drm_crtc_internal.h
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


//
// Copyright © 2006 Keith Packard
// Copyright © 2007-2008 Dave Airlie
// Copyright © 2007-2008 Intel Corporation
// Jesse Barnes <jesse.barnes@intel.com>
// Copyright © 2014 Intel Corporation
// Daniel Vetter <daniel.vetter@ffwll.ch>
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
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
// This header file contains mode setting related functions and definitions
// which are only used within the drm module as internal implementation details
// and are not exported to drivers.
//

// drm_crtc.c
extern "C" {
    pub fn drm_crtc_register_all(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn drm_crtc_unregister_all(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_crtc_force_disable(crtc: *mut drm_crtc) -> c_int;
}
// IOCTLs
// drm_mode_config.c
extern "C" {
    pub fn drm_modeset_register_all(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn drm_modeset_unregister_all(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_mode_config_validate(dev: *mut drm_device);
}
// drm_modes.c
// IOCTLs
// drm_dumb_buffers.c
// IOCTLs
// drm_color_mgmt.c
// IOCTLs
// drm_property.c
// IOCTL
// drm_mode_object.c
// IOCTL
// drm_encoder.c
extern "C" {
    pub fn drm_encoder_register_all(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn drm_encoder_unregister_all(dev: *mut drm_device);
}
// IOCTL
// drm_connector.c
extern "C" {
    pub fn drm_connector_ida_init();
}
extern "C" {
    pub fn drm_connector_ida_destroy();
}
extern "C" {
    pub fn drm_connector_unregister_all(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_connector_register_all(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn drm_connector_create_standard_properties(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn drm_connector_free_work_fn(work: *mut work_struct);
}
// IOCTL
// drm_framebuffer.c
extern "C" {
    pub fn drm_framebuffer_free(kref: *mut kref);
}
extern "C" {
    pub fn drm_fb_release(file_priv: *mut drm_file);
}
// IOCTL
// drm_atomic.c

extern "C" {
    pub fn drm_atomic_debugfs_init(dev: *mut drm_device);
}

// drm_atomic_uapi.c
// IOCTL
// drm_plane.c
extern "C" {
    pub fn drm_plane_register_all(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn drm_plane_unregister_all(dev: *mut drm_device);
}
// drm_bridge.c
extern "C" {
    pub fn drm_bridge_detach(bridge: *mut drm_bridge);
}
// IOCTL
// drm_edid.c
extern "C" {
    pub fn drm_mode_fixup_1366x768(mode: *mut drm_display_mode);
}
extern "C" {
    pub fn drm_edid_override_show(connector: *mut drm_connector, m: *mut seq_file) -> c_int;
}
extern "C" {
    pub fn drm_edid_override_set(connector: *mut drm_connector, edid: *const c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn drm_edid_override_reset(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn drm_edid_cta_sad_get(cta_sad: *const cea_sad, sad: *mut u8);
}
extern "C" {
    pub fn drm_edid_cta_sad_set(cta_sad: *mut cea_sad, sad: *const u8);
}
// drm_edid_load.c

extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}

// drm_panic.c

extern "C" {
    pub fn drm_panic_is_enabled(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn drm_panic_register(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_panic_unregister(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_panic_init();
}
extern "C" {
    pub fn drm_panic_exit();
}

