//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vkms/vkms_connector.h
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


// SPDX-License-Identifier: GPL-2.0+

//
// struct vkms_connector - VKMS custom type wrapping around the DRM connector
//
// @drm: Base DRM connector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_connector {
    pub base: drm_connector,
}

//
// vkms_connector_init() - Initialize a connector
// @vkmsdev: VKMS device containing the connector
//
// Returns:
// The connector or an error on failure.
//
// vkms_trigger_connector_hotplug() - Update the device's connectors status
// @vkmsdev: VKMS device to update
//
extern "C" {
    pub fn vkms_trigger_connector_hotplug(vkmsdev: *mut vkms_device);
}
