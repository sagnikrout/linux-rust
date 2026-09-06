//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3_enet.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2016-2017 Hisilicon Limited.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_nic_state {
    HNS3_NIC_STATE_TESTING,
    HNS3_NIC_STATE_RESETTING,
    HNS3_NIC_STATE_INITED,
    HNS3_NIC_STATE_DOWN,
    HNS3_NIC_STATE_DISABLED,
    HNS3_NIC_STATE_REMOVING,
    HNS3_NIC_STATE_SERVICE_INITED,
    HNS3_NIC_STATE_SERVICE_SCHED,
    HNS3_NIC_STATE2_RESET_REQUESTED,
    HNS3_NIC_STATE_HW_TX_CSUM_ENABLE,
    HNS3_NIC_STATE_RXD_ADV_LAYOUT_ENABLE,
    HNS3_NIC_STATE_TX_PUSH_ENABLE,
    HNS3_NIC_STATE_MAX
}

pub const HNS3_MAX_PUSH_BD_NUM: c_int = 2;
pub const HNS3_RING_RX_RING_BASEADDR_L_REG: c_uint = 0x00000;
pub const HNS3_RING_RX_RING_BASEADDR_H_REG: c_uint = 0x00004;
pub const HNS3_RING_RX_RING_BD_NUM_REG: c_uint = 0x00008;
pub const HNS3_RING_RX_RING_BD_LEN_REG: c_uint = 0x0000C;
pub const HNS3_RING_RX_RING_TAIL_REG: c_uint = 0x00018;
pub const HNS3_RING_RX_RING_HEAD_REG: c_uint = 0x0001C;
pub const HNS3_RING_RX_RING_FBDNUM_REG: c_uint = 0x00020;
pub const HNS3_RING_RX_RING_PKTNUM_RECORD_REG: c_uint = 0x0002C;
pub const HNS3_RING_TX_RING_BASEADDR_L_REG: c_uint = 0x00040;
pub const HNS3_RING_TX_RING_BASEADDR_H_REG: c_uint = 0x00044;
pub const HNS3_RING_TX_RING_BD_NUM_REG: c_uint = 0x00048;
pub const HNS3_RING_TX_RING_TC_REG: c_uint = 0x00050;
pub const HNS3_RING_TX_RING_TAIL_REG: c_uint = 0x00058;
pub const HNS3_RING_TX_RING_HEAD_REG: c_uint = 0x0005C;
pub const HNS3_RING_TX_RING_FBDNUM_REG: c_uint = 0x00060;
pub const HNS3_RING_TX_RING_OFFSET_REG: c_uint = 0x00064;
pub const HNS3_RING_TX_RING_EBDNUM_REG: c_uint = 0x00068;
pub const HNS3_RING_TX_RING_PKTNUM_RECORD_REG: c_uint = 0x0006C;
pub const HNS3_RING_TX_RING_EBD_OFFSET_REG: c_uint = 0x00070;
pub const HNS3_RING_TX_RING_BD_ERR_REG: c_uint = 0x00074;
pub const HNS3_RING_EN_REG: c_uint = 0x00090;
pub const HNS3_RING_RX_EN_REG: c_uint = 0x00098;
pub const HNS3_RING_TX_EN_REG: c_uint = 0x000D4;
pub const HNS3_RX_HEAD_SIZE: c_int = 256;

pub const HNS3_RING_NAME_LEN: c_int = 16;
pub const HNS3_BUFFER_SIZE_2048: c_int = 2048;
pub const HNS3_RING_MAX_PENDING: c_int = 32760;
pub const HNS3_RING_MIN_PENDING: c_int = 72;
pub const HNS3_RING_BD_MULTIPLE: c_int = 8;
// max frame size of mac

pub const HNS3_BD_SIZE_512_TYPE: c_int = 0;
pub const HNS3_BD_SIZE_1024_TYPE: c_int = 1;
pub const HNS3_BD_SIZE_2048_TYPE: c_int = 2;
pub const HNS3_BD_SIZE_4096_TYPE: c_int = 3;
pub const HNS3_RX_FLAG_VLAN_PRESENT: c_uint = 0x1;
pub const HNS3_RX_FLAG_L3ID_IPV4: c_uint = 0x0;
pub const HNS3_RX_FLAG_L3ID_IPV6: c_uint = 0x1;
pub const HNS3_RX_FLAG_L4ID_UDP: c_uint = 0x0;
pub const HNS3_RX_FLAG_L4ID_TCP: c_uint = 0x1;
pub const HNS3_RXD_DMAC_S: c_int = 0;

