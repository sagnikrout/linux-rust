//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pldmfw.h
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
// Copyright (C) 2018-2019, Intel Corporation.

pub const PLDM_STRING_TYPE_UNKNOWN: c_int = 0;
pub const PLDM_STRING_TYPE_ASCII: c_int = 1;
pub const PLDM_STRING_TYPE_UTF8: c_int = 2;
pub const PLDM_STRING_TYPE_UTF16: c_int = 3;
pub const PLDM_STRING_TYPE_UTF16LE: c_int = 4;
pub const PLDM_STRING_TYPE_UTF16BE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pldmfw_record {
    pub entry: list_head,
// List of descriptor TLVs
    pub descs: list_head,
// Component Set version string
    pub version_string: *const u8,
    pub version_type: u8,
    pub version_len: u8,
// Package Data length
    pub package_data_len: u16,
// Bitfield of Device Update Flags
    pub device_update_flags: u32,
// Package Data block
    pub package_data: *const u8,
// Bitmap of components applicable to this record
    pub component_bitmap: *mut c_ulong,
    pub component_bitmap_len: u16,
}

// Standard descriptor TLV identifiers
pub const PLDM_DESC_ID_PCI_VENDOR_ID: c_uint = 0x0000;
pub const PLDM_DESC_ID_IANA_ENTERPRISE_ID: c_uint = 0x0001;
pub const PLDM_DESC_ID_UUID: c_uint = 0x0002;
pub const PLDM_DESC_ID_PNP_VENDOR_ID: c_uint = 0x0003;
pub const PLDM_DESC_ID_ACPI_VENDOR_ID: c_uint = 0x0004;
pub const PLDM_DESC_ID_PCI_DEVICE_ID: c_uint = 0x0100;
pub const PLDM_DESC_ID_PCI_SUBVENDOR_ID: c_uint = 0x0101;
pub const PLDM_DESC_ID_PCI_SUBDEV_ID: c_uint = 0x0102;
pub const PLDM_DESC_ID_PCI_REVISION_ID: c_uint = 0x0103;
pub const PLDM_DESC_ID_PNP_PRODUCT_ID: c_uint = 0x0104;
pub const PLDM_DESC_ID_ACPI_PRODUCT_ID: c_uint = 0x0105;
pub const PLDM_DESC_ID_VENDOR_DEFINED: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pldmfw_desc_tlv {
    pub entry: list_head,
    pub data: *const u8,
    pub type: u16,
    pub size: u16,
}

pub const PLDM_CLASSIFICATION_UNKNOWN: c_uint = 0x0000;
pub const PLDM_CLASSIFICATION_OTHER: c_uint = 0x0001;
pub const PLDM_CLASSIFICATION_DRIVER: c_uint = 0x0002;
pub const PLDM_CLASSIFICATION_CONFIG_SW: c_uint = 0x0003;
pub const PLDM_CLASSIFICATION_APP_SW: c_uint = 0x0004;
pub const PLDM_CLASSIFICATION_INSTRUMENTATION: c_uint = 0x0005;
pub const PLDM_CLASSIFICATION_BIOS: c_uint = 0x0006;
pub const PLDM_CLASSIFICATION_DIAGNOSTIC_SW: c_uint = 0x0007;
pub const PLDM_CLASSIFICATION_OS: c_uint = 0x0008;
pub const PLDM_CLASSIFICATION_MIDDLEWARE: c_uint = 0x0009;
pub const PLDM_CLASSIFICATION_FIRMWARE: c_uint = 0x000A;
pub const PLDM_CLASSIFICATION_CODE: c_uint = 0x000B;
pub const PLDM_CLASSIFICATION_SERVICE_PACK: c_uint = 0x000C;
pub const PLDM_CLASSIFICATION_SOFTWARE_BUNDLE: c_uint = 0x000D;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pldmfw_component {
    pub entry: list_head,
// component identifier
    pub classification: u16,
    pub identifier: u16,
    pub options: u16,
    pub activation_method: u16,
    pub comparison_stamp: u32,
    pub component_size: u32,
    pub component_data: *const u8,
// Component version string
    pub version_string: *const u8,
    pub version_type: u8,
    pub version_len: u8,
// component index
    pub index: u8,
}

// Transfer flag used for sending components to the firmware

// Main entry point to the PLDM firmware update engine. Device drivers
// should embed this in a private structure and use container_of to obtain
// a pointer to their own data, used to implement the device specific
// operations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pldmfw_update_mode {
    PLDMFW_UPDATE_MODE_FULL,
    PLDMFW_UPDATE_MODE_SINGLE_COMPONENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pldmfw {
    pub ops: *const pldmfw_ops,
    pub dev: *mut device,
    pub component_identifier: u16,
    pub mode: pldmfw_update_mode,
}

extern "C" {
    pub fn pldmfw_op_pci_match_record(context: *mut pldmfw, record: *mut pldmfw_record) -> bool;
}
// Operations invoked by the generic PLDM firmware update engine. Used to
// implement device specific logic.
//
// @match_record: check if the device matches the given record. For
// convenience, a standard implementation is provided for PCI devices.
//
// @send_package_data: send the package data associated with the matching
// record to firmware.
//
// @send_component_table: send the component data associated with a given
// component to firmware. Called once for each applicable component.
//
// @flash_component: Flash the data for a given component to the device.
// Called once for each applicable component, after all component tables have
// been sent.
//
// @finalize_update: (optional) Finish the update. Called after all components
// have been flashed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pldmfw_ops {
    pub record): *mut *mut *mut bool (match_record)(struct pldmfw context, struct pldmfw_record,
    pub length): *const *const *const *const int (send_package_data)(struct pldmfw context, u8 data, u16,
    pub transfer_flag): u8,
    pub component): *mut *mut *mut int (flash_component)(struct pldmfw context, struct pldmfw_component,
    pub context): *mut *mut int (finalize_update)(struct pldmfw,
}

extern "C" {
    pub fn pldmfw_flash_image(context: *mut pldmfw, fw: *const firmware) -> c_int;
}
