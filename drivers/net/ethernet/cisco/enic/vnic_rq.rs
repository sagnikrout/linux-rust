//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_rq.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

// Receive queue control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_rq_ctrl {
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
    pub /: *mut *mut u32 error_interrupt_enable; / 0x38,
    pub pad6: u32,
    pub /: *mut *mut u32 error_interrupt_offset; / 0x40,
    pub pad7: u32,
    pub /: *mut *mut u32 error_status; / 0x48,
    pub pad8: u32,
    pub /: *mut *mut u32 dropped_packet_count; / 0x50,
    pub pad9: u32,
    pub /: *mut *mut u32 dropped_packet_count_rc; / 0x58,
    pub pad10: u32,
}

// Break the vnic_rq_buf allocations into blocks of 32/64 entries
pub const VNIC_RQ_BUF_MIN_BLK_ENTRIES: c_int = 32;
pub const VNIC_RQ_BUF_DFLT_BLK_ENTRIES: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_rq_buf {
    pub next: *mut vnic_rq_buf,
    pub dma_addr: dma_addr_t,
    pub os_buf: *mut c_void,
    pub os_buf_index: c_uint,
    pub len: c_uint,
    pub index: c_uint,
    pub desc: *mut c_void,
    pub wr_id: u64,
    pub offset: c_uint,
    pub truesize: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enic_poll_state {
    ENIC_POLL_STATE_IDLE,
    ENIC_POLL_STATE_NAPI,
    ENIC_POLL_STATE_POLL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_rq {
    pub index: c_uint,
    pub vdev: *mut vnic_dev,
    pub /: *mut *mut *mut vnic_rq_ctrl __iomem ctrl; / memory-mapped,
    pub ring: vnic_dev_ring,
    pub bufs: [*mut vnic_rq_buf; VNIC_RQ_BUF_BLKS_MAX],
    pub to_use: *mut vnic_rq_buf,
    pub to_clean: *mut vnic_rq_buf,
    pub os_buf_head: *mut c_void,
    pub pkts_outstanding: c_uint,
}

// how many does SW own?
// how many does HW own?
// Move the posted_index every nth descriptor
//

pub const VNIC_RQ_RETURN_RATE: c_uint = 0xf	/* keep 2^n - 1 */;

// Adding write memory barrier prevents compiler and/or CPU
// reordering, thus avoiding descriptor posting before
// descriptor is initialized. Otherwise, hardware can read
// stale descriptor fields.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum desc_return_options {
    VNIC_RQ_RETURN_DESC,
    VNIC_RQ_DEFER_RETURN_DESC,
}

extern "C" {
    pub fn vnic_rq_free(rq: *mut vnic_rq);
}
extern "C" {
    pub fn vnic_rq_error_status(rq: *mut vnic_rq) -> c_uint;
}
extern "C" {
    pub fn vnic_rq_enable(rq: *mut vnic_rq);
}
extern "C" {
    pub fn vnic_rq_disable(rq: *mut vnic_rq) -> c_int;
}
