//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/hid/progs/hid_bpf_helpers.h
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
// "undefine" structs and enums in vmlinux.h, because we "override" them below

// do not define kfunc through vmlinux.h as this messes up our custom hack
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_report_type {
    HID_INPUT_REPORT		= 0,
    HID_OUTPUT_REPORT		= 1,
    HID_FEATURE_REPORT		= 2,

    HID_REPORT_TYPES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_device {
    pub id: c_uint,
    pub name: [c_char; 128],
    pub phys: [c_char; 64],
    pub uniq: [c_char; 64],
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_wq {
    pub __opaque: [__u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_bpf_ctx {
    pub hid: *mut hid_device,
    pub allocated_size: __u32,
    pub retval: __s32,
    pub size: __s32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_class_request {
    HID_REQ_GET_REPORT		= 0x01,
    HID_REQ_GET_IDLE		= 0x02,
    HID_REQ_GET_PROTOCOL		= 0x03,
    HID_REQ_SET_REPORT		= 0x09,
    HID_REQ_SET_IDLE		= 0x0A,
    HID_REQ_SET_PROTOCOL		= 0x0B,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_bpf_ops {
    pub hid_id: c_int,
    pub flags: u32,
    pub list: list_head,
    pub source): u64,
    pub ctx): *mut *mut int (hid_rdesc_fixup)(struct hid_bpf_ctx,
    pub source): u64,
    pub source): *mut *mut *mut int (hid_hw_output_report)(struct hid_bpf_ctx ctx, u64,
    pub hdev: *mut hid_device,
}

// following are kfuncs exported by HID for HID-BPF
// bpf_wq implementation
