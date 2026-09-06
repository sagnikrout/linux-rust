//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_uvc.h
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
// u_uvc.h
//
// Utility definitions for the uvc function
//
// Copyright (c) 2013-2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_uvc_opts {
    pub func_inst: usb_function_instance,
    pub streaming_interval: c_uint,
    pub streaming_maxpacket: c_uint,
    pub streaming_maxburst: c_uint,
    pub control_interface: c_uint,
    pub streaming_interface: c_uint,
    pub function_name: [c_char; 32],
    pub last_unit_id: c_uint,
    pub enable_interrupt_ep: bool,
//
// Control descriptors array pointers for full-/high-speed and
// super-speed. They point by default to the uvc_fs_control_cls and
// uvc_ss_control_cls arrays respectively. Legacy gadgets must
// override them in their gadget bind callback.
//
    pub fs_control: *const *const uvc_descriptor_header,
    pub ss_control: *const *const uvc_descriptor_header,
//
// Streaming descriptors array pointers for full-speed, high-speed and
// super-speed. They will point to the uvc_[fhs]s_streaming_cls arrays
// for configfs-based gadgets. Legacy gadgets must initialize them in
// their gadget bind callback.
//
    pub fs_streaming: *const *const uvc_descriptor_header,
    pub hs_streaming: *const *const uvc_descriptor_header,
    pub ss_streaming: *const *const uvc_descriptor_header,
// Default control descriptors for configfs-based gadgets.
    pub uvc_camera_terminal: uvc_camera_terminal_descriptor,
    pub uvc_processing: uvc_processing_unit_descriptor,
    pub uvc_output_terminal: uvc_output_terminal_descriptor,
//
// Control descriptors pointers arrays for full-/high-speed and
// super-speed. The first element is a configurable control header
// descriptor, the other elements point to the fixed default control
// descriptors. Used by configfs only, must not be touched by legacy
// gadgets.
//
    pub uvc_fs_control_cls: [*mut uvc_descriptor_header; 5],
    pub uvc_ss_control_cls: [*mut uvc_descriptor_header; 5],
//
// Control descriptors for extension units. There could be any number
// of these, including none at all.
//
    pub extension_units: list_head,
//
// Streaming descriptors for full-speed, high-speed and super-speed.
// Used by configfs only, must not be touched by legacy gadgets. The
// arrays are allocated at runtime as the number of descriptors isn't
// known in advance.
//
    pub uvc_fs_streaming_cls: *mut uvc_descriptor_header,
    pub uvc_hs_streaming_cls: *mut uvc_descriptor_header,
    pub uvc_ss_streaming_cls: *mut uvc_descriptor_header,
//
// Indexes into the function's string descriptors allowing users to set
// custom descriptions rather than the hard-coded defaults.
//
    pub iad_index: u8,
    pub vs0_index: u8,
    pub vs1_index: u8,
//
// Read/write access to configfs attributes is handled by configfs.
//
// This lock protects the descriptors from concurrent access by
// read/write and symlink creation/removal.
//
    pub lock: mutex,
    pub refcnt: c_int,
//
// Only for legacy gadget. Shall be NULL for configfs-composed gadgets,
// which is guaranteed by alloc_inst implementation of f_uvc doing kzalloc.
//
    pub header: *mut uvcg_streaming_header,
}
