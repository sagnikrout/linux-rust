//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/atl1e/atl1e.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright(c) 2007 Atheros Corporation. All rights reserved.
// Copyright(c) 2007 xiong huang <xiong.huang@atheros.com>
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

pub const PCI_REG_COMMAND: c_uint = 0x04    /* PCI Command Register */;
pub const CMD_IO_SPACE: c_uint = 0x0001;
pub const CMD_MEMORY_SPACE: c_uint = 0x0002;
pub const CMD_BUS_MASTER: c_uint = 0x0004;
pub const BAR_0: c_int = 0;
pub const BAR_1: c_int = 1;
pub const BAR_5: c_int = 5;
// Wake Up Filter Control
pub const AT_WUFC_LNKC: c_uint = 0x00000001 /* Link Status Change Wakeup Enable */;
pub const AT_WUFC_MAG: c_uint = 0x00000002 /* Magic Packet Wakeup Enable */;
pub const AT_WUFC_EX: c_uint = 0x00000004 /* Directed Exact Wakeup Enable */;
pub const AT_WUFC_MC: c_uint = 0x00000008 /* Multicast Wakeup Enable */;
pub const AT_WUFC_BC: c_uint = 0x00000010 /* Broadcast Wakeup Enable */;
pub const SPEED_0: c_uint = 0xffff;
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;
// Error Codes
pub const AT_ERR_EEPROM: c_int = 1;
pub const AT_ERR_PHY: c_int = 2;
pub const AT_ERR_CONFIG: c_int = 3;
pub const AT_ERR_PARAM: c_int = 4;
pub const AT_ERR_MAC_TYPE: c_int = 5;
pub const AT_ERR_PHY_TYPE: c_int = 6;
pub const AT_ERR_PHY_SPEED: c_int = 7;
pub const AT_ERR_PHY_RES: c_int = 8;
pub const AT_ERR_TIMEOUT: c_int = 9;
pub const MAX_JUMBO_FRAME_SIZE: c_uint = 0x2000;

pub const AT_MAX_RECEIVE_QUEUE: c_int = 4;
pub const AT_PAGE_NUM_PER_QUEUE: c_int = 2;
pub const AT_DMA_HI_ADDR_MASK: c_uint = 0xffffffff00000000ULL;
pub const AT_DMA_LO_ADDR_MASK: c_uint = 0x00000000ffffffffULL;

pub const AT_MAX_INT_WORK: c_int = 10;
pub const AT_TWSI_EEPROM_TIMEOUT: c_int = 100;
pub const AT_HW_MAX_IDLE_DELAY: c_int = 10;
pub const AT_SUSPEND_LINK_TIMEOUT: c_int = 28;
pub const AT_REGS_LEN: c_int = 75;
pub const AT_EEPROM_LEN: c_int = 512;

// tpd word 2
pub const TPD_BUFLEN_MASK: c_uint = 0x3FFF;
pub const TPD_BUFLEN_SHIFT: c_int = 0;
pub const TPD_DMAINT_MASK: c_uint = 0x0001;
pub const TPD_DMAINT_SHIFT: c_int = 14;
pub const TPD_PKTNT_MASK: c_uint = 0x0001;
pub const TPD_PKTINT_SHIFT: c_int = 15;
pub const TPD_VLANTAG_MASK: c_uint = 0xFFFF;
pub const TPD_VLAN_SHIFT: c_int = 16;
// tpd word 3 bits 0:4
pub const TPD_EOP_MASK: c_uint = 0x0001;
pub const TPD_EOP_SHIFT: c_int = 0;
pub const TPD_IP_VERSION_MASK: c_uint = 0x0001;

