//! Automatically rewritten from C Header to Rust Module
//! Source: samples/bpf/hbm.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2019 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// Include file for Host Bandwidth Management (HBM) programs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_vqueue {
    pub lock: bpf_spin_lock,
// 4 byte hole
    pub /: *mut *mut unsigned long long lasttime; / In ns,
    pub /: *mut *mut int credit; / In bytes,
    pub /: *mut *mut unsigned int rate; / In bytes per NS << 20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_queue_stats {
    pub Mbps*/: *mut *mut unsigned long rate; / in,
    pub /: *mut *mut no_cn:1; / do not use cn flags,
    pub pkts_marked: c_ulonglong,
    pub bytes_marked: c_ulonglong,
    pub pkts_dropped: c_ulonglong,
    pub bytes_dropped: c_ulonglong,
    pub pkts_total: c_ulonglong,
    pub bytes_total: c_ulonglong,
    pub firstPacketTime: c_ulonglong,
    pub lastPacketTime: c_ulonglong,
    pub pkts_ecn_ce: c_ulonglong,
    pub returnValCount: [c_ulonglong; 4],
    pub sum_cwnd: c_ulonglong,
    pub sum_rtt: c_ulonglong,
    pub sum_cwnd_cnt: c_ulonglong,
    pub sum_credit: c_longlong,
}
