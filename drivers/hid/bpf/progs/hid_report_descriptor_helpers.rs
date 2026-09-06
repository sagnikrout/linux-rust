//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/bpf/progs/hid_report_descriptor_helpers.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2022 Benjamin Tissoires
//

// Compiler attributes

// Report Descriptor Structures
pub const HID_MAX_COLLECTIONS: c_int = 32;
pub const HID_MAX_FIELDS: c_int = 64;
pub const HID_MAX_REPORTS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_rdesc_field_type {
    HID_FIELD_VARIABLE = 0,
    HID_FIELD_ARRAY = 1,
    HID_FIELD_CONSTANT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_rdesc_collection {
    pub usage_page: __u16,
    pub usage_id: __u16,
    pub collection_type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_rdesc_field {
    pub /: *mut *mut __u8 field_type; / enum hid_rdesc_field_type,
    pub num_collections: __u8,
    pub bits_start: __u16,
    pub bits_end: __u16,
    pub usage_page: __u16,
    pub /: *mut *mut __u16 usage_id; / For Variable fields,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub usage_minimum: __u16,
    pub usage_maximum: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_rdesc_report {
    pub /: *mut *mut __u8 report_id; / 0 means no report ID,
    pub size_in_bits: __u16,
    pub num_fields: __u8,
    pub fields: [hid_rdesc_field; HID_MAX_FIELDS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_rdesc_descriptor {
    pub num_input_reports: __u8,
    pub num_output_reports: __u8,
    pub num_feature_reports: __u8,
    pub input_reports: [hid_rdesc_report; HID_MAX_REPORTS],
    pub output_reports: [hid_rdesc_report; HID_MAX_REPORTS],
    pub feature_reports: [hid_rdesc_report; HID_MAX_REPORTS],
    pub __packed: },
