//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_hdmi_cec_helper.h
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

// Macro flag: #define DRM_DISPLAY_HDMI_CEC_HELPER

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_connector_hdmi_cec_funcs {
//
// @init: perform hardware-specific initialization before registering the CEC adapter
//
    pub connector): *mut *mut int (init)(struct drm_connector,
//
// @uninit: perform hardware-specific teardown for the CEC adapter
//
    pub connector): *mut *mut void (uninit)(struct drm_connector,
//
// @enable: enable or disable CEC adapter
//
    pub enable): *mut *mut *mut int (enable)(struct drm_connector connector, bool,
//
// @log_addr: set adapter's logical address, can be called multiple
// times if adapter supports several LAs
//
    pub logical_addr): *mut *mut *mut int (log_addr)(struct drm_connector connector, u8,
//
// @transmit: start transmission of the specified CEC message
//
    pub msg): *mut u32 signal_free_time, struct cec_msg,
}

