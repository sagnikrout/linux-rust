//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tsm-mr.h
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
// struct tsm_measurement_register - describes an architectural measurement
// register (MR)
// @mr_name: name of the MR
// @mr_value: buffer containing the current value of the MR
// @mr_size: size of the MR - typically the digest size of @mr_hash
// @mr_flags: bitwise OR of one or more flags, detailed below
// @mr_hash: optional hash identifier defined in include/uapi/linux/hash_info.h.
//
// A CC guest driver encloses an array of this structure in struct
// tsm_measurements to detail the measurement facility supported by the
// underlying CC hardware.
//
// @mr_name and @mr_value must stay valid until this structure is no longer in
// use.
//
// @mr_flags is the bitwise-OR of zero or more of the flags below.
//
// * %TSM_MR_F_READABLE - the sysfs attribute corresponding to this MR is readable.
// * %TSM_MR_F_WRITABLE - the sysfs attribute corresponding to this MR is writable.
// The semantics is typically to extend the MR but could vary depending on the
// architecture and the MR.
// * %TSM_MR_F_LIVE - this MR's value may differ from the last value written, so
// must be read back from the underlying CC hardware/firmware.
// * %TSM_MR_F_RTMR - bitwise-OR of %TSM_MR_F_LIVE and %TSM_MR_F_WRITABLE.
// * %TSM_MR_F_NOHASH - this MR does NOT have an associated hash algorithm.
// @mr_hash will be ignored when this flag is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_measurement_register {
    pub mr_name: *const c_char,
    pub mr_value: *mut c_void,
    pub mr_size: u32,
    pub mr_flags: u32,
    pub mr_hash: hash_algo,
}

pub const TSM_MR_F_NOHASH: c_int = 1;
pub const TSM_MR_F_WRITABLE: c_int = 2;
pub const TSM_MR_F_READABLE: c_int = 4;
pub const TSM_MR_F_LIVE: c_int = 8;

//
// struct tsm_measurements - defines the CC architecture specific measurement
// facility and methods for updating measurement registers (MRs)
// @mrs: Array of MR definitions.
// @nr_mrs: Number of elements in @mrs.
// @refresh: Callback function to load/sync all MRs from TVM hardware/firmware
// into the kernel cache.
// @write: Callback function to write to the MR specified by the parameter @mr.
// Typically, writing to an MR extends the input buffer to that MR.
//
// The @refresh callback is invoked when an MR with %TSM_MR_F_LIVE set is being
// read and the cache is stale. It must reload all MRs with %TSM_MR_F_LIVE set.
// The function parameter @tm is a pointer pointing back to this structure.
//
// The @write callback is invoked whenever an MR is being written. It takes two
// additional parameters besides @tm:
//
// * @mr - points to the MR (an element of @tm->mrs) being written.
// * @data - contains the bytes to write and whose size is @mr->mr_size.
//
// Both @refresh and @write should return 0 on success and an appropriate error
// code on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_measurements {
    pub mrs: *const tsm_measurement_register,
    pub nr_mrs: usize,
    pub tm): *const *const int (refresh)(struct tsm_measurements,
    pub data): *const *const tsm_measurement_register mr, u8,
}

extern "C" {
    pub fn tsm_mr_free_attribute_group(attr_grp: *const attribute_group);
}
