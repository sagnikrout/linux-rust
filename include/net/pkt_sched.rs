//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/pkt_sched.h
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

pub const DEFAULT_TX_QUEUE_LEN: c_int = 1000;
pub const STAB_SIZE_LOG_MAX: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdisc_walker {
    pub stop: c_int,
    pub skip: c_int,
    pub count: c_int,
    pub ): *mut *mut *mut int (fn)(struct Qdisc , unsigned long cl, struct qdisc_walker,
}

//
pub type psched_time_t = u64;
// Avoid doing 64 bit divide
pub const PSCHED_SHIFT: c_int = 6;

pub const PSCHED_PASTPERFECT: c_int = 0;
extern "C" {
    pub fn PSCHED_NS2TICKS(_arg: ktime_get_ns()) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdisc_watchdog {
    pub timer: hrtimer,
    pub qdisc: *mut Qdisc,
}

extern "C" {
    pub fn qdisc_watchdog_init(wd: *mut qdisc_watchdog, qdisc: *mut Qdisc);
}
extern "C" {
    pub fn qdisc_watchdog_schedule_range_ns(_arg: wd, _arg: expires, _arg: 0ULL) -> return;
}
extern "C" {
    pub fn qdisc_watchdog_cancel(wd: *mut qdisc_watchdog);
}
extern "C" {
    pub fn fifo_set_limit(q: *mut Qdisc, limit: c_uint) -> c_int;
}
extern "C" {
    pub fn register_qdisc(qops: *mut Qdisc_ops) -> c_int;
}
extern "C" {
    pub fn unregister_qdisc(qops: *mut Qdisc_ops);
}

extern "C" {
    pub fn qdisc_get_default(id: *mut c_char, len: usize);
}
extern "C" {
    pub fn qdisc_set_default(id: *const c_char) -> c_int;
}
extern "C" {
    pub fn qdisc_hash_add(q: *mut Qdisc, invisible: bool);
}
extern "C" {
    pub fn qdisc_hash_del(q: *mut Qdisc);
}
extern "C" {
    pub fn qdisc_put_rtab(tab: *mut qdisc_rate_table);
}
extern "C" {
    pub fn qdisc_put_stab(tab: *mut qdisc_size_table);
}
extern "C" {
    pub fn __qdisc_run(q: *mut Qdisc);
}
extern "C" {
    pub fn qdisc_run_end(_arg: q) -> return;
}
// Calculate maximal size of packet seen by hard_start_xmit
//
extern "C" {
    pub fn dev_net(_arg: q->dev_queue->dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_query_caps_base {
    pub type: tc_setup_type,
    pub caps: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_cbs_qopt_offload {
    pub enable: u8,
    pub queue: i32,
    pub hicredit: i32,
    pub locredit: i32,
    pub idleslope: i32,
    pub sendslope: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_etf_qopt_offload {
    pub enable: u8,
    pub queue: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_mqprio_caps {
    pub validate_queue_counts:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_mqprio_qopt_offload {
// struct tc_mqprio_qopt must always be the first element
    pub qopt: tc_mqprio_qopt,
    pub extack: *mut netlink_ext_ack,
    pub mode: u16,
    pub shaper: u16,
    pub flags: u32,
    pub min_rate: [u64; TC_QOPT_MAX_QUEUE],
    pub max_rate: [u64; TC_QOPT_MAX_QUEUE],
    pub preemptible_tcs: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_taprio_caps {
    pub supports_queue_max_sdu:1: bool,
    pub gate_mask_per_txq:1: bool,
// Device expects lower TXQ numbers to have higher priority over higher
// TXQs, regardless of their TC mapping. DO NOT USE FOR NEW DRIVERS,
// INSTEAD ENFORCE A PROPER TC:TXQ MAPPING COMING FROM USER SPACE.
//
    pub broken_mqprio:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_taprio_qopt_cmd {
    TAPRIO_CMD_REPLACE,
    TAPRIO_CMD_DESTROY,
    TAPRIO_CMD_STATS,
    TAPRIO_CMD_QUEUE_STATS,
}

//
// struct tc_taprio_qopt_stats - IEEE 802.1Qbv statistics
// @window_drops: Frames that were dropped because they were too large to be
// transmitted in any of the allotted time windows (open gates) for their
// traffic class.
// @tx_overruns: Frames still being transmitted by the MAC after the
// transmission gate associated with their traffic class has closed.
// Equivalent to `12.29.1.1.2 TransmissionOverrun` from 802.1Q-2018.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_taprio_qopt_stats {
    pub window_drops: u64,
    pub tx_overruns: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_taprio_qopt_queue_stats {
    pub queue: c_int,
    pub stats: tc_taprio_qopt_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_taprio_sched_entry {
    pub /: *mut *mut *mut u8 command; / TC_TAPRIO_CMD_,
// The gate_mask in the offloading side refers to traffic classes
    pub gate_mask: u32,
    pub interval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_taprio_qopt_offload {
    pub cmd: tc_taprio_qopt_cmd,
// TAPRIO_CMD_STATS
    pub stats: tc_taprio_qopt_stats,
// TAPRIO_CMD_QUEUE_STATS
    pub queue_stats: tc_taprio_qopt_queue_stats,
// TAPRIO_CMD_REPLACE
    pub mqprio: tc_mqprio_qopt_offload,
    pub extack: *mut netlink_ext_ack,
    pub base_time: ktime_t,
    pub cycle_time: u64,
    pub cycle_time_extension: u64,
    pub max_sdu: [u32; TC_MAX_QUEUE],
    pub num_entries: usize,
    pub entries: [tc_taprio_sched_entry; ],
}

// Reference counting
// offload);
extern "C" {
    pub fn taprio_offload_free(offload: *mut tc_taprio_qopt_offload);
}

// Reference counting

// Ensure skb_mstamp_ns, which might have been populated with the txtime, is
// not mistaken for a software timestamp, because this will otherwise prevent
// the dispatch of hardware timestamps to the socket.
//
// Skip dynamic keys if nesting is not possible
