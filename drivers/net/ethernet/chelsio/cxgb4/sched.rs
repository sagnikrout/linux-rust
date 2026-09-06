//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/sched.h
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


//
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2016 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const SCHED_CLS_NONE: c_uint = 0xff;
pub const FW_SCHED_CLS_NONE: c_uint = 0xffffffff;
// Max rate that can be set to a scheduling class is 100 Gbps

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sched_fw_ops {
    SCHED_FW_OP_ADD,
    SCHED_FW_OP_DEL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sched_bind_type {
    SCHED_QUEUE,
    SCHED_FLOWC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_queue_entry {
    pub list: list_head,
    pub cntxt_id: c_uint,
    pub param: ch_sched_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_flowc_entry {
    pub list: list_head,
    pub param: ch_sched_flowc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_sched_class {
    pub state: u8,
    pub idx: u8,
    pub info: ch_sched_params,
    pub bind_type: sched_bind_type,
    pub entry_list: list_head,
    pub refcnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_table {
    pub sched_size: u8,
    pub __counted_by(sched_size): ch_sched_class tab[],
}

extern "C" {
    pub fn cxgb4_sched_class_free(dev: *mut net_device, classid: u8);
}
extern "C" {
    pub fn t4_cleanup_sched(adap: *mut adapter);
}
