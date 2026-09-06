//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx4/mlx4_en.h
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
// Copyright (c) 2007 Mellanox Technologies. All rights reserved.
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

//
// Device constants
//
pub const MLX4_EN_PAGE_SHIFT: c_int = 12;

pub const DEF_RX_RINGS: c_int = 16;
pub const MAX_RX_RINGS: c_int = 128;
pub const MIN_RX_RINGS: c_int = 1;
pub const LOG_TXBB_SIZE: c_int = 6;

pub const STAMP_STRIDE: c_int = 64;

pub const STAMP_SHIFT: c_int = 31;
pub const STAMP_VAL: c_uint = 0x7fffffff;

pub const MAX_NUM_OF_FS_RULES: c_int = 256;
pub const MLX4_EN_FILTER_HASH_SHIFT: c_int = 4;
pub const MLX4_EN_FILTER_EXPIRY_QUOTA: c_int = 60;

// Maximal size of the bounce buffer:
// 256 bytes for LSO headers.
// CTRL_SIZE for control desc.
// DS_SIZE if skb->head contains some payload.
// MAX_SKB_FRAGS frags.
//

//
// OS related constants and tunables
//
pub const MLX4_EN_PRIV_FLAGS_BLUEFLAME: c_int = 1;
pub const MLX4_EN_PRIV_FLAGS_PHV: c_int = 2;

// Use the maximum between 16384 and a single page

pub const MLX4_EN_MAX_RX_FRAGS: c_int = 4;
// Maximum ring sizes
pub const MLX4_EN_MAX_TX_SIZE: c_int = 8192;
pub const MLX4_EN_MAX_RX_SIZE: c_int = 8192;
// Minimum ring size for our page-allocation scheme to work

pub const MLX4_EN_SMALL_PKT_SIZE: c_int = 64;
pub const MLX4_EN_MIN_TX_RING_P_UP: c_int = 1;
pub const MLX4_EN_MAX_TX_RING_P_UP: c_int = 32;
pub const MLX4_EN_NUM_UP_LOW: c_int = 1;
pub const MLX4_EN_NUM_UP_HIGH: c_int = 8;
pub const MLX4_EN_DEF_RX_RING_SIZE: c_int = 1024;

pub const MLX4_EN_DEFAULT_TX_WORK: c_int = 256;
// Target number of packets to coalesce with interrupt moderation
pub const MLX4_EN_RX_COAL_TARGET: c_int = 44;
pub const MLX4_EN_RX_COAL_TIME: c_uint = 0x10;
pub const MLX4_EN_TX_COAL_PKTS: c_int = 16;
pub const MLX4_EN_TX_COAL_TIME: c_uint = 0x10;

pub const MLX4_EN_RX_RATE_LOW: c_int = 400000;
pub const MLX4_EN_RX_COAL_TIME_LOW: c_int = 0;
pub const MLX4_EN_RX_RATE_HIGH: c_int = 450000;
pub const MLX4_EN_RX_COAL_TIME_HIGH: c_int = 128;
pub const MLX4_EN_RX_SIZE_THRESH: c_int = 1024;

pub const MLX4_EN_SAMPLE_INTERVAL: c_int = 0;
pub const MLX4_EN_AVG_PKT_SMALL: c_int = 256;
pub const MLX4_EN_AUTO_CONF: c_uint = 0xffff;
pub const MLX4_EN_DEF_RX_PAUSE: c_int = 1;
pub const MLX4_EN_DEF_TX_PAUSE: c_int = 1;
// Interval between successive polls in the Tx routine when polling is used
pub const MLX4_EN_TX_POLL_MODER: c_int = 16;

pub const PREAMBLE_LEN: c_int = 8;

// VLAN_HLEN is added twice,to support skb vlan tagged with multiple
// headers. (For example: ETH_P_8021Q and ETH_P_8021AD).
//

pub const ETH_BCAST: c_uint = 0xffffffffffffULL;
pub const MLX4_EN_LOOPBACK_RETRIES: c_int = 5;
pub const MLX4_EN_LOOPBACK_TIMEOUT: c_int = 100;
// Constants for TX flow
//
// Configurables
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cq_type {
// keep tx types first
    TX,
    TX_XDP,

    RX,
}

//
// Useful macros
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_tx_info {
    pub skb: *mut sk_buff,
    pub page: *mut page,
}

pub const MLX4_EN_BIT_DESC_OWN: c_uint = 0x80000000;
pub const MLX4_EN_MEMTYPE_PAD: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_tx_desc {
    pub ctrl: mlx4_wqe_ctrl_seg,
    pub /: *mut *mut mlx4_wqe_data_seg data; / at least one data segment,
    pub lso: mlx4_wqe_lso_seg,
    pub inl: mlx4_wqe_inline_seg,
}

