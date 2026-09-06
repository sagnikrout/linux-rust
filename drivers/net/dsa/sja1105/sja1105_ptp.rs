//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/sja1105/sja1105_ptp.h
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
// Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
//

// Timestamps are in units of 8 ns clock ticks (equivalent to
// a fixed 125 MHz clock).
//
pub const SJA1105_TICK_NS: c_int = 8;
// Calculate the first base_time in the future that satisfies this
// relationship:
//
// future_base_time = base_time + N x cycle_time >= now, or
//
// now - base_time
// N >= ---------------
// cycle_time
//
// Because N is an integer, the ceiling value of the above "a / b" ratio
// is in fact precisely the floor value of "(a + b - 1) / b", which is
// easier to calculate only having integer division tools.
//
// This is not a preprocessor macro because the "ns" argument may or may not be
// s64 at caller side. This ensures it is properly type-cast before div_s64.
//
extern "C" {
    pub fn div_s64(_arg: ns, _arg: 200) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_ptp_cmd {
    pub /: *mut *mut u64 startptpcp; / start toggling PTP_CLK pin,
    pub /: *mut *mut u64 stopptpcp; / stop toggling PTP_CLK pin,
    pub /: *mut *mut u64 ptpstrtsch; / start schedule,
    pub /: *mut *mut u64 ptpstopsch; / stop schedule,
    pub /: *mut *mut u64 resptp; / reset,
    pub /: *mut *mut u64 corrclk4ts; / use the corrected clock for timestamps,
    pub /: *mut *mut u64 ptpclkadd; / enum sja1105_ptp_clk_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_ptp_data {
    pub extts_timer: timer_list,
// Used only on SJA1105 to reconstruct partial timestamps
    pub skb_rxtstamp_queue: sk_buff_head,
// Used on SJA1110 where meta frames are generated only for
// 2-step TX timestamps
//
    pub skb_txtstamp_queue: sk_buff_head,
    pub caps: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub cmd: sja1105_ptp_cmd,
// Serializes all operations on the PTP hardware clock
    pub lock: mutex,
    pub extts_enabled: bool,
    pub ptpsyncts: u64,
}

extern "C" {
    pub fn sja1105_ptp_clock_register(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn sja1105_ptp_clock_unregister(ds: *mut dsa_switch);
}
extern "C" {
    pub fn __sja1105_ptp_adjtime(ds: *mut dsa_switch, delta: i64) -> c_int;
}
extern "C" {
    pub fn sja1105_rxtstamp(ds: *mut dsa_switch, port: c_int, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn sja1110_rxtstamp(ds: *mut dsa_switch, port: c_int, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn sja1110_txtstamp(ds: *mut dsa_switch, port: c_int, skb: *mut sk_buff);
}

// Structures cannot be empty in C. Bah!
// Keep the mutex as the only element, which is a bit more difficult to
// refactor out of sja1105_main.c anyway.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_ptp_data {
    pub lock: mutex,
}

