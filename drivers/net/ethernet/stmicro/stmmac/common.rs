//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/common.h
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

// Macro flag: #define STMMAC_VLAN_TAG_USED

// Synopsys Core versions
pub const DWMAC_CORE_3_40: c_uint = 0x34;
pub const DWMAC_CORE_3_50: c_uint = 0x35;
pub const DWMAC_CORE_3_70: c_uint = 0x37;
pub const DWMAC_CORE_4_00: c_uint = 0x40;
pub const DWMAC_CORE_4_10: c_uint = 0x41;
pub const DWMAC_CORE_5_00: c_uint = 0x50;
pub const DWMAC_CORE_5_10: c_uint = 0x51;
pub const DWMAC_CORE_5_20: c_uint = 0x52;
pub const DWXGMAC_CORE_2_10: c_uint = 0x21;
pub const DWXGMAC_CORE_2_20: c_uint = 0x22;
pub const DWXLGMAC_CORE_2_00: c_uint = 0x20;
// Device ID
pub const DWXGMAC_ID: c_uint = 0x76;
pub const DWXLGMAC_ID: c_uint = 0x27;

// TX and RX Descriptor Length, these need to be power of two.
// TX descriptor length less than 64 may cause transmit queue timed out error.
// RX descriptor length less than 64 may cause inconsistent Rx chain error.
//
pub const DMA_MIN_TX_SIZE: c_int = 64;
pub const DMA_MAX_TX_SIZE: c_int = 1024;
pub const DMA_DEFAULT_TX_SIZE: c_int = 512;
pub const DMA_MIN_RX_SIZE: c_int = 64;
pub const DMA_MAX_RX_SIZE: c_int = 1024;
pub const DMA_DEFAULT_RX_SIZE: c_int = 512;

