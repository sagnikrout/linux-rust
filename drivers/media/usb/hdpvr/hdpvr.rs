//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/hdpvr/hdpvr.h
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
// Hauppauge HD PVR USB driver
//
// Copyright (C) 2008      Janne Grunau (j@jannau.net)
//

pub const HDPVR_MAX: c_int = 8;
pub const HDPVR_I2C_MAX_SIZE: c_int = 128;
// Define these values to match your devices
pub const HD_PVR_VENDOR_ID: c_uint = 0x2040;
pub const HD_PVR_PRODUCT_ID: c_uint = 0x4900;
pub const HD_PVR_PRODUCT_ID1: c_uint = 0x4901;
pub const HD_PVR_PRODUCT_ID2: c_uint = 0x4902;
pub const HD_PVR_PRODUCT_ID4: c_uint = 0x4903;
pub const HD_PVR_PRODUCT_ID3: c_uint = 0x4982;

pub const NUM_BUFFERS: c_int = 64;
pub const HDPVR_FIRMWARE_VERSION: c_uint = 0x08;
pub const HDPVR_FIRMWARE_VERSION_AC3: c_uint = 0x0d;
pub const HDPVR_FIRMWARE_VERSION_0X12: c_uint = 0x12;
pub const HDPVR_FIRMWARE_VERSION_0X15: c_uint = 0x15;
pub const HDPVR_FIRMWARE_VERSION_0X1E: c_uint = 0x1e;
// #define HDPVR_DEBUG
pub const MSG_INFO: c_int = 1;
pub const MSG_BUFFER: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdpvr_options {
    pub video_std: u8,
    pub video_input: u8,
    pub audio_input: u8,
    pub /: *mut *mut u8 bitrate; / in 100kbps,
    pub /: *mut *mut u8 peak_bitrate; / in 100kbps,
    pub bitrate_mode: u8,
    pub gop_mode: u8,
    pub audio_codec: v4l2_mpeg_audio_encoding,
    pub brightness: u8,
    pub contrast: u8,
    pub hue: u8,
    pub saturation: u8,
    pub sharpness: u8,
}

// Structure to hold all of our device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdpvr_device {
// the v4l device for this device
    pub video_dev: video_device,
// the control handler for this device
    pub hdl: v4l2_ctrl_handler,
// the usb device for this device
    pub udev: *mut usb_device,
// v4l2-device unused
    pub v4l2_dev: v4l2_device,
    pub video_mode: *mut v4l2_ctrl,
    pub video_bitrate: *mut v4l2_ctrl,
    pub video_bitrate_peak: *mut v4l2_ctrl,
}

// v4l2 format
// the max packet size of the bulk endpoint
// the address of the bulk in endpoint
// holds the current device status
// holds the current set options
// synchronize I/O
// available buffers
// in progress buffers
// waitqueue for buffers
// waitqueue for data
//
// current stream owner
// I2C adapter
// I2C lock
// I2C message buffer space
// For passing data to ir-kbd-i2c
// usb control transfer buffer and lock
extern "C" {
    pub fn container_of(_arg: v4l2_dev, hdpvr_device: struct, _arg: v4l2_dev) -> return;
}
// buffer one bulk urb of data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdpvr_buffer {
    pub buff_list: list_head,
    pub urb: *mut urb,
    pub dev: *mut hdpvr_device,
    pub pos: c_uint,
    pub status: __u8,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdpvr_video_info {
    pub width: u16,
    pub height: u16,
    pub fps: u8,
    pub valid: bool,
}

