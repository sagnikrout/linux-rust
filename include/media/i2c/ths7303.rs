//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/ths7303.h
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
// Copyright (C) 2013 Texas Instruments Inc
//
// Copyright 2013 Cisco Systems, Inc. and/or its affiliates.
//
// Contributors:
// Hans Verkuil <hverkuil@kernel.org>
// Lad, Prabhakar <prabhakar.lad@ti.com>
// Martin Bugge <marbugge@cisco.com>
//
// struct ths7303_platform_data - Platform dependent data
// @ch_1: Bias value for channel one.
// @ch_2: Bias value for channel two.
// @ch_3: Bias value for channel three.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ths7303_platform_data {
    pub ch_1: u8,
    pub ch_2: u8,
    pub ch_3: u8,
}
