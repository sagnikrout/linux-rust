//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/debugfs.h
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

pub const ATH11K_TX_POWER_MAX_VAL: c_int = 70;
pub const ATH11K_TX_POWER_MIN_VAL: c_int = 0;
// htt_dbg_ext_stats_type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_dbg_htt_ext_stats_type {
    ATH11K_DBG_HTT_EXT_STATS_RESET                      =  0,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_TX                    =  1,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_RX                    =  2,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_TX_HWQ                =  3,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_TX_SCHED              =  4,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_ERROR                 =  5,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_TQM                   =  6,
    ATH11K_DBG_HTT_EXT_STATS_TQM_CMDQ                   =  7,
    ATH11K_DBG_HTT_EXT_STATS_TX_DE_INFO                 =  8,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_TX_RATE               =  9,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_RX_RATE               =  10,
    ATH11K_DBG_HTT_EXT_STATS_PEER_INFO                  =  11,
    ATH11K_DBG_HTT_EXT_STATS_TX_SELFGEN_INFO            =  12,
    ATH11K_DBG_HTT_EXT_STATS_TX_MU_HWQ                  =  13,
    ATH11K_DBG_HTT_EXT_STATS_RING_IF_INFO               =  14,
    ATH11K_DBG_HTT_EXT_STATS_SRNG_INFO                  =  15,
    ATH11K_DBG_HTT_EXT_STATS_SFM_INFO                   =  16,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_TX_MU                 =  17,
    ATH11K_DBG_HTT_EXT_STATS_ACTIVE_PEERS_LIST          =  18,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_CCA_STATS             =  19,
    ATH11K_DBG_HTT_EXT_STATS_TWT_SESSIONS               =  20,
    ATH11K_DBG_HTT_EXT_STATS_REO_RESOURCE_STATS         =  21,
    ATH11K_DBG_HTT_EXT_STATS_TX_SOUNDING_INFO           =  22,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_OBSS_PD_STATS	    =  23,
    ATH11K_DBG_HTT_EXT_STATS_RING_BACKPRESSURE_STATS    =  24,
    ATH11K_DBG_HTT_EXT_STATS_PEER_CTRL_PATH_TXRX_STATS  =  29,
    ATH11K_DBG_HTT_EXT_STATS_PDEV_TX_RATE_TXBF_STATS    =  31,
    ATH11K_DBG_HTT_EXT_STATS_TXBF_OFDMA		    =  32,
    ATH11K_DBG_HTT_EXT_PHY_COUNTERS_AND_PHY_STATS	    =  37,

// keep this last
    ATH11K_DBG_HTT_NUM_EXT_STATS,
}

