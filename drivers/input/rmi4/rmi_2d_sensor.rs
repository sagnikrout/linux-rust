//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/rmi4/rmi_2d_sensor.h
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
// Copyright (c) 2011-2016 Synaptics Incorporated
// Copyright (c) 2011 Unixphere
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rmi_2d_sensor_object_type {
    RMI_2D_OBJECT_NONE,
    RMI_2D_OBJECT_FINGER,
    RMI_2D_OBJECT_STYLUS,
    RMI_2D_OBJECT_PALM,
    RMI_2D_OBJECT_UNCLASSIFIED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_2d_sensor_abs_object {
    pub type: rmi_2d_sensor_object_type,
    pub mt_tool: c_int,
    pub x: u16,
    pub y: u16,
    pub z: u8,
    pub wx: u8,
    pub wy: u8,
}

//
// @axis_align - controls parameters that are useful in system prototyping
// and bring up.
// @max_x - The maximum X coordinate that will be reported by this sensor.
// @max_y - The maximum Y coordinate that will be reported by this sensor.
// @nbr_fingers - How many fingers can this sensor report?
// @data_pkt - buffer for data reported by this sensor.
// @pkt_size - number of bytes in that buffer.
// @attn_size - Size of the HID attention report (only contains abs data).
// position when two fingers are on the device.  When this is true, we
// assume we have one of those sensors and report events appropriately.
// @sensor_type - indicates whether we're touchscreen or touchpad.
// @input - input device for absolute pointing stream
// @input_phys - buffer for the absolute phys name for this sensor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_2d_sensor {
    pub axis_align: rmi_2d_axis_alignment,
    pub tracking_pos: *mut input_mt_pos,
    pub tracking_slots: *mut c_int,
    pub kernel_tracking: bool,
    pub objs: *mut rmi_2d_sensor_abs_object,
    pub dmax: c_int,
    pub min_x: u16,
    pub max_x: u16,
    pub min_y: u16,
    pub max_y: u16,
    pub nbr_fingers: u8,
    pub data_pkt: *mut u8,
    pub pkt_size: u32,
    pub attn_size: u32,
    pub topbuttonpad: bool,
    pub sensor_type: rmi_sensor_type,
    pub input: *mut input_dev,
    pub fn: *mut rmi_function,
    pub input_phys: [c_char; 32],
    pub report_abs: u8,
    pub report_rel: u8,
    pub x_mm: u8,
    pub y_mm: u8,
    pub dribble: rmi_reg_state,
    pub palm_detect: rmi_reg_state,
}

extern "C" {
    pub fn rmi_2d_sensor_rel_report(sensor: *mut rmi_2d_sensor, x: c_int, y: c_int);
}
