//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pwc/pwc.h
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
// (C) 1999-2003 Nemosoft Unv.
//

// Version block

// Trace certain actions in the driver

pub const pwc_trace: c_int = 0;

// Defines for ToUCam cameras
pub const TOUCAM_HEADER_SIZE: c_int = 8;
pub const TOUCAM_TRAILER_SIZE: c_int = 4;
pub const FEATURE_MOTOR_PANTILT: c_uint = 0x0001;
pub const FEATURE_CODEC1: c_uint = 0x0002;
pub const FEATURE_CODEC2: c_uint = 0x0004;
pub const MAX_WIDTH: c_int = 640;
pub const MAX_HEIGHT: c_int = 480;
// Ignore errors in the first N frames, to allow for startup delays
pub const FRAME_LOWMARK: c_int = 5;
// Size and number of buffers for the ISO pipe.
pub const MAX_ISO_BUFS: c_int = 3;
pub const ISO_FRAMES_PER_DESC: c_int = 10;
pub const ISO_MAX_FRAME_SIZE: c_int = 960;

// Maximum size after decompression is 640x480 YUV data, 1.5 * 640 * 480

// Absolute minimum and maximum number of buffers available for mmap()
pub const MIN_FRAMES: c_int = 2;
pub const MAX_FRAMES: c_int = 16;
// Some macros to quickly find the type of a webcam

