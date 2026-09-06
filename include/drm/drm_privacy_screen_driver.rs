//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_privacy_screen_driver.h
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
// Copyright (C) 2020 Red Hat, Inc.
//
// Authors:
// Hans de Goede <hdegoede@redhat.com>
//

//
// struct drm_privacy_screen_ops - drm_privacy_screen operations
//
// Defines the operations which the privacy-screen class code may call.
// These functions should be implemented by the privacy-screen driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_privacy_screen_ops {
//
// @set_sw_state: Called to request a change of the privacy-screen
// state. The privacy-screen class code contains a check to avoid this
// getting called when the hw_state reports the state is locked.
// It is the driver's responsibility to update sw_state and hw_state.
// This is always called with the drm_privacy_screen's lock held.
//
    pub sw_state): drm_privacy_screen_status,
//
// @get_hw_state: Called to request that the driver gets the current
// privacy-screen state from the hardware and then updates sw_state and
// hw_state accordingly. This will be called by the core just before
// the privacy-screen is registered in sysfs.
//
    pub priv): *mut *mut void (get_hw_state)(struct drm_privacy_screen,
}

//
// struct drm_privacy_screen - central privacy-screen structure
//
// Central privacy-screen structure, this contains the struct device used
// to register the screen in sysfs, the screen's state, ops, etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_privacy_screen {
// @dev: device used to register the privacy-screen in sysfs.
    pub dev: device,
// @lock: mutex protection all fields in this struct.
    pub lock: mutex,
// @list: privacy-screen devices list list-entry.
    pub list: list_head,
// @notifier_head: privacy-screen notifier head.
    pub notifier_head: blocking_notifier_head,
//
// @ops: &struct drm_privacy_screen_ops for this privacy-screen.
// This is NULL if the driver has unregistered the privacy-screen.
//
    pub ops: *const drm_privacy_screen_ops,
//
// @sw_state: The privacy-screen's software state, see
// :ref:`Standard Connector Properties<standard_connector_properties>`
// for more info.
//
    pub sw_state: drm_privacy_screen_status,
//
// @hw_state: The privacy-screen's hardware state, see
// :ref:`Standard Connector Properties<standard_connector_properties>`
// for more info.
//
    pub hw_state: drm_privacy_screen_status,
//
// @drvdata: Private data owned by the privacy screen provider
//
    pub drvdata: *mut c_void,
}

extern "C" {
    pub fn drm_privacy_screen_unregister(priv: *mut drm_privacy_screen);
}
extern "C" {
    pub fn drm_privacy_screen_call_notifier_chain(priv: *mut drm_privacy_screen);
}
