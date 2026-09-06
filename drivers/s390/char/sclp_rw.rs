//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/sclp_rw.h
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
// interface to the SCLP-read/write driver
//
// Copyright IBM Corporation 1999, 2009
//
// Author(s): Martin Peschke <mpeschke@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mto {
    pub length: u16,
    pub type: u16,
    pub line_type_flags: u16,
    pub alarm_control: u8,
    pub _reserved: [u8; 3],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct go {
    pub length: u16,
    pub type: u16,
    pub domid: u32,
    pub hhmmss_time: [u8; 8],
    pub th_time: [u8; 3],
    pub reserved_0: u8,
    pub dddyyyy_date: [u8; 7],
    pub _reserved_1: u8,
    pub general_msg_flags: u16,
    pub _reserved_2: [u8; 10],
    pub originating_system_name: [u8; 8],
    pub job_guest_name: [u8; 8],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdb_header {
    pub length: u16,
    pub type: u16,
    pub tag: u32,
    pub revision_code: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdb {
    pub header: mdb_header,
    pub go: go,
    pub mto: mto,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_buf {
    pub header: evbuf_header,
    pub mdb: mdb,
    pub __attribute__((packed)): },
// The number of empty mto buffers that can be contained in a single sccb.

//
// data structure for information about list of SCCBs (only for writing),
// will be located at the end of a SCCBs page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_buffer {
    pub /: *mut *mut list_head list; / list_head for sccb_info chain,
    pub request: sclp_req,
    pub sccb: *mut c_void,
    pub current_msg: *mut msg_buf,
    pub current_line: *mut c_char,
    pub current_length: c_int,
    pub retry_count: c_int,
// output format settings
    pub columns: c_ushort,
    pub htab: c_ushort,
// statistics about this buffer
    pub /: *mut *mut unsigned int char_sum; / # chars in sccb,
    pub /: *mut *mut unsigned int messages; / # messages in sccb,
// Callback that is called after reaching final status.
    pub int): *mut *mut *mut void (callback)(struct sclp_buffer ,,
}

extern "C" {
    pub fn sclp_rw_init() -> c_int;
}
extern "C" {
    pub fn sclp_buffer_space(: *mut sclp_buffer) -> c_int;
}
extern "C" {
    pub fn sclp_write(buffer: *mut sclp_buffer, : *const c_uchar, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn sclp_emit_buffer(: *mut sclp_buffer, : *mut *mut void ()(struct sclp_buffer, _arg: int)) -> c_int;
}
extern "C" {
    pub fn sclp_chars_in_buffer(: *mut sclp_buffer) -> c_uint;
}
