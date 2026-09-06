//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-io.h
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
//
// Copyright (C) 2005 Mike Isely <isely@pobox.com>
//

extern "C" {
    pub fn void(: *mut *mut pvr2_stream_callback)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr2_buffer_state {
    pvr2_buffer_state_none = 0,   // Not on any list
    pvr2_buffer_state_idle = 1,   // Buffer is ready to be used again
    pvr2_buffer_state_queued = 2, // Buffer has been queued for filling
    pvr2_buffer_state_ready = 3,  // Buffer has data available
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_stream_stats {
    pub buffers_in_queue: c_uint,
    pub buffers_in_idle: c_uint,
    pub buffers_in_ready: c_uint,
    pub buffers_processed: c_uint,
    pub buffers_failed: c_uint,
    pub bytes_processed: c_uint,
}

// Initialize / tear down stream structure
extern "C" {
    pub fn pvr2_stream_destroy(: *mut pvr2_stream);
}
// Query / set the nominal buffer count
extern "C" {
    pub fn pvr2_stream_get_buffer_count(: *mut pvr2_stream) -> c_int;
}
extern "C" {
    pub fn pvr2_stream_set_buffer_count(: *mut pvr2_stream, int: unsigned) -> c_int;
}
// Get a pointer to a buffer that is either idle, ready, or is specified
// Find out how many buffers are idle or ready
extern "C" {
    pub fn pvr2_stream_get_ready_count(: *mut pvr2_stream) -> c_int;
}
// Kill all pending buffers and throw away any ready buffers as well
extern "C" {
    pub fn pvr2_stream_kill(: *mut pvr2_stream);
}
// Set up the actual storage for a buffer
extern "C" {
    pub fn pvr2_buffer_set_buffer(: *mut pvr2_buffer, ptr: *mut c_void, cnt: c_uint) -> c_int;
}
// Find out size of data in the given ready buffer
extern "C" {
    pub fn pvr2_buffer_get_count(: *mut pvr2_buffer) -> c_uint;
}
// Retrieve completion code for given ready buffer
extern "C" {
    pub fn pvr2_buffer_get_status(: *mut pvr2_buffer) -> c_int;
}
// Retrieve ID of given buffer
extern "C" {
    pub fn pvr2_buffer_get_id(: *mut pvr2_buffer) -> c_int;
}
// Start reading into given buffer (kill it if needed)
extern "C" {
    pub fn pvr2_buffer_queue(: *mut pvr2_buffer) -> c_int;
}
