//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-priv.h
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
// Copyright (c) 2011-2012, The Linux Foundation. All rights reserved.
//

//
// Coresight management registers (0xf00-0xfcc)
// 0xfa0 - 0xfa4: Management	registers in PFTv1.0
// Trace		registers in PFTv1.1
//
pub const CORESIGHT_ITCTRL: c_uint = 0xf00;
pub const CORESIGHT_CLAIMSET: c_uint = 0xfa0;
pub const CORESIGHT_CLAIMCLR: c_uint = 0xfa4;
pub const CORESIGHT_LAR: c_uint = 0xfb0;
pub const CORESIGHT_LSR: c_uint = 0xfb4;
pub const CORESIGHT_DEVARCH: c_uint = 0xfbc;
pub const CORESIGHT_AUTHSTATUS: c_uint = 0xfb8;
pub const CORESIGHT_DEVID: c_uint = 0xfc8;
pub const CORESIGHT_DEVTYPE: c_uint = 0xfcc;
//
// Coresight device CLAIM protocol.
// See PSCI - ARM DEN 0022D, Section: 6.8.1 Debug and Trace save and restore.
//

pub const CORESIGHT_CLAIM_FREE: c_int = 0;
pub const CORESIGHT_CLAIM_EXTERNAL: c_int = 1;
pub const CORESIGHT_CLAIM_SELF_HOSTED: c_int = 2;
pub const CORESIGHT_CLAIM_INVALID: c_int = 3;
pub const TIMEOUT_US: c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_pair_attribute {
    pub attr: device_attribute,
    pub lo_off: u32,
    pub hi_off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_off_attribute {
    pub attr: device_attribute,
    pub off: u32,
}

extern "C" {
    pub fn coresight_simple_show32(_dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn coresight_simple_show_pair(_dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etm_addr_type {
    ETM_ADDR_TYPE_NONE,
    ETM_ADDR_TYPE_SINGLE,
    ETM_ADDR_TYPE_RANGE,
    ETM_ADDR_TYPE_START,
    ETM_ADDR_TYPE_STOP,
}

//
// struct cs_buffer - keep track of a recording session' specifics
// @cur:	index of the current buffer
// @nr_pages:	max number of pages granted to us
// @pid:	PID this cs_buffer belongs to
// @offset:	offset within the current buffer
// @data_size:	how much we collected in this run
// @snapshot:	is this run in snapshot mode
// @data_pages:	a handle the ring buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_buffers {
    pub cur: c_uint,
    pub nr_pages: c_uint,
    pub pid: pid_t,
    pub offset: c_ulong,
    pub data_size: local_t,
    pub snapshot: bool,
    pub data_pages: *mut c_void,
}

// Wait for things to settle
// Make sure everyone has seen this
extern "C" {
    pub fn coresight_disable_path(path: *mut coresight_path);
}
extern "C" {
    pub fn coresight_enable_path(path: *mut coresight_path, mode: cs_mode) -> c_int;
}
extern "C" {
    pub fn coresight_release_path(path: *mut coresight_path);
}
extern "C" {
    pub fn coresight_add_sysfs_link(info: *mut coresight_sysfs_link) -> c_int;
}
extern "C" {
    pub fn coresight_remove_sysfs_link(info: *mut coresight_sysfs_link);
}
extern "C" {
    pub fn coresight_create_conns_sysfs_group(csdev: *mut coresight_device) -> c_int;
}
extern "C" {
    pub fn coresight_remove_conns_sysfs_group(csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_get_sink_id(csdev: *mut coresight_device) -> u32;
}

extern "C" {
    pub fn etm_readl_cp14(off: u32, val: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn etm_writel_cp14(off: u32, val: u32) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cti_assoc_op {
    pub csdev): *mut *mut void (add)(struct coresight_device,
    pub csdev): *mut *mut void (remove)(struct coresight_device,
}

extern "C" {
    pub fn coresight_set_cti_ops(cti_op: *const cti_assoc_op);
}
extern "C" {
    pub fn coresight_remove_cti_ops();
}
//
// Macros and inline functions to handle CoreSight UCI data and driver
// private data in AMBA ID table entries, and extract data values.
//
// coresight AMBA ID, no UCI, no driver data: id table entry

// coresight AMBA ID, UCI with driver data only: id table entry.

// coresight AMBA ID, full UCI structure: id table entry.

//
// PIDR2[JEDEC], BIT(3) must be 1 (Read As One) to indicate that rest of the
// PIDR1, PIDR2 DES_* fields follow JEDEC encoding for the designer. Use that
// as a match value for blanket matching all devices in the given CoreSight
// device type and architecture.
//

//
// Match all PIDs in a given CoreSight device type and architecture, defined
// by the uci.
//

// extract the data value from a UCI structure given amba_id pointer.
extern "C" {
    pub fn coresight_get_uci_data(_arg: table) -> return;
}
extern "C" {
    pub fn coresight_set_percpu_sink(cpu: c_int, csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_put_percpu_source_ref(csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_disable_source(csdev: *mut coresight_device, data: *mut c_void);
}
extern "C" {
    pub fn coresight_pause_source(csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_resume_source(csdev: *mut coresight_device) -> c_int;
}
