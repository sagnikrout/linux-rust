//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/hsi/cs-protocol.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// cmt-speech interface definitions
//
// Copyright (C) 2008,2009,2010 Nokia Corporation. All rights reserved.
//
// Contact: Kai Vehmanen <kai.vehmanen@nokia.com>
// Original author: Peter Ujfalusi <peter.ujfalusi@nokia.com>
//

// chardev parameters

// user-space API versioning
pub const CS_IF_VERSION: c_int = 2;
// APE kernel <-> user space messages
pub const CS_CMD_SHIFT: c_int = 28;
pub const CS_DOMAIN_SHIFT: c_int = 24;
pub const CS_CMD_MASK: c_uint = 0xff000000;
pub const CS_PARAM_MASK: c_uint = 0xffffff;

// params to CS_ERROR indication
pub const CS_ERR_PEER_RESET: c_int = 0;
// ioctl interface
// parameters to CS_CONFIG_BUFS ioctl

// parameters to CS_GET_STATE ioctl
pub const CS_STATE_CLOSED: c_int = 0;

// maximum number of TX/RX buffers
pub const CS_MAX_BUFFERS_SHIFT: c_int = 4;

// Parameters for setting up the data buffers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_buffer_config {
    pub /: *mut *mut __u32 rx_bufs; / number of RX buffer slots,
    pub /: *mut *mut __u32 tx_bufs; / number of TX buffer slots,
    pub /: *mut *mut __u32 buf_size; / bytes,
    pub /: *mut *mut *mut __u32 flags; / see CS_FEAT_,
    pub reserved: [__u32; 4],
}

//
// struct for monotonic timestamp taken when the
// last control command was received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_timestamp {
    pub /: *mut *mut __u32 tv_sec; / seconds,
    pub /: *mut *mut __u32 tv_nsec; / nanoseconds,
}

//
// Struct describing the layout and contents of the driver mmap area.
// This information is meant as read-only information for the application.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_mmap_config_block {
    pub reserved1: __u32,
    pub /: *mut *mut __u32 buf_size; / 0=disabled, otherwise the transfer size,
    pub /: *mut *mut __u32 rx_bufs; / # of RX buffers,
    pub /: *mut *mut __u32 tx_bufs; / # of TX buffers,
    pub reserved2: __u32,
// array of offsets within the mmap area for each RX and TX buffer
    pub rx_offsets: [__u32; CS_MAX_BUFFERS],
    pub tx_offsets: [__u32; CS_MAX_BUFFERS],
    pub rx_ptr: __u32,
    pub rx_ptr_boundary: __u32,
    pub reserved3: [__u32; 2],
// enabled with CS_FEAT_TSTAMP_RX_CTRL
    pub tstamp_rx_ctrl: cs_timestamp,
}

