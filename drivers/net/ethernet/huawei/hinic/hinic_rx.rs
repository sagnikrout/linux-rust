//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_rx.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_RX_CSUM_OFFLOAD_EN: c_uint = 0xFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rxq_stats {
    pub pkts: u64,
    pub bytes: u64,
    pub errors: u64,
    pub csum_errors: u64,
    pub other_errors: u64,
    pub alloc_skb_err: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rxq {
    pub netdev: *mut net_device,
    pub rq: *mut hinic_rq,
    pub rxq_stats: hinic_rxq_stats,
    pub irq_name: *mut c_char,
    pub buf_len: u16,
    pub rx_buff_shift: u32,
    pub napi: napi_struct,
}

extern "C" {
    pub fn hinic_rxq_get_stats(rxq: *mut hinic_rxq, stats: *mut hinic_rxq_stats);
}
extern "C" {
    pub fn hinic_clean_rxq(rxq: *mut hinic_rxq);
}
