//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_tc_matchall.h
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
// Copyright (C) 2019 Chelsio Communications.  All rights reserved.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_matchall_state {
    CXGB4_MATCHALL_STATE_DISABLED = 0,
    CXGB4_MATCHALL_STATE_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_matchall_egress_entry {
    pub /: *mut *mut cxgb4_matchall_state state; / Current MATCHALL offload state,
    pub /: *mut *mut u8 hwtc; / Traffic class bound to port,
    pub /: *mut *mut u64 cookie; / Used to identify the MATCHALL rule offloaded,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_matchall_ingress_entry {
    pub /: *mut *mut cxgb4_matchall_state state; / Current MATCHALL offload state,
    pub /: *mut *mut u32 tid[CXGB4_FILTER_TYPE_MAX]; / Index to hardware filter entries,
// Filter entries
    pub fs: [ch_filter_specification; CXGB4_FILTER_TYPE_MAX],
    pub /: *mut *mut u16 viid_mirror; / Identifier for allocated Mirror VI,
    pub /: *mut *mut u64 bytes; / # of bytes hitting the filter,
    pub /: *mut *mut u64 packets; / # of packets hitting the filter,
    pub /: *mut *mut u64 last_used; / Last updated jiffies time,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_tc_port_matchall {
    pub /: *mut *mut cxgb4_matchall_egress_entry egress; / Egress offload info,
    pub /: *mut *mut cxgb4_matchall_ingress_entry ingress; / Ingress offload info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_tc_matchall {
    pub /: *mut *mut *mut cxgb4_tc_port_matchall port_matchall; / Per port entry,
}

extern "C" {
    pub fn cxgb4_init_tc_matchall(adap: *mut adapter) -> c_int;
}
extern "C" {
    pub fn cxgb4_cleanup_tc_matchall(adap: *mut adapter);
}