// #define FRAME_FILTER_DEBUG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_q_tx_stats {
    pub tx_bytes: u64_stats_t,
    pub tx_set_ic_bit: u64_stats_t,
    pub tx_tso_frames: u64_stats_t,
    pub tx_tso_nfrags: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_napi_tx_stats {
    pub tx_packets: u64_stats_t,
    pub tx_pkt_n: u64_stats_t,
    pub poll: u64_stats_t,
    pub tx_clean: u64_stats_t,
    pub tx_set_ic_bit: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_txq_stats {
// Updates protected by tx queue lock.
    pub q_syncp: u64_stats_sync,
    pub q: stmmac_q_tx_stats,
// Updates protected by NAPI poll logic.
    pub napi_syncp: u64_stats_sync,
    pub napi: stmmac_napi_tx_stats,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_napi_rx_stats {
    pub rx_bytes: u64_stats_t,
    pub rx_packets: u64_stats_t,
    pub rx_pkt_n: u64_stats_t,
    pub poll: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_rxq_stats {
// Updates protected by NAPI poll logic.
    pub napi_syncp: u64_stats_sync,
    pub napi: stmmac_napi_rx_stats,
    pub ____cacheline_aligned_in_smp: },
// Updates on each CPU protected by not allowing nested irqs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_pcpu_stats {
    pub syncp: u64_stats_sync,
    pub rx_normal_irq_n: [u64_stats_t; MTL_MAX_RX_QUEUES],
    pub tx_normal_irq_n: [u64_stats_t; MTL_MAX_TX_QUEUES],
}

// Extra statistic and debug information exposed by ethtool
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_extra_stats {
// Transmit errors
    pub ____cacheline_aligned: unsigned long tx_underflow,
    pub tx_carrier: c_ulong,
    pub tx_losscarrier: c_ulong,
    pub vlan_tag: c_ulong,
    pub tx_deferred: c_ulong,
    pub tx_vlan: c_ulong,
    pub tx_jabber: c_ulong,
    pub tx_frame_flushed: c_ulong,
    pub tx_payload_error: c_ulong,
    pub tx_ip_header_error: c_ulong,
    pub tx_collision: c_ulong,
// Receive errors
    pub rx_desc: c_ulong,
    pub sa_filter_fail: c_ulong,
    pub overflow_error: c_ulong,
    pub ipc_csum_error: c_ulong,
    pub rx_collision: c_ulong,
    pub rx_crc_errors: c_ulong,
    pub dribbling_bit: c_ulong,
    pub rx_length: c_ulong,
    pub rx_mii: c_ulong,
    pub rx_multicast: c_ulong,
    pub rx_gmac_overflow: c_ulong,
    pub rx_watchdog: c_ulong,
    pub da_rx_filter_fail: c_ulong,
    pub sa_rx_filter_fail: c_ulong,
    pub rx_missed_cntr: c_ulong,
    pub rx_overflow_cntr: c_ulong,
    pub rx_vlan: c_ulong,
    pub rx_split_hdr_pkt_n: c_ulong,
// Tx/Rx IRQ error info
    pub tx_undeflow_irq: c_ulong,
    pub tx_process_stopped_irq: c_ulong,
    pub tx_jabber_irq: c_ulong,
    pub rx_overflow_irq: c_ulong,
    pub rx_buf_unav_irq: c_ulong,
    pub rx_process_stopped_irq: c_ulong,
    pub rx_watchdog_irq: c_ulong,
    pub tx_early_irq: c_ulong,
    pub fatal_bus_error_irq: c_ulong,
// Tx/Rx IRQ Events
    pub rx_early_irq: c_ulong,
    pub threshold: c_ulong,
    pub irq_receive_pmt_irq_n: c_ulong,
// MMC info
    pub mmc_tx_irq_n: c_ulong,
    pub mmc_rx_irq_n: c_ulong,
    pub mmc_rx_csum_offload_irq_n: c_ulong,
// EEE
    pub irq_tx_path_in_lpi_mode_n: c_ulong,
    pub irq_tx_path_exit_lpi_mode_n: c_ulong,
    pub irq_rx_path_in_lpi_mode_n: c_ulong,
    pub irq_rx_path_exit_lpi_mode_n: c_ulong,
    pub phy_eee_wakeup_error_n: c_ulong,
// Extended RDES status
    pub ip_hdr_err: c_ulong,
    pub ip_payload_err: c_ulong,
    pub ip_csum_bypassed: c_ulong,
    pub ipv4_pkt_rcvd: c_ulong,
    pub ipv6_pkt_rcvd: c_ulong,
    pub no_ptp_rx_msg_type_ext: c_ulong,
    pub ptp_rx_msg_type_sync: c_ulong,
    pub ptp_rx_msg_type_follow_up: c_ulong,
    pub ptp_rx_msg_type_delay_req: c_ulong,
    pub ptp_rx_msg_type_delay_resp: c_ulong,
    pub ptp_rx_msg_type_pdelay_req: c_ulong,
    pub ptp_rx_msg_type_pdelay_resp: c_ulong,
    pub ptp_rx_msg_type_pdelay_follow_up: c_ulong,
    pub ptp_rx_msg_type_announce: c_ulong,
    pub ptp_rx_msg_type_management: c_ulong,
    pub ptp_rx_msg_pkt_reserved_type: c_ulong,
    pub ptp_frame_type: c_ulong,
    pub ptp_ver: c_ulong,
    pub timestamp_dropped: c_ulong,
    pub av_pkt_rcvd: c_ulong,
    pub av_tagged_pkt_rcvd: c_ulong,
    pub vlan_tag_priority_val: c_ulong,
    pub l3_filter_match: c_ulong,
    pub l4_filter_match: c_ulong,
    pub l3_l4_filter_no_match: c_ulong,
// PCS
    pub irq_pcs_ane_n: c_ulong,
    pub irq_pcs_link_n: c_ulong,
    pub irq_rgmii_n: c_ulong,
// debug register
    pub mtl_tx_status_fifo_full: c_ulong,
    pub mtl_tx_fifo_not_empty: c_ulong,
    pub mmtl_fifo_ctrl: c_ulong,
    pub mtl_tx_fifo_read_ctrl_write: c_ulong,
    pub mtl_tx_fifo_read_ctrl_wait: c_ulong,
    pub mtl_tx_fifo_read_ctrl_read: c_ulong,
    pub mtl_tx_fifo_read_ctrl_idle: c_ulong,
    pub mac_tx_in_pause: c_ulong,
    pub mac_tx_frame_ctrl_xfer: c_ulong,
    pub mac_tx_frame_ctrl_idle: c_ulong,
    pub mac_tx_frame_ctrl_wait: c_ulong,
    pub mac_tx_frame_ctrl_pause: c_ulong,
    pub mac_gmii_tx_proto_engine: c_ulong,
    pub mtl_rx_fifo_fill_level_full: c_ulong,
    pub mtl_rx_fifo_fill_above_thresh: c_ulong,
    pub mtl_rx_fifo_fill_below_thresh: c_ulong,
    pub mtl_rx_fifo_fill_level_empty: c_ulong,
    pub mtl_rx_fifo_read_ctrl_flush: c_ulong,
    pub mtl_rx_fifo_read_ctrl_read_data: c_ulong,
    pub mtl_rx_fifo_read_ctrl_status: c_ulong,
    pub mtl_rx_fifo_read_ctrl_idle: c_ulong,
    pub mtl_rx_fifo_ctrl_active: c_ulong,
    pub mac_rx_frame_ctrl_fifo: c_ulong,
    pub mac_gmii_rx_proto_engine: c_ulong,
// EST
    pub mtl_est_cgce: c_ulong,
    pub mtl_est_hlbs: c_ulong,
    pub mtl_est_hlbf: c_ulong,
    pub mtl_est_btre: c_ulong,
    pub mtl_est_btrlm: c_ulong,
    pub max_sdu_txq_drop: [c_ulong; MTL_MAX_TX_QUEUES],
    pub mtl_est_txq_hlbf: [c_ulong; MTL_MAX_TX_QUEUES],
    pub mtl_est_txq_hlbs: [c_ulong; MTL_MAX_TX_QUEUES],
// per queue statistics
    pub txq_stats: [stmmac_txq_stats; MTL_MAX_TX_QUEUES],
    pub rxq_stats: [stmmac_rxq_stats; MTL_MAX_RX_QUEUES],
    pub pcpu_stats: *mut stmmac_pcpu_stats __percpu,
    pub rx_dropped: c_ulong,
    pub rx_errors: c_ulong,
    pub tx_dropped: c_ulong,
    pub tx_errors: c_ulong,
}

// Safety Feature statistics exposed by ethtool
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_safety_stats {
    pub mac_errors: [c_ulong; 32],
    pub mtl_errors: [c_ulong; 32],
    pub dma_errors: [c_ulong; 32],
    pub dma_dpp_errors: [c_ulong; 32],
}

// Number of fields in Safety Stats

// CSR Frequency Access Defines
pub const CSR_F_20M: c_int = 20000000;
pub const CSR_F_35M: c_int = 35000000;
pub const CSR_F_60M: c_int = 60000000;
pub const CSR_F_100M: c_int = 100000000;
pub const CSR_F_150M: c_int = 150000000;
pub const CSR_F_250M: c_int = 250000000;
pub const CSR_F_300M: c_int = 300000000;
pub const CSR_F_500M: c_int = 500000000;
pub const CSR_F_800M: c_int = 800000000;
pub const MAC_CSR_H_FRQ_MASK: c_uint = 0x20;
pub const HASH_TABLE_SIZE: c_int = 64;
pub const PAUSE_TIME: c_uint = 0xffff;
// Flow Control defines
pub const FLOW_OFF: c_int = 0;
pub const FLOW_RX: c_int = 1;
pub const FLOW_TX: c_int = 2;

// DMA HW feature register fields
pub const DMA_HW_FEAT_MIISEL: c_uint = 0x00000001	/* 10/100 Mbps Support */;
pub const DMA_HW_FEAT_GMIISEL: c_uint = 0x00000002	/* 1000 Mbps Support */;
pub const DMA_HW_FEAT_HDSEL: c_uint = 0x00000004	/* Half-Duplex Support */;
pub const DMA_HW_FEAT_EXTHASHEN: c_uint = 0x00000008	/* Expanded DA Hash Filter */;
pub const DMA_HW_FEAT_HASHSEL: c_uint = 0x00000010	/* HASH Filter */;
pub const DMA_HW_FEAT_ADDMAC: c_uint = 0x00000020	/* Multiple MAC Addr Reg */;
pub const DMA_HW_FEAT_PCSSEL: c_uint = 0x00000040	/* PCS registers */;
pub const DMA_HW_FEAT_L3L4FLTREN: c_uint = 0x00000080	/* Layer 3 & Layer 4 Feature */;
pub const DMA_HW_FEAT_SMASEL: c_uint = 0x00000100	/* SMA(MDIO) Interface */;
pub const DMA_HW_FEAT_RWKSEL: c_uint = 0x00000200	/* PMT Remote Wakeup */;
pub const DMA_HW_FEAT_MGKSEL: c_uint = 0x00000400	/* PMT Magic Packet */;
pub const DMA_HW_FEAT_MMCSEL: c_uint = 0x00000800	/* RMON Module */;
pub const DMA_HW_FEAT_TSVER1SEL: c_uint = 0x00001000	/* Only IEEE 1588-2002 */;
pub const DMA_HW_FEAT_TSVER2SEL: c_uint = 0x00002000	/* IEEE 1588-2008 PTPv2 */;
pub const DMA_HW_FEAT_EEESEL: c_uint = 0x00004000	/* Energy Efficient Ethernet */;
pub const DMA_HW_FEAT_AVSEL: c_uint = 0x00008000	/* AV Feature */;
pub const DMA_HW_FEAT_TXCOESEL: c_uint = 0x00010000	/* Checksum Offload in Tx */;
pub const DMA_HW_FEAT_RXTYP1COE: c_uint = 0x00020000	/* IP COE (Type 1) in Rx */;
pub const DMA_HW_FEAT_RXTYP2COE: c_uint = 0x00040000	/* IP COE (Type 2) in Rx */;
pub const DMA_HW_FEAT_RXFIFOSIZE: c_uint = 0x00080000	/* Rx FIFO > 2048 Bytes */;
pub const DMA_HW_FEAT_RXCHCNT: c_uint = 0x00300000	/* No. additional Rx Channels */;
pub const DMA_HW_FEAT_TXCHCNT: c_uint = 0x00c00000	/* No. additional Tx Channels */;
pub const DMA_HW_FEAT_ENHDESSEL: c_uint = 0x01000000	/* Alternate Descriptor */;
// Timestamping with Internal System Time
pub const DMA_HW_FEAT_INTTSEN: c_uint = 0x02000000;
pub const DMA_HW_FEAT_FLEXIPPSEN: c_uint = 0x04000000	/* Flexible PPS Output */;
pub const DMA_HW_FEAT_SAVLANINS: c_uint = 0x08000000	/* Source Addr or VLAN */;
pub const DMA_HW_FEAT_ACTPHYIF: c_uint = 0x70000000	/* Active/selected PHY iface */;
pub const DEFAULT_DMA_PBL: c_int = 8;
// phy_intf_sel_i and ACTPHYIF encodings
pub const PHY_INTF_SEL_GMII_MII: c_int = 0;
pub const PHY_INTF_SEL_RGMII: c_int = 1;
pub const PHY_INTF_SEL_SGMII: c_int = 2;
pub const PHY_INTF_SEL_TBI: c_int = 3;
pub const PHY_INTF_SEL_RMII: c_int = 4;
pub const PHY_INTF_SEL_RTBI: c_int = 5;
pub const PHY_INTF_SEL_SMII: c_int = 6;
pub const PHY_INTF_SEL_REVMII: c_int = 7;
// XGMAC uses a different encoding - from the AgileX5 documentation
pub const PHY_INTF_GMII: c_int = 0;
pub const PHY_INTF_RGMII: c_int = 1;
// MSI defines
pub const STMMAC_MSI_VEC_MAX: c_int = 32;
// PCS status and mask defines

// Max/Min RI Watchdog Timer count value
pub const MAX_DMA_RIWT: c_uint = 0xff;
pub const MIN_DMA_RIWT: c_uint = 0x10;
pub const DEF_DMA_RIWT: c_uint = 0xa0;
// Tx coalesce parameters
pub const STMMAC_COAL_TX_TIMER: c_int = 5000;
pub const STMMAC_MAX_COAL_TX_TICK: c_int = 100000;
pub const STMMAC_TX_MAX_FRAMES: c_int = 256;
pub const STMMAC_TX_FRAMES: c_int = 25;
pub const STMMAC_RX_FRAMES: c_int = 0;
// Packets types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum packets_types {
    PACKET_AVCPQ = 0x1, /* AV Untagged Control packets */
    PACKET_PTPQ = 0x2, /* PTP Packets */
    PACKET_DCBCPQ = 0x3, /* DCB Control Packets */
    PACKET_UPQ = 0x4, /* Untagged Packets */
    PACKET_MCBCQ = 0x5, /* Multicast & Broadcast Packets */
}

// Rx IPC status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_frame_status {
    good_frame = 0x0,
    discard_frame = 0x1,
    csum_none = 0x2,
    llc_snap = 0x4,
    dma_own = 0x8,
    rx_not_ls = 0x10,
}

