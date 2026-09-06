//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ak5558.h
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
// Audio driver header for AK5558
//
// Copyright (C) 2016 Asahi Kasei Microdevices Corporation
// Copyright 2018 NXP
//
pub const AK5558_00_POWER_MANAGEMENT1: c_uint = 0x00;
pub const AK5558_01_POWER_MANAGEMENT2: c_uint = 0x01;
pub const AK5558_02_CONTROL1: c_uint = 0x02;
pub const AK5558_03_CONTROL2: c_uint = 0x03;
pub const AK5558_04_CONTROL3: c_uint = 0x04;
pub const AK5558_05_DSD: c_uint = 0x05;
// AK5558_02_CONTROL1 fields

// AK5558_03_CONTROL2 fields

