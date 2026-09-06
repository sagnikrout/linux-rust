//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hid-over-i2c.h
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
// Copyright 2024 Intel Corporation

// Input report type definition in HIDI2C protocol
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hidi2c_report_type {
    HIDI2C_RESERVED = 0,
    HIDI2C_INPUT,
    HIDI2C_OUTPUT,
    HIDI2C_FEATURE,
}

// Power state type definition in HIDI2C protocol
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hidi2c_power_state {
    HIDI2C_ON,
    HIDI2C_SLEEP,
}

// Opcode type definition in HIDI2C protocol
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hidi2c_opcode {
    HIDI2C_RESET = 1,
    HIDI2C_GET_REPORT,
    HIDI2C_SET_REPORT,
    HIDI2C_GET_IDLE,
    HIDI2C_SET_IDLE,
    HIDI2C_GET_PROTOCOL,
    HIDI2C_SET_PROTOCOL,
    HIDI2C_SET_POWER,
}

//
// struct hidi2c_report_packet - Report packet definition in HIDI2C protocol
// @len: data field length
// @data: HIDI2C report packet data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidi2c_report_packet {
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },

pub const HIDI2C_CMD_MAX_RI: c_uint = 0x0F;
//
// HIDI2C command data packet - Command packet definition in HIDI2C protocol
// @report_id:		[0:3] report id (<15) for features or output reports
// @report_type:	[4:5] indicate report type, reference to hidi2c_report_type
// @reserved0:		[6:7] reserved bits
// @opcode:		[8:11] command operation code, reference to hidi2c_opcode
// @reserved1:		[12:15] reserved bits
// @report_id_optional: [23:16] appended 3rd byte.
// If the report_id in the low byte is set to the
// sentinel value (HIDI2C_CMD_MAX_RI), then this
// optional third byte represents the report id (>=15)
// Otherwise, not this 3rd byte.
//

pub const HIDI2C_HID_DESC_BCDVERSION: c_uint = 0x100;
//
// struct hidi2c_dev_descriptor - HIDI2C device descriptor definition
// @dev_desc_len: The length of the complete device descriptor, fixed to 0x1E (30).
// @bcd_ver: The version number of the HIDI2C protocol supported.
// In binary coded decimal (BCD) format.
// @report_desc_len: The length of the report descriptor
// @report_desc_reg: The register address to retrieve report descriptor
// @input_reg: the register address to retrieve input report
// @max_input_len: The length of the largest possible HID input (or feature) report
// @output_reg: the register address to send output report
// @max_output_len: The length of the largest output (or feature) report
// @cmd_reg: the register address to send command
// @data_reg: the register address to send command data
// @vendor_id: Device manufacturers vendor ID
// @product_id: Device unique model/product ID
// @version_id: Device’s unique version
// @reserved0: Reserved and should be 0
// @reserved1: Reserved and should be 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidi2c_dev_descriptor {
    pub dev_desc_len: __le16,
    pub bcd_ver: __le16,
    pub report_desc_len: __le16,
    pub report_desc_reg: __le16,
    pub input_reg: __le16,
    pub max_input_len: __le16,
    pub output_reg: __le16,
    pub max_output_len: __le16,
    pub cmd_reg: __le16,
    pub data_reg: __le16,
    pub vendor_id: __le16,
    pub product_id: __le16,
    pub version_id: __le16,
    pub reserved0: __le16,
    pub reserved1: __le16,
    pub __packed: },

