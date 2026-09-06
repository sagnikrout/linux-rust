//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/lio_vf_rep.h
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
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2017 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more
// details.
//
// This file may also be available under a different license from Cavium.
// Contact Cavium, Inc. for more information
//
// ! \file octeon_vf_main.h
// \brief Host Driver: This file defines vf_rep related macros and structures
//
pub const LIO_VF_REP_REQ_TMO_MS: c_int = 5000;
pub const LIO_VF_REP_STATS_POLL_TIME_MS: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_desc {
    pub parent_ndev: *mut net_device,
    pub ndev: *mut net_device,
    pub oct: *mut octeon_device,
    pub stats: lio_vf_rep_stats,
    pub stats_wk: cavium_wk,
    pub ifstate: core::sync::atomic::AtomicI32,
    pub ifidx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_sc_ctx {
    pub complete: completion,
}

extern "C" {
    pub fn lio_vf_rep_create(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn lio_vf_rep_destroy(oct: *mut octeon_device);
}
extern "C" {
    pub fn lio_vf_rep_modinit() -> c_int;
}
extern "C" {
    pub fn lio_vf_rep_modexit();
}
