//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/dp.h
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
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const MAX_RXDMA_PER_PDEV: c_int = 2;
pub const DP_MON_PURGE_TIMEOUT_MS: c_int = 100;
pub const DP_MON_SERVICE_BUDGET: c_int = 128;
pub const DP_ENCAP_TYPE_MAX: c_int = 4;
pub const DP_ENCRYPT_TYPE_MAX: c_int = 12;
pub const DP_DESC_TYPE_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_srng {
    pub vaddr_unaligned: *mut u32,
    pub vaddr: *mut u32,
    pub paddr_unaligned: dma_addr_t,
    pub paddr: dma_addr_t,
    pub size: c_int,
    pub ring_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_rxdma_mon_ring {
    pub refill_buf_ring: dp_srng,
    pub bufs_idr: idr,
// Protects bufs_idr
    pub idr_lock: spinlock_t,
    pub bufs_max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_rxdma_ring {
    pub refill_buf_ring: dp_srng,
    pub bufs_max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_tx_ring {
    pub tcl_data_ring_id: u8,
    pub tcl_data_ring: dp_srng,
    pub tcl_comp_ring: dp_srng,
    pub tx_status: *mut hal_wbm_completion_ring_tx,
    pub tx_status_head: c_int,
    pub tx_status_tail: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pdev_mon_stats {
    pub status_ppdu_state: u32,
    pub status_ppdu_start: u32,
    pub status_ppdu_end: u32,
    pub status_ppdu_compl: u32,
    pub status_ppdu_start_mis: u32,
    pub status_ppdu_end_mis: u32,
    pub status_ppdu_done: u32,
    pub dest_ppdu_done: u32,
    pub dest_mpdu_done: u32,
    pub dest_mpdu_drop: u32,
    pub dup_mon_linkdesc_cnt: u32,
    pub dup_mon_buf_cnt: u32,
    pub dest_mon_stuck: u32,
    pub dest_mon_not_reaped: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_mon_status_buf_state {
    DP_MON_STATUS_MATCH,
    DP_MON_STATUS_NO_DMA,
    DP_MON_STATUS_LAG,
    DP_MON_STATUS_LEAD,
    DP_MON_STATUS_REPLINISH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_link_desc_bank {
    pub vaddr_unaligned: *mut c_void,
    pub vaddr: *mut c_void,
    pub paddr_unaligned: dma_addr_t,
    pub paddr: dma_addr_t,
    pub size: u32,
}

// Size to enforce scatter idle list mode
pub const DP_LINK_DESC_ALLOC_SIZE_THRESH: c_uint = 0x200000;
pub const DP_LINK_DESC_BANKS_MAX: c_int = 8;
pub const DP_LINK_DESC_START: c_uint = 0x4000;
pub const DP_LINK_DESC_SHIFT: c_int = 3;

pub const DP_RX_DESC_COOKIE_INDEX_MAX: c_uint = 0x3ffff;
pub const DP_RX_DESC_COOKIE_POOL_ID_MAX: c_uint = 0x1c0000;

pub const DP_NOT_PPDU_ID_WRAP_AROUND: c_int = 20000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_dp_ppdu_state {
    DP_PPDU_STATUS_START,
    DP_PPDU_STATUS_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_mon_mpdu {
    pub list: list_head,
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
    pub err_bitmap: u32,
    pub decap_format: u8,
}

pub const DP_MON_MAX_STATUS_BUF: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_mon_data {
    pub link_desc_banks: [dp_link_desc_bank; DP_LINK_DESC_BANKS_MAX],
    pub mon_ppdu_info: hal_rx_mon_ppdu_info,
    pub mon_ppdu_status: u32,
    pub mon_last_buf_cookie: u32,
    pub mon_last_linkdesc_paddr: u64,
    pub chan_noise_floor: u16,
    pub err_bitmap: u32,
    pub decap_format: u8,
    pub rx_mon_stats: ath12k_pdev_mon_stats,
    pub buf_state: dp_mon_status_buf_state,
// lock for monitor data
    pub mon_lock: spinlock_t,
    pub rx_status_q: sk_buff_head,
    pub mon_mpdu: *mut dp_mon_mpdu,
    pub dp_rx_mon_mpdu_list: list_head,
    pub tx_prot_ppdu_info: *mut dp_mon_tx_ppdu_info,
    pub tx_data_ppdu_info: *mut dp_mon_tx_ppdu_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pdev_dp {
    pub mac_id: u32,
    pub num_tx_pending: core::sync::atomic::AtomicI32,
    pub tx_empty_waitq: wait_queue_head_t,
    pub dp: *mut ath12k_dp,
    pub hw: *mut ieee80211_hw,
    pub hw_link_id: u8,
    pub dp_hw: *mut ath12k_dp_hw,
// Protects ppdu stats
    pub ppdu_list_lock: spinlock_t,
    pub peer_tx_stats: ath12k_per_peer_tx_stats,
    pub ppdu_stats_info: list_head,
    pub ppdu_stat_list_depth: u32,
    pub rxdma_mon_dst_ring: [dp_srng; MAX_RXDMA_PER_PDEV],
    pub tx_mon_dst_ring: [dp_srng; MAX_RXDMA_PER_PDEV],
    pub rx_status: ieee80211_rx_status,
    pub mon_data: ath12k_mon_data,
}

pub const DP_NUM_CLIENTS_MAX: c_int = 64;
pub const DP_AVG_TIDS_PER_CLIENT: c_int = 2;

pub const DP_AVG_MSDUS_PER_FLOW: c_int = 128;
pub const DP_AVG_FLOWS_PER_TID: c_int = 2;
pub const DP_AVG_MPDUS_PER_TID_MAX: c_int = 128;
pub const DP_AVG_MSDUS_PER_MPDU: c_int = 4;

pub const DP_BA_WIN_SZ_MAX: c_int = 1024;
pub const DP_TCL_NUM_RING_MAX: c_int = 4;
pub const DP_IDLE_SCATTER_BUFS_MAX: c_int = 16;
pub const DP_WBM_RELEASE_RING_SIZE: c_int = 64;
pub const DP_TCL_DATA_RING_SIZE: c_int = 512;

pub const DP_TCL_CMD_RING_SIZE: c_int = 32;
pub const DP_TCL_STATUS_RING_SIZE: c_int = 32;
pub const DP_REO_DST_RING_MAX: c_int = 8;
pub const DP_REO_DST_RING_SIZE: c_int = 2048;
pub const DP_REO_REINJECT_RING_SIZE: c_int = 32;

pub const DP_REO_EXCEPTION_RING_SIZE: c_int = 128;
pub const DP_REO_CMD_RING_SIZE: c_int = 256;
pub const DP_REO_STATUS_RING_SIZE: c_int = 2048;
pub const DP_RXDMA_BUF_RING_SIZE: c_int = 4096;
pub const DP_RX_MAC_BUF_RING_SIZE: c_int = 4096;
pub const DP_RXDMA_REFILL_RING_SIZE: c_int = 2048;
pub const DP_RXDMA_ERR_DST_RING_SIZE: c_int = 1024;
pub const DP_RXDMA_MON_STATUS_RING_SIZE: c_int = 1024;

pub const DP_RXDMA_MONITOR_DESC_RING_SIZE: c_int = 4096;
pub const DP_TX_MONITOR_BUF_RING_SIZE: c_int = 4096;
pub const DP_TX_MONITOR_DEST_RING_SIZE: c_int = 2048;
pub const DP_TX_MONITOR_BUF_SIZE: c_int = 2048;
pub const DP_TX_MONITOR_BUF_SIZE_MIN: c_int = 48;
pub const DP_TX_MONITOR_BUF_SIZE_MAX: c_int = 8192;
pub const DP_RX_BUFFER_SIZE: c_int = 2048;
pub const DP_RX_BUFFER_SIZE_LITE: c_int = 1024;
pub const DP_RX_BUFFER_ALIGN_SIZE: c_int = 128;
pub const RX_MON_STATUS_BASE_BUF_SIZE: c_int = 2048;
pub const RX_MON_STATUS_BUF_ALIGN: c_int = 128;
pub const RX_MON_STATUS_BUF_RESERVATION: c_int = 128;

pub const ATH12K_SHADOW_DP_TIMER_INTERVAL: c_int = 20;
pub const ATH12K_SHADOW_CTRL_TIMER_INTERVAL: c_int = 10;

// TODO: revisit this count during testing

// Total 1024 entries in PPT, i.e 4K/4 considering 4K aligned
// SPT pages which makes lower 12bits 0
//
pub const ATH12K_MAX_PPT_ENTRIES: c_int = 1024;
// Total 512 entries in a SPT, i.e 4K Page/8
pub const ATH12K_MAX_SPT_ENTRIES: c_int = 512;

pub const ATH12K_TX_SPT_PAGE_OFFSET: c_int = 0;

// The SPT pages are divided for RX and TX, first block for RX
// and remaining for TX
//

pub const ATH12K_DP_RX_DESC_MAGIC: c_uint = 0xBABABABA;
// 4K aligned address have last 12 bits set to 0, this check is done
// so that two spt pages address can be stored per 8bytes
// of CMEM (PPT)
//
pub const ATH12K_SPT_4K_ALIGN_CHECK: c_uint = 0xFFF;
pub const ATH12K_SPT_4K_ALIGN_OFFSET: c_int = 12;

// To indicate HW of CMEM address, b0-31 are cmem base received via QMI
pub const ATH12K_CMEM_ADDR_MSB: c_uint = 0x10;
// Of 20 bits cookie, b0-b8 is to indicate SPT offset and b9-19 for PPT
pub const ATH12K_CC_SPT_MSB: c_int = 8;
pub const ATH12K_CC_PPT_MSB: c_int = 19;
pub const ATH12K_CC_PPT_SHIFT: c_int = 9;

pub const DP_MAX_PEER_ID: c_int = 2047;
// Total size of the LUT is based on 2K peers, each having reference
// for 17tids, note each entry is of type ath12k_reo_queue_ref
// hence total size is 2048 * 17 * 8 = 278528
//
pub const DP_REOQ_LUT_SIZE: c_int = 278528;
// Invalid TX Bank ID value

pub const MAX_TQM_RELEASE_REASON: c_int = 15;
pub const MAX_FW_TX_STATUS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_tx_bank_profile {
    pub is_configured: u8,
    pub num_users: u32,
    pub bank_config: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hp_update_timer {
    pub timer: timer_list,
    pub started: bool,
    pub init: bool,
    pub tx_num: u32,
    pub timer_tx_num: u32,
    pub ring_id: u32,
    pub interval: u32,
    pub ab: *mut ath12k_base,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_rx_desc_info {
    pub list: list_head,
    pub skb: *mut sk_buff,
    pub cookie: u32,
    pub magic: u32,
    pub 4: reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_tx_desc_info {
    pub list: list_head,
    pub skb: *mut sk_buff,
    pub skb_ext_desc: *mut sk_buff,
    pub /: *mut *mut u32 desc_id; / Cookie,
    pub mac_id: u8,
    pub pool_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_tx_desc_params {
    pub skb: *mut sk_buff,
    pub skb_ext_desc: *mut sk_buff,
    pub mac_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_spt_info {
    pub paddr: dma_addr_t,
    pub vaddr: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_reo_queue_ref {
    pub info0: u32,
    pub info1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_reo_q_addr_lut {
    pub vaddr_unaligned: *mut u32,
    pub vaddr: *mut u32,
    pub paddr_unaligned: dma_addr_t,
    pub paddr: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_link_stats {
    pub tx_enqueued: u32,
    pub tx_completed: u32,
    pub tx_bcast_mcast: u32,
    pub tx_dropped: u32,
    pub tx_encap_type: [u32; DP_ENCAP_TYPE_MAX],
    pub tx_encrypt_type: [u32; DP_ENCRYPT_TYPE_MAX],
    pub tx_desc_type: [u32; DP_DESC_TYPE_MAX],
}

// DP arch ops to communicate from common module
// to arch specific module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_arch_ops {
    pub budget): c_int,
    pub arvif): *mut ath12k_link_vif,
    pub status)): hal_reo_cmd_status,
    pub key_cmd): u32 cipher, enum set_key_cmd,
    pub tid): *mut *mut ath12k_dp_link_peer peer, u8,
    pub rx_tid): *mut ath12k_dp_rx_tid_rxq,
    pub action): hal_wbm_rel_bm_act,
    pub rel_link_desc): bool,
    pub update_ssn): bool,
    pub pn_type): u16 ssn, enum hal_pn_type,
    pub paddr): dma_addr_t,
    pub tid): *mut *mut *mut void (peer_rx_tid_qref_reset)(struct ath12k_base ab, u16 peer_id, u16,
    pub rx_tid): *mut ath12k_dp_rx_tid_rxq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_device_dp_tx_err_stats {
// TCL Ring Descriptor unavailable
    pub desc_na: [u32; DP_TCL_NUM_RING_MAX],
// Other failures during dp_tx due to mem allocation failure
// idr unavailable etc.
//
    pub misc_fail: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_device_dp_stats {
    pub err_ring_pkts: u32,
    pub invalid_rbm: u32,
    pub rxdma_error: [u32; HAL_REO_ENTR_RING_RXDMA_ECODE_MAX],
    pub reo_error: [u32; HAL_REO_DEST_RING_ERROR_CODE_MAX],
    pub hal_reo_error: [u32; DP_REO_DST_RING_MAX],
    pub tx_err: ath12k_device_dp_tx_err_stats,
    pub reo_rx: [u32; DP_REO_DST_RING_MAX][ATH12K_MAX_DEVICES],
    pub rx_wbm_rel_source: [u32; HAL_WBM_REL_SRC_MODULE_MAX][ATH12K_MAX_DEVICES],
    pub tqm_rel_reason: [u32; MAX_TQM_RELEASE_REASON],
    pub fw_tx_status: [u32; MAX_FW_TX_STATUS],
    pub tx_wbm_rel_source: [u32; HAL_WBM_REL_SRC_MODULE_MAX],
    pub tx_enqueued: [u32; DP_TCL_NUM_RING_MAX],
    pub tx_completed: [u32; DP_TCL_NUM_RING_MAX],
    pub reo_excep_msdu_buf_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp {
    pub ab: *mut ath12k_base,
    pub mon_dest_ring_stuck_cnt: u32,
    pub num_bank_profiles: u8,
// protects the access and update of bank_profiles
    pub tx_bank_lock: spinlock_t,
    pub bank_profiles: *mut ath12k_dp_tx_bank_profile,
    pub eid: ath12k_htc_ep_id,
    pub htt_tgt_version_received: completion,
    pub htt_tgt_ver_major: u8,
    pub htt_tgt_ver_minor: u8,
    pub link_desc_banks: [dp_link_desc_bank; DP_LINK_DESC_BANKS_MAX],
    pub idle_link_rbm: hal_rx_buf_return_buf_manager,
    pub wbm_idle_ring: dp_srng,
    pub wbm_desc_rel_ring: dp_srng,
    pub reo_reinject_ring: dp_srng,
    pub rx_rel_ring: dp_srng,
    pub reo_except_ring: dp_srng,
    pub reo_cmd_ring: dp_srng,
    pub reo_status_ring: dp_srng,
    pub peer_metadata_ver: ath12k_peer_metadata_version,
    pub reo_dst_ring: [dp_srng; DP_REO_DST_RING_MAX],
    pub tx_ring: [dp_tx_ring; DP_TCL_NUM_RING_MAX],
    pub scatter_list: [hal_wbm_idle_scatter_list; DP_IDLE_SCATTER_BUFS_MAX],
    pub reo_cmd_update_rx_queue_list: list_head,
    pub reo_cmd_cache_flush_list: list_head,
    pub reo_cmd_cache_flush_count: u32,
// protects access to below fields,
// - reo_cmd_update_rx_queue_list
// - reo_cmd_cache_flush_list
// - reo_cmd_cache_flush_count
//
    pub reo_rxq_flush_lock: spinlock_t,
    pub reo_cmd_list: list_head,
// protects access to below fields,
// - reo_cmd_list
//
    pub reo_cmd_lock: spinlock_t,
    pub reo_cmd_timer: ath12k_hp_update_timer,
    pub tx_ring_timer: [ath12k_hp_update_timer; DP_TCL_NUM_RING_MAX],
    pub spt_info: *mut ath12k_spt_info,
    pub num_spt_pages: u32,
    pub rx_ppt_base: u32,
    pub rxbaddr: *mut ath12k_rx_desc_info,
    pub txbaddr: *mut ath12k_tx_desc_info,
    pub rx_desc_free_list: list_head,
// protects the free desc list
    pub rx_desc_lock: spinlock_t,
    pub tx_desc_free_list: [list_head; ATH12K_HW_MAX_QUEUES],
    pub tx_desc_used_list: [list_head; ATH12K_HW_MAX_QUEUES],
// protects the free and used desc lists
    pub tx_desc_lock: [spinlock_t; ATH12K_HW_MAX_QUEUES],
    pub rx_refill_buf_ring: dp_rxdma_ring,
    pub rx_mac_buf_ring: [dp_srng; MAX_RXDMA_PER_PDEV],
    pub rxdma_err_dst_ring: [dp_srng; MAX_RXDMA_PER_PDEV],
    pub rxdma_mon_buf_ring: dp_rxdma_mon_ring,
    pub tx_mon_buf_ring: dp_rxdma_mon_ring,
    pub rx_mon_status_refill_ring: [dp_rxdma_mon_ring; MAX_RXDMA_PER_PDEV],
    pub reoq_lut: ath12k_reo_q_addr_lut,
    pub ml_reoq_lut: ath12k_reo_q_addr_lut,
    pub hw_params: *const ath12k_hw_params,
    pub dev: *mut device,
    pub hal: *mut ath12k_hal,
// RCU on dp_pdevs[] provides a teardown synchronization mechanism,
// ensuring in-flight data path readers complete before reclaim. Writers
// update internal fields under their own synchronization, while readers of
// internal fields may perform lockless read if occasional inconsistency
// is acceptable or use additional synchronization for a coherent view.
//
// RCU is used for dp_pdevs[] at this stage to align with
// ab->pdevs_active[]. However, if the teardown paths ensure quiescence,
// both dp_pdevs[] and pdevs_active[] can be converted to plain pointers,
// removing RCU synchronize overhead.
//
// TODO: evaluate removal of RCU from dp_pdevs in the future
//
    pub dp_pdevs: [*mut ath12k_pdev_dp __rcu; MAX_RADIOS],
    pub ag: *mut ath12k_hw_group,
    pub device_id: u8,
// Lock for protection of peers and rhead_peer_addr
    pub dp_lock: spinlock_t,
    pub ops: *const ath12k_dp_arch_ops,
// Linked list of struct ath12k_dp_link_peer
    pub peers: list_head,
// For rhash table init and deinit protection
    pub link_peer_rhash_tbl_lock: mutex,
// The rhashtable containing struct ath12k_link_peer keyed by mac addr
    pub rhead_peer_addr: *mut rhashtable,
    pub rhash_peer_addr_param: rhashtable_params,
    pub device_stats: ath12k_device_dp_stats,
}

extern "C" {
    pub fn rcu_dereference(_arg: dp->dp_pdevs[pdev_idx]) -> return;
}
extern "C" {
    pub fn ath12k_dp_vdev_tx_attach(ar: *mut ath12k, arvif: *mut ath12k_link_vif);
}
extern "C" {
    pub fn ath12k_dp_partner_cc_init(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_dp_pdev_alloc(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_dp_pdev_pre_alloc(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_dp_pdev_free(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_dp_peer_setup(ar: *mut ath12k, vdev_id: c_int, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn ath12k_dp_peer_cleanup(ar: *mut ath12k, vdev_id: c_int, addr: *const u8);
}
extern "C" {
    pub fn ath12k_dp_srng_cleanup(ab: *mut ath12k_base, ring: *mut dp_srng);
}
extern "C" {
    pub fn ath12k_dp_reoq_lut_addr_reset(dp: *mut ath12k_dp);
}
