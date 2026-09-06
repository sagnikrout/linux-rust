//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/samsung/sxgbe/sxgbe_common.h
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
// 10G controller driver for Samsung SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//
// forward references

// MAX HW feature words
pub const SXGBE_HW_WORDS: c_int = 3;
pub const SXGBE_RX_COE_NONE: c_int = 0;
// CSR Frequency Access Defines
pub const SXGBE_CSR_F_150M: c_int = 150000000;
pub const SXGBE_CSR_F_250M: c_int = 250000000;
pub const SXGBE_CSR_F_300M: c_int = 300000000;
pub const SXGBE_CSR_F_350M: c_int = 350000000;
pub const SXGBE_CSR_F_400M: c_int = 400000000;
pub const SXGBE_CSR_F_500M: c_int = 500000000;
// pause time
pub const SXGBE_PAUSE_TIME: c_uint = 0x200;
// tx queues
pub const SXGBE_TX_QUEUES: c_int = 8;
pub const SXGBE_RX_QUEUES: c_int = 16;
// Calculated based how much time does it take to fill 256KB Rx memory
// at 10Gb speed at 156MHz clock rate and considered little less then
// the actual value.
//
pub const SXGBE_MAX_DMA_RIWT: c_uint = 0x70;
pub const SXGBE_MIN_DMA_RIWT: c_uint = 0x01;
// Tx coalesce parameters
pub const SXGBE_COAL_TX_TIMER: c_int = 40000;
pub const SXGBE_MAX_COAL_TX_TICK: c_int = 100000;
pub const SXGBE_TX_MAX_FRAMES: c_int = 512;
pub const SXGBE_TX_FRAMES: c_int = 128;
// SXGBE TX FIFO is 8K, Rx FIFO is 16K
pub const BUF_SIZE_16KiB: c_int = 16384;
pub const BUF_SIZE_8KiB: c_int = 8192;
pub const BUF_SIZE_4KiB: c_int = 4096;
pub const BUF_SIZE_2KiB: c_int = 2048;
pub const SXGBE_DEFAULT_LIT_LS: c_uint = 0x3E8;
pub const SXGBE_DEFAULT_TWT_LS: c_uint = 0x0;
// Flow Control defines
pub const SXGBE_FLOW_OFF: c_int = 0;
pub const SXGBE_FLOW_RX: c_int = 1;
pub const SXGBE_FLOW_TX: c_int = 2;

// errors
pub const RX_GMII_ERR: c_uint = 0x01;
pub const RX_WATCHDOG_ERR: c_uint = 0x02;
pub const RX_CRC_ERR: c_uint = 0x03;
pub const RX_GAINT_ERR: c_uint = 0x04;
pub const RX_IP_HDR_ERR: c_uint = 0x05;
pub const RX_PAYLOAD_ERR: c_uint = 0x06;
pub const RX_OVERFLOW_ERR: c_uint = 0x07;
// pkt type
pub const RX_LEN_PKT: c_uint = 0x00;
pub const RX_MACCTL_PKT: c_uint = 0x01;
pub const RX_DCBCTL_PKT: c_uint = 0x02;
pub const RX_ARP_PKT: c_uint = 0x03;
pub const RX_OAM_PKT: c_uint = 0x04;
pub const RX_UNTAG_PKT: c_uint = 0x05;
pub const RX_OTHER_PKT: c_uint = 0x07;
pub const RX_SVLAN_PKT: c_uint = 0x08;
pub const RX_CVLAN_PKT: c_uint = 0x09;
pub const RX_DVLAN_OCVLAN_ICVLAN_PKT: c_uint = 0x0A;
pub const RX_DVLAN_OSVLAN_ISVLAN_PKT: c_uint = 0x0B;
pub const RX_DVLAN_OSVLAN_ICVLAN_PKT: c_uint = 0x0C;
pub const RX_DVLAN_OCVLAN_ISVLAN_PKT: c_uint = 0x0D;
pub const RX_NOT_IP_PKT: c_uint = 0x00;
pub const RX_IPV4_TCP_PKT: c_uint = 0x01;
pub const RX_IPV4_UDP_PKT: c_uint = 0x02;
pub const RX_IPV4_ICMP_PKT: c_uint = 0x03;
pub const RX_IPV4_UNKNOWN_PKT: c_uint = 0x07;
pub const RX_IPV6_TCP_PKT: c_uint = 0x09;
pub const RX_IPV6_UDP_PKT: c_uint = 0x0A;
pub const RX_IPV6_ICMP_PKT: c_uint = 0x0B;
pub const RX_IPV6_UNKNOWN_PKT: c_uint = 0x0F;
pub const RX_NO_PTP: c_uint = 0x00;
pub const RX_PTP_SYNC: c_uint = 0x01;
pub const RX_PTP_FOLLOW_UP: c_uint = 0x02;
pub const RX_PTP_DELAY_REQ: c_uint = 0x03;
pub const RX_PTP_DELAY_RESP: c_uint = 0x04;
pub const RX_PTP_PDELAY_REQ: c_uint = 0x05;
pub const RX_PTP_PDELAY_RESP: c_uint = 0x06;
pub const RX_PTP_PDELAY_FOLLOW_UP: c_uint = 0x07;
pub const RX_PTP_ANNOUNCE: c_uint = 0x08;
pub const RX_PTP_MGMT: c_uint = 0x09;
pub const RX_PTP_SIGNAL: c_uint = 0x0A;
pub const RX_PTP_RESV_MSG: c_uint = 0x0F;
// EEE-LPI mode  flags
pub const TX_ENTRY_LPI_MODE: c_uint = 0x10;
pub const TX_EXIT_LPI_MODE: c_uint = 0x20;
pub const RX_ENTRY_LPI_MODE: c_uint = 0x40;
pub const RX_EXIT_LPI_MODE: c_uint = 0x80;
// EEE-LPI Interrupt status flag

