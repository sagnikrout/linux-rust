//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/humidity/hts221.h
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
// STMicroelectronics hts221 sensor driver
//
// Copyright 2016 STMicroelectronics Inc.
//
// Lorenzo Bianconi <lorenzo.bianconi@st.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hts221_sensor_type {
    HTS221_SENSOR_H,
    HTS221_SENSOR_T,
    HTS221_SENSOR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hts221_sensor {
    pub cur_avg_idx: u8,
    pub b_gen: int slope,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hts221_hw {
    pub name: *const c_char,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub trig: *mut iio_trigger,
    pub irq: c_int,
    pub sensors: [hts221_sensor; HTS221_SENSOR_MAX],
    pub enabled: bool,
    pub odr: u8,
// Ensure natural alignment of timestamp
    pub channels: [__le16; 2],
    pub ts: aligned_s64,
    pub scan: },
}

extern "C" {
    pub fn hts221_set_enable(hw: *mut hts221_hw, enable: bool) -> c_int;
}
extern "C" {
    pub fn hts221_allocate_buffers(iio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn hts221_allocate_trigger(iio_dev: *mut iio_dev) -> c_int;
}
