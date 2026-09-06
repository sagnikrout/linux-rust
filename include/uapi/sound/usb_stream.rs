//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/usb_stream.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright (C) 2007, 2008 Karsten Wiese <fzu@wemgehoertderstaat.de>
//
pub const USB_STREAM_INTERFACE_VERSION: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_stream_packet {
    pub offset: unsigned,
    pub length: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_stream_config {
    pub version: unsigned,
    pub sample_rate: unsigned,
    pub period_frames: unsigned,
    pub frame_size: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_stream {
    pub cfg: usb_stream_config,
    pub read_size: unsigned,
    pub write_size: unsigned,
    pub period_size: c_int,
    pub state: unsigned,
    pub idle_insize: c_int,
    pub idle_outsize: c_int,
    pub sync_packet: c_int,
    pub insize_done: unsigned,
    pub periods_done: unsigned,
    pub periods_polled: unsigned,
    pub outpacket: [usb_stream_packet; 2],
    pub inpackets: unsigned,
    pub inpacket_head: unsigned,
    pub inpacket_split: unsigned,
    pub inpacket_split_at: unsigned,
    pub next_inpacket_split: unsigned,
    pub next_inpacket_split_at: unsigned,
    pub inpacket: [usb_stream_packet; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_stream_state {
    usb_stream_invalid,
    usb_stream_stopped,
    usb_stream_sync0,
    usb_stream_sync1,
    usb_stream_ready,
    usb_stream_running,
    usb_stream_xrun,
}
