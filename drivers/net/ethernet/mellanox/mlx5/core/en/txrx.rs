//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/txrx.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2019 Mellanox Technologies.

// IPSEC inline data includes:
// 1. ESP trailer: up to 255 bytes of padding, 1 byte for pad length, 1 byte for
// next header.
// 2. ESP authentication data: 16 bytes for ICV.
//

// 366 should be big enough to cover all L2, L3 and L4 headers with possible
// encapsulations.
//

// Sync the calculation with mlx5e_sq_calc_wqe_attr.

// Macro flag: #define MLX5E_KSM_UMR_WQE_SZ(sgl_len)\

// Macro flag: #define MLX5E_KSM_UMR_DS_CNT(ksm_entries)\
// Macro flag: #define MLX5E_KSM_MAX_ENTRIES_PER_WQE(wqe_size)\
// Macro flag: #define MLX5E_KSM_ENTRIES_PER_WQE(wqe_size)\

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_icosq_wqe_type {
    MLX5E_ICOSQ_WQE_NOP,
    MLX5E_ICOSQ_WQE_UMR_RX,

    MLX5E_ICOSQ_WQE_UMR_TLS,
    MLX5E_ICOSQ_WQE_SET_PSV_TLS,
    MLX5E_ICOSQ_WQE_GET_PSV_TLS,

}

// General
extern "C" {
    pub fn mlx5e_trigger_irq(sq: *mut mlx5e_icosq);
}
extern "C" {
    pub fn mlx5e_completion_event(mcq: *mut mlx5_core_cq, eqe: *mut mlx5_eqe);
}
extern "C" {
    pub fn mlx5e_cq_error_event(mcq: *mut mlx5_core_cq, event: mlx5_event);
}
extern "C" {
    pub fn mlx5e_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_poll_ico_cq(cq: *mut mlx5e_cq) -> c_int;
}
// RX
extern "C" {
    pub fn mlx5e_poll_rx_cq(cq: *mut mlx5e_cq, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_free_rx_descs(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_free_rx_missing_descs(rq: *mut mlx5e_rq);
}
// TX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xmit_data {
    pub dma_addr: dma_addr_t,
    pub data: *mut c_void,
    pub 31: u32 len :,
    pub 1: u32 has_frags :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xmit_data_frags {
    pub xd: mlx5e_xmit_data,
    pub sinfo: *mut skb_shared_info,
    pub dma_arr: *mut dma_addr_t,
}

extern "C" {
    pub fn mlx5e_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn mlx5e_poll_tx_cq(cq: *mut mlx5e_cq, napi_budget: c_int) -> bool;
}
extern "C" {
    pub fn mlx5e_free_txqsq_descs(sq: *mut mlx5e_txqsq);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tx_wqe_info {
    pub skb: *mut sk_buff,
    pub num_bytes: u32,
    pub num_wqebbs: u8,
    pub num_dma: u8,
    pub num_fifo_pkts: u8,

    pub resync_dump_frag_page: *mut page,

}

// Fill SQ frag edge with NOPs to avoid WQE wrapping two pages.
// wi = (struct mlx5e_tx_wqe_info) {
// size = min_t(u16, contig_wqebbs, sq->max_sq_mpw_wqebbs);
extern "C" {
    pub fn mlx5e_txqsq_wake(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn be16_to_cpu(1: cqe->shampo.header_entry_index) & (rq->mpwqe.shampo->hd_per_wq -) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_shampo_umr {
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_icosq_wqe_info {
    pub wqe_type: u8,
    pub num_wqebbs: u8,
// Auxiliary data for different wqe types.
    pub rq: *mut mlx5e_rq,
    pub umr: },
    pub shampo: mlx5e_shampo_umr,

    pub priv_rx: *mut mlx5e_ktls_offload_context_rx,
    pub tls_set_params: },
    pub buf: *mut mlx5e_ktls_rx_resync_buf,
    pub tls_get_params: },

}

extern "C" {
    pub fn mlx5e_free_icosq_descs(sq: *mut mlx5e_icosq);
}
// Fill SQ frag edge with NOPs to avoid WQE wrapping two pages.
// wi = (struct mlx5e_icosq_wqe_info) {
// ensure wqe is visible to device before updating doorbell record
// wq->db = cpu_to_be32(pc);
// ensure doorbell record is visible to device before ringing the
// doorbell
//
// skb_item = skb;
extern "C" {
    pub fn mlx5e_tx_mpwqe_ensure_complete(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn mlx5_wq_ll_get_size(_arg: &rq->mpwqe.wq) -> return;
}
extern "C" {
    pub fn mlx5_wq_cyc_get_size(_arg: &rq->wqe.wq) -> return;
}
extern "C" {
    pub fn mlx5_wq_ll_get_head(_arg: &rq->mpwqe.wq) -> return;
}
extern "C" {
    pub fn mlx5_wq_cyc_get_head(_arg: &rq->wqe.wq) -> return;
}
extern "C" {
    pub fn mlx5_wq_ll_get_counter(_arg: &rq->mpwqe.wq) -> return;
}
extern "C" {
    pub fn mlx5_wq_cyc_get_counter(_arg: &rq->wqe.wq) -> return;
}
// SW parser related functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_swp_spec {
    pub l3_proto: __be16,
    pub l4_proto: u8,
    pub is_tun: u8,
    pub tun_l3_proto: __be16,
    pub tun_l4_proto: u8,
}

// SWP offsets are in 2-bytes words

// A WQE must not cross the page boundary, hence two conditions:
// 1. Its size must not exceed the page size.
// 2. If the WQE size is X, and the space remaining in a page is less
// than X, this space needs to be padded with NOPs. So, one WQE of
// size X may require up to X-1 WQEBBs of padding, which makes the
// stop room of X-1 + X.
// WQE size is also limited by the hardware limit.
//
extern "C" {
    pub fn MLX5E_STOP_ROOM(_arg: wqe_size) -> return;
}
extern "C" {
    pub fn MLX5E_STOP_ROOM(_arg: mlx5e_get_max_sq_wqebbs(mdev)) -> return;
}
extern "C" {
    pub fn mlx5e_stop_room_for_wqe(_arg: mdev, _arg: mpwqe_wqebbs) -> return;
}
extern "C" {
    pub fn mlx5e_wqc_has_room_for(_arg: &sq->wq, _arg: sq->cc, _arg: sq->pc, _arg: room) -> return;
}
