//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/uvc_configfs.h
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
// uvc_configfs.h
//
// Configfs support for the uvc function.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

pub const UVCG_STREAMING_CONTROL_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_control_header {
    pub item: config_item,
    pub desc: UVC_HEADER_DESCRIPTOR(1),
    pub linked: unsigned,
}

extern "C" {
    pub fn container_of(_arg: item, uvcg_control_header: struct, _arg: item) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_color_matching {
    pub group: config_group,
    pub desc: uvc_color_matching_descriptor,
    pub refcnt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uvcg_format_type {
    UVCG_UNCOMPRESSED = 0,
    UVCG_MJPEG,
    UVCG_FRAMEBASED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_format {
    pub group: config_group,
    pub type: uvcg_format_type,
    pub linked: unsigned,
    pub frames: list_head,
    pub num_frames: unsigned,
    pub bmaControls: [__u8; UVCG_STREAMING_CONTROL_SIZE],
    pub color_matching: *mut uvcg_color_matching,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_format_ptr {
    pub fmt: *mut uvcg_format,
    pub entry: list_head,
}

extern "C" {
    pub fn container_of(_arg: to_config_group(item), uvcg_format: struct, _arg: group) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_streaming_header {
    pub item: config_item,
    pub linked: unsigned,
    pub formats: list_head,
    pub num_fmt: unsigned,
// Must be last --ends in a flexible-array member.
    pub desc: uvc_input_header_descriptor,
}

extern "C" {
    pub fn container_of(_arg: item, uvcg_streaming_header: struct, _arg: item) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_frame_ptr {
    pub frm: *mut uvcg_frame,
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_frame {
    pub item: config_item,
    pub fmt_type: uvcg_format_type,
    pub b_length: u8,
    pub b_descriptor_type: u8,
    pub b_descriptor_subtype: u8,
    pub b_frame_index: u8,
    pub bm_capabilities: u8,
    pub w_width: u16,
    pub w_height: u16,
    pub dw_min_bit_rate: u32,
    pub dw_max_bit_rate: u32,
    pub dw_max_video_frame_buffer_size: u32,
    pub dw_default_frame_interval: u32,
    pub b_frame_interval_type: u8,
    pub dw_bytes_perline: u32,
// C attribute field omitted
    pub dw_frame_interval: *mut u32,
}

extern "C" {
    pub fn container_of(_arg: item, uvcg_frame: struct, _arg: item) -> return;
}
// -----------------------------------------------------------------------------
// streaming/uncompressed/<NAME>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_uncompressed {
    pub fmt: uvcg_format,
    pub desc: uvc_format_uncompressed,
}

extern "C" {
    pub fn container_of(_arg: to_uvcg_format(item), uvcg_uncompressed: struct, _arg: fmt) -> return;
}
// -----------------------------------------------------------------------------
// streaming/mjpeg/<NAME>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_mjpeg {
    pub fmt: uvcg_format,
    pub desc: uvc_format_mjpeg,
}

extern "C" {
    pub fn container_of(_arg: to_uvcg_format(item), uvcg_mjpeg: struct, _arg: fmt) -> return;
}
// -----------------------------------------------------------------------------
// streaming/framebased/<NAME>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_framebased {
    pub fmt: uvcg_format,
    pub desc: uvc_format_framebased,
}

extern "C" {
    pub fn container_of(_arg: to_uvcg_format(item), uvcg_framebased: struct, _arg: fmt) -> return;
}
// -----------------------------------------------------------------------------
// control/extensions/<NAME>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_extension_unit_descriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubType: u8,
    pub bUnitID: u8,
    pub guidExtensionCode: [u8; 16],
    pub bNumControls: u8,
    pub bNrInPins: u8,
    pub baSourceID: *mut u8,
    pub bControlSize: u8,
    pub bmControls: *mut u8,
    pub iExtension: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvcg_extension {
    pub item: config_item,
    pub list: list_head,
    pub string_descriptor_index: u8,
    pub desc: uvcg_extension_unit_descriptor,
}

extern "C" {
    pub fn container_of(_arg: item, uvcg_extension: struct, _arg: item) -> return;
}
extern "C" {
    pub fn uvcg_attach_configfs(opts: *mut f_uvc_opts) -> c_int;
}
