//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/wilco-ec.h
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
// ChromeOS Wilco Embedded Controller
//
// Copyright 2018 Google LLC
//

// Message flags for using the mailbox() interface

// Normal commands have a maximum 32 bytes of data
pub const EC_MAILBOX_DATA_SIZE: c_int = 32;
//
// struct wilco_ec_device - Wilco Embedded Controller handle.
// @dev: Device handle.
// @mailbox_lock: Mutex to ensure one mailbox command at a time.
// @io_command: I/O port for mailbox command.  Provided by ACPI.
// @io_data: I/O port for mailbox data.  Provided by ACPI.
// @io_packet: I/O port for mailbox packet data.  Provided by ACPI.
// @data_buffer: Buffer used for EC communication.  The same buffer
// is used to hold the request and the response.
// @data_size: Size of the data buffer used for EC communication.
// @debugfs_pdev: The child platform_device used by the debugfs sub-driver.
// @rtc_pdev: The child platform_device used by the RTC sub-driver.
// @charger_pdev: Child platform_device used by the charger config sub-driver.
// @telem_pdev: The child platform_device used by the telemetry sub-driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_ec_device {
    pub dev: *mut device,
    pub mailbox_lock: mutex,
    pub io_command: *mut resource,
    pub io_data: *mut resource,
    pub io_packet: *mut resource,
    pub data_buffer: *mut c_void,
    pub data_size: usize,
    pub debugfs_pdev: *mut platform_device,
    pub rtc_pdev: *mut platform_device,
    pub charger_pdev: *mut platform_device,
    pub telem_pdev: *mut platform_device,
}

//
// struct wilco_ec_request - Mailbox request message format.
// @struct_version: Should be %EC_MAILBOX_PROTO_VERSION
// @checksum: Sum of all bytes must be 0.
// @mailbox_id: Mailbox identifier, specifies the command set.
// @mailbox_version: Mailbox interface version %EC_MAILBOX_VERSION
// @reserved: Set to zero.
// @data_size: Length of following data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_ec_request {
    pub struct_version: u8,
    pub checksum: u8,
    pub mailbox_id: u16,
    pub mailbox_version: u8,
    pub reserved: u8,
    pub data_size: u16,
    pub __packed: },
//
// struct wilco_ec_response - Mailbox response message format.
// @struct_version: Should be %EC_MAILBOX_PROTO_VERSION
// @checksum: Sum of all bytes must be 0.
// @result: Result code from the EC.  Non-zero indicates an error.
// @data_size: Length of the response data buffer.
// @reserved: Set to zero.
// @data: Response data buffer.  Max size is %EC_MAILBOX_DATA_SIZE_EXTENDED.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_ec_response {
    pub struct_version: u8,
    pub checksum: u8,
    pub result: u16,
    pub data_size: u16,
    pub reserved: [u8; 2],
    pub data: [u8; ],
    pub __packed: },
//
// enum wilco_ec_msg_type - Message type to select a set of command codes.
// @WILCO_EC_MSG_LEGACY: Legacy EC messages for standard EC behavior.
// @WILCO_EC_MSG_PROPERTY: Get/Set/Sync EC controlled NVRAM property.
// @WILCO_EC_MSG_TELEMETRY: Request telemetry data from the EC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wilco_ec_msg_type {
    WILCO_EC_MSG_LEGACY = 0x00f0,
    WILCO_EC_MSG_PROPERTY = 0x00f2,
    WILCO_EC_MSG_TELEMETRY = 0x00f5,
}

//
// struct wilco_ec_message - Request and response message.
// @type: Mailbox message type.
// @flags: Message flags, e.g. %WILCO_EC_FLAG_NO_RESPONSE.
// @request_size: Number of bytes to send to the EC.
// @request_data: Buffer containing the request data.
// @response_size: Number of bytes to read from EC.
// @response_data: Buffer containing the response data, should be
// response_size bytes and allocated by caller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_ec_message {
    pub type: wilco_ec_msg_type,
    pub flags: u8,
    pub request_size: usize,
    pub request_data: *mut c_void,
    pub response_size: usize,
    pub response_data: *mut c_void,
}

//
// wilco_ec_mailbox() - Send request to the EC and receive the response.
// @ec: Wilco EC device.
// @msg: Wilco EC message.
//
// Return: Number of bytes received or negative error code on failure.
//
extern "C" {
    pub fn wilco_ec_mailbox(ec: *mut wilco_ec_device, msg: *mut wilco_ec_message) -> c_int;
}
//
// wilco_keyboard_leds_init() - Set up the keyboard backlight LEDs.
// @ec: EC device to query.
//
// After this call, the keyboard backlight will be exposed through a an LED
// device at /sys/class/leds.
//
// This may sleep because it uses wilco_ec_mailbox().
//
// Return: 0 on success, negative error code on failure.
//
extern "C" {
    pub fn wilco_keyboard_leds_init(ec: *mut wilco_ec_device) -> c_int;
}
//
// A Property is typically a data item that is stored to NVRAM
// by the EC. Each of these data items has an index associated
// with it, known as the Property ID (PID). Properties may have
// variable lengths, up to a max of WILCO_EC_PROPERTY_MAX_SIZE
// bytes. Properties can be simple integers, or they may be more
// complex binary data.
//
pub const WILCO_EC_PROPERTY_MAX_SIZE: c_int = 4;
//
// struct ec_property_set_msg - Message to get or set a property.
// @property_id: Which property to get or set.
// @length: Number of bytes of |data| that are used.
// @data: Actual property data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_ec_property_msg {
    pub property_id: u32,
    pub length: c_int,
    pub data: [u8; WILCO_EC_PROPERTY_MAX_SIZE],
}

//
// wilco_ec_get_property() - Retrieve a property from the EC.
// @ec: Embedded Controller device.
// @prop_msg: Message for request and response.
//
// The property_id field of |prop_msg| should be filled before calling this
// function. The result will be stored in the data and length fields.
//
// Return: 0 on success, negative error code on failure.
//
// wilco_ec_set_property() - Store a property on the EC.
// @ec: Embedded Controller device.
// @prop_msg: Message for request and response.
//
// The property_id, length, and data fields of |prop_msg| should be
// filled before calling this function.
//
// Return: 0 on success, negative error code on failure.
//
// wilco_ec_get_byte_property() - Retrieve a byte-size property from the EC.
// @ec: Embedded Controller device.
// @property_id: Which property to retrieve.
// @val: The result value, will be filled by this function.
//
// Return: 0 on success, negative error code on failure.
//
// wilco_ec_get_byte_property() - Store a byte-size property on the EC.
// @ec: Embedded Controller device.
// @property_id: Which property to store.
// @val: Value to store.
//
// Return: 0 on success, negative error code on failure.
//
// wilco_ec_add_sysfs() - Create sysfs entries
// @ec: Wilco EC device
//
// wilco_ec_remove_sysfs() needs to be called afterwards
// to perform the necessary cleanup.
//
// Return: 0 on success or negative error code on failure.
//
extern "C" {
    pub fn wilco_ec_add_sysfs(ec: *mut wilco_ec_device) -> c_int;
}
extern "C" {
    pub fn wilco_ec_remove_sysfs(ec: *mut wilco_ec_device);
}