pub const HNS3_RXD_VLAN_S: c_int = 2;

pub const HNS3_RXD_L3ID_S: c_int = 4;

pub const HNS3_RXD_L4ID_S: c_int = 8;

pub const HNS3_RXD_FRAG_B: c_int = 12;
pub const HNS3_RXD_STRP_TAGP_S: c_int = 13;

pub const HNS3_RXD_L2E_B: c_int = 16;
pub const HNS3_RXD_L3E_B: c_int = 17;
pub const HNS3_RXD_L4E_B: c_int = 18;
pub const HNS3_RXD_TRUNCAT_B: c_int = 19;
pub const HNS3_RXD_HOI_B: c_int = 20;
pub const HNS3_RXD_DOI_B: c_int = 21;
pub const HNS3_RXD_OL3E_B: c_int = 22;
pub const HNS3_RXD_OL4E_B: c_int = 23;
pub const HNS3_RXD_GRO_COUNT_S: c_int = 24;

pub const HNS3_RXD_GRO_FIXID_B: c_int = 30;
pub const HNS3_RXD_GRO_ECN_B: c_int = 31;
pub const HNS3_RXD_ODMAC_S: c_int = 0;

pub const HNS3_RXD_OVLAN_S: c_int = 2;

pub const HNS3_RXD_OL3ID_S: c_int = 4;

pub const HNS3_RXD_OL4ID_S: c_int = 8;

pub const HNS3_RXD_FBHI_S: c_int = 12;

pub const HNS3_RXD_FBLI_S: c_int = 14;

pub const HNS3_RXD_PTYPE_S: c_int = 4;

pub const HNS3_RXD_BDTYPE_S: c_int = 0;

pub const HNS3_RXD_VLD_B: c_int = 4;
pub const HNS3_RXD_UDP0_B: c_int = 5;
pub const HNS3_RXD_EXTEND_B: c_int = 7;
pub const HNS3_RXD_FE_B: c_int = 8;
pub const HNS3_RXD_LUM_B: c_int = 9;
pub const HNS3_RXD_CRCP_B: c_int = 10;
pub const HNS3_RXD_L3L4P_B: c_int = 11;
pub const HNS3_RXD_TSIDX_S: c_int = 12;

pub const HNS3_RXD_TS_VLD_B: c_int = 14;
pub const HNS3_RXD_LKBK_B: c_int = 15;
pub const HNS3_RXD_GRO_SIZE_S: c_int = 16;

pub const HNS3_TXD_L3T_S: c_int = 0;

pub const HNS3_TXD_L4T_S: c_int = 2;

pub const HNS3_TXD_L3CS_B: c_int = 4;
pub const HNS3_TXD_L4CS_B: c_int = 5;
pub const HNS3_TXD_VLAN_B: c_int = 6;
pub const HNS3_TXD_TSO_B: c_int = 7;
pub const HNS3_TXD_L2LEN_S: c_int = 8;

pub const HNS3_TXD_L3LEN_S: c_int = 16;

pub const HNS3_TXD_L4LEN_S: c_int = 24;

pub const HNS3_TXD_CSUM_START_S: c_int = 8;

pub const HNS3_TXD_OL3T_S: c_int = 0;

pub const HNS3_TXD_OVLAN_B: c_int = 2;
pub const HNS3_TXD_MACSEC_B: c_int = 3;
pub const HNS3_TXD_TUNTYPE_S: c_int = 4;

pub const HNS3_TXD_CSUM_OFFSET_S: c_int = 8;

pub const HNS3_TXD_BDTYPE_S: c_int = 0;

pub const HNS3_TXD_FE_B: c_int = 4;
pub const HNS3_TXD_SC_S: c_int = 5;

pub const HNS3_TXD_EXTEND_B: c_int = 7;
pub const HNS3_TXD_VLD_B: c_int = 8;
pub const HNS3_TXD_RI_B: c_int = 9;
pub const HNS3_TXD_RA_B: c_int = 10;
pub const HNS3_TXD_TSYN_B: c_int = 11;
pub const HNS3_TXD_DECTTL_S: c_int = 12;

pub const HNS3_TXD_OL4CS_B: c_int = 22;
pub const HNS3_TXD_MSS_S: c_int = 0;

pub const HNS3_TXD_HW_CS_B: c_int = 14;

