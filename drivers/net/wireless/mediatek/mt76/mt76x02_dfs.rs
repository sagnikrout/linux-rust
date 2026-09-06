//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_dfs.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2016 Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

// Macro flag: #define __MT76x02_DFS_H

pub const MT_DFS_NUM_ENGINES: c_int = 4;
// bbp params
pub const MT_DFS_SYM_ROUND: c_int = 0;
pub const MT_DFS_DELTA_DELAY: c_int = 2;
pub const MT_DFS_VGA_MASK: c_int = 0;
pub const MT_DFS_PWR_GAIN_OFFSET: c_int = 3;
pub const MT_DFS_PWR_DOWN_TIME: c_uint = 0xf;
pub const MT_DFS_RX_PE_MASK: c_uint = 0xff;
pub const MT_DFS_PKT_END_MASK: c_int = 0;
pub const MT_DFS_CH_EN: c_uint = 0xf;
// sw detector params
pub const MT_DFS_EVENT_LOOP: c_int = 64;

pub const MT_DFS_EVENT_TIME_MARGIN: c_int = 2000;
pub const MT_DFS_PRI_MARGIN: c_int = 4;
pub const MT_DFS_SEQUENCE_TH: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_radar_specs {
    pub mode: u8,
    pub avg_len: u16,
    pub e_low: u16,
    pub e_high: u16,
    pub w_low: u16,
    pub w_high: u16,
    pub w_margin: u16,
    pub t_low: u32,
    pub t_high: u32,
    pub t_margin: u16,
    pub b_low: u32,
    pub b_high: u32,
    pub event_expiration: u32,
    pub pwr_jmp: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_event {
    pub fetch_ts: c_ulong,
    pub ts: u32,
    pub width: u16,
    pub engine: u8,
}

pub const MT_DFS_EVENT_BUFLEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_event_rb {
    pub data: [mt76x02_dfs_event; MT_DFS_EVENT_BUFLEN],
    pub t_rb: int h_rb,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_sequence {
    pub head: list_head,
    pub first_ts: u32,
    pub last_ts: u32,
    pub pri: u32,
    pub count: u16,
    pub engine: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_hw_pulse {
    pub engine: u8,
    pub period: u32,
    pub w1: u32,
    pub w2: u32,
    pub burst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_sw_detector_params {
    pub min_pri: u32,
    pub max_pri: u32,
    pub pri_margin: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_engine_stats {
    pub hw_pattern: u32,
    pub hw_pulse_discarded: u32,
    pub sw_pattern: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_seq_stats {
    pub seq_pool_len: u32,
    pub seq_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dfs_pattern_detector {
    pub chirp_pulse_cnt: u8,
    pub chirp_pulse_ts: u32,
    pub sw_dpd_params: mt76x02_dfs_sw_detector_params,
    pub event_rb: [mt76x02_dfs_event_rb; 2],
    pub sequences: list_head,
    pub seq_pool: list_head,
    pub seq_stats: mt76x02_dfs_seq_stats,
    pub last_sw_check: c_ulong,
    pub last_event_ts: u32,
    pub stats: [mt76x02_dfs_engine_stats; MT_DFS_NUM_ENGINES],
    pub dfs_tasklet: tasklet_struct,
}

extern "C" {
    pub fn mt76x02_dfs_init_params(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_dfs_init_detector(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_phy_dfs_adjust_agc(dev: *mut mt76x02_dev);
}