// Request types: video
pub const SET_LUM_CTL: c_uint = 0x01;
pub const GET_LUM_CTL: c_uint = 0x02;
pub const SET_CHROM_CTL: c_uint = 0x03;
pub const GET_CHROM_CTL: c_uint = 0x04;
pub const SET_STATUS_CTL: c_uint = 0x05;
pub const GET_STATUS_CTL: c_uint = 0x06;
pub const SET_EP_STREAM_CTL: c_uint = 0x07;
pub const GET_EP_STREAM_CTL: c_uint = 0x08;
pub const GET_XX_CTL: c_uint = 0x09;
pub const SET_XX_CTL: c_uint = 0x0A;
pub const GET_XY_CTL: c_uint = 0x0B;
pub const SET_XY_CTL: c_uint = 0x0C;
pub const SET_MPT_CTL: c_uint = 0x0D;
pub const GET_MPT_CTL: c_uint = 0x0E;
// Selectors for the Luminance controls [GS]ET_LUM_CTL
pub const AGC_MODE_FORMATTER: c_uint = 0x2000;
pub const PRESET_AGC_FORMATTER: c_uint = 0x2100;
pub const SHUTTER_MODE_FORMATTER: c_uint = 0x2200;
pub const PRESET_SHUTTER_FORMATTER: c_uint = 0x2300;
pub const PRESET_CONTOUR_FORMATTER: c_uint = 0x2400;
pub const AUTO_CONTOUR_FORMATTER: c_uint = 0x2500;
pub const BACK_LIGHT_COMPENSATION_FORMATTER: c_uint = 0x2600;
pub const CONTRAST_FORMATTER: c_uint = 0x2700;
pub const DYNAMIC_NOISE_CONTROL_FORMATTER: c_uint = 0x2800;
pub const FLICKERLESS_MODE_FORMATTER: c_uint = 0x2900;
pub const AE_CONTROL_SPEED: c_uint = 0x2A00;
pub const BRIGHTNESS_FORMATTER: c_uint = 0x2B00;
pub const GAMMA_FORMATTER: c_uint = 0x2C00;
// Selectors for the Chrominance controls [GS]ET_CHROM_CTL
pub const WB_MODE_FORMATTER: c_uint = 0x1000;
pub const AWB_CONTROL_SPEED_FORMATTER: c_uint = 0x1100;
pub const AWB_CONTROL_DELAY_FORMATTER: c_uint = 0x1200;
pub const PRESET_MANUAL_RED_GAIN_FORMATTER: c_uint = 0x1300;
pub const PRESET_MANUAL_BLUE_GAIN_FORMATTER: c_uint = 0x1400;
pub const COLOUR_MODE_FORMATTER: c_uint = 0x1500;
pub const SATURATION_MODE_FORMATTER1: c_uint = 0x1600;
pub const SATURATION_MODE_FORMATTER2: c_uint = 0x1700;
// Selectors for the Status controls [GS]ET_STATUS_CTL
pub const SAVE_USER_DEFAULTS_FORMATTER: c_uint = 0x0200;
pub const RESTORE_USER_DEFAULTS_FORMATTER: c_uint = 0x0300;
pub const RESTORE_FACTORY_DEFAULTS_FORMATTER: c_uint = 0x0400;
pub const READ_AGC_FORMATTER: c_uint = 0x0500;
pub const READ_SHUTTER_FORMATTER: c_uint = 0x0600;
pub const READ_RED_GAIN_FORMATTER: c_uint = 0x0700;
pub const READ_BLUE_GAIN_FORMATTER: c_uint = 0x0800;
// Formatters for the motorized pan & tilt [GS]ET_MPT_CTL
pub const PT_RELATIVE_CONTROL_FORMATTER: c_uint = 0x01;
pub const PT_RESET_CONTROL_FORMATTER: c_uint = 0x02;
pub const PT_STATUS_FORMATTER: c_uint = 0x03;
// Enumeration of image sizes
pub const PSZ_SQCIF: c_uint = 0x00;
pub const PSZ_QSIF: c_uint = 0x01;
pub const PSZ_QCIF: c_uint = 0x02;
pub const PSZ_SIF: c_uint = 0x03;
pub const PSZ_CIF: c_uint = 0x04;
pub const PSZ_VGA: c_uint = 0x05;
pub const PSZ_MAX: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwc_raw_frame {
    pub /: *mut *mut __le16 type; / type of the webcam,
    pub the: *mut *mut __le16 vbandlength; / Size of 4 lines compressed (used by,
    pub of: *mut *mut __u8 cmd[4]; / the four byte of the command (in case,
    pub /: *mut *mut *mut __u8 rawframe[]; / frame_size = H / 4  vbandlength,
    pub __packed: },
// intermediate buffers with raw data from the USB cam
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub data: *mut c_void,
    pub /: *mut *mut int filled; / number of bytes filled,
}

// videobuf2 queue and queued buffers list
// If taking both locks vb_queue_lock must always be locked first!
// Pointer to our usb_device, will be NULL after unplug
// type of cam (645, 646, 675, 680, 690, 720, 730, 740, 750)
// Video data
//
// Frame currently being filled, this only gets touched by the
// isoc urb complete handler, and by stream start / stop since
// start / stop touch it before / after starting / killing the urbs
// no locking is needed around this
//
// We have an 'image' and a 'view', where 'image' is the fixed-size img
// as delivered by the camera, and 'view' is the size requested by the
// program. The camera image is centered in this viewport, laced with
// a gray or black border. view_min <= image <= view <= view_max;
//

// controls
// awb / red-blue balance cluster
// usb ctrl transfers are slow, so we cache things
// autogain / gain cluster
// exposure_auto / exposure cluster
// autocontour/contour cluster
// motor control cluster
// CODEC3 models have both gain and exposure controlled by autogain
// Global variables

// Functions in pwc-misc.c
// sizes in pixels
extern "C" {
    pub fn pwc_get_size(pdev: *mut pwc_device, width: c_int, height: c_int) -> c_int;
}
extern "C" {
    pub fn pwc_construct(pdev: *mut pwc_device);
}
// Functions in pwc-ctrl.c
// Request a certain video mode. Returns < 0 if not possible
extern "C" {
    pub fn pwc_get_fps(pdev: *mut pwc_device, index: c_uint, size: c_uint) -> c_uint;
}
extern "C" {
    pub fn pwc_set_leds(pdev: *mut pwc_device, on_value: c_int, off_value: c_int) -> c_int;
}
extern "C" {
    pub fn pwc_get_cmos_sensor(pdev: *mut pwc_device, sensor: *mut c_int) -> c_int;
}
// Control get / set helpers
extern "C" {
    pub fn pwc_get_u8_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pwc_set_u8_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: u8) -> c_int;
}
extern "C" {
    pub fn pwc_get_s8_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: *mut c_int) -> c_int;
}

extern "C" {
    pub fn pwc_get_u16_ctrl(pdev: *mut pwc_device, request: u8, value: u16, dat: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pwc_set_u16_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: u16) -> c_int;
}
extern "C" {
    pub fn pwc_button_ctrl(pdev: *mut pwc_device, value: u16) -> c_int;
}
extern "C" {
    pub fn pwc_init_controls(pdev: *mut pwc_device) -> c_int;
}
// Power down or up the camera; not supported by all models
extern "C" {
    pub fn pwc_camera_power(pdev: *mut pwc_device, power: c_int);
}
// pwc-uncompress.c
// Expand frame to image, possibly including decompression. Uses read_frame and fill_image
extern "C" {
    pub fn pwc_decompress(pdev: *mut pwc_device, fbuf: *mut pwc_frame_buf) -> c_int;
}
