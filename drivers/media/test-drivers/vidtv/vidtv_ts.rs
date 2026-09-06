//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_ts.h
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
// The Virtual DVB test driver serves as a reference DVB driver and helps
// validate the existing APIs in the media subsystem. It can also aid
// developers working on userspace applications.
//
// Copyright (C) 2020 Daniel W. S. Almeida
//

pub const TS_SYNC_BYTE: c_uint = 0x47;
pub const TS_PACKET_LEN: c_int = 188;
pub const TS_PAYLOAD_LEN: c_int = 184;
pub const TS_NULL_PACKET_PID: c_uint = 0x1fff;
pub const TS_CC_MAX_VAL: c_uint = 0x0f /* 4 bits */;
pub const TS_LAST_VALID_PID: c_int = 8191;
pub const TS_FILL_BYTE: c_uint = 0xff /* the byte used in packet stuffing */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mpeg_ts_adaption {
    pub length: u8,
    pub extension:1: u8,
    pub private_data:1: u8,
    pub splicing_point:1: u8,
    pub OPCR:1: u8,
    pub PCR:1: u8,
    pub priority:1: u8,
    pub random_access:1: u8,
    pub discontinued:1: u8,
    pub __packed: },
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mpeg_ts {
    pub sync_byte: u8,
    pub /: *mut *mut __be16 bitfield; / tei: 1, payload_start:1 priority: 1, pid:13,
    pub continuity_counter:4: u8,
    pub payload:1: u8,
    pub adaptation_field:1: u8,
    pub scrambling:2: u8,
    pub __packed: },
    pub __packed: },
//
// struct pcr_write_args - Arguments for the pcr_write_into function.
// @dest_buf: The buffer to write into.
// @dest_offset: The byte offset into the buffer.
// @pid: The TS PID for the PCR packets.
// @buf_sz: The size of the buffer in bytes.
// @continuity_counter: The TS continuity_counter.
// @pcr: A sample from the system clock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcr_write_args {
    pub dest_buf: *mut c_void,
    pub dest_offset: u32,
    pub pid: u16,
    pub buf_sz: u32,
    pub continuity_counter: *mut u8,
    pub pcr: u64,
}

//
// struct null_packet_write_args - Arguments for the null_write_into function
// @dest_buf: The buffer to write into.
// @dest_offset: The byte offset into the buffer.
// @buf_sz: The size of the buffer in bytes.
// @continuity_counter: The TS continuity_counter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct null_packet_write_args {
    pub dest_buf: *mut c_void,
    pub dest_offset: u32,
    pub buf_sz: u32,
    pub continuity_counter: *mut u8,
}

// Increment the continuity counter
extern "C" {
    pub fn vidtv_ts_inc_cc(continuity_counter: *mut u8);
}
//
// vidtv_ts_null_write_into - Write a TS null packet into a buffer.
// @args: the arguments to use when writing.
//
// This function will write a null packet into a buffer. This is usually used to
// pad TS streams.
//
// Return: The number of bytes written into the buffer.
//
extern "C" {
    pub fn vidtv_ts_null_write_into(args: *const null_packet_write_args) -> u32;
}
//
// vidtv_ts_pcr_write_into - Write a PCR  packet into a buffer.
// @args: the arguments to use when writing.
//
// This function will write a PCR packet into a buffer. This is used to
// synchronize the clocks between encoders and decoders.
//
// Return: The number of bytes written into the buffer.
//
extern "C" {
    pub fn vidtv_ts_pcr_write_into(args: *const pcr_write_args) -> u32;
}
