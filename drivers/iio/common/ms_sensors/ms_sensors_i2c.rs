//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/common/ms_sensors/ms_sensors_i2c.h
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
// Measurements Specialties common sensor driver
//
// Copyright (c) 2015 Measurement-Specialties
//

pub const MS_SENSORS_TP_PROM_WORDS_NB: c_int = 8;
//
// struct ms_ht_dev - Humidity/Temperature sensor device structure
// @client:	i2c client
// @lock:	lock protecting the i2c conversion
// @res_index:	index to selected sensor resolution
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_ht_dev {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub res_index: u8,
}

//
// struct ms_hw_data - Temperature/Pressure sensor hardware data
// @prom_len:		number of words in the PROM
// @max_res_index:	maximum sensor resolution index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_tp_hw_data {
    pub prom_len: u8,
    pub max_res_index: u8,
}

//
// struct ms_tp_dev - Temperature/Pressure sensor device structure
// @client:	i2c client
// @lock:	lock protecting the i2c conversion
// @prom:	array of PROM coefficients used for conversion. Added element
// for CRC computation
// @res_index:	index to selected sensor resolution
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_tp_dev {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub hw: *const ms_tp_hw_data,
    pub prom: [u16; MS_SENSORS_TP_PROM_WORDS_NB],
    pub res_index: u8,
}

extern "C" {
    pub fn ms_sensors_reset(cli: *mut c_void, cmd: u8, delay: c_uint) -> c_int;
}
extern "C" {
    pub fn ms_sensors_read_prom_word(cli: *mut c_void, cmd: c_int, word: *mut u16) -> c_int;
}
extern "C" {
    pub fn ms_sensors_read_serial(client: *mut i2c_client, sn: *mut u64) -> c_int;
}
extern "C" {
    pub fn ms_sensors_show_serial(dev_data: *mut ms_ht_dev, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn ms_sensors_write_resolution(dev_data: *mut ms_ht_dev, i: u8) -> isize;
}
extern "C" {
    pub fn ms_sensors_show_battery_low(dev_data: *mut ms_ht_dev, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn ms_sensors_show_heater(dev_data: *mut ms_ht_dev, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn ms_sensors_tp_read_prom(dev_data: *mut ms_tp_dev) -> c_int;
}