pub const TPD_INS_VL_TAG_MASK: c_uint = 0x0001;
pub const TPD_INS_VL_TAG_SHIFT: c_int = 2;
pub const TPD_CC_SEGMENT_EN_MASK: c_uint = 0x0001;
pub const TPD_CC_SEGMENT_EN_SHIFT: c_int = 3;
pub const TPD_SEGMENT_EN_MASK: c_uint = 0x0001;
pub const TPD_SEGMENT_EN_SHIFT: c_int = 4;
// tdp word 3 bits 5:7 if ip version is 0
pub const TPD_IP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_IP_CSUM_SHIFT: c_int = 5;
pub const TPD_TCP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_TCP_CSUM_SHIFT: c_int = 6;
pub const TPD_UDP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_UDP_CSUM_SHIFT: c_int = 7;
// tdp word 3 bits 5:7 if ip version is 1
pub const TPD_V6_IPHLLO_MASK: c_uint = 0x0007;
pub const TPD_V6_IPHLLO_SHIFT: c_int = 7;
// tpd word 3 bits 8:9 bit
pub const TPD_VL_TAGGED_MASK: c_uint = 0x0001;
pub const TPD_VL_TAGGED_SHIFT: c_int = 8;
pub const TPD_ETHTYPE_MASK: c_uint = 0x0001;
pub const TPD_ETHTYPE_SHIFT: c_int = 9;
// tdp word 3 bits 10:13 if ip version is 0
pub const TDP_V4_IPHL_MASK: c_uint = 0x000F;
pub const TPD_V4_IPHL_SHIFT: c_int = 10;
// tdp word 3 bits 10:13 if ip version is 1
pub const TPD_V6_IPHLHI_MASK: c_uint = 0x000F;
pub const TPD_V6_IPHLHI_SHIFT: c_int = 10;
// tpd word 3 bit 14:31 if segment enabled
pub const TPD_TCPHDRLEN_MASK: c_uint = 0x000F;
pub const TPD_TCPHDRLEN_SHIFT: c_int = 14;
pub const TPD_HDRFLAG_MASK: c_uint = 0x0001;
pub const TPD_HDRFLAG_SHIFT: c_int = 18;
pub const TPD_MSS_MASK: c_uint = 0x1FFF;
pub const TPD_MSS_SHIFT: c_int = 19;
// tdp word 3 bit 16:31 if custom csum enabled
pub const TPD_PLOADOFFSET_MASK: c_uint = 0x00FF;
pub const TPD_PLOADOFFSET_SHIFT: c_int = 16;
pub const TPD_CCSUMOFFSET_MASK: c_uint = 0x00FF;
pub const TPD_CCSUMOFFSET_SHIFT: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_tpd_desc {
    pub buffer_addr: __le64,
    pub word2: __le32,
    pub word3: __le32,
}

