//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_tx.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

pub const TCP_HDR_DATA_OFF_UNIT_SHIFT: c_int = 2;

pub const HINIC3_COMPACT_WQEE_SKB_MAX_LEN: c_int = 16383;
pub const HINIC3_TX_POLL_WEIGHT: c_int = 64;
pub const HINIC3_DEFAULT_STOP_THRS: c_int = 6;
pub const HINIC3_DEFAULT_START_THRS: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sq_wqe_data_format {
    SQ_NORMAL_WQE = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sq_wqe_ec_type {
    SQ_WQE_COMPACT_TYPE  = 0,
    SQ_WQE_EXTENDED_TYPE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sq_wqe_tasksect_len_type {
    SQ_WQE_TASKSECT_46BITS  = 0,
    SQ_WQE_TASKSECT_16BYTES = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_tx_offload_type {
    HINIC3_TX_OFFLOAD_TSO     = BIT(0),
    HINIC3_TX_OFFLOAD_CSUM    = BIT(1),
    HINIC3_TX_OFFLOAD_VLAN    = BIT(2),
    HINIC3_TX_OFFLOAD_INVALID = BIT(3),
    HINIC3_TX_OFFLOAD_ESP     = BIT(4),
}

pub const SQ_CTRL_MAX_PLDOFF: c_int = 221;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_sq_wqe_desc {
    pub ctrl_len: __le32,
    pub queue_info: __le32,
    pub hi_addr: __le32,
    pub lo_addr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_sq_task {
    pub pkt_info0: __le32,
    pub ip_identify: __le32,
    pub rsvd: __le32,
    pub vlan_offload: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_sq_wqe_combo {
    pub ctrl_bd0: *mut hinic3_sq_wqe_desc,
    pub task: *mut hinic3_sq_task,
    pub bds_head: *mut hinic3_sq_bufdesc,
    pub bds_sec2: *mut hinic3_sq_bufdesc,
    pub first_bds_num: u16,
    pub wqe_type: u32,
    pub task_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_txq_stats {
    pub packets: u64,
    pub bytes: u64,
    pub busy: u64,
    pub dropped: u64,
    pub skb_pad_err: u64,
    pub frag_len_overflow: u64,
    pub offload_cow_skb_err: u64,
    pub map_frag_err: u64,
    pub unknown_tunnel_pkt: u64,
    pub frag_size_err: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_dma_info {
    pub dma: dma_addr_t,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_tx_info {
    pub skb: *mut sk_buff,
    pub wqebb_cnt: u16,
    pub dma_info: *mut hinic3_dma_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_txq {
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub q_id: u16,
    pub tx_stop_thrs: u16,
    pub tx_start_thrs: u16,
    pub q_mask: u32,
    pub q_depth: u32,
    pub tx_info: *mut hinic3_tx_info,
    pub sq: *mut hinic3_io_queue,
    pub txq_stats: hinic3_txq_stats,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_dyna_txq_res {
    pub tx_info: *mut hinic3_tx_info,
    pub bds: *mut hinic3_dma_info,
}

extern "C" {
    pub fn hinic3_alloc_txqs(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn hinic3_free_txqs(netdev: *mut net_device);
}
extern "C" {
    pub fn hinic3_xmit_frame(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn hinic3_tx_poll(txq: *mut hinic3_txq, budget: c_int) -> bool;
}
extern "C" {
    pub fn hinic3_flush_txqs(netdev: *mut net_device);
}
