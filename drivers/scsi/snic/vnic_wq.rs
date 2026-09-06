//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/vnic_wq.h
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.

// Work queue control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_wq_ctrl {
    pub /: *mut *mut u64 ring_base; / 0x00,
    pub /: *mut *mut u32 ring_size; / 0x08,
    pub pad0: u32,
    pub /: *mut *mut u32 posted_index; / 0x10,
    pub pad1: u32,
    pub /: *mut *mut u32 cq_index; / 0x18,
    pub pad2: u32,
    pub /: *mut *mut u32 enable; / 0x20,
    pub pad3: u32,
    pub /: *mut *mut u32 running; / 0x28,
    pub pad4: u32,
    pub /: *mut *mut u32 fetch_index; / 0x30,
    pub pad5: u32,
    pub /: *mut *mut u32 dca_value; / 0x38,
    pub pad6: u32,
    pub /: *mut *mut u32 error_interrupt_enable; / 0x40,
    pub pad7: u32,
    pub /: *mut *mut u32 error_interrupt_offset; / 0x48,
    pub pad8: u32,
    pub /: *mut *mut u32 error_status; / 0x50,
    pub pad9: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_wq_buf {
    pub next: *mut vnic_wq_buf,
    pub dma_addr: dma_addr_t,
    pub os_buf: *mut c_void,
    pub len: c_uint,
    pub index: c_uint,
    pub sop: c_int,
    pub desc: *mut c_void,
}

// Break the vnic_wq_buf allocations into blocks of 64 entries
pub const VNIC_WQ_BUF_MIN_BLK_ENTRIES: c_int = 32;
pub const VNIC_WQ_BUF_DFLT_BLK_ENTRIES: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_wq {
    pub index: c_uint,
    pub vdev: *mut vnic_dev,
    pub /: *mut *mut *mut vnic_wq_ctrl __iomem ctrl; / memory-mapped,
    pub ring: vnic_dev_ring,
    pub bufs: [*mut vnic_wq_buf; VNIC_WQ_BUF_BLKS_MAX],
    pub to_use: *mut vnic_wq_buf,
    pub to_clean: *mut vnic_wq_buf,
    pub pkts_outstanding: c_uint,
}

// how many does SW own?
// how many does HW own?
// Adding write memory barrier prevents compiler and/or CPU
// reordering, thus avoiding descriptor posting before
// descriptor is initialized. Otherwise, hardware can read
// stale descriptor fields.
//
extern "C" {
    pub fn svnic_wq_free(wq: *mut vnic_wq);
}
extern "C" {
    pub fn svnic_wq_error_status(wq: *mut vnic_wq) -> c_uint;
}
extern "C" {
    pub fn svnic_wq_enable(wq: *mut vnic_wq);
}
extern "C" {
    pub fn svnic_wq_disable(wq: *mut vnic_wq) -> c_int;
}
