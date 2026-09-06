//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en.h
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


//
// Copyright (c) 2015-2016, Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const MLX5E_METADATA_ETHER_LEN: c_int = 8;

// Keep in sync with mlx5e_mpwrq_log_wqe_sz.
// These are theoretical maximums, which can be further restricted by
// capabilities. These values are used for static resource allocations and
// sanity checks.
// MLX5_SEND_WQE_MAX_SIZE is a bit bigger than the maximum cacheline-aligned WQE
// size actually used at runtime, but it's not a problem when calculating static
// array sizes.
//

pub const MLX5E_PARAMS_MINIMUM_LOG_SQ_SIZE: c_uint = 0x6;
pub const MLX5E_PARAMS_DEFAULT_LOG_SQ_SIZE: c_uint = 0xa;
pub const MLX5E_PARAMS_MAXIMUM_LOG_SQ_SIZE: c_uint = 0xd;

pub const MLX5E_PARAMS_DEFAULT_LOG_RQ_SIZE: c_uint = 0xa;
pub const MLX5E_PARAMS_MAXIMUM_LOG_RQ_SIZE: c_uint = 0xd;
pub const MLX5E_PARAMS_MINIMUM_LOG_RQ_SIZE_MPW: c_uint = 0x2;
pub const MLX5E_DEFAULT_LRO_TIMEOUT: c_int = 32;
pub const MLX5E_DEFAULT_SHAMPO_TIMEOUT: c_int = 1024;
pub const MLX5E_PARAMS_DEFAULT_RX_CQ_MODERATION_USEC: c_uint = 0x10;
pub const MLX5E_PARAMS_DEFAULT_RX_CQ_MODERATION_USEC_FROM_CQE: c_uint = 0x3;
pub const MLX5E_PARAMS_DEFAULT_RX_CQ_MODERATION_PKTS: c_uint = 0x20;
pub const MLX5E_PARAMS_DEFAULT_TX_CQ_MODERATION_USEC: c_uint = 0x10;
pub const MLX5E_PARAMS_DEFAULT_TX_CQ_MODERATION_USEC_FROM_CQE: c_uint = 0x10;
pub const MLX5E_PARAMS_DEFAULT_TX_CQ_MODERATION_PKTS: c_uint = 0x20;
pub const MLX5E_PARAMS_DEFAULT_MIN_RX_WQES: c_uint = 0x80;
pub const MLX5E_PARAMS_DEFAULT_MIN_RX_WQES_MPW: c_uint = 0x2;
pub const MLX5E_MIN_NUM_CHANNELS: c_uint = 0x1;
pub const MLX5E_MAX_NUM_CHANNELS: c_int = 256;
pub const MLX5E_TX_CQ_POLL_BUDGET: c_int = 128;
pub const MLX5E_TX_XSK_POLL_BUDGET: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_devcom_events {
    MPV_DEVCOM_MASTER_UP,
    MPV_DEVCOM_MASTER_DOWN,
    MPV_DEVCOM_IPSEC_MASTER_UP,
    MPV_DEVCOM_IPSEC_MASTER_DOWN,
}

