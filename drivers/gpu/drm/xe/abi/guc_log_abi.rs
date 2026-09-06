//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/guc_log_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2024 Intel Corporation
//

//
// DOC: GuC Log buffer Layout
//
// The in-memory log buffer layout is as follows::
//
// +===============================+	      0000h
// |    Crash dump state header    |		^
// +-------------------------------+ 32B	|
// |      Debug state header       |		|
// +-------------------------------+ 64B	4KB
// |     Capture state header      |		|
// +-------------------------------+ 96B	|
// |                               |		v
// +===============================+ <--- EVENT_DATA_OFFSET
// |  Event logs(raw data)         |		^
// |                               |		|
// |                               | EVENT_DATA_BUFFER_SIZE
// |                               |		|
// |                               |		v
// +===============================+ <--- CRASH_DUMP_OFFSET
// | Crash Dump(raw data)          |		^
// |                               |		|
// |                               | CRASH_DUMP_BUFFER_SIZE
// |                               |		|
// |                               |		v
// +===============================+ <--- STATE_CAPTURE_OFFSET
// | Error state capture(raw data) |		^
// |                               |		|
// |                               | STATE_CAPTURE_BUFFER_SIZE
// |                               |		|
// |                               |		v
// +===============================+ Total: GUC_LOG_SIZE
//
// GuC logging buffer types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_log_type {
    GUC_LOG_TYPE_EVENT_DATA,
    GUC_LOG_TYPE_CRASH_DUMP,
    GUC_LOG_TYPE_STATE_CAPTURE,
}

pub const GUC_LOG_BUFFER_TYPE_MAX: c_int = 3;
//
// struct guc_log_buffer_state - GuC log buffer state
//
// Below state structure is used for coordination of retrieval of GuC firmware
// logs. Separate state is maintained for each log buffer type.
// read_ptr points to the location where Xe read last in log buffer and
// is read only for GuC firmware. write_ptr is incremented by GuC with number
// of bytes written for each log entry and is read only for Xe.
// When any type of log buffer becomes half full, GuC sends a flush interrupt.
// GuC firmware expects that while it is writing to 2nd half of the buffer,
// first half would get consumed by Host and then get a flush completed
// acknowledgment from Host, so that it does not end up doing any overwrite
// causing loss of logs. So when buffer gets half filled & Xe has requested
// for interrupt, GuC will set flush_to_file field, set the sampled_write_ptr
// to the value of write_ptr and raise the interrupt.
// On receiving the interrupt Xe should read the buffer, clear flush_to_file
// field and also update read_ptr with the value of sample_write_ptr, before
// sending an acknowledgment to GuC. marker & version fields are for internal
// usage of GuC and opaque to Xe. buffer_full_cnt field is incremented every
// time GuC detects the log buffer overflow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_log_buffer_state {
// @marker: buffer state start marker
    pub marker: [u32; 2],
// @read_ptr: the last byte offset that was read by KMD previously
    pub read_ptr: u32,
//
// @write_ptr: the next byte offset location that will be written by
// GuC
//
    pub write_ptr: u32,
// @size: Log buffer size
    pub size: u32,
//
// @sampled_write_ptr: Log buffer write pointer
// This is written by GuC to the byte offset of the next free entry in
// the buffer on log buffer half full or state capture notification
//
    pub sampled_write_ptr: u32,
//
// @wrap_offset: wraparound offset
// This is the byte offset of location 1 byte after last valid guc log
// event entry written by Guc firmware before there was a wraparound.
// This field is updated by guc firmware and should be used by Host
// when copying buffer contents to file.
//
    pub wrap_offset: u32,
// @flags: Flush to file flag and buffer full count
    pub flags: u32,

// @version: The Guc-Log-Entry format version
    pub version: u32,
    pub __packed: },
