//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/xgbe/xgbe.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-3-Clause)
//
// Copyright (c) 2014-2025, Advanced Micro Devices, Inc.
// Copyright (c) 2014, Synopsys, Inc.
// All rights reserved
//

// Descriptor related defines
pub const XGBE_TX_DESC_CNT: c_int = 512;

pub const XGBE_RX_DESC_CNT: c_int = 512;
pub const XGBE_TX_DESC_CNT_MIN: c_int = 64;
pub const XGBE_TX_DESC_CNT_MAX: c_int = 4096;
pub const XGBE_RX_DESC_CNT_MIN: c_int = 64;
pub const XGBE_RX_DESC_CNT_MAX: c_int = 4096;

// Descriptors required for maximum contiguous TSO/GSO packet

// Maximum possible descriptors needed for an SKB:
// - Maximum number of SKB frags
// - Maximum descriptors for contiguous TSO/GSO packet
// - Possible context descriptor
// - Possible TSO header descriptor
//

pub const XGBE_RX_BUF_ALIGN: c_int = 64;
pub const XGBE_SKB_ALLOC_SIZE: c_int = 256;

pub const XGBE_MAX_DMA_CHANNELS: c_int = 16;
pub const XGBE_MAX_QUEUES: c_int = 16;
pub const XGBE_PRIORITY_QUEUES: c_int = 8;
pub const XGBE_DMA_STOP_TIMEOUT: c_int = 1;
// DMA cache settings - Outer sharable, write-back, write-allocate
pub const XGBE_DMA_OS_ARCR: c_uint = 0x002b2b2b;
pub const XGBE_DMA_OS_AWCR: c_uint = 0x2f2f2f2f;
// DMA cache settings - System, no caches used
pub const XGBE_DMA_SYS_ARCR: c_uint = 0x00303030;
pub const XGBE_DMA_SYS_AWCR: c_uint = 0x30303030;
// DMA cache settings - PCI device
pub const XGBE_DMA_PCI_ARCR: c_uint = 0x000f0f0f;
pub const XGBE_DMA_PCI_AWCR: c_uint = 0x0f0f0f0f;
pub const XGBE_DMA_PCI_AWARCR: c_uint = 0x00000f0f;
// DMA channel interrupt modes
pub const XGBE_IRQ_MODE_EDGE: c_int = 0;
pub const XGBE_IRQ_MODE_LEVEL: c_int = 1;

pub const XGMAC_MIN_PACKET: c_int = 60;
pub const XGMAC_STD_PACKET_MTU: c_int = 1500;
pub const XGMAC_MAX_STD_PACKET: c_int = 1518;
pub const XGMAC_JUMBO_PACKET_MTU: c_int = 9000;
pub const XGMAC_MAX_JUMBO_PACKET: c_int = 9018;
pub const XGMAC_GIANT_PACKET_MTU: c_int = 16368;

pub const XGMAC_PFC_DATA_LEN: c_int = 46;
pub const XGMAC_PFC_DELAYS: c_int = 14000;

// Common property names

// Device-tree clock names

// ACPI property names

// PCI BAR mapping
pub const XGBE_XGMAC_BAR: c_int = 0;
pub const XGBE_XPCS_BAR: c_int = 1;
pub const XGBE_MAC_PROP_OFFSET: c_uint = 0x1d000;
pub const XGBE_I2C_CTRL_OFFSET: c_uint = 0x1e000;
// PCI MSI/MSIx support
pub const XGBE_MSI_BASE_COUNT: c_int = 4;

// Initial PTP register values based on Link Speed.
pub const MAC_TICNR_1G_INITVAL: c_uint = 0x10;
pub const MAC_TECNR_1G_INITVAL: c_uint = 0x28;
pub const MAC_TICSNR_10G_INITVAL: c_uint = 0x33;
pub const MAC_TECNR_10G_INITVAL: c_uint = 0x14;
pub const MAC_TECSNR_10G_INITVAL: c_uint = 0xCC;
// PCI clock frequencies

// Timestamp support - values based on 50MHz PTP clock
// 50MHz => 20 nsec
//
pub const XGBE_TSTAMP_SSINC: c_int = 20;
pub const XGBE_TSTAMP_SNSINC: c_int = 0;

pub const XGBE_V2_TSTAMP_SSINC: c_uint = 0xA;
pub const XGBE_V2_TSTAMP_SNSINC: c_int = 0;

// Define maximum supported values
pub const XGBE_MAX_PPS_OUT: c_int = 4;
pub const XGBE_MAX_AUX_SNAP: c_int = 4;
pub const XGMAC_FIFO_MIN_ALLOC: c_int = 2048;
pub const XGMAC_FIFO_UNIT: c_int = 256;

pub const XGMAC_FIFO_FC_OFF: c_int = 2048;
pub const XGMAC_FIFO_FC_MIN: c_int = 4096;
pub const XGBE_TC_MIN_QUANTUM: c_int = 10;
// Helper macro for descriptor handling
// Always use XGBE_GET_DESC_DATA to access the descriptor data
// since the index is free-running and needs to be and-ed
// with the descriptor count value of the ring to index to
// the proper descriptor data.
//

// Default coalescing parameters
pub const XGMAC_INIT_DMA_TX_USECS: c_int = 1000;
pub const XGMAC_INIT_DMA_TX_FRAMES: c_int = 25;
pub const XGMAC_MAX_COAL_TX_TICK: c_int = 100000;
pub const XGMAC_MAX_DMA_RIWT: c_uint = 0xff;
pub const XGMAC_INIT_DMA_RX_USECS: c_int = 30;
pub const XGMAC_INIT_DMA_RX_FRAMES: c_int = 25;
// Flow control queue count
pub const XGMAC_MAX_FLOW_CONTROL_QUEUES: c_int = 8;
// Flow control threshold units
pub const XGMAC_FLOW_CONTROL_UNIT: c_int = 512;

