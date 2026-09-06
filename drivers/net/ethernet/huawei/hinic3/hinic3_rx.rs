//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_rx.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_rxq_stats {
    pub packets: u64,
    pub bytes: u64,
    pub errors: u64,
    pub csum_errors: u64,
    pub other_errors: u64,
    pub dropped: u64,
    pub rx_buf_empty: u64,
    pub alloc_skb_err: u64,
    pub alloc_rx_buf_err: u64,
    pub restore_drop_sge: u64,
    pub syncp: u64_stats_sync,
}

// RX Completion information that is provided by HW for a specific RX WQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_rq_cqe {
    pub status: __le32,
    pub vlan_len: __le32,
    pub offload_type: __le32,
    pub rsvd3: __le32,
    pub rsvd4: __le32,
    pub rsvd5: __le32,
    pub rsvd6: __le32,
    pub pkt_info: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_rq_wqe {
    pub buf_hi_addr: __le32,
    pub buf_lo_addr: __le32,
    pub cqe_hi_addr: __le32,
    pub cqe_lo_addr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_rx_info {
    pub page: *mut page,
    pub page_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_rxq {
    pub netdev: *mut net_device,
    pub q_id: u16,
    pub q_depth: u32,
    pub q_mask: u32,
    pub buf_len: u16,
    pub buf_len_shift: u32,
    pub rxq_stats: hinic3_rxq_stats,
    pub cons_idx: u32,
    pub delta: u32,
    pub irq_id: u32,
    pub msix_entry_idx: u16,
// cqe_arr and rx_info are arrays of rq_depth elements. Each element is
// statically associated (by index) to a specific rq_wqe.
//
    pub cqe_arr: *mut hinic3_rq_cqe,
    pub rx_info: *mut hinic3_rx_info,
    pub page_pool: *mut page_pool,
    pub rq: *mut hinic3_io_queue,
    pub irq_cfg: *mut hinic3_irq_cfg,
    pub next_to_alloc: u16,
    pub next_to_update: u16,
    pub /: *mut *mut *mut device dev; / device for DMA mapping,
    pub cqe_start_paddr: dma_addr_t,
    pub dim: dim,
    pub last_coalesc_timer_cfg: u8,
    pub last_pending_limit: u8,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_dyna_rxq_res {
    pub next_to_alloc: u16,
    pub rx_info: *mut hinic3_rx_info,
    pub cqe_start_paddr: dma_addr_t,
    pub cqe_start_vaddr: *mut c_void,
    pub page_pool: *mut page_pool,
}

extern "C" {
    pub fn hinic3_alloc_rxqs(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn hinic3_free_rxqs(netdev: *mut net_device);
}
extern "C" {
    pub fn hinic3_rx_poll(rxq: *mut hinic3_rxq, budget: c_int) -> c_int;
}
