//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pensando/ionic/ionic_lif.h
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
// Copyright(c) 2017 - 2019 Pensando Systems, Inc

pub const IONIC_RX_COPYBREAK_DEFAULT: c_int = 256;
pub const IONIC_TX_BUDGET_DEFAULT: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_tx_stats {
    pub pkts: u64,
    pub bytes: u64,
    pub csum_none: u64,
    pub csum: u64,
    pub tso: u64,
    pub tso_bytes: u64,
    pub frags: u64,
    pub vlan_inserted: u64,
    pub clean: u64,
    pub linearize: u64,
    pub crc32_csum: u64,
    pub dma_map_err: u64,
    pub hwstamp_valid: u64,
    pub hwstamp_invalid: u64,
    pub xdp_frames: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_stats {
    pub pkts: u64,
    pub bytes: u64,
    pub csum_none: u64,
    pub csum_complete: u64,
    pub dropped: u64,
    pub vlan_stripped: u64,
    pub csum_error: u64,
    pub dma_map_err: u64,
    pub alloc_err: u64,
    pub hwstamp_valid: u64,
    pub hwstamp_invalid: u64,
    pub xdp_drop: u64,
    pub xdp_aborted: u64,
    pub xdp_pass: u64,
    pub xdp_tx: u64,
    pub xdp_redirect: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qcq {
    pub q_base: *mut c_void,
    pub q_base_pa: dma_addr_t,
    pub q_size: u32,
    pub cq_size: u32,
    pub cq_base: *mut c_void,
    pub cq_base_pa: dma_addr_t,
    pub sg_base: *mut c_void,
    pub sg_base_pa: dma_addr_t,
    pub sg_size: u32,
    pub flags: c_uint,
    pub cmb_q_base: *mut void __iomem,
    pub cmb_q_base_pa: phys_addr_t,
    pub cmb_q_size: u32,
    pub cmb_pgid: u32,
    pub cmb_order: u32,
    pub dim: dim,
    pub q: ionic_queue,
    pub cq: ionic_cq,
    pub napi: napi_struct,
    pub intr: ionic_intr_info,
    pub doorbell_napi_work: work_struct,
    pub dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_deferred_work_type {
    IONIC_DW_TYPE_RX_MODE,
    IONIC_DW_TYPE_LINK_STATUS,
    IONIC_DW_TYPE_LIF_RESET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_deferred_work {
    pub list: list_head,
    pub type: ionic_deferred_work_type,
    pub addr: [u8; ETH_ALEN],
    pub fw_status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_deferred {
    pub /: *mut *mut spinlock_t lock; / lock for deferred work list,
    pub list: list_head,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_sw_stats {
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_tso: u64,
    pub tx_tso_bytes: u64,
    pub tx_csum_none: u64,
    pub tx_csum: u64,
    pub rx_csum_none: u64,
    pub rx_csum_complete: u64,
    pub rx_csum_error: u64,
    pub tx_hwstamp_valid: u64,
    pub tx_hwstamp_invalid: u64,
    pub rx_hwstamp_valid: u64,
    pub rx_hwstamp_invalid: u64,
    pub hw_tx_dropped: u64,
    pub hw_rx_dropped: u64,
    pub hw_rx_over_errors: u64,
    pub hw_rx_missed_errors: u64,
    pub hw_tx_aborted_errors: u64,
    pub xdp_drop: u64,
    pub xdp_aborted: u64,
    pub xdp_pass: u64,
    pub xdp_tx: u64,
    pub xdp_redirect: u64,
    pub xdp_frames: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_lif_state_flags {
    IONIC_LIF_F_INITED,
    IONIC_LIF_F_UP,
    IONIC_LIF_F_LINK_CHECK_REQUESTED,
    IONIC_LIF_F_FILTER_SYNC_NEEDED,
    IONIC_LIF_F_FW_RESET,
    IONIC_LIF_F_FW_STOPPING,
    IONIC_LIF_F_SPLIT_INTR,
    IONIC_LIF_F_BROKEN,
    IONIC_LIF_F_TX_DIM_INTR,
    IONIC_LIF_F_RX_DIM_INTR,
    IONIC_LIF_F_CMB_TX_RINGS,
    IONIC_LIF_F_CMB_RX_RINGS,

// leave this as last
    IONIC_LIF_F_STATE_SIZE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qtype_info {
    pub version: u8,
    pub supported: u8,
    pub features: u64,
    pub desc_sz: u16,
    pub comp_sz: u16,
    pub sg_desc_sz: u16,
    pub max_sg_elems: u16,
    pub sg_desc_stride: u16,
}

pub const IONIC_LIF_NAME_MAX_SZ: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif {
    pub netdev: *mut net_device,
    pub IONIC_LIF_F_STATE_SIZE): DECLARE_BITMAP(state,,
    pub ionic: *mut ionic,
    pub index: c_uint,
    pub hw_index: c_uint,
    pub /: *mut *mut mutex queue_lock; / lock for queue structures,
    pub /: *mut *mut mutex config_lock; / lock for config actions,
    pub /: *mut *mut spinlock_t adminq_lock; / lock for AdminQ operations,
    pub adminqcq: *mut ionic_qcq,
    pub notifyqcq: *mut ionic_qcq,
    pub txqcqs: *mut ionic_qcq,
    pub hwstamp_txq: *mut ionic_qcq,
    pub txqstats: *mut ionic_tx_stats,
    pub rxqcqs: *mut ionic_qcq,
    pub hwstamp_rxq: *mut ionic_qcq,
    pub rxqstats: *mut ionic_rx_stats,
    pub deferred: ionic_deferred,
    pub tx_timeout_work: work_struct,
    pub last_eid: u64,
    pub kern_pid: c_uint,
    pub kern_dbpage: *mut u64 __iomem,
    pub neqs: c_uint,
    pub nxqs: c_uint,
    pub ntxq_descs: c_uint,
    pub nrxq_descs: c_uint,
    pub rxq_features: u64,
    pub hw_features: u64,
    pub rx_copybreak: u16,
    pub rx_mode: u16,
    pub registered: bool,
    pub doorbell_wa: bool,
    pub lif_type: u16,
    pub nmcast: c_uint,
    pub nucast: c_uint,
    pub nvlans: c_uint,
    pub max_vlans: c_uint,
    pub name: [c_char; IONIC_LIF_NAME_MAX_SZ],
    pub identity: *mut ionic_lif_identity,
    pub info: *mut ionic_lif_info,
    pub info_pa: dma_addr_t,
    pub info_sz: u32,
    pub qtype_info: [ionic_qtype_info; IONIC_QTYPE_MAX],
    pub ionic_adev: *mut ionic_aux_dev,
    pub /: *mut *mut mutex adev_lock; / lock for aux_dev actions,
    pub rss_hash_key: [u8; IONIC_RSS_HASH_KEY_SIZE],
    pub rss_ind_tbl: *mut u8,
    pub rss_ind_tbl_pa: dma_addr_t,
    pub rss_ind_tbl_sz: u32,
    pub rss_types: u16,
    pub rx_filters: ionic_rx_filters,
    pub /: *mut *mut u32 rx_coalesce_usecs; / what the user asked for,
    pub /: *mut *mut u32 rx_coalesce_hw; / what the hw is using,
    pub /: *mut *mut u32 tx_coalesce_usecs; / what the user asked for,
    pub /: *mut *mut u32 tx_coalesce_hw; / what the hw is using,
    pub dbid_count: c_uint,
    pub phc: *mut ionic_phc,
    pub dentry: *mut dentry,
    pub xdp_prog: *mut bpf_prog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_phc {
    pub /: *mut *mut spinlock_t lock; / lock for state_page, cc and tc,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub /: *mut *mut mutex config_lock; / lock for ts_config,
    pub ts_config: kernel_hwtstamp_config,
    pub ts_config_rx_filt: u64,
    pub ts_config_tx_mode: u32,
    pub init_cc_mult: u32,
    pub aux_work_delay: c_long,
    pub ptp_info: ptp_clock_info,
    pub state_page: *mut ib_uverbs_clock_info,
    pub ptp: *mut ptp_clock,
    pub lif: *mut ionic_lif,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_queue_params {
    pub nxqs: c_uint,
    pub ntxq_descs: c_uint,
    pub nrxq_descs: c_uint,
    pub rxq_features: u64,
    pub xdp_prog: *mut bpf_prog,
    pub intr_split: bool,
    pub cmb_tx: bool,
    pub cmb_rx: bool,
}

// Div-by-zero should never be an issue, but check anyway
// Round up in case usecs is close to the next hw unit
// Convert from usecs to device units
extern "C" {
    pub fn ionic_link_status_check_request(lif: *mut ionic_lif, can_sleep: bool);
}
extern "C" {
    pub fn ionic_lif_alloc(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_lif_init(lif: *mut ionic_lif) -> c_int;
}
extern "C" {
    pub fn ionic_lif_free(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_deinit(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_addr_add(lif: *mut ionic_lif, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn ionic_lif_addr_del(lif: *mut ionic_lif, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn ionic_stop_queues_reconfig(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_txrx_free(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_qcqs_free(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_restart_lif(lif: *mut ionic_lif) -> c_int;
}
extern "C" {
    pub fn ionic_lif_register(lif: *mut ionic_lif) -> c_int;
}
extern "C" {
    pub fn ionic_lif_unregister(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_size(ionic: *mut ionic) -> c_int;
}

extern "C" {
    pub fn ionic_lif_hwstamp_replay(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_hwstamp_recreate_queues(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_phc_ktime(lif: *mut ionic_lif, counter: u64) -> ktime_t;
}
extern "C" {
    pub fn ionic_lif_register_phc(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_unregister_phc(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_alloc_phc(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_free_phc(lif: *mut ionic_lif);
}

extern "C" {
    pub fn ns_to_ktime(_arg: 0) -> return;
}

extern "C" {
    pub fn ionic_lif_create_hwstamp_txq(lif: *mut ionic_lif) -> c_int;
}
extern "C" {
    pub fn ionic_lif_create_hwstamp_rxq(lif: *mut ionic_lif) -> c_int;
}
extern "C" {
    pub fn ionic_lif_config_hwstamp_rxq_all(lif: *mut ionic_lif, rx_all: bool) -> c_int;
}
extern "C" {
    pub fn ionic_lif_set_hwstamp_txmode(lif: *mut ionic_lif, txstamp_mode: u16) -> c_int;
}
extern "C" {
    pub fn ionic_lif_set_hwstamp_rxfilt(lif: *mut ionic_lif, pkt_class: u64) -> c_int;
}
extern "C" {
    pub fn ionic_lif_rx_mode(lif: *mut ionic_lif);
}
