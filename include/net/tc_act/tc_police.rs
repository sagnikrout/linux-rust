//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_police.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_police_params {
    pub action: c_int,
    pub tcfp_result: c_int,
    pub tcfp_ewma_rate: u32,
    pub tcfp_mtu: u32,
    pub tcfp_burst: i64,
    pub tcfp_mtu_ptoks: i64,
    pub tcfp_pkt_burst: i64,
    pub rate: psched_ratecfg,
    pub rate_present: bool,
    pub peak: psched_ratecfg,
    pub peak_present: bool,
    pub ppsrate: psched_pktrate,
    pub pps_present: bool,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_police {
    pub common: tc_action,
    pub params: *mut tcf_police_params __rcu,
    pub ____cacheline_aligned_in_smp: spinlock_t tcfp_lock,
    pub tcfp_toks: i64,
    pub tcfp_ptoks: i64,
    pub tcfp_pkttoks: i64,
    pub tcfp_t_c: i64,
}

// old policer structure from before tc actions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_police_compat {
    pub index: u32,
    pub action: c_int,
    pub limit: u32,
    pub burst: u32,
    pub mtu: u32,
    pub rate: tc_ratespec,
    pub peakrate: tc_ratespec,
}

//
// "rate" bytes   "burst" nanoseconds
// ------------ * -------------------
// 1 second          2^6 ticks
//
// ------------------------------------
// NSEC_PER_SEC nanoseconds
// ------------------------
// 2^6 ticks
//
// "rate" bytes   "burst" nanoseconds            2^6 ticks
// = ------------ * ------------------- * ------------------------
// 1 second          2^6 ticks        NSEC_PER_SEC nanoseconds
//
// "rate" * "burst"
// = ---------------- bytes/nanosecond
// NSEC_PER_SEC^2
//
// "rate" * "burst"
// = ---------------- bytes/second
// NSEC_PER_SEC
//
// "rate" pkts     "burst" nanoseconds
// ------------ *  -------------------
// 1 second          2^6 ticks
//
// ------------------------------------
// NSEC_PER_SEC nanoseconds
// ------------------------
// 2^6 ticks
//
// "rate" pkts    "burst" nanoseconds            2^6 ticks
// = ------------ * ------------------- * ------------------------
// 1 second          2^6 ticks        NSEC_PER_SEC nanoseconds
//
// "rate" * "burst"
// = ---------------- pkts/nanosecond
// NSEC_PER_SEC^2
//
// "rate" * "burst"
// = ---------------- pkts/second
// NSEC_PER_SEC
//
