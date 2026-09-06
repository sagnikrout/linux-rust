//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vboxvideo/hgsmi_defs.h
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
// Copyright (C) 2006-2017 Oracle Corporation
// Buffer sequence type mask.
pub const HGSMI_BUFFER_HEADER_F_SEQ_MASK: c_uint = 0x03;
// Single buffer, not a part of a sequence.
pub const HGSMI_BUFFER_HEADER_F_SEQ_SINGLE: c_uint = 0x00;
// The first buffer in a sequence.
pub const HGSMI_BUFFER_HEADER_F_SEQ_START: c_uint = 0x01;
// A middle buffer in a sequence.
pub const HGSMI_BUFFER_HEADER_F_SEQ_CONTINUE: c_uint = 0x02;
// The last buffer in a sequence.
pub const HGSMI_BUFFER_HEADER_F_SEQ_END: c_uint = 0x03;
// 16 bytes buffer header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hgsmi_buffer_header {
    pub /: *mut *mut u32 data_size; / Size of data that follows the header.,
    pub /: *mut *mut *mut u8 flags; / HGSMI_BUFFER_HEADER_F_,
    pub /: *mut *mut u8 channel; / The channel the data must be routed to.,
    pub /: *mut *mut u16 channel_info; / Opaque to the HGSMI, used by the channel.,
// Opaque placeholder to make the union 8 bytes.
    pub header_data: [u8; 8],
// HGSMI_BUFFER_HEADER_F_SEQ_SINGLE
    pub /: *mut *mut u32 reserved1; / A reserved field, initialize to 0.,
    pub /: *mut *mut u32 reserved2; / A reserved field, initialize to 0.,
    pub buffer: },
// HGSMI_BUFFER_HEADER_F_SEQ_START
// Must be the same for all buffers in the sequence.
    pub sequence_number: u32,
// The total size of the sequence.
    pub sequence_size: u32,
    pub sequence_start: },
//
// HGSMI_BUFFER_HEADER_F_SEQ_CONTINUE and
// HGSMI_BUFFER_HEADER_F_SEQ_END
//
// Must be the same for all buffers in the sequence.
    pub sequence_number: u32,
// Data offset in the entire sequence.
    pub sequence_offset: u32,
    pub sequence_continue: },
    pub u: },
    pub __packed: },
// 8 bytes buffer tail.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hgsmi_buffer_tail {
// Reserved, must be initialized to 0.
    pub reserved: u32,
//
// One-at-a-Time Hash: https://www.burtleburtle.net/bob/hash/doobs.html
// Over the header, offset and for first 4 bytes of the tail.
//
    pub checksum: u32,
    pub __packed: },
//
// The size of the array of channels. Array indexes are u8.
// Note: the value must not be changed.
//
pub const HGSMI_NUMBER_OF_CHANNELS: c_uint = 0x100;
