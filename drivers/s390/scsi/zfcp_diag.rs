//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_diag.h
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
// zfcp device driver
//
// Definitions for handling diagnostics in the zfcp device driver.
//
// Copyright IBM Corp. 2018, 2020
//

//
// struct zfcp_diag_header - general part of a diagnostic buffer.
// @access_lock: lock protecting all the data in this buffer.
// @updating: flag showing that an update for this buffer is currently running.
// @incomplete: flag showing that the data in @buffer is incomplete.
// @timestamp: time in jiffies when the data of this buffer was last captured.
// @buffer: implementation-depending data of this buffer
// @buffer_size: size of @buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_diag_header {
    pub access_lock: spinlock_t,
// Flags
    pub :1: u64 updating,
    pub :1: u64 incomplete,
    pub timestamp: c_ulong,
    pub buffer: *mut c_void,
    pub buffer_size: usize,
}

//
// struct zfcp_diag_adapter - central storage for all diagnostics concerning an
// adapter.
// @max_age: maximum age of data in diagnostic buffers before they need to be
// refreshed (in ms).
// @port_data: data retrieved using exchange port data.
// @port_data.header: header with metadata for the cache in @port_data.data.
// @port_data.data: cached QTCB Bottom of command exchange port data.
// @config_data: data retrieved using exchange config data.
// @config_data.header: header with metadata for the cache in @config_data.data.
// @config_data.data: cached QTCB Bottom of command exchange config data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_diag_adapter {
    pub max_age: c_ulong,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_diag_adapter_port_data {
    pub header: zfcp_diag_header,
    pub data: fsf_qtcb_bottom_port,
    pub port_data: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_diag_adapter_config_data {
    pub header: zfcp_diag_header,
    pub data: fsf_qtcb_bottom_config,
    pub config_data: },
}

extern "C" {
    pub fn zfcp_diag_adapter_setup(adapter: *const *const zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_diag_adapter_free(adapter: *const *const zfcp_adapter);
}
//
// Function-Type used in zfcp_diag_update_buffer_limited() for the function
// that does the buffer-implementation dependent work.
//
extern "C" {
    pub fn int(adapter: *const *const *const zfcp_diag_update_buffer_func)(struct zfcp_adapter) -> typedef;
}
extern "C" {
    pub fn zfcp_diag_update_config_data_buffer(adapter: *const *const zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_diag_update_port_data_buffer(adapter: *const *const zfcp_adapter) -> c_int;
}
//
// zfcp_diag_support_sfp() - Return %true if the @adapter supports reporting
// SFP Data.
// @adapter: adapter to test the availability of SFP Data reporting for.
//
