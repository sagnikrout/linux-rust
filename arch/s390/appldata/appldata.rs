//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/appldata/appldata.h
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
// Definitions and interface for Linux - z/VM Monitor Stream.
//
// Copyright IBM Corp. 2003, 2008
//
// Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
//

// data buffer
pub const APPLDATA_MAX_PROCS: c_int = 100;

pub const APPLDATA_RECORD_MEM_ID: c_uint = 0x01	/* IDs to identify the */;
pub const APPLDATA_RECORD_OS_ID: c_uint = 0x02	/* individual records, */;
pub const APPLDATA_RECORD_NET_SUM_ID: c_uint = 0x03	/* must be < 256 !     */;
pub const APPLDATA_RECORD_PROC_ID: c_uint = 0x04;

pub const CTL_APPLDATA_INTERVAL: c_int = 2122;
pub const CTL_APPLDATA_MEM: c_int = 2123;
pub const CTL_APPLDATA_OS: c_int = 2124;
pub const CTL_APPLDATA_NET_SUM: c_int = 2125;
pub const CTL_APPLDATA_PROC: c_int = 2126;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appldata_ops {
    pub list: list_head,
    pub sysctl_header: *mut ctl_table_header,
    pub ctl_table: *mut ctl_table,
    pub /: *mut *mut int active; / monitoring status,
// fill in from here
    pub /: *mut *mut char name[APPLDATA_PROC_NAME_LENGTH]; / name of /proc fs node,
    pub /: *mut *mut unsigned char record_nr; / Record Nr. for Product ID,
    pub /: *mut *mut *mut *mut void (callback)(void data); / callback function,
    pub /: *mut *mut *mut void data; / record data,
    pub /: *mut *mut unsigned int size; / size of record,
    pub /: *mut *mut *mut module owner; / THIS_MODULE,
    pub /: *mut *mut char mod_lvl[2]; / modification level, EBCDIC,
}

extern "C" {
    pub fn appldata_register_ops(ops: *mut appldata_ops) -> c_int;
}
extern "C" {
    pub fn appldata_unregister_ops(ops: *mut appldata_ops);
}
