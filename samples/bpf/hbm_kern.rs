//! Automatically rewritten from C Header to Rust Module
//! Source: samples/bpf/hbm_kern.h
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
// Include file for sample Host Bandwidth Manager (HBM) BPF programs
//

pub const DROP_PKT: c_int = 0;
pub const ALLOW_PKT: c_int = 1;
pub const TCP_ECN_OK: c_int = 1;
pub const CWR: c_int = 2;

pub const INITIAL_CREDIT_PACKETS: c_int = 100;
pub const MAX_BYTES_PER_PACKET: c_int = 1500;

pub const LARGE_PKT_THRESH: c_int = 120;

// Time base accounting for fq's EDT

// Reserve 20us of queuing for small packets (less than 120 bytes)

// rate in bytes per ns << 20

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_pkt_info {
    pub cwnd: c_int,
    pub rtt: c_int,
    pub packets_out: c_int,
    pub is_ip: bool,
    pub is_tcp: bool,
    pub ecn: c_short,
}

// Following is needed for work conserving
// Optionally update statistics
