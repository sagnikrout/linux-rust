//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_common.h
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
// Functions used by both the SCSI initiator code and the SCSI target code.
//

// From the standard INQUIRY data description in SPC-6.
pub const INQUIRY_VENDOR_OFFSET: c_int = 8;
pub const INQUIRY_VENDOR_LEN: c_int = 8;
pub const INQUIRY_MODEL_OFFSET: c_int = 16;
pub const INQUIRY_MODEL_LEN: c_int = 16;
pub const INQUIRY_REVISION_OFFSET: c_int = 32;
pub const INQUIRY_REVISION_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_pr_type {
    SCSI_PR_WRITE_EXCLUSIVE			= 0x01,
    SCSI_PR_EXCLUSIVE_ACCESS		= 0x03,
    SCSI_PR_WRITE_EXCLUSIVE_REG_ONLY	= 0x05,
    SCSI_PR_EXCLUSIVE_ACCESS_REG_ONLY	= 0x06,
    SCSI_PR_WRITE_EXCLUSIVE_ALL_REGS	= 0x07,
    SCSI_PR_EXCLUSIVE_ACCESS_ALL_REGS	= 0x08,
}

extern "C" {
    pub fn block_pr_type_to_scsi(type: pr_type) -> scsi_pr_type;
}
extern "C" {
    pub fn scsi_pr_type_to_block(type: scsi_pr_type) -> pr_type;
}

// Returns a human-readable name for the device
extern "C" {
    pub fn int_to_scsilun(_arg: u64, : *mut scsi_lun);
}
extern "C" {
    pub fn scsilun_to_int(: *mut scsi_lun) -> u64;
}
//
// This is a slightly modified SCSI sense "descriptor" format header.
// The addition is to allow the 0x70 and 0x71 response codes. The idea
// is to place the salient data from either "fixed" or "descriptor" sense
// format into one structure to ease application processing.
//
// The original sense buffer should be kept around for those cases
// in which more information is required (e.g. the LBA of a MEDIUM ERROR).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_sense_hdr {
    pub /: *mut *mut u8 response_code; / permit: 0x0, 0x70, 0x71, 0x72, 0x73,
    pub sense_key: u8,
    pub asc: u8,
    pub ascq: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub /: *mut *mut u8 additional_length; / always 0 for fixed sense format,
}

extern "C" {
    pub fn scsi_build_sense_buffer(desc: c_int, buf: *mut u8, key: u8, asc: u8, ascq: u8);
}
extern "C" {
    pub fn scsi_set_sense_information(buf: *mut u8, buf_len: c_int, info: u64) -> c_int;
}
extern "C" {
    pub fn scsi_set_sense_field_pointer(buf: *mut u8, buf_len: c_int, fp: u16, bp: u8, cd: bool) -> c_int;
}
