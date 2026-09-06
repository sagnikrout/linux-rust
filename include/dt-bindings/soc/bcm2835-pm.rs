//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/soc/bcm2835-pm.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
pub const BCM2835_POWER_DOMAIN_GRAFX: c_int = 0;
pub const BCM2835_POWER_DOMAIN_GRAFX_V3D: c_int = 1;
pub const BCM2835_POWER_DOMAIN_IMAGE: c_int = 2;
pub const BCM2835_POWER_DOMAIN_IMAGE_PERI: c_int = 3;
pub const BCM2835_POWER_DOMAIN_IMAGE_ISP: c_int = 4;
pub const BCM2835_POWER_DOMAIN_IMAGE_H264: c_int = 5;
pub const BCM2835_POWER_DOMAIN_USB: c_int = 6;
pub const BCM2835_POWER_DOMAIN_DSI0: c_int = 7;
pub const BCM2835_POWER_DOMAIN_DSI1: c_int = 8;
pub const BCM2835_POWER_DOMAIN_CAM0: c_int = 9;
pub const BCM2835_POWER_DOMAIN_CAM1: c_int = 10;
pub const BCM2835_POWER_DOMAIN_CCP2TX: c_int = 11;
pub const BCM2835_POWER_DOMAIN_HDMI: c_int = 12;
pub const BCM2835_POWER_DOMAIN_COUNT: c_int = 13;
pub const BCM2835_RESET_V3D: c_int = 0;
pub const BCM2835_RESET_ISP: c_int = 1;
pub const BCM2835_RESET_H264: c_int = 2;
pub const BCM2835_RESET_COUNT: c_int = 3;
