//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ieee8021q.h
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
// Copyright (c) 2024 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>

//
// enum ieee8021q_traffic_type - 802.1Q traffic type priority values (802.1Q-2022)
//
// @IEEE8021Q_TT_BK: Background
// @IEEE8021Q_TT_BE: Best Effort (default). According to 802.1Q-2022, BE is 0
// but has higher priority than BK which is 1.
// @IEEE8021Q_TT_EE: Excellent Effort
// @IEEE8021Q_TT_CA: Critical Applications
// @IEEE8021Q_TT_VI: Video, < 100 ms latency and jitter
// @IEEE8021Q_TT_VO: Voice, < 10 ms latency and jitter
// @IEEE8021Q_TT_IC: Internetwork Control
// @IEEE8021Q_TT_NC: Network Control
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee8021q_traffic_type {
    IEEE8021Q_TT_BK = 0,
    IEEE8021Q_TT_BE = 1,
    IEEE8021Q_TT_EE = 2,
    IEEE8021Q_TT_CA = 3,
    IEEE8021Q_TT_VI = 4,
    IEEE8021Q_TT_VO = 5,
    IEEE8021Q_TT_IC = 6,
    IEEE8021Q_TT_NC = 7,

// private:
    IEEE8021Q_TT_MAX,
}

extern "C" {
    pub fn ietf_dscp_to_ieee8021q_tt(dscp: u8) -> c_int;
}
extern "C" {
    pub fn ieee8021q_tt_to_tc(tt: ieee8021q_traffic_type, num_queues: c_uint) -> c_int;
}

