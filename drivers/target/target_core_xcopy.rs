//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_xcopy.h
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

pub const XCOPY_HDR_LEN: c_int = 16;
pub const XCOPY_TARGET_DESC_LEN: c_int = 32;
pub const XCOPY_SEGMENT_DESC_LEN: c_int = 28;
pub const XCOPY_NAA_IEEE_REGEX_LEN: c_int = 16;

//
// SPC4r37 6.4.6.1
// Table 150 — CSCD descriptor ID values
//
pub const XCOPY_CSCD_DESC_ID_LIST_OFF_MAX: c_uint = 0x07FF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xcopy_origin_list {
    XCOL_SOURCE_RECV_OP = 0x01,
    XCOL_DEST_RECV_OP = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcopy_op {
    pub op_origin: c_int,
    pub xop_se_cmd: *mut se_cmd,
    pub src_dev: *mut se_device,
    pub src_tid_wwn: [c_uchar; XCOPY_NAA_IEEE_REGEX_LEN],
    pub dst_dev: *mut se_device,
    pub dst_tid_wwn: [c_uchar; XCOPY_NAA_IEEE_REGEX_LEN],
    pub local_dev_wwn: [c_uchar; XCOPY_NAA_IEEE_REGEX_LEN],
    pub remote_lun_ref: *mut percpu_ref,
    pub src_lba: sector_t,
    pub dst_lba: sector_t,
    pub stdi: c_ushort,
    pub dtdi: c_ushort,
    pub nolb: c_ushort,
    pub xop_data_bytes: u32,
    pub xop_data_nents: u32,
    pub xop_data_sg: *mut scatterlist,
    pub xop_work: work_struct,
}

//
// Receive Copy Results Sevice Actions
//
pub const RCR_SA_COPY_STATUS: c_uint = 0x00;
pub const RCR_SA_RECEIVE_DATA: c_uint = 0x01;
pub const RCR_SA_OPERATING_PARAMETERS: c_uint = 0x03;
pub const RCR_SA_FAILED_SEGMENT_DETAILS: c_uint = 0x04;
//
// Receive Copy Results defs for Operating Parameters
//
pub const RCR_OP_MAX_TARGET_DESC_COUNT: c_uint = 0x2;
pub const RCR_OP_MAX_SG_DESC_COUNT: c_uint = 0x1;
pub const RCR_OP_MAX_DESC_LIST_LEN: c_int = 1024;

pub const RCR_OP_TOTAL_CONCURR_COPIES: c_uint = 0x1 /* Must be <= 16384 */;
pub const RCR_OP_MAX_CONCURR_COPIES: c_uint = 0x1 /* Must be <= 255 */;

extern "C" {
    pub fn target_xcopy_setup_pt() -> c_int;
}
extern "C" {
    pub fn target_xcopy_release_pt();
}
extern "C" {
    pub fn target_do_xcopy(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_do_receive_copy_results(: *mut se_cmd) -> sense_reason_t;
}
