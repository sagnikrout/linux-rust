//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/yaps.h
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
//
// Copyright (c) 2017-2026 Morse Micro
//

pub const YAPS_TX_SKBQ_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_yaps_to_chip_q {
    MM81X_YAPS_TX_Q = 0,
    MM81X_YAPS_CMD_Q,
    MM81X_YAPS_BEACON_Q,
    MM81X_YAPS_MGMT_Q,
// Keep this last
    MM81X_YAPS_NUM_TC_Q
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_yaps_pkt {
    pub skb: *mut sk_buff,
    pub tc_queue: mm81x_yaps_to_chip_q,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_yaps {
    pub mors: *mut mm81x,
    pub aux_data: *mut mm81x_yaps_hw_aux_data,
    pub ops: *const mm81x_yaps_ops,
    pub flags: u8,
    pub to_chip_pkts: *mut mm81x_yaps_pkt,
    pub from_chip_pkts: *mut mm81x_yaps_pkt,
    pub hw: },
// Chip interface is stopping, new work should not be enqueued.
    pub finish: bool,
    pub data_tx_qs: [mm81x_skbq; YAPS_TX_SKBQ_MAX],
    pub beacon_q: mm81x_skbq,
    pub mgmt_q: mm81x_skbq,
    pub data_rx_q: mm81x_skbq,
    pub cmd_q: mm81x_skbq,
    pub cmd_resp_q: mm81x_skbq,
    pub timer: timer_list,
    pub retry_expiry: c_ulong,
    pub is_full: bool,
    pub chip_queue_full: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_yaps_ops {
    pub num_pkts_sent): *mut int num_pkts, int,
    pub num_pkts_received): *mut int num_pkts_max, int,
    pub yaps): *mut *mut int (update_status)(struct mm81x_yaps,
}

extern "C" {
    pub fn mm81x_yaps_init(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_yaps_show(yaps: *mut mm81x_yaps, file: *mut seq_file);
}
extern "C" {
    pub fn mm81x_yaps_finish(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_yaps_flush_tx_data(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_yaps_flush_cmds(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_yaps_work(work: *mut work_struct);
}
extern "C" {
    pub fn mm81x_yaps_stale_tx_work(work: *mut work_struct);
}
extern "C" {
    pub fn mm81x_yaps_get_tx_status_pending_count(mors: *mut mm81x) -> c_int;
}
extern "C" {
    pub fn mm81x_yaps_get_tx_buffered_count(mors: *mut mm81x) -> c_int;
}
