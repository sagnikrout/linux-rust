//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/airoha/airoha_eth.h
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
// Copyright (c) 2024 AIROHA Inc
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
//

pub const AIROHA_MAX_NUM_GDM_PORTS: c_int = 4;
pub const AIROHA_MAX_NUM_GDM_DEVS: c_int = 2;
pub const AIROHA_MAX_NUM_QDMA: c_int = 2;
pub const AIROHA_MAX_NUM_IRQ_BANKS: c_int = 4;
pub const AIROHA_MAX_DSA_PORTS: c_int = 7;
pub const AIROHA_MAX_NUM_RSTS: c_int = 3;
pub const AIROHA_MAX_MTU: c_int = 9220;
pub const AIROHA_MAX_RX_SIZE: c_int = 16128;
pub const AIROHA_MAX_PACKET_SIZE: c_int = 2048;
pub const AIROHA_NUM_QOS_CHANNELS: c_int = 4;
pub const AIROHA_NUM_QOS_QUEUES: c_int = 8;
pub const AIROHA_NUM_TX_RING: c_int = 32;
pub const AIROHA_NUM_RX_RING: c_int = 32;

pub const AIROHA_FE_MC_MAX_VLAN_TABLE: c_int = 64;
pub const AIROHA_FE_MC_MAX_VLAN_PORT: c_int = 16;
pub const AIROHA_NUM_TX_IRQ: c_int = 2;

pub const HW_DSCP_NUM: c_int = 2048;

pub const TX_DSCP_NUM: c_int = 1024;

pub const PSE_RSV_PAGES: c_int = 128;
pub const PSE_QUEUE_RSV_PAGES: c_int = 64;

pub const PPE_ENTRY_SIZE: c_int = 80;

