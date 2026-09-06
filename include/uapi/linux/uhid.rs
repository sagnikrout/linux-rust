//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/uhid.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// User-space I/O driver support for HID subsystem
// Copyright (c) 2012 David Herrmann
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// Public header for user-space communication. We try to keep every structure
// aligned but to be safe we also use __attribute__((__packed__)). Therefore,
// the communication should be ABI compatible even between architectures.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uhid_event_type {
    __UHID_LEGACY_CREATE,
    UHID_DESTROY,
    UHID_START,
    UHID_STOP,
    UHID_OPEN,
    UHID_CLOSE,
    UHID_OUTPUT,
    __UHID_LEGACY_OUTPUT_EV,
    __UHID_LEGACY_INPUT,
    UHID_GET_REPORT,
    UHID_GET_REPORT_REPLY,
    UHID_CREATE2,
    UHID_INPUT2,
    UHID_SET_REPORT,
    UHID_SET_REPORT_REPLY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_create2_req {
    pub name: [__u8; 128],
    pub phys: [__u8; 64],
    pub uniq: [__u8; 64],
    pub rd_size: __u16,
    pub bus: __u16,
    pub vendor: __u32,
    pub product: __u32,
    pub version: __u32,
    pub country: __u32,
    pub rd_data: [__u8; HID_MAX_DESCRIPTOR_SIZE],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uhid_dev_flag {
    UHID_DEV_NUMBERED_FEATURE_REPORTS			= (1ULL << 0),
    UHID_DEV_NUMBERED_OUTPUT_REPORTS			= (1ULL << 1),
    UHID_DEV_NUMBERED_INPUT_REPORTS				= (1ULL << 2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_start_req {
    pub dev_flags: __u64,
}

pub const UHID_DATA_MAX: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uhid_report_type {
    UHID_FEATURE_REPORT,
    UHID_OUTPUT_REPORT,
    UHID_INPUT_REPORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_input2_req {
    pub size: __u16,
    pub data: [__u8; UHID_DATA_MAX],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_output_req {
    pub data: [__u8; UHID_DATA_MAX],
    pub size: __u16,
    pub rtype: __u8,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_get_report_req {
    pub id: __u32,
    pub rnum: __u8,
    pub rtype: __u8,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_get_report_reply_req {
    pub id: __u32,
    pub err: __u16,
    pub size: __u16,
    pub data: [__u8; UHID_DATA_MAX],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_set_report_req {
    pub id: __u32,
    pub rnum: __u8,
    pub rtype: __u8,
    pub size: __u16,
    pub data: [__u8; UHID_DATA_MAX],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_set_report_reply_req {
    pub id: __u32,
    pub err: __u16,
    pub __attribute__((__packed__)): },
//
// Compat Layer
// All these commands and requests are obsolete. You should avoid using them in
// new code. We support them for backwards-compatibility, but you might not get
// access to new feature in case you use them.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uhid_legacy_event_type {
    UHID_CREATE			= __UHID_LEGACY_CREATE,
    UHID_OUTPUT_EV			= __UHID_LEGACY_OUTPUT_EV,
    UHID_INPUT			= __UHID_LEGACY_INPUT,
    UHID_FEATURE			= UHID_GET_REPORT,
    UHID_FEATURE_ANSWER		= UHID_GET_REPORT_REPLY,
}

// Obsolete! Use UHID_CREATE2.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_create_req {
    pub name: [__u8; 128],
    pub phys: [__u8; 64],
    pub uniq: [__u8; 64],
    pub rd_data: *mut __u8 __user,
    pub rd_size: __u16,
    pub bus: __u16,
    pub vendor: __u32,
    pub product: __u32,
    pub version: __u32,
    pub country: __u32,
    pub __attribute__((__packed__)): },
// Obsolete! Use UHID_INPUT2.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_input_req {
    pub data: [__u8; UHID_DATA_MAX],
    pub size: __u16,
    pub __attribute__((__packed__)): },
// Obsolete! Kernel uses UHID_OUTPUT exclusively now.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_output_ev_req {
    pub type: __u16,
    pub code: __u16,
    pub value: __s32,
    pub __attribute__((__packed__)): },
// Obsolete! Kernel uses ABI compatible UHID_GET_REPORT.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_feature_req {
    pub id: __u32,
    pub rnum: __u8,
    pub rtype: __u8,
    pub __attribute__((__packed__)): },
// Obsolete! Use ABI compatible UHID_GET_REPORT_REPLY.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_feature_answer_req {
    pub id: __u32,
    pub err: __u16,
    pub size: __u16,
    pub data: [__u8; UHID_DATA_MAX],
    pub __attribute__((__packed__)): },
//
// UHID Events
// All UHID events from and to the kernel are encoded as "struct uhid_event".
// The "type" field contains a UHID_* type identifier. All payload depends on
// that type and can be accessed via ev->u.XYZ accordingly.
// If user-space writes short events, they're extended with 0s by the kernel. If
// the kernel writes short events, user-space shall extend them with 0s.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_event {
    pub type: __u32,
    pub create: uhid_create_req,
    pub input: uhid_input_req,
    pub output: uhid_output_req,
    pub output_ev: uhid_output_ev_req,
    pub feature: uhid_feature_req,
    pub get_report: uhid_get_report_req,
    pub feature_answer: uhid_feature_answer_req,
    pub get_report_reply: uhid_get_report_reply_req,
    pub create2: uhid_create2_req,
    pub input2: uhid_input2_req,
    pub set_report: uhid_set_report_req,
    pub set_report_reply: uhid_set_report_reply_req,
    pub start: uhid_start_req,
    pub u: },
    pub __attribute__((__packed__)): },
