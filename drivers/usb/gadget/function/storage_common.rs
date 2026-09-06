//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/storage_common.h
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

// const u8 * */ buf, /* unsigned */ length)		\

// const u8 * */ buf, /* unsigned */ length) do { } while (0)

// Length of a SCSI Command Data Block
pub const MAX_COMMAND_SIZE: c_int = 16;
// SCSI Sense Key/Additional Sense Code/ASC Qualifier values
pub const SS_NO_SENSE: c_int = 0;
pub const SS_COMMUNICATION_FAILURE: c_uint = 0x040800;
pub const SS_INVALID_COMMAND: c_uint = 0x052000;
pub const SS_INVALID_FIELD_IN_CDB: c_uint = 0x052400;
pub const SS_LOGICAL_BLOCK_ADDRESS_OUT_OF_RANGE: c_uint = 0x052100;
pub const SS_LOGICAL_UNIT_NOT_SUPPORTED: c_uint = 0x052500;
pub const SS_MEDIUM_NOT_PRESENT: c_uint = 0x023a00;
pub const SS_MEDIUM_REMOVAL_PREVENTED: c_uint = 0x055302;
pub const SS_NOT_READY_TO_READY_TRANSITION: c_uint = 0x062800;
pub const SS_RESET_OCCURRED: c_uint = 0x062900;
pub const SS_SAVING_PARAMETERS_NOT_SUPPORTED: c_uint = 0x053900;
pub const SS_UNRECOVERED_READ_ERROR: c_uint = 0x031100;
pub const SS_WRITE_ERROR: c_uint = 0x030c02;
pub const SS_WRITE_PROTECTED: c_uint = 0x072700;

//
// Vendor (8 chars), product (16 chars), release (4 hexadecimal digits) and NUL
// byte
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsg_lun {
    pub filp: *mut file,
    pub file_length: loff_t,
    pub num_sectors: loff_t,
    pub initially_ro:1: c_uint,
    pub ro:1: c_uint,
    pub removable:1: c_uint,
    pub cdrom:1: c_uint,
    pub prevent_medium_removal:1: c_uint,
    pub registered:1: c_uint,
    pub info_valid:1: c_uint,
    pub nofua:1: c_uint,
    pub sense_data: u32,
    pub sense_data_info: u32,
    pub unit_attention_data: u32,
    pub size: *mut *mut unsigned int blkbits; / Bits of logical block,
    pub /: *mut *mut unsigned int blksize; / logical block size of bound block device,
    pub dev: device,
    pub /: *const *const *const char name; / "lun.name",
    pub /: *const *const *const *const char name_pfx; / "function.name",
    pub inquiry_string: [c_char; INQUIRY_STRING_LEN],
}

// Default size of buffer length.

// Maximal number of LUNs supported in mass storage function

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsg_buffer_state {
    BUF_STATE_SENDING = -2,
    BUF_STATE_RECEIVING,
    BUF_STATE_EMPTY = 0,
    BUF_STATE_FULL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsg_buffhd {
    pub buf: *mut c_void,
    pub state: fsg_buffer_state,
    pub next: *mut fsg_buffhd,
//
// The NetChip 2280 is faster, and handles some protocol faults
// better, if we don't submit any short bulk-out read requests.
// So we will record the intended request length here.
//
    pub bulk_out_intended_length: c_uint,
    pub inreq: *mut usb_request,
    pub outreq: *mut usb_request,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsg_state {
    FSG_STATE_NORMAL,
    FSG_STATE_ABORT_BULK_OUT,
    FSG_STATE_PROTOCOL_RESET,
    FSG_STATE_CONFIG_CHANGE,
    FSG_STATE_EXIT,
    FSG_STATE_TERMINATED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_direction {
    DATA_DIR_UNKNOWN = 0,
    DATA_DIR_FROM_HOST,
    DATA_DIR_TO_HOST,
    DATA_DIR_NONE
}

extern "C" {
    pub fn container_of(_arg: dev, fsg_lun: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn fsg_lun_close(curlun: *mut fsg_lun);
}
extern "C" {
    pub fn fsg_lun_open(curlun: *mut fsg_lun, filename: *const c_char) -> c_int;
}
extern "C" {
    pub fn fsg_lun_fsync_sub(curlun: *mut fsg_lun) -> c_int;
}
extern "C" {
    pub fn store_cdrom_address(dest: *mut u8, msf: c_int, addr: u32);
}
extern "C" {
    pub fn fsg_show_ro(curlun: *mut fsg_lun, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn fsg_show_nofua(curlun: *mut fsg_lun, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn fsg_show_inquiry_string(curlun: *mut fsg_lun, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn fsg_show_cdrom(curlun: *mut fsg_lun, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn fsg_show_removable(curlun: *mut fsg_lun, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn fsg_store_nofua(curlun: *mut fsg_lun, buf: *const c_char, count: usize) -> isize;
}
