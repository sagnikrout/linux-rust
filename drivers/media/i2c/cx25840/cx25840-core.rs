//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/cx25840/cx25840-core.h
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
// cx25840 internal API header
//
// Copyright (C) 2003-2004 Chris Kennedy
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx25840_model {
    CX23885_AV,
    CX23887_AV,
    CX23888_AV,
    CX2310X_AV,
    CX25840,
    CX25841,
    CX25842,
    CX25843,
    CX25836,
    CX25837,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx25840_media_pads {
    CX25840_PAD_INPUT,
    CX25840_PAD_VID_OUT,

    CX25840_NUM_PADS
}

//
// struct cx25840_state - a device instance private data
// @c:			i2c_client struct representing this device
// @sd:		our V4L2 sub-device
// @hdl:		our V4L2 control handler
// @volume:		audio volume V4L2 control (non-cx2583x devices only)
// @mute:		audio mute V4L2 control (non-cx2583x devices only)
// @pvr150_workaround:	whether we enable workaround for Hauppauge PVR150
// hardware bug (audio dropping out)
// @generic_mode:	whether we disable ivtv-specific hacks
// this mode gets turned on when the bridge driver calls
// cx25840 subdevice init core op
// @radio:		set if we are currently in the radio mode, otherwise
// the current mode is non-radio (that is, video)
// @std:		currently set video standard
// @vid_input:		currently set video input
// @vid_config:	currently set video output configuration
// only used in the generic mode
// @aud_input:		currently set audio input
// @audclk_freq:	currently set audio sample rate
// @audmode:		currently set audio mode (when in non-radio mode)
// @vbi_line_offset:	vbi line number offset
// @id:		exact device model
// @rev:		raw device id read from the chip
// @is_initialized:	whether we have already loaded firmware into the chip
// and initialized it
// @vbi_regs_offset:	offset of vbi regs
// @fw_wait:		wait queue to wake an initialization function up when
// firmware loading (on a separate workqueue) finishes
// @fw_work:		a work that actually loads the firmware on a separate
// workqueue
// @ir_state:		a pointer to chip IR controller private data
// @pads:		array of supported chip pads (currently only a stub)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25840_state {
    pub c: *mut i2c_client,
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
// volume cluster
    pub volume: *mut v4l2_ctrl,
    pub mute: *mut v4l2_ctrl,
}

extern "C" {
    pub fn container_of(_arg: sd, cx25840_state: struct, _arg: sd) -> return;
}
// -----------------------------------------------------------------------
// cx25850-core.c
extern "C" {
    pub fn cx25840_write(client: *mut i2c_client, addr: u16, value: u8) -> c_int;
}
extern "C" {
    pub fn cx25840_write4(client: *mut i2c_client, addr: u16, value: u32) -> c_int;
}
extern "C" {
    pub fn cx25840_read(client: *mut i2c_client, addr: u16) -> u8;
}
extern "C" {
    pub fn cx25840_read4(client: *mut i2c_client, addr: u16) -> u32;
}
extern "C" {
    pub fn cx25840_std_setup(client: *mut i2c_client);
}
// -----------------------------------------------------------------------
// cx25850-firmware.c
extern "C" {
    pub fn cx25840_loadfw(client: *mut i2c_client) -> c_int;
}
// -----------------------------------------------------------------------
// cx25850-audio.c
extern "C" {
    pub fn cx25840_audio_set_path(client: *mut i2c_client);
}
extern "C" {
    pub fn cx25840_s_clock_freq(sd: *mut v4l2_subdev, freq: u32) -> c_int;
}
// -----------------------------------------------------------------------
// cx25850-vbi.c
extern "C" {
    pub fn cx25840_s_raw_fmt(sd: *mut v4l2_subdev, fmt: *mut v4l2_vbi_format) -> c_int;
}
// -----------------------------------------------------------------------
// cx25850-ir.c
extern "C" {
    pub fn cx25840_ir_log_status(sd: *mut v4l2_subdev) -> c_int;
}
extern "C" {
    pub fn cx25840_ir_irq_handler(sd: *mut v4l2_subdev, status: u32, handled: *mut bool) -> c_int;
}
extern "C" {
    pub fn cx25840_ir_probe(sd: *mut v4l2_subdev) -> c_int;
}
extern "C" {
    pub fn cx25840_ir_remove(sd: *mut v4l2_subdev) -> c_int;
}
