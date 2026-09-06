//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/side-codecs/hda_component.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// HD audio Component Binding Interface
//
// Copyright (C) 2021 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const HDA_MAX_COMPONENTS: c_int = 4;
pub const HDA_MAX_NAME_SIZE: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_component {
    pub dev: *mut device,
    pub name: [c_char; HDA_MAX_NAME_SIZE],
    pub adev: *mut acpi_device,
    pub acpi_notifications_supported: bool,
    pub dev): *mut *mut void (acpi_notify)(acpi_handle handle, u32 event, struct device,
    pub action): *mut *mut *mut void (pre_playback_hook)(struct device dev, int,
    pub action): *mut *mut *mut void (playback_hook)(struct device dev, int,
    pub action): *mut *mut *mut void (post_playback_hook)(struct device dev, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_component_parent {
    pub mutex: mutex,
    pub codec: *mut hda_codec,
    pub comps: [hda_component; HDA_MAX_COMPONENTS],
}

extern "C" {
    pub fn hda_component_manager_playback_hook(parent: *mut hda_component_parent, action: c_int);
}
extern "C" {
    pub fn hda_component_manager_bind(cdc: *mut hda_codec, parent: *mut hda_component_parent) -> c_int;
}
