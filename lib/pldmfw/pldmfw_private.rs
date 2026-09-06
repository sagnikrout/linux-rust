//! Automatically rewritten from C Header to Rust Module
//! Source: lib/pldmfw/pldmfw_private.h
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
// The following data structures define the layout of a firmware binary
// following the "PLDM For Firmware Update Specification", DMTF standard
// #DSP0267.
//
// pldmfw.c uses these structures to implement a simple engine that will parse
// a fw binary file in this format and perform a firmware update for a given
// device.
//
// Due to the variable sized data layout, alignment of fields within these
// structures is not guaranteed when reading. For this reason, all multi-byte
// field accesses should be done using the unaligned access macros.
// Additionally, the standard specifies that multi-byte fields are in
// LittleEndian format.
//
// The structure definitions are not made public, in order to keep direct
// accesses within code that is prepared to deal with the limitation of
// unaligned access.
//
// UUID for PLDM firmware packages: f018878c-cb7d-4943-9800-a02f059aca02
// Revision number of the PLDM header format this code supports
pub const PACKAGE_HEADER_FORMAT_REVISION: c_uint = 0x01;
// timestamp104 structure defined in PLDM Base specification
pub const PLDM_TIMESTAMP_SIZE: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pldm_timestamp {
    pub b: [u8; PLDM_TIMESTAMP_SIZE],
    pub __aligned(1): } __packed,
// Package Header Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pldm_header {
    pub /: *mut *mut uuid_t id; / PackageHeaderIdentifier,
    pub /: *mut *mut u8 revision; / PackageHeaderFormatRevision,
    pub /: *mut *mut __le16 size; / PackageHeaderSize,
    pub /: *mut *mut __pldm_timestamp release_date; / PackageReleaseDateTime,
    pub /: *mut *mut __le16 component_bitmap_len; / ComponentBitmapBitLength,
    pub /: *mut *mut u8 version_type; / PackageVersionStringType,
    pub /: *mut *mut u8 version_len; / PackageVersionStringLength,
//
// DSP0267 also includes the following variable length fields at the
// end of this structure:
//
// PackageVersionString, length is version_len.
//
// The total size of this section is
// sizeof(pldm_header) + version_len;
//
    pub /: *mut *mut u8 version_string[]; / PackageVersionString,
    pub __aligned(1): } __packed,
// Firmware Device ID Record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pldmfw_record_info {
    pub /: *mut *mut __le16 record_len; / RecordLength,
    pub /: *mut *mut u8 descriptor_count; / DescriptorCount,
    pub /: *mut *mut __le32 device_update_flags; / DeviceUpdateOptionFlags,
    pub /: *mut *mut u8 version_type; / ComponentImageSetVersionType,
    pub /: *mut *mut u8 version_len; / ComponentImageSetVersionLength,
    pub /: *mut *mut __le16 package_data_len; / FirmwareDevicePackageDataLength,
//
// DSP0267 also includes the following variable length fields at the
// end of this structure:
//
// ApplicableComponents, length is component_bitmap_len from header
// ComponentImageSetVersionString, length is version_len
// RecordDescriptors, a series of TLVs with 16bit type and length
// FirmwareDevicePackageData, length is package_data_len
//
// The total size of each record is
// sizeof(pldmfw_record_info) +
// component_bitmap_len (converted to bytes!) +
// version_len +
// <length of RecordDescriptors> +
// package_data_len
//
    pub variable_record_data: [u8; ],
    pub __aligned(1): } __packed,
// Firmware Descriptor Definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pldmfw_desc_tlv {
    pub /: *mut *mut __le16 type; / DescriptorType,
    pub /: *mut *mut __le16 size; / DescriptorSize,
    pub /: *mut *mut u8 data[]; / DescriptorData,
    pub __aligned(1): },
// Firmware Device Identification Area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pldmfw_record_area {
    pub /: *mut *mut u8 record_count; / DeviceIDRecordCount,
// This is not a struct type because the size of each record varies
    pub records: [u8; ],
    pub __aligned(1): },
// Individual Component Image Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pldmfw_component_info {
    pub /: *mut *mut __le16 classification; / ComponentClassfication,
    pub /: *mut *mut __le16 identifier; / ComponentIdentifier,
    pub /: *mut *mut __le32 comparison_stamp; / ComponentComparisonStamp,
    pub /: *mut *mut __le16 options; / componentOptions,
    pub /: *mut *mut __le16 activation_method; / RequestedComponentActivationMethod,
    pub /: *mut *mut __le32 location_offset; / ComponentLocationOffset,
    pub /: *mut *mut __le32 size; / ComponentSize,
    pub /: *mut *mut u8 version_type; / ComponentVersionStringType,
    pub /: *mut *mut u8 version_len; / ComponentVersionStringLength,
//
// DSP0267 also includes the following variable length fields at the
// end of this structure:
//
// ComponentVersionString, length is version_len
//
// The total size of this section is
// sizeof(pldmfw_component_info) + version_len;
//
    pub /: *mut *mut u8 version_string[]; / ComponentVersionString,
    pub __aligned(1): } __packed,
// Component Image Information Area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pldmfw_component_area {
    pub component_image_count: __le16,
// This is not a struct type because the component size varies
    pub components: [u8; ],
    pub __aligned(1): },
//
// pldm_first_desc_tlv
// @start: byte offset of the start of the descriptor TLVs
//
// Converts the starting offset of the descriptor TLVs into a pointer to the
// first descriptor.
//

//
// pldm_next_desc_tlv
// @desc: pointer to a descriptor TLV
//
// Finds the pointer to the next descriptor following a given descriptor
//

//
// pldm_for_each_desc_tlv
// @i: variable to store descriptor index
// @desc: variable to store descriptor pointer
// @start: byte offset of the start of the descriptors
// @count: the number of descriptors
//
// for loop macro to iterate over all of the descriptors of a given PLDM
// record.
//

    pub \: for ((i) = 0, (desc) = pldm_first_desc_tlv(start);,
    pub \: (i) < (count);,
//
// pldm_first_record
// @start: byte offset of the start of the PLDM records
//
// Converts a starting offset of the PLDM records into a pointer to the first
// record.
//

//
// pldm_next_record
// @record: pointer to a PLDM record
//
// Finds a pointer to the next record following a given record
//

//
// pldm_for_each_record
// @i: variable to store record index
// @record: variable to store record pointer
// @start: byte offset of the start of the records
// @count: the number of records
//
// for loop macro to iterate over all of the records of a PLDM file.
//

    pub \: for ((i) = 0, (record) = pldm_first_record(start);,
    pub \: (i) < (count);,
//
// pldm_first_component
// @start: byte offset of the start of the PLDM components
//
// Convert a starting offset of the PLDM components into a pointer to the
// first component
//

//
// pldm_next_component
// @component: pointer to a PLDM component
//
// Finds a pointer to the next component following a given component
//

//
// pldm_for_each_component
// @i: variable to store component index
// @component: variable to store component pointer
// @start: byte offset to the start of the first component
// @count: the number of components
//
// for loop macro to iterate over all of the components of a PLDM file.
//

    pub \: for ((i) = 0, (component) = pldm_first_component(start);,
    pub \: (i) < (count);,
