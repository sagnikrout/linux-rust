//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fnic_trace.h
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
// Copyright 2012 Cisco Systems, Inc.  All rights reserved.
pub const FNIC_ENTRY_SIZE_BYTES: c_int = 64;
pub const FC_TRC_SIZE_BYTES: c_int = 256;

//
// Fisrt bit of FNIC_FC_RECV and FNIC_FC_SEND is used to represent the type
// of frame 1 => Eth frame, 0=> FC frame
//
pub const FNIC_FC_RECV: c_uint = 0x52 /* Character R */;
pub const FNIC_FC_SEND: c_uint = 0x54 /* Character T */;
pub const FNIC_FC_LE: c_uint = 0x4C /* Character L */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_trace_data {
    pub low: u32,
    pub high: u32,
}

pub type fnic_trace_data_t = fnic_trace_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_trace_hdr {
    pub time_stamp: timespec64,
    pub host_no: u32,
    pub frame_type: u8,
    pub frame_len: u8,
    pub __attribute__((__packed__)): },

    pub \: *mut *mut fnic_trace_data_t trace_buf = fnic_trace_get_buf();,
    pub \: trace_buf->timestamp.low = jiffies;,
    pub \: trace_buf->fnaddr.low = (u32)(unsigned long)_fn;,
    pub \: trace_buf->timestamp.val = jiffies;,
    pub \: trace_buf->fnaddr.val = (u64)(unsigned long)_fn;,
    pub \: trace_buf->host_no = _hn;,
    pub \: trace_buf->tag = _t;,
    pub \: trace_buf->data[0] = (u64)(unsigned long)_a;,
    pub \: trace_buf->data[1] = (u64)(unsigned long)_b;,
    pub \: trace_buf->data[2] = (u64)(unsigned long)_c;,
    pub \: trace_buf->data[3] = (u64)(unsigned long)_d;,
    pub \: trace_buf->data[4] = (u64)(unsigned long)_e;,
    pub fnic_trace_get_buf(void): *mut fnic_trace_data_t,
    pub ): *mut int fnic_get_trace_data(fnic_dbgfs_t,
    pub fnic_trace_buf_init(void): c_int,
    pub fnic_trace_free(void): c_void,
    pub fnic_debugfs_init(void): c_int,
    pub fnic_debugfs_terminate(void): c_void,
    pub fnic_trace_debugfs_init(void): c_void,
    pub fnic_trace_debugfs_terminate(void): c_void,
// Fnic FC CTLR Trace releated function
    pub fnic_fc_trace_init(void): c_int,
    pub fnic_fc_trace_free(void): c_void,
    pub fc_frame_len): *mut *mut char frame, u32,
    pub rdata_flag): *mut *mut int fnic_fc_trace_get_data(fnic_dbgfs_t fnic_dbgfs_prt, u8,
    pub rdata_flag): *mut *mut int len, u8,
    pub fnic_fc_trace_debugfs_init(void): c_void,
    pub fnic_fc_trace_debugfs_terminate(void): c_void,
