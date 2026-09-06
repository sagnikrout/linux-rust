//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/enetc/enetc.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2017-2019, 2025-2026 NXP

pub const ENETC_MAC_MAXFRM_SIZE: c_int = 9600;

pub const ENETC_CBD_DATA_MEM_ALIGN: c_int = 64;
pub const ENETC_MADDR_HASH_TBL_SZ: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_mac_addr_type {

    struct enetc_mac_filter {
    union {
    char mac_addr[ETH_ALEN];
    DECLARE_BITMAP(mac_hash_table, ENETC_MADDR_HASH_TBL_SZ);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_tx_swbd {
    pub skb: *mut sk_buff,
    pub xdp_frame: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_skb_cb {
    pub flag: u8,
    pub udp: bool,
    pub correction_off: u16,
    pub origin_tstamp_off: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_lso_t {
    pub ipv6: bool,
    pub tcp: bool,
    pub l3_hdr_len: u8,
    pub /: *mut *mut u8 hdr_len; / LSO header length,
    pub l3_start: u8,
    pub lso_seg_size: u16,
    pub /: *mut *mut int total_len; / total data length, not include LSO header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_rx_swbd {
    pub dma: dma_addr_t,
    pub page: *mut page,
    pub page_offset: u16,
    pub dir: dma_data_direction,
    pub len: u16,
}

// ENETC overhead: optional extension BD + 1 BD gap

// For LS1028A, max # of chained Tx BDs is 15, including head and
// extension BD.
//
pub const ENETC_MAX_SKB_FRAGS: c_int = 13;
// For ENETC v4 and later versions, max # of chained Tx BDs is 63,
// including head and extension BD, but the range of MAX_SKB_FRAGS
// is 17 ~ 45, so set ENETC4_MAX_SKB_FRAGS to MAX_SKB_FRAGS.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_ring_stats {
    pub packets: c_ulong,
    pub bytes: c_ulong,
    pub rx_alloc_errs: c_ulong,
    pub xdp_drops: c_ulong,
    pub xdp_tx: c_ulong,
    pub xdp_tx_drops: c_ulong,
    pub xdp_redirect: c_ulong,
    pub xdp_redirect_failures: c_ulong,
    pub recycles: c_ulong,
    pub recycle_failures: c_ulong,
    pub win_drop: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_xdp_data {
    pub rxq: xdp_rxq_info,
    pub prog: *mut bpf_prog,
    pub xdp_tx_in_flight: c_int,
}

pub const ENETC_RX_RING_DEFAULT_SIZE: c_int = 2048;
pub const ENETC_TX_RING_DEFAULT_SIZE: c_int = 2048;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_bdr_resource {
// Input arguments saved for teardown
    pub /: *mut *mut *mut device dev; / for DMA mapping,
    pub bd_count: usize,
    pub bd_size: usize,
// Resource proper
    pub /: *mut *mut *mut void bd_base; / points to Rx or Tx BD ring,
    pub bd_dma_base: dma_addr_t,
    pub tx_swbd: *mut enetc_tx_swbd,
    pub rx_swbd: *mut enetc_rx_swbd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_bdr {
    pub /: *mut *mut *mut device dev; / for DMA mapping,
    pub ndev: *mut net_device,
    pub /: *mut *mut *mut void bd_base; / points to Rx or Tx BD ring,
    pub tpir: *mut void __iomem,
    pub rcir: *mut void __iomem,
}

// DMA buffer for TSO headers
// i = 0;
// Control BD ring
pub const ENETC_CBDR_DEFAULT_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_cbdr {
    pub /: *mut *mut *mut void bd_base; / points to Rx or Tx BD ring,
    pub pir: *mut void __iomem,
    pub cir: *mut void __iomem,
    pub /: *mut *mut *mut void __iomem mr; / mode register,
    pub /: *mut *mut int bd_count; / # of BDs,
    pub next_to_use: c_int,
    pub next_to_clean: c_int,
    pub bd_dma_base: dma_addr_t,
    pub dma_dev: *mut device,
}

// old_rxbd = new_rxbd;
// old_index = new_index;
pub const ENETC_REV1: c_uint = 0x1;
pub const ENETC_REV4: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_errata {
    ENETC_ERR_VLAN_ISOL	= BIT(0),
    ENETC_ERR_UCMCSWP	= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_drvdata {
    pub /: *mut *mut u32 pmac_offset; / Only valid for PSI which supports 802.1Qbu,
    pub tx_csum:1: u8,
    pub max_frags: u8,
    pub sysclk_freq: u64,
    pub eth_ops: *const ethtool_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_platform_info {
    pub revision: u16,
    pub dev_id: u16,
    pub data: *const enetc_drvdata,
}

//
// This structure defines the some common hooks for ENETC PSI and VSI.
// In addition, since VSI only uses the struct enetc_si as its private
// driver data, so this structure also define some hooks specifically
// for VSI. For VSI-specific hooks, the format is ‘vf_*()’.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_si_ops {
    pub count): *mut *mut *mut *mut int (get_rss_table)(struct enetc_si si, u32 table, int,
    pub count): *const *const *const *const int (set_rss_table)(struct enetc_si si, u32 table, int,
    pub si): *mut *mut int (setup_cbdr)(struct enetc_si,
    pub si): *mut *mut void (teardown_cbdr)(struct enetc_si,
}

// PCI IEP device data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_si {
    pub pdev: *mut pci_dev,
    pub hw: enetc_hw,
    pub errata: enetc_errata,
    pub /: *mut *mut *mut net_device ndev; / back ref.,
// General-purpose lock serializing updates that must not race,
// e.g. read-modify-write of shared hardware registers and of
// selected priv->flags bits between the phylink link callbacks
// and the ring (re)configuration path.
//
    pub gen_lock: spinlock_t,
    pub /: *mut *mut enetc_cbdr cbd_ring; / Only ENETC 1.0,
    pub /: *mut *mut ntmp_user ntmp_user; / ENETC 4.1 and later,
}

pub const ENETC_SI_ALIGN: c_int = 32;
pub const ENETC_MAX_NUM_TXQS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_int_vector {
    pub rbier: *mut void __iomem,
    pub tbier_base: *mut void __iomem,
    pub ricr1: *mut void __iomem,
    pub tx_rings_map: c_ulong,
    pub count_tx_rings: c_int,
    pub rx_ictt: u32,
    pub comp_cnt: u16,
    pub rx_napi_work: bool rx_dim_en,,
    pub ____cacheline_aligned_in_smp: napi_napi,
    pub ____cacheline_aligned_in_smp: dim rx_dim,
    pub name: [c_char; ENETC_INT_NAME_MAX],
    pub rx_ring: enetc_bdr,
    pub __counted_by(count_tx_rings): enetc_bdr tx_ring[],
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_cls_rule {
    pub fs: ethtool_rx_flow_spec,
    pub used: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psfp_cap {
    pub max_streamid: u32,
    pub max_psfp_filter: u32,
    pub max_psfp_gate: u32,
    pub max_psfp_gatelist: u32,
    pub max_psfp_meter: u32,
}

pub const ENETC_F_TX_TSTAMP_MASK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_active_offloads {
// 8 bits reserved for TX timestamp types (hwtstamp_tx_types)
    ENETC_F_TX_TSTAMP		= BIT(0),
    ENETC_F_TX_ONESTEP_SYNC_TSTAMP	= BIT(1),

    ENETC_F_RX_TSTAMP		= BIT(8),
    ENETC_F_QBV			= BIT(9),
    ENETC_F_QCI			= BIT(10),
    ENETC_F_QBU			= BIT(11),
    ENETC_F_TXCSUM			= BIT(12),
    ENETC_F_LSO			= BIT(13),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_flags_bit {
    ENETC_TX_ONESTEP_TSTAMP_IN_PROGRESS = 0,
    ENETC_TX_DOWN,
    ENETC_RXBDR_CM,
}

// interrupt coalescing modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_ic_mode {
// one interrupt per frame
    ENETC_IC_NONE = 0,
// activated when int coalescing time is set to a non-0 value
    ENETC_IC_RX_MANUAL = BIT(0),
    ENETC_IC_TX_MANUAL = BIT(1),
// use dynamic interrupt moderation
    ENETC_IC_RX_ADAPTIVE = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_ndev_priv {
    pub ndev: *mut net_device,
    pub /: *mut *mut *mut device dev; / dma-mapping device,
    pub si: *mut enetc_si,
    pub /: *mut *mut int bdr_int_num; / number of Rx/Tx ring interrupts,
    pub int_vector: [*mut enetc_int_vector; ENETC_MAX_BDR_INT],
    pub num_tx_rings: u16 num_rx_rings,,
    pub tx_bd_count: u16 rx_bd_count,,
    pub msg_enable: u16,
    pub preemptible_tcs: u8,
    pub /: *mut *mut u8 max_frags; / The maximum number of BDs for fragments,
    pub active_offloads: enetc_active_offloads,
    pub /: *mut *mut u32 speed; / store speed for compare update pspeed,
    pub xdp_tx_ring: *mut enetc_bdr,
    pub tx_ring: [*mut enetc_bdr; 16],
    pub rx_ring: [*mut enetc_bdr; 16],
    pub tx_res: *const enetc_bdr_resource,
    pub rx_res: *const enetc_bdr_resource,
    pub cls_rules: *mut enetc_cls_rule,
    pub psfp_cap: psfp_cap,
// Minimum number of TX queues required by the network stack
    pub min_num_stack_tx_queues: c_uint,
    pub phylink: *mut phylink,
    pub ic_mode: c_int,
    pub tx_ictt: u32,
    pub xdp_prog: *mut bpf_prog,
    pub flags: c_ulong,
    pub tx_onestep_tstamp: work_struct,
    pub tx_skbs: sk_buff_head,
// Serialize access to MAC Merge state between ethtool requests
// and link state updates
//
    pub mm_lock: mutex,
    pub /: *mut *mut *mut clk ref_clk; / RGMII/RMII reference clock,
    pub /: *mut *mut u64 sysclk_freq; / NETC system clock frequency,
}

// SI common
extern "C" {
    pub fn enetc_port_mac_rd(si: *mut enetc_si, reg: u32) -> u32;
}
extern "C" {
    pub fn enetc_port_mac_wr(si: *mut enetc_si, reg: u32, val: u32);
}
extern "C" {
    pub fn enetc_pci_probe(pdev: *mut pci_dev, name: *const c_char, sizeof_priv: c_int) -> c_int;
}
extern "C" {
    pub fn enetc_pci_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn enetc_alloc_msix(priv: *mut enetc_ndev_priv) -> c_int;
}
extern "C" {
    pub fn enetc_free_msix(priv: *mut enetc_ndev_priv);
}
extern "C" {
    pub fn enetc_get_si_caps(si: *mut enetc_si);
}
extern "C" {
    pub fn enetc_init_si_rings_params(priv: *mut enetc_ndev_priv);
}
extern "C" {
    pub fn enetc_alloc_si_resources(priv: *mut enetc_ndev_priv) -> c_int;
}
extern "C" {
    pub fn enetc_free_si_resources(priv: *mut enetc_ndev_priv);
}
extern "C" {
    pub fn enetc_configure_si(priv: *mut enetc_ndev_priv) -> c_int;
}
extern "C" {
    pub fn enetc_get_driver_data(si: *mut enetc_si) -> c_int;
}
extern "C" {
    pub fn enetc_reset_mac_addr_filter(filter: *mut enetc_mac_filter);
}
extern "C" {
    pub fn enetc_set_congestion_mode(priv: *mut enetc_ndev_priv, enable: bool);
}
extern "C" {
    pub fn enetc_open(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn enetc_close(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn enetc_start(ndev: *mut net_device);
}
extern "C" {
    pub fn enetc_stop(ndev: *mut net_device);
}
extern "C" {
    pub fn enetc_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn enetc_set_features(ndev: *mut net_device, features: netdev_features_t);
}
extern "C" {
    pub fn enetc_ioctl(ndev: *mut net_device, rq: *mut ifreq, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn enetc_setup_tc_mqprio(ndev: *mut net_device, type_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn enetc_reset_tc_mqprio(ndev: *mut net_device);
}
extern "C" {
    pub fn enetc_setup_bpf(ndev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
// ethtool
extern "C" {
    pub fn enetc_set_ethtool_ops(ndev: *mut net_device);
}
extern "C" {
    pub fn enetc_mm_link_state_update(priv: *mut enetc_ndev_priv, link: bool);
}
extern "C" {
    pub fn enetc_mm_commit_preemptible_tcs(priv: *mut enetc_ndev_priv);
}
// control buffer descriptor ring (CBDR)
extern "C" {
    pub fn enetc_setup_cbdr(si: *mut enetc_si) -> c_int;
}
extern "C" {
    pub fn enetc_teardown_cbdr(si: *mut enetc_si);
}
extern "C" {
    pub fn enetc4_setup_cbdr(si: *mut enetc_si) -> c_int;
}
extern "C" {
    pub fn enetc4_teardown_cbdr(si: *mut enetc_si);
}
extern "C" {
    pub fn enetc_clear_mac_flt_entry(si: *mut enetc_si, index: c_int) -> c_int;
}
extern "C" {
    pub fn enetc_set_rss_key(si: *mut enetc_si, bytes: *const u8);
}
extern "C" {
    pub fn enetc_get_rss_table(si: *mut enetc_si, table: *mut u32, count: c_int) -> c_int;
}
extern "C" {
    pub fn enetc_set_rss_table(si: *mut enetc_si, table: *const u32, count: c_int) -> c_int;
}
extern "C" {
    pub fn enetc_send_cmd(si: *mut enetc_si, cbd: *mut enetc_cbd) -> c_int;
}
extern "C" {
    pub fn enetc4_get_rss_table(si: *mut enetc_si, table: *mut u32, count: c_int) -> c_int;
}
extern "C" {
    pub fn enetc4_set_rss_table(si: *mut enetc_si, table: *const u32, count: c_int) -> c_int;
}
// data_align = PTR_ALIGN(data, ENETC_CBD_DATA_MEM_ALIGN);
extern "C" {
    pub fn enetc_reset_ptcmsdur(hw: *mut enetc_hw);
}
extern "C" {
    pub fn enetc_set_ptcmsdur(hw: *mut enetc_hw, queue_max_sdu: *mut u32);
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_FSL_ENETC_PTP_CLOCK) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_PTP_NETC_V4_TIMER) -> return;
}

extern "C" {
    pub fn enetc_qos_query_caps(ndev: *mut net_device, type_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn enetc_setup_tc_taprio(ndev: *mut net_device, type_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn enetc_sched_speed_set(priv: *mut enetc_ndev_priv, speed: c_int);
}
extern "C" {
    pub fn enetc_setup_tc_cbs(ndev: *mut net_device, type_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn enetc_setup_tc_txtime(ndev: *mut net_device, type_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn enetc_setup_tc_psfp(ndev: *mut net_device, type_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn enetc_psfp_init(priv: *mut enetc_ndev_priv) -> c_int;
}
extern "C" {
    pub fn enetc_psfp_clean(priv: *mut enetc_ndev_priv) -> c_int;
}
extern "C" {
    pub fn enetc_set_psfp(ndev: *mut net_device, en: bool) -> c_int;
}
// Port stream filter capability
// Port stream gate capability
// Port flow meter capability