pub const MLX4_EN_USE_SRQ: c_uint = 0x01000000;
pub const MLX4_EN_CX3_LOW_ID: c_uint = 0x1000;
pub const MLX4_EN_CX3_HIGH_ID: c_uint = 0x1005;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_rx_alloc {
    pub page: *mut page,
    pub page_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_tx_ring {
// cache line used and dirtied in tx completion
// (mlx4_en_free_tx_buf())
//
    pub last_nr_txbb: u32,
    pub cons: u32,
    pub wake_queue: c_ulong,
    pub tx_queue: *mut netdev_queue,
    pub napi_mode): u64 timestamp, int,
    pub recycle_ring: *mut mlx4_en_rx_ring,
// cache line used and dirtied in mlx4_en_xmit()
    pub ____cacheline_aligned_in_smp: u32 prod,
    pub tx_dropped: c_uint,
    pub bytes: c_ulong,
    pub packets: c_ulong,
    pub tx_csum: c_ulong,
    pub tso_packets: c_ulong,
    pub xmit_more: c_ulong,
    pub bf: mlx4_bf,
// Following part should be mostly read
    pub doorbell_address: *mut void __iomem,
    pub doorbell_qpn: __be32,
    pub mr_key: __be32,
    pub /: *mut *mut u32 size; / number of TXBBs,
    pub size_mask: u32,
    pub full_size: u32,
    pub buf_size: u32,
    pub buf: *mut c_void,
    pub tx_info: *mut mlx4_en_tx_info,
    pub qpn: c_int,
    pub queue_index: u8,
    pub bf_enabled: bool,
    pub bf_alloced: bool,
    pub hwtstamp_tx_type: u8,
    pub bounce_buf: *mut u8,
// Not used in fast path
// Only queue_stopped might be used if BQL is not properly working.
//
    pub queue_stopped: c_ulong,
    pub state: c_ulong,
    pub sp_wqres: mlx4_hwq_resources,
    pub sp_qp: mlx4_qp,
    pub sp_context: mlx4_qp_context,
    pub sp_affinity_mask: cpumask_t,
    pub sp_qp_state: mlx4_qp_state,
    pub sp_stride: u16,
    pub /: *mut *mut u16 sp_cqn; / index of port CQ associated with this ring,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_rx_desc {
// actual number of entries depends on rx ring stride
    pub data): DECLARE_FLEX_ARRAY(struct mlx4_wqe_data_seg,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_rx_ring {
    pub wqres: mlx4_hwq_resources,
    pub descs*/: *mut *mut u32 size ; / number of Rx,
    pub actual_size: u32,
    pub size_mask: u32,
    pub stride: u16,
    pub log_stride: u16,
    pub /: *mut *mut u16 cqn; / index of port CQ associated with this ring,
    pub fcs_del: u8,
    pub prod: u32,
    pub cons: u32,
    pub buf_size: u32,
    pub pp: *mut page_pool,
    pub buf: *mut c_void,
    pub rx_info: *mut c_void,
    pub xdp_prog: *mut bpf_prog __rcu,
    pub bytes: c_ulong,
    pub packets: c_ulong,
    pub csum_ok: c_ulong,
    pub csum_none: c_ulong,
    pub csum_complete: c_ulong,
    pub rx_alloc_pages: c_ulong,
    pub xdp_drop: c_ulong,
    pub xdp_redirect: c_ulong,
    pub xdp_redirect_fail: c_ulong,
    pub xdp_tx: c_ulong,
    pub xdp_tx_full: c_ulong,
    pub dropped: c_ulong,
    pub alloc_fail: c_ulong,
    pub hwtstamp_rx_filter: c_int,
    pub affinity_mask: cpumask_var_t,
    pub xdp_rxq: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_cq {
    pub mcq: mlx4_cq,
    pub wqres: mlx4_hwq_resources,
    pub ring: c_int,
    pub dev: *mut net_device,
    pub napi: napi_struct,
    pub xdp_busy: bool,
}

pub const MLX4_EN_OPCODE_ERROR: c_uint = 0x1e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_port_profile {
    pub flags: u32,
    pub tx_ring_num: [u32; MLX4_EN_NUM_TX_TYPES],
    pub rx_ring_num: u32,
    pub tx_ring_size: u32,
    pub rx_ring_size: u32,
    pub num_tx_rings_p_up: u8,
    pub rx_pause: u8,
    pub rx_ppp: u8,
    pub tx_pause: u8,
    pub tx_ppp: u8,
    pub num_up: u8,
    pub rss_rings: c_int,
    pub inline_thold: c_int,
    pub hwtstamp_config: kernel_hwtstamp_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_profile {
    pub udp_rss: c_int,
    pub rss_mask: u8,
    pub active_ports: u32,
    pub small_pkt_int: u32,
    pub no_reset: u8,
    pub max_num_tx_rings_p_up: u8,
    pub 1]: mlx4_en_port_profile prof[MLX4_MAX_PORTS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_dev {
    pub dev: *mut mlx4_dev,
    pub pdev: *mut pci_dev,
    pub state_lock: mutex,
    pub 1]: *mut *mut net_device pndev[MLX4_MAX_PORTS +,
    pub 1]: *mut *mut net_device upper[MLX4_MAX_PORTS +,
    pub port_cnt: u32,
    pub device_up: bool,
    pub profile: mlx4_en_profile,
    pub LSO_support: u32,
    pub workqueue: *mut workqueue_struct,
    pub dma_device: *mut device,
    pub uar_map: *mut void __iomem,
    pub priv_uar: mlx4_uar,
    pub mr: mlx4_mr,
    pub priv_pdn: u32,
    pub uar_lock: spinlock_t,
    pub 1]: u8 mac_removed[MLX4_MAX_PORTS +,
    pub nominal_c_mult: u32,
    pub cycles: cyclecounter,
    pub clock_lock: seqlock_t,
    pub clock: timecounter,
    pub last_overflow_check: c_ulong,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub netdev_nb: notifier_block,
    pub mlx_nb: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_rss_map {
    pub base_qpn: c_int,
    pub qps: [mlx4_qp; MAX_RX_RINGS],
    pub state: [mlx4_qp_state; MAX_RX_RINGS],
    pub indir_qp: *mut mlx4_qp,
    pub indir_state: mlx4_qp_state,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_en_port_flag {
    MLX4_EN_PORT_ANC = 1<<0, /* Auto-negotiation complete */
    MLX4_EN_PORT_ANE = 1<<1, /* Auto-negotiation enabled */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_port_state {
    pub link_state: c_int,
    pub link_speed: c_int,
    pub transceiver: c_int,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_en_mclist_act {
    MCLIST_NONE,
    MCLIST_REM,
    MCLIST_ADD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_mc_list {
    pub list: list_head,
    pub action: mlx4_en_mclist_act,
    pub addr: [u8; ETH_ALEN],
    pub reg_id: u64,
    pub tunnel_reg_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_frag_info {
    pub frag_size: u16,
    pub frag_stride: u32,
}

// Minimal TC BW - setting to 0 will block traffic
pub const MLX4_EN_BW_MIN: c_int = 1;

pub const MLX4_EN_TC_VENDOR: c_int = 0;
pub const MLX4_EN_TC_ETS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcb_pfc_type {
    pfc_disabled = 0,
    pfc_enabled_full,
    pfc_enabled_tx,
    pfc_enabled_rx
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_cee_config {
    pub pfc_state: bool,
    pub dcb_pfc: [dcb_pfc_type; MLX4_EN_NUM_UP_HIGH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_flow_id {
    pub list: list_head,
    pub flow_spec: ethtool_rx_flow_spec,
    pub id: u64,
}

// whether we need to enable hardware loopback by putting dmac
// in Tx WQE
//
// whether we need to drop packets that hardware loopback-ed

pub const MLX4_EN_MAC_HASH_IDX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_stats_bitmap {
    pub NUM_ALL_STATS): DECLARE_BITMAP(bitmap,,
    pub /: *mut *mut mutex mutex; / for mutual access to stats bitmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_priv {
    pub mdev: *mut mlx4_en_dev,
    pub prof: *mut mlx4_en_port_profile,
    pub dev: *mut net_device,
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub port_state: mlx4_en_port_state,
    pub stats_lock: spinlock_t,
    pub ethtool_rules: [ethtool_flow_id; MAX_NUM_OF_FS_RULES],
// To allow rules removal while port is going down
    pub ethtool_list: list_head,
    pub last_moder_packets: [c_ulong; MAX_RX_RINGS],
    pub last_moder_tx_packets: c_ulong,
    pub last_moder_bytes: [c_ulong; MAX_RX_RINGS],
    pub last_moder_jiffies: c_ulong,
    pub last_moder_time: [c_int; MAX_RX_RINGS],
    pub rx_usecs: u16,
    pub rx_frames: u16,
    pub tx_usecs: u16,
    pub tx_frames: u16,
    pub pkt_rate_low: u32,
    pub rx_usecs_low: u16,
    pub pkt_rate_high: u32,
    pub rx_usecs_high: u16,
    pub sample_interval: u32,
    pub adaptive_rx_coal: u32,
    pub msg_enable: u32,
    pub loopback_ok: u32,
    pub validate_loopback: u32,
    pub res: mlx4_hwq_resources,
    pub link_state: c_int,
    pub port_up: bool,
    pub port: c_int,
    pub registered: c_int,
    pub allocated: c_int,
    pub stride: c_int,
    pub 2]: unsigned char current_mac[ETH_ALEN +,
    pub mac_index: c_int,
    pub max_mtu: unsigned,
    pub base_qpn: c_int,
    pub cqe_factor: c_int,
    pub cqe_size: c_int,
    pub rss_map: mlx4_en_rss_map,
    pub ctrl_flags: __be32,
    pub flags: u32,
    pub num_tx_rings_p_up: u8,
    pub tx_work_limit: u32,
    pub tx_ring_num: [u32; MLX4_EN_NUM_TX_TYPES],
    pub rx_ring_num: u32,
    pub rx_skb_size: u32,
    pub frag_info: [mlx4_en_frag_info; MLX4_EN_MAX_RX_FRAGS],
    pub num_frags: u8,
    pub log_rx_info: u8,
    pub dma_dir: u8,
    pub rx_headroom: u16,
    pub tx_ring: [*mut mlx4_en_tx_ring; MLX4_EN_NUM_TX_TYPES],
    pub rx_ring: [*mut mlx4_en_rx_ring; MAX_RX_RINGS],
    pub tx_cq: [*mut mlx4_en_cq; MLX4_EN_NUM_TX_TYPES],
    pub rx_cq: [*mut mlx4_en_cq; MAX_RX_RINGS],
    pub drop_qp: mlx4_qp,
    pub rx_mode_task: work_struct,
    pub restart_task: work_struct,
    pub linkstate_task: work_struct,
    pub stats_task: delayed_work,
    pub service_task: delayed_work,
    pub pkstats: mlx4_en_pkt_stats,
    pub pf_stats: mlx4_en_counter_stats,
    pub rx_priority_flowstats: [mlx4_en_flow_stats_rx; MLX4_NUM_PRIORITIES],
    pub tx_priority_flowstats: [mlx4_en_flow_stats_tx; MLX4_NUM_PRIORITIES],
    pub rx_flowstats: mlx4_en_flow_stats_rx,
    pub tx_flowstats: mlx4_en_flow_stats_tx,
    pub port_stats: mlx4_en_port_stats,
    pub xdp_stats: mlx4_en_xdp_stats,
    pub phy_stats: mlx4_en_phy_stats,
    pub stats_bitmap: mlx4_en_stats_bitmap,
    pub mc_list: list_head,
    pub curr_list: list_head,
    pub broadcast_id: u64,
    pub hw_stats: mlx4_en_stat_out_mbox,
    pub vids: [c_int; 128],
    pub wol: bool,
    pub ddev: *mut device,
    pub mac_hash: [hlist_head; MLX4_EN_MAC_HASH_SIZE],
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub counter_index: u32,

pub const MLX4_EN_DCB_ENABLED: c_uint = 0x3;
    pub ets: ieee_ets,
    pub maxrate: [u16; IEEE_8021QAZ_MAX_TCS],
    pub cndd_state: [dcbnl_cndd_states; IEEE_8021QAZ_MAX_TCS],
    pub cee_config: mlx4_en_cee_config,
    pub dcbx_cap: u8,

    pub filters_lock: spinlock_t,
    pub last_filter_id: c_int,
    pub filters: list_head,
    pub MLX4_EN_FILTER_HASH_SHIFT]: hlist_head filter_hash[1 <<,

    pub tunnel_reg_id: u64,
    pub vxlan_port: __be16,
    pub pflags: u32,
    pub rss_key: [u8; MLX4_EN_RSS_KEY_SIZE],
    pub rss_hash_fn: u8,
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_en_wol {
    MLX4_EN_WOL_MAGIC = (1ULL << 61),
    MLX4_EN_WOL_ENABLED = (1ULL << 62),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mac_entry {
    pub hlist: hlist_node,
    pub 2]: unsigned char mac[ETH_ALEN +,
    pub reg_id: u64,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn mlx4_en_init_ptys2ethtool_map();
}
extern "C" {
    pub fn mlx4_en_destroy_netdev(dev: *mut net_device);
}
extern "C" {
    pub fn mlx4_en_start_port(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx4_en_stop_port(dev: *mut net_device, detach: c_int);
}
extern "C" {
    pub fn mlx4_en_destroy_cq(priv: *mut mlx4_en_priv, pcq: *mut mlx4_en_cq);
}
extern "C" {
    pub fn mlx4_en_deactivate_cq(priv: *mut mlx4_en_priv, cq: *mut mlx4_en_cq);
}
extern "C" {
    pub fn mlx4_en_set_cq_moder(priv: *mut mlx4_en_priv, cq: *mut mlx4_en_cq) -> c_int;
}
extern "C" {
    pub fn mlx4_en_arm_cq(priv: *mut mlx4_en_priv, cq: *mut mlx4_en_cq);
}
extern "C" {
    pub fn mlx4_en_tx_irq(mcq: *mut mlx4_cq);
}
extern "C" {
    pub fn mlx4_en_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn mlx4_en_xmit_doorbell(ring: *mut mlx4_en_tx_ring);
}
extern "C" {
    pub fn mlx4_en_set_num_rx_rings(mdev: *mut mlx4_en_dev);
}
extern "C" {
    pub fn mlx4_en_recover_from_oom(priv: *mut mlx4_en_priv);
}
extern "C" {
    pub fn mlx4_en_activate_rx_rings(priv: *mut mlx4_en_priv) -> c_int;
}
extern "C" {
    pub fn mlx4_en_poll_rx_cq(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_en_poll_tx_cq(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_en_sqp_event(qp: *mut mlx4_qp, event: mlx4_event);
}
extern "C" {
    pub fn mlx4_en_calc_rx_buf(dev: *mut net_device);
}
extern "C" {
    pub fn mlx4_en_config_rss_steer(priv: *mut mlx4_en_priv) -> c_int;
}
extern "C" {
    pub fn mlx4_en_release_rss_steer(priv: *mut mlx4_en_priv);
}
extern "C" {
    pub fn mlx4_en_create_drop_qp(priv: *mut mlx4_en_priv) -> c_int;
}
extern "C" {
    pub fn mlx4_en_destroy_drop_qp(priv: *mut mlx4_en_priv);
}
extern "C" {
    pub fn mlx4_en_free_tx_buf(dev: *mut net_device, ring: *mut mlx4_en_tx_ring) -> c_int;
}
extern "C" {
    pub fn mlx4_en_rx_irq(mcq: *mut mlx4_cq);
}
extern "C" {
    pub fn mlx4_SET_MCAST_FLTR(dev: *mut mlx4_dev, port: u8, mac: u64, clear: u64, mode: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_VLAN_FLTR(dev: *mut mlx4_dev, priv: *mut mlx4_en_priv) -> c_int;
}
extern "C" {
    pub fn mlx4_en_fold_software_stats(dev: *mut net_device);
}
extern "C" {
    pub fn mlx4_en_DUMP_ETH_STATS(mdev: *mut mlx4_en_dev, port: u8, reset: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_en_QUERY_PORT(mdev: *mut mlx4_en_dev, port: u8) -> c_int;
}

extern "C" {
    pub fn mlx4_en_setup_tc(dev: *mut net_device, up: u8) -> c_int;
}
extern "C" {
    pub fn mlx4_en_alloc_tx_queue_per_tc(dev: *mut net_device, tc: u8) -> c_int;
}

extern "C" {
    pub fn mlx4_en_cleanup_filters(priv: *mut mlx4_en_priv);
}

pub const MLX4_EN_NUM_SELF_TEST: c_int = 5;
extern "C" {
    pub fn mlx4_en_ex_selftest(dev: *mut net_device, flags: *mut u32, buf: *mut u64);
}
extern "C" {
    pub fn mlx4_en_ptp_overflow_check(mdev: *mut mlx4_en_dev);
}

extern "C" {
    pub fn mlx4_en_moderation_update(priv: *mut mlx4_en_priv) -> c_int;
}
extern "C" {
    pub fn mlx4_en_xdp_rx_timestamp(ctx: *const xdp_md, timestamp: *mut u64) -> c_int;
}
//
// Functions for time stamping
//
extern "C" {
    pub fn mlx4_en_get_cqe_ts(cqe: *mut mlx4_cqe) -> u64;
}
extern "C" {
    pub fn mlx4_en_get_hwtstamp(mdev: *mut mlx4_en_dev, timestamp: u64) -> u64;
}
extern "C" {
    pub fn mlx4_en_init_timestamp(mdev: *mut mlx4_en_dev);
}
extern "C" {
    pub fn mlx4_en_remove_timestamp(mdev: *mut mlx4_en_dev);
}
// Globals
//
// printk / logging functions
//

