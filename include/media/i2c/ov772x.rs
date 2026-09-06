//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/ov772x.h
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
// ov772x Camera
//
// Copyright (C) 2008 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// for flags

//
// for Edge ctrl
//
// strength also control Auto or Manual Edge Control Mode
// see also OV772X_MANUAL_EDGE_CTRL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov772x_edge_ctrl {
    pub strength: c_uchar,
    pub threshold: c_uchar,
    pub upper: c_uchar,
    pub lower: c_uchar,
}

pub const OV772X_MANUAL_EDGE_CTRL: c_uint = 0x80 /* un-used bit of strength */;
pub const OV772X_EDGE_STRENGTH_MASK: c_uint = 0x1F;
pub const OV772X_EDGE_THRESHOLD_MASK: c_uint = 0x0F;
pub const OV772X_EDGE_UPPER_MASK: c_uint = 0xFF;
pub const OV772X_EDGE_LOWER_MASK: c_uint = 0xFF;

//
// struct ov772x_camera_info -	ov772x driver interface structure
// @flags:		Sensor configuration flags
// @edgectrl:		Sensor edge control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov772x_camera_info {
    pub flags: c_ulong,
    pub edgectrl: ov772x_edge_ctrl,
}