// Tx status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_frame_status {
    tx_done = 0x0,
    tx_not_ls = 0x1,
    tx_err = 0x2,
    tx_dma_own = 0x4,
    tx_err_bump_tc = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_irq_status {
    tx_hard_error = 0x1,
    tx_hard_error_bump_tc = 0x2,
    handle_rx = 0x4,
    handle_tx = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_irq_dir {
    DMA_DIR_RX = 0x1,
    DMA_DIR_TX = 0x2,
    DMA_DIR_RXTX = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum request_irq_err {
    REQ_IRQ_ERR_ALL,
    REQ_IRQ_ERR_TX,
    REQ_IRQ_ERR_RX,
    REQ_IRQ_ERR_SFTY,
    REQ_IRQ_ERR_SFTY_UE,
    REQ_IRQ_ERR_SFTY_CE,
    REQ_IRQ_ERR_WOL,
    REQ_IRQ_ERR_MAC,
    REQ_IRQ_ERR_NO,
}

// EEE and LPI defines

// FPE defines
pub const FPE_EVENT_UNKNOWN: c_int = 0;

// DMA HW capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_features {
    pub mbps_10_100: c_uint,
    pub mbps_1000: c_uint,
    pub half_duplex: c_uint,
    pub hash_filter: c_uint,
    pub multi_addr: c_uint,
    pub pcs: c_uint,
    pub sma_mdio: c_uint,
    pub pmt_remote_wake_up: c_uint,
    pub pmt_magic_frame: c_uint,
    pub rmon: c_uint,
// IEEE 1588-2002
    pub time_stamp: c_uint,
// IEEE 1588-2008
    pub atime_stamp: c_uint,
// 802.3az - Energy-Efficient Ethernet (EEE)
    pub eee: c_uint,
    pub av: c_uint,
    pub hash_tb_sz: c_uint,
    pub tsoen: c_uint,
// TX and RX csum
    pub tx_coe: c_uint,
    pub rx_coe: c_uint,
    pub rx_coe_type1: c_uint,
    pub rx_coe_type2: c_uint,
    pub rxfifo_over_2048: c_uint,
// TX and RX number of channels
    pub number_rx_channel: c_uint,
    pub number_tx_channel: c_uint,
// TX and RX number of queues
    pub number_rx_queues: u8,
    pub number_tx_queues: u8,
// PPS output
    pub pps_out_num: c_uint,
// Number of Traffic Classes
    pub numtc: c_uint,
// DCB Feature Enable
    pub dcben: c_uint,
// IEEE 1588 High Word Register Enable
    pub advthword: c_uint,
// PTP Offload Enable
    pub ptoen: c_uint,
// One-Step Timestamping Enable
    pub osten: c_uint,
// Priority-Based Flow Control Enable
    pub pfcen: c_uint,
// Alternate (enhanced) DESC mode
    pub enh_desc: c_uint,
// TX and RX FIFO sizes
    pub tx_fifo_size: c_uint,
    pub rx_fifo_size: c_uint,
// Automotive Safety Package
    pub asp: c_uint,
// RX Parser
    pub frpsel: c_uint,
    pub frpbs: c_uint,
    pub frpes: c_uint,
    pub addr64: c_uint,
    pub host_dma_width: c_uint,
    pub rssen: c_uint,
    pub vlhash: c_uint,
    pub sphen: c_uint,
    pub vlins: c_uint,
    pub dvlan: c_uint,
    pub l3l4fnum: c_uint,
    pub arpoffsel: c_uint,
// One Step for PTP over UDP/IP Feature Enable
    pub pou_ost_en: c_uint,
// Tx Timestamp FIFO Depth
    pub ttsfd: c_uint,
// Queue/Channel-Based VLAN tag insertion on Tx
    pub cbtisel: c_uint,
// Supported Parallel Instruction Processor Engines
    pub frppipe_num: c_uint,
// Number of Extended VLAN Tag Filters
    pub nrvf_num: c_uint,
// TSN Features
    pub estwid: c_uint,
    pub estdep: c_uint,
    pub estsel: c_uint,
    pub fpesel: c_uint,
    pub tbssel: c_uint,
// Number of DMA channels enabled for TBS
    pub tbs_ch_num: c_uint,
// Per-Stream Filtering Enable
    pub sgfsel: c_uint,
// Numbers of Auxiliary Snapshot Inputs
    pub aux_snapshot_n: c_uint,
// Timestamp System Time Source
    pub tssrc: c_uint,
// Enhanced DMA Enable
    pub edma: c_uint,
// Different Descriptor Cache Enable
    pub ediffc: c_uint,
// VxLAN/NVGRE Enable
    pub vxn: c_uint,
// Debug Memory Interface Enable
    pub dbgmem: c_uint,
// Number of Policing Counters
    pub pcsel: c_uint,
// Active PHY interface, PHY_INTF_SEL_xxx
    pub actphyif: u8,
}

// RX Buffer size must be multiple of 4/8/16 bytes
pub const BUF_SIZE_16KiB: c_int = 16368;
pub const BUF_SIZE_8KiB: c_int = 8188;
pub const BUF_SIZE_4KiB: c_int = 4096;
pub const BUF_SIZE_2KiB: c_int = 2048;
// Power Down and WOL
pub const PMT_NOT_SUPPORTED: c_int = 0;
pub const PMT_SUPPORTED: c_int = 1;
// Common MAC defines
pub const MAC_CTRL_REG: c_uint = 0x00000000	/* MAC Control */;
pub const MAC_ENABLE_TX: c_uint = 0x00000008	/* Transmitter Enable */;
pub const MAC_ENABLE_RX: c_uint = 0x00000004	/* Receiver Enable */;
// Default LPI timers
pub const STMMAC_DEFAULT_LIT_LS: c_uint = 0x3E8;
pub const STMMAC_DEFAULT_TWT_LS: c_uint = 0x1E;
pub const STMMAC_ET_MAX: c_uint = 0xFFFFF;
// Common LPI register bits

// Common definitions for AXI Master Bus Mode

extern "C" {
    pub fn stmmac_axi_blen_to_mask(regval: *mut u32, blen: *const u32, len: usize);
}
pub const STMMAC_CHAIN_MODE: c_uint = 0x1;
pub const STMMAC_RING_MODE: c_uint = 0x2;
pub const JUMBO_LEN: c_int = 9000;
// Receive Side Scaling
pub const STMMAC_RSS_HASH_KEY_SIZE: c_int = 40;
pub const STMMAC_RSS_MAX_TABLE_SIZE: c_int = 256;
// VLAN
pub const STMMAC_VLAN_NONE: c_uint = 0x0;
pub const STMMAC_VLAN_REMOVE: c_uint = 0x1;
pub const STMMAC_VLAN_INSERT: c_uint = 0x2;
pub const STMMAC_VLAN_REPLACE: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_link {
    pub caps: u32,
    pub speed_mask: u32,
    pub speed10: u32,
    pub speed100: u32,
    pub speed1000: u32,
    pub speed2500: u32,
    pub duplex: u32,
    pub speed2500: u32,
    pub speed5000: u32,
    pub speed10000: u32,
    pub xgmii: },
    pub speed25000: u32,
    pub speed40000: u32,
    pub speed50000: u32,
    pub speed100000: u32,
    pub xlgmii: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_regs {
    pub /: *mut *mut unsigned int addr; / MII Address,
    pub /: *mut *mut unsigned int data; / MII Data,
    pub /: *mut *mut u32 addr_mask; / MII address mask,
    pub /: *mut *mut u32 reg_mask; / MII reg mask,
    pub clk_csr_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_device_info {
    pub mac: *const stmmac_ops,
    pub desc: *const stmmac_desc_ops,
    pub dma: *const stmmac_dma_ops,
    pub mode: *const stmmac_mode_ops,
    pub ptp: *const stmmac_hwtimestamp,
    pub tc: *const stmmac_tc_ops,
    pub mmc: *const stmmac_mmc_ops,
    pub est: *const stmmac_est_ops,
    pub vlan: *const stmmac_vlan_ops,
    pub xpcs: *mut dw_xpcs,
    pub phylink_pcs: *mut phylink_pcs,
    pub /: *mut *mut mii_regs mii; / MII register Addresses,
    pub link: mac_link,
    pub /: *mut *mut *mut void __iomem pcsr; / vpointer to device CSRs,
    pub multicast_filter_bins: c_uint,
    pub unicast_filter_entries: c_uint,
    pub mcast_bits_log2: c_uint,
    pub rx_csum: c_uint,
    pub num_vlan: c_uint,
    pub vlan_filter: [u32; 32],
    pub vlan_fail_q_en: bool,
    pub vlan_fail_q: u8,
    pub hw_vlan_en: bool,
    pub reverse_sgmii_enable: bool,
// This spinlock protects read-modify-write of the interrupt
// mask/enable registers.
//
    pub irq_ctrl_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_rx_routing {
    pub reg_mask: u32,
    pub reg_shift: u32,
}

extern "C" {
    pub fn dwmac100_setup(priv: *mut stmmac_priv) -> c_int;
}
extern "C" {
    pub fn dwmac1000_setup(priv: *mut stmmac_priv) -> c_int;
}
extern "C" {
    pub fn dwmac4_setup(priv: *mut stmmac_priv) -> c_int;
}
extern "C" {
    pub fn dwxgmac2_setup(priv: *mut stmmac_priv) -> c_int;
}
extern "C" {
    pub fn dwxlgmac2_setup(priv: *mut stmmac_priv) -> c_int;
}
extern "C" {
    pub fn stmmac_set_mac(ioaddr: *mut void __iomem, enable: bool);
}
extern "C" {
    pub fn stmmac_dwmac4_set_mac(ioaddr: *mut void __iomem, enable: bool);
}
extern "C" {
    pub fn dwmac_dma_flush_tx_fifo(ioaddr: *mut void __iomem);
}
