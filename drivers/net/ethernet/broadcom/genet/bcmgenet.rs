//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/genet/bcmgenet.h
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
// Copyright (c) 2014-2025 Broadcom
//

// Maximum number of hardware queues, downsized if needed
pub const GENET_MAX_MQ_CNT: c_int = 4;
// total number of Buffer Descriptors, same for Rx/Tx
pub const TOTAL_DESC: c_int = 256;
// which ring is descriptor based
pub const DESC_INDEX: c_int = 16;
// Body(1500) + EH_SIZE(14) + VLANTAG(4) + BRCMTAG(6) + FCS(4) = 1528.
// 1536 is multiple of 256 bytes
//
pub const ENET_BRCM_TAG_LEN: c_int = 6;
pub const ENET_PAD: c_int = 8;

pub const DMA_MAX_BURST_LENGTH: c_uint = 0x10;
// misc. configuration
pub const MAX_NUM_OF_FS_RULES: c_int = 16;
pub const CLEAR_ALL_HFB: c_uint = 0xFF;

pub const DMA_FC_THRESH_LO: c_int = 5;
// 64B receive/transmit status block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_64 {
    pub /: *mut *mut u32 length_status; / length and peripheral status,
    pub status*/: *mut *mut u32 ext_status; / Extended,
    pub /: *mut *mut u32 rx_csum; / partial rx checksum,
    pub /: *mut *mut u32 unused1[9]; / unused,
    pub /: *mut *mut u32 tx_csum_info; / Tx checksum info.,
    pub /: *mut *mut u32 unused2[3]; / unused,
}

// Rx status bits
pub const STATUS_RX_EXT_MASK: c_uint = 0x1FFFFF;
pub const STATUS_RX_CSUM_MASK: c_uint = 0xFFFF;
pub const STATUS_RX_CSUM_OK: c_uint = 0x10000;
pub const STATUS_RX_CSUM_FR: c_uint = 0x20000;
pub const STATUS_RX_PROTO_TCP: c_int = 0;
pub const STATUS_RX_PROTO_UDP: c_int = 1;
pub const STATUS_RX_PROTO_ICMP: c_int = 2;
pub const STATUS_RX_PROTO_OTHER: c_int = 3;
pub const STATUS_RX_PROTO_MASK: c_int = 3;
pub const STATUS_RX_PROTO_SHIFT: c_int = 18;
pub const STATUS_FILTER_INDEX_MASK: c_uint = 0xFFFF;
// Tx status bits

pub const STATUS_TX_CSUM_START_SHIFT: c_int = 16;
pub const STATUS_TX_CSUM_PROTO_UDP: c_uint = 0x8000;
pub const STATUS_TX_CSUM_OFFSET_MASK: c_uint = 0x7FFF;
pub const STATUS_TX_CSUM_LV: c_uint = 0x80000000;
// DMA Descriptor
pub const DMA_DESC_LENGTH_STATUS: c_uint = 0x00	/* in bytes of data in buffer */;
pub const DMA_DESC_ADDRESS_LO: c_uint = 0x04	/* lower bits of PA */;
pub const DMA_DESC_ADDRESS_HI: c_uint = 0x08	/* upper 32 bits of PA, GENETv4+ */;
// Rx/Tx common counter group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_pkt_counters {
    pub /: *mut *mut u32 cnt_64; / RO Received/Transmited 64 bytes packet,
    pub /: *mut *mut u32 cnt_127; / RO Rx/Tx 127 bytes packet,
    pub /: *mut *mut u32 cnt_255; / RO Rx/Tx 65-255 bytes packet,
    pub /: *mut *mut u32 cnt_511; / RO Rx/Tx 256-511 bytes packet,
    pub /: *mut *mut u32 cnt_1023; / RO Rx/Tx 512-1023 bytes packet,
    pub /: *mut *mut u32 cnt_1518; / RO Rx/Tx 1024-1518 bytes packet,
    pub /: *mut *mut u32 cnt_mgv; / RO Rx/Tx 1519-1522 good VLAN packet,
    pub packet*/: *mut *mut u32 cnt_2047; / RO Rx/Tx 1522-2047 bytes,
    pub packet*/: *mut *mut u32 cnt_4095; / RO Rx/Tx 2048-4095 bytes,
    pub packet*/: *mut *mut u32 cnt_9216; / RO Rx/Tx 4096-9216 bytes,
}

