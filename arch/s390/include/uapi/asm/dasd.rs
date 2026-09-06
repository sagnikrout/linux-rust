//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/dasd.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 1999, 2000
// EMC Symmetrix ioctl Copyright EMC Corporation, 2008
// Author.........: Nigel Hislop <hislop_nigel@emc.com>
//
// This file is the interface of the DASD device driver, which is exported to user space
// any future changes wrt the API will result in a change of the APIVERSION reported
// to userspace by the DASDAPIVER-ioctl
//

pub const DASD_API_VERSION: c_int = 6;
//
// struct dasd_information2_t
// represents any data about the device, which is visible to userspace.
// including format and featueres.
//
// values to be used for dasd_information_t.format
// 0x00: NOT formatted
// 0x01: Linux disc layout
// 0x02: Common disc layout
//
pub const DASD_FORMAT_NONE: c_int = 0;
pub const DASD_FORMAT_LDL: c_int = 1;
pub const DASD_FORMAT_CDL: c_int = 2;
//
// values to be used for dasd_information_t.features
// 0x100: default features
// 0x001: readonly (ro)
// 0x002: use diag discipline (diag)
// 0x004: set the device initially online (internal use only)
// 0x008: enable ERP related logging
// 0x010: allow I/O to fail on lost paths
// 0x020: allow I/O to fail when a lock was stolen
// 0x040: give access to raw eckd data
// 0x080: enable discard support
// 0x100: enable autodisable for IFCC errors (default)
// 0x200: enable requeue of all requests on autoquiesce
//
pub const DASD_FEATURE_READONLY: c_uint = 0x001;
pub const DASD_FEATURE_USEDIAG: c_uint = 0x002;
pub const DASD_FEATURE_INITIAL_ONLINE: c_uint = 0x004;
pub const DASD_FEATURE_ERPLOG: c_uint = 0x008;
pub const DASD_FEATURE_FAILFAST: c_uint = 0x010;
pub const DASD_FEATURE_FAILONSLCK: c_uint = 0x020;
pub const DASD_FEATURE_USERAW: c_uint = 0x040;
pub const DASD_FEATURE_DISCARD: c_uint = 0x080;
pub const DASD_FEATURE_PATH_AUTODISABLE: c_uint = 0x100;
pub const DASD_FEATURE_REQUEUEQUIESCE: c_uint = 0x200;

pub const DASD_PARTN_BITS: c_int = 2;
//
// struct dasd_information_t
// represents any data about the data, which is visible to userspace
//
// Read Subsystem Data - Performance Statistics
//
// struct profile_info_t
// holds the profinling information
//
// struct format_data_t
// represents all data necessary to format a dasd
//
// struct dasd_copypair_swap_data_t
// represents all data necessary to issue a swap of the copy pair relation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_copypair_swap_data_t {
    pub /: *mut *mut char primary[20]; / BUSID of primary,
    pub /: *mut *mut char secondary[20]; / BUSID of secondary,
// Reserved for future updates.
    pub reserved: [__u8; 64],
}

//
// values to be used for format_data_t.intensity
// 0/8: normal format
// 1/9: also write record zero
// 3/11: also write home address
// 4/12: invalidate track
//

//
// struct format_check_t
// represents all data necessary to evaluate the format of
// different tracks of a dasd
//
// Input
// Output
// Values returned in format_check_t when a format error is detected:
// Too few records were found on a single track
pub const DASD_FMT_ERR_TOO_FEW_RECORDS: c_int = 1;
// Too many records were found on a single track
pub const DASD_FMT_ERR_TOO_MANY_RECORDS: c_int = 2;
// Blocksize/data-length of a record was wrong
pub const DASD_FMT_ERR_BLKSIZE: c_int = 3;
// A record ID is defined by cylinder, head, and record number (CHR).
// On mismatch, this error is set
pub const DASD_FMT_ERR_RECORD_ID: c_int = 4;
// If key-length was != 0
pub const DASD_FMT_ERR_KEY_LENGTH: c_int = 5;
//
// struct attrib_data_t
// represents the operation (cache) bits for the device.
// Used in DE to influence caching of the DASD.
//
// definition of operation (cache) bits within attributes of DE
pub const DASD_NORMAL_CACHE: c_uint = 0x0;
pub const DASD_BYPASS_CACHE: c_uint = 0x1;
pub const DASD_INHIBIT_LOAD: c_uint = 0x2;
pub const DASD_SEQ_ACCESS: c_uint = 0x3;
pub const DASD_SEQ_PRESTAGE: c_uint = 0x4;
pub const DASD_REC_ACCESS: c_uint = 0x5;
//
// Perform EMC Symmetrix I/O
//
// Data returned by Sense Path Group ID (SNID)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_snid_data {
    pub group:2: __u8,
    pub reserve:2: __u8,
    pub mode:1: __u8,
    pub res:3: __u8,
// C attribute field omitted
    pub pgid: [__u8; 11],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_snid_ioctl_data {
    pub data: dasd_snid_data,
    pub path_mask: __u8,
// C attribute field omitted
//
// SECTION: Definition of IOCTLs
//
// Here is how the ioctl-nr should be used:
// 0 -   31   DASD driver itself
// 32 -  239   still open
// 240 -  255	 reserved for EMC
//
// Disable the volume (for Linux)

// Enable the volume (for Linux)

// Issue a reserve/release command, rsp.

// reset profiling information of a device

// Quiesce IO on device

// Resume IO on device

// Abort all I/O on a device

// Allow I/O on a device

// retrieve API version number

// Get information on a dasd device

// retrieve profiling information of a device

// Get information on a dasd device (enhanced)

// Performance Statistics Read

// Get Attributes (cache operations)

// #define BIODASDFORMAT  _IOW(IOCTL_LETTER,0,format_data_t) , deprecated

// Set Attributes (cache operations)

// Release Allocated Space

// Swap copy pair relation

// Get Sense Path Group ID (SNID) data

// Check device format according to format_check_t