// EEE-LPI Default timer values
pub const LPI_LINK_STATUS_TIMER: c_uint = 0x3E8;
pub const LPI_MAC_WAIT_TIMER: c_uint = 0x00;
// EEE-LPI Control and status definitions

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_irq_status {
    tx_hard_error	= BIT(0),
    tx_bump_tc	= BIT(1),
    handle_tx	= BIT(2),
    rx_hard_error	= BIT(3),
    rx_bump_tc	= BIT(4),
    handle_rx	= BIT(5),
}

// MMC control defines
pub const SXGBE_MMC_CTRL_CNT_FRZ: c_uint = 0x00000008;
// SXGBE HW ADDR regs

pub const SXGBE_FRAME_FILTER: c_uint = 0x00000004      /* Frame Filter */;
// SXGBE Frame Filter defines
pub const SXGBE_FRAME_FILTER_PR: c_uint = 0x00000001      /* Promiscuous Mode */;
pub const SXGBE_FRAME_FILTER_HUC: c_uint = 0x00000002      /* Hash Unicast */;
pub const SXGBE_FRAME_FILTER_HMC: c_uint = 0x00000004      /* Hash Multicast */;
pub const SXGBE_FRAME_FILTER_DAIF: c_uint = 0x00000008      /* DA Inverse Filtering */;
pub const SXGBE_FRAME_FILTER_PM: c_uint = 0x00000010      /* Pass all multicast */;
pub const SXGBE_FRAME_FILTER_DBF: c_uint = 0x00000020      /* Disable Broadcast frames */;
pub const SXGBE_FRAME_FILTER_SAIF: c_uint = 0x00000100      /* Inverse Filtering */;
pub const SXGBE_FRAME_FILTER_SAF: c_uint = 0x00000200      /* Source Address Filter */;
pub const SXGBE_FRAME_FILTER_HPF: c_uint = 0x00000400      /* Hash or perfect Filter */;
pub const SXGBE_FRAME_FILTER_RA: c_uint = 0x80000000      /* Receive all mode */;
pub const SXGBE_HASH_TABLE_SIZE: c_int = 64;
pub const SXGBE_HASH_HIGH: c_uint = 0x00000008      /* Multicast Hash Table High */;
pub const SXGBE_HASH_LOW: c_uint = 0x0000000c      /* Multicast Hash Table Low */;
pub const SXGBE_HI_REG_AE: c_uint = 0x80000000;
// Minimum and maximum MTU
pub const MIN_MTU: c_int = 68;
pub const MAX_MTU: c_int = 9000;