// RSV, Receive Status Vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_rx_counters {
    pub pkt_cnt: bcmgenet_pkt_counters,
    pub count*/: *mut *mut u32 pkt; / RO (0x428) Received pkt,
    pub /: *mut *mut u32 bytes; / RO Received byte count,
    pub /: *mut *mut u32 mca; / RO # of Received multicast pkt,
    pub /: *mut *mut u32 bca; / RO # of Receive broadcast pkt,
    pub /: *mut *mut u32 fcs; / RO # of Received FCS error,
    pub pkt*/: *mut *mut u32 cf; / RO # of Received control frame,
    pub /: *mut *mut u32 pf; / RO # of Received pause frame pkt,
    pub /: *mut *mut u32 uo; / RO # of unknown op code pkt,
    pub /: *mut *mut u32 aln; / RO # of alignment error count,
    pub /: *mut *mut u32 flr; / RO # of frame length out of range count,
    pub /: *mut *mut u32 cde; / RO # of code error pkt,
    pub /: *mut *mut u32 fcr; / RO # of carrier sense error pkt,
    pub pkt*/: *mut *mut u32 ovr; / RO # of oversize,
    pub /: *mut *mut u32 jbr; / RO # of jabber count,
    pub pkt*/: *mut *mut u32 mtue; / RO # of MTU error,
    pub /: *mut *mut u32 pok; / RO # of Received good pkt,
    pub /: *mut *mut u32 uc; / RO # of unicast pkt,
    pub /: *mut *mut u32 ppp; / RO # of PPP pkt,
    pub /: *mut *mut u32 rcrc; / RO (0x470),# of CRC match pkt,
}

