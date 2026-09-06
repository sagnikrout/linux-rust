//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/cmb.h
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
// struct cmbdata - channel measurement block data for user space
// @size: size of the stored data
// @elapsed_time: time since last sampling
// @ssch_rsch_count: number of ssch and rsch
// @sample_count: number of samples
// @device_connect_time: time of device connect
// @function_pending_time: time of function pending
// @device_disconnect_time: time of device disconnect
// @control_unit_queuing_time: time of control unit queuing
// @device_active_only_time: time of device active only
// @device_busy_time: time of device busy (ext. format)
// @initial_command_response_time: initial command response time (ext. format)
//
// All values are stored as 64 bit for simplicity, especially
// in 32 bit emulation mode. All time values are normalized to
// nanoseconds.
// Currently, two formats are known, which differ by the size of
// this structure, i.e. the last two members are only set when
// the extended channel measurement facility (first shipped in
// z990 machines) is activated.
// Potentially, more fields could be added, which would result in a
// new ioctl number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmbdata {
    pub size: __u64,
    pub elapsed_time: __u64,
// basic and extended format:
    pub ssch_rsch_count: __u64,
    pub sample_count: __u64,
    pub device_connect_time: __u64,
    pub function_pending_time: __u64,
    pub device_disconnect_time: __u64,
    pub control_unit_queuing_time: __u64,
    pub device_active_only_time: __u64,
// extended format only:
    pub device_busy_time: __u64,
    pub initial_command_response_time: __u64,
}

// enable channel measurement

// enable channel measurement

// read channel measurement data

