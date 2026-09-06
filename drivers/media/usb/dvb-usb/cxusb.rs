//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/cxusb.h
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

pub const CXUSB_VIDEO_PKT_SIZE: c_int = 3030;
pub const CXUSB_VIDEO_MAX_FRAME_PKTS: c_int = 346;

// usb commands - some of it are guesses, don't have a reference yet
pub const CMD_BLUEBIRD_GPIO_RW: c_uint = 0x05;
pub const CMD_I2C_WRITE: c_uint = 0x08;
pub const CMD_I2C_READ: c_uint = 0x09;
pub const CMD_GPIO_READ: c_uint = 0x0d;
pub const CMD_GPIO_WRITE: c_uint = 0x0e;
pub const GPIO_TUNER: c_uint = 0x02;
pub const CMD_POWER_OFF: c_uint = 0xdc;
pub const CMD_POWER_ON: c_uint = 0xde;
pub const CMD_STREAMING_ON: c_uint = 0x36;
pub const CMD_STREAMING_OFF: c_uint = 0x37;
pub const CMD_AVER_STREAM_ON: c_uint = 0x18;
pub const CMD_AVER_STREAM_OFF: c_uint = 0x19;
pub const CMD_GET_IR_CODE: c_uint = 0x47;
pub const CMD_ANALOG: c_uint = 0x50;
pub const CMD_DIGITAL: c_uint = 0x51;

pub const CXUSB_BT656_FIELD_1: c_int = 0;

pub const CXUSB_BT656_VBI_OFF: c_int = 0;

pub const CXUSB_BT656_SEAV_SAV: c_int = 0;
// Max transfer size done by I2C transfer functions
pub const MAX_XFER_SIZE: c_int = 80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxusb_state {
    pub gpio_write_state: [u8; 3],
    pub gpio_write_refresh: [bool; 3],
    pub i2c_client_demod: *mut i2c_client,
    pub i2c_client_tuner: *mut i2c_client,
    pub data: [c_uchar; MAX_XFER_SIZE],
    pub stream_mutex: mutex,
    pub last_lock: u8,
    pub status): *mut fe_status,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxusb_open_type {
    CXUSB_OPEN_INIT,
    CXUSB_OPEN_NONE,
    CXUSB_OPEN_ANALOG,
    CXUSB_OPEN_DIGITAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxusb_medion_auxbuf {
    pub buf: *mut u8,
    pub len: c_uint,
    pub paylen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxusb_bt656_mode {
    NEW_FRAME, FIRST_FIELD, SECOND_FIELD
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxusb_bt656_fmode {
    START_SEARCH, LINE_SAMPLES, VBI_SAMPLES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxusb_bt656_params {
    pub mode: cxusb_bt656_mode,
    pub fmode: cxusb_bt656_fmode,
    pub pos: c_uint,
    pub line: c_uint,
    pub linesamples: c_uint,
    pub buf: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxusb_medion_dev {
// has to be the first one
    pub state: cxusb_state,
    pub dvbdev: *mut dvb_usb_device,
    pub open_type: cxusb_open_type,
    pub open_ctr: c_uint,
    pub open_lock: mutex,

    pub v4l2dev: v4l2_device,
    pub cx25840: *mut v4l2_subdev,
    pub tuner: *mut v4l2_subdev,
    pub tda9887: *mut v4l2_subdev,
    pub radiodev: *mut *mut video_device videodev,,
    pub dev_lock: mutex,
    pub videoqueue: vb2_queue,
    pub input: u32,
    pub stop_streaming: bool,
    pub height: u32 width,,
    pub field_order: u32,
    pub auxbuf: cxusb_medion_auxbuf,
    pub norm: v4l2_std_id,
    pub streamurbs: [*mut urb; CXUSB_VIDEO_URBS],
    pub urbcomplete: c_ulong,
    pub urbwork: work_struct,
    pub nexturb: c_uint,
    pub bt656: cxusb_bt656_params,
    pub vbuf: *mut cxusb_medion_vbuffer,
    pub vbuf_sequence: __u32,
    pub buflist: list_head,
    pub v4l2_release: completion,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxusb_medion_vbuffer {
    pub vb2: vb2_v4l2_buffer,
    pub list: list_head,
}

// defines for "debug" module parameter

extern "C" {
    pub fn cxusb_medion_analog_init(dvbdev: *mut dvb_usb_device) -> c_int;
}
extern "C" {
    pub fn cxusb_medion_register_analog(dvbdev: *mut dvb_usb_device) -> c_int;
}
extern "C" {
    pub fn cxusb_medion_unregister_analog(dvbdev: *mut dvb_usb_device);
}

extern "C" {
    pub fn cxusb_medion_put(dvbdev: *mut dvb_usb_device);
}