pub const MTK_HDR_LEN: c_int = 4;
pub const MTK_HDR_XMIT_TAGGED_TPID_8100: c_int = 1;
pub const MTK_HDR_XMIT_TAGGED_TPID_88A8: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_gdm_index {
    AIROHA_GDM1_IDX = 1,
    AIROHA_GDM2_IDX = 2,
    AIROHA_GDM3_IDX = 3,
    AIROHA_GDM4_IDX = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_sched_mode {
    TC_SCH_WRR8,
    TC_SCH_SP,
    TC_SCH_WRR7,
    TC_SCH_WRR6,
    TC_SCH_WRR5,
    TC_SCH_WRR4,
    TC_SCH_WRR3,
    TC_SCH_WRR2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trtcm_unit_type {
    TRTCM_BYTE_UNIT,
    TRTCM_PACKET_UNIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trtcm_param_type {
    TRTCM_MISC_MODE, /* meter_en, pps_mode, tick_sel */
    TRTCM_TOKEN_RATE_MODE,
    TRTCM_BUCKETSIZE_SHIFT_MODE,
    TRTCM_BUCKET_COUNTER_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trtcm_mode_type {
    TRTCM_COMMIT_MODE,
    TRTCM_PEAK_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trtcm_param {
    TRTCM_TICK_SEL = BIT(0),
    TRTCM_PKT_MODE = BIT(1),
    TRTCM_METER_MODE = BIT(2),
}

pub const MIN_TOKEN_SIZE: c_int = 4096;
pub const MAX_TOKEN_SIZE_OFFSET: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_dma_map_type {
    AIROHA_DMA_UNMAPPED,
    AIROHA_DMA_MAP_SINGLE,
    AIROHA_DMA_MAP_PAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_queue_entry {
    pub buf: *mut c_void,
    pub list: list_head,
    pub skb: *mut sk_buff,
    pub dma_type: airoha_dma_map_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_queue {
    pub qdma: *mut airoha_qdma,
// protect concurrent queue accesses
    pub lock: spinlock_t,
    pub entry: *mut airoha_queue_entry,
    pub desc: *mut airoha_qdma_desc,
    pub head: u16,
    pub tail: u16,
    pub queued: c_int,
    pub ndesc: c_int,
    pub free_thr: c_int,
    pub buf_size: c_int,
    pub txq_stopped: bool,
    pub flushing: bool,
    pub napi: napi_struct,
    pub page_pool: *mut page_pool,
    pub skb: *mut sk_buff,
    pub tx_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_tx_irq_queue {
    pub qdma: *mut airoha_qdma,
    pub napi: napi_struct,
    pub size: c_int,
    pub q: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_hw_stats {
    pub syncp: u64_stats_sync,
// get_stats64
    pub rx_ok_pkts: u64,
    pub tx_ok_pkts: u64,
    pub rx_ok_bytes: u64,
    pub tx_ok_bytes: u64,
    pub rx_multicast: u64,
    pub rx_errors: u64,
    pub rx_drops: u64,
    pub tx_drops: u64,
    pub rx_crc_error: u64,
    pub rx_over_errors: u64,
// ethtool stats
    pub tx_broadcast: u64,
    pub tx_multicast: u64,
    pub tx_len: [u64; 7],
    pub rx_broadcast: u64,
    pub rx_fragment: u64,
    pub rx_jabber: u64,
    pub rx_len: [u64; 7],
// Previous HW register values for 32-bit counter delta
// tracking. Storing the last seen value and accumulating
// (u32)(curr - prev) into the 64-bit software counter
// handles wrap-around transparently via unsigned arithmetic.
// tx_runt64/rx_runt64 hold the running sum of runt deltas.
// These fields are never reported to userspace.
//
    pub tx_drops: u32,
    pub tx_broadcast: u32,
    pub tx_multicast: u32,
    pub tx_runt: u32,
    pub tx_long: u32,
    pub tx_runt64: u64,
    pub rx_drops: u32,
    pub rx_broadcast: u32,
    pub rx_multicast: u32,
    pub rx_errors: u32,
    pub rx_crc_error: u32,
    pub rx_over_errors: u32,
    pub rx_fragment: u32,
    pub rx_jabber: u32,
    pub rx_runt: u32,
    pub rx_long: u32,
    pub rx_runt64: u64,
    pub mib_prev: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_mac_info_common {
    pub vlan1: u16,
    pub etype: u16,
    pub dest_mac_hi: u32,
    pub vlan2: u16,
    pub dest_mac_lo: u16,
    pub src_mac_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_mac_info {
    pub common: airoha_foe_mac_info_common,
    pub pppoe_id: u16,
    pub src_mac_lo: u16,
    pub meter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_bridge {
    pub dest_mac_hi: u32,
    pub src_mac_hi: u16,
    pub dest_mac_lo: u16,
    pub src_mac_lo: u32,
    pub ib2: u32,
    pub rsv: [u32; 5],
    pub data: u32,
    pub l2: airoha_foe_mac_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_ipv4_tuple {
    pub src_ip: u32,
    pub dest_ip: u32,
    pub dest_port: u16,
    pub src_port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_ipv4 {
    pub orig_tuple: airoha_foe_ipv4_tuple,
    pub ib2: u32,
    pub new_tuple: airoha_foe_ipv4_tuple,
    pub rsv: [u32; 2],
    pub data: u32,
    pub l2: airoha_foe_mac_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_ipv4_dslite {
    pub ip4: airoha_foe_ipv4_tuple,
    pub ib2: u32,
    pub flow_label: [u8; 3],
    pub priority: u8,
    pub rsv: [u32; 4],
    pub data: u32,
    pub l2: airoha_foe_mac_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_ipv6 {
    pub src_ip: [u32; 4],
    pub dest_ip: [u32; 4],
    pub dest_port: u16,
    pub src_port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_entry {
    pub ib1: u32,
    pub bridge: airoha_foe_bridge,
    pub ipv4: airoha_foe_ipv4,
    pub dslite: airoha_foe_ipv4_dslite,
    pub ipv6: airoha_foe_ipv6,
    pub d): DECLARE_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_stats {
    pub bytes: u32,
    pub packets: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_foe_stats64 {
    pub bytes: u64,
    pub packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_flow_data {
    pub eth: ethhdr,
    pub src_addr: __be32,
    pub dst_addr: __be32,
    pub v4: },
    pub src_addr: in6_addr,
    pub dst_addr: in6_addr,
    pub v6: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_flow_entry_type {
    FLOW_TYPE_L4,
    FLOW_TYPE_L2,
    FLOW_TYPE_L2_SUBFLOW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_flow_table_entry {
    pub /: *mut *mut hlist_node list; / PPE L3 flow entry,
    pub /: *mut *mut rhash_head l2_node; / L2 flow entry,
    pub /: *mut *mut hlist_head l2_flows; / PPE L2 subflows list,
}

// Must be last --ends in a flexible-array member.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_wdma_info {
    pub idx: u8,
    pub queue: u8,
    pub wcid: u16,
    pub bss: u8,
}

// RX queue to IRQ mapping: BIT(q) in IRQ(n)
pub const RX_IRQ0_BANK_PIN_MASK: c_uint = 0x839f;
pub const RX_IRQ1_BANK_PIN_MASK: c_uint = 0xffe00000;
pub const RX_IRQ2_BANK_PIN_MASK: c_uint = 0x20;
pub const RX_IRQ3_BANK_PIN_MASK: c_uint = 0x40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_irq_bank {
    pub qdma: *mut airoha_qdma,
// protect concurrent irqmask accesses
    pub irq_lock: spinlock_t,
    pub irqmask: [u32; QDMA_INT_REG_MAX],
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_qdma {
    pub eth: *mut airoha_eth,
    pub regs: *mut void __iomem,
    pub irq_banks: [airoha_irq_bank; AIROHA_MAX_NUM_IRQ_BANKS],
    pub q_tx_irq: [airoha_tx_irq_queue; AIROHA_NUM_TX_IRQ],
    pub q_tx: [airoha_queue; AIROHA_NUM_TX_RING],
    pub q_rx: [airoha_queue; AIROHA_NUM_RX_RING],
    pub AIROHA_NUM_QOS_CHANNELS): DECLARE_BITMAP(qos_channel_map,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_dev_flags {
    AIROHA_DEV_F_WAN,
    AIROHA_DEV_F_TX_QOS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_gdm_dev {
    pub qdma: *mut airoha_qdma __rcu,
    pub port: *mut airoha_gdm_port,
    pub eth: *mut airoha_eth,
    pub AIROHA_NUM_QOS_CHANNELS): DECLARE_BITMAP(qos_sq_bmap,,
    pub cpu_tx_packets: u32,
    pub fwd_tx_packets: u32,
    pub qos_stats: [}; AIROHA_NUM_QOS_CHANNELS],
    pub flags: c_ulong,
    pub nbq: c_int,
    pub stats: airoha_hw_stats,
// Serialize netdev_tx_completed_queue() calls per TX queue during
// QDMA migration.
//
    pub txq_lock: [spinlock_t; AIROHA_NUM_NETDEV_TX_RINGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_gdm_port {
    pub devs: [*mut airoha_gdm_dev; AIROHA_MAX_NUM_GDM_DEVS],
    pub id: c_int,
    pub users: c_int,
// protect concurrent hw_stats accesses
    pub stats_lock: spinlock_t,
    pub dsa_meta: [*mut metadata_dst; AIROHA_MAX_DSA_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_ppe {
    pub dev: airoha_ppe_dev,
    pub eth: *mut airoha_eth,
    pub foe: *mut c_void,
    pub foe_dma: dma_addr_t,
    pub l2_flows: rhashtable,
    pub foe_flow: *mut hlist_head,
    pub foe_check_time: *mut u16,
    pub foe_stats: *mut airoha_foe_stats,
    pub foe_stats_dma: dma_addr_t,
    pub debugfs_dir: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_eth_soc_data {
    pub version: u16,
    pub xsi_rsts_names: *const *const c_char,
    pub num_xsi_rsts: c_int,
    pub num_ppe: c_int,
    pub nbq): *mut *mut *mut int (get_sport)(struct airoha_gdm_port port, int,
    pub nbq): *mut *mut *mut u32 (get_vip_port)(struct airoha_gdm_port port, int,
    pub dev): *mut *mut u16 port, u16,
    pub ops: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_eth {
    pub dev: *mut device,
    pub soc: *const airoha_eth_soc_data,
    pub state: c_ulong,
    pub fe_regs: *mut void __iomem,
    pub npu: *mut airoha_npu __rcu,
    pub ppe: *mut airoha_ppe,
    pub flow_table: rhashtable,
    pub rsts: [reset_control_bulk_data; AIROHA_MAX_NUM_RSTS],
    pub xsi_rsts: *mut reset_control_bulk_data,
    pub napi_dev: *mut net_device,
    pub qdma: [airoha_qdma; AIROHA_MAX_NUM_QDMA],
    pub ports: [*mut airoha_gdm_port; AIROHA_MAX_NUM_GDM_PORTS],
}

extern "C" {
    pub fn airoha_rr(base: *mut void __iomem, offset: u32) -> u32;
}
extern "C" {
    pub fn airoha_wr(base: *mut void __iomem, offset: u32, val: u32);
}
extern "C" {
    pub fn airoha_rmw(base: *mut void __iomem, offset: u32, mask: u32, val: u32) -> u32;
}

extern "C" {
    pub fn airoha_get_fe_port(dev: *mut airoha_gdm_dev) -> c_int;
}
extern "C" {
    pub fn airoha_ppe_set_xmit_frame_size(dev: *mut airoha_gdm_dev);
}
extern "C" {
    pub fn airoha_ppe_set_cpu_port(dev: *mut airoha_gdm_dev, ppe_id: u8, fport: u8);
}
extern "C" {
    pub fn airoha_ppe_is_enabled(eth: *mut airoha_eth, index: c_int) -> bool;
}
extern "C" {
    pub fn airoha_ppe_setup_tc_block_cb(dev: *mut airoha_ppe_dev, type_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn airoha_ppe_init(eth: *mut airoha_eth) -> c_int;
}
extern "C" {
    pub fn airoha_ppe_deinit(eth: *mut airoha_eth);
}
extern "C" {
    pub fn airoha_ppe_init_upd_mem(dev: *mut airoha_gdm_dev, addr: *const u8);
}
extern "C" {
    pub fn airoha_ppe_get_total_num_entries(ppe: *mut airoha_ppe) -> u32;
}

extern "C" {
    pub fn airoha_ppe_debugfs_init(ppe: *mut airoha_ppe) -> c_int;
}

