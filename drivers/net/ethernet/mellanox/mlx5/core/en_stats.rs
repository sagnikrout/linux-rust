//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_stats.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_desc {
    pub format: [c_char; ETH_GSTRING_LEN],
    pub /: *mut *mut size_t offset; / Byte offset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_stats_grp {
    pub update_stats_mask: u16,
    pub priv): *mut *mut int (get_num_stats)(struct mlx5e_priv,
    pub data): *mut *mut *mut void (fill_strings)(struct mlx5e_priv priv, u8,
    pub data): *mut *mut *mut void (fill_stats)(struct mlx5e_priv priv, u64,
    pub priv): *mut *mut void (update_stats)(struct mlx5e_priv,
}

extern "C" {
    pub fn mlx5e_ethtool_put_stat(data: *mut u64, val: u64);
}
pub type mlx5e_stats_grp_t = *const mlx5e_stats_grp const;

extern "C" {
    pub fn mlx5e_stats_total_num(priv: *mut mlx5e_priv) -> c_uint;
}
extern "C" {
    pub fn mlx5e_stats_update(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_stats_fill(priv: *mut mlx5e_priv, data: *mut u64, idx: c_int);
}
extern "C" {
    pub fn mlx5e_stats_fill_strings(priv: *mut mlx5e_priv, data: *mut u8);
}
extern "C" {
    pub fn mlx5e_stats_update_ndo_stats(priv: *mut mlx5e_priv);
}
// Concrete NIC Stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_sw_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_tso_packets: u64,
    pub tx_tso_bytes: u64,
    pub tx_tso_inner_packets: u64,
    pub tx_tso_inner_bytes: u64,
    pub tx_added_vlan_packets: u64,
    pub tx_nop: u64,
    pub tx_mpwqe_blks: u64,
    pub tx_mpwqe_pkts: u64,
    pub rx_lro_packets: u64,
    pub rx_lro_bytes: u64,
    pub rx_gro_packets: u64,
    pub rx_gro_bytes: u64,
    pub rx_gro_skbs: u64,
    pub rx_gro_large_hds: u64,
    pub rx_hds_nodata_packets: u64,
    pub rx_hds_nodata_bytes: u64,
    pub rx_hds_nosplit_packets: u64,
    pub rx_hds_nosplit_bytes: u64,
    pub rx_mcast_packets: u64,
    pub rx_ecn_mark: u64,
    pub rx_removed_vlan_packets: u64,
    pub rx_csum_unnecessary: u64,
    pub rx_csum_none: u64,
    pub rx_csum_complete: u64,
    pub rx_csum_complete_tail: u64,
    pub rx_csum_complete_tail_slow: u64,
    pub rx_csum_unnecessary_inner: u64,
    pub rx_xdp_drop: u64,
    pub rx_xdp_redirect: u64,
    pub rx_xdp_tx_xmit: u64,
    pub rx_xdp_tx_mpwqe: u64,
    pub rx_xdp_tx_inlnw: u64,
    pub rx_xdp_tx_nops: u64,
    pub rx_xdp_tx_full: u64,
    pub rx_xdp_tx_err: u64,
    pub rx_xdp_tx_cqe: u64,
    pub tx_csum_none: u64,
    pub tx_csum_partial: u64,
    pub tx_csum_partial_inner: u64,
    pub tx_queue_stopped: u64,
    pub tx_queue_dropped: u64,
    pub tx_xmit_more: u64,
    pub tx_recover: u64,
    pub tx_cqes: u64,
    pub tx_queue_wake: u64,
    pub tx_cqe_err: u64,
    pub tx_xdp_xmit: u64,
    pub tx_xdp_mpwqe: u64,
    pub tx_xdp_inlnw: u64,
    pub tx_xdp_nops: u64,
    pub tx_xdp_full: u64,
    pub tx_xdp_err: u64,
    pub tx_xdp_cqes: u64,
    pub rx_wqe_err: u64,
    pub rx_mpwqe_filler_cqes: u64,
    pub rx_mpwqe_filler_strides: u64,
    pub rx_oversize_pkts_sw_drop: u64,
    pub rx_buff_alloc_err: u64,
    pub rx_cqe_compress_blks: u64,
    pub rx_cqe_compress_pkts: u64,
    pub rx_congst_umr: u64,

    pub rx_arfs_add: u64,
    pub rx_arfs_request_in: u64,
    pub rx_arfs_request_out: u64,
    pub rx_arfs_expired: u64,
    pub rx_arfs_err: u64,

    pub rx_recover: u64,
    pub ch_events: u64,
    pub ch_poll: u64,
    pub ch_arm: u64,
    pub ch_aff_change: u64,
    pub ch_force_irq: u64,
    pub ch_eq_rearm: u64,
    pub rx_pp_alloc_fast: u64,
    pub rx_pp_alloc_slow: u64,
    pub rx_pp_alloc_slow_high_order: u64,
    pub rx_pp_alloc_empty: u64,
    pub rx_pp_alloc_refill: u64,
    pub rx_pp_alloc_waive: u64,
    pub rx_pp_recycle_cached: u64,
    pub rx_pp_recycle_cache_full: u64,
    pub rx_pp_recycle_ring: u64,
    pub rx_pp_recycle_ring_full: u64,
    pub rx_pp_recycle_released_ref: u64,

    pub tx_tls_encrypted_packets: u64,
    pub tx_tls_encrypted_bytes: u64,
    pub tx_tls_ooo: u64,
    pub tx_tls_dump_packets: u64,
    pub tx_tls_dump_bytes: u64,
    pub tx_tls_resync_bytes: u64,
    pub tx_tls_skip_no_sync_data: u64,
    pub tx_tls_drop_no_sync_data: u64,
    pub tx_tls_drop_bypass_req: u64,
    pub rx_tls_decrypted_packets: u64,
    pub rx_tls_decrypted_bytes: u64,
    pub rx_tls_resync_req_pkt: u64,
    pub rx_tls_resync_req_start: u64,
    pub rx_tls_resync_req_end: u64,
    pub rx_tls_resync_req_skip: u64,
    pub rx_tls_resync_res_ok: u64,
    pub rx_tls_resync_res_retry: u64,
    pub rx_tls_resync_res_skip: u64,
    pub rx_tls_err: u64,

    pub rx_xsk_packets: u64,
    pub rx_xsk_bytes: u64,
    pub rx_xsk_csum_complete: u64,
    pub rx_xsk_csum_unnecessary: u64,
    pub rx_xsk_csum_unnecessary_inner: u64,
    pub rx_xsk_csum_none: u64,
    pub rx_xsk_ecn_mark: u64,
    pub rx_xsk_removed_vlan_packets: u64,
    pub rx_xsk_xdp_drop: u64,
    pub rx_xsk_xdp_redirect: u64,
    pub rx_xsk_wqe_err: u64,
    pub rx_xsk_mpwqe_filler_cqes: u64,
    pub rx_xsk_mpwqe_filler_strides: u64,
    pub rx_xsk_oversize_pkts_sw_drop: u64,
    pub rx_xsk_buff_alloc_err: u64,
    pub rx_xsk_cqe_compress_blks: u64,
    pub rx_xsk_cqe_compress_pkts: u64,
    pub rx_xsk_congst_umr: u64,
    pub tx_xsk_xmit: u64,
    pub tx_xsk_mpwqe: u64,
    pub tx_xsk_inlnw: u64,
    pub tx_xsk_full: u64,
    pub tx_xsk_err: u64,
    pub tx_xsk_cqes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_qcounter_stats {
    pub rx_out_of_buffer: u32,
    pub rx_if_down_packets: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_vnic_env_stats {
    pub query_vnic_env_out: [__be64; MLX5_ST_SZ_QW(query_vnic_env_out)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_vport_stats {
    pub query_vport_out: [__be64; MLX5_ST_SZ_QW(query_vport_counter_out)],
}

pub const NUM_PPORT_PRIO: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_pport_stats {
    pub IEEE_802_3_counters: [__be64; MLX5_ST_SZ_QW(ppcnt_reg)],
    pub RFC_2863_counters: [__be64; MLX5_ST_SZ_QW(ppcnt_reg)],
    pub RFC_2819_counters: [__be64; MLX5_ST_SZ_QW(ppcnt_reg)],
    pub per_prio_counters: [__be64; NUM_PPORT_PRIO][MLX5_ST_SZ_QW(ppcnt_reg)],
    pub phy_counters: [__be64; MLX5_ST_SZ_QW(ppcnt_reg)],
    pub phy_statistical_counters: [__be64; MLX5_ST_SZ_QW(ppcnt_reg)],
    pub phy_recovery_counters: [__be64; MLX5_ST_SZ_QW(ppcnt_reg)],
    pub eth_ext_counters: [__be64; MLX5_ST_SZ_QW(ppcnt_reg)],
    pub per_tc_prio_counters: [__be64; NUM_PPORT_PRIO][MLX5_ST_SZ_QW(ppcnt_reg)],
    pub per_tc_congest_prio_counters: [__be64; NUM_PPORT_PRIO][MLX5_ST_SZ_QW(ppcnt_reg)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_pcie_stats {
    pub pcie_perf_counters: [__be64; MLX5_ST_SZ_QW(mpcnt_reg)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rq_stats {
    pub packets: u64,
    pub bytes: u64,
    pub csum_complete: u64,
    pub csum_complete_tail: u64,
    pub csum_complete_tail_slow: u64,
    pub csum_unnecessary: u64,
    pub csum_unnecessary_inner: u64,
    pub csum_none: u64,
    pub lro_packets: u64,
    pub lro_bytes: u64,
    pub gro_packets: u64,
    pub gro_bytes: u64,
    pub gro_skbs: u64,
    pub gro_large_hds: u64,
    pub hds_nodata_packets: u64,
    pub hds_nodata_bytes: u64,
    pub hds_nosplit_packets: u64,
    pub hds_nosplit_bytes: u64,
    pub mcast_packets: u64,
    pub ecn_mark: u64,
    pub removed_vlan_packets: u64,
    pub xdp_drop: u64,
    pub xdp_redirect: u64,
    pub wqe_err: u64,
    pub mpwqe_filler_cqes: u64,
    pub mpwqe_filler_strides: u64,
    pub oversize_pkts_sw_drop: u64,
    pub buff_alloc_err: u64,
    pub cqe_compress_blks: u64,
    pub cqe_compress_pkts: u64,
    pub congst_umr: u64,

    pub arfs_add: u64,
    pub arfs_request_in: u64,
    pub arfs_request_out: u64,
    pub arfs_expired: u64,
    pub arfs_err: u64,

    pub recover: u64,
    pub pp_alloc_fast: u64,
    pub pp_alloc_slow: u64,
    pub pp_alloc_slow_high_order: u64,
    pub pp_alloc_empty: u64,
    pub pp_alloc_refill: u64,
    pub pp_alloc_waive: u64,
    pub pp_recycle_cached: u64,
    pub pp_recycle_cache_full: u64,
    pub pp_recycle_ring: u64,
    pub pp_recycle_ring_full: u64,
    pub pp_recycle_released_ref: u64,

    pub tls_decrypted_packets: u64,
    pub tls_decrypted_bytes: u64,
    pub tls_resync_req_pkt: u64,
    pub tls_resync_req_start: u64,
    pub tls_resync_req_end: u64,
    pub tls_resync_req_skip: u64,
    pub tls_resync_res_ok: u64,
    pub tls_resync_res_retry: u64,
    pub tls_resync_res_skip: u64,
    pub tls_err: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_sq_stats {
// commonly accessed in data path
    pub packets: u64,
    pub bytes: u64,
    pub xmit_more: u64,
    pub tso_packets: u64,
    pub tso_bytes: u64,
    pub tso_inner_packets: u64,
    pub tso_inner_bytes: u64,
    pub csum_partial: u64,
    pub csum_partial_inner: u64,
    pub added_vlan_packets: u64,
    pub nop: u64,
    pub mpwqe_blks: u64,
    pub mpwqe_pkts: u64,

    pub tls_encrypted_packets: u64,
    pub tls_encrypted_bytes: u64,
    pub tls_ooo: u64,
    pub tls_dump_packets: u64,
    pub tls_dump_bytes: u64,
    pub tls_resync_bytes: u64,
    pub tls_skip_no_sync_data: u64,
    pub tls_drop_no_sync_data: u64,
    pub tls_drop_bypass_req: u64,

// less likely accessed in data path
    pub csum_none: u64,
    pub stopped: u64,
    pub dropped: u64,
    pub recover: u64,
    pub timestamps: u64,
// dirtied @completion
    pub ____cacheline_aligned_in_smp: u64 cqes,
    pub wake: u64,
    pub cqe_err: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xdpsq_stats {
    pub xmit: u64,
    pub mpwqe: u64,
    pub inlnw: u64,
    pub nops: u64,
    pub full: u64,
    pub err: u64,
// dirtied @completion
    pub ____cacheline_aligned_in_smp: u64 cqes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ch_stats {
    pub events: u64,
    pub poll: u64,
    pub arm: u64,
    pub aff_change: u64,
    pub force_irq: u64,
    pub eq_rearm: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ptp_cq_stats {
    pub cqe: u64,
    pub err_cqe: u64,
    pub abort: u64,
    pub abort_abs_diff_ns: u64,
    pub late_cqe: u64,
    pub lost_cqe: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rep_stats {
    pub vport_rx_packets: u64,
    pub vport_tx_packets: u64,
    pub vport_rx_bytes: u64,
    pub vport_tx_bytes: u64,
    pub rx_vport_rdma_unicast_packets: u64,
    pub tx_vport_rdma_unicast_packets: u64,
    pub rx_vport_rdma_unicast_bytes: u64,
    pub tx_vport_rdma_unicast_bytes: u64,
    pub rx_vport_rdma_multicast_packets: u64,
    pub tx_vport_rdma_multicast_packets: u64,
    pub rx_vport_rdma_multicast_bytes: u64,
    pub tx_vport_rdma_multicast_bytes: u64,
    pub vport_loopback_packets: u64,
    pub vport_loopback_bytes: u64,
    pub rx_vport_out_of_buffer: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_stats {
    pub sw: mlx5e_sw_stats,
    pub qcnt: mlx5e_qcounter_stats,
    pub vnic: mlx5e_vnic_env_stats,
    pub vport: mlx5e_vport_stats,
    pub pport: mlx5e_pport_stats,
    pub pcie: mlx5e_pcie_stats,
    pub rep_stats: mlx5e_rep_stats,
}

extern "C" {
    pub fn mlx5e_nic_stats_grps_num(priv: *mut mlx5e_priv) -> c_uint;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: sw) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: qcnt) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: vnic_env) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: vport) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: 802_3) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: 2863) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: 2819) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: phy) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: eth_ext) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: pcie) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: per_prio) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: pme) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: channels) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: per_port_buff_congest) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: ipsec_hw) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: ipsec_sw) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: ptp) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: macsec_hw) -> extern;
}
extern "C" {
    pub fn MLX5E_DECLARE_STATS_GRP(_arg: pcie_cong) -> extern;
}