// how about 0x2000
pub const MAX_TX_BUF_LEN: c_uint = 0x2000;
pub const MAX_TX_BUF_SHIFT: c_int = 13;
pub const MAX_TSO_SEG_SIZE: c_uint = 0x3c00;
// rrs word 1 bit 0:31
pub const RRS_RX_CSUM_MASK: c_uint = 0xFFFF;
pub const RRS_RX_CSUM_SHIFT: c_int = 0;
pub const RRS_PKT_SIZE_MASK: c_uint = 0x3FFF;
pub const RRS_PKT_SIZE_SHIFT: c_int = 16;
pub const RRS_CPU_NUM_MASK: c_uint = 0x0003;
pub const RRS_CPU_NUM_SHIFT: c_int = 30;
pub const RRS_IS_RSS_IPV4: c_uint = 0x0001;
pub const RRS_IS_RSS_IPV4_TCP: c_uint = 0x0002;
pub const RRS_IS_RSS_IPV6: c_uint = 0x0004;
pub const RRS_IS_RSS_IPV6_TCP: c_uint = 0x0008;
pub const RRS_IS_IPV6: c_uint = 0x0010;
pub const RRS_IS_IP_FRAG: c_uint = 0x0020;
pub const RRS_IS_IP_DF: c_uint = 0x0040;
pub const RRS_IS_802_3: c_uint = 0x0080;
pub const RRS_IS_VLAN_TAG: c_uint = 0x0100;
pub const RRS_IS_ERR_FRAME: c_uint = 0x0200;
pub const RRS_IS_IPV4: c_uint = 0x0400;
pub const RRS_IS_UDP: c_uint = 0x0800;
pub const RRS_IS_TCP: c_uint = 0x1000;
pub const RRS_IS_BCAST: c_uint = 0x2000;
pub const RRS_IS_MCAST: c_uint = 0x4000;
pub const RRS_IS_PAUSE: c_uint = 0x8000;
pub const RRS_ERR_BAD_CRC: c_uint = 0x0001;
pub const RRS_ERR_CODE: c_uint = 0x0002;
pub const RRS_ERR_DRIBBLE: c_uint = 0x0004;
pub const RRS_ERR_RUNT: c_uint = 0x0008;
pub const RRS_ERR_RX_OVERFLOW: c_uint = 0x0010;
pub const RRS_ERR_TRUNC: c_uint = 0x0020;
pub const RRS_ERR_IP_CSUM: c_uint = 0x0040;
pub const RRS_ERR_L4_CSUM: c_uint = 0x0080;
pub const RRS_ERR_LENGTH: c_uint = 0x0100;
pub const RRS_ERR_DES_ADDR: c_uint = 0x0200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_recv_ret_status {
    pub seq_num: u16,
    pub hash_lo: u16,
    pub word1: __le32,
    pub pkt_flag: u16,
    pub err_flag: u16,
    pub hash_hi: u16,
    pub vtag: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1e_dma_req_block {
    atl1e_dma_req_128 = 0,
    atl1e_dma_req_256 = 1,
    atl1e_dma_req_512 = 2,
    atl1e_dma_req_1024 = 3,
    atl1e_dma_req_2048 = 4,
    atl1e_dma_req_4096 = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1e_rrs_type {
    atl1e_rrs_disable = 0,
    atl1e_rrs_ipv4 = 1,
    atl1e_rrs_ipv4_tcp = 2,
    atl1e_rrs_ipv6 = 4,
    atl1e_rrs_ipv6_tcp = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1e_nic_type {
    athr_l1e = 0,
    athr_l2e_revA = 1,
    athr_l2e_revB = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_hw_stats {
// rx
    pub /: *mut *mut unsigned long rx_ok; / The number of good packet received.,
    pub /: *mut *mut unsigned long rx_bcast; / The number of good broadcast packet received.,
    pub /: *mut *mut unsigned long rx_mcast; / The number of good multicast packet received.,
    pub /: *mut *mut unsigned long rx_pause; / The number of Pause packet received.,
    pub /: *mut *mut unsigned long rx_ctrl; / The number of Control packet received other than Pause frame.,
    pub /: *mut *mut unsigned long rx_fcs_err; / The number of packets with bad FCS.,
    pub /: *mut *mut unsigned long rx_len_err; / The number of packets with mismatch of length field and actual size.,
    pub /: *mut *mut unsigned long rx_byte_cnt; / The number of bytes of good packet received. FCS is NOT included.,
    pub /: *mut *mut unsigned long rx_runt; / The number of packets received that are less than 64 byte long and with good FCS.,
    pub /: *mut *mut unsigned long rx_frag; / The number of packets received that are less than 64 byte long and with bad FCS.,
    pub /: *mut *mut unsigned long rx_sz_64; / The number of good and bad packets received that are 64 byte long.,
    pub /: *mut *mut unsigned long rx_sz_65_127; / The number of good and bad packets received that are between 65 and 127-byte long.,
    pub /: *mut *mut unsigned long rx_sz_128_255; / The number of good and bad packets received that are between 128 and 255-byte long.,
    pub /: *mut *mut unsigned long rx_sz_256_511; / The number of good and bad packets received that are between 256 and 511-byte long.,
    pub /: *mut *mut unsigned long rx_sz_512_1023; / The number of good and bad packets received that are between 512 and 1023-byte long.,
    pub /: *mut *mut unsigned long rx_sz_1024_1518; / The number of good and bad packets received that are between 1024 and 1518-byte long.,
    pub /: *mut *mut unsigned long rx_sz_1519_max; / The number of good and bad packets received that are between 1519-byte and MTU.,
    pub /: *mut *mut unsigned long rx_sz_ov; / The number of good and bad packets received that are more than MTU size truncated by Selene.,
    pub /: *mut *mut unsigned long rx_rxf_ov; / The number of frame dropped due to occurrence of RX FIFO overflow.,
    pub /: *mut *mut unsigned long rx_rrd_ov; / The number of frame dropped due to occurrence of RRD overflow.,
    pub /: *mut *mut unsigned long rx_align_err; / Alignment Error,
    pub /: *mut *mut unsigned long rx_bcast_byte_cnt; / The byte count of broadcast packet received, excluding FCS.,
    pub /: *mut *mut unsigned long rx_mcast_byte_cnt; / The byte count of multicast packet received, excluding FCS.,
    pub /: *mut *mut unsigned long rx_err_addr; / The number of packets dropped due to address filtering.,
// tx
    pub /: *mut *mut unsigned long tx_ok; / The number of good packet transmitted.,
    pub /: *mut *mut unsigned long tx_bcast; / The number of good broadcast packet transmitted.,
    pub /: *mut *mut unsigned long tx_mcast; / The number of good multicast packet transmitted.,
    pub /: *mut *mut unsigned long tx_pause; / The number of Pause packet transmitted.,
    pub /: *mut *mut unsigned long tx_exc_defer; / The number of packets transmitted with excessive deferral.,
    pub /: *mut *mut unsigned long tx_ctrl; / The number of packets transmitted is a control frame, excluding Pause frame.,
    pub /: *mut *mut unsigned long tx_defer; / The number of packets transmitted that is deferred.,
    pub /: *mut *mut unsigned long tx_byte_cnt; / The number of bytes of data transmitted. FCS is NOT included.,
    pub /: *mut *mut unsigned long tx_sz_64; / The number of good and bad packets transmitted that are 64 byte long.,
    pub /: *mut *mut unsigned long tx_sz_65_127; / The number of good and bad packets transmitted that are between 65 and 127-byte long.,
    pub /: *mut *mut unsigned long tx_sz_128_255; / The number of good and bad packets transmitted that are between 128 and 255-byte long.,
    pub /: *mut *mut unsigned long tx_sz_256_511; / The number of good and bad packets transmitted that are between 256 and 511-byte long.,
    pub /: *mut *mut unsigned long tx_sz_512_1023; / The number of good and bad packets transmitted that are between 512 and 1023-byte long.,
    pub /: *mut *mut unsigned long tx_sz_1024_1518; / The number of good and bad packets transmitted that are between 1024 and 1518-byte long.,
    pub /: *mut *mut unsigned long tx_sz_1519_max; / The number of good and bad packets transmitted that are between 1519-byte and MTU.,
    pub /: *mut *mut unsigned long tx_1_col; / The number of packets subsequently transmitted successfully with a single prior collision.,
    pub /: *mut *mut unsigned long tx_2_col; / The number of packets subsequently transmitted successfully with multiple prior collisions.,
    pub /: *mut *mut unsigned long tx_late_col; / The number of packets transmitted with late collisions.,
    pub /: *mut *mut unsigned long tx_abort_col; / The number of transmit packets aborted due to excessive collisions.,
    pub /: *mut *mut unsigned long tx_underrun; / The number of transmit packets aborted due to transmit FIFO underrun, or TRD FIFO underrun,
    pub /: *mut *mut unsigned long tx_rd_eop; / The number of times that read beyond the EOP into the next frame area when TRD was not written timely,
    pub /: *mut *mut unsigned long tx_len_err; / The number of transmit packets with length field does NOT match the actual frame size.,
    pub /: *mut *mut unsigned long tx_trunc; / The number of transmit packets truncated due to size exceeding MTU.,
    pub /: *mut *mut unsigned long tx_bcast_byte; / The byte count of broadcast packet transmitted, excluding FCS.,
    pub /: *mut *mut unsigned long tx_mcast_byte; / The byte count of multicast packet transmitted, excluding FCS.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_hw {
    pub /: *mut *mut *mut u8 __iomem hw_addr; / inner register address,
    pub mem_rang: resource_size_t,
    pub adapter: *mut atl1e_adapter,
    pub nic_type: atl1e_nic_type,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub pci_cmd_word: u16,
    pub mac_addr: [u8; ETH_ALEN],
    pub perm_mac_addr: [u8; ETH_ALEN],
    pub preamble_len: u8,
    pub max_frame_size: u16,
    pub rx_jumbo_th: u16,
    pub tx_jumbo_th: u16,
    pub media_type: u16,
pub const MEDIA_TYPE_AUTO_SENSOR: c_int = 0;
pub const MEDIA_TYPE_100M_FULL: c_int = 1;
pub const MEDIA_TYPE_100M_HALF: c_int = 2;
pub const MEDIA_TYPE_10M_FULL: c_int = 3;
pub const MEDIA_TYPE_10M_HALF: c_int = 4;
    pub autoneg_advertised: u16,
pub const ADVERTISE_10_HALF: c_uint = 0x0001;
pub const ADVERTISE_10_FULL: c_uint = 0x0002;
pub const ADVERTISE_100_HALF: c_uint = 0x0004;
pub const ADVERTISE_100_FULL: c_uint = 0x0008;
pub const ADVERTISE_1000_HALF: c_uint = 0x0010 /* Not used, just FYI */;
pub const ADVERTISE_1000_FULL: c_uint = 0x0020;
    pub mii_autoneg_adv_reg: u16,
    pub mii_1000t_ctrl_reg: u16,
    pub /: *mut *mut u16 imt; / Interrupt Moderator timer ( 2us resolution),
    pub /: *mut *mut u16 ict; / Interrupt Clear timer (2us resolution),
    pub smb_timer: u32,
    pub trigger: *mut *mut u16 rrd_thresh; / Threshold of number of RRD produced to,
    pub tpd_thresh: u16,
    pub /: *mut *mut u16 rx_count_down; / 2us resolution,
    pub tx_count_down: u16,
    pub /: *mut *mut u8 tpd_burst; / Number of TPD to prefetch in cache-aligned burst.,
    pub rrs_type: atl1e_rrs_type,
    pub base_cpu: u32,
    pub indirect_tab: u32,
    pub dmar_block: atl1e_dma_req_block,
    pub dmaw_block: atl1e_dma_req_block,
    pub dmaw_dly_cnt: u8,
    pub dmar_dly_cnt: u8,
    pub phy_configured: bool,
    pub re_autoneg: bool,
    pub emi_ca: bool,
}

//
// wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_tx_buffer {
    pub skb: *mut sk_buff,
    pub flags: u16,
pub const ATL1E_TX_PCIMAP_SINGLE: c_uint = 0x0001;
pub const ATL1E_TX_PCIMAP_PAGE: c_uint = 0x0002;
pub const ATL1E_TX_PCIMAP_TYPE_MASK: c_uint = 0x0003;
    pub length: u16,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_rx_page {
    pub /: *mut *mut dma_addr_t dma; / receive rage DMA address,
    pub /: *mut *mut *mut u8 addr; / receive rage virtual address,
    pub the: *mut *mut dma_addr_t write_offset_dma; / the DMA address which contain,
    pub contain: *mut *mut *mut u32 write_offset_addr; / the virtaul address which,
    pub /: *mut *mut u32 read_offset; / the offset where we have read,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_rx_page_desc {
    pub rx_page: [atl1e_rx_page; AT_PAGE_NUM_PER_QUEUE],
    pub rx_using: u8,
    pub rx_nxseq: u16,
}

// transmit packet descriptor (tpd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_tx_ring {
    pub /: *mut *mut *mut atl1e_tpd_desc desc; / descriptor ring virtual address,
    pub /: *mut *mut dma_addr_t dma; / descriptor ring physical address,
    pub /: *mut *mut u16 count; / the count of transmit rings,
    pub tx_lock: rwlock_t,
    pub next_to_use: u16,
    pub next_to_clean: core::sync::atomic::AtomicI32,
    pub tx_buffer: *mut atl1e_tx_buffer,
    pub cmb_dma: dma_addr_t,
    pub cmb: *mut u32,
}

// receive packet descriptor ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_rx_ring {
    pub desc: *mut c_void,
    pub dma: dma_addr_t,
    pub size: c_int,
    pub /: *mut *mut u32 page_size; / bytes length of rxf page,
    pub /: *mut *mut u32 real_page_size; / real_page_size = page_size + jumbo + aliagn,
    pub rx_page_desc: [atl1e_rx_page_desc; AT_MAX_RECEIVE_QUEUE],
}

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1e_adapter {
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub napi: napi_struct,
    pub /: *mut *mut mii_if_info mii; / MII interface info,
    pub hw: atl1e_hw,
    pub hw_stats: atl1e_hw_stats,
    pub wol: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub mdio_lock: spinlock_t,
    pub irq_sem: core::sync::atomic::AtomicI32,
    pub reset_task: work_struct,
    pub link_chg_task: work_struct,
    pub watchdog_timer: timer_list,
    pub phy_config_timer: timer_list,
// All Descriptor memory
    pub ring_dma: dma_addr_t,
    pub ring_vir_addr: *mut c_void,
    pub ring_size: u32,
    pub tx_ring: atl1e_tx_ring,
    pub rx_ring: atl1e_rx_ring,
    pub num_rx_queues: c_int,
    pub flags: c_ulong,
pub const __AT_TESTING: c_uint = 0x0001;
pub const __AT_RESETTING: c_uint = 0x0002;
pub const __AT_DOWN: c_uint = 0x0003;
    pub number;*/: *mut *mut u32 bd_number; / board,
    pub pci_state: [u32; 16],
    pub config_space: *mut u32,
}

extern "C" {
    pub fn atl1e_check_options(adapter: *mut atl1e_adapter);
}
extern "C" {
    pub fn atl1e_up(adapter: *mut atl1e_adapter) -> c_int;
}
extern "C" {
    pub fn atl1e_down(adapter: *mut atl1e_adapter);
}
extern "C" {
    pub fn atl1e_reinit_locked(adapter: *mut atl1e_adapter);
}
extern "C" {
    pub fn atl1e_reset_hw(hw: *mut atl1e_hw) -> i32;
}
extern "C" {
    pub fn atl1e_set_ethtool_ops(netdev: *mut net_device);
}
