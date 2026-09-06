//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_of.h
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
// enum drm_lvds_dual_link_pixels - Pixel order of an LVDS dual-link connection
// @DRM_LVDS_DUAL_LINK_EVEN_ODD_PIXELS: Even pixels are expected to be generated
// from the first port, odd pixels from the second port
// @DRM_LVDS_DUAL_LINK_ODD_EVEN_PIXELS: Odd pixels are expected to be generated
// from the first port, even pixels from the second port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_lvds_dual_link_pixels {
    DRM_LVDS_DUAL_LINK_EVEN_ODD_PIXELS = 0,
    DRM_LVDS_DUAL_LINK_ODD_EVEN_PIXELS = 1,
}

extern "C" {
    pub fn drm_of_lvds_get_data_mapping(port: *const device_node) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

//
// drm_of_panel_bridge_remove - remove panel bridge
// @np: device tree node containing panel bridge output ports
//
// Remove the panel bridge of a given DT node's port and endpoint number
//
// Returns zero if successful, or one of the standard error codes if it fails.
//

