//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/libiscsi_tcp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// iSCSI over TCP/IP Data-Path lib
//
// Copyright (C) 2008 Mike Christie
// Copyright (C) 2008 Red Hat, Inc.  All rights reserved.
// maintained by open-iscsi@googlegroups.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_segment {
    pub data: *mut c_uchar,
    pub size: c_uint,
    pub copied: c_uint,
    pub total_size: c_uint,
    pub total_copied: c_uint,
    pub crcp: *mut u32,
    pub padbuf: [c_uchar; ISCSI_PAD_LEN],
    pub recv_digest: [c_uchar; ISCSI_DIGEST_SIZE],
    pub digest: [c_uchar; ISCSI_DIGEST_SIZE],
    pub digest_len: c_uint,
    pub sg: *mut scatterlist,
    pub sg_mapped: *mut c_void,
    pub sg_offset: c_uint,
    pub atomic_mapped: bool,
    pub done: *mut iscsi_segment_done_fn_t,
}

// Socket connection receive helper
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tcp_recv {
    pub hdr: *mut iscsi_hdr,
    pub segment: iscsi_segment,
// Allocate buffer for BHS + AHS
    pub hdr_buf: [u32; 64],
// copied and flipped values
    pub datalen: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tcp_conn {
    pub iscsi_conn: *mut iscsi_conn,
    pub dd_data: *mut c_void,
    pub : *mut *mut int stop_stage; / conn_stop() flag:,
// stop to recover,
// stop to terminate
// control data
    pub /: *mut *mut iscsi_tcp_recv in; / TCP receive context,
// CRC32C (Rx) LLD should set this if they do not offload
    pub rx_crcp: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tcp_task {
    pub /: *mut *mut uint32_t exp_datasn; / expected target's R2TSN/DataSN,
    pub data_offset: c_int,
    pub /: *mut *mut *mut iscsi_r2t_info r2t; / in progress solict R2T,
    pub r2tpool: iscsi_pool,
    pub r2tqueue: kfifo,
    pub dd_data: *mut c_void,
    pub pool2queue: spinlock_t,
    pub queue2pool: spinlock_t,
}

extern "C" {
    pub fn iscsi_tcp_hdr_recv_prep(tcp_conn: *mut iscsi_tcp_conn);
}
extern "C" {
    pub fn iscsi_tcp_cleanup_task(task: *mut iscsi_task);
}
extern "C" {
    pub fn iscsi_tcp_task_init(task: *mut iscsi_task) -> c_int;
}
extern "C" {
    pub fn iscsi_tcp_task_xmit(task: *mut iscsi_task) -> c_int;
}
// segment helpers
extern "C" {
    pub fn iscsi_tcp_recv_segment_is_hdr(tcp_conn: *mut iscsi_tcp_conn) -> c_int;
}
extern "C" {
    pub fn iscsi_tcp_segment_unmap(segment: *mut iscsi_segment);
}
// digest helpers
extern "C" {
    pub fn iscsi_tcp_conn_teardown(cls_conn: *mut iscsi_cls_conn);
}
// misc helpers
extern "C" {
    pub fn iscsi_tcp_r2tpool_alloc(session: *mut iscsi_session) -> c_int;
}
extern "C" {
    pub fn iscsi_tcp_r2tpool_free(session: *mut iscsi_session);
}
extern "C" {
    pub fn iscsi_tcp_set_max_r2t(conn: *mut iscsi_conn, buf: *mut c_char) -> c_int;
}
