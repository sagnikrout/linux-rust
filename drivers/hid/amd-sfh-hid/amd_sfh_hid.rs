//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/amd-sfh-hid/amd_sfh_hid.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMD MP2 Sensors transport driver
//
// Copyright 2020-2021 Advanced Micro Devices, Inc.
// Authors: Nehal Bakulchandra Shah <Nehal-bakulchandra.shah@amd.com>
// Sandeep Singh <sandeep.singh@amd.com>
// Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//
pub const MAX_HID_DEVICES: c_int = 7;
pub const AMD_SFH_HID_VENDOR: c_uint = 0x1022;
pub const AMD_SFH_HID_PRODUCT: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_list {
    pub hid: *mut hid_device,
    pub list: list_head,
    pub report_id: u8,
    pub sensor_idx: u8,
    pub report_type: u8,
    pub current_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_input_data {
    pub sensor_virt_addr: [*mut u32; MAX_HID_DEVICES],
    pub input_report: [*mut u8; MAX_HID_DEVICES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtp_cl_data {
    pub init_done: u8,
    pub cur_hid_dev: u32,
    pub is_any_sensor_enabled: bool,
    pub hid_dev_count: u32,
    pub num_hid_devices: u32,
    pub hid_devices: *mut device_info,
    pub report_descr: [*mut u8; MAX_HID_DEVICES],
    pub report_descr_sz: [c_int; MAX_HID_DEVICES],
    pub hid_sensor_hubs: [*mut hid_device; MAX_HID_DEVICES],
    pub hid_descr: [*mut u8; MAX_HID_DEVICES],
    pub hid_descr_size: [c_int; MAX_HID_DEVICES],
    pub phys_addr_base: phys_addr_t,
    pub sensor_dma_addr: [dma_addr_t; MAX_HID_DEVICES],
    pub sensor_sts: [u32; MAX_HID_DEVICES],
    pub sensor_requested_cnt: [u32; MAX_HID_DEVICES],
    pub report_type: [u8; MAX_HID_DEVICES],
    pub report_id: [u8; MAX_HID_DEVICES],
    pub sensor_idx: [u8; MAX_HID_DEVICES],
    pub feature_report: [*mut u8; MAX_HID_DEVICES],
    pub request_done: [u8; MAX_HID_DEVICES],
    pub in_data: *mut amd_input_data,
    pub work: delayed_work,
    pub work_buffer: delayed_work,
    pub req_list: request_list,
}

//
// struct amdtp_hid_data - Per instance HID data
// @index:		Device index in the order of enumeration
// @request_done:	Get Feature/Input report complete flag
// used during get/set request from hid core
// @cli_data:		Link to the client instance
// @hid_wait:		Completion waitq
//
// Used to tie hid->driver data to driver client instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtp_hid_data {
    pub index: c_int,
    pub cli_data: *mut amdtp_cl_data,
    pub hid_wait: wait_queue_head_t,
}

// Interface functions between HID LL driver and AMD SFH client
extern "C" {
    pub fn amdtp_hid_probe(cur_hid_dev: u32, cli_data: *mut amdtp_cl_data) -> c_int;
}
extern "C" {
    pub fn amdtp_hid_remove(cli_data: *mut amdtp_cl_data);
}
extern "C" {
    pub fn amd_sfh_get_report(hid: *mut hid_device, report_id: c_int, report_type: c_int) -> c_int;
}
extern "C" {
    pub fn amd_sfh_set_report(hid: *mut hid_device, report_id: c_int, report_type: c_int);
}
extern "C" {
    pub fn amdtp_hid_wakeup(hid: *mut hid_device);
}
