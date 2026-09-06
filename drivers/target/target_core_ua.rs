//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_ua.h
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
// From spc4r17, Table D.1: ASC and ASCQ Assignement
//
pub const ASCQ_29H_POWER_ON_RESET_OR_BUS_DEVICE_RESET_OCCURED: c_uint = 0x00;
pub const ASCQ_29H_POWER_ON_OCCURRED: c_uint = 0x01;
pub const ASCQ_29H_SCSI_BUS_RESET_OCCURED: c_uint = 0x02;
pub const ASCQ_29H_BUS_DEVICE_RESET_FUNCTION_OCCURRED: c_uint = 0x03;
pub const ASCQ_29H_DEVICE_INTERNAL_RESET: c_uint = 0x04;
pub const ASCQ_29H_TRANSCEIVER_MODE_CHANGED_TO_SINGLE_ENDED: c_uint = 0x05;
pub const ASCQ_29H_TRANSCEIVER_MODE_CHANGED_TO_LVD: c_uint = 0x06;
pub const ASCQ_29H_NEXUS_LOSS_OCCURRED: c_uint = 0x07;
pub const ASCQ_2AH_PARAMETERS_CHANGED: c_uint = 0x00;
pub const ASCQ_2AH_MODE_PARAMETERS_CHANGED: c_uint = 0x01;
pub const ASCQ_2AH_LOG_PARAMETERS_CHANGED: c_uint = 0x02;
pub const ASCQ_2AH_RESERVATIONS_PREEMPTED: c_uint = 0x03;
pub const ASCQ_2AH_RESERVATIONS_RELEASED: c_uint = 0x04;
pub const ASCQ_2AH_REGISTRATIONS_PREEMPTED: c_uint = 0x05;
pub const ASCQ_2AH_ASYMMETRIC_ACCESS_STATE_CHANGED: c_uint = 0x06;
pub const ASCQ_2AH_IMPLICIT_ASYMMETRIC_ACCESS_STATE_TRANSITION_FAILED: c_uint = 0x07;
pub const ASCQ_2AH_PRIORITY_CHANGED: c_uint = 0x08;
pub const ASCQ_2CH_PREVIOUS_RESERVATION_CONFLICT_STATUS: c_uint = 0x09;
pub const ASCQ_3FH_INQUIRY_DATA_HAS_CHANGED: c_uint = 0x03;
pub const ASCQ_3FH_REPORTED_LUNS_DATA_HAS_CHANGED: c_uint = 0x0E;
extern "C" {
    pub fn target_scsi3_ua_check(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn core_scsi3_ua_allocate(: *mut se_dev_entry, _arg: u8, _arg: u8) -> c_int;
}
extern "C" {
    pub fn target_ua_allocate_lun(: *mut se_node_acl, _arg: u32, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn core_scsi3_ua_release_all(: *mut se_dev_entry);
}