pub const XGMAC_FLOW_CONTROL_MAX: c_int = 33280;
// Maximum MAC address hash table size (256 bits = 8 bytes)
pub const XGBE_MAC_HASH_TABLE_SIZE: c_int = 8;
// Receive Side Scaling
pub const XGBE_RSS_HASH_KEY_SIZE: c_int = 40;
pub const XGBE_RSS_MAX_TABLE_SIZE: c_int = 256;
pub const XGBE_RSS_LOOKUP_TABLE_TYPE: c_int = 0;
pub const XGBE_RSS_HASH_KEY_TYPE: c_int = 1;
// Auto-negotiation
pub const XGBE_AN_MS_TIMEOUT: c_int = 500;
pub const XGBE_LINK_TIMEOUT: c_int = 5;
pub const XGBE_KR_TRAINING_WAIT_ITER: c_int = 50;

pub const XGBE_SGMII_AN_LINK_SPEED_10: c_uint = 0x00;
pub const XGBE_SGMII_AN_LINK_SPEED_100: c_uint = 0x04;
pub const XGBE_SGMII_AN_LINK_SPEED_1000: c_uint = 0x08;

// ECC correctable error notification window (seconds)
pub const XGBE_ECC_LIMIT: c_int = 60;
// MDIO port types
pub const XGMAC_MAX_C22_PORT: c_int = 3;
// Link mode bit operations

// XGBE PCI device id
pub const XGBE_RV_PCI_DEVICE_ID: c_uint = 0x15d0;
pub const XGBE_YC_PCI_DEVICE_ID: c_uint = 0x14b5;
pub const XGBE_RN_PCI_DEVICE_ID: c_uint = 0x1630;
pub const XGBE_P100a_PCI_DEVICE_ID: c_uint = 0x1122;
// Generic low and high masks

// MAC hardware version numbers (SNPSVER field in MAC_VR register)
pub const XGBE_MAC_VER_30: c_uint = 0x30	/* Baseline Rx adaptation support */;
pub const XGBE_MAC_VER_33: c_uint = 0x33	/* P100a platform */;
// MAC Speed Select (SS) values for MAC_TCR register
// These values are written to the SS field to configure link speed.
// Note: P100a uses XGMII mode (0x06) for 2.5G instead of GMII (0x02)
//
// Note: 100M and 2.5G GMII share the same value (0x02) but are
// differentiated by the mode/interface type at the PHY level
//
pub const XGBE_MAC_SS_10G: c_uint = 0x00	/* 10Gbps - XGMII mode */;
pub const XGBE_MAC_SS_2_5G_GMII: c_uint = 0x02	/* 2.5Gbps - GMII mode (YC) */;
pub const XGBE_MAC_SS_2_5G_XGMII: c_uint = 0x06	/* 2.5Gbps - XGMII mode (P100a) */;
pub const XGBE_MAC_SS_1G: c_uint = 0x03	/* 1Gbps */;
pub const XGBE_MAC_SS_100M: c_uint = 0x02	/* 100Mbps */;
pub const XGBE_MAC_SS_10M: c_uint = 0x07	/* 10Mbps */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_packet_data {
    pub skb: *mut sk_buff,
    pub attributes: c_uint,
    pub errors: c_uint,
    pub rdesc_count: c_uint,
    pub length: c_uint,
    pub header_len: c_uint,
    pub tcp_header_len: c_uint,
    pub tcp_payload_len: c_uint,
    pub mss: c_ushort,
    pub vlan_ctag: c_ushort,
    pub rx_tstamp: u64,
    pub rss_hash: u32,
    pub rss_hash_type: pkt_hash_types,
    pub tx_packets: c_uint,
    pub tx_bytes: c_uint,
}

// Common Rx and Tx descriptor mapping
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_ring_desc {
    pub desc0: __le32,
    pub desc1: __le32,
    pub desc2: __le32,
    pub desc3: __le32,
}

// Page allocation related values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_page_alloc {
    pub pages: *mut page,
    pub pages_len: c_uint,
    pub pages_offset: c_uint,
    pub pages_dma: dma_addr_t,
}

// Ring entry buffer data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_buffer_data {
    pub pa: xgbe_page_alloc,
    pub pa_unmap: xgbe_page_alloc,
    pub dma_base: dma_addr_t,
    pub dma_off: c_ulong,
    pub dma_len: c_uint,
}

// Tx-related ring data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_tx_ring_data {
    pub /: *mut *mut unsigned int packets; / BQL packet count,
    pub /: *mut *mut unsigned int bytes; / BQL byte count,
}

// Rx-related ring data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_rx_ring_data {
    pub /: *mut *mut xgbe_buffer_data hdr; / Header locations,
    pub /: *mut *mut xgbe_buffer_data buf; / Payload locations,
    pub /: *mut *mut unsigned short hdr_len; / Length of received header,
    pub /: *mut *mut unsigned short len; / Length of received packet,
}

// Structure used to hold information related to the descriptor
// and the packet associated with the descriptor (always use
// the XGBE_GET_DESC_DATA macro to access this data from the ring)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_ring_data {
    pub /: *mut *mut *mut xgbe_ring_desc rdesc; / Virtual address of descriptor,
    pub /: *mut *mut dma_addr_t rdesc_dma; / DMA address of descriptor,
    pub /: *mut *mut *mut sk_buff skb; / Virtual address of SKB,
    pub /: *mut *mut dma_addr_t skb_dma; / DMA address of SKB data,
    pub /: *mut *mut unsigned int skb_dma_len; / Length of SKB DMA area,
    pub /: *mut *mut xgbe_tx_ring_data tx; / Tx-related data,
    pub /: *mut *mut xgbe_rx_ring_data rx; / Rx-related data,
    pub mapped_as_page: c_uint,
// Incomplete receive save location.  If the budget is exhausted
// or the last descriptor (last normal descriptor or a following
// context descriptor) has not been DMA'd yet the current state
// of the receive processing needs to be saved.
//
    pub state_saved: c_uint,
    pub skb: *mut sk_buff,
    pub len: c_uint,
    pub error: c_uint,
    pub state: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_ring {
// Ring lock - used just for TX rings at the moment
    pub lock: spinlock_t,
// Per packet related information
    pub packet_data: xgbe_packet_data,
// Virtual/DMA addresses and count of allocated descriptor memory
    pub rdesc: *mut xgbe_ring_desc,
    pub rdesc_dma: dma_addr_t,
    pub rdesc_count: c_uint,
// Array of descriptor data corresponding the descriptor memory
// (always use the XGBE_GET_DESC_DATA macro to access this data)
//
    pub rdata: *mut xgbe_ring_data,
// Page allocation for RX buffers
    pub rx_hdr_pa: xgbe_page_alloc,
    pub rx_buf_pa: xgbe_page_alloc,
    pub node: c_int,
// Ring index values
// cur   - Tx: index of descriptor to be used for current transfer
// Rx: index of descriptor to check for packet availability
// dirty - Tx: index of descriptor to check for transfer complete
// Rx: index of descriptor to check for buffer reallocation
//
    pub cur: c_uint,
    pub dirty: c_uint,
// Coalesce frame count used for interrupt bit setting
    pub coalesce_count: c_uint,
    pub queue_stopped: c_uint,
    pub xmit_more: c_uint,
    pub cur_mss: c_ushort,
    pub cur_vlan_ctag: c_ushort,
    pub tx: },
}

