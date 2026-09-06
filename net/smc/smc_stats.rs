//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_stats.h
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
//
// Shared Memory Communications over RDMA (SMC-R) and RoCE
//
// Macros for SMC statistics
//
// Copyright IBM Corp. 2021
//
// Author(s):  Guvenc Gulce
//

pub const SMC_MAX_FBACK_RSN_CNT: c_int = 36;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_stats_fback {
    pub fback_code: c_int,
    pub count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_stats_rsn {
    pub srv: [smc_stats_fback; SMC_MAX_FBACK_RSN_CNT],
    pub clnt: [smc_stats_fback; SMC_MAX_FBACK_RSN_CNT],
    pub srv_fback_cnt: u64,
    pub clnt_fback_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_stats_rmbcnt {
    pub buf_size_small_peer_cnt: u64,
    pub buf_size_small_cnt: u64,
    pub buf_full_peer_cnt: u64,
    pub buf_full_cnt: u64,
    pub reuse_cnt: u64,
    pub alloc_cnt: u64,
    pub dgrade_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_stats_memsize {
    pub buf: [u64; SMC_BUF_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_stats_tech {
    pub tx_rmbsize: smc_stats_memsize,
    pub rx_rmbsize: smc_stats_memsize,
    pub tx_pd: smc_stats_memsize,
    pub rx_pd: smc_stats_memsize,
    pub rmb_tx: smc_stats_rmbcnt,
    pub rmb_rx: smc_stats_rmbcnt,
    pub clnt_v1_succ_cnt: u64,
    pub clnt_v2_succ_cnt: u64,
    pub srv_v1_succ_cnt: u64,
    pub srv_v2_succ_cnt: u64,
    pub urg_data_cnt: u64,
    pub splice_cnt: u64,
    pub cork_cnt: u64,
    pub ndly_cnt: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_cnt: u64,
    pub tx_cnt: u64,
    pub rx_rmbuse: u64,
    pub tx_rmbuse: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_stats {
    pub smc: [smc_stats_tech; 2],
    pub clnt_hshake_err_cnt: u64,
    pub srv_hshake_err_cnt: u64,
}

extern "C" {
    pub fn smc_nl_get_stats(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn smc_nl_get_fback_stats(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn smc_stats_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn smc_stats_exit(net: *mut net);
}