extern "C" {
    pub fn clamp_t(_arg: u8, _arg: MLX5_CAP_GEN(mdev, _arg: num_lag_ports), _arg: 1, _arg: MLX5_MAX_PORTS) -> return;
}
// Use this function to get max num channels (rxqs/txqs) only to create netdev
// The maximum WQE size can be retrieved by max_wqe_sz_sq in
// bytes units. Driver hardens the limitation to 1KB (16
// WQEBBs), unless firmware capability is stricter.
//
// The return value will be multiplied by MLX5_SEND_WQEBB_NUM_DS.
// Since max_sq_wqebbs may be up to MLX5_SEND_WQE_MAX_WQEBBS == 16,
// see mlx5e_get_max_sq_wqebbs(), the multiplication (16 * 4 == 64)
// overflows the 6-bit DS field of Ctrl Segment. Use a bound lower
// than MLX5_SEND_WQE_MAX_WQEBBS to let a full-session WQE be
// cache-aligned.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tx_wqe {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub eth: mlx5_wqe_eth_seg,
    pub data: [mlx5_wqe_data_seg; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rx_wqe_ll {
    pub next: mlx5_wqe_srq_next_seg,
    pub data: [mlx5_wqe_data_seg; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rx_wqe_cyc {
    pub data): DECLARE_FLEX_ARRAY(struct mlx5_wqe_data_seg,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_umr_wqe_hdr {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub uctrl: mlx5_wqe_umr_ctrl_seg,
    pub mkc: mlx5_mkey_seg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_umr_wqe {
    pub hdr: mlx5e_umr_wqe_hdr,
    pub inline_mtts): DECLARE_FLEX_ARRAY(struct mlx5_mtt,,
    pub inline_klms): DECLARE_FLEX_ARRAY(struct mlx5_klm,,
    pub inline_ksms): DECLARE_FLEX_ARRAY(struct mlx5_ksm,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_priv_flag {
    MLX5E_PFLAG_RX_CQE_BASED_MODER,
    MLX5E_PFLAG_TX_CQE_BASED_MODER,
    MLX5E_PFLAG_RX_CQE_COMPRESS,
    MLX5E_PFLAG_RX_STRIDING_RQ,
    MLX5E_PFLAG_RX_NO_CSUM_COMPLETE,
    MLX5E_PFLAG_XDP_TX_MPWQE,
    MLX5E_PFLAG_SKB_TX_MPWQE,
    MLX5E_PFLAG_TX_PORT_TS,
    MLX5E_NUM_PFLAGS, /* Keep last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum packet_merge {
    MLX5E_PACKET_MERGE_NONE,
    MLX5E_PACKET_MERGE_LRO,
    MLX5E_PACKET_MERGE_SHAMPO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_packet_merge_param {
    pub type: packet_merge,
    pub timeout: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_params {
    pub log_sq_size: u8,
    pub rq_wq_type: u8,
    pub log_rq_mtu_frames: u8,
    pub num_channels: u16,
    pub mode: u16,
    pub num_tc: u8,
    pub tc_to_txq: [netdev_tc_txq; TC_MAX_QUEUE],
    pub max_rate: [u64; TC_MAX_QUEUE],
    pub hw_id: [u32; TC_MAX_QUEUE],
    pub channel: },
    pub mqprio: },
    pub rx_cqe_compress_def: bool,
    pub rx_cq_moderation: dim_cq_moder,
    pub tx_cq_moderation: dim_cq_moder,
    pub packet_merge: mlx5e_packet_merge_param,
    pub tx_min_inline_mode: u8,
    pub vlan_strip_disable: bool,
    pub scatter_fcs_en: bool,
    pub rx_dim_enabled: bool,
    pub tx_dim_enabled: bool,
    pub rx_moder_use_cqe_mode: bool,
    pub tx_moder_use_cqe_mode: bool,
    pub pflags: u32,
    pub xdp_prog: *mut bpf_prog,
    pub xsk: *mut mlx5e_xsk,
    pub sw_mtu: c_uint,
    pub hard_mtu: c_int,
    pub ptp_rx: bool,
    pub terminate_lkey_be: __be32,
}

// Keep this enum consistent with the corresponding strings array
// declared in en/reporter_rx.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_cq {
// data path - accessed per cqe
    pub wq: mlx5_cqwq,
// data path - accessed per napi poll
    pub event_ctr: u16,
    pub napi: *mut napi_struct,
    pub uar: *mut mlx5_uars_page,
    pub mcq: mlx5_core_cq,
    pub ch_stats: *mut mlx5e_ch_stats,
// control
    pub netdev: *mut net_device,
    pub mdev: *mut mlx5_core_dev,
    pub workqueue: *mut workqueue_struct,
    pub wq_ctrl: mlx5_wq_ctrl,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_cq_decomp {
// cqe decompression
    pub title: mlx5_cqe64,
    pub mini_arr: [mlx5_mini_cqe8; MLX5_MINI_CQE_ARRAY_SIZE],
    pub mini_arr_idx: u8,
    pub left: u16,
    pub wqe_counter: u16,
    pub last_cqe_title: bool,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_dma_map_type {
    MLX5E_DMA_MAP_SINGLE,
    MLX5E_DMA_MAP_PAGE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_sq_dma {
    pub addr: dma_addr_t,
    pub size: u32,
    pub type: mlx5e_dma_map_type,
}

// Keep this enum consistent with the corresponding strings array
// declared in en/reporter_tx.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tx_mpwqe {
// Current MPWQE session
    pub wqe: *mut mlx5e_tx_wqe,
    pub bytes_count: u32,
    pub ds_count: u8,
    pub ds_count_max: u8,
    pub pkt_count: u8,
    pub inline_on: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_skb_fifo {
    pub fifo: *mut sk_buff,
    pub pc: *mut u16,
    pub cc: *mut u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_txqsq {
// data path
// dirtied @completion
    pub cc: u16,
    pub skb_fifo_cc: u16,
    pub dma_fifo_cc: u32,
    pub /: *mut *mut *mut dim dim; / Adaptive Moderation,
// dirtied @xmit
    pub ____cacheline_aligned_in_smp: u16 pc,
    pub skb_fifo_pc: u16,
    pub dma_fifo_pc: u32,
    pub mpwqe: mlx5e_tx_mpwqe,
    pub cq: mlx5e_cq,
// read only
    pub wq: mlx5_wq_cyc,
    pub dma_fifo_mask: u32,
    pub stats: *mut mlx5e_sq_stats,
    pub dma_fifo: *mut mlx5e_sq_dma,
    pub skb_fifo: mlx5e_skb_fifo,
    pub wqe_info: *mut mlx5e_tx_wqe_info,
    pub db: },
    pub uar_map: *mut void __iomem,
    pub txq: *mut netdev_queue,
    pub sqn: u32,
    pub stop_room: u16,
    pub max_sq_mpw_wqebbs: u8,
    pub min_inline_mode: u8,
    pub pdev: *mut device,
    pub mkey_be: __be32,
    pub state: c_ulong,
    pub hw_mtu: c_uint,
    pub clock: *mut mlx5_clock,
    pub netdev: *mut net_device,
    pub mdev: *mut mlx5_core_dev,
    pub channel: *mut mlx5e_channel,
    pub priv: *mut mlx5e_priv,
// control path
    pub wq_ctrl: mlx5_wq_ctrl,
    pub ch_ix: c_int,
    pub txq_ix: c_int,
    pub rate_limit: u32,
    pub recover_work: work_struct,
    pub ptpsq: *mut mlx5e_ptpsq,
    pub ptp_cyc2time: cqe_ts_to_ns,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xdp_info_fifo {
    pub xi: *mut mlx5e_xdp_info,
    pub cc: *mut u32,
    pub pc: *mut u32,
    pub mask: u32,
}

extern "C" {
    pub fn int(: *mut *mut mlx5e_fp_xmit_xdp_frame_check)(struct mlx5e_xdpsq) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xdpsq {
// data path
// dirtied @completion
    pub xdpi_fifo_cc: u32,
    pub cc: u16,
// dirtied @xmit
    pub ____cacheline_aligned_in_smp: u32 xdpi_fifo_pc,
    pub pc: u16,
    pub doorbell_cseg: *mut mlx5_wqe_ctrl_seg,
    pub mpwqe: mlx5e_tx_mpwqe,
    pub cq: mlx5e_cq,
// read only
    pub xsk_pool: *mut xsk_buff_pool,
    pub wq: mlx5_wq_cyc,
    pub stats: *mut mlx5e_xdpsq_stats,
    pub xmit_xdp_frame_check: mlx5e_fp_xmit_xdp_frame_check,
    pub xmit_xdp_frame: mlx5e_fp_xmit_xdp_frame,
    pub wqe_info: *mut mlx5e_xdp_wqe_info,
    pub xdpi_fifo: mlx5e_xdp_info_fifo,
    pub db: },
    pub uar_map: *mut void __iomem,
    pub sqn: u32,
    pub pdev: *mut device,
    pub mkey_be: __be32,
    pub stop_room: u16,
    pub max_sq_mpw_wqebbs: u8,
    pub min_inline_mode: u8,
    pub state: c_ulong,
    pub hw_mtu: c_uint,
// control path
    pub wq_ctrl: mlx5_wq_ctrl,
    pub channel: *mut mlx5e_channel,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xdp_buff {
    pub xdp: xdp_buff,
    pub cqe: *mut mlx5_cqe64,
    pub rq: *mut mlx5e_rq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_icosq {
// data path
    pub cc: u16,
    pub pc: u16,
    pub doorbell_cseg: *mut mlx5_wqe_ctrl_seg,
    pub cq: mlx5e_cq,
// write@xmit, read@completion
    pub wqe_info: *mut mlx5e_icosq_wqe_info,
    pub db: },
// read only
    pub wq: mlx5_wq_cyc,
    pub uar_map: *mut void __iomem,
    pub sqn: u32,
    pub reserved_room: u16,
    pub state: c_ulong,
// icosq can be accessed from any CPU and from different contexts
// (NAPI softirq or process/workqueue). Always use spin_lock_bh for
// simplicity and correctness across all contexts.
//
    pub lock: spinlock_t,
    pub ktls_resync: *mut mlx5e_ktls_resync_resp,
// control path
    pub wq_ctrl: mlx5_wq_ctrl,
    pub channel: *mut mlx5e_channel,
    pub recover_work: work_struct,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_frag_page {
    pub netmem: netmem_ref,
    pub frags: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_wqe_frag_flag {
    MLX5E_WQE_FRAG_LAST_IN_PAGE,
    MLX5E_WQE_FRAG_SKIP_RELEASE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_wqe_frag_info {
    pub frag_page: *mut mlx5e_frag_page,
    pub xskp: *mut xdp_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5e_alloc_units {
    pub frag_pages): DECLARE_FLEX_ARRAY(struct mlx5e_frag_page,,
    pub pages): *mut *mut DECLARE_FLEX_ARRAY(struct page ,,
    pub xsk_buffs): *mut *mut DECLARE_FLEX_ARRAY(struct xdp_buff ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_mpw_info {
    pub consumed_strides: u16,
    pub MLX5_MPWRQ_MAX_PAGES_PER_WQE): DECLARE_BITMAP(skip_release_bitmap,,
    pub alloc_units: mlx5e_alloc_units,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_mpw_linear_info {
    pub frag_page: mlx5e_frag_page,
    pub max_frags: u16,
}

pub const MLX5E_MAX_RX_FRAGS: c_int = 4;
extern "C" {
    pub fn void(mlx5e_rq*: *mut *mut mlx5e_fp_handle_rx_cqe)(struct, mlx5_cqe64*: *mut struct) -> typedef;
}
extern "C" {
    pub fn bool(rq: *mut *mut mlx5e_fp_post_rx_wqes)(struct mlx5e_rq) -> typedef;
}
extern "C" {
    pub fn void(mlx5e_rq*: *mut *mut mlx5e_fp_dealloc_wqe)(struct, _arg: u16) -> typedef;
}
extern "C" {
    pub fn void(mlx5e_rq*: *mut *mut mlx5e_fp_shampo_dealloc_hd)(struct, _arg: u16, _arg: u16, _arg: bool) -> typedef;
}
extern "C" {
    pub fn mlx5e_rq_set_handlers(rq: *mut mlx5e_rq, params: *mut mlx5e_params, xsk: bool) -> c_int;
}
extern "C" {
    pub fn mlx5e_rq_set_trap_handlers(rq: *mut mlx5e_rq, params: *mut mlx5e_params);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_rq_flag {
    MLX5E_RQ_FLAG_XDP_XMIT,
    MLX5E_RQ_FLAG_XDP_REDIRECT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rq_frag_info {
    pub frag_size: c_int,
    pub frag_stride: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rq_frags_info {
    pub arr: [mlx5e_rq_frag_info; MLX5E_MAX_RX_FRAGS],
    pub num_frags: u8,
    pub log_num_frags: u8,
    pub wqe_bulk: u16,
    pub refill_unit: u16,
    pub wqe_index_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_dma_info {
    pub addr: dma_addr_t,
    pub frag_page: *mut mlx5e_frag_page,
    pub page: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_shampo_hd {
    pub hd_per_wq: u32,
    pub hd_buf_size: u32,
    pub mkey: u32,
    pub nentries: u32,
    pub hd_buf_pages): DECLARE_FLEX_ARRAY(struct mlx5e_dma_info,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_hw_gro_data {
    pub skb: *mut sk_buff,
    pub fk: flow_keys,
    pub second_ip_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_mpwrq_umr_mode {
    MLX5E_MPWRQ_UMR_MODE_ALIGNED,
    MLX5E_MPWRQ_UMR_MODE_UNALIGNED,
    MLX5E_MPWRQ_UMR_MODE_OVERSIZED,
    MLX5E_MPWRQ_UMR_MODE_TRIPLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rq {
// data path
    pub wq: mlx5_wq_cyc,
    pub frags: *mut mlx5e_wqe_frag_info,
    pub alloc_units: *mut mlx5e_alloc_units,
    pub info: mlx5e_rq_frags_info,
    pub skb_from_cqe: mlx5e_fp_skb_from_cqe,
    pub wqe: },
    pub wq: mlx5_wq_ll,
    pub umr_wqe: mlx5e_umr_wqe_hdr,
    pub info: *mut mlx5e_mpw_info,
    pub skb_from_cqe_mpwrq: mlx5e_fp_skb_from_cqe_mpwrq,
    pub umr_mkey_be: __be32,
    pub num_strides: u16,
    pub actual_wq_head: u16,
    pub log_stride_sz: u8,
    pub umr_in_progress: u8,
    pub umr_last_bulk: u8,
    pub umr_completed: u8,
    pub min_wqe_bulk: u8,
    pub page_shift: u8,
    pub pages_per_wqe: u8,
    pub umr_wqebbs: u8,
    pub mtts_per_wqe: u8,
    pub umr_mode: u8,
    pub linear_info: *mut mlx5e_mpw_linear_info,
    pub shampo: *mut mlx5e_shampo_hd,
    pub mpwqe: },
}

// XDP
// page pools
// AF_XDP zero-copy
// control
// XDP read-mostly
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_channel_state {
    MLX5E_CHANNEL_STATE_XSK,
    MLX5E_CHANNEL_NUM_STATES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_channel {
// data path
    pub rq: mlx5e_rq,
    pub rq_xdpsq: mlx5e_xdpsq,
    pub sq: [mlx5e_txqsq; MLX5_MAX_NUM_TC],
    pub /: *mut *mut mlx5e_icosq icosq; / internal control operations,
    pub qos_sqs: *mut *mut mlx5e_txqsq __rcu  __rcu,
    pub xdp: bool,
    pub napi: napi_struct,
    pub pdev: *mut device,
    pub netdev: *mut net_device,
    pub mkey_be: __be32,
    pub qos_sqs_size: u16,
    pub num_tc: u8,
    pub lag_port: u8,
// XDP_REDIRECT
    pub xdpsq: *mut mlx5e_xdpsq,
// AF_XDP zero-copy
    pub xskrq: mlx5e_rq,
    pub xsksq: mlx5e_xdpsq,
// Async ICOSQ
    pub async_icosq: *mut mlx5e_icosq,
// data path - accessed per napi poll
    pub aff_mask: *const cpumask,
    pub stats: *mut mlx5e_ch_stats,
// control
    pub priv: *mut mlx5e_priv,
    pub mdev: *mut mlx5_core_dev,
    pub MLX5E_CHANNEL_NUM_STATES): DECLARE_BITMAP(state,,
    pub ix: c_int,
    pub vec_ix: c_int,
    pub sd_ix: c_int,
    pub cpu: c_int,
    pub bfreg: *mut mlx5_sq_bfreg,
// Sync between icosq recovery and XSK enable/disable.
    pub icosq_recovery_lock: mutex,
// coalescing configuration
    pub rx_cq_moder: dim_cq_moder,
    pub tx_cq_moder: dim_cq_moder,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_channels {
    pub c: *mut mlx5e_channel,
    pub ptp: *mut mlx5e_ptp,
    pub num: c_uint,
    pub params: mlx5e_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_channel_stats {
    pub ch: mlx5e_ch_stats,
    pub sq: [mlx5e_sq_stats; MLX5_MAX_NUM_TC],
    pub rq: mlx5e_rq_stats,
    pub xskrq: mlx5e_rq_stats,
    pub rq_xdpsq: mlx5e_xdpsq_stats,
    pub xdpsq: mlx5e_xdpsq_stats,
    pub xsksq: mlx5e_xdpsq_stats,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ptp_stats {
    pub ch: mlx5e_ch_stats,
    pub sq: [mlx5e_sq_stats; MLX5_MAX_NUM_TC],
    pub cq: [mlx5e_ptp_cq_stats; MLX5_MAX_NUM_TC],
    pub rq: mlx5e_rq_stats,
    pub ____cacheline_aligned_in_smp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_modify_sq_param {
    pub curr_state: c_int,
    pub next_state: c_int,
    pub rl_update: c_int,
    pub rl_index: c_int,
    pub qos_update: bool,
    pub qos_queue_group_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_hv_vhca_stats_agent {
    pub agent: *mut mlx5_hv_vhca_agent,
    pub work: delayed_work,
    pub delay: u16,
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xsk {
// XSK buffer pools are stored separately from channels,
// because we don't want to lose them when channels are
// recreated. The kernel also stores buffer pool, but it doesn't
// distinguish between zero-copy and non-zero-copy UMEMs, so
// rely on our mechanism.
//
    pub pools: *mut xsk_buff_pool,
    pub refcnt: u16,
    pub ever_used: bool,
}

// Temporary storage for variables that are allocated when struct mlx5e_priv is
// initialized, and used where we can't allocate them because that functions
// must not fail. Use with care and make sure the same variable is not used
// simultaneously by multiple users.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_scratchpad {
    pub cpumask: cpumask_var_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_priv {
// priv data path fields - start
    pub selq: mlx5e_selq,
    pub txq2sq: *mut mlx5e_txqsq,
    pub txq2sq_stats: *mut mlx5e_sq_stats,

    pub dcbx_dp: mlx5e_dcbx_dp,

// priv data path fields - end
    pub state: c_ulong,
    pub /: *mut *mut mutex state_lock; / Protects Interface state,
    pub drop_rq: mlx5e_rq,
    pub channels: mlx5e_channels,
    pub rx_res: *mut mlx5e_rx_res,
    pub tx_rates: *mut u32,
    pub fs: *mut mlx5e_flow_steering,
    pub wq: *mut workqueue_struct,
    pub update_carrier_work: work_struct,
    pub set_rx_mode_work: work_struct,
    pub tx_timeout_work: work_struct,
    pub update_stats_work: work_struct,
    pub monitor_counters_work: work_struct,
    pub monitor_counters_nb: mlx5_nb,
    pub mdev: *mut mlx5_core_dev,
    pub netdev: *mut net_device,
    pub en_trap: *mut mlx5e_trap,
    pub stats: mlx5e_stats,
    pub channel_stats: *mut mlx5e_channel_stats,
    pub trap_stats: mlx5e_channel_stats,
    pub ptp_stats: mlx5e_ptp_stats,
    pub htb_qos_sq_stats: *mut mlx5e_sq_stats,
    pub htb_max_qos_sqs: u16,
    pub stats_nch: u16,
    pub max_nch: u16,
    pub max_opened_tc: u8,
    pub tx_ptp_opened: bool,
    pub rx_ptp_opened: bool,
    pub ktls_rx_was_enabled: bool,
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub q_counter: [u16; MLX5_SD_MAX_GROUP_SZ],
    pub drop_rq_q_counter: u16,
    pub events_nb: notifier_block,
    pub blocking_events_nb: notifier_block,
    pub cong_event: *mut mlx5e_pcie_cong_event,
    pub nic_info: udp_tunnel_nic_info,

    pub dcbx: mlx5e_dcbx,

    pub profile: *const mlx5e_profile,
    pub ppriv: *mut c_void,

    pub macsec: *mut mlx5e_macsec,

    pub ipsec: *mut mlx5e_ipsec,

    pub psp: *mut mlx5e_psp,

    pub tls: *mut mlx5e_tls,

    pub tx_reporter: *mut devlink_health_reporter,
    pub rx_reporter: *mut devlink_health_reporter,
    pub xsk: mlx5e_xsk,

    pub stats_agent: mlx5e_hv_vhca_stats_agent,

    pub scratchpad: mlx5e_scratchpad,
    pub htb: *mut mlx5e_htb,
    pub mqprio_rl: *mut mlx5e_mqprio_rl,
    pub dfs_root: *mut dentry,
    pub devcom: *mut mlx5_devcom_comp_dev,
}

// Pairs with smp_store_release in mlx5e_stats_nch_write().
extern "C" {
    pub fn smp_load_acquire(_arg: &priv->stats_nch) -> return;
}
// Pairs with smp_load_acquire in mlx5e_stats_nch_read().
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_dev {
    pub netdev: *mut net_device,
    pub dl_port: devlink_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rx_handlers {
    pub handle_rx_cqe: mlx5e_fp_handle_rx_cqe,
    pub handle_rx_cqe_mpwqe: mlx5e_fp_handle_rx_cqe,
    pub handle_rx_cqe_mpwqe_shampo: mlx5e_fp_handle_rx_cqe,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_profile_feature {
    MLX5E_PROFILE_FEATURE_PTP_RX,
    MLX5E_PROFILE_FEATURE_PTP_TX,
    MLX5E_PROFILE_FEATURE_QOS_HTB,
    MLX5E_PROFILE_FEATURE_FS_VLAN,
    MLX5E_PROFILE_FEATURE_FS_TC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_profile {
    pub netdev): *mut net_device,
    pub priv): *mut *mut void (cleanup)(struct mlx5e_priv,
    pub priv): *mut *mut int (init_rx)(struct mlx5e_priv,
    pub priv): *mut *mut void (cleanup_rx)(struct mlx5e_priv,
    pub priv): *mut *mut int (init_tx)(struct mlx5e_priv,
    pub priv): *mut *mut void (cleanup_tx)(struct mlx5e_priv,
    pub priv): *mut *mut int (enable)(struct mlx5e_priv,
    pub priv): *mut *mut void (disable)(struct mlx5e_priv,
    pub priv): *mut *mut int (update_rx)(struct mlx5e_priv,
    pub priv): *mut *mut void (update_stats)(struct mlx5e_priv,
    pub priv): *mut *mut void (update_carrier)(struct mlx5e_priv,
    pub mdev): *mut *mut int (max_nch_limit)(struct mlx5_core_dev,
    pub tc): u8 lag_port, u8,
    pub priv): *mut *mut unsigned int (stats_grps_num)(struct mlx5e_priv,
    pub stats_grps: *mut mlx5e_stats_grp_t,
    pub rx_handlers: *const mlx5e_rx_handlers,
    pub max_tc: c_int,
    pub features: u32,
}

extern "C" {
    pub fn mlx5e_build_ptys2ethtool_map();
}
extern "C" {
    pub fn mlx5e_get_stats(dev: *mut net_device, stats: *mut rtnl_link_stats64);
}
extern "C" {
    pub fn mlx5e_fold_sw_stats64(priv: *mut mlx5e_priv, s: *mut rtnl_link_stats64);
}
extern "C" {
    pub fn mlx5e_self_test_num(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_self_test_fill_strings(priv: *mut mlx5e_priv, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5e_set_rx_mode_work(work: *mut work_struct);
}
extern "C" {
    pub fn mlx5e_modify_rx_cqe_compression_locked(priv: *mut mlx5e_priv, val: bool, rx_filter: bool) -> c_int;
}
extern "C" {
    pub fn mlx5e_timestamp_init(priv: *mut mlx5e_priv);
}

extern "C" {
    pub fn mlx5e_wait_for_min_rx_wqes(rq: *mut mlx5e_rq, wait_time: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_close_rq(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_destroy_rq(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_mpwqe_dealloc_linear_page(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_close_xdpsq(sq: *mut mlx5e_xdpsq);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_create_cq_param {
    pub netdev: *mut net_device,
    pub wq: *mut workqueue_struct,
    pub napi: *mut napi_struct,
    pub ch_stats: *mut mlx5e_ch_stats,
    pub node: c_int,
    pub ix: c_int,
    pub uar: *mut mlx5_uars_page,
}

extern "C" {
    pub fn mlx5e_close_cq(cq: *mut mlx5e_cq);
}
extern "C" {
    pub fn mlx5e_open_locked(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx5e_close_locked(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx5e_trigger_napi_icosq(c: *mut mlx5e_channel);
}
extern "C" {
    pub fn mlx5e_trigger_napi_async_icosq(c: *mut mlx5e_channel);
}
extern "C" {
    pub fn mlx5e_trigger_napi_sched(napi: *mut napi_struct);
}
extern "C" {
    pub fn mlx5e_close_channels(chs: *mut mlx5e_channels);
}
// Function pointer to be used to modify HW or kernel settings while
// switching channels
//
extern "C" {
    pub fn int(priv: *mut *mut mlx5e_fp_preactivate)(struct mlx5e_priv, context: *mut c_void) -> typedef;
}

extern "C" {
    pub fn mlx5e_safe_reopen_channels(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_update_tx_netdev_queues(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_num_channels_changed_ctx(priv: *mut mlx5e_priv, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mlx5e_update_tc_and_tx_queues_ctx(priv: *mut mlx5e_priv, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mlx5e_activate_priv_channels(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_deactivate_priv_channels(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_ptp_rx_manage_fs_ctx(priv: *mut mlx5e_priv, ctx: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mlx5e_flush_rq(rq: *mut mlx5e_rq, curr_state: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_activate_rq(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_deactivate_rq(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_activate_icosq(icosq: *mut mlx5e_icosq);
}
extern "C" {
    pub fn mlx5e_deactivate_icosq(icosq: *mut mlx5e_icosq);
}
extern "C" {
    pub fn mlx5e_activate_txqsq(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn mlx5e_deactivate_txqsq(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn mlx5e_free_txqsq(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn mlx5e_tx_disable_queue(txq: *mut netdev_queue);
}
extern "C" {
    pub fn mlx5e_alloc_txqsq_db(sq: *mut mlx5e_txqsq, numa: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_free_txqsq_db(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn mlx5e_tx_err_cqe_work(recover_work: *mut work_struct);
}
extern "C" {
    pub fn mlx5e_close_txqsq(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn mlx5e_create_mkey(mdev: *mut mlx5_core_dev, pdn: u32, mkey: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5e_create_mdev_resources(mdev: *mut mlx5_core_dev, create_tises: bool) -> c_int;
}
extern "C" {
    pub fn mlx5e_destroy_mdev_resources(mdev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5e_mkey_set_relaxed_ordering(mdev: *mut mlx5_core_dev, mkc: *mut c_void);
}
// common netdev helpers
extern "C" {
    pub fn mlx5e_create_q_counters(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_destroy_q_counters(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_close_drop_rq(drop_rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_create_tis(mdev: *mut mlx5_core_dev, in: *mut c_void, tisn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5e_destroy_tis(mdev: *mut mlx5_core_dev, tisn: u32);
}
extern "C" {
    pub fn mlx5e_update_carrier(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx5e_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx5e_queue_update_stats(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_set_dev_port_mtu(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_set_dev_port_mtu_ctx(priv: *mut mlx5e_priv, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mlx5e_vxlan_set_netdev_info(priv: *mut mlx5e_priv);
}
// ethtool helpers
extern "C" {
    pub fn mlx5e_ethtool_get_sset_count(priv: *mut mlx5e_priv, sset: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_ethtool_get_rxfh_key_size(priv: *mut mlx5e_priv) -> u32;
}
extern "C" {
    pub fn mlx5e_ethtool_get_rxfh_indir_size(priv: *mut mlx5e_priv) -> u32;
}
// mlx5e generic netdev management API
extern "C" {
    pub fn mlx5e_get_pf_num_tirs(mdev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5e_priv_cleanup(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_attach_netdev(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_detach_netdev(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_destroy_netdev(netdev: *mut net_device);
}
extern "C" {
    pub fn mlx5e_set_netdev_mtu_boundaries(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_build_nic_params(priv: *mut mlx5e_priv, xsk: *mut mlx5e_xsk, mtu: u16);
}
extern "C" {
    pub fn mlx5e_set_xdp_feature(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_set_features(netdev: *mut net_device, features: netdev_features_t) -> c_int;
}

extern "C" {
    pub fn mlx5e_set_vf_mac(dev: *mut net_device, vf: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5e_set_vf_rate(dev: *mut net_device, vf: c_int, min_tx_rate: c_int, max_tx_rate: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_get_vf_config(dev: *mut net_device, vf: c_int, ivi: *mut ifla_vf_info) -> c_int;
}
extern "C" {
    pub fn mlx5e_get_vf_stats(dev: *mut net_device, vf: c_int, vf_stats: *mut ifla_vf_stats) -> c_int;
}

extern "C" {
    pub fn mlx5e_create_mkey(mdev: *mut mlx5_core_dev, pdn: u32, mkey: *mut u32) -> c_int;
}
