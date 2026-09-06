//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/libie/fwlog.h
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
// Copyright (C) 2022, Intel Corporation.

// Only a single log level should be set and all log levels under the set value
// are enabled, e.g. if log level is set to LIBIE_FW_LOG_LEVEL_VERBOSE, then all
// other log levels are included (except LIBIE_FW_LOG_LEVEL_NONE)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libie_fwlog_level {
    LIBIE_FWLOG_LEVEL_NONE = 0,
    LIBIE_FWLOG_LEVEL_ERROR = 1,
    LIBIE_FWLOG_LEVEL_WARNING = 2,
    LIBIE_FWLOG_LEVEL_NORMAL = 3,
    LIBIE_FWLOG_LEVEL_VERBOSE = 4,
    LIBIE_FWLOG_LEVEL_INVALID, /* all values >= this entry are invalid */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_fwlog_module_entry {
// module ID for the corresponding firmware logging event
    pub module_id: u16,
// verbosity level for the module_id
    pub log_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_fwlog_cfg {
// list of modules for configuring log level
    pub module_entries: [libie_fwlog_module_entry; LIBIE_AQC_FW_LOG_ID_MAX],
// options used to configure firmware logging
    pub options: u16,

// set before calling libie_fwlog_init() so the PF registers for
// firmware logging on initialization
//

// set in the libie_aq_fwlog_get() response if the PF is registered for
// FW logging events over ARQ
//

// minimum number of log events sent per Admin Receive Queue event
    pub log_resolution: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_fwlog_data {
    pub data_size: u16,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_fwlog_ring {
    pub rings: *mut libie_fwlog_data,
    pub index: u16,
    pub size: u16,
    pub head: u16,
    pub tail: u16,
}

pub const LIBIE_FWLOG_RING_SIZE_INDEX_DFLT: c_int = 3;
pub const LIBIE_FWLOG_RING_SIZE_DFLT: c_int = 256;
pub const LIBIE_FWLOG_RING_SIZE_MAX: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_fwlog {
    pub cfg: libie_fwlog_cfg,
    pub /: *mut *mut bool supported; / does hardware support FW logging?,
    pub ring: libie_fwlog_ring,
    pub debugfs: *mut dentry,
// keep track of all the dentrys for FW log modules
    pub debugfs_modules: *mut dentry,
    pub pdev: *mut pci_dev,
    pub u16): *mut *mut *mut *mut *mut int (send_cmd)(void , struct libie_aq_desc , void ,,
    pub priv: *mut c_void,
    pub debugfs_root: *mut dentry,
}

extern "C" {
    pub fn libie_fwlog_init(fwlog: *mut libie_fwlog, api: *mut libie_fwlog_api) -> c_int;
}
extern "C" {
    pub fn libie_fwlog_deinit(fwlog: *mut libie_fwlog);
}
extern "C" {
    pub fn libie_fwlog_reregister(fwlog: *mut libie_fwlog);
}
extern "C" {
    pub fn libie_get_fwlog_data(fwlog: *mut libie_fwlog, buf: *mut u8, len: u16);
}

