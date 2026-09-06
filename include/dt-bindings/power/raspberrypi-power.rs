//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/raspberrypi-power.h
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
// Copyright © 2015 Broadcom
//
// These power domain indices are the firmware interface's indices
// minus one.
//
pub const RPI_POWER_DOMAIN_I2C0: c_int = 0;
pub const RPI_POWER_DOMAIN_I2C1: c_int = 1;
pub const RPI_POWER_DOMAIN_I2C2: c_int = 2;
pub const RPI_POWER_DOMAIN_VIDEO_SCALER: c_int = 3;
pub const RPI_POWER_DOMAIN_VPU1: c_int = 4;
pub const RPI_POWER_DOMAIN_HDMI: c_int = 5;
pub const RPI_POWER_DOMAIN_USB: c_int = 6;
pub const RPI_POWER_DOMAIN_VEC: c_int = 7;
pub const RPI_POWER_DOMAIN_JPEG: c_int = 8;
pub const RPI_POWER_DOMAIN_H264: c_int = 9;
pub const RPI_POWER_DOMAIN_V3D: c_int = 10;
pub const RPI_POWER_DOMAIN_ISP: c_int = 11;
pub const RPI_POWER_DOMAIN_UNICAM0: c_int = 12;
pub const RPI_POWER_DOMAIN_UNICAM1: c_int = 13;
pub const RPI_POWER_DOMAIN_CCP2RX: c_int = 14;
pub const RPI_POWER_DOMAIN_CSI2: c_int = 15;
pub const RPI_POWER_DOMAIN_CPI: c_int = 16;
pub const RPI_POWER_DOMAIN_DSI0: c_int = 17;
pub const RPI_POWER_DOMAIN_DSI1: c_int = 18;
pub const RPI_POWER_DOMAIN_TRANSPOSER: c_int = 19;
pub const RPI_POWER_DOMAIN_CCP2TX: c_int = 20;
pub const RPI_POWER_DOMAIN_CDP: c_int = 21;
pub const RPI_POWER_DOMAIN_ARM: c_int = 22;
pub const RPI_POWER_DOMAIN_COUNT: c_int = 23;
