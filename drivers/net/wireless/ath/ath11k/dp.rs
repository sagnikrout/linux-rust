//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/dp.h
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
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const MAX_RXDMA_PER_PDEV: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_rx_tid {
    pub tid: u8,
    pub paddr: dma_addr_t,
    pub size: u32,
    pub ba_win_sz: u32,
    pub active: bool,
// Info related to rx fragments
    pub cur_sn: u32,
    pub last_frag_no: u16,
    pub rx_frag_bitmap: u16,
    pub rx_frags: sk_buff_head,
    pub dst_ring_desc: *mut hal_reo_dest_ring,
// Timer info related to fragments
    pub frag_timer: timer_list,
    pub ab: *mut ath11k_base,
    pub vaddr_unaligned: *mut u32,
    pub paddr_unaligned: dma_addr_t,
    pub unaligned_size: u32,
}

pub const DP_REO_DESC_FREE_THRESHOLD: c_int = 64;
pub const DP_REO_DESC_FREE_TIMEOUT_MS: c_int = 1000;
pub const DP_MON_PURGE_TIMEOUT_MS: c_int = 100;
pub const DP_MON_SERVICE_BUDGET: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_reo_cache_flush_elem {
    pub list: list_head,
    pub data: dp_rx_tid,
    pub ts: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_reo_cmd {
    pub list: list_head,
    pub data: dp_rx_tid,
    pub cmd_num: c_int,
    pub status): hal_reo_cmd_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_srng {
    pub vaddr_unaligned: *mut u32,
    pub vaddr: *mut u32,
    pub paddr_unaligned: dma_addr_t,
    pub paddr: dma_addr_t,
    pub size: c_int,
    pub ring_id: u32,
    pub cached: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_rxdma_ring {
    pub refill_buf_ring: dp_srng,
    pub bufs_idr: idr,
// Protects bufs_idr
    pub idr_lock: spinlock_t,
    pub bufs_max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_tx_ring {
    pub tcl_data_ring_id: u8,
    pub tcl_data_ring: dp_srng,
    pub tcl_comp_ring: dp_srng,
    pub txbuf_idr: idr,
// Protects txbuf_idr and num_pending
    pub tx_idr_lock: spinlock_t,
    pub tx_status: *mut hal_wbm_release_ring,
    pub tx_status_head: c_int,
    pub tx_status_tail: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_mon_status_buf_state {
// PPDU id matches in dst ring and status ring
    DP_MON_STATUS_MATCH,
// status ring dma is not done
    DP_MON_STATUS_NO_DMA,
// status ring is lagging, reap status ring
    DP_MON_STATUS_LAG,
// status ring is leading, reap dst ring and drop
    DP_MON_STATUS_LEAD,
// replinish monitor status ring
    DP_MON_STATUS_REPLINISH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_pdev_mon_stats {
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
#[derive(Copy, Clone)]
pub struct dp_full_mon_mpdu {
    pub list: list_head,
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
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
pub const DP_RX_DESC_COOKIE_INDEX_MAX: c_uint = 0x3ffff;
pub const DP_RX_DESC_COOKIE_POOL_ID_MAX: c_uint = 0x1c0000;

pub const DP_NOT_PPDU_ID_WRAP_AROUND: c_int = 20000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_dp_ppdu_state {
    DP_PPDU_STATUS_START,
    DP_PPDU_STATUS_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_mon_data {
    pub link_desc_banks: [dp_link_desc_bank; DP_LINK_DESC_BANKS_MAX],
    pub mon_ppdu_info: hal_rx_mon_ppdu_info,
    pub mon_ppdu_status: u32,
    pub mon_last_buf_cookie: u32,
    pub mon_last_linkdesc_paddr: u64,
    pub chan_noise_floor: u16,
    pub hold_mon_dst_ring: bool,
    pub buf_state: dp_mon_status_buf_state,
    pub mon_status_paddr: dma_addr_t,
    pub mon_mpdu: *mut dp_full_mon_mpdu,
    pub sw_mon_entries: hal_sw_mon_ring_entries,
    pub rx_mon_stats: ath11k_pdev_mon_stats,
// lock for monitor data
    pub mon_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_pdev_dp {
    pub mac_id: u32,
    pub mon_dest_ring_stuck_cnt: u32,
    pub num_tx_pending: core::sync::atomic::AtomicI32,
    pub tx_empty_waitq: wait_queue_head_t,
    pub rx_refill_buf_ring: dp_rxdma_ring,
    pub rx_mac_buf_ring: [dp_srng; MAX_RXDMA_PER_PDEV],
    pub rxdma_err_dst_ring: [dp_srng; MAX_RXDMA_PER_PDEV],
    pub rxdma_mon_dst_ring: dp_srng,
    pub rxdma_mon_desc_ring: dp_srng,
    pub rxdma_mon_buf_ring: dp_rxdma_ring,
    pub rx_mon_status_refill_ring: [dp_rxdma_ring; MAX_RXDMA_PER_PDEV],
    pub rx_status: ieee80211_rx_status,
    pub mon_data: ath11k_mon_data,
}

pub const DP_NUM_CLIENTS_MAX: c_int = 64;
pub const DP_AVG_TIDS_PER_CLIENT: c_int = 2;

pub const DP_AVG_MSDUS_PER_FLOW: c_int = 128;
pub const DP_AVG_FLOWS_PER_TID: c_int = 2;
pub const DP_AVG_MPDUS_PER_TID_MAX: c_int = 128;
pub const DP_AVG_MSDUS_PER_MPDU: c_int = 4;

pub const DP_BA_WIN_SZ_MAX: c_int = 256;
pub const DP_TCL_NUM_RING_MAX: c_int = 3;
pub const DP_IDLE_SCATTER_BUFS_MAX: c_int = 16;
pub const DP_WBM_RELEASE_RING_SIZE: c_int = 64;
pub const DP_TCL_DATA_RING_SIZE: c_int = 512;
pub const DP_TCL_DATA_RING_SIZE_WCN6750: c_int = 2048;
pub const DP_TX_COMP_RING_SIZE: c_int = 32768;

pub const DP_TCL_CMD_RING_SIZE: c_int = 32;
pub const DP_TCL_STATUS_RING_SIZE: c_int = 32;
pub const DP_REO_DST_RING_MAX: c_int = 4;
pub const DP_REO_DST_RING_SIZE: c_int = 2048;
pub const DP_REO_REINJECT_RING_SIZE: c_int = 32;
pub const DP_RX_RELEASE_RING_SIZE: c_int = 1024;
pub const DP_REO_EXCEPTION_RING_SIZE: c_int = 128;
pub const DP_REO_CMD_RING_SIZE: c_int = 256;
pub const DP_REO_STATUS_RING_SIZE: c_int = 2048;
pub const DP_RXDMA_BUF_RING_SIZE: c_int = 4096;
pub const DP_RXDMA_REFILL_RING_SIZE: c_int = 2048;
pub const DP_RXDMA_ERR_DST_RING_SIZE: c_int = 1024;
pub const DP_RXDMA_MON_STATUS_RING_SIZE: c_int = 1024;
pub const DP_RXDMA_MONITOR_BUF_RING_SIZE: c_int = 4096;
pub const DP_RXDMA_MONITOR_DST_RING_SIZE: c_int = 2048;
pub const DP_RXDMA_MONITOR_DESC_RING_SIZE: c_int = 4096;
pub const DP_RX_RELEASE_RING_NUM: c_int = 3;
pub const DP_RX_BUFFER_SIZE: c_int = 2048;
pub const DP_RX_BUFFER_SIZE_LITE: c_int = 1024;
pub const DP_RX_BUFFER_ALIGN_SIZE: c_int = 128;

pub const ATH11K_SHADOW_DP_TIMER_INTERVAL: c_int = 20;
pub const ATH11K_SHADOW_CTRL_TIMER_INTERVAL: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_hp_update_timer {
    pub timer: timer_list,
    pub started: bool,
    pub init: bool,
    pub tx_num: u32,
    pub timer_tx_num: u32,
    pub ring_id: u32,
    pub interval: u32,
    pub ab: *mut ath11k_base,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dp {
    pub ab: *mut ath11k_base,
    pub eid: ath11k_htc_ep_id,
    pub htt_tgt_version_received: completion,
    pub htt_tgt_ver_major: u8,
    pub htt_tgt_ver_minor: u8,
    pub link_desc_banks: [dp_link_desc_bank; DP_LINK_DESC_BANKS_MAX],
    pub wbm_idle_ring: dp_srng,
    pub wbm_desc_rel_ring: dp_srng,
    pub tcl_cmd_ring: dp_srng,
    pub tcl_status_ring: dp_srng,
    pub reo_reinject_ring: dp_srng,
    pub rx_rel_ring: dp_srng,
    pub reo_except_ring: dp_srng,
    pub reo_cmd_ring: dp_srng,
    pub reo_status_ring: dp_srng,
    pub reo_dst_ring: [dp_srng; DP_REO_DST_RING_MAX],
    pub tx_ring: [dp_tx_ring; DP_TCL_NUM_RING_MAX],
    pub scatter_list: [hal_wbm_idle_scatter_list; DP_IDLE_SCATTER_BUFS_MAX],
    pub reo_cmd_list: list_head,
    pub reo_cmd_cache_flush_list: list_head,
    pub dp_full_mon_mpdu_list: list_head,
    pub reo_cmd_cache_flush_count: u32,
//
// protects access to below fields,
// - reo_cmd_list
// - reo_cmd_cache_flush_list
// - reo_cmd_cache_flush_count
//
    pub reo_cmd_lock: spinlock_t,
    pub reo_cmd_timer: ath11k_hp_update_timer,
    pub tx_ring_timer: [ath11k_hp_update_timer; DP_TCL_NUM_RING_MAX],
}

// HTT definitions

// vdev meta data

// peer meta data

pub const HTT_TX_WBM_COMP_STATUS_OFFSET: c_int = 8;
pub const HTT_INVALID_PEER_ID: c_uint = 0xffff;
// HTT tx completion is overlaid in wbm_release_ring

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_wbm_completion {
    pub info0: u32,
    pub info1: u32,
    pub info2: u32,
    pub info3: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_h2t_msg_type {
    HTT_H2T_MSG_TYPE_VERSION_REQ		= 0,
    HTT_H2T_MSG_TYPE_SRING_SETUP		= 0xb,
    HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG	= 0xc,
    HTT_H2T_MSG_TYPE_EXT_STATS_CFG		= 0x10,
    HTT_H2T_MSG_TYPE_PPDU_STATS_CFG		= 0x11,
    HTT_H2T_MSG_TYPE_RX_FULL_MONITOR_MODE	= 0x17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ver_req_cmd {
    pub ver_reg_info: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_srng_ring_type {
    HTT_HW_TO_SW_RING,
    HTT_SW_TO_HW_RING,
    HTT_SW_TO_SW_RING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_srng_ring_id {
    HTT_RXDMA_HOST_BUF_RING,
    HTT_RXDMA_MONITOR_STATUS_RING,
    HTT_RXDMA_MONITOR_BUF_RING,
    HTT_RXDMA_MONITOR_DESC_RING,
    HTT_RXDMA_MONITOR_DEST_RING,
    HTT_HOST1_TO_FW_RXBUF_RING,
    HTT_HOST2_TO_FW_RXBUF_RING,
    HTT_RXDMA_NON_MONITOR_DEST_RING,
}

// host -> target  HTT_SRING_SETUP message
//
// After target is booted up, Host can send SRING setup message for
// each host facing LMAC SRING. Target setups up HW registers based
// on setup message and confirms back to Host if response_required is set.
// Host should wait for confirmation message before sending new SRING
// setup message
//
// The message would appear as follows:
//
// |31            24|23    20|19|18 16|15|14          8|7                0|
// |--------------- +-----------------+----------------+------------------|
// |    ring_type   |      ring_id    |    pdev_id     |     msg_type     |
// |----------------------------------------------------------------------|
// |                          ring_base_addr_lo                           |
// |----------------------------------------------------------------------|
// |                         ring_base_addr_hi                            |
// |----------------------------------------------------------------------|
// |ring_misc_cfg_flag|ring_entry_size|            ring_size              |
// |----------------------------------------------------------------------|
// |                         ring_head_offset32_remote_addr_lo            |
// |----------------------------------------------------------------------|
// |                         ring_head_offset32_remote_addr_hi            |
// |----------------------------------------------------------------------|
// |                         ring_tail_offset32_remote_addr_lo            |
// |----------------------------------------------------------------------|
// |                         ring_tail_offset32_remote_addr_hi            |
// |----------------------------------------------------------------------|
// |                          ring_msi_addr_lo                            |
// |----------------------------------------------------------------------|
// |                          ring_msi_addr_hi                            |
// |----------------------------------------------------------------------|
// |                          ring_msi_data                               |
// |----------------------------------------------------------------------|
// |         intr_timer_th            |IM|      intr_batch_counter_th     |
// |----------------------------------------------------------------------|
// |          reserved        |RR|PTCF|        intr_low_threshold         |
// |----------------------------------------------------------------------|
// Where
// IM = sw_intr_mode
// RR = response_required
// PTCF = prefetch_timer_cfg
//
// The message is interpreted as follows:
// dword0  - b'0:7   - msg_type: This will be set to
// HTT_H2T_MSG_TYPE_SRING_SETUP
// b'8:15  - pdev_id:
// 0 (for rings at SOC/UMAC level),
// 1/2/3 mac id (for rings at LMAC level)
// b'16:23 - ring_id: identify which ring is to setup,
// more details can be got from enum htt_srng_ring_id
// b'24:31 - ring_type: identify type of host rings,
// more details can be got from enum htt_srng_ring_type
// dword1  - b'0:31  - ring_base_addr_lo: Lower 32bits of ring base address
// dword2  - b'0:31  - ring_base_addr_hi: Upper 32bits of ring base address
// dword3  - b'0:15  - ring_size: size of the ring in unit of 4-bytes words
// b'16:23 - ring_entry_size: Size of each entry in 4-byte word units
// b'24:31 - ring_misc_cfg_flag: Valid only for HW_TO_SW_RING and
// SW_TO_HW_RING.
// Refer to HTT_SRING_SETUP_RING_MISC_CFG_RING defs.
// dword4  - b'0:31  - ring_head_off32_remote_addr_lo:
// Lower 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the head
// element within the ring.
// (The head offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword5  - b'0:31  - ring_head_off32_remote_addr_hi:
// Upper 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the head
// element within the ring.
// (The head offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword6  - b'0:31  - ring_tail_off32_remote_addr_lo:
// Lower 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the tail
// element within the ring.
// (The tail offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword7  - b'0:31  - ring_tail_off32_remote_addr_hi:
// Upper 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the tail
// element within the ring.
// (The tail offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword8  - b'0:31  - ring_msi_addr_lo: Lower 32bits of MSI cfg address
// valid only for HW_TO_SW_RING and SW_TO_HW_RING
// dword9  - b'0:31  - ring_msi_addr_hi: Upper 32bits of MSI cfg address
// valid only for HW_TO_SW_RING and SW_TO_HW_RING
// dword10 - b'0:31  - ring_msi_data: MSI data
// Refer to HTT_SRING_SETUP_RING_MSC_CFG_xxx defs
// valid only for HW_TO_SW_RING and SW_TO_HW_RING
// dword11 - b'0:14  - intr_batch_counter_th:
// batch counter threshold is in units of 4-byte words.
// HW internally maintains and increments batch count.
// (see SRING spec for detail description).
// When batch count reaches threshold value, an interrupt
// is generated by HW.
// b'15    - sw_intr_mode:
// This configuration shall be static.
// Only programmed at power up.
// 0: generate pulse style sw interrupts
// 1: generate level style sw interrupts
// b'16:31 - intr_timer_th:
// The timer init value when timer is idle or is
// initialized to start downcounting.
// In 8us units (to cover a range of 0 to 524 ms)
// dword12 - b'0:15  - intr_low_threshold:
// Used only by Consumer ring to generate ring_sw_int_p.
// Ring entries low threshold water mark, that is used
// in combination with the interrupt timer as well as
// the clearing of the level interrupt.
// b'16:18 - prefetch_timer_cfg:
// Used only by Consumer ring to set timer mode to
// support Application prefetch handling.
// The external tail offset/pointer will be updated
// at following intervals:
// 3'b000: (Prefetch feature disabled; used only for debug)
// 3'b001: 1 usec
// 3'b010: 4 usec
// 3'b011: 8 usec (default)
// 3'b100: 16 usec
// Others: Reserved
// b'19    - response_required:
// Host needs HTT_T2H_MSG_TYPE_SRING_SETUP_DONE as response
// b'20:31 - reserved:  reserved for future use
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_srng_setup_cmd {
    pub info0: u32,
    pub ring_base_addr_lo: u32,
    pub ring_base_addr_hi: u32,
    pub info1: u32,
    pub ring_head_off32_remote_addr_lo: u32,
    pub ring_head_off32_remote_addr_hi: u32,
    pub ring_tail_off32_remote_addr_lo: u32,
    pub ring_tail_off32_remote_addr_hi: u32,
    pub ring_msi_addr_lo: u32,
    pub ring_msi_addr_hi: u32,
    pub msi_data: u32,
    pub intr_info: u32,
    pub info2: u32,
    pub __packed: },
// host -> target FW  PPDU_STATS config message
//
// @details
// The following field definitions describe the format of the HTT host
// to target FW for PPDU_STATS_CFG msg.
// The message allows the host to configure the PPDU_STATS_IND messages
// produced by the target.
//
// |31          24|23          16|15           8|7            0|
// |-----------------------------------------------------------|
// |    REQ bit mask             |   pdev_mask  |   msg type   |
// |-----------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: identifies this is a req to configure ppdu_stats_ind from target
// Value: 0x11
// - PDEV_MASK
// Bits 8:15
// Purpose: identifies which pdevs this PPDU stats configuration applies to
// Value: This is a overloaded field, refer to usage and interpretation of
// PDEV in interface document.
// Bit   8    :  Reserved for SOC stats
// Bit 9 - 15 :  Indicates PDEV_MASK in DBDC
// Indicates MACID_MASK in DBS
// - REQ_TLV_BIT_MASK
// Bits 16:31
// Purpose: each set bit indicates the corresponding PPDU stats TLV type
// needs to be included in the target's PPDU_STATS_IND messages.
// Value: refer htt_ppdu_stats_tlv_tag_t <<<???
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_cfg_cmd {
    pub msg: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_ppdu_stats_tag_type {
    HTT_PPDU_STATS_TAG_COMMON,
    HTT_PPDU_STATS_TAG_USR_COMMON,
    HTT_PPDU_STATS_TAG_USR_RATE,
    HTT_PPDU_STATS_TAG_USR_MPDU_ENQ_BITMAP_64,
    HTT_PPDU_STATS_TAG_USR_MPDU_ENQ_BITMAP_256,
    HTT_PPDU_STATS_TAG_SCH_CMD_STATUS,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_COMMON,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_BA_BITMAP_64,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_BA_BITMAP_256,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_ACK_BA_STATUS,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_FLUSH,
    HTT_PPDU_STATS_TAG_USR_COMMON_ARRAY,
    HTT_PPDU_STATS_TAG_INFO,
    HTT_PPDU_STATS_TAG_TX_MGMTCTRL_PAYLOAD,

// New TLV's are added above to this line
    HTT_PPDU_STATS_TAG_MAX,
}

// HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG Message
//
// details:
// HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG message is sent by host to
// configure RXDMA rings.
// The configuration is per ring based and includes both packet subtypes
// and PPDU/MPDU TLVs.
//
// The message would appear as follows:
//
// |31       26|25|24|23            16|15             8|7             0|
// |-----------------+----------------+----------------+---------------|
// |   rsvd1   |PS|SS|     ring_id    |     pdev_id    |    msg_type   |
// |-------------------------------------------------------------------|
// |              rsvd2               |           ring_buffer_size     |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_0                 |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_1                 |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_2                 |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_3                 |
// |-------------------------------------------------------------------|
// |                         tlv_filter_in_flags                       |
// |-------------------------------------------------------------------|
// Where:
// PS = pkt_swap
// SS = status_swap
// The message is interpreted as follows:
// dword0 - b'0:7   - msg_type: This will be set to
// HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG
// b'8:15  - pdev_id:
// 0 (for rings at SOC/UMAC level),
// 1/2/3 mac id (for rings at LMAC level)
// b'16:23 - ring_id : Identify the ring to configure.
// More details can be got from enum htt_srng_ring_id
// b'24    - status_swap: 1 is to swap status TLV
// b'25    - pkt_swap:  1 is to swap packet TLV
// b'26:31 - rsvd1:  reserved for future use
// dword1 - b'0:16  - ring_buffer_size: size of buffers referenced by rx ring,
// in byte units.
// Valid only for HW_TO_SW_RING and SW_TO_HW_RING
// - b'16:31 - rsvd2: Reserved for future use
// dword2 - b'0:31  - packet_type_enable_flags_0:
// Enable MGMT packet from 0b0000 to 0b1001
// bits from low to high: FP, MD, MO - 3 bits
// FP: Filter_Pass
// MD: Monitor_Direct
// MO: Monitor_Other
// 10 mgmt subtypes * 3 bits -> 30 bits
// Refer to PKT_TYPE_ENABLE_FLAG0_xxx_MGMT_xxx defs
// dword3 - b'0:31  - packet_type_enable_flags_1:
// Enable MGMT packet from 0b1010 to 0b1111
// bits from low to high: FP, MD, MO - 3 bits
// Refer to PKT_TYPE_ENABLE_FLAG1_xxx_MGMT_xxx defs
// dword4 - b'0:31 -  packet_type_enable_flags_2:
// Enable CTRL packet from 0b0000 to 0b1001
// bits from low to high: FP, MD, MO - 3 bits
// Refer to PKT_TYPE_ENABLE_FLAG2_xxx_CTRL_xxx defs
// dword5 - b'0:31  - packet_type_enable_flags_3:
// Enable CTRL packet from 0b1010 to 0b1111,
// MCAST_DATA, UCAST_DATA, NULL_DATA
// bits from low to high: FP, MD, MO - 3 bits
// Refer to PKT_TYPE_ENABLE_FLAG3_xxx_CTRL_xxx defs
// dword6 - b'0:31 -  tlv_filter_in_flags:
// Filter in Attention/MPDU/PPDU/Header/User tlvs
// Refer to CFG_TLV_FILTER_IN_FLAG defs
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_filter_tlv_flags {
    HTT_RX_FILTER_TLV_FLAGS_MPDU_START		= BIT(0),
    HTT_RX_FILTER_TLV_FLAGS_MSDU_START		= BIT(1),
    HTT_RX_FILTER_TLV_FLAGS_RX_PACKET		= BIT(2),
    HTT_RX_FILTER_TLV_FLAGS_MSDU_END		= BIT(3),
    HTT_RX_FILTER_TLV_FLAGS_MPDU_END		= BIT(4),
    HTT_RX_FILTER_TLV_FLAGS_PACKET_HEADER		= BIT(5),
    HTT_RX_FILTER_TLV_FLAGS_PER_MSDU_HEADER		= BIT(6),
    HTT_RX_FILTER_TLV_FLAGS_ATTENTION		= BIT(7),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_START		= BIT(8),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END		= BIT(9),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END_USER_STATS	= BIT(10),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END_USER_STATS_EXT	= BIT(11),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END_STATUS_DONE	= BIT(12),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_mgmt_pkt_filter_tlv_flags0 {
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_REQ		= BIT(0),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_REQ		= BIT(1),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_REQ		= BIT(2),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_RESP		= BIT(3),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_RESP		= BIT(4),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_RESP		= BIT(5),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_REQ	= BIT(6),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_REQ	= BIT(7),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_REQ	= BIT(8),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_RESP	= BIT(9),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_RESP	= BIT(10),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_RESP	= BIT(11),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_REQ		= BIT(12),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_REQ		= BIT(13),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_REQ		= BIT(14),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_RESP		= BIT(15),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_RESP		= BIT(16),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_RESP		= BIT(17),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_TIMING_ADV	= BIT(18),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_TIMING_ADV	= BIT(19),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_TIMING_ADV	= BIT(20),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_RESERVED_7		= BIT(21),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_RESERVED_7		= BIT(22),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_RESERVED_7		= BIT(23),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_BEACON		= BIT(24),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_BEACON		= BIT(25),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_BEACON		= BIT(26),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_ATIM		= BIT(27),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_ATIM		= BIT(28),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_ATIM		= BIT(29),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_mgmt_pkt_filter_tlv_flags1 {
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_DISASSOC		= BIT(0),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_DISASSOC		= BIT(1),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_DISASSOC		= BIT(2),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_AUTH		= BIT(3),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_AUTH		= BIT(4),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_AUTH		= BIT(5),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_DEAUTH		= BIT(6),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_DEAUTH		= BIT(7),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_DEAUTH		= BIT(8),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION		= BIT(9),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION		= BIT(10),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION		= BIT(11),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION_NOACK	= BIT(12),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION_NOACK	= BIT(13),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION_NOACK	= BIT(14),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_RESERVED_15	= BIT(15),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_RESERVED_15	= BIT(16),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_RESERVED_15	= BIT(17),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_ctrl_pkt_filter_tlv_flags2 {
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_1	= BIT(0),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_1	= BIT(1),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_1	= BIT(2),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_2	= BIT(3),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_2	= BIT(4),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_2	= BIT(5),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_TRIGGER	= BIT(6),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_TRIGGER	= BIT(7),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_TRIGGER	= BIT(8),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_4	= BIT(9),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_4	= BIT(10),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_4	= BIT(11),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_BF_REP_POLL	= BIT(12),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_BF_REP_POLL	= BIT(13),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_BF_REP_POLL	= BIT(14),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_VHT_NDP	= BIT(15),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_VHT_NDP	= BIT(16),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_VHT_NDP	= BIT(17),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_FRAME_EXT	= BIT(18),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_FRAME_EXT	= BIT(19),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_FRAME_EXT	= BIT(20),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_WRAPPER	= BIT(21),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_WRAPPER	= BIT(22),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_WRAPPER	= BIT(23),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_BAR		= BIT(24),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_BAR		= BIT(25),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_BAR		= BIT(26),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_BA			= BIT(27),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_BA			= BIT(28),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_BA			= BIT(29),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_ctrl_pkt_filter_tlv_flags3 {
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_PSPOLL		= BIT(0),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_PSPOLL		= BIT(1),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_PSPOLL		= BIT(2),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_RTS		= BIT(3),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_RTS		= BIT(4),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_RTS		= BIT(5),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_CTS		= BIT(6),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_CTS		= BIT(7),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_CTS		= BIT(8),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_ACK		= BIT(9),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_ACK		= BIT(10),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_ACK		= BIT(11),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND		= BIT(12),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND		= BIT(13),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND		= BIT(14),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND_ACK		= BIT(15),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND_ACK		= BIT(16),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND_ACK		= BIT(17),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_data_pkt_filter_tlv_flasg3 {
    HTT_RX_FP_DATA_PKT_FILTER_TLV_FLASG3_MCAST	= BIT(18),
    HTT_RX_MD_DATA_PKT_FILTER_TLV_FLASG3_MCAST	= BIT(19),
    HTT_RX_MO_DATA_PKT_FILTER_TLV_FLASG3_MCAST	= BIT(20),
    HTT_RX_FP_DATA_PKT_FILTER_TLV_FLASG3_UCAST	= BIT(21),
    HTT_RX_MD_DATA_PKT_FILTER_TLV_FLASG3_UCAST	= BIT(22),
    HTT_RX_MO_DATA_PKT_FILTER_TLV_FLASG3_UCAST	= BIT(23),
    HTT_RX_FP_DATA_PKT_FILTER_TLV_FLASG3_NULL_DATA	= BIT(24),
    HTT_RX_MD_DATA_PKT_FILTER_TLV_FLASG3_NULL_DATA	= BIT(25),
    HTT_RX_MO_DATA_PKT_FILTER_TLV_FLASG3_NULL_DATA	= BIT(26),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_selection_cfg_cmd {
    pub info0: u32,
    pub info1: u32,
    pub pkt_type_en_flags0: u32,
    pub pkt_type_en_flags1: u32,
    pub pkt_type_en_flags2: u32,
    pub pkt_type_en_flags3: u32,
    pub rx_filter_tlv: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_tlv_filter {
    pub /: *mut *mut u32 rx_filter; / see htt_rx_filter_tlv_flags,
    pub /: *mut *mut u32 pkt_filter_flags0; / MGMT,
    pub /: *mut *mut u32 pkt_filter_flags1; / MGMT,
    pub /: *mut *mut u32 pkt_filter_flags2; / CTRL,
    pub /: *mut *mut u32 pkt_filter_flags3; / DATA,
}

// Enumeration for full monitor mode destination ring select
// 0 - REO destination ring select
// 1 - FW destination ring select
// 2 - SW destination ring select
// 3 - Release destination ring select
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_full_mon_release_ring {
    HTT_RX_MON_RING_REO,
    HTT_RX_MON_RING_FW,
    HTT_RX_MON_RING_SW,
    HTT_RX_MON_RING_RELEASE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_full_monitor_mode_cfg_cmd {
    pub info0: u32,
    pub cfg: u32,
    pub __packed: },
// HTT message target->host
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_t2h_msg_type {
    HTT_T2H_MSG_TYPE_VERSION_CONF,
    HTT_T2H_MSG_TYPE_PEER_MAP	= 0x3,
    HTT_T2H_MSG_TYPE_PEER_UNMAP	= 0x4,
    HTT_T2H_MSG_TYPE_RX_ADDBA	= 0x5,
    HTT_T2H_MSG_TYPE_PKTLOG		= 0x8,
    HTT_T2H_MSG_TYPE_SEC_IND	= 0xb,
    HTT_T2H_MSG_TYPE_PEER_MAP2	= 0x1e,
    HTT_T2H_MSG_TYPE_PEER_UNMAP2	= 0x1f,
    HTT_T2H_MSG_TYPE_PPDU_STATS_IND = 0x1d,
    HTT_T2H_MSG_TYPE_EXT_STATS_CONF = 0x1c,
    HTT_T2H_MSG_TYPE_BKPRESSURE_EVENT_IND = 0x24,
}

pub const HTT_TARGET_VERSION_MAJOR: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_version_conf_msg {
    pub version: u32,
    pub __packed: },

pub const HTT_T2H_PEER_MAP_INFO2_NEXT_HOP_S: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_peer_map_event {
    pub info: u32,
    pub mac_addr_l32: u32,
    pub info1: u32,
    pub info2: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_peer_unmap_event {
    pub info: u32,
    pub mac_addr_l32: u32,
    pub info1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_resp_msg {
    pub version_msg: htt_t2h_version_conf_msg,
    pub peer_map_ev: htt_t2h_peer_map_event,
    pub peer_unmap_ev: htt_t2h_peer_unmap_event,
}

pub const HTT_BACKPRESSURE_UMAC_RING_TYPE: c_int = 0;
pub const HTT_BACKPRESSURE_LMAC_RING_TYPE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_backpressure_umac_ringid {
    HTT_SW_RING_IDX_REO_REO2SW1_RING,
    HTT_SW_RING_IDX_REO_REO2SW2_RING,
    HTT_SW_RING_IDX_REO_REO2SW3_RING,
    HTT_SW_RING_IDX_REO_REO2SW4_RING,
    HTT_SW_RING_IDX_REO_WBM2REO_LINK_RING,
    HTT_SW_RING_IDX_REO_REO2TCL_RING,
    HTT_SW_RING_IDX_REO_REO2FW_RING,
    HTT_SW_RING_IDX_REO_REO_RELEASE_RING,
    HTT_SW_RING_IDX_WBM_PPE_RELEASE_RING,
    HTT_SW_RING_IDX_TCL_TCL2TQM_RING,
    HTT_SW_RING_IDX_WBM_TQM_RELEASE_RING,
    HTT_SW_RING_IDX_WBM_REO_RELEASE_RING,
    HTT_SW_RING_IDX_WBM_WBM2SW0_RELEASE_RING,
    HTT_SW_RING_IDX_WBM_WBM2SW1_RELEASE_RING,
    HTT_SW_RING_IDX_WBM_WBM2SW2_RELEASE_RING,
    HTT_SW_RING_IDX_WBM_WBM2SW3_RELEASE_RING,
    HTT_SW_RING_IDX_REO_REO_CMD_RING,
    HTT_SW_RING_IDX_REO_REO_STATUS_RING,
    HTT_SW_UMAC_RING_IDX_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_backpressure_lmac_ringid {
    HTT_SW_RING_IDX_FW2RXDMA_BUF_RING,
    HTT_SW_RING_IDX_FW2RXDMA_STATUS_RING,
    HTT_SW_RING_IDX_FW2RXDMA_LINK_RING,
    HTT_SW_RING_IDX_SW2RXDMA_BUF_RING,
    HTT_SW_RING_IDX_WBM2RXDMA_LINK_RING,
    HTT_SW_RING_IDX_RXDMA2FW_RING,
    HTT_SW_RING_IDX_RXDMA2SW_RING,
    HTT_SW_RING_IDX_RXDMA2RELEASE_RING,
    HTT_SW_RING_IDX_RXDMA2REO_RING,
    HTT_SW_RING_IDX_MONITOR_STATUS_RING,
    HTT_SW_RING_IDX_MONITOR_BUF_RING,
    HTT_SW_RING_IDX_MONITOR_DESC_RING,
    HTT_SW_RING_IDX_MONITOR_DEST_RING,
    HTT_SW_LMAC_RING_IDX_MAX,
}

// ppdu stats
//
// @details
// The following field definitions describe the format of the HTT target
// to host ppdu stats indication message.
//
// |31                         16|15   12|11   10|9      8|7            0 |
// |----------------------------------------------------------------------|
// |    payload_size             | rsvd  |pdev_id|mac_id  |    msg type   |
// |----------------------------------------------------------------------|
// |                          ppdu_id                                     |
// |----------------------------------------------------------------------|
// |                        Timestamp in us                               |
// |----------------------------------------------------------------------|
// |                          reserved                                    |
// |----------------------------------------------------------------------|
// |                    type-specific stats info                          |
// |                     (see htt_ppdu_stats.h)                           |
// |----------------------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: Identifies this is a PPDU STATS indication
// message.
// Value: 0x1d
// - mac_id
// Bits 9:8
// Purpose: mac_id of this ppdu_id
// Value: 0-3
// - pdev_id
// Bits 11:10
// Purpose: pdev_id of this ppdu_id
// Value: 0-3
// 0 (for rings at SOC level),
// 1/2/3 PDEV -> 0/1/2
// - payload_size
// Bits 31:16
// Purpose: total tlv size
// Value: payload_size in bytes
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htt_ppdu_stats_msg {
    pub info: u32,
    pub ppdu_id: u32,
    pub timestamp: u32,
    pub rsvd: u32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tlv {
    pub header: u32,
    pub value: [u8; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HTT_PPDU_STATS_BW {
    HTT_PPDU_STATS_BANDWIDTH_5MHZ   = 0,
    HTT_PPDU_STATS_BANDWIDTH_10MHZ  = 1,
    HTT_PPDU_STATS_BANDWIDTH_20MHZ  = 2,
    HTT_PPDU_STATS_BANDWIDTH_40MHZ  = 3,
    HTT_PPDU_STATS_BANDWIDTH_80MHZ  = 4,
    HTT_PPDU_STATS_BANDWIDTH_160MHZ = 5, /* includes 80+80 */
    HTT_PPDU_STATS_BANDWIDTH_DYN    = 6,
}

// bw - HTT_PPDU_STATS_BW

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_common {
    pub ppdu_id: u32,
    pub sched_cmdid: u16,
    pub ring_id: u8,
    pub num_users: u8,
    pub %HTT_PPDU_STATS_COMMON_FLAGS_*/: *mut *mut u32 flags; /,
    pub chain_mask: u32,
    pub /: *mut *mut u32 fes_duration_us; / frame exchange sequence,
    pub ppdu_sch_eval_start_tstmp_us: u32,
    pub ppdu_sch_end_tstmp_us: u32,
    pub ppdu_start_tstmp_us: u32,
// BIT [15 :  0] - phy mode (WLAN_PHY_MODE) with which ppdu was transmitted
// BIT [31 : 16] - bandwidth (in MHz) with which ppdu was transmitted
//
    pub phy_mode: u16,
    pub bw_mhz: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_ppdu_stats_gi {
    HTT_PPDU_STATS_SGI_0_8_US,
    HTT_PPDU_STATS_SGI_0_4_US,
    HTT_PPDU_STATS_SGI_1_6_US,
    HTT_PPDU_STATS_SGI_3_2_US,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_user_rate {
    pub tid_num: u8,
    pub reserved0: u8,
    pub sw_peer_id: u16,
    pub %HTT_PPDU_STATS_USER_RATE_INFO0_*/: *mut *mut u32 info0; /,
    pub ru_end: u16,
    pub ru_start: u16,
    pub resp_ru_end: u16,
    pub resp_ru_start: u16,
    pub /: *mut *mut u32 info1; / %HTT_PPDU_STATS_USER_RATE_INFO1_,
    pub /: *mut *mut u32 rate_flags; / %HTT_PPDU_STATS_USER_RATE_FLAGS_,
// Note: resp_rate_info is only valid for if resp_type is UL
    pub /: *mut *mut u32 resp_rate_flags; / %HTT_PPDU_STATS_USER_RATE_RESP_FLAGS_,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_ppdu_stats_usr_compln_status {
    HTT_PPDU_STATS_USER_STATUS_OK,
    HTT_PPDU_STATS_USER_STATUS_FILTERED,
    HTT_PPDU_STATS_USER_STATUS_RESP_TIMEOUT,
    HTT_PPDU_STATS_USER_STATUS_RESP_MISMATCH,
    HTT_PPDU_STATS_USER_STATUS_ABORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_usr_cmpltn_cmn {
    pub status: u8,
    pub tid_num: u8,
    pub sw_peer_id: u16,
// RSSI value of last ack packet (units = dB above noise floor)
    pub ack_rssi: u32,
    pub mpdu_tried: u16,
    pub mpdu_success: u16,
    pub %HTT_PPDU_STATS_USR_CMPLTN_CMN_FLAGS_LONG_RETRIES*/: *mut *mut u32 flags; /,
    pub __packed: },

pub const HTT_PPDU_STATS_NON_QOS_TID: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_usr_cmpltn_ack_ba_status {
    pub ppdu_id: u32,
    pub sw_peer_id: u16,
    pub reserved0: u16,
    pub /: *mut *mut u32 info; / %HTT_PPDU_STATS_USR_CMPLTN_CMN_INFO_,
    pub current_seq: u16,
    pub start_seq: u16,
    pub success_bytes: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_user_stats {
    pub peer_id: u16,
    pub tlv_flags: u32,
    pub is_valid_peer_id: bool,
    pub rate: htt_ppdu_stats_user_rate,
    pub cmpltn_cmn: htt_ppdu_stats_usr_cmpltn_cmn,
    pub ack_ba: htt_ppdu_stats_usr_cmpltn_ack_ba_status,
}

pub const HTT_PPDU_STATS_MAX_USERS: c_int = 8;
pub const HTT_PPDU_DESC_MAX_DEPTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats {
    pub common: htt_ppdu_stats_common,
    pub user_stats: [htt_ppdu_user_stats; HTT_PPDU_STATS_MAX_USERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_info {
    pub ppdu_id: u32,
    pub ppdu_stats: htt_ppdu_stats,
    pub list: list_head,
}

// @brief target -> host packet log message
//
// @details
// The following field definitions describe the format of the packet log
// message sent from the target to the host.
// The message consists of a 4-octet header,followed by a variable number
// of 32-bit character values.
//
// |31                         16|15  12|11   10|9    8|7            0|
// |------------------------------------------------------------------|
// |        payload_size         | rsvd |pdev_id|mac_id|   msg type   |
// |------------------------------------------------------------------|
// |                              payload                             |
// |------------------------------------------------------------------|
// - MSG_TYPE
// Bits 7:0
// Purpose: identifies this as a pktlog message
// Value: HTT_T2H_MSG_TYPE_PKTLOG
// - mac_id
// Bits 9:8
// Purpose: identifies which MAC/PHY instance generated this pktlog info
// Value: 0-3
// - pdev_id
// Bits 11:10
// Purpose: pdev_id
// Value: 0-3
// 0 (for rings at SOC level),
// 1/2/3 PDEV -> 0/1/2
// - payload_size
// Bits 31:16
// Purpose: explicitly specify the payload size
// Value: payload size in bytes (payload size is a multiple of 4 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_pktlog_msg {
    pub hdr: u32,
    pub payload: [u8; ],
}

// @brief host -> target FW extended statistics retrieve
//
// @details
// The following field definitions describe the format of the HTT host
// to target FW extended stats retrieve message.
// The message specifies the type of stats the host wants to retrieve.
//
// |31          24|23          16|15           8|7            0|
// |-----------------------------------------------------------|
// |   reserved   | stats type   |   pdev_mask  |   msg type   |
// |-----------------------------------------------------------|
// |                   config param [0]                        |
// |-----------------------------------------------------------|
// |                   config param [1]                        |
// |-----------------------------------------------------------|
// |                   config param [2]                        |
// |-----------------------------------------------------------|
// |                   config param [3]                        |
// |-----------------------------------------------------------|
// |                         reserved                          |
// |-----------------------------------------------------------|
// |                        cookie LSBs                        |
// |-----------------------------------------------------------|
// |                        cookie MSBs                        |
// |-----------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: identifies this is a extended stats upload request message
// Value: 0x10
// - PDEV_MASK
// Bits 8:15
// Purpose: identifies the mask of PDEVs to retrieve stats from
// Value: This is a overloaded field, refer to usage and interpretation of
// PDEV in interface document.
// Bit   8    :  Reserved for SOC stats
// Bit 9 - 15 :  Indicates PDEV_MASK in DBDC
// Indicates MACID_MASK in DBS
// - STATS_TYPE
// Bits 23:16
// Purpose: identifies which FW statistics to upload
// Value: Defined by htt_dbg_ext_stats_type (see htt_stats.h)
// - Reserved
// Bits 31:24
// - CONFIG_PARAM [0]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - CONFIG_PARAM [1]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - CONFIG_PARAM [2]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - CONFIG_PARAM [3]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - Reserved [31:0] for future use.
// - COOKIE_LSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: LSBs of the opaque cookie specified by the host-side requestor
// - COOKIE_MSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: MSBs of the opaque cookie specified by the host-side requestor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ext_stats_cfg_hdr {
    pub msg_type: u8,
    pub pdev_mask: u8,
    pub stats_type: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ext_stats_cfg_cmd {
    pub hdr: htt_ext_stats_cfg_hdr,
    pub cfg_param0: u32,
    pub cfg_param1: u32,
    pub cfg_param2: u32,
    pub cfg_param3: u32,
    pub reserved: u32,
    pub cookie_lsb: u32,
    pub cookie_msb: u32,
    pub __packed: },
// htt stats config default params
pub const HTT_STAT_DEFAULT_RESET_START_OFFSET: c_int = 0;
pub const HTT_STAT_DEFAULT_CFG0_ALL_HWQS: c_uint = 0xffffffff;
pub const HTT_STAT_DEFAULT_CFG0_ALL_TXQS: c_uint = 0xffffffff;
pub const HTT_STAT_DEFAULT_CFG0_ALL_CMDQS: c_uint = 0xffff;
pub const HTT_STAT_DEFAULT_CFG0_ALL_RINGS: c_uint = 0xffff;
pub const HTT_STAT_DEFAULT_CFG0_ACTIVE_PEERS: c_uint = 0xff;
pub const HTT_STAT_DEFAULT_CFG0_CCA_CUMULATIVE: c_uint = 0x00;
pub const HTT_STAT_DEFAULT_CFG0_ACTIVE_VDEVS: c_uint = 0x00;
// HTT_DBG_EXT_STATS_PEER_INFO
// PARAMS:
// @config_param0:
// [Bit0] - [0] for sw_peer_id, [1] for mac_addr based request
// [Bit15 : Bit 1] htt_peer_stats_req_mode_t
// [Bit31 : Bit16] sw_peer_id
// @config_param1:
// peer_stats_req_type_mask:32 (enum htt_peer_stats_tlv_enum)
// 0 bit htt_peer_stats_cmn_tlv
// 1 bit htt_peer_details_tlv
// 2 bit htt_tx_peer_rate_stats_tlv
// 3 bit htt_rx_peer_rate_stats_tlv
// 4 bit htt_tx_tid_stats_tlv/htt_tx_tid_stats_v1_tlv
// 5 bit htt_rx_tid_stats_tlv
// 6 bit htt_msdu_flow_stats_tlv
// @config_param2: [Bit31 : Bit0] mac_addr31to0
// @config_param3: [Bit15 : Bit0] mac_addr47to32
// [Bit31 : Bit16] reserved
//

pub const HTT_STAT_DEFAULT_PEER_REQ_TYPE: c_uint = 0x7f;
// Used to set different configs to the specified stats type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ext_stats_cfg_params {
    pub cfg0: u32,
    pub cfg1: u32,
    pub cfg2: u32,
    pub cfg3: u32,
}

// @brief target -> host extended statistics upload
//
// @details
// The following field definitions describe the format of the HTT target
// to host stats upload confirmation message.
// The message contains a cookie echoed from the HTT host->target stats
// upload request, which identifies which request the confirmation is
// for, and a single stats can span over multiple HTT stats indication
// due to the HTT message size limitation so every HTT ext stats indication
// will have tag-length-value stats information elements.
// The tag-length header for each HTT stats IND message also includes a
// status field, to indicate whether the request for the stat type in
// question was fully met, partially met, unable to be met, or invalid
// (if the stat type in question is disabled in the target).
// A Done bit 1's indicate the end of the of stats info elements.
//
// |31                         16|15    12|11|10 8|7   5|4       0|
// |--------------------------------------------------------------|
// |                   reserved                   |    msg type   |
// |--------------------------------------------------------------|
// |                         cookie LSBs                          |
// |--------------------------------------------------------------|
// |                         cookie MSBs                          |
// |--------------------------------------------------------------|
// |      stats entry length     | rsvd   | D|  S |   stat type   |
// |--------------------------------------------------------------|
// |                   type-specific stats info                   |
// |                      (see htt_stats.h)                       |
// |--------------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: Identifies this is a extended statistics upload confirmation
// message.
// Value: 0x1c
// - COOKIE_LSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: LSBs of the opaque cookie specified by the host-side requestor
// - COOKIE_MSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: MSBs of the opaque cookie specified by the host-side requestor
//
// Stats Information Element tag-length header fields:
// - STAT_TYPE
// Bits 7:0
// Purpose: identifies the type of statistics info held in the
// following information element
// Value: htt_dbg_ext_stats_type
// - STATUS
// Bits 10:8
// Purpose: indicate whether the requested stats are present
// Value: htt_dbg_ext_stats_status
// - DONE
// Bits 11
// Purpose:
// Indicates the completion of the stats entry, this will be the last
// stats conf HTT segment for the requested stats type.
// Value:
// 0 -> the stats retrieval is ongoing
// 1 -> the stats retrieval is complete
// - LENGTH
// Bits 31:16
// Purpose: indicate the stats information size
// Value: This field specifies the number of bytes of stats information
// that follows the element tag-length header.
// It is expected but not required that this length is a multiple of
// 4 bytes.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htt_extd_stats_msg {
    pub info0: u32,
    pub cookie: u64,
    pub info1: u32,
    pub data: [u8; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_mac_addr {
    pub mac_addr_l32: u32,
    pub mac_addr_h16: u32,
}

extern "C" {
    pub fn ath11k_dp_htt_connect(dp: *mut ath11k_dp) -> c_int;
}
extern "C" {
    pub fn ath11k_dp_vdev_tx_attach(ar: *mut ath11k, arvif: *mut ath11k_vif);
}
extern "C" {
    pub fn ath11k_dp_free(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_dp_alloc(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_dp_pdev_alloc(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_dp_pdev_pre_alloc(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_dp_pdev_free(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_dp_peer_setup(ar: *mut ath11k, vdev_id: c_int, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn ath11k_dp_peer_cleanup(ar: *mut ath11k, vdev_id: c_int, addr: *const u8);
}
extern "C" {
    pub fn ath11k_dp_srng_cleanup(ab: *mut ath11k_base, ring: *mut dp_srng);
}
extern "C" {
    pub fn ath11k_dp_stop_shadow_timers(ab: *mut ath11k_base);
}
