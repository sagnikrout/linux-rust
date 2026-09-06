//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/snic_trc.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.

// Global Data structure for trace to manage trace functionality
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_trc_data {
    pub /: *mut *mut u64 ts; / Time Stamp,
    pub /: *mut *mut *mut char fn; / Ptr to Function Name,
    pub /: *mut *mut u32 hno; / SCSI Host ID,
    pub /: *mut *mut u32 tag; / Command Tag,
    pub data: [u64; 5],
    pub __attribute__((__packed__)): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_trc {
    pub lock: spinlock_t,
    pub /: *mut *mut *mut snic_trc_data buf; / Trace Buffer,
    pub /: *mut *mut u32 max_idx; / Max Index into trace buffer,
    pub rd_idx: u32,
    pub wr_idx: u32,
    pub /: *mut *mut bool enable; / Control Variable for Tracing,
}

extern "C" {
    pub fn snic_trc_init() -> c_int;
}
extern "C" {
    pub fn snic_trc_free();
}
extern "C" {
    pub fn snic_trc_debugfs_init();
}
extern "C" {
    pub fn snic_trc_debugfs_term();
}
extern "C" {
    pub fn snic_get_trc_data(buf: *mut c_char, buf_sz: c_int) -> c_int;
}
extern "C" {
    pub fn snic_debugfs_init();
}
extern "C" {
    pub fn snic_debugfs_term();
}

