//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/clock.h
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
// Copyright (c) 2017, Mellanox Technologies, Ltd.  All rights reserved.
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

pub const MAX_PIN_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_pps {
    pub pin_caps: [u8; MAX_PIN_NUM],
    pub start: [u64; MAX_PIN_NUM],
    pub enabled: u8,
    pub min_npps_period: u64,
    pub min_out_pulse_duration_ns: u64,
    pub pin_armed: [bool; MAX_PIN_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_timer {
    pub cycles: cyclecounter,
    pub tc: timecounter,
    pub nominal_c_mult: u32,
    pub overflow_period: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_clock {
    pub lock: seqlock_t,
    pub ptp: *mut ptp_clock,
    pub ptp_info: ptp_clock_info,
    pub pps_info: mlx5_pps,
    pub timer: mlx5_timer,
    pub shared: bool,
}

extern "C" {
    pub fn ktime_t(: *mut *mut cqe_ts_to_ns)(struct mlx5_clock, _arg: u64) -> typedef;
}

extern "C" {
    pub fn mlx5_init_clock(mdev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cleanup_clock(mdev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_clock_load(mdev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_clock_unload(mdev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn ns_to_ktime(_arg: nsec) -> return;
}

extern "C" {
    pub fn ns_to_ktime(_arg: time) -> return;
}