pub const SXGBE_MAX_RX_CHANNELS: c_int = 16;
pub const SXGBE_MAX_TX_CHANNELS: c_int = 16;
pub const START_MAC_REG_OFFSET: c_uint = 0x0000;
pub const MAX_MAC_REG_OFFSET: c_uint = 0x0DFC;
pub const START_MTL_REG_OFFSET: c_uint = 0x1000;
pub const MAX_MTL_REG_OFFSET: c_uint = 0x18FC;
pub const START_DMA_REG_OFFSET: c_uint = 0x3000;
pub const MAX_DMA_REG_OFFSET: c_uint = 0x38FC;
pub const REG_SPACE_SIZE: c_uint = 0x2000;
// sxgbe statistics counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_extra_stats {
// TX/RX IRQ events
    pub tx_underflow_irq: c_ulong,
    pub tx_process_stopped_irq: c_ulong,
    pub tx_ctxt_desc_err: c_ulong,
    pub tx_threshold: c_ulong,
    pub rx_threshold: c_ulong,
    pub tx_pkt_n: c_ulong,
    pub rx_pkt_n: c_ulong,
    pub normal_irq_n: c_ulong,
    pub tx_normal_irq_n: c_ulong,
    pub rx_normal_irq_n: c_ulong,
    pub napi_poll: c_ulong,
    pub tx_clean: c_ulong,
    pub tx_reset_ic_bit: c_ulong,
    pub rx_process_stopped_irq: c_ulong,
    pub rx_underflow_irq: c_ulong,
// Bus access errors
    pub fatal_bus_error_irq: c_ulong,
    pub tx_read_transfer_err: c_ulong,
    pub tx_write_transfer_err: c_ulong,
    pub tx_desc_access_err: c_ulong,
    pub tx_buffer_access_err: c_ulong,
    pub tx_data_transfer_err: c_ulong,
    pub rx_read_transfer_err: c_ulong,
    pub rx_write_transfer_err: c_ulong,
    pub rx_desc_access_err: c_ulong,
    pub rx_buffer_access_err: c_ulong,
    pub rx_data_transfer_err: c_ulong,
// EEE-LPI stats
    pub tx_lpi_entry_n: c_ulong,
    pub tx_lpi_exit_n: c_ulong,
    pub rx_lpi_entry_n: c_ulong,
    pub rx_lpi_exit_n: c_ulong,
    pub eee_wakeup_error_n: c_ulong,
// RX specific
// L2 error
    pub rx_code_gmii_err: c_ulong,
    pub rx_watchdog_err: c_ulong,
    pub rx_crc_err: c_ulong,
    pub rx_gaint_pkt_err: c_ulong,
    pub ip_hdr_err: c_ulong,
    pub ip_payload_err: c_ulong,
    pub overflow_error: c_ulong,
// L2 Pkt type
    pub len_pkt: c_ulong,
    pub mac_ctl_pkt: c_ulong,
    pub dcb_ctl_pkt: c_ulong,
    pub arp_pkt: c_ulong,
    pub oam_pkt: c_ulong,
    pub untag_okt: c_ulong,
    pub other_pkt: c_ulong,
    pub svlan_tag_pkt: c_ulong,
    pub cvlan_tag_pkt: c_ulong,
    pub dvlan_ocvlan_icvlan_pkt: c_ulong,
    pub dvlan_osvlan_isvlan_pkt: c_ulong,
    pub dvlan_osvlan_icvlan_pkt: c_ulong,
    pub dvan_ocvlan_icvlan_pkt: c_ulong,
// L3/L4 Pkt type
    pub not_ip_pkt: c_ulong,
    pub ip4_tcp_pkt: c_ulong,
    pub ip4_udp_pkt: c_ulong,
    pub ip4_icmp_pkt: c_ulong,
    pub ip4_unknown_pkt: c_ulong,
    pub ip6_tcp_pkt: c_ulong,
    pub ip6_udp_pkt: c_ulong,
    pub ip6_icmp_pkt: c_ulong,
    pub ip6_unknown_pkt: c_ulong,
// Filter specific
    pub vlan_filter_match: c_ulong,
    pub sa_filter_fail: c_ulong,
    pub da_filter_fail: c_ulong,
    pub hash_filter_pass: c_ulong,
    pub l3_filter_match: c_ulong,
    pub l4_filter_match: c_ulong,
