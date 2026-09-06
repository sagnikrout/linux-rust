//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_pr.h
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
// PERSISTENT_RESERVE_OUT service action codes
//
// spc5r04b section 6.15.2 Table 174
//
pub const PRO_REGISTER: c_uint = 0x00;
pub const PRO_RESERVE: c_uint = 0x01;
pub const PRO_RELEASE: c_uint = 0x02;
pub const PRO_CLEAR: c_uint = 0x03;
pub const PRO_PREEMPT: c_uint = 0x04;
pub const PRO_PREEMPT_AND_ABORT: c_uint = 0x05;
pub const PRO_REGISTER_AND_IGNORE_EXISTING_KEY: c_uint = 0x06;
pub const PRO_REGISTER_AND_MOVE: c_uint = 0x07;
pub const PRO_REPLACE_LOST_RESERVATION: c_uint = 0x08;
//
// PERSISTENT_RESERVE_IN service action codes
//
// spc5r04b section 6.14.1 Table 162
//
pub const PRI_READ_KEYS: c_uint = 0x00;
pub const PRI_READ_RESERVATION: c_uint = 0x01;
pub const PRI_REPORT_CAPABILITIES: c_uint = 0x02;
pub const PRI_READ_FULL_STATUS: c_uint = 0x03;
//
// PERSISTENT_RESERVE_ SCOPE field
//
// spc5r04b section 6.14.3.2 Table 166
//
pub const PR_SCOPE_LU_SCOPE: c_uint = 0x00;
//
// PERSISTENT_RESERVE_* TYPE field
//
// spc5r04b section 6.14.3.3 Table 167
//
pub const PR_TYPE_WRITE_EXCLUSIVE: c_uint = 0x01;
pub const PR_TYPE_EXCLUSIVE_ACCESS: c_uint = 0x03;
pub const PR_TYPE_WRITE_EXCLUSIVE_REGONLY: c_uint = 0x05;
pub const PR_TYPE_EXCLUSIVE_ACCESS_REGONLY: c_uint = 0x06;
pub const PR_TYPE_WRITE_EXCLUSIVE_ALLREG: c_uint = 0x07;
pub const PR_TYPE_EXCLUSIVE_ACCESS_ALLREG: c_uint = 0x08;
pub const PR_APTPL_MAX_IPORT_LEN: c_int = 256;
pub const PR_APTPL_MAX_TPORT_LEN: c_int = 256;
//
// Function defined in target_core_spc.c
//
extern "C" {
    pub fn spc_gen_naa_6h_vendor_specific(: *mut se_device, : *mut c_uchar);
}
extern "C" {
    pub fn target_release_reservation(dev: *mut se_device);
}
extern "C" {
    pub fn target_scsi2_reservation_release(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_scsi2_reservation_reserve(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn core_scsi3_free_all_registrations(: *mut se_device);
}
extern "C" {
    pub fn target_scsi3_emulate_pr_in(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_scsi3_emulate_pr_out(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_check_reservation(: *mut se_cmd) -> sense_reason_t;
}
