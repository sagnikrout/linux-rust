//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/int1092/intel_sar.h
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
// Copyright (c) 2021, Intel Corporation.
//
pub const COMMAND_ID_DEV_MODE: c_int = 1;
pub const COMMAND_ID_CONFIG_TABLE: c_int = 2;

pub const MAX_DEV_MODES: c_int = 50;
pub const MAX_REGULATORY: c_int = 3;

pub const SAR_EVENT: c_uint = 0x80;

pub const TOTAL_DATA: c_int = 4;
//
// Structure wwan_device_mode_info - device mode information
// Holds the data that needs to be passed to userspace.
// The data is updated from the BIOS sensor information.
// @device_mode: Specific mode of the device
// @bandtable_index: Index of RF band
// @antennatable_index: Index of antenna
// @sartable_index: Index of SAR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_device_mode_info {
    pub device_mode: c_int,
    pub bandtable_index: c_int,
    pub antennatable_index: c_int,
    pub sartable_index: c_int,
}

//
// Structure wwan_device_mode_configuration - device configuration
// Holds the data that is configured and obtained on probe event.
// The data is updated from the BIOS sensor information.
// @version: Mode configuration version
// @total_dev_mode: Total number of device modes
// @device_mode_info: pointer to structure wwan_device_mode_info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_device_mode_configuration {
    pub version: c_int,
    pub total_dev_mode: c_int,
    pub device_mode_info: *mut wwan_device_mode_info,
}

//
// Structure wwan_supported_info - userspace datastore
// Holds the data that is obtained from userspace
// The data is updated from the userspace and send value back in the
// structure format that is mentioned here.
// @reg_mode_needed: regulatory mode set by user for tests
// @bios_table_revision: Version of SAR table
// @num_supported_modes: Total supported modes based on reg_mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_supported_info {
    pub reg_mode_needed: c_int,
    pub bios_table_revision: c_int,
    pub num_supported_modes: c_int,
}

//
// Structure wwan_sar_context - context of SAR
// Holds the complete context as long as the driver is in existence
// The context holds instance of the data used for different cases.
// @guid: Group id
// @handle: store acpi handle
// @reg_value: regulatory value
// Regulatory 0: FCC, 1: CE, 2: ISED
// @sar_device: platform_device type
// @sar_kobject: kobject for sysfs
// @supported_data: wwan_supported_info struct
// @sar_data: wwan_device_mode_info struct
// @config_data: wwan_device_mode_configuration array struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_sar_context {
    pub guid: guid_t,
    pub handle: acpi_handle,
    pub reg_value: c_int,
    pub sar_device: *mut platform_device,
    pub supported_data: wwan_supported_info,
    pub sar_data: wwan_device_mode_info,
    pub config_data: [wwan_device_mode_configuration; MAX_REGULATORY],
}
