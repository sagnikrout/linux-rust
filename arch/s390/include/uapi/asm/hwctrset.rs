//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/hwctrset.h
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
// Copyright IBM Corp. 2021
// Interface implementation for communication with the CPU Measurement
// counter facility device driver.
//
// Author(s): Thomas Richter <tmricht@linux.ibm.com>
//
// Define for ioctl() commands to communicate with the CPU Measurement
// counter facility device driver.
//

pub const S390_HWCTR_START_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_ctrset_start {
    pub /: *mut *mut __u64 version; / Version of interface,
    pub /: *mut *mut __u64 data_bytes; / # of bytes required,
    pub /: *mut *mut __u64 cpumask_len; / Length of CPU mask in bytes,
    pub /: *mut *mut *mut __u64 cpumask; / Pointer to CPU mask,
    pub /: *mut *mut __u64 counter_sets; / Bit mask of counter sets to get,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_ctrset_setdata {
    pub /: *mut *mut __u32 set; / Counter set number,
    pub /: *mut *mut __u32 no_cnts; / # of counters stored in cv[],
    pub /: *mut *mut __u64 cv[]; / Counter values (variable length),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_ctrset_cpudata {
    pub /: *mut *mut __u32 cpu_nr; / CPU number,
    pub /: *mut *mut __u32 no_sets; / # of counters sets in data[],
    pub data: [s390_ctrset_setdata; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_ctrset_read {
    pub /: *mut *mut __u64 no_cpus; / Total # of CPUs data taken from,
    pub data: [s390_ctrset_cpudata; ],
}

