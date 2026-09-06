//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/io_uring/query.h
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
// Header file for the io_uring query interface.
//
// Copyright (C) 2026 Pavel Begunkov <asml.silence@gmail.com>
// Copyright (C) Meta Platforms, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_query_hdr {
    pub next_entry: __u64,
    pub query_data: __u64,
    pub query_op: __u32,
    pub size: __u32,
    pub result: __s32,
    pub __resv: [__u32; 3],
}

// Doesn't require a ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_query_opcode {
// The number of supported IORING_OP_* opcodes
    pub nr_request_opcodes: __u32,
// The number of supported IORING_[UN]REGISTER_* opcodes
    pub nr_register_opcodes: __u32,
// Bitmask of all supported IORING_FEAT_* flags
    pub feature_flags: __u64,
// Bitmask of all supported IORING_SETUP_* flags
    pub ring_setup_flags: __u64,
// Bitmask of all supported IORING_ENTER_** flags
    pub enter_flags: __u64,
// Bitmask of all supported IOSQE_* flags
    pub sqe_flags: __u64,
// The number of available query opcodes
    pub nr_query_opcodes: __u32,
    pub __pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_query_zcrx {
// Bitmask of supported ZCRX_REG_* flags,
    pub register_flags: __u64,
// Bitmask of all supported IORING_ZCRX_AREA_* flags
    pub area_flags: __u64,
// The number of supported ZCRX_CTRL_* opcodes
    pub nr_ctrl_opcodes: __u32,
// Bitmask of ZCRX_FEATURE_* indicating which features are available
    pub features: __u32,
// The refill ring header size
    pub rq_hdr_size: __u32,
// The alignment for the header
    pub rq_hdr_alignment: __u32,
    pub __resv2: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_query_zcrx_event {
// Bitmask of supported ZCRX_EVENT_* flags
    pub event_flags: __u32,
// Size of zcrx_stats
    pub stats_size: __u32,
// Required alignment for the stats struct within the region (ie stats_offset)
    pub stats_off_alignment: __u32,
    pub __resv1: __u32,
    pub __resv2: [__u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_query_scq {
// The SQ/CQ rings header size
    pub hdr_size: __u64,
// The alignment for the header
    pub hdr_alignment: __u64,
}