pub const HNS3_VECTOR_NOT_INITED: c_int = 0;
pub const HNS3_VECTOR_INITED: c_int = 1;
pub const HNS3_MAX_BD_SIZE: c_int = 65535;

pub const HNS3_VECTOR_GL0_OFFSET: c_uint = 0x100;
pub const HNS3_VECTOR_GL1_OFFSET: c_uint = 0x200;
pub const HNS3_VECTOR_GL2_OFFSET: c_uint = 0x300;
pub const HNS3_VECTOR_RL_OFFSET: c_uint = 0x900;
pub const HNS3_VECTOR_RL_EN_B: c_int = 6;

pub const HNS3_VECTOR_TX_QL_OFFSET: c_uint = 0xe00;
pub const HNS3_VECTOR_RX_QL_OFFSET: c_uint = 0xf00;
pub const HNS3_RING_EN_B: c_int = 0;
pub const HNS3_GL0_CQ_MODE_REG: c_uint = 0x20d00;
pub const HNS3_GL1_CQ_MODE_REG: c_uint = 0x20d04;
pub const HNS3_GL2_CQ_MODE_REG: c_uint = 0x20d08;

pub const HNS3_RESCHED_BD_NUM: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_l2t_type {
    HNS3_L2_TYPE_UNICAST,
    HNS3_L2_TYPE_MULTICAST,
    HNS3_L2_TYPE_BROADCAST,
    HNS3_L2_TYPE_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_l3t_type {
    HNS3_L3T_NONE,
    HNS3_L3T_IPV6,
    HNS3_L3T_IPV4,
    HNS3_L3T_RESERVED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_l4t_type {
    HNS3_L4T_UNKNOWN,
    HNS3_L4T_TCP,
    HNS3_L4T_UDP,
    HNS3_L4T_SCTP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_ol3t_type {
    HNS3_OL3T_NONE,
    HNS3_OL3T_IPV6,
    HNS3_OL3T_IPV4_NO_CSUM,
    HNS3_OL3T_IPV4_CSUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_tun_type {
    HNS3_TUN_NONE,
    HNS3_TUN_MAC_IN_UDP,
    HNS3_TUN_NVGRE,
    HNS3_TUN_OTHER
}

// hardware spec ring buffer format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_desc_type {
    DESC_TYPE_UNKNOWN		= 0,
    DESC_TYPE_SKB			= 1 << 0,
    DESC_TYPE_FRAGLIST_SKB		= 1 << 1,
    DESC_TYPE_PAGE			= 1 << 2,
    DESC_TYPE_BOUNCE_ALL		= 1 << 3,
    DESC_TYPE_BOUNCE_HEAD		= 1 << 4,
    DESC_TYPE_SGL_SKB		= 1 << 5,
    DESC_TYPE_PP_FRAG		= 1 << 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_desc_cb {
    pub /: *mut *mut dma_addr_t dma; / dma address of this desc,
    pub /: *mut *mut *mut void buf; / cpu addr for a desc,
// priv data for the desc, e.g. skb when use with ip stack
    pub priv: *mut c_void,
    pub /: *mut *mut u32 page_offset; / for rx,
    pub /: *mut *mut u32 send_bytes; / for tx,
}

// desc type, used by the ring user to mark the type of the priv data
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_l3type {
    HNS3_L3_TYPE_IPV4,
    HNS3_L3_TYPE_IPV6,
    HNS3_L3_TYPE_ARP,
    HNS3_L3_TYPE_RARP,
    HNS3_L3_TYPE_IPV4_OPT,
    HNS3_L3_TYPE_IPV6_EXT,
    HNS3_L3_TYPE_LLDP,
    HNS3_L3_TYPE_BPDU,
    HNS3_L3_TYPE_MAC_PAUSE,
    HNS3_L3_TYPE_PFC_PAUSE, /* 0x9 */

// reserved for 0xA~0xB

    HNS3_L3_TYPE_CNM = 0xc,

// reserved for 0xD~0xE

    HNS3_L3_TYPE_PARSE_FAIL	= 0xf /* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_l4type {
    HNS3_L4_TYPE_UDP,
    HNS3_L4_TYPE_TCP,
    HNS3_L4_TYPE_GRE,
    HNS3_L4_TYPE_SCTP,
    HNS3_L4_TYPE_IGMP,
    HNS3_L4_TYPE_ICMP,

// reserved for 0x6~0xE

    HNS3_L4_TYPE_PARSE_FAIL	= 0xf /* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_ol3type {
    HNS3_OL3_TYPE_IPV4 = 0,
    HNS3_OL3_TYPE_IPV6,
// reserved for 0x2~0x3
    HNS3_OL3_TYPE_IPV4_OPT = 4,
    HNS3_OL3_TYPE_IPV6_EXT,

// reserved for 0x6~0xE

    HNS3_OL3_TYPE_PARSE_FAIL = 0xf	/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_pkt_ol4type {
    HNS3_OL4_TYPE_NO_TUN,
    HNS3_OL4_TYPE_MAC_IN_UDP,
    HNS3_OL4_TYPE_NVGRE,
    HNS3_OL4_TYPE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_rx_ptype {
    pub 8: u32 ptype :,
    pub 2: u32 csum_level :,
    pub 2: u32 ip_summed :,
    pub 4: u32 l3_type :,
    pub 1: u32 valid :,
    pub 3: u32 hash_type:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_stats {
    pub sw_err_cnt: u64,
    pub seg_pkt_cnt: u64,
    pub tx_pkts: u64,
    pub tx_bytes: u64,
    pub tx_more: u64,
    pub tx_push: u64,
    pub tx_mem_doorbell: u64,
    pub restart_queue: u64,
    pub tx_busy: u64,
    pub tx_copy: u64,
    pub tx_vlan_err: u64,
    pub tx_l4_proto_err: u64,
    pub tx_l2l3l4_err: u64,
    pub tx_tso_err: u64,
    pub over_max_recursion: u64,
    pub hw_limitation: u64,
    pub tx_bounce: u64,
    pub tx_spare_full: u64,
    pub copy_bits_err: u64,
    pub tx_sgl: u64,
    pub skb2sgl_err: u64,
    pub map_sg_err: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_tx_spare {
    pub dma: dma_addr_t,
    pub buf: *mut c_void,
    pub next_to_use: u32,
    pub next_to_clean: u32,
    pub last_to_clean: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_enet_ring {
    pub /: *mut *mut *mut hns3_desc desc; / dma map address space,
    pub desc_cb: *mut hns3_desc_cb,
    pub next: *mut hns3_enet_ring,
    pub tqp_vector: *mut hns3_enet_tqp_vector,
    pub tqp: *mut hnae3_queue,
    pub queue_index: c_int,
    pub /: *mut *mut *mut device dev; / will be used for DMA mapping of descriptors,
    pub page_pool: *mut page_pool,
// statistic
    pub stats: ring_stats,
    pub syncp: u64_stats_sync,
    pub desc_dma_addr: dma_addr_t,
    pub /: *mut *mut u32 buf_size; / size for hnae_desc->addr, preset by AE,
    pub /: *mut *mut u16 desc_num; / total number of desc,
    pub /: *mut *mut int next_to_use; / idx of next spare desc,
// idx of lastest sent desc, the ring is empty when equal to
// next_to_use
//
    pub next_to_clean: c_int,
    pub /: *mut *mut u32 flag; / ring attribute,
    pub pending_buf: c_int,
// for Tx ring
    pub fd_qb_tx_sample: u32,
    pub /: *mut *mut int last_to_use; / last idx used by xmit,
    pub tx_copybreak: u32,
    pub tx_spare: *mut hns3_tx_spare,
}

// for Rx ring
// first buffer address for current packet
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_flow_level_range {
    HNS3_FLOW_LOW = 0,
    HNS3_FLOW_MID = 1,
    HNS3_FLOW_HIGH = 2,
    HNS3_FLOW_ULTRA = 3,
}

pub const HNS3_INT_GL_50K: c_uint = 0x0014;
pub const HNS3_INT_GL_20K: c_uint = 0x0032;
pub const HNS3_INT_GL_18K: c_uint = 0x0036;
pub const HNS3_INT_GL_8K: c_uint = 0x007C;

pub const HNS3_INT_RL_MAX: c_uint = 0x00EC;
pub const HNS3_INT_RL_ENABLE_MASK: c_uint = 0x40;
pub const HNS3_INT_QL_DEFAULT_CFG: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_enet_coalesce {
    pub int_gl: u16,
    pub int_ql: u16,
    pub int_ql_max: u16,
    pub 1: u8 adapt_enable :,
    pub 1: u8 ql_enable :,
    pub 1: u8 unit_1us :,
    pub flow_level: hns3_flow_level_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_enet_ring_group {
// array of pointers to rings
    pub ring: *mut hns3_enet_ring,
    pub /: *mut *mut u64 total_bytes; / total bytes processed this group,
    pub /: *mut *mut u64 total_packets; / total packets processed this group,
    pub count: u16,
    pub coal: hns3_enet_coalesce,
    pub dim: dim,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_enet_tqp_vector {
    pub handle: *mut hnae3_handle,
    pub mask_addr: *mut u8 __iomem,
    pub vector_irq: c_int,
    pub irq_init_flag: c_int,
    pub /: *mut *mut u16 idx; / index in the TQP vector array per handle.,
    pub napi: napi_struct,
    pub rx_group: hns3_enet_ring_group,
    pub tx_group: hns3_enet_ring_group,
    pub affinity_mask: cpumask_t,
    pub /: *mut *mut u16 num_tqps; / total number of tqps in TQP vector,
    pub affinity_notify: irq_affinity_notify,
    pub name: [c_char; HNAE3_INT_NAME_LEN],
    pub event_cnt: u64,
    pub ____cacheline_internodealigned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_nic_priv {
    pub ae_handle: *mut hnae3_handle,
    pub netdev: *mut net_device,
    pub dev: *mut device,
//
// the cb for nic to manage the ring buffer, the first half of the
// array is for tx_ring and vice versa for the second half
//
    pub ring: *mut hns3_enet_ring,
    pub tqp_vector: *mut hns3_enet_tqp_vector,
    pub vector_num: u16,
    pub max_non_tso_bd_num: u8,
    pub tx_timeout_count: u64,
    pub state: c_ulong,
    pub tx_cqe_mode: dim_cq_period_mode,
    pub rx_cqe_mode: dim_cq_period_mode,
    pub tx_coal: hns3_enet_coalesce,
    pub rx_coal: hns3_enet_coalesce,
    pub tx_copybreak: u32,
    pub rx_copybreak: u32,
    pub min_tx_copybreak: u32,
    pub min_tx_spare_buf_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union l3_hdr_info {
    pub v4: *mut iphdr,
    pub v6: *mut ipv6hdr,
    pub hdr: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union l4_hdr_info {
    pub tcp: *mut tcphdr,
    pub udp: *mut udphdr,
    pub gre: *mut gre_base_hdr,
    pub hdr: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_hw_error_info {
    pub type: hnae3_hw_error_type,
    pub msg: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_reset_type_map {
    pub rst_flags: ethtool_reset_flags,
    pub rst_type: hnae3_reset_type,
}

// This smp_load_acquire() pairs with smp_store_release() in
// hns3_nic_reclaim_one_desc called by hns3_clean_tx_ring.
//
extern "C" {
    pub fn readl_relaxed(reg: ring->tqp->io_base +) -> return;
}
extern "C" {
    pub fn readl(reg: base +) -> return;
}

extern "C" {
    pub fn test_bit(_arg: HNS3_NIC_STATE_RESETTING, _arg: &priv->state) -> return;
}

// iterator for handling rings in ring group

extern "C" {
    pub fn hns3_ethtool_set_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn hns3_clean_tx_ring(ring: *mut hns3_enet_ring, budget: c_int);
}
extern "C" {
    pub fn hns3_init_all_ring(priv: *mut hns3_nic_priv) -> c_int;
}
extern "C" {
    pub fn hns3_nic_reset_all_ring(h: *mut hnae3_handle) -> c_int;
}
extern "C" {
    pub fn hns3_fini_ring(ring: *mut hns3_enet_ring);
}
extern "C" {
    pub fn hns3_nic_net_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn hns3_is_phys_func(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn hns3_request_update_promisc_mode(handle: *mut hnae3_handle);
}

extern "C" {
    pub fn hns3_dcbnl_setup(handle: *mut hnae3_handle);
}

extern "C" {
    pub fn hns3_dbg_init(handle: *mut hnae3_handle) -> c_int;
}
extern "C" {
    pub fn hns3_dbg_uninit(handle: *mut hnae3_handle);
}
extern "C" {
    pub fn hns3_dbg_register_debugfs(debugfs_dir_name: *const c_char);
}
extern "C" {
    pub fn hns3_dbg_unregister_debugfs();
}
extern "C" {
    pub fn hns3_shinfo_pack(shinfo: *mut skb_shared_info, size: *mut __u32);
}
extern "C" {
    pub fn hns3_get_max_available_channels(h: *mut hnae3_handle) -> u16;
}
extern "C" {
    pub fn hns3_external_lb_prepare(ndev: *mut net_device, if_running: bool);
}
extern "C" {
    pub fn hns3_external_lb_restore(ndev: *mut net_device, if_running: bool);
}
