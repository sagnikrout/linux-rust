//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lis3lv02d.h
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
// struct lis3lv02d_platform_data - lis3 chip family platform data
// @click_flags:	Click detection unit configuration
// @click_thresh_x:	Click detection unit x axis threshold
// @click_thresh_y:	Click detection unit y axis threshold
// @click_thresh_z:	Click detection unit z axis threshold
// @click_time_limit:	Click detection unit time parameter
// @click_latency:	Click detection unit latency parameter
// @click_window:	Click detection unit window parameter
// @irq_cfg:		On chip irq source and type configuration (click
// data available / wake up, open drain, polarity)
// @irq_flags1:		Additional irq triggering flags for irq channel 0
// @irq_flags2:		Additional irq triggering flags for irq channel 1
// @duration1:		Wake up unit 1 duration parameter
// @duration2:		Wake up unit 2 duration parameter
// @wakeup_flags:	Wake up unit 1 flags
// @wakeup_thresh:	Wake up unit 1 threshold value
// @wakeup_flags2:	Wake up unit 2 flags
// @wakeup_thresh2:	Wake up unit 2 threshold value
// @hipass_ctrl:	High pass filter control (enable / disable, cut off
// frequency)
// @axis_x:		Sensor orientation remapping for x-axis
// @axis_y:		Sensor orientation remapping for y-axis
// @axis_z:		Sensor orientation remapping for z-axis
// @driver_features:	Enable bits for different features. Disabled by default
// @default_rate:	Default sampling rate. 0 means reset default
// @setup_resources:	Interrupt line setup call back function
// @release_resources:	Interrupt line release call back function
// @st_min_limits:	Selftest acceptance minimum values (x, y, z)
// @st_max_limits:	Selftest acceptance maximum values (x, y, z)
// @irq2:		Irq line 2 number
//
// Platform data is used to setup the sensor chip. Meaning of the different
// chip features can be found from the data sheet. It is publicly available
// at www.st.com web pages. Currently the platform data is used
// only for the 8 bit device. The 8 bit device has two wake up / free fall
// detection units and click detection unit. There are plenty of ways to
// configure the chip which makes is quite hard to explain deeper meaning of
// the fields here. Behaviour of the detection blocks varies heavily depending
// on the configuration. For example, interrupt detection block can use high
// pass filtered data which makes it react to the changes in the acceleration.
// Irq_flags can be used to enable interrupt detection on the both edges.
// With proper chip configuration this produces interrupt when some trigger
// starts and when it goes away.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lis3lv02d_platform_data {
// please note: the 'click' feature is only supported for
// LIS[32]02DL variants of the chip and will be ignored for
// others

    pub click_flags: c_uchar,
    pub click_thresh_x: c_uchar,
    pub click_thresh_y: c_uchar,
    pub click_thresh_z: c_uchar,
    pub click_time_limit: c_uchar,
    pub click_latency: c_uchar,
    pub click_window: c_uchar,

    pub irq_cfg: c_uchar,
    pub /: *mut *mut unsigned char irq_flags1; / Additional irq edge / level flags,
    pub /: *mut *mut unsigned char irq_flags2; / Additional irq edge / level flags,
    pub duration1: c_uchar,
    pub duration2: c_uchar,

    pub wakeup_flags: c_uchar,
    pub wakeup_thresh: c_uchar,
    pub wakeup_flags2: c_uchar,
    pub wakeup_thresh2: c_uchar,
pub const LIS3_HIPASS_CUTFF_8HZ: c_int = 0;
pub const LIS3_HIPASS_CUTFF_4HZ: c_int = 1;
pub const LIS3_HIPASS_CUTFF_2HZ: c_int = 2;
pub const LIS3_HIPASS_CUTFF_1HZ: c_int = 3;

    pub hipass_ctrl: c_uchar,
pub const LIS3_NO_MAP: c_int = 0;
pub const LIS3_DEV_X: c_int = 1;
pub const LIS3_DEV_Y: c_int = 2;
pub const LIS3_DEV_Z: c_int = 3;

    pub axis_x: i8,
    pub axis_y: i8,
    pub axis_z: i8,
pub const LIS3_USE_BLOCK_READ: c_uint = 0x02;
    pub driver_features: u16,
    pub default_rate: c_int,
    pub (*setup_resources)(void): *mut c_int,
    pub (*release_resources)(void): *mut c_int,
// Limits for selftest are specified in chip data sheet
    pub /: *mut *mut s16 st_min_limits[3]; / min pass limit x, y, z,
    pub /: *mut *mut s16 st_max_limits[3]; / max pass limit x, y, z,
    pub irq2: c_int,
}
