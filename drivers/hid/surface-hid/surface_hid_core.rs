//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/surface-hid/surface_hid_core.h
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
// Common/core components for the Surface System Aggregator Module (SSAM) HID
// transport driver. Provides support for integrated HID devices on Microsoft
// Surface models.
//
// Copyright (C) 2019-2021 Maximilian Luz <luzmaximilian@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum surface_hid_descriptor_entry {
    SURFACE_HID_DESC_HID    = 0,
    SURFACE_HID_DESC_REPORT = 1,
    SURFACE_HID_DESC_ATTRS  = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface_hid_descriptor {
    pub /: *mut *mut __u8 desc_len; / = 9,
    pub /: *mut *mut __u8 desc_type; / = HID_DT_HID,
    pub hid_version: __le16,
    pub country_code: __u8,
    pub /: *mut *mut __u8 num_descriptors; / = 1,
    pub /: *mut *mut __u8 report_desc_type; / = HID_DT_REPORT,
    pub report_desc_len: __le16,
    pub __packed: },
    pub 9): static_assert(sizeof(struct surface_hid_descriptor) ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface_hid_attributes {
    pub length: __le32,
    pub vendor: __le16,
    pub product: __le16,
    pub version: __le16,
    pub _unknown: [__u8; 22],
    pub __packed: },
    pub 32): static_assert(sizeof(struct surface_hid_attributes) ==,
    pub surface_hid_device: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface_hid_device_ops {
    pub len): *mut *mut *mut *mut int (get_descriptor)(struct surface_hid_device shid, u8 entry, u8 buf, size_t,
    pub len): *mut *mut *mut *mut int (output_report)(struct surface_hid_device shid, u8 rprt_id, u8 buf, size_t,
    pub len): *mut *mut *mut *mut int (get_feature_report)(struct surface_hid_device shid, u8 rprt_id, u8 buf, size_t,
    pub len): *mut *mut *mut *mut int (set_feature_report)(struct surface_hid_device shid, u8 rprt_id, u8 buf, size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface_hid_device {
    pub dev: *mut device,
    pub ctrl: *mut ssam_controller,
    pub uid: ssam_device_uid,
    pub hid_desc: surface_hid_descriptor,
    pub attrs: surface_hid_attributes,
    pub notif: ssam_event_notifier,
    pub hid: *mut hid_device,
    pub ops: surface_hid_device_ops,
}

extern "C" {
    pub fn surface_hid_device_add(shid: *mut surface_hid_device) -> c_int;
}
extern "C" {
    pub fn surface_hid_device_destroy(shid: *mut surface_hid_device);
}
