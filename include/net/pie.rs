//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/pie.h
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

pub const QUEUE_THRESHOLD: c_int = 16384;

pub const PIE_SCALE: c_int = 8;
//
// struct pie_params - contains pie parameters
// @target:		target delay in pschedtime
// @tupdate:		interval at which drop probability is calculated
// @limit:		total number of packets that can be in the queue
// @alpha:		parameter to control drop probability
// @beta:		parameter to control drop probability
// @ecn:		is ECN marking of packets enabled
// @bytemode:		is drop probability scaled based on pkt size
// @dq_rate_estimator:	is Little's law used for qdelay calculation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pie_params {
    pub target: psched_time_t,
    pub tupdate: u32,
    pub limit: u32,
    pub alpha: u32,
    pub beta: u32,
    pub ecn: u8,
    pub bytemode: u8,
    pub dq_rate_estimator: u8,
}

//
// struct pie_vars - contains pie variables
// @qdelay:		current queue delay
// @qdelay_old:		queue delay in previous qdelay calculation
// @burst_time:		burst time allowance
// @dq_tstamp:		timestamp at which dq rate was last calculated
// @prob:		drop probability
// @accu_prob:		accumulated drop probability
// @dq_count:		number of bytes dequeued in a measurement cycle
// @avg_dq_rate:	calculated average dq rate
// @backlog_old:	queue backlog during previous qdelay calculation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pie_vars {
    pub qdelay: psched_time_t,
    pub qdelay_old: psched_time_t,
    pub burst_time: psched_time_t,
    pub dq_tstamp: psched_time_t,
    pub prob: u64,
    pub accu_prob: u64,
    pub dq_count: u64,
    pub avg_dq_rate: u32,
    pub backlog_old: u32,
}

//
// struct pie_stats - contains pie stats
// @packets_in:	total number of packets enqueued
// @dropped:	packets dropped due to pie action
// @overlimit:	packets dropped due to lack of space in queue
// @ecn_mark:	packets marked with ECN
// @maxq:	maximum queue size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pie_stats {
    pub packets_in: u32,
    pub dropped: u32,
    pub overlimit: u32,
    pub ecn_mark: u32,
    pub maxq: u32,
}

//
// struct pie_skb_cb - contains private skb vars
// @enqueue_time:	timestamp when the packet is enqueued
// @mem_usage:		size of the skb during enqueue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pie_skb_cb {
    pub enqueue_time: psched_time_t,
    pub mem_usage: u32,
}
