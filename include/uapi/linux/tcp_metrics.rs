//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tcp_metrics.h
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
// tcp_metrics.h - TCP Metrics Interface

// NETLINK_GENERIC related info
//

pub const TCP_METRICS_GENL_VERSION: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_metric_index {
    TCP_METRIC_RTT,		/* in ms units */
    TCP_METRIC_RTTVAR,	/* in ms units */
    TCP_METRIC_SSTHRESH,
    TCP_METRIC_CWND,
    TCP_METRIC_REORDERING,

    TCP_METRIC_RTT_US,	/* in usec units */
    TCP_METRIC_RTTVAR_US,	/* in usec units */

// Always last.
    __TCP_METRIC_MAX,
}

// Re-define enum tcp_metric_index, again, using the values carried
// as netlink attribute types.
//

