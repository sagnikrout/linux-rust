//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_probe_helper.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT

// connector, uint32_t maxX,
extern "C" {
    pub fn drmm_kms_helper_poll_init(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_kms_helper_poll_init(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_kms_helper_poll_fini(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_helper_hpd_irq_event(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn drm_connector_helper_hpd_irq_event(connector: *mut drm_connector) -> bool;
}
extern "C" {
    pub fn drm_kms_helper_hotplug_event(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_kms_helper_connector_hotplug_event(connector: *mut drm_connector);
}
extern "C" {
    pub fn drm_kms_helper_poll_disable(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_kms_helper_poll_enable(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_kms_helper_poll_reschedule(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_kms_helper_is_poll_worker() -> bool;
}
extern "C" {
    pub fn drm_connector_helper_get_modes(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn drm_connector_helper_tv_get_modes(connector: *mut drm_connector) -> c_int;
}