// RX context specific
    pub timestamp_dropped: c_ulong,
    pub rx_msg_type_no_ptp: c_ulong,
    pub rx_ptp_type_sync: c_ulong,
    pub rx_ptp_type_follow_up: c_ulong,
    pub rx_ptp_type_delay_req: c_ulong,
    pub rx_ptp_type_delay_resp: c_ulong,
    pub rx_ptp_type_pdelay_req: c_ulong,
    pub rx_ptp_type_pdelay_resp: c_ulong,
    pub rx_ptp_type_pdelay_follow_up: c_ulong,
    pub rx_ptp_announce: c_ulong,
    pub rx_ptp_mgmt: c_ulong,
    pub rx_ptp_signal: c_ulong,
    pub rx_ptp_resv_msg_type: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_link {
    pub port: c_int,
    pub duplex: c_int,
    pub speed: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_regs {
    pub /: *mut *mut unsigned int addr; / MII Address,
    pub /: *mut *mut unsigned int data; / MII Data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_core_ops {
// MAC core initialization
    pub ioaddr): *mut *mut void (core_init)(void __iomem,
// Dump MAC registers
    pub ioaddr): *mut *mut void (dump_regs)(void __iomem,
// Handle extra events on specific interrupts hw dependent
    pub x): *mut sxgbe_extra_stats,
// Set power management mode (e.g. magic frame)
    pub mode): *mut *mut *mut void (pmt)(void __iomem ioaddr, unsigned long,
// Set/Get Unicast MAC addresses
    pub reg_n): c_uint,
    pub reg_n): c_uint,
    pub enable): *mut *mut *mut void (enable_rx)(void __iomem ioaddr, bool,
    pub enable): *mut *mut *mut void (enable_tx)(void __iomem ioaddr, bool,
// controller version specific operations
    pub ioaddr): *mut *mut int (get_controller_version)(void __iomem,
// If supported then get the optional core features
    pub feature_index): c_uchar,
