//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/io_uring/zcrx.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT
//
// Header file for the io_uring zerocopy receive (zcrx) interface.
//
// Copyright (C) 2026 Pavel Begunkov
// Copyright (C) 2026 David Wei
// Copyright (C) Meta Platforms, Inc.
//

// Zero copy receive refill queue entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_zcrx_rqe {
    pub off: __u64,
    pub len: __u32,
    pub __pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_zcrx_cqe {
    pub off: __u64,
    pub __pad: __u64,
}

// The bit from which area id is encoded into offsets
pub const IORING_ZCRX_AREA_SHIFT: c_int = 48;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_zcrx_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub rqes: __u32,
    pub __resv2: __u32,
    pub __resv: [__u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_uring_zcrx_area_flags {
    IORING_ZCRX_AREA_DMABUF		= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_zcrx_area_reg {
    pub addr: __u64,
    pub len: __u64,
    pub rq_area_token: __u64,
    pub flags: __u32,
    pub dmabuf_fd: __u32,
    pub __resv2: [__u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zcrx_reg_flags {
    ZCRX_REG_IMPORT		= 1,

//
// Register a zcrx instance without a net device. All data will be
// copied. The refill queue entries might not be automatically
// consumed and need to be flushed, see ZCRX_CTRL_FLUSH_RQ.
//
    ZCRX_REG_NODEV		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zcrx_features {
//
// The user can ask for the desired rx page size by passing the
// value in struct io_uring_zcrx_ifq_reg::rx_buf_len.
//
    ZCRX_FEATURE_RX_PAGE_SIZE	= 1 << 0,
    ZCRX_FEATURE_EVENT		= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zcrx_event_type {
    ZCRX_EVENT_ALLOC_FAIL,
    ZCRX_EVENT_COPY,

    __ZCRX_EVENT_TYPE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zcrx_event_desc_flags {
// If set, stats_offset holds a valid offset to a zcrx_stats struct
    ZCRX_EVENT_DESC_FLAG_STATS = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_stats {
    pub /: *mut *mut __u64 copy_count; / cumulative copy-fallback CQEs,
    pub /: *mut *mut __u64 copy_bytes; / cumulative bytes copied,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_event_desc {
    pub user_data: __u64,
    pub type_mask: __u32,
    pub /: *mut *mut __u32 flags; / see enum zcrx_event_desc_flags,
    pub /: *mut *mut __u64 stats_offset; / offset from the beginning of refill ring region for stats,
    pub __resv2: [__u64; 9],
}

//
// Argument for IORING_REGISTER_ZCRX_IFQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_zcrx_ifq_reg {
    pub if_idx: __u32,
    pub if_rxq: __u32,
    pub rq_entries: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u64 area_ptr; / pointer to struct io_uring_zcrx_area_reg,
    pub /: *mut *mut *mut __u64 region_ptr; / struct io_uring_region_desc,
    pub offsets: io_uring_zcrx_offsets,
    pub zcrx_id: __u32,
    pub rx_buf_len: __u32,
    pub /: *mut *mut __u64 event_desc; / see struct zcrx_event_desc,
    pub __resv: [__u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zcrx_ctrl_op {
    ZCRX_CTRL_FLUSH_RQ,
    ZCRX_CTRL_EXPORT,
    ZCRX_CTRL_ARM_EVENT,
    ZCRX_CTRL_ADD_AREA,

    __ZCRX_CTRL_LAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_ctrl_flush_rq {
    pub __resv: [__u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_ctrl_export {
    pub zcrx_fd: __u32,
    pub __resv1: [__u32; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_ctrl_arm_event {
    pub /: *mut *mut __u32 event_type; / see enum zcrx_event_type,
    pub __resv: [__u32; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_ctrl_add_area {
    pub /: *mut *mut __u64 area_ptr; / pointer to struct io_uring_zcrx_area_reg,
    pub __resv: [__u64; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrx_ctrl {
    pub zcrx_id: __u32,
    pub /: *mut *mut __u32 op; / see enum zcrx_ctrl_op,
    pub __resv: [__u64; 2],
    pub zc_export: zcrx_ctrl_export,
    pub zc_flush: zcrx_ctrl_flush_rq,
    pub zc_arm_event: zcrx_ctrl_arm_event,
    pub zc_area: zcrx_ctrl_add_area,
}
