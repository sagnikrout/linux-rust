//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/gen_stats.h
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
// struct gnet_stats_basic - byte/packet throughput statistics
// @bytes: number of seen bytes
// @packets: number of seen packets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_stats_basic {
    pub bytes: __u64,
    pub packets: __u32,
}

//
// struct gnet_stats_rate_est - rate estimator
// @bps: current byte rate
// @pps: current packet rate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_stats_rate_est {
    pub bps: __u32,
    pub pps: __u32,
}

//
// struct gnet_stats_rate_est64 - rate estimator
// @bps: current byte rate
// @pps: current packet rate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_stats_rate_est64 {
    pub bps: __u64,
    pub pps: __u64,
}

//
// struct gnet_stats_queue - queuing statistics
// @qlen: queue length
// @backlog: backlog size of queue
// @drops: number of dropped packets
// @requeues: number of requeues
// @overlimits: number of enqueues over the limit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_stats_queue {
    pub qlen: __u32,
    pub backlog: __u32,
    pub drops: __u32,
    pub requeues: __u32,
    pub overlimits: __u32,
}

//
// struct gnet_estimator - rate estimator configuration
// @interval: sampling period
// @ewma_log: the log of measurement window weight
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_estimator {
    pub interval: signed char,
    pub ewma_log: c_uchar,
}