// TSV, Transmit Status Vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_tx_counters {
    pub pkt_cnt: bcmgenet_pkt_counters,
    pub /: *mut *mut u32 pkts; / RO (0x4a8) Transmited pkt,
    pub /: *mut *mut u32 mca; / RO # of xmited multicast pkt,
    pub /: *mut *mut u32 bca; / RO # of xmited broadcast pkt,
    pub /: *mut *mut u32 pf; / RO # of xmited pause frame count,
    pub /: *mut *mut u32 cf; / RO # of xmited control frame count,
    pub /: *mut *mut u32 fcs; / RO # of xmited FCS error count,
    pub /: *mut *mut u32 ovr; / RO # of xmited oversize pkt,
    pub /: *mut *mut u32 drf; / RO # of xmited deferral pkt,
    pub pkt*/: *mut *mut u32 edf; / RO # of xmited Excessive deferral,
    pub /: *mut *mut u32 scl; / RO # of xmited single collision pkt,
    pub pkt*/: *mut *mut u32 mcl; / RO # of xmited multiple collision,
    pub /: *mut *mut u32 lcl; / RO # of xmited late collision pkt,
    pub pkt*/: *mut *mut u32 ecl; / RO # of xmited excessive collision,
    pub pkt*/: *mut *mut u32 frg; / RO # of xmited fragments,
    pub /: *mut *mut u32 ncl; / RO # of xmited total collision count,
    pub count*/: *mut *mut u32 jbr; / RO # of xmited jabber,
    pub /: *mut *mut u32 bytes; / RO # of xmited byte count,
    pub /: *mut *mut u32 pok; / RO # of xmited good pkt,
    pub /: *mut *mut u32 uc; / RO (0x0x4f0)# of xmited unitcast pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_mib_counters {
    pub rx: bcmgenet_rx_counters,
    pub tx: bcmgenet_tx_counters,
    pub rx_runt_cnt: u32,
    pub rx_runt_fcs: u32,
    pub rx_runt_fcs_align: u32,
    pub rx_runt_bytes: u32,
    pub rbuf_ovflow_cnt: u32,
    pub rbuf_err_cnt: u32,
    pub mdf_err_cnt: u32,
    pub alloc_rx_buff_failed: u32,
    pub tx_dma_failed: u32,
    pub tx_realloc_tsb: u32,
    pub tx_realloc_tsb_failed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_tx_stats64 {
    pub syncp: u64_stats_sync,
    pub packets: u64_stats_t,
    pub bytes: u64_stats_t,
    pub errors: u64_stats_t,
    pub dropped: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_rx_stats64 {
    pub syncp: u64_stats_sync,
    pub bytes: u64_stats_t,
    pub packets: u64_stats_t,
    pub errors: u64_stats_t,
    pub dropped: u64_stats_t,
    pub multicast: u64_stats_t,
    pub broadcast: u64_stats_t,
    pub missed: u64_stats_t,
    pub length_errors: u64_stats_t,
    pub over_errors: u64_stats_t,
    pub crc_errors: u64_stats_t,
    pub frame_errors: u64_stats_t,
    pub fragmented_errors: u64_stats_t,
}

pub const UMAC_MIB_START: c_uint = 0x400;
pub const UMAC_MDIO_CMD: c_uint = 0x614;

pub const MDIO_PMD_SHIFT: c_int = 21;
pub const MDIO_PMD_MASK: c_uint = 0x1F;
pub const MDIO_REG_SHIFT: c_int = 16;
pub const MDIO_REG_MASK: c_uint = 0x1F;
pub const UMAC_RBUF_OVFL_CNT_V1: c_uint = 0x61C;
pub const RBUF_OVFL_CNT_V2: c_uint = 0x80;
pub const RBUF_OVFL_CNT_V3PLUS: c_uint = 0x94;
pub const UMAC_MPD_CTRL: c_uint = 0x620;

pub const MPD_MSEQ_LEN_SHIFT: c_int = 16;
pub const MPD_MSEQ_LEN_MASK: c_uint = 0xFF;
pub const UMAC_MPD_PW_MS: c_uint = 0x624;
pub const UMAC_MPD_PW_LS: c_uint = 0x628;
pub const UMAC_RBUF_ERR_CNT_V1: c_uint = 0x634;
pub const RBUF_ERR_CNT_V2: c_uint = 0x84;
pub const RBUF_ERR_CNT_V3PLUS: c_uint = 0x98;
pub const UMAC_MDF_ERR_CNT: c_uint = 0x638;
pub const UMAC_MDF_CTRL: c_uint = 0x650;
pub const UMAC_MDF_ADDR: c_uint = 0x654;
pub const UMAC_MIB_CTRL: c_uint = 0x580;

pub const RBUF_CTRL: c_uint = 0x00;

pub const RBUF_STATUS: c_uint = 0x0C;

pub const RBUF_CHK_CTRL: c_uint = 0x14;

pub const RBUF_ENERGY_CTRL: c_uint = 0x9c;

pub const RBUF_TBUF_SIZE_CTRL: c_uint = 0xb4;
pub const RBUF_HFB_CTRL_V1: c_uint = 0x38;
pub const RBUF_HFB_FILTER_EN_SHIFT: c_int = 16;
pub const RBUF_HFB_FILTER_EN_MASK: c_uint = 0xffff0000;

pub const RBUF_HFB_LEN_V1: c_uint = 0x3C;
pub const RBUF_FLTR_LEN_MASK: c_uint = 0xFF;
pub const RBUF_FLTR_LEN_SHIFT: c_int = 8;
pub const TBUF_CTRL: c_uint = 0x00;

pub const TBUF_BP_MC: c_uint = 0x0C;
pub const TBUF_ENERGY_CTRL: c_uint = 0x14;

pub const TBUF_CTRL_V1: c_uint = 0x80;
pub const TBUF_BP_MC_V1: c_uint = 0xA0;
pub const HFB_CTRL: c_uint = 0x00;
pub const HFB_FLT_ENABLE_V3PLUS: c_uint = 0x04;
pub const HFB_FLT_LEN_V2: c_uint = 0x04;
pub const HFB_FLT_LEN_V3PLUS: c_uint = 0x1C;
// uniMac intrl2 registers
pub const INTRL2_CPU_STAT: c_uint = 0x00;
pub const INTRL2_CPU_SET: c_uint = 0x04;
pub const INTRL2_CPU_CLEAR: c_uint = 0x08;
pub const INTRL2_CPU_MASK_STATUS: c_uint = 0x0C;
pub const INTRL2_CPU_MASK_SET: c_uint = 0x10;
pub const INTRL2_CPU_MASK_CLEAR: c_uint = 0x14;
// INTRL2 instance 0 definitions

// Only valid for GENETv3+

// INTRL2 instance 1 definitions
pub const UMAC_IRQ1_TX_INTR_MASK: c_uint = 0xFFFF;
pub const UMAC_IRQ1_RX_INTR_MASK: c_uint = 0xFFFF;
pub const UMAC_IRQ1_RX_INTR_SHIFT: c_int = 16;
// Register block offsets
pub const GENET_SYS_OFF: c_uint = 0x0000;
pub const GENET_GR_BRIDGE_OFF: c_uint = 0x0040;
pub const GENET_EXT_OFF: c_uint = 0x0080;
pub const GENET_INTRL2_0_OFF: c_uint = 0x0200;
pub const GENET_INTRL2_1_OFF: c_uint = 0x0240;
pub const GENET_RBUF_OFF: c_uint = 0x0300;
pub const GENET_UMAC_OFF: c_uint = 0x0800;
// SYS block offsets and register definitions
pub const SYS_REV_CTRL: c_uint = 0x00;
pub const SYS_PORT_CTRL: c_uint = 0x04;
pub const PORT_MODE_INT_EPHY: c_int = 0;
pub const PORT_MODE_INT_GPHY: c_int = 1;
pub const PORT_MODE_EXT_EPHY: c_int = 2;
pub const PORT_MODE_EXT_GPHY: c_int = 3;

pub const PORT_MODE_EXT_RVMII_50: c_int = 4;

pub const SYS_RBUF_FLUSH_CTRL: c_uint = 0x08;
pub const SYS_TBUF_FLUSH_CTRL: c_uint = 0x0C;
pub const RBUF_FLUSH_CTRL_V1: c_uint = 0x04;
// Ext block register offsets and definitions
pub const EXT_EXT_PWR_MGMT: c_uint = 0x00;

pub const EXT_RGMII_OOB_CTRL: c_uint = 0x0C;

pub const EXT_GPHY_CTRL: c_uint = 0x1C;

// DMA rings size

// DMA registers common definitions
pub const DMA_RW_POINTER_MASK: c_uint = 0x1FF;
pub const DMA_P_INDEX_DISCARD_CNT_MASK: c_uint = 0xFFFF;
pub const DMA_P_INDEX_DISCARD_CNT_SHIFT: c_int = 16;
pub const DMA_BUFFER_DONE_CNT_MASK: c_uint = 0xFFFF;
pub const DMA_BUFFER_DONE_CNT_SHIFT: c_int = 16;
pub const DMA_P_INDEX_MASK: c_uint = 0xFFFF;
pub const DMA_C_INDEX_MASK: c_uint = 0xFFFF;
// DMA ring size register
pub const DMA_RING_SIZE_MASK: c_uint = 0xFFFF;
pub const DMA_RING_SIZE_SHIFT: c_int = 16;
pub const DMA_RING_BUFFER_SIZE_MASK: c_uint = 0xFFFF;
// DMA interrupt threshold register
pub const DMA_INTR_THRESHOLD_MASK: c_uint = 0x01FF;
// DMA XON/XOFF register
pub const DMA_XON_THREHOLD_MASK: c_uint = 0xFFFF;
pub const DMA_XOFF_THRESHOLD_MASK: c_uint = 0xFFFF;
pub const DMA_XOFF_THRESHOLD_SHIFT: c_int = 16;
// DMA flow period register
pub const DMA_FLOW_PERIOD_MASK: c_uint = 0xFFFF;
pub const DMA_MAX_PKT_SIZE_MASK: c_uint = 0xFFFF;
pub const DMA_MAX_PKT_SIZE_SHIFT: c_int = 16;
// DMA control register

pub const DMA_RING_BUF_EN_SHIFT: c_uint = 0x01;
pub const DMA_RING_BUF_EN_MASK: c_uint = 0xFFFF;

// DMA status register

// DMA SCB burst size register
pub const DMA_SCB_BURST_SIZE_MASK: c_uint = 0x1F;
// DMA activity vector register
pub const DMA_ACTIVITY_VECTOR_MASK: c_uint = 0x1FFFF;
// DMA backpressure mask register
pub const DMA_BACKPRESSURE_MASK: c_uint = 0x1FFFF;

// DMA backpressure status register
pub const DMA_BACKPRESSURE_STATUS_MASK: c_uint = 0x1FFFF;
// DMA override register

// DMA timeout register
pub const DMA_TIMEOUT_MASK: c_uint = 0xFFFF;

// TDMA rate limiting control register
pub const DMA_RATE_LIMIT_EN_MASK: c_uint = 0xFFFF;
// TDMA arbitration control register
pub const DMA_ARBITER_MODE_MASK: c_uint = 0x03;
pub const DMA_RING_BUF_PRIORITY_MASK: c_uint = 0x1F;
pub const DMA_RING_BUF_PRIORITY_SHIFT: c_int = 5;

pub const DMA_RATE_ADJ_MASK: c_uint = 0xFF;
// Tx/Rx Dma Descriptor common bits
pub const DMA_BUFLENGTH_MASK: c_uint = 0x0fff;
pub const DMA_BUFLENGTH_SHIFT: c_int = 16;
pub const DMA_OWN: c_uint = 0x8000;
pub const DMA_EOP: c_uint = 0x4000;
pub const DMA_SOP: c_uint = 0x2000;
pub const DMA_WRAP: c_uint = 0x1000;
// Tx specific Dma descriptor bits
pub const DMA_TX_UNDERRUN: c_uint = 0x0200;
pub const DMA_TX_APPEND_CRC: c_uint = 0x0040;
pub const DMA_TX_OW_CRC: c_uint = 0x0020;
pub const DMA_TX_DO_CSUM: c_uint = 0x0010;
pub const DMA_TX_QTAG_SHIFT: c_int = 7;
// Rx Specific Dma descriptor bits
pub const DMA_RX_CHK_V3PLUS: c_uint = 0x8000;
pub const DMA_RX_CHK_V12: c_uint = 0x1000;
pub const DMA_RX_BRDCAST: c_uint = 0x0040;
pub const DMA_RX_MULT: c_uint = 0x0020;
pub const DMA_RX_LG: c_uint = 0x0010;
pub const DMA_RX_NO: c_uint = 0x0008;
pub const DMA_RX_RXER: c_uint = 0x0004;
pub const DMA_RX_CRC_ERROR: c_uint = 0x0002;
pub const DMA_RX_OV: c_uint = 0x0001;
pub const DMA_RX_FI_MASK: c_uint = 0x001F;
pub const DMA_RX_FI_SHIFT: c_uint = 0x0007;
pub const DMA_DESC_ALLOC_MASK: c_uint = 0x00FF;
pub const DMA_ARBITER_RR: c_uint = 0x00;
pub const DMA_ARBITER_WRR: c_uint = 0x01;
pub const DMA_ARBITER_SP: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enet_cb {
    pub skb: *mut sk_buff,
    pub rx_page: *mut page,
    pub bd_addr: *mut void __iomem,
}

// power management mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcmgenet_power_mode {
    GENET_POWER_CABLE_SENSE = 0,
    GENET_POWER_PASSIVE,
    GENET_POWER_WOL_MAGIC,
}

