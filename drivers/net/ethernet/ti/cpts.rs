//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/cpts.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// TI Common Platform Time Sync
//
// Copyright (C) 2012 Richard Cochran <richardcochran@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_cpts {
    pub /: *mut *mut u32 idver; / Identification and version,
    pub /: *mut *mut u32 control; / Time sync control,
    pub /: *mut *mut u32 rftclk_sel; / Reference Clock Select Register,
    pub /: *mut *mut u32 ts_push; / Time stamp event push,
    pub /: *mut *mut u32 ts_load_val; / Time stamp load value,
    pub /: *mut *mut u32 ts_load_en; / Time stamp load enable,
    pub res2: [u32; 2],
    pub /: *mut *mut u32 intstat_raw; / Time sync interrupt status raw,
    pub /: *mut *mut u32 intstat_masked; / Time sync interrupt status masked,
    pub /: *mut *mut u32 int_enable; / Time sync interrupt enable,
    pub res3: u32,
    pub /: *mut *mut u32 event_pop; / Event interrupt pop,
    pub /: *mut *mut u32 event_low; / 32 Bit Event Time Stamp,
    pub /: *mut *mut u32 event_high; / Event Type Fields,
}

// Bit definitions for the IDVER register

// Bit definitions for the CONTROL register

//
// Definitions for the single bit resisters:
// TS_PUSH TS_LOAD_EN  INTSTAT_RAW INTSTAT_MASKED INT_ENABLE EVENT_POP
//

// Bit definitions for the EVENT_HIGH register

pub const CPTS_FIFO_DEPTH: c_int = 16;
pub const CPTS_MAX_EVENTS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpts_event {
    pub list: list_head,
    pub tmo: c_ulong,
    pub high: u32,
    pub low: u32,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpts {
    pub dev: *mut device,
    pub reg: *mut cpsw_cpts __iomem,
    pub tx_enable: c_int,
    pub rx_enable: c_int,
    pub info: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub /: *mut *mut spinlock_t lock; / protects fifo/events,
    pub /: *mut *mut u32 cc_mult; / for the nominal frequency,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub phc_index: c_int,
    pub refclk: *mut clk,
    pub events: list_head,
    pub pool: list_head,
    pub pool_data: [cpts_event; CPTS_MAX_EVENTS],
    pub ov_check_period: c_ulong,
    pub txq: sk_buff_head,
    pub cur_timestamp: u64,
    pub mult_new: u32,
    pub /: *mut *mut mutex ptp_clk_mutex; / sync PTP interface and worker,
    pub irq_poll: bool,
    pub ts_push_complete: completion,
    pub hw_ts_enable: u32,
}

extern "C" {
    pub fn cpts_rx_timestamp(cpts: *mut cpts, skb: *mut sk_buff);
}
extern "C" {
    pub fn cpts_tx_timestamp(cpts: *mut cpts, skb: *mut sk_buff);
}
extern "C" {
    pub fn cpts_register(cpts: *mut cpts) -> c_int;
}
extern "C" {
    pub fn cpts_unregister(cpts: *mut cpts);
}
extern "C" {
    pub fn cpts_release(cpts: *mut cpts);
}
extern "C" {
    pub fn cpts_misc_interrupt(cpts: *mut cpts);
}