pub const ATH11K_DEBUG_DBR_ENTRIES_MAX: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_dbg_dbr_event {
    ATH11K_DBG_DBR_EVENT_INVALID,
    ATH11K_DBG_DBR_EVENT_RX,
    ATH11K_DBG_DBR_EVENT_REPLENISH,
    ATH11K_DBG_DBR_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbg_dbr_entry {
    pub hp: u32,
    pub tp: u32,
    pub timestamp: u64,
    pub event: ath11k_dbg_dbr_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbg_dbr_data {
// protects ath11k_db_ring_debug data
    pub lock: spinlock_t,
    pub entries: *mut ath11k_dbg_dbr_entry,
    pub dbr_debug_idx: u32,
    pub num_ring_debug_entries: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_debug_dbr {
    pub dbr_dbg_data: ath11k_dbg_dbr_data,
    pub dbr_debugfs: *mut dentry,
    pub dbr_debug_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_htt_stats_req {
    pub done: bool,
    pub pdev_id: u8,
    pub type: u8,
    pub peer_addr: [u8; ETH_ALEN],
    pub cmpln: completion,
    pub buf_len: u32,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_pktlog_hdr {
    pub flags: u16,
    pub missed_cnt: u16,
    pub log_type: u16,
    pub size: u16,
    pub timestamp: u32,
    pub type_specific_data: u32,
    pub payload: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_pktlog_filter {
    ATH11K_PKTLOG_RX		= 0x000000001,
    ATH11K_PKTLOG_TX		= 0x000000002,
    ATH11K_PKTLOG_RCFIND		= 0x000000004,
    ATH11K_PKTLOG_RCUPDATE		= 0x000000008,
    ATH11K_PKTLOG_EVENT_SMART_ANT	= 0x000000020,
    ATH11K_PKTLOG_EVENT_SW		= 0x000000040,
    ATH11K_PKTLOG_ANY		= 0x00000006f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_pktlog_mode {
    ATH11K_PKTLOG_MODE_LITE = 1,
    ATH11K_PKTLOG_MODE_FULL = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_pktlog_enum {
    ATH11K_PKTLOG_TYPE_INVALID      = 0,
    ATH11K_PKTLOG_TYPE_TX_CTRL      = 1,
    ATH11K_PKTLOG_TYPE_TX_STAT      = 2,
    ATH11K_PKTLOG_TYPE_TX_MSDU_ID   = 3,
    ATH11K_PKTLOG_TYPE_RX_STAT      = 5,
    ATH11K_PKTLOG_TYPE_RC_FIND      = 6,
    ATH11K_PKTLOG_TYPE_RC_UPDATE    = 7,
    ATH11K_PKTLOG_TYPE_TX_VIRT_ADDR = 8,
    ATH11K_PKTLOG_TYPE_RX_CBF       = 10,
    ATH11K_PKTLOG_TYPE_RX_STATBUF   = 22,
    ATH11K_PKTLOG_TYPE_PPDU_STATS   = 23,
    ATH11K_PKTLOG_TYPE_LITE_RX      = 24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_dbg_aggr_mode {
    ATH11K_DBG_AGGR_MODE_AUTO,
    ATH11K_DBG_AGGR_MODE_MANUAL,
    ATH11K_DBG_AGGR_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_dbglog_wlan_module_id {
    WLAN_MODULE_ID_MIN = 0,
    WLAN_MODULE_INF = WLAN_MODULE_ID_MIN,
    WLAN_MODULE_WMI,
    WLAN_MODULE_STA_PWRSAVE,
    WLAN_MODULE_WHAL,
    WLAN_MODULE_COEX,
    WLAN_MODULE_ROAM,
    WLAN_MODULE_RESMGR_CHAN_MANAGER,
    WLAN_MODULE_RESMGR,
    WLAN_MODULE_VDEV_MGR,
    WLAN_MODULE_SCAN,
    WLAN_MODULE_RATECTRL,
    WLAN_MODULE_AP_PWRSAVE,
    WLAN_MODULE_BLOCKACK,
    WLAN_MODULE_MGMT_TXRX,
    WLAN_MODULE_DATA_TXRX,
    WLAN_MODULE_HTT,
    WLAN_MODULE_HOST,
    WLAN_MODULE_BEACON,
    WLAN_MODULE_OFFLOAD,
    WLAN_MODULE_WAL,
    WLAN_WAL_MODULE_DE,
    WLAN_MODULE_PCIELP,
    WLAN_MODULE_RTT,
    WLAN_MODULE_RESOURCE,
    WLAN_MODULE_DCS,
    WLAN_MODULE_CACHEMGR,
    WLAN_MODULE_ANI,
    WLAN_MODULE_P2P,
    WLAN_MODULE_CSA,
    WLAN_MODULE_NLO,
    WLAN_MODULE_CHATTER,
    WLAN_MODULE_WOW,
    WLAN_MODULE_WAL_VDEV,
    WLAN_MODULE_WAL_PDEV,
    WLAN_MODULE_TEST,
    WLAN_MODULE_STA_SMPS,
    WLAN_MODULE_SWBMISS,
    WLAN_MODULE_WMMAC,
    WLAN_MODULE_TDLS,
    WLAN_MODULE_HB,
    WLAN_MODULE_TXBF,
    WLAN_MODULE_BATCH_SCAN,
    WLAN_MODULE_THERMAL_MGR,
    WLAN_MODULE_PHYERR_DFS,
    WLAN_MODULE_RMC,
    WLAN_MODULE_STATS,
    WLAN_MODULE_NAN,
    WLAN_MODULE_IBSS_PWRSAVE,
    WLAN_MODULE_HIF_UART,
    WLAN_MODULE_LPI,
    WLAN_MODULE_EXTSCAN,
    WLAN_MODULE_UNIT_TEST,
    WLAN_MODULE_MLME,
    WLAN_MODULE_SUPPL,
    WLAN_MODULE_ERE,
    WLAN_MODULE_OCB,
    WLAN_MODULE_RSSI_MONITOR,
    WLAN_MODULE_WPM,
    WLAN_MODULE_CSS,
    WLAN_MODULE_PPS,
    WLAN_MODULE_SCAN_CH_PREDICT,
    WLAN_MODULE_MAWC,
    WLAN_MODULE_CMC_QMIC,
    WLAN_MODULE_EGAP,
    WLAN_MODULE_NAN20,
    WLAN_MODULE_QBOOST,
    WLAN_MODULE_P2P_LISTEN_OFFLOAD,
    WLAN_MODULE_HALPHY,
    WLAN_WAL_MODULE_ENQ,
    WLAN_MODULE_GNSS,
    WLAN_MODULE_WAL_MEM,
    WLAN_MODULE_SCHED_ALGO,
    WLAN_MODULE_TX,
    WLAN_MODULE_RX,
    WLAN_MODULE_WLM,
    WLAN_MODULE_RU_ALLOCATOR,
    WLAN_MODULE_11K_OFFLOAD,
    WLAN_MODULE_STA_TWT,
    WLAN_MODULE_AP_TWT,
    WLAN_MODULE_UL_OFDMA,
    WLAN_MODULE_HPCS_PULSE,
    WLAN_MODULE_DTF,
    WLAN_MODULE_QUIET_IE,
    WLAN_MODULE_SHMEM_MGR,
    WLAN_MODULE_CFIR,
    WLAN_MODULE_CODE_COVER,
    WLAN_MODULE_SHO,
    WLAN_MODULE_MLO_MGR,
    WLAN_MODULE_PEER_INIT,
    WLAN_MODULE_STA_MLO_PS,

    WLAN_MODULE_ID_MAX,
    WLAN_MODULE_ID_INVALID = WLAN_MODULE_ID_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_dbglog_log_level {
    ATH11K_FW_DBGLOG_ML = 0,
    ATH11K_FW_DBGLOG_VERBOSE = 0,
    ATH11K_FW_DBGLOG_INFO,
    ATH11K_FW_DBGLOG_INFO_LVL_1,
    ATH11K_FW_DBGLOG_INFO_LVL_2,
    ATH11K_FW_DBGLOG_WARN,
    ATH11K_FW_DBGLOG_ERR,
    ATH11K_FW_DBGLOG_LVL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_fw_dbglog {
    pub param: wmi_debug_log_param,
// log_level values are given in enum fw_dbglog_log_level
    pub log_level: u16,
// module_id values are given in  enum fw_dbglog_wlan_module_id
    pub module_id: u16,
}

// value is either log_level&module_id/vdev_id/vdev_id_bitmap/log_level
// according to param
//

extern "C" {
    pub fn ath11k_debugfs_soc_create(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_debugfs_soc_destroy(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_debugfs_pdev_create(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_debugfs_pdev_destroy(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_debugfs_register(ar: *mut ath11k) -> c_int;
}
extern "C" {
    pub fn ath11k_debugfs_unregister(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_debugfs_fw_stats_process(ar: *mut ath11k, stats: *mut ath11k_fw_stats);
}
extern "C" {
    pub fn ath11k_debugfs_fw_stats_init(ar: *mut ath11k);
}