// Structure used to describe the descriptor rings associated with
// a DMA channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_channel {
    pub name: [c_char; 20],
// Address of private data area for device
    pub pdata: *mut xgbe_prv_data,
// Queue index and base address of queue's DMA registers
    pub queue_index: c_uint,
    pub dma_regs: *mut void __iomem,
// Per channel interrupt irq number
    pub dma_irq: c_int,
    pub 32]: char dma_irq_name[IFNAMSIZ +,
// Netdev related settings
    pub napi: napi_struct,
// Per channel interrupt enablement tracker
    pub curr_ier: c_uint,
    pub saved_ier: c_uint,
    pub tx_timer_active: c_uint,
    pub tx_timer: timer_list,
    pub tx_ring: *mut xgbe_ring,
    pub rx_ring: *mut xgbe_ring,
    pub node: c_int,
    pub affinity_mask: cpumask_t,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_state {
    XGBE_DOWN,
    XGBE_LINK_INIT,
    XGBE_LINK_ERR,
    XGBE_STOPPED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_int {
    XGMAC_INT_DMA_CH_SR_TI,
    XGMAC_INT_DMA_CH_SR_TPS,
    XGMAC_INT_DMA_CH_SR_TBU,
    XGMAC_INT_DMA_CH_SR_RI,
    XGMAC_INT_DMA_CH_SR_RBU,
    XGMAC_INT_DMA_CH_SR_RPS,
    XGMAC_INT_DMA_CH_SR_TI_RI,
    XGMAC_INT_DMA_CH_SR_FBE,
    XGMAC_INT_DMA_ALL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_int_state {
    XGMAC_INT_STATE_SAVE,
    XGMAC_INT_STATE_RESTORE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_ecc_sec {
    XGBE_ECC_SEC_TX,
    XGBE_ECC_SEC_RX,
    XGBE_ECC_SEC_DESC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_speed {
    XGBE_SPEED_1000 = 0,
    XGBE_SPEED_2500,
    XGBE_SPEED_10000,
    XGBE_SPEEDS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_xpcs_access {
    XGBE_XPCS_ACCESS_V1 = 0,
    XGBE_XPCS_ACCESS_V2,
    XGBE_XPCS_ACCESS_V3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_an_mode {
    XGBE_AN_MODE_CL73 = 0,
    XGBE_AN_MODE_CL73_REDRV,
    XGBE_AN_MODE_CL37,
    XGBE_AN_MODE_CL37_SGMII,
    XGBE_AN_MODE_NONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_an {
    XGBE_AN_READY = 0,
    XGBE_AN_PAGE_RECEIVED,
    XGBE_AN_INCOMPAT_LINK,
    XGBE_AN_COMPLETE,
    XGBE_AN_NO_LINK,
    XGBE_AN_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_rx {
    XGBE_RX_BPA = 0,
    XGBE_RX_XNP,
    XGBE_RX_COMPLETE,
    XGBE_RX_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_mode {
    XGBE_MODE_KX_1000 = 0,
    XGBE_MODE_KX_2500,
    XGBE_MODE_KR,
    XGBE_MODE_X,
    XGBE_MODE_SGMII_10,
    XGBE_MODE_SGMII_100,
    XGBE_MODE_SGMII_1000,
    XGBE_MODE_SFI,
    XGBE_MODE_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_speedset {
    XGBE_SPEEDSET_1000_10000 = 0,
    XGBE_SPEEDSET_2500_10000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_mdio_mode {
    XGBE_MDIO_MODE_NONE = 0,
    XGBE_MDIO_MODE_CL22,
    XGBE_MDIO_MODE_CL45,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_mb_cmd {
    XGBE_MB_CMD_POWER_OFF = 0,
    XGBE_MB_CMD_SET_1G,
    XGBE_MB_CMD_SET_2_5G,
    XGBE_MB_CMD_SET_10G_SFI,
    XGBE_MB_CMD_SET_10G_KR,
    XGBE_MB_CMD_RRC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_mb_subcmd {
    XGBE_MB_SUBCMD_NONE = 0,
    XGBE_MB_SUBCMD_RX_ADAP,

// 10GbE SFP subcommands
    XGBE_MB_SUBCMD_ACTIVE = 0,
    XGBE_MB_SUBCMD_PASSIVE_1M,
    XGBE_MB_SUBCMD_PASSIVE_3M,
    XGBE_MB_SUBCMD_PASSIVE_OTHER,

// 1GbE Mode subcommands
    XGBE_MB_SUBCMD_10MBITS = 0,
    XGBE_MB_SUBCMD_100MBITS,
    XGBE_MB_SUBCMD_1G_SGMII,
    XGBE_MB_SUBCMD_1G_KX,

// 2.5GbE Mode subcommands
    XGBE_MB_SUBCMD_2_5G_KX = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_phy {
    pub lks: ethtool_link_ksettings,
    pub address: c_int,
    pub autoneg: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub link: c_int,
    pub pause_autoneg: c_int,
    pub tx_pause: c_int,
    pub rx_pause: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgbe_i2c_cmd {
    XGBE_I2C_CMD_READ = 0,
    XGBE_I2C_CMD_WRITE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_i2c_op {
    pub cmd: xgbe_i2c_cmd,
    pub target: c_uint,
    pub buf: *mut c_void,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_i2c_op_state {
    pub op: *mut xgbe_i2c_op,
    pub tx_len: c_uint,
    pub tx_buf: *mut c_uchar,
    pub rx_len: c_uint,
    pub rx_buf: *mut c_uchar,
    pub tx_abort_source: c_uint,
    pub ret: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_i2c {
    pub started: c_uint,
    pub max_speed_mode: c_uint,
    pub rx_fifo_size: c_uint,
    pub tx_fifo_size: c_uint,
    pub op_state: xgbe_i2c_op_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_mmc_stats {
// Tx Stats
    pub txoctetcount_gb: u64,
    pub txframecount_gb: u64,
    pub txbroadcastframes_g: u64,
    pub txmulticastframes_g: u64,
    pub tx64octets_gb: u64,
    pub tx65to127octets_gb: u64,
    pub tx128to255octets_gb: u64,
    pub tx256to511octets_gb: u64,
    pub tx512to1023octets_gb: u64,
    pub tx1024tomaxoctets_gb: u64,
    pub txunicastframes_gb: u64,
    pub txmulticastframes_gb: u64,
    pub txbroadcastframes_gb: u64,
    pub txunderflowerror: u64,
    pub txoctetcount_g: u64,
    pub txframecount_g: u64,
    pub txpauseframes: u64,
    pub txvlanframes_g: u64,
// Rx Stats
    pub rxframecount_gb: u64,
    pub rxoctetcount_gb: u64,
    pub rxoctetcount_g: u64,
    pub rxbroadcastframes_g: u64,
    pub rxmulticastframes_g: u64,
    pub rxcrcerror: u64,
    pub rxrunterror: u64,
    pub rxjabbererror: u64,
    pub rxundersize_g: u64,
    pub rxoversize_g: u64,
    pub rx64octets_gb: u64,
    pub rx65to127octets_gb: u64,
    pub rx128to255octets_gb: u64,
    pub rx256to511octets_gb: u64,
    pub rx512to1023octets_gb: u64,
    pub rx1024tomaxoctets_gb: u64,
    pub rxunicastframes_g: u64,
    pub rxlengtherror: u64,
    pub rxoutofrangetype: u64,
    pub rxpauseframes: u64,
    pub rxfifooverflow: u64,
    pub rxvlanframes_gb: u64,
    pub rxwatchdogerror: u64,
    pub rxalignmenterror: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_ext_stats {
    pub tx_tso_packets: u64,
    pub rx_split_header_packets: u64,
    pub rx_buffer_unavailable: u64,
    pub txq_packets: [u64; XGBE_MAX_DMA_CHANNELS],
    pub txq_bytes: [u64; XGBE_MAX_DMA_CHANNELS],
    pub rxq_packets: [u64; XGBE_MAX_DMA_CHANNELS],
    pub rxq_bytes: [u64; XGBE_MAX_DMA_CHANNELS],
    pub tx_vxlan_packets: u64,
    pub rx_vxlan_packets: u64,
    pub rx_csum_errors: u64,
    pub rx_vxlan_csum_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_pps_config {
    pub start: timespec64,
    pub period: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_hw_if {
    pub ): *mut *mut int (tx_complete)(struct xgbe_ring_desc,
    pub addr): *const *const *const int (set_mac_address)(struct xgbe_prv_data , u8,
    pub ): *mut *mut int (config_rx_mode)(struct xgbe_prv_data,
    pub ): *mut *mut int (enable_rx_csum)(struct xgbe_prv_data,
    pub ): *mut *mut int (disable_rx_csum)(struct xgbe_prv_data,
    pub ): *mut *mut int (enable_rx_vlan_stripping)(struct xgbe_prv_data,
    pub ): *mut *mut int (disable_rx_vlan_stripping)(struct xgbe_prv_data,
    pub ): *mut *mut int (enable_rx_vlan_filtering)(struct xgbe_prv_data,
    pub ): *mut *mut int (disable_rx_vlan_filtering)(struct xgbe_prv_data,
    pub ): *mut *mut int (update_vlan_hash_table)(struct xgbe_prv_data,
    pub int): *mut *mut *mut int (read_mmd_regs)(struct xgbe_prv_data , int,,
    pub int): *mut *mut *mut void (write_mmd_regs)(struct xgbe_prv_data , int, int,,
    pub int): *mut *mut *mut int (set_speed)(struct xgbe_prv_data ,,
    pub xgbe_mdio_mode): enum,
    pub int): *mut *mut *mut int (read_ext_mii_regs_c22)(struct xgbe_prv_data , int,,
    pub u16): *mut *mut *mut int (write_ext_mii_regs_c22)(struct xgbe_prv_data , int, int,,
    pub int): *mut *mut *mut int (read_ext_mii_regs_c45)(struct xgbe_prv_data , int, int,,
    pub int): *mut *mut *mut int (set_gpio)(struct xgbe_prv_data , unsigned,
    pub int): *mut *mut *mut int (clr_gpio)(struct xgbe_prv_data , unsigned,
    pub ): *mut *mut void (enable_tx)(struct xgbe_prv_data,
    pub ): *mut *mut void (disable_tx)(struct xgbe_prv_data,
    pub ): *mut *mut void (enable_rx)(struct xgbe_prv_data,
    pub ): *mut *mut void (disable_rx)(struct xgbe_prv_data,
    pub ): *mut *mut void (powerup_tx)(struct xgbe_prv_data,
    pub ): *mut *mut void (powerdown_tx)(struct xgbe_prv_data,
    pub ): *mut *mut void (powerup_rx)(struct xgbe_prv_data,
    pub ): *mut *mut void (powerdown_rx)(struct xgbe_prv_data,
    pub ): *mut *mut int (init)(struct xgbe_prv_data,
    pub ): *mut *mut int (exit)(struct xgbe_prv_data,
    pub xgbe_int): *mut *mut *mut int (enable_int)(struct xgbe_channel , enum,
    pub xgbe_int): *mut *mut *mut int (disable_int)(struct xgbe_channel , enum,
    pub ): *mut *mut void (dev_xmit)(struct xgbe_channel,
    pub ): *mut *mut int (dev_read)(struct xgbe_channel,
    pub ): *mut *mut void (tx_desc_init)(struct xgbe_channel,
    pub ): *mut *mut void (rx_desc_init)(struct xgbe_channel,
    pub ): *mut *mut void (tx_desc_reset)(struct xgbe_ring_data,
    pub int): unsigned,
    pub ): *mut *mut int (is_last_desc)(struct xgbe_ring_desc,
    pub ): *mut *mut int (is_context_desc)(struct xgbe_ring_desc,
    pub ): *mut *mut *mut void (tx_start_xmit)(struct xgbe_channel , struct xgbe_ring,
// For FLOW ctrl
    pub ): *mut *mut int (config_tx_flow_control)(struct xgbe_prv_data,
    pub ): *mut *mut int (config_rx_flow_control)(struct xgbe_prv_data,
// For RX coalescing
    pub ): *mut *mut int (config_rx_coalesce)(struct xgbe_prv_data,
    pub ): *mut *mut int (config_tx_coalesce)(struct xgbe_prv_data,
    pub int): *mut *mut *mut unsigned int (usec_to_riwt)(struct xgbe_prv_data , unsigned,
    pub int): *mut *mut *mut unsigned int (riwt_to_usec)(struct xgbe_prv_data , unsigned,
// For RX and TX threshold config
    pub int): *mut *mut *mut int (config_rx_threshold)(struct xgbe_prv_data , unsigned,
    pub int): *mut *mut *mut int (config_tx_threshold)(struct xgbe_prv_data , unsigned,
// For RX and TX Store and Forward Mode config
    pub int): *mut *mut *mut int (config_rsf_mode)(struct xgbe_prv_data , unsigned,
    pub int): *mut *mut *mut int (config_tsf_mode)(struct xgbe_prv_data , unsigned,
// For TX DMA Operate on Second Frame config
    pub ): *mut *mut int (config_osp_mode)(struct xgbe_prv_data,
// For MMC statistics
    pub ): *mut *mut void (rx_mmc_int)(struct xgbe_prv_data,
    pub ): *mut *mut void (tx_mmc_int)(struct xgbe_prv_data,
    pub ): *mut *mut void (read_mmc_stats)(struct xgbe_prv_data,
// For Data Center Bridging config
    pub ): *mut *mut void (config_tc)(struct xgbe_prv_data,
    pub ): *mut *mut void (config_dcb_tc)(struct xgbe_prv_data,
    pub ): *mut *mut void (config_dcb_pfc)(struct xgbe_prv_data,
// For Receive Side Scaling
    pub ): *mut *mut int (enable_rss)(struct xgbe_prv_data,
    pub ): *mut *mut int (disable_rss)(struct xgbe_prv_data,
    pub ): *const *const *const int (set_rss_hash_key)(struct xgbe_prv_data , u8,
    pub ): *const *const *const int (set_rss_lookup_table)(struct xgbe_prv_data , u32,
// For ECC
    pub ): *mut *mut void (disable_ecc_ded)(struct xgbe_prv_data,
    pub xgbe_ecc_sec): *mut *mut *mut void (disable_ecc_sec)(struct xgbe_prv_data , enum,
// For VXLAN
    pub ): *mut *mut void (enable_vxlan)(struct xgbe_prv_data,
    pub ): *mut *mut void (disable_vxlan)(struct xgbe_prv_data,
    pub ): *mut *mut void (set_vxlan_id)(struct xgbe_prv_data,
// For Split Header
    pub pdata): *mut *mut void (enable_sph)(struct xgbe_prv_data,
    pub pdata): *mut *mut void (disable_sph)(struct xgbe_prv_data,
}

// This structure represents implementation specific routines for an
// implementation of a PHY. All routines are required unless noted below.
// Optional routines:
// an_pre, an_post
// kr_training_pre, kr_training_post
// module_info, module_eeprom
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_phy_impl_if {
// Perform Setup/teardown actions
    pub ): *mut *mut int (init)(struct xgbe_prv_data,
    pub ): *mut *mut void (exit)(struct xgbe_prv_data,
// Perform start/stop specific actions
    pub ): *mut *mut int (reset)(struct xgbe_prv_data,
    pub ): *mut *mut int (start)(struct xgbe_prv_data,
    pub ): *mut *mut void (stop)(struct xgbe_prv_data,
// Return the link status
    pub ): *mut *mut *mut int (link_status)(struct xgbe_prv_data , int,
// Indicate if a particular speed is valid
    pub int): *mut *mut *mut bool (valid_speed)(struct xgbe_prv_data ,,
// Check if the specified mode can/should be used
    pub xgbe_mode): *mut *mut *mut bool (use_mode)(struct xgbe_prv_data , enum,
// Switch the PHY into various modes
    pub xgbe_mode): *mut *mut *mut void (set_mode)(struct xgbe_prv_data , enum,
// Retrieve mode needed for a specific speed
    pub int): *mut *mut *mut xgbe_mode (get_mode)(struct xgbe_prv_data ,,
// Retrieve new/next mode when trying to auto-negotiate
    pub ): *mut *mut xgbe_mode (switch_mode)(struct xgbe_prv_data,
// Retrieve current mode
    pub ): *mut *mut xgbe_mode (cur_mode)(struct xgbe_prv_data,
// Retrieve current auto-negotiation mode
    pub ): *mut *mut xgbe_an_mode (an_mode)(struct xgbe_prv_data,
// Configure auto-negotiation settings
    pub ): *mut *mut int (an_config)(struct xgbe_prv_data,
// Set/override auto-negotiation advertisement settings
    pub ): *mut ethtool_link_ksettings,
// Process results of auto-negotiation
    pub ): *mut *mut xgbe_mode (an_outcome)(struct xgbe_prv_data,
// Pre/Post auto-negotiation support
    pub ): *mut *mut void (an_pre)(struct xgbe_prv_data,
    pub ): *mut *mut void (an_post)(struct xgbe_prv_data,
// Pre/Post KR training enablement support
    pub ): *mut *mut void (kr_training_pre)(struct xgbe_prv_data,
    pub ): *mut *mut void (kr_training_post)(struct xgbe_prv_data,
// SFP module related info
    pub modinfo): *mut ethtool_modinfo,
    pub data): *mut *mut ethtool_eeprom eeprom, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_phy_if {
// For PHY setup/teardown
    pub ): *mut *mut int (phy_init)(struct xgbe_prv_data,
    pub ): *mut *mut void (phy_exit)(struct xgbe_prv_data,
// For PHY support when setting device up/down
    pub ): *mut *mut int (phy_reset)(struct xgbe_prv_data,
    pub ): *mut *mut int (phy_start)(struct xgbe_prv_data,
    pub ): *mut *mut void (phy_stop)(struct xgbe_prv_data,
// For PHY support while device is up
    pub ): *mut *mut void (phy_status)(struct xgbe_prv_data,
    pub ): *mut *mut int (phy_config_aneg)(struct xgbe_prv_data,
// For PHY settings validation
    pub int): *mut *mut *mut bool (phy_valid_speed)(struct xgbe_prv_data ,,
// For single interrupt support
    pub ): *mut *mut irqreturn_t (an_isr)(struct xgbe_prv_data,
// For ethtool PHY support
    pub modinfo): *mut ethtool_modinfo,
    pub data): *mut *mut ethtool_eeprom eeprom, u8,
// PHY implementation specific services
    pub phy_impl: xgbe_phy_impl_if,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_i2c_if {
// For initial I2C setup
    pub ): *mut *mut int (i2c_init)(struct xgbe_prv_data,
// For I2C support when setting device up/down
    pub ): *mut *mut int (i2c_start)(struct xgbe_prv_data,
    pub ): *mut *mut void (i2c_stop)(struct xgbe_prv_data,
// For performing I2C operations
    pub ): *mut *mut *mut int (i2c_xfer)(struct xgbe_prv_data , struct xgbe_i2c_op,
// For single interrupt support
    pub ): *mut *mut irqreturn_t (i2c_isr)(struct xgbe_prv_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_desc_if {
    pub ): *mut *mut int (alloc_ring_resources)(struct xgbe_prv_data,
    pub ): *mut *mut void (free_ring_resources)(struct xgbe_prv_data,
    pub ): *mut *mut *mut int (map_tx_skb)(struct xgbe_channel , struct sk_buff,
    pub ): *mut xgbe_ring_data,
    pub ): *mut *mut *mut void (unmap_rdata)(struct xgbe_prv_data , struct xgbe_ring_data,
    pub ): *mut *mut void (wrapper_tx_desc_init)(struct xgbe_prv_data,
    pub ): *mut *mut void (wrapper_rx_desc_init)(struct xgbe_prv_data,
}

// This structure contains flags that indicate what hardware features
// or configurations are present in the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_hw_features {
// HW Version
    pub version: c_uint,
// HW Feature Register0
    pub /: *mut *mut unsigned int gmii; / 1000 Mbps support,
    pub /: *mut *mut unsigned int vlhash; / VLAN Hash Filter,
    pub /: *mut *mut unsigned int sma; / SMA(MDIO) Interface,
    pub /: *mut *mut unsigned int rwk; / PMT remote wake-up packet,
    pub /: *mut *mut unsigned int mgk; / PMT magic packet,
    pub /: *mut *mut unsigned int mmc; / RMON module,
    pub /: *mut *mut unsigned int aoe; / ARP Offload,
    pub /: *mut *mut unsigned int ts; / IEEE 1588-2008 Advanced Timestamp,
    pub /: *mut *mut unsigned int eee; / Energy Efficient Ethernet,
    pub /: *mut *mut unsigned int tx_coe; / Tx Checksum Offload,
    pub /: *mut *mut unsigned int rx_coe; / Rx Checksum Offload,
    pub /: *mut *mut unsigned int addn_mac; / Additional MAC Addresses,
    pub /: *mut *mut unsigned int ts_src; / Timestamp Source,
    pub /: *mut *mut unsigned int sa_vlan_ins; / Source Address or VLAN Insertion,
    pub /: *mut *mut unsigned int vxn; / VXLAN/NVGRE,
// HW Feature Register1
    pub /: *mut *mut unsigned int rx_fifo_size; / MTL Receive FIFO Size,
    pub /: *mut *mut unsigned int tx_fifo_size; / MTL Transmit FIFO Size,
    pub /: *mut *mut unsigned int adv_ts_hi; / Advance Timestamping High Word,
    pub /: *mut *mut unsigned int dma_width; / DMA width,
    pub /: *mut *mut unsigned int dcb; / DCB Feature,
    pub /: *mut *mut unsigned int sph; / Split Header Feature,
    pub /: *mut *mut unsigned int tso; / TCP Segmentation Offload,
    pub /: *mut *mut unsigned int dma_debug; / DMA Debug Registers,
    pub /: *mut *mut unsigned int rss; / Receive Side Scaling,
    pub /: *mut *mut unsigned int tc_cnt; / Number of Traffic Classes,
    pub /: *mut *mut unsigned int hash_table_size; / Hash Table Size,
    pub /: *mut *mut unsigned int l3l4_filter_num; / Number of L3-L4 Filters,
// HW Feature Register2
    pub /: *mut *mut unsigned int rx_q_cnt; / Number of MTL Receive Queues,
    pub /: *mut *mut unsigned int tx_q_cnt; / Number of MTL Transmit Queues,
    pub /: *mut *mut unsigned int rx_ch_cnt; / Number of DMA Receive Channels,
    pub /: *mut *mut unsigned int tx_ch_cnt; / Number of DMA Transmit Channels,
    pub /: *mut *mut unsigned int pps_out_num; / Number of PPS outputs,
    pub /: *mut *mut unsigned int aux_snap_num; / Number of Aux snapshot inputs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_version_data {
    pub ): *mut *mut void (init_function_ptrs_phy_impl)(struct xgbe_phy_if,
    pub xpcs_access: xgbe_xpcs_access,
    pub mmc_64bit: c_uint,
    pub tx_max_fifo_size: c_uint,
    pub rx_max_fifo_size: c_uint,
    pub tx_tstamp_workaround: c_uint,
    pub tstamp_ptp_clock_freq: c_uint,
    pub ecc_support: c_uint,
    pub i2c_support: c_uint,
    pub irq_reissue_support: c_uint,
    pub tx_desc_prefetch: c_uint,
    pub rx_desc_prefetch: c_uint,
    pub an_cdr_workaround: c_uint,
    pub enable_rrc: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgbe_prv_data {
    pub netdev: *mut net_device,
    pub pcidev: *mut pci_dev,
    pub platdev: *mut platform_device,
    pub adev: *mut acpi_device,
    pub dev: *mut device,
    pub phy_platdev: *mut platform_device,
    pub phy_dev: *mut device,
    pub smn_base: c_uint,
// Version related data
    pub vdata: *mut xgbe_version_data,
// ACPI or DT flag
    pub use_acpi: c_uint,
// XGMAC/XPCS related mmio registers
    pub /: *mut *mut *mut void __iomem xgmac_regs; / XGMAC CSRs,
    pub /: *mut *mut *mut void __iomem xpcs_regs; / XPCS MMD registers,
    pub /: *mut *mut *mut void __iomem rxtx_regs; / SerDes Rx/Tx CSRs,
    pub /: *mut *mut *mut void __iomem sir0_regs; / SerDes integration registers (1/2),
    pub /: *mut *mut *mut void __iomem sir1_regs; / SerDes integration registers (2/2),
    pub /: *mut *mut *mut void __iomem xprop_regs; / XGBE property registers,
    pub /: *mut *mut *mut void __iomem xi2c_regs; / XGBE I2C CSRs,
// Port property registers
    pub pp0: c_uint,
    pub pp1: c_uint,
    pub pp2: c_uint,
    pub pp3: c_uint,
    pub pp4: c_uint,
// XPCS indirect addressing lock
    pub xpcs_lock: spinlock_t,
    pub xpcs_window_def_reg: c_uint,
    pub xpcs_window_sel_reg: c_uint,
    pub xpcs_window: c_uint,
    pub xpcs_window_size: c_uint,
    pub xpcs_window_mask: c_uint,
// RSS addressing mutex
    pub rss_mutex: mutex,
// Flags representing xgbe_state
    pub dev_state: c_ulong,
// ECC support
    pub tx_sec_period: c_ulong,
    pub tx_ded_period: c_ulong,
    pub rx_sec_period: c_ulong,
    pub rx_ded_period: c_ulong,
    pub desc_sec_period: c_ulong,
    pub desc_ded_period: c_ulong,
    pub tx_sec_count: c_uint,
    pub tx_ded_count: c_uint,
    pub rx_sec_count: c_uint,
    pub rx_ded_count: c_uint,
    pub desc_ded_count: c_uint,
    pub desc_sec_count: c_uint,
    pub dev_irq: c_int,
    pub ecc_irq: c_int,
    pub i2c_irq: c_int,
    pub channel_irq: [c_int; XGBE_MAX_DMA_CHANNELS],
    pub per_channel_irq: c_uint,
    pub irq_count: c_uint,
    pub channel_irq_count: c_uint,
    pub channel_irq_mode: c_uint,
    pub 32]: char ecc_name[IFNAMSIZ +,
    pub hw_if: xgbe_hw_if,
    pub phy_if: xgbe_phy_if,
    pub desc_if: xgbe_desc_if,
    pub i2c_if: xgbe_i2c_if,
// AXI DMA settings
    pub coherent: c_uint,
    pub arcr: c_uint,
    pub awcr: c_uint,
    pub awarcr: c_uint,
// Service routine support
    pub dev_workqueue: *mut workqueue_struct,
    pub service_work: work_struct,
    pub service_timer: timer_list,
// Rings for Tx/Rx on a DMA channel
    pub channel: [*mut xgbe_channel; XGBE_MAX_DMA_CHANNELS],
    pub tx_max_channel_count: c_uint,
    pub rx_max_channel_count: c_uint,
    pub channel_count: c_uint,
    pub tx_ring_count: c_uint,
    pub tx_desc_count: c_uint,
    pub rx_ring_count: c_uint,
    pub rx_desc_count: c_uint,
    pub new_tx_ring_count: c_uint,
    pub new_rx_ring_count: c_uint,
    pub tx_max_q_count: c_uint,
    pub rx_max_q_count: c_uint,
    pub tx_q_count: c_uint,
    pub rx_q_count: c_uint,
// Tx/Rx common settings
    pub blen: c_uint,
    pub pbl: c_uint,
    pub aal: c_uint,
    pub rd_osr_limit: c_uint,
    pub wr_osr_limit: c_uint,
// Tx settings
    pub tx_sf_mode: c_uint,
    pub tx_threshold: c_uint,
    pub tx_osp_mode: c_uint,
    pub tx_max_fifo_size: c_uint,
// Rx settings
    pub rx_sf_mode: c_uint,
    pub rx_threshold: c_uint,
    pub rx_max_fifo_size: c_uint,
// Tx coalescing settings
    pub tx_usecs: c_uint,
    pub tx_frames: c_uint,
// Rx coalescing settings
    pub rx_riwt: c_uint,
    pub rx_usecs: c_uint,
    pub rx_frames: c_uint,
// Current Rx buffer size
    pub rx_buf_size: c_uint,
// Flow control settings
    pub pause_autoneg: c_uint,
    pub tx_pause: c_uint,
    pub rx_pause: c_uint,
    pub rx_rfa: [c_uint; XGBE_MAX_QUEUES],
    pub rx_rfd: [c_uint; XGBE_MAX_QUEUES],
// Receive Side Scaling settings
    pub rss_key: [u8; XGBE_RSS_HASH_KEY_SIZE],
    pub rss_table: [u32; XGBE_RSS_MAX_TABLE_SIZE],
    pub rss_options: u32,
// VXLAN settings
    pub vxlan_port: u16,
// Netdev related settings
    pub mac_addr: [c_uchar; ETH_ALEN],
    pub netdev_features: netdev_features_t,
    pub napi: napi_struct,
    pub mmc_stats: xgbe_mmc_stats,
    pub ext_stats: xgbe_ext_stats,
// Filtering support
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
// Device clocks
    pub sysclk: *mut clk,
    pub sysclk_rate: c_ulong,
    pub ptpclk: *mut clk,
    pub ptpclk_rate: c_ulong,
// Timestamp support
    pub tstamp_lock: spinlock_t,
    pub ptp_clock_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub tstamp_config: kernel_hwtstamp_config,
    pub tstamp_addend: c_uint,
    pub tx_tstamp_work: work_struct,
    pub tx_tstamp_skb: *mut sk_buff,
    pub tx_tstamp: u64,
// Pulse Per Second output
    pub pps: [xgbe_pps_config; XGBE_MAX_PPS_OUT],
// DCB support
    pub ets: *mut ieee_ets,
    pub pfc: *mut ieee_pfc,
    pub q2tc_map: [c_uint; XGBE_MAX_QUEUES],
    pub prio2q_map: [c_uint; IEEE_8021QAZ_MAX_TCS],
    pub pfcq: [c_uint; XGBE_MAX_QUEUES],
    pub pfc_rfa: c_uint,
    pub num_tcs: u8,
// Hardware features of the device
    pub hw_feat: xgbe_hw_features,
// Device work structures
    pub restart_work: work_struct,
    pub stopdev_work: work_struct,
// Keeps track of power mode
    pub power_down: c_uint,
// Network interface message level setting
    pub msg_enable: u32,
// Current PHY settings
    pub phy_mode: phy_interface_t,
    pub phy_link: c_int,
    pub phy_speed: c_int,
// MDIO/PHY related settings
    pub phy_started: c_uint,
    pub phy_data: *mut c_void,
    pub phy: xgbe_phy,
    pub mdio_mmd: c_int,
    pub link_check: c_ulong,
    pub mdio_complete: completion,
    pub kr_redrv: c_uint,
    pub 32]: char an_name[IFNAMSIZ +,
    pub an_workqueue: *mut workqueue_struct,
    pub an_irq: c_int,
    pub an_irq_work: work_struct,
// Auto-negotiation state machine support
    pub an_int: c_uint,
    pub an_status: c_uint,
    pub an_mutex: mutex,
    pub an_result: xgbe_an,
    pub an_state: xgbe_an,
    pub kr_state: xgbe_rx,
    pub kx_state: xgbe_rx,
    pub an_work: work_struct,
    pub an_again: c_uint,
    pub an_supported: c_uint,
    pub parallel_detect: c_uint,
    pub fec_ability: c_uint,
    pub an_start: c_ulong,
    pub kr_start_time: c_ulong,
    pub an_mode: xgbe_an_mode,
// I2C support
    pub i2c: xgbe_i2c,
    pub i2c_mutex: mutex,
    pub i2c_complete: completion,
    pub 32]: char i2c_name[IFNAMSIZ +,
    pub /: *mut *mut unsigned int lpm_ctrl; / CTRL1 for resume,
    pub isr_as_bh_work: c_uint,
    pub dev_bh_work: work_struct,
    pub ecc_bh_work: work_struct,
    pub i2c_bh_work: work_struct,
    pub an_bh_work: work_struct,
    pub xgbe_debugfs: *mut dentry,
    pub debugfs_xgmac_reg: c_uint,
    pub debugfs_xpcs_mmd: c_uint,
    pub debugfs_xpcs_reg: c_uint,
    pub debugfs_xprop_reg: c_uint,
    pub debugfs_xi2c_reg: c_uint,
    pub debugfs_an_cdr_workaround: bool,
    pub debugfs_an_cdr_track_early: bool,
    pub en_rx_adap: bool,
    pub rx_adapt_retries: c_int,
    pub rx_adapt_done: bool,
// Flag to track if data path (TX/RX) was stopped for RX adaptation.
// This prevents packet corruption during the adaptation window.
//
    pub data_path_stopped: bool,
    pub mode_set: bool,
    pub sph: bool,
}

// Function prototypes
extern "C" {
    pub fn xgbe_free_pdata(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_set_counts(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_config_netdev(: *mut xgbe_prv_data) -> c_int;
}
extern "C" {
    pub fn xgbe_deconfig_netdev(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_platform_init() -> c_int;
}
extern "C" {
    pub fn xgbe_platform_exit();
}

extern "C" {
    pub fn xgbe_pci_init() -> c_int;
}
extern "C" {
    pub fn xgbe_pci_exit();
}

extern "C" {
    pub fn xgbe_init_function_ptrs_dev(: *mut xgbe_hw_if);
}
extern "C" {
    pub fn xgbe_init_function_ptrs_phy(: *mut xgbe_phy_if);
}
extern "C" {
    pub fn xgbe_init_function_ptrs_phy_v1(: *mut xgbe_phy_if);
}
extern "C" {
    pub fn xgbe_init_function_ptrs_phy_v2(: *mut xgbe_phy_if);
}
extern "C" {
    pub fn xgbe_init_function_ptrs_desc(: *mut xgbe_desc_if);
}
extern "C" {
    pub fn xgbe_init_function_ptrs_i2c(: *mut xgbe_i2c_if);
}

extern "C" {
    pub fn xgbe_ptp_register(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_ptp_unregister(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_print_pkt(: *mut net_device, : *mut sk_buff, _arg: bool);
}
extern "C" {
    pub fn xgbe_get_all_hw_features(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_powerup(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn xgbe_powerdown(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn xgbe_init_rx_coalesce(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_init_tx_coalesce(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_restart_dev(pdata: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_full_restart_dev(pdata: *mut xgbe_prv_data);
}
// For Timestamp config
extern "C" {
    pub fn xgbe_config_tstamp(pdata: *mut xgbe_prv_data, mac_tscr: c_uint);
}
extern "C" {
    pub fn xgbe_get_tstamp_time(pdata: *mut xgbe_prv_data) -> u64;
}
extern "C" {
    pub fn xgbe_get_tx_tstamp(pdata: *mut xgbe_prv_data) -> u64;
}
extern "C" {
    pub fn xgbe_tx_tstamp(work: *mut work_struct);
}
extern "C" {
    pub fn xgbe_init_ptp(pdata: *mut xgbe_prv_data) -> c_int;
}
// Selftest functions
extern "C" {
    pub fn xgbe_selftest_get_strings(pdata: *mut xgbe_prv_data, data: *mut u8);
}
extern "C" {
    pub fn xgbe_selftest_get_count(pdata: *mut xgbe_prv_data) -> c_int;
}
// Loopback control
extern "C" {
    pub fn xgbe_enable_mac_loopback(pdata: *mut xgbe_prv_data) -> c_int;
}
extern "C" {
    pub fn xgbe_disable_mac_loopback(pdata: *mut xgbe_prv_data);
}

extern "C" {
    pub fn xgbe_debugfs_init(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_debugfs_exit(: *mut xgbe_prv_data);
}
extern "C" {
    pub fn xgbe_debugfs_rename(pdata: *mut xgbe_prv_data);
}

// NOTE: Uncomment for function trace log messages in KERNEL LOG

// Macro flag: #define YDEBUG
// Macro flag: #define YDEBUG_MDIO

// For debug prints

