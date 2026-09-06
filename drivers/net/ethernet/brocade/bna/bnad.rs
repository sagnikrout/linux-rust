//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bnad.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

pub const BNAD_TXQ_DEPTH: c_int = 2048;
pub const BNAD_RXQ_DEPTH: c_int = 2048;
pub const BNAD_MAX_TX: c_int = 1;

pub const BNAD_TXQ_NUM: c_int = 1;
pub const BNAD_MAX_RX: c_int = 1;
pub const BNAD_MAX_RXP_PER_RX: c_int = 16;
pub const BNAD_MAX_RXQ_PER_RXP: c_int = 2;
//
// Control structure pointed to ccb->ctrl, which
// determines the NAPI / LRO behavior CCB
// There is 1:1 corres. between ccb & ctrl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_rx_ctrl {
    pub ccb: *mut bna_ccb,
    pub bnad: *mut bnad,
    pub flags: c_ulong,
    pub napi: napi_struct,
    pub rx_intr_ctr: u64,
    pub rx_poll_ctr: u64,
    pub rx_schedule: u64,
    pub rx_keep_poll: u64,
    pub rx_complete: u64,
}

//
// GLOBAL #defines (CONSTANTS)
//

pub const BNAD_NAME_LEN: c_int = 64;
pub const BNAD_MAILBOX_MSIX_INDEX: c_int = 0;
pub const BNAD_MAILBOX_MSIX_VECTORS: c_int = 1;
pub const BNAD_INTX_TX_IB_BITMASK: c_uint = 0x1;
pub const BNAD_INTX_RX_IB_BITMASK: c_uint = 0x2;

pub const BNAD_IOCETH_TIMEOUT: c_int = 10000;
pub const BNAD_MIN_Q_DEPTH: c_int = 512;
pub const BNAD_MAX_RXQ_DEPTH: c_int = 16384;
pub const BNAD_MAX_TXQ_DEPTH: c_int = 2048;
pub const BNAD_JUMBO_MTU: c_int = 9000;
pub const BNAD_NETIF_WAKE_THRESHOLD: c_int = 8;
pub const BNAD_RXQ_REFILL_THRESHOLD_SHIFT: c_int = 3;
// Bit positions for tcb->flags
pub const BNAD_TXQ_FREE_SENT: c_int = 0;
pub const BNAD_TXQ_TX_STARTED: c_int = 1;
// Bit positions for rcb->flags
pub const BNAD_RXQ_STARTED: c_int = 0;
pub const BNAD_RXQ_POST_OK: c_int = 1;
// Resource limits