// adjust SXGBE speed
    pub speed): *mut *mut *mut void (set_speed)(void __iomem ioaddr, unsigned char,
// EEE-LPI specific operations
    pub ioaddr): *mut *mut void (set_eee_mode)(void __iomem,
    pub ioaddr): *mut *mut void (reset_eee_mode)(void __iomem,
    pub tw): c_int,
    pub link): *const *const *const void (set_eee_pls)(void __iomem ioaddr, int,
// Enable disable checksum offload operations
    pub ioaddr): *mut *mut void (enable_rx_csum)(void __iomem,
    pub ioaddr): *mut *mut void (disable_rx_csum)(void __iomem,
    pub queue_num): *mut *mut *mut void (enable_rxqueue)(void __iomem ioaddr, int,
    pub queue_num): *mut *mut *mut void (disable_rxqueue)(void __iomem ioaddr, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_ops {
    pub mac: *const sxgbe_core_ops,
    pub desc: *const sxgbe_desc_ops,
    pub dma: *const sxgbe_dma_ops,
    pub mtl: *const sxgbe_mtl_ops,
    pub /: *mut *mut mii_regs mii; / MII register Addresses,
    pub link: mac_link,
    pub ctrl_uid: c_uint,
    pub ctrl_id: c_uint,
}

// SXGBE private data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_tx_queue {
    pub irq_no: c_uint,
    pub priv_ptr: *mut sxgbe_priv_data,
    pub dma_tx: *mut sxgbe_tx_norm_desc,
    pub dma_tx_phy: dma_addr_t,
    pub tx_skbuff_dma: *mut dma_addr_t,
    pub tx_skbuff: *mut sk_buff,
    pub txtimer: timer_list,
    pub cur_tx: c_uint,
    pub dirty_tx: c_uint,
    pub tx_count_frames: u32,
    pub tx_coal_frames: u32,
    pub tx_coal_timer: u32,
    pub hwts_tx_en: c_int,
    pub prev_mss: u16,
    pub queue_no: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_rx_queue {
    pub priv_ptr: *mut sxgbe_priv_data,
    pub dma_rx: *mut sxgbe_rx_norm_desc,
    pub rx_skbuff: *mut sk_buff,
    pub cur_rx: c_uint,
    pub dirty_rx: c_uint,
    pub irq_no: c_uint,
    pub rx_riwt: u32,
    pub rx_skbuff_dma: *mut dma_addr_t,
    pub dma_rx_phy: dma_addr_t,
    pub queue_no: u8,
}

// SXGBE HW capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_hw_features {
// CAP [0]
    pub pmt_remote_wake_up: c_uint,
    pub pmt_magic_frame: c_uint,
// IEEE 1588-2008
    pub atime_stamp: c_uint,
    pub eee: c_uint,
    pub tx_csum_offload: c_uint,
    pub rx_csum_offload: c_uint,
    pub multi_macaddr: c_uint,
    pub tstamp_srcselect: c_uint,
    pub sa_vlan_insert: c_uint,
// CAP [1]
    pub rxfifo_size: c_uint,
    pub txfifo_size: c_uint,
    pub atstmap_hword: c_uint,
    pub dcb_enable: c_uint,
    pub splithead_enable: c_uint,
    pub tcpseg_offload: c_uint,
    pub debug_mem: c_uint,
    pub rss_enable: c_uint,
    pub hash_tsize: c_uint,
    pub l3l4_filer_size: c_uint,
// This value is in bytes and
// as mentioned in HW features
// of SXGBE data book
//
    pub rx_mtl_qsize: c_uint,
    pub tx_mtl_qsize: c_uint,
// CAP [2]
// TX and RX number of channels
    pub rx_mtl_queues: c_uint,
    pub tx_mtl_queues: c_uint,
    pub rx_dma_channels: c_uint,
    pub tx_dma_channels: c_uint,
    pub pps_output_count: c_uint,
    pub aux_input_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_priv_data {
// DMA descriptos
    pub txq: [*mut sxgbe_tx_queue; SXGBE_TX_QUEUES],
    pub rxq: [*mut sxgbe_rx_queue; SXGBE_RX_QUEUES],
    pub cur_rx_qnum: u8,
    pub dma_tx_size: c_uint,
    pub dma_rx_size: c_uint,
    pub dma_buf_sz: c_uint,
    pub rx_riwt: u32,
    pub napi: napi_struct,
    pub ioaddr: *mut void __iomem,
    pub dev: *mut net_device,
    pub device: *mut device,
    pub /: *mut *mut *mut sxgbe_ops hw; / sxgbe specific ops,
    pub no_csum_insertion: c_int,
    pub irq: c_int,
    pub rxcsum_insertion: c_int,
    pub /: *mut *mut spinlock_t stats_lock; / lock for tx/rx statatics,
    pub oldlink: c_int,
    pub speed: c_int,
    pub oldduplex: c_int,
    pub mii: *mut mii_bus,
    pub mii_irq: [c_int; PHY_MAX_ADDR],
    pub rx_pause: u8,
    pub tx_pause: u8,
    pub xstats: sxgbe_extra_stats,
    pub plat: *mut sxgbe_plat_data,
    pub hw_cap: sxgbe_hw_features,
    pub msg_enable: u32,
    pub sxgbe_clk: *mut clk,
    pub clk_csr: c_int,
    pub mode: c_uint,
    pub default_addend: c_uint,
// advanced time stamp support
    pub adv_ts: u32,
    pub use_riwt: c_int,
    pub ptp_clock: *mut ptp_clock,
// tc control
    pub tx_tc: c_int,
    pub rx_tc: c_int,
// EEE-LPI specific members
    pub eee_ctrl_timer: timer_list,
    pub tx_path_in_lpi_mode: bool,
    pub lpi_irq: c_int,
    pub eee_enabled: c_int,
    pub tx_lpi_timer: c_int,
}

// Function prototypes
extern "C" {
    pub fn sxgbe_drv_remove(ndev: *mut net_device);
}
extern "C" {
    pub fn sxgbe_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn sxgbe_mdio_unregister(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn sxgbe_mdio_register(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn sxgbe_register_platform() -> c_int;
}
extern "C" {
    pub fn sxgbe_unregister_platform();
}

extern "C" {
    pub fn sxgbe_suspend(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn sxgbe_resume(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn sxgbe_freeze(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn sxgbe_restore(ndev: *mut net_device) -> c_int;
}

extern "C" {
    pub fn sxgbe_disable_eee_mode(priv: *const *const sxgbe_priv_data);
}
extern "C" {
    pub fn sxgbe_eee_init(priv: *const *const sxgbe_priv_data) -> bool;
}
