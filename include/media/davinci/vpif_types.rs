//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/davinci/vpif_types.h
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
//
// Copyright (C) 2011 Texas Instruments Inc
//

pub const VPIF_CAPTURE_MAX_CHANNELS: c_int = 2;
pub const VPIF_DISPLAY_MAX_CHANNELS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpif_if_type {
    VPIF_IF_BT656,
    VPIF_IF_BT1120,
    VPIF_IF_RAW_BAYER
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_interface {
    pub if_type: vpif_if_type,
    pub hd_pol:1: unsigned,
    pub vd_pol:1: unsigned,
    pub fid_pol:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_subdev_info {
    pub name: *const c_char,
    pub board_info: i2c_board_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_output {
    pub output: v4l2_output,
    pub subdev_name: *const c_char,
    pub input_route: u32,
    pub output_route: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_display_chan_config {
    pub outputs: *const vpif_output,
    pub output_count: c_int,
    pub clip_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_display_config {
    pub int): *mut *mut int (set_clock)(int,,
    pub subdevinfo: *mut vpif_subdev_info,
    pub subdev_count: c_int,
    pub i2c_adapter_id: c_int,
    pub chan_config: [vpif_display_chan_config; VPIF_DISPLAY_MAX_CHANNELS],
    pub card_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_input {
    pub input: v4l2_input,
    pub subdev_name: *mut c_char,
    pub input_route: u32,
    pub output_route: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_capture_chan_config {
    pub vpif_if: vpif_interface,
    pub inputs: *mut vpif_input,
    pub input_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_capture_config {
    pub (*setup_input_channel_mode)(int): *mut c_int,
    pub ): *const *const int (setup_input_path)(int, char,
    pub chan_config: [vpif_capture_chan_config; VPIF_CAPTURE_MAX_CHANNELS],
    pub subdev_info: *mut vpif_subdev_info,
    pub subdev_count: c_int,
    pub i2c_adapter_id: c_int,
    pub card_name: *const c_char,
    pub asd: [*mut v4l2_async_connection; VPIF_CAPTURE_MAX_CHANNELS],
    pub asd_sizes: [c_int; VPIF_CAPTURE_MAX_CHANNELS],
}
