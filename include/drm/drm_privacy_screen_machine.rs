//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_privacy_screen_machine.h
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
// struct drm_privacy_screen_lookup -  static privacy-screen lookup list entry
//
// Used for the static lookup-list for mapping privacy-screen consumer
// dev-connector pairs to a privacy-screen provider.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_privacy_screen_lookup {
// @list: Lookup list list-entry.
    pub list: list_head,
// @dev_id: Consumer device name or NULL to match all devices.
    pub dev_id: *const c_char,
// @con_id: Consumer connector name or NULL to match all connectors.
    pub con_id: *const c_char,
// @provider: dev_name() of the privacy_screen provider.
    pub provider: *const c_char,
}

extern "C" {
    pub fn drm_privacy_screen_lookup_add(lookup: *mut drm_privacy_screen_lookup);
}
extern "C" {
    pub fn drm_privacy_screen_lookup_remove(lookup: *mut drm_privacy_screen_lookup);
}

extern "C" {
    pub fn drm_privacy_screen_lookup_init();
}
extern "C" {
    pub fn drm_privacy_screen_lookup_exit();
}