pub const CTRL_START_STREAMING_VALUE: c_uint = 0x0700;
pub const CTRL_STOP_STREAMING_VALUE: c_uint = 0x0800;
pub const CTRL_BITRATE_VALUE: c_uint = 0x1000;
pub const CTRL_BITRATE_MODE_VALUE: c_uint = 0x1200;
pub const CTRL_GOP_MODE_VALUE: c_uint = 0x1300;
pub const CTRL_VIDEO_INPUT_VALUE: c_uint = 0x1500;
pub const CTRL_VIDEO_STD_TYPE: c_uint = 0x1700;
pub const CTRL_AUDIO_INPUT_VALUE: c_uint = 0x2500;
pub const CTRL_BRIGHTNESS: c_uint = 0x2900;
pub const CTRL_CONTRAST: c_uint = 0x2a00;
pub const CTRL_HUE: c_uint = 0x2b00;
pub const CTRL_SATURATION: c_uint = 0x2c00;
pub const CTRL_SHARPNESS: c_uint = 0x2d00;
pub const CTRL_LOW_PASS_FILTER_VALUE: c_uint = 0x3100;
pub const CTRL_DEFAULT_INDEX: c_uint = 0x0003;
// :0 s 38 01 1000 0003 0004 4 = 0a00ca00
// BITRATE SETTING
// 1st and 2nd byte (little endian): average bitrate in 100 000 bit/s
// min: 1 mbit/s, max: 13.5 mbit/s
// 3rd and 4th byte (little endian): peak bitrate in 100 000 bit/s
// min: average + 100kbit/s,
// max: 20.2 mbit/s
//
// :0 s 38 01 1200 0003 0001 1 = 02
// BIT RATE MODE
// constant = 1, variable (peak) = 2, variable (average) = 3
//
// :0 s 38 01 1300 0003 0001 1 = 03
// GOP MODE (2 bit)
// low bit 0/1: advanced/simple GOP
// high bit 0/1: IDR(4/32/128) / no IDR (4/32/0)
//
// :0 s 38 01 1700 0003 0001 1 = 00
// VIDEO STANDARD or FREQUENCY 0 = 60hz, 1 = 50hz
//
// :0 s 38 01 3100 0003 0004 4 = 03030000
// FILTER CONTROL
// 1st byte luma low pass filter strength,
// 2nd byte chroma low pass filter strength,
// 3rd byte MF enable chroma, min=0, max=1
// 4th byte n
//
// :0 s 38 b9 0001 0000 0000 0
// :0 s 38 d3 0000 0000 0001 1 = 00
// ret = usb_control_msg(dev->udev,
// usb_sndctrlpipe(dev->udev, 0),
// 0xd3, 0x38,
// 0, 0,
// "\0", 1,
// 1000);
// info("control request returned %d", ret);
// msleep(5000);
// :0 s b8 81 1400 0003 0005 5 <
// :0 0 5 = d0024002 19
// QUERY FRAME SIZE AND RATE
// 1st and 2nd byte (little endian): horizontal resolution
// 3rd and 4th byte (little endian): vertical resolution
// 5th byte: frame rate
//
// :0 s b8 81 1800 0003 0003 3 <
// :0 0 3 = 030104
// QUERY SIGNAL AND DETECTED LINES, maybe INPUT
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdpvr_video_std {
    HDPVR_60HZ = 0,
    HDPVR_50HZ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdpvr_video_input {
    HDPVR_COMPONENT = 0,
    HDPVR_SVIDEO,
    HDPVR_COMPOSITE,
    HDPVR_VIDEO_INPUTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdpvr_audio_inputs {
    HDPVR_RCA_BACK = 0,
    HDPVR_RCA_FRONT,
    HDPVR_SPDIF,
    HDPVR_AUDIO_INPUTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdpvr_bitrate_mode {
    HDPVR_CONSTANT = 1,
    HDPVR_VARIABLE_PEAK,
    HDPVR_VARIABLE_AVERAGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdpvr_gop_mode {
    HDPVR_ADVANCED_IDR_GOP = 0,
    HDPVR_SIMPLE_IDR_GOP,
    HDPVR_ADVANCED_NOIDR_GOP,
    HDPVR_SIMPLE_NOIDR_GOP,
}

extern "C" {
    pub fn hdpvr_delete(dev: *mut hdpvr_device);
}
// ========================================================================
// hardware control functions
extern "C" {
    pub fn hdpvr_set_options(dev: *mut hdpvr_device) -> c_int;
}
extern "C" {
    pub fn hdpvr_set_bitrate(dev: *mut hdpvr_device) -> c_int;
}
extern "C" {
    pub fn get_video_info(dev: *mut hdpvr_device, vid_info: *mut hdpvr_video_info) -> c_int;
}
// :0 s b8 81 1800 0003 0003 3 <
// :0 0 3 = 0301ff
extern "C" {
    pub fn get_input_lines_info(dev: *mut hdpvr_device) -> c_int;
}
// ========================================================================
// v4l2 registration
extern "C" {
    pub fn hdpvr_cancel_queue(dev: *mut hdpvr_device) -> c_int;
}
// ========================================================================
// i2c adapter registration
extern "C" {
    pub fn hdpvr_register_i2c_adapter(dev: *mut hdpvr_device) -> c_int;
}
// ========================================================================
// buffer management
extern "C" {
    pub fn hdpvr_free_buffers(dev: *mut hdpvr_device) -> c_int;
}
extern "C" {
    pub fn hdpvr_alloc_buffers(dev: *mut hdpvr_device, count: c_uint) -> c_int;
}
