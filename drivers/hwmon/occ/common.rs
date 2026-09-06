//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwmon/occ/common.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright IBM Corp 2019

pub const OCC_RESP_DATA_BYTES: c_int = 4089;
//
// Same response format for all OCC versions.
// Allocate the largest possible response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_response {
    pub seq_no: u8,
    pub cmd_type: u8,
    pub return_status: u8,
    pub data_length: __be16,
    pub data: [u8; OCC_RESP_DATA_BYTES],
    pub checksum: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_sensor_data_block_header {
    pub eye_catcher: [u8; 4],
    pub reserved: u8,
    pub sensor_format: u8,
    pub sensor_length: u8,
    pub num_sensors: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_sensor_data_block {
    pub header: occ_sensor_data_block_header,
    pub data: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_poll_response_header {
    pub status: u8,
    pub ext_status: u8,
    pub occs_present: u8,
    pub config_data: u8,
    pub occ_state: u8,
    pub mode: u8,
    pub ips_status: u8,
    pub error_log_id: u8,
    pub error_log_start_address: __be32,
    pub error_log_length: __be16,
    pub reserved: u16,
    pub occ_code_level: [u8; 16],
    pub eye_catcher: [u8; 6],
    pub num_sensor_data_blocks: u8,
    pub sensor_data_block_header_version: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_poll_response {
    pub header: occ_poll_response_header,
    pub block: occ_sensor_data_block,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_sensor {
    pub num_sensors: u8,
    pub version: u8,
    pub /: *mut *mut *mut void data; / pointer to sensor data start within response,
}

//
// OCC only provides one sensor data block of each type, but any number of
// sensors within that block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_sensors {
    pub temp: occ_sensor,
    pub freq: occ_sensor,
    pub power: occ_sensor,
    pub caps: occ_sensor,
    pub extended: occ_sensor,
}

//
// Use our own attribute struct so we can dynamically allocate space for the
// name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_attribute {
    pub name: [c_char; 32],
    pub sensor: sensor_device_attribute_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ {
    pub bus_dev: *mut device,
    pub resp: occ_response,
    pub sensors: occ_sensors,
    pub /: *mut *mut int powr_sample_time_us; / average power sample time,
    pub /: *mut *mut u8 poll_cmd_data; / to perform OCC poll command,
    pub resp_len): usize,
    pub next_update: c_ulong,
    pub /: *mut *mut mutex lock; / lock OCC access,
    pub /: *mut *mut mutex hwmon_lock; / serialize hwmon registration/removal,
    pub hwmon: *mut device,
    pub attrs: *mut occ_attribute,
    pub group: attribute_group,
    pub groups: [*const attribute_group; 2],
    pub active: bool,
    pub /: *mut *mut int error; / final transfer error after retry,
    pub /: *mut *mut int last_error; / latest transfer error,
    pub /: *mut *mut unsigned int error_count; / number of xfr errors observed,
    pub /: *mut *mut unsigned long last_safe; / time OCC entered "safe" state,
//
// Store the previous state data for comparison in order to notify
// sysfs readers of state changes.
//
    pub prev_error: c_int,
    pub prev_stat: u8,
    pub prev_ext_stat: u8,
    pub prev_occs_present: u8,
    pub prev_ips_status: u8,
    pub prev_mode: u8,
}

extern "C" {
    pub fn occ_active(occ: *mut occ, active: bool) -> c_int;
}
extern "C" {
    pub fn occ_setup(occ: *mut occ) -> c_int;
}
extern "C" {
    pub fn occ_setup_sysfs(occ: *mut occ) -> c_int;
}
extern "C" {
    pub fn occ_shutdown(occ: *mut occ);
}
extern "C" {
    pub fn occ_shutdown_sysfs(occ: *mut occ);
}
extern "C" {
    pub fn occ_sysfs_poll_done(occ: *mut occ);
}
extern "C" {
    pub fn occ_update_response(occ: *mut occ) -> c_int;
}