//
// DATA STRUCTURES
//
// enums
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnad_intr_source {
    BNAD_INTR_TX		= 1,
    BNAD_INTR_RX		= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnad_link_state {
    BNAD_LS_DOWN		= 0,
    BNAD_LS_UP		= 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_iocmd_comp {
    pub bnad: *mut bnad,
    pub comp: completion,
    pub comp_status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_completion {
    pub ioc_comp: completion,
    pub ucast_comp: completion,
    pub mcast_comp: completion,
    pub tx_comp: completion,
    pub rx_comp: completion,
    pub stats_comp: completion,
    pub enet_comp: completion,
    pub mtu_comp: completion,
    pub ioc_comp_status: u8,
    pub ucast_comp_status: u8,
    pub mcast_comp_status: u8,
    pub tx_comp_status: u8,
    pub rx_comp_status: u8,
    pub stats_comp_status: u8,
    pub port_comp_status: u8,
    pub mtu_comp_status: u8,
}

// Tx Rx Control Stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_drv_stats {
    pub netif_queue_stop: u64,
    pub netif_queue_wakeup: u64,
    pub netif_queue_stopped: u64,
    pub tso4: u64,
    pub tso6: u64,
    pub tso_err: u64,
    pub tcpcsum_offload: u64,
    pub udpcsum_offload: u64,
    pub csum_help: u64,
    pub tx_skb_too_short: u64,
    pub tx_skb_stopping: u64,
    pub tx_skb_max_vectors: u64,
    pub tx_skb_mss_too_long: u64,
    pub tx_skb_tso_too_short: u64,
    pub tx_skb_tso_prepare: u64,
    pub tx_skb_non_tso_too_long: u64,
    pub tx_skb_tcp_hdr: u64,
    pub tx_skb_udp_hdr: u64,
    pub tx_skb_csum_err: u64,
    pub tx_skb_headlen_too_long: u64,
    pub tx_skb_headlen_zero: u64,
    pub tx_skb_frag_zero: u64,
    pub tx_skb_len_mismatch: u64,
    pub tx_skb_map_failed: u64,
    pub hw_stats_updates: u64,
    pub netif_rx_dropped: u64,
    pub link_toggle: u64,
    pub cee_toggle: u64,
    pub rxp_info_alloc_failed: u64,
    pub mbox_intr_disabled: u64,
    pub mbox_intr_enabled: u64,
    pub tx_unmap_q_alloc_failed: u64,
    pub rx_unmap_q_alloc_failed: u64,
    pub rxbuf_alloc_failed: u64,
    pub rxbuf_map_failed: u64,
}

// Complete driver stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_stats {
    pub drv_stats: bnad_drv_stats,
    pub bna_stats: *mut bna_stats,
}

// Tx / Rx Resources
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_tx_res_info {
    pub res_info: [bna_res_info; BNA_TX_RES_T_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_rx_res_info {
    pub res_info: [bna_res_info; BNA_RX_RES_T_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_tx_info {
    pub /: *mut *mut *mut bna_tx tx; / 1:1 between tx_info & tx,
    pub tcb: [*mut bna_tcb; BNAD_MAX_TXQ_PER_TX],
    pub tx_id: u32,
    pub tx_cleanup_work: delayed_work,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_rx_info {
    pub /: *mut *mut *mut bna_rx rx; / 1:1 between rx_info & rx,
    pub rx_ctrl: [bnad_rx_ctrl; BNAD_MAX_RXP_PER_RX],
    pub rx_id: u32,
    pub rx_cleanup_work: work_struct,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_tx_vector {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_tx_unmap {
    pub skb: *mut sk_buff,
    pub nvecs: u32,
    pub vectors: [bnad_tx_vector; BFI_TX_MAX_VECTORS_PER_WI],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_rx_vector {
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_rx_unmap {
    pub page: *mut page,
    pub skb: *mut sk_buff,
    pub vector: bnad_rx_vector,
    pub page_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnad_rxbuf_type {
    BNAD_RXBUF_NONE		= 0,
    BNAD_RXBUF_SK_BUFF	= 1,
    BNAD_RXBUF_PAGE		= 2,
    BNAD_RXBUF_MULTI_BUFF	= 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_rx_unmap_q {
    pub reuse_pi: c_int,
    pub alloc_order: c_int,
    pub map_size: u32,
    pub type: bnad_rxbuf_type,
    pub ____cacheline_aligned: bnad_rx_unmap unmap[],
}

// Bit mask values for bnad->cfg_flags
pub const BNAD_CF_DIM_ENABLED: c_uint = 0x01	/* DIM */;
pub const BNAD_CF_PROMISC: c_uint = 0x02;
pub const BNAD_CF_ALLMULTI: c_uint = 0x04;
pub const BNAD_CF_DEFAULT: c_uint = 0x08;
pub const BNAD_CF_MSIX: c_uint = 0x10	/* If in MSIx mode */;
// Defines for run_flags bit-mask
// Set, tested & cleared using xxx_bit() functions
// Values indicated bit positions
pub const BNAD_RF_CEE_RUNNING: c_int = 0;
pub const BNAD_RF_MTU_SET: c_int = 1;
pub const BNAD_RF_MBOX_IRQ_DISABLED: c_int = 2;
pub const BNAD_RF_NETDEV_REGISTERED: c_int = 3;
pub const BNAD_RF_DIM_TIMER_RUNNING: c_int = 4;
pub const BNAD_RF_STATS_TIMER_RUNNING: c_int = 5;
pub const BNAD_RF_TX_PRIO_SET: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad {
    pub netdev: *mut net_device,
    pub id: u32,
// Data path
    pub tx_info: [bnad_tx_info; BNAD_MAX_TX],
    pub rx_info: [bnad_rx_info; BNAD_MAX_RX],
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
//
// These q numbers are global only because
// they are used to calculate MSIx vectors.
// Actually the exact # of queues are per Tx/Rx
// object.
//
    pub num_tx: u32,
    pub num_rx: u32,
    pub num_txq_per_tx: u32,
    pub num_rxp_per_rx: u32,
    pub txq_depth: u32,
    pub rxq_depth: u32,
    pub tx_coalescing_timeo: u8,
    pub rx_coalescing_timeo: u8,
    pub ____cacheline_aligned: bna_rx_config rx_config[BNAD_MAX_RX],
    pub ____cacheline_aligned: bna_tx_config tx_config[BNAD_MAX_TX],
    pub /: *mut *mut *mut void __iomem bar0; / BAR0 address,
    pub bna: bna,
    pub cfg_flags: u32,
    pub run_flags: c_ulong,
    pub pcidev: *mut pci_dev,
    pub mmio_start: u64,
    pub mmio_len: u64,
    pub msix_num: u32,
    pub msix_table: *mut msix_entry,
    pub conf_mutex: mutex,
    pub ____cacheline_aligned: spinlock_t bna_lock,
// Timers
    pub ioc_timer: timer_list,
    pub dim_timer: timer_list,
    pub stats_timer: timer_list,
// Control path resources, memory & irq
    pub res_info: [bna_res_info; BNA_RES_T_MAX],
    pub mod_res_info: [bna_res_info; BNA_MOD_RES_T_MAX],
    pub tx_res_info: [bnad_tx_res_info; BNAD_MAX_TX],
    pub rx_res_info: [bnad_rx_res_info; BNAD_MAX_RX],
    pub bnad_completions: bnad_completion,
// Burnt in MAC address
    pub perm_addr: [u8; ETH_ALEN],
    pub work_q: *mut workqueue_struct,
// Statistics
    pub stats: bnad_stats,
    pub diag: *mut bnad_diag,
    pub adapter_name: [c_char; BNAD_NAME_LEN],
    pub port_name: [c_char; BNAD_NAME_LEN],
    pub mbox_irq_name: [c_char; BNAD_NAME_LEN],
    pub wq_name: [c_char; BNAD_NAME_LEN],
// debugfs specific data
    pub regdata: *mut c_char,
    pub reglen: u32,
    pub port_debugfs_root: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_drvinfo {
    pub ioc_attr: bfa_ioc_attr,
    pub cee_attr: bfa_cee_attr,
    pub flash_attr: bfa_flash_attr,
    pub cee_status: u32,
    pub flash_status: u32,
}

//
// EXTERN VARIABLES
//
// EXTERN PROTOTYPES
//
// Netdev entry point prototypes
extern "C" {
    pub fn bnad_set_rx_mode(netdev: *mut net_device);
}
extern "C" {
    pub fn bnad_mac_addr_set_locked(bnad: *mut bnad, mac_addr: *const u8) -> c_int;
}
extern "C" {
    pub fn bnad_enable_default_bcast(bnad: *mut bnad) -> c_int;
}
extern "C" {
    pub fn bnad_restore_vlans(bnad: *mut bnad, rx_id: u32);
}
extern "C" {
    pub fn bnad_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn bnad_cb_completion(arg: *mut c_void, status: bfa_status);
}
// Configuration & setup
extern "C" {
    pub fn bnad_tx_coalescing_timeo_set(bnad: *mut bnad);
}
extern "C" {
    pub fn bnad_rx_coalescing_timeo_set(bnad: *mut bnad);
}
extern "C" {
    pub fn bnad_setup_rx(bnad: *mut bnad, rx_id: u32) -> c_int;
}
extern "C" {
    pub fn bnad_setup_tx(bnad: *mut bnad, tx_id: u32) -> c_int;
}
extern "C" {
    pub fn bnad_destroy_tx(bnad: *mut bnad, tx_id: u32);
}
extern "C" {
    pub fn bnad_destroy_rx(bnad: *mut bnad, rx_id: u32);
}
// Timer start/stop protos
extern "C" {
    pub fn bnad_dim_timer_start(bnad: *mut bnad);
}
// Statistics
// Debugfs
extern "C" {
    pub fn bnad_debugfs_init(bnad: *mut bnad);
}
extern "C" {
    pub fn bnad_debugfs_uninit(bnad: *mut bnad);
}
// MACROS
// To set & get the stats counters