// We support both runtime GENET detection and compile-time
// to optimize code-paths for a given hardware
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcmgenet_version {
    GENET_V1 = 1,
    GENET_V2,
    GENET_V3,
    GENET_V4,
    GENET_V5
}

// Hardware flags

// BCMGENET hardware parameters, keep this structure nicely aligned
// since it is going to be used in hot paths
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_hw_params {
    pub tx_queues: u8,
    pub tx_bds_per_q: u8,
    pub rx_queues: u8,
    pub rx_bds_per_q: u8,
    pub bp_in_en_shift: u8,
    pub bp_in_mask: u32,
    pub hfb_filter_cnt: u8,
    pub hfb_filter_size: u8,
    pub qtag_mask: u8,
    pub tbuf_offset: u16,
    pub hfb_offset: u32,
    pub hfb_reg_offset: u32,
    pub rdma_offset: u32,
    pub tdma_offset: u32,
    pub words_per_bd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_skb_cb {
    pub /: *mut *mut *mut enet_cb first_cb; / First control block of SKB,
    pub /: *mut *mut *mut enet_cb last_cb; / Last control block of SKB,
    pub /: *mut *mut unsigned int bytes_sent; / bytes on the wire (no TSB),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_tx_ring {
    pub /: *mut *mut spinlock_t lock; / ring lock,
    pub /: *mut *mut napi_napi; / NAPI per tx queue,
    pub stats64: bcmgenet_tx_stats64,
    pub /: *mut *mut unsigned int index; / ring index,
    pub block*/: *mut *mut *mut enet_cb cbs; / tx ring buffer control,
    pub /: *mut *mut unsigned int size; / size of each tx ring,
    pub /: *mut *mut unsigned int clean_ptr; / Tx ring clean pointer,
    pub ring*/: *mut *mut unsigned int c_index; / last consumer index of each,
    pub /: *mut *mut unsigned int free_bds; / # of free bds for each ring,
    pub /: *mut *mut unsigned int write_ptr; / Tx ring write pointer SW copy,
    pub /: *mut *mut unsigned int prod_index; / Tx ring producer index SW copy,
    pub /: *mut *mut unsigned int cb_ptr; / Tx ring initial CB ptr,
    pub /: *mut *mut unsigned int end_ptr; / Tx ring end CB ptr,
    pub priv: *mut bcmgenet_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_net_dim {
    pub use_dim: u16,
    pub event_ctr: u16,
    pub packets: c_ulong,
    pub bytes: c_ulong,
    pub dim: dim,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_rx_ring {
    pub /: *mut *mut napi_napi; / Rx NAPI struct,
    pub stats64: bcmgenet_rx_stats64,
    pub /: *mut *mut unsigned int index; / Rx ring index,
    pub /: *mut *mut *mut enet_cb cbs; / Rx ring buffer control block,
    pub /: *mut *mut unsigned int size; / Rx ring size,
    pub /: *mut *mut unsigned int c_index; / Rx last consumer index,
    pub /: *mut *mut unsigned int read_ptr; / Rx ring read pointer,
    pub /: *mut *mut unsigned int cb_ptr; / Rx ring initial CB ptr,
    pub /: *mut *mut unsigned int end_ptr; / Rx ring end CB ptr,
    pub old_discards: c_uint,
    pub dim: bcmgenet_net_dim,
    pub rx_max_coalesced_frames: u32,
    pub rx_coalesce_usecs: u32,
    pub page_pool: *mut page_pool,
    pub priv: *mut bcmgenet_priv,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcmgenet_rxnfc_state {
    BCMGENET_RXNFC_STATE_UNUSED = 0,
    BCMGENET_RXNFC_STATE_DISABLED,
    BCMGENET_RXNFC_STATE_ENABLED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_rxnfc_rule {
    pub list: list_head,
    pub fs: ethtool_rx_flow_spec,
    pub state: bcmgenet_rxnfc_state,
}

// device context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmgenet_priv {
    pub base: *mut void __iomem,
// reg_lock: lock to serialize access to shared registers
    pub reg_lock: spinlock_t,
    pub version: bcmgenet_version,
    pub dev: *mut net_device,
// transmit variables
    pub tx_bds: *mut void __iomem,
    pub tx_cbs: *mut enet_cb,
    pub num_tx_bds: c_uint,
    pub 1]: bcmgenet_tx_ring tx_rings[GENET_MAX_MQ_CNT +,
// receive variables
    pub rx_bds: *mut void __iomem,
    pub rx_cbs: *mut enet_cb,
    pub num_rx_bds: c_uint,
    pub rxnfc_rules: [bcmgenet_rxnfc_rule; MAX_NUM_OF_FS_RULES],
    pub rxnfc_list: list_head,
    pub 1]: bcmgenet_rx_ring rx_rings[GENET_MAX_MQ_CNT +,
// other misc variables
    pub hw_params: *const bcmgenet_hw_params,
    pub flags: u32,
    pub autoneg_pause:1: unsigned,
    pub tx_pause:1: unsigned,
    pub rx_pause:1: unsigned,
// MDIO bus variables
    pub wq: wait_queue_head_t,
    pub internal_phy: bool,
    pub phy_dn: *mut device_node,
    pub mdio_dn: *mut device_node,
    pub mii_bus: *mut mii_bus,
    pub gphy_rev: u16,
    pub clk_eee: *mut clk,
    pub clk_eee_enabled: bool,
// PHY device variables
    pub phy_interface: phy_interface_t,
    pub phy_addr: c_int,
    pub ext_phy: c_int,
// Interrupt variables
    pub bcmgenet_irq_work: work_struct,
    pub irq0: c_int,
    pub irq1: c_int,
    pub wol_irq: c_int,
    pub wol_irq_disabled: bool,
// shared status
    pub lock: spinlock_t,
    pub irq0_stat: c_uint,
// HW descriptors/checksum variables
    pub crc_fwd_en: bool,
    pub dma_max_burst_length: u32,
    pub msg_enable: u32,
    pub clk: *mut clk,
    pub pdev: *mut platform_device,
    pub mii_pdev: *mut platform_device,
// WOL
    pub clk_wol: *mut clk,
    pub wolopts: u32,
    pub sopass: [u8; SOPASS_MAX],
    pub mib: bcmgenet_mib_counters,
}

// MIPS chips strapped for BE will automagically configure the	\
// peripheral registers for CPU-native byte order.		\
// \
// interrupt l2 registers accessors
// HFB register accessors
// GENET v2+ HFB control and filter len helpers
// RBUF register accessors
// MDIO routines
extern "C" {
    pub fn bcmgenet_mii_init(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn bcmgenet_mii_config(dev: *mut net_device, init: bool) -> c_int;
}
extern "C" {
    pub fn bcmgenet_mii_probe(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn bcmgenet_mii_exit(dev: *mut net_device);
}
extern "C" {
    pub fn bcmgenet_phy_pause_set(dev: *mut net_device, rx: bool, tx: bool);
}
extern "C" {
    pub fn bcmgenet_phy_power_set(dev: *mut net_device, enable: bool);
}
extern "C" {
    pub fn bcmgenet_mii_setup(dev: *mut net_device);
}
// Wake-on-LAN routines
extern "C" {
    pub fn bcmgenet_get_wol(dev: *mut net_device, wol: *mut ethtool_wolinfo);
}
extern "C" {
    pub fn bcmgenet_set_wol(dev: *mut net_device, wol: *mut ethtool_wolinfo) -> c_int;
}
extern "C" {
    pub fn bcmgenet_eee_enable_set(dev: *mut net_device, enable: bool);
}
