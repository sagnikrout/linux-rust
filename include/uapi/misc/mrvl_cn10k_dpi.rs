//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/misc/mrvl_cn10k_dpi.h
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
// Marvell Octeon CN10K DPI driver
//
// Copyright (C) 2024 Marvell.
//

pub const DPI_MAX_ENGINES: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpi_mps_mrrs_cfg {
    pub /: *mut *mut __u16 max_read_req_sz; / Max read request size,
    pub /: *mut *mut __u16 max_payload_sz; / Max payload size,
    pub /: *mut *mut __u16 port; / Ebus port,
    pub /: *mut *mut __u16 reserved; / Reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpi_engine_cfg {
    pub /: *mut *mut __u64 fifo_mask; / FIFO size mask in KBytes,
    pub /: *mut *mut __u16 molr[DPI_MAX_ENGINES]; / Max outstanding load requests,
    pub /: *mut *mut __u16 update_molr; / '1' to update engine MOLR,
    pub /: *mut *mut __u16 reserved; / Reserved,
}

// DPI ioctl numbers
pub const DPI_MAGIC_NUM: c_uint = 0xB8;
// Set MPS & MRRS parameters

// Set Engine FIFO configuration

