//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/scsi_iu.h
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


//
// This file is in the public domain.
//
pub const _SCSI_SCSI_IU_H: c_int = 1;
pub const SIU_SNSVALID: c_uint = 0x2;
pub const SIU_RSPVALID: c_uint = 0x1;
pub const SIU_PKTFAIL_OFFSET(siu): c_int = 12;

pub const SIU_PFC_NONE: c_int = 0;
pub const SIU_PFC_CIU_FIELDS_INVALID: c_int = 2;
pub const SIU_PFC_TMF_NOT_SUPPORTED: c_int = 4;
pub const SIU_PFC_TMF_FAILED: c_int = 5;
pub const SIU_PFC_INVALID_TYPE_CODE: c_int = 6;
pub const SIU_PFC_ILLEGAL_REQUEST: c_int = 7;

pub const SIU_TASKMGMT_NONE: c_uint = 0x00;
pub const SIU_TASKMGMT_ABORT_TASK: c_uint = 0x01;
pub const SIU_TASKMGMT_ABORT_TASK_SET: c_uint = 0x02;
pub const SIU_TASKMGMT_CLEAR_TASK_SET: c_uint = 0x04;
pub const SIU_TASKMGMT_LUN_RESET: c_uint = 0x08;
pub const SIU_TASKMGMT_TARGET_RESET: c_uint = 0x20;
pub const SIU_TASKMGMT_CLEAR_ACA: c_uint = 0x40;
