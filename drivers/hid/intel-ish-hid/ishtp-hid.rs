//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ishtp-hid.h
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
// ISHTP-HID glue driver's definitions.
//
// Copyright (c) 2014-2016, Intel Corporation.
//
// The fixed ISH product and vendor id
pub const ISH_HID_VENDOR: c_uint = 0x8086;
pub const ISH_HID_PRODUCT: c_uint = 0x22D8;
pub const ISH_HID_VERSION: c_uint = 0x0200;
pub const CMD_MASK: c_uint = 0x7F;
pub const IS_RESPONSE: c_uint = 0x80;
// Used to dump to Linux trace buffer, if enabled

// ISH HID message structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostif_msg_hdr {
    pub /: *mut *mut uint8_t command; / Bit 7: is_response,
    pub device_id: u8,
    pub status: u8,
    pub flags: u8,
    pub size: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostif_msg {
    pub hdr: hostif_msg_hdr,
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hostif_msg_to_sensor {
    pub hdr: hostif_msg_hdr,
    pub report_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_info {
    pub dev_id: u32,
    pub dev_class: u8,
    pub pid: u16,
    pub vid: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_version {
    pub major: u8,
    pub minor: u8,
    pub hotfix: u8,
    pub build: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report {
    pub size: u16,
    pub msg: hostif_msg_hdr,
    pub __packed: },
// struct for ISHTP aggregated input data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_list {
    pub total_size: u16,
    pub num_of_reports: u8,
    pub flags: u8,
    pub reports: [report; ],
    pub __packed: },
// HOSTIF commands
pub const HOSTIF_HID_COMMAND_BASE: c_int = 0;
pub const HOSTIF_GET_HID_DESCRIPTOR: c_int = 0;
pub const HOSTIF_GET_REPORT_DESCRIPTOR: c_int = 1;
pub const HOSTIF_GET_FEATURE_REPORT: c_int = 2;
pub const HOSTIF_SET_FEATURE_REPORT: c_int = 3;
pub const HOSTIF_GET_INPUT_REPORT: c_int = 4;
pub const HOSTIF_PUBLISH_INPUT_REPORT: c_int = 5;
pub const HOSTIF_PUBLISH_INPUT_REPORT_LIST: c_int = 6;
pub const HOSTIF_DM_COMMAND_BASE: c_int = 32;
pub const HOSTIF_DM_ENUM_DEVICES: c_int = 33;
pub const HOSTIF_DM_ADD_DEVICE: c_int = 34;
pub const MAX_HID_DEVICES: c_int = 32;
//
// struct ishtp_cl_data - Encapsulate per ISH TP HID Client
// @enum_device_done:	Enum devices response complete flag
// @hid_descr_done:	HID descriptor complete flag
// @report_descr_done:	Get report descriptor complete flag
// @init_done:		Init process completed successfully
// @suspended:		System is under suspend state or in progress
// @num_hid_devices:	Number of HID devices enumerated in this client
// @cur_hid_dev:	This keeps track of the device index for which
// initialization and registration with HID core
// in progress.
// @hid_devices:	Store vid/pid/devid for each enumerated HID device
// @report_descr:	Stores the raw report descriptors for each HID device
// @report_descr_size:	Report description of size of above repo_descr[]
// @hid_sensor_hubs:	Pointer to hid_device for all HID device, so that
// when clients are removed, they can be freed
// @hid_descr:		Pointer to hid descriptor for each enumerated hid
// device
// @hid_descr_size:	Size of each above report descriptor
// @init_wait:		Wait queue to wait during initialization, where the
// client send message to ISH FW and wait for response
// @ishtp_hid_wait:	The wait for get report during wait callback from hid
// core
// @bad_recv_cnt:	Running count of packets received with error
// @multi_packet_cnt:	Count of fragmented packet count
//
// This structure is used to store completion flags and per client data like
// report description, number of HID devices etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_cl_data {
// completion flags
    pub enum_devices_done: bool,
    pub hid_descr_done: bool,
    pub report_descr_done: bool,
    pub init_done: bool,
    pub suspended: bool,
    pub num_hid_devices: c_uint,
    pub cur_hid_dev: c_uint,
    pub hid_dev_count: c_uint,
    pub hid_devices: *mut device_info,
    pub report_descr: [*mut c_uchar; MAX_HID_DEVICES],
    pub report_descr_size: [c_int; MAX_HID_DEVICES],
    pub hid_sensor_hubs: [*mut hid_device; MAX_HID_DEVICES],
    pub hid_descr: [*mut c_uchar; MAX_HID_DEVICES],
    pub hid_descr_size: [c_int; MAX_HID_DEVICES],
    pub init_wait: wait_queue_head_t,
    pub ishtp_resume_wait: wait_queue_head_t,
    pub hid_ishtp_cl: *mut ishtp_cl,
// Statistics
    pub bad_recv_cnt: c_uint,
    pub multi_packet_cnt: c_int,
    pub work: work_struct,
    pub resume_work: work_struct,
    pub cl_device: *mut ishtp_cl_device,
}

//
// struct ishtp_hid_data - Per instance HID data
// @index:		Device index in the order of enumeration
// @request_done:	Get Feature/Input report complete flag
// used during get/set request from hid core
// @client_data:	Link to the client instance
// @hid_wait:		Completion waitq
//
// @raw_get_req:	Flag indicating raw get request ongoing
// @raw_buf:		raw request buffer filled on receiving get report
// @raw_buf_size:	raw request buffer size
// Used to tie hid hid->driver data to driver client instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_hid_data {
    pub index: c_int,
    pub request_done: bool,
    pub client_data: *mut ishtp_cl_data,
    pub hid_wait: wait_queue_head_t,
// raw request
    pub raw_get_req: bool,
    pub raw_buf: *mut u8,
    pub raw_buf_size: usize,
}

// Interface functions between HID LL driver and ISH TP client
extern "C" {
    pub fn ishtp_hid_remove(client_data: *mut ishtp_cl_data);
}
extern "C" {
    pub fn ishtp_hid_link_ready_wait(client_data: *mut ishtp_cl_data) -> c_int;
}
extern "C" {
    pub fn ishtp_hid_wakeup(hid: *mut hid_device);
}
