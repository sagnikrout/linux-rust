//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/cxgb3_ioctl.h
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
// Copyright (c) 2003-2008 Chelsio, Inc. All rights reserved.
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
// Ioctl commands specific to this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_reg {
    pub cmd: u32,
    pub addr: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_cntxt {
    pub cmd: u32,
    pub cntxt_type: u32,
    pub cntxt_id: u32,
    pub data: [u32; 4],
}

// context types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_desc {
    pub cmd: u32,
    pub queue_num: u32,
    pub idx: u32,
    pub size: u32,
    pub data: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_mem_range {
    pub cmd: u32,
    pub mem_id: u32,
    pub addr: u32,
    pub len: u32,
    pub version: u32,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_qset_params {
    pub cmd: u32,
    pub qset_idx: u32,
    pub txq_size: [i32; 3],
    pub rspq_size: i32,
    pub fl_size: [i32; 2],
    pub intr_lat: i32,
    pub polling: i32,
    pub lro: i32,
    pub cong_thres: i32,
    pub vector: i32,
    pub qnum: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_pktsched_params {
    pub cmd: u32,
    pub sched: u8,
    pub idx: u8,
    pub min: u8,
    pub max: u8,
    pub binding: u8,
}

// TCB size in 32-bit words

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_mtus {
    pub cmd: u32,
    pub nmtus: u32,
    pub mtus: [u16; NMTUS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_pm {
    pub cmd: u32,
    pub tx_pg_sz: u32,
    pub tx_num_pg: u32,
    pub rx_pg_sz: u32,
    pub rx_num_pg: u32,
    pub pm_total: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_tcam {
    pub cmd: u32,
    pub tcam_size: u32,
    pub nservers: u32,
    pub nroutes: u32,
    pub nfilters: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_tcb {
    pub cmd: u32,
    pub tcb_index: u32,
    pub tcb_data: [u32; TCB_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_tcam_word {
    pub cmd: u32,
    pub addr: u32,
    pub buf: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_trace {
    pub cmd: u32,
    pub sip: u32,
    pub sip_mask: u32,
    pub dip: u32,
    pub dip_mask: u32,
    pub sport: u16,
    pub sport_mask: u16,
    pub dport: u16,
    pub dport_mask: u16,
    pub vlan:12: u32,
    pub vlan_mask:12: u32,
    pub intf:4: u32,
    pub intf_mask:4: u32,
    pub proto: u8,
    pub proto_mask: u8,
    pub invert_match:1: u8,
    pub config_tx:1: u8,
    pub config_rx:1: u8,
    pub trace_tx:1: u8,
    pub trace_rx:1: u8,
}

