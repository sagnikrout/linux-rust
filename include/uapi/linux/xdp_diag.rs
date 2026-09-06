//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/xdp_diag.h
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
// xdp_diag: interface for query/monitor XDP sockets
// Copyright(c) 2019 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_diag_req {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub pad: __u16,
    pub xdiag_ino: __u32,
    pub xdiag_show: __u32,
    pub xdiag_cookie: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_diag_msg {
    pub xdiag_family: __u8,
    pub xdiag_type: __u8,
    pub pad: __u16,
    pub xdiag_ino: __u32,
    pub xdiag_cookie: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_diag_info {
    pub ifindex: __u32,
    pub queue_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_diag_ring {
    pub /: *mut *mut __u32 entries; /num descs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_diag_umem {
    pub size: __u64,
    pub id: __u32,
    pub num_pages: __u32,
    pub chunk_size: __u32,
    pub headroom: __u32,
    pub ifindex: __u32,
    pub queue_id: __u32,
    pub flags: __u32,
    pub refs: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_diag_stats {
    pub n_rx_dropped: __u64,
    pub n_rx_invalid: __u64,
    pub n_rx_full: __u64,
    pub n_fill_ring_empty: __u64,
    pub n_tx_invalid: __u64,
    pub n_tx_ring_empty: __u64,
}
