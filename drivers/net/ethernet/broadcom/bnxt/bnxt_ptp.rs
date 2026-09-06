//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_ptp.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2021 Broadcom Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

pub const BNXT_PTP_GRC_WIN: c_int = 6;
pub const BNXT_PTP_GRC_WIN_BASE: c_uint = 0x6000;
pub const BNXT_MAX_PHC_DRIFT: c_int = 31000000;
pub const BNXT_CYCLES_SHIFT: c_int = 23;
pub const BNXT_DEVCLK_FREQ: c_int = 1000000;
pub const BNXT_LO_TIMER_MASK: c_uint = 0x0000ffffffffUL;
pub const BNXT_HI_TIMER_MASK: c_uint = 0xffff00000000UL;
pub const BNXT_HI_TIMER_SHIFT: c_int = 24;

pub const BNXT_PTP_QTS_TIMEOUT: c_int = 1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_pin {
    pub event: u8,
    pub usage: u8,
    pub state: u8,
}

pub const BNXT_PPS_PIN_DISABLE: c_int = 0;
pub const BNXT_PPS_PIN_ENABLE: c_int = 1;
pub const BNXT_PPS_PIN_NONE: c_int = 0;
pub const BNXT_PPS_PIN_PPS_IN: c_int = 1;
pub const BNXT_PPS_PIN_PPS_OUT: c_int = 2;
pub const BNXT_PPS_PIN_SYNC_IN: c_int = 3;
pub const BNXT_PPS_PIN_SYNC_OUT: c_int = 4;
pub const BNXT_PPS_EVENT_INTERNAL: c_int = 1;
pub const BNXT_PPS_EVENT_EXTERNAL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_pps {
    pub num_pins: u8,
pub const BNXT_MAX_TSIO_PINS: c_int = 4;
    pub pins: [pps_pin; BNXT_MAX_TSIO_PINS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ptp_stats {
    pub ts_pkts: u64,
    pub ts_lost: u64,
    pub ts_err: core::sync::atomic::AtomicI64,
}

pub const BNXT_MAX_TX_TS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ptp_tx_req {
    pub tx_skb: *mut sk_buff,
    pub tx_seqid: u16,
    pub tx_hdr_off: u16,
    pub abs_txts_tmo: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ptp_cfg {
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub pps_info: bnxt_pps,
// serialize timecounter access
    pub ptp_lock: seqlock_t,
// serialize ts tx request queuing
    pub ptp_tx_lock: spinlock_t,
    pub current_time: u64,
    pub next_period: c_ulong,
    pub next_overflow_check: c_ulong,
    pub cmult: u32,
// cache of upper 24 bits of cyclecoutner. 8 bits are used to check for roll-over
    pub old_time: u32,
// a 23b shift cyclecounter will overflow in ~36 mins.  Check overflow every 18 mins.
    pub txts_req: [bnxt_ptp_tx_req; BNXT_MAX_TX_TS],
    pub bp: *mut bnxt,
    pub tx_avail: u32,
    pub rxctl: u16,

    pub tx_tstamp_en:1: u8,
    pub rtc_configured:1: u8,
    pub rx_filter: c_int,
    pub tstamp_filters: u32,
    pub refclk_regs: [u32; 2],
    pub refclk_mapped_regs: [u32; 2],
    pub txts_tmo: u32,
    pub txts_prod: u16,
    pub txts_cons: u16,
    pub stats: bnxt_ptp_stats,
}

extern "C" {
    pub fn bnxt_ptp_parse(skb: *mut sk_buff, seq_id: *mut u16, hdr_off: *mut u16) -> c_int;
}
extern "C" {
    pub fn bnxt_ptp_update_current_time(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_ptp_pps_event(bp: *mut bnxt, data1: u32, data2: u32);
}
extern "C" {
    pub fn bnxt_ptp_cfg_tstamp_filters(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_ptp_reapply_pps(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_ptp_free_txts_skbs(ptp: *mut bnxt_ptp_cfg);
}
extern "C" {
    pub fn bnxt_ptp_get_txts_prod(ptp: *mut bnxt_ptp_cfg, prod: *mut u16) -> c_int;
}
extern "C" {
    pub fn bnxt_get_tx_ts_p5(bp: *mut bnxt, skb: *mut sk_buff, prod: u16);
}
extern "C" {
    pub fn bnxt_get_rx_ts_p5(bp: *mut bnxt, ts: *mut u64, pkt_ts: u32) -> c_int;
}
extern "C" {
    pub fn bnxt_ptp_rtc_timecounter_init(ptp: *mut bnxt_ptp_cfg, ns: u64);
}
extern "C" {
    pub fn bnxt_ptp_init_rtc(bp: *mut bnxt, phc_cfg: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_ptp_init(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_ptp_clear(bp: *mut bnxt);
}
