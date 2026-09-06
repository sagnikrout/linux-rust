//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/invensense_mpu6050.h
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
// Copyright (C) 2012 Invensense, Inc.
//
// struct inv_mpu6050_platform_data - Platform data for the mpu driver
// @orientation:	Orientation matrix of the chip (deprecated in favor of
// mounting matrix retrieved from device-tree)
//
// Contains platform specific information on how to configure the MPU6050 to
// work on this platform.  The orientation matrices are 3x3 rotation matrices
// that are applied to the data to rotate from the mounting orientation to the
// platform orientation.  The values must be one of 0, 1, or -1 and each row and
// column should have exactly 1 non-zero value.
//
// Deprecated in favor of mounting matrix retrieved from device-tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_mpu6050_platform_data {
    pub orientation: [__s8; 9],
}
