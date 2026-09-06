//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/atl1c/atl1c.h
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
// Copyright(c) 2008 - 2009 Atheros Corporation. All rights reserved.
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

// Wake Up Filter Control
pub const AT_WUFC_LNKC: c_uint = 0x00000001 /* Link Status Change Wakeup Enable */;
pub const AT_WUFC_MAG: c_uint = 0x00000002 /* Magic Packet Wakeup Enable */;
pub const AT_WUFC_EX: c_uint = 0x00000004 /* Directed Exact Wakeup Enable */;
pub const AT_WUFC_MC: c_uint = 0x00000008 /* Multicast Wakeup Enable */;
pub const AT_WUFC_BC: c_uint = 0x00000010 /* Broadcast Wakeup Enable */;

pub const SPEED_0: c_uint = 0xffff;
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;

pub const AT_MAX_RECEIVE_QUEUE: c_int = 4;
pub const AT_DEF_RECEIVE_QUEUE: c_int = 1;
pub const AT_MAX_TRANSMIT_QUEUE: c_int = 4;
pub const AT_DMA_HI_ADDR_MASK: c_uint = 0xffffffff00000000ULL;
pub const AT_DMA_LO_ADDR_MASK: c_uint = 0x00000000ffffffffULL;

pub const AT_MAX_INT_WORK: c_int = 5;
pub const AT_TWSI_EEPROM_TIMEOUT: c_int = 100;
pub const AT_HW_MAX_IDLE_DELAY: c_int = 10;
pub const AT_SUSPEND_LINK_TIMEOUT: c_int = 100;
pub const AT_ASPM_L0S_TIMER: c_int = 6;
pub const AT_ASPM_L1_TIMER: c_int = 12;
pub const AT_LCKDET_TIMER: c_int = 12;
pub const ATL1C_PCIE_L0S_L1_DISABLE: c_uint = 0x01;
pub const ATL1C_PCIE_PHY_RESET: c_uint = 0x02;
pub const ATL1C_ASPM_L0s_ENABLE: c_uint = 0x0001;
pub const ATL1C_ASPM_L1_ENABLE: c_uint = 0x0002;

pub const AT_EEPROM_LEN: c_int = 512;

// tpd word 1 bit 0:7 General Checksum task offload
pub const TPD_L4HDR_OFFSET_MASK: c_uint = 0x00FF;
pub const TPD_L4HDR_OFFSET_SHIFT: c_int = 0;
// tpd word 1 bit 0:7 Large Send task offload (IPv4/IPV6)
pub const TPD_TCPHDR_OFFSET_MASK: c_uint = 0x00FF;
pub const TPD_TCPHDR_OFFSET_SHIFT: c_int = 0;
// tpd word 1 bit 0:7 Custom Checksum task offload
pub const TPD_PLOADOFFSET_MASK: c_uint = 0x00FF;
pub const TPD_PLOADOFFSET_SHIFT: c_int = 0;
// tpd word 1 bit 8:17
pub const TPD_CCSUM_EN_MASK: c_uint = 0x0001;
pub const TPD_CCSUM_EN_SHIFT: c_int = 8;
pub const TPD_IP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_IP_CSUM_SHIFT: c_int = 9;
pub const TPD_TCP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_TCP_CSUM_SHIFT: c_int = 10;
pub const TPD_UDP_CSUM_MASK: c_uint = 0x0001;
pub const TPD_UDP_CSUM_SHIFT: c_int = 11;
pub const TPD_LSO_EN_MASK: c_uint = 0x0001	/* TCP Large Send Offload */;
pub const TPD_LSO_EN_SHIFT: c_int = 12;
pub const TPD_LSO_VER_MASK: c_uint = 0x0001;

pub const TPD_CON_VTAG_MASK: c_uint = 0x0001;
pub const TPD_CON_VTAG_SHIFT: c_int = 14;
pub const TPD_INS_VTAG_MASK: c_uint = 0x0001;
pub const TPD_INS_VTAG_SHIFT: c_int = 15;
pub const TPD_IPV4_PACKET_MASK: c_uint = 0x0001  /* valid when LSO VER  is 1 */;
pub const TPD_IPV4_PACKET_SHIFT: c_int = 16;
pub const TPD_ETH_TYPE_MASK: c_uint = 0x0001;

// tpd word 18:25 Custom Checksum task offload
pub const TPD_CCSUM_OFFSET_MASK: c_uint = 0x00FF;
pub const TPD_CCSUM_OFFSET_SHIFT: c_int = 18;
pub const TPD_CCSUM_EPAD_MASK: c_uint = 0x0001;
pub const TPD_CCSUM_EPAD_SHIFT: c_int = 30;
// tpd word 18:30 Large Send task offload (IPv4/IPV6)
pub const TPD_MSS_MASK: c_uint = 0x1FFF;
pub const TPD_MSS_SHIFT: c_int = 18;
pub const TPD_EOP_MASK: c_uint = 0x0001;
pub const TPD_EOP_SHIFT: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_tpd_desc {
    pub /: *mut *mut __le16 buffer_len; / include 4-byte CRC,
    pub vlan_tag: __le16,
    pub word1: __le32,
    pub buffer_addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_tpd_ext_desc {
    pub reservd_0: u32,
    pub word1: __le32,
    pub pkt_len: __le32,
    pub reservd_1: u32,
}

// rrs word 0 bit 0:31
pub const RRS_RX_CSUM_MASK: c_uint = 0xFFFF;
pub const RRS_RX_CSUM_SHIFT: c_int = 0;
pub const RRS_RX_RFD_CNT_MASK: c_uint = 0x000F;
pub const RRS_RX_RFD_CNT_SHIFT: c_int = 16;
pub const RRS_RX_RFD_INDEX_MASK: c_uint = 0x0FFF;
pub const RRS_RX_RFD_INDEX_SHIFT: c_int = 20;
// rrs flag bit 0:16
pub const RRS_HEAD_LEN_MASK: c_uint = 0x00FF;
pub const RRS_HEAD_LEN_SHIFT: c_int = 0;
pub const RRS_HDS_TYPE_MASK: c_uint = 0x0003;
pub const RRS_HDS_TYPE_SHIFT: c_int = 8;
pub const RRS_CPU_NUM_MASK: c_uint = 0x0003;
pub const RRS_CPU_NUM_SHIFT: c_int = 10;
pub const RRS_HASH_FLG_MASK: c_uint = 0x000F;
pub const RRS_HASH_FLG_SHIFT: c_int = 12;
pub const RRS_HDS_TYPE_HEAD: c_int = 1;
pub const RRS_HDS_TYPE_DATA: c_int = 2;

// rrs word 3 bit 0:31
pub const RRS_PKT_SIZE_MASK: c_uint = 0x3FFF;
pub const RRS_PKT_SIZE_SHIFT: c_int = 0;
pub const RRS_ERR_L4_CSUM_MASK: c_uint = 0x0001;
pub const RRS_ERR_L4_CSUM_SHIFT: c_int = 14;
pub const RRS_ERR_IP_CSUM_MASK: c_uint = 0x0001;
pub const RRS_ERR_IP_CSUM_SHIFT: c_int = 15;
pub const RRS_VLAN_INS_MASK: c_uint = 0x0001;
pub const RRS_VLAN_INS_SHIFT: c_int = 16;
pub const RRS_PROT_ID_MASK: c_uint = 0x0007;
pub const RRS_PROT_ID_SHIFT: c_int = 17;
pub const RRS_RX_ERR_SUM_MASK: c_uint = 0x0001;
pub const RRS_RX_ERR_SUM_SHIFT: c_int = 20;
pub const RRS_RX_ERR_CRC_MASK: c_uint = 0x0001;
pub const RRS_RX_ERR_CRC_SHIFT: c_int = 21;
pub const RRS_RX_ERR_FAE_MASK: c_uint = 0x0001;
pub const RRS_RX_ERR_FAE_SHIFT: c_int = 22;
pub const RRS_RX_ERR_TRUNC_MASK: c_uint = 0x0001;
pub const RRS_RX_ERR_TRUNC_SHIFT: c_int = 23;
pub const RRS_RX_ERR_RUNC_MASK: c_uint = 0x0001;
pub const RRS_RX_ERR_RUNC_SHIFT: c_int = 24;
pub const RRS_RX_ERR_ICMP_MASK: c_uint = 0x0001;
pub const RRS_RX_ERR_ICMP_SHIFT: c_int = 25;
pub const RRS_PACKET_BCAST_MASK: c_uint = 0x0001;
pub const RRS_PACKET_BCAST_SHIFT: c_int = 26;
pub const RRS_PACKET_MCAST_MASK: c_uint = 0x0001;
pub const RRS_PACKET_MCAST_SHIFT: c_int = 27;
pub const RRS_PACKET_TYPE_MASK: c_uint = 0x0001;
pub const RRS_PACKET_TYPE_SHIFT: c_int = 28;
pub const RRS_FIFO_FULL_MASK: c_uint = 0x0001;
pub const RRS_FIFO_FULL_SHIFT: c_int = 29;
pub const RRS_802_3_LEN_ERR_MASK: c_uint = 0x0001;
pub const RRS_802_3_LEN_ERR_SHIFT: c_int = 30;
pub const RRS_RXD_UPDATED_MASK: c_uint = 0x0001;
pub const RRS_RXD_UPDATED_SHIFT: c_int = 31;
pub const RRS_ERR_L4_CSUM: c_uint = 0x00004000;
pub const RRS_ERR_IP_CSUM: c_uint = 0x00008000;
pub const RRS_VLAN_INS: c_uint = 0x00010000;
pub const RRS_RX_ERR_SUM: c_uint = 0x00100000;
pub const RRS_RX_ERR_CRC: c_uint = 0x00200000;
pub const RRS_802_3_LEN_ERR: c_uint = 0x40000000;
pub const RRS_RXD_UPDATED: c_uint = 0x80000000;
pub const RRS_PACKET_TYPE_802_3: c_int = 1;
pub const RRS_PACKET_TYPE_ETH: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_recv_ret_status {
    pub word0: __le32,
    pub rss_hash: __le32,
    pub vlan_tag: __le16,
    pub flag: __le16,
    pub word3: __le32,
}

// RFD descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_rx_free_desc {
    pub buffer_addr: __le64,
}

// DMA Order Settings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1c_dma_order {
    atl1c_dma_ord_in = 1,
    atl1c_dma_ord_enh = 2,
    atl1c_dma_ord_out = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1c_dma_rcb {
    atl1c_rcb_64 = 0,
    atl1c_rcb_128 = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1c_mac_speed {
    atl1c_mac_speed_0 = 0,
    atl1c_mac_speed_10_100 = 1,
    atl1c_mac_speed_1000 = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1c_dma_req_block {
    atl1c_dma_req_128 = 0,
    atl1c_dma_req_256 = 1,
    atl1c_dma_req_512 = 2,
    atl1c_dma_req_1024 = 3,
    atl1c_dma_req_2048 = 4,
    atl1c_dma_req_4096 = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl1c_nic_type {
    athr_l1c = 0,
    athr_l2c = 1,
    athr_l2c_b,
    athr_l2c_b2,
    athr_l1d,
    athr_l1d_2,
    athr_mt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_hw_stats {
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
pub struct atl1c_hw {
    pub /: *mut *mut *mut u8 __iomem hw_addr; / inner register address,
    pub adapter: *mut atl1c_adapter,
    pub nic_type: atl1c_nic_type,
    pub dma_order: atl1c_dma_order,
    pub rcb_value: atl1c_dma_rcb,
    pub dmar_block: atl1c_dma_req_block,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
    pub phy_id1: u16,
    pub phy_id2: u16,
    pub /: *mut *mut spinlock_t intr_mask_lock; / protect the intr_mask,
    pub intr_mask: u32,
    pub preamble_len: u8,
    pub max_frame_size: u16,
    pub min_frame_size: u16,
    pub mac_speed: atl1c_mac_speed,
    pub mac_duplex: bool,
    pub hibernate: bool,
    pub media_type: u16,
pub const MEDIA_TYPE_AUTO_SENSOR: c_int = 0;
pub const MEDIA_TYPE_100M_FULL: c_int = 1;
pub const MEDIA_TYPE_100M_HALF: c_int = 2;
pub const MEDIA_TYPE_10M_FULL: c_int = 3;
pub const MEDIA_TYPE_10M_HALF: c_int = 4;
    pub autoneg_advertised: u16,
    pub mii_autoneg_adv_reg: u16,
    pub mii_1000t_ctrl_reg: u16,
    pub /: *mut *mut u16 tx_imt; / TX Interrupt Moderator timer ( 2us resolution),
    pub /: *mut *mut u16 rx_imt; / RX Interrupt Moderator timer ( 2us resolution),
    pub /: *mut *mut u16 ict; / Interrupt Clear timer (2us resolution),
    pub ctrl_flags: u16,
pub const ATL1C_INTR_CLEAR_ON_READ: c_uint = 0x0001;
pub const ATL1C_INTR_MODRT_ENABLE: c_uint = 0x0002;
pub const ATL1C_CMB_ENABLE: c_uint = 0x0004;
pub const ATL1C_SMB_ENABLE: c_uint = 0x0010;
pub const ATL1C_TXQ_MODE_ENHANCE: c_uint = 0x0020;
pub const ATL1C_RX_IPV6_CHKSUM: c_uint = 0x0040;
pub const ATL1C_ASPM_L0S_SUPPORT: c_uint = 0x0080;
pub const ATL1C_ASPM_L1_SUPPORT: c_uint = 0x0100;
pub const ATL1C_ASPM_CTRL_MON: c_uint = 0x0200;
pub const ATL1C_HIB_DISABLE: c_uint = 0x0400;
pub const ATL1C_APS_MODE_ENABLE: c_uint = 0x0800;
pub const ATL1C_LINK_EXT_SYNC: c_uint = 0x1000;
pub const ATL1C_CLK_GATING_EN: c_uint = 0x2000;
pub const ATL1C_FPGA_VERSION: c_uint = 0x8000;
    pub link_cap_flags: u16,
pub const ATL1C_LINK_CAP_1000M: c_uint = 0x0001;
    pub smb_timer: u32,
    pub trigger: *mut *mut u16 rrd_thresh; / Threshold of number of RRD produced to,
    pub tpd_thresh: u16,
    pub /: *mut *mut u8 tpd_burst; / Number of TPD to prefetch in cache-aligned burst.,
    pub rfd_burst: u8,
    pub base_cpu: u32,
    pub indirect_tab: u32,
    pub mac_addr: [u8; ETH_ALEN],
    pub perm_mac_addr: [u8; ETH_ALEN],
    pub phy_configured: bool,
    pub re_autoneg: bool,
    pub emi_ca: bool,
    pub /: *mut *mut bool msi_lnkpatch; / link patch for specific platforms,
}

//
// atl1c_ring_header represents a single, contiguous block of DMA space
// mapped for the three descriptor rings (tpd, rfd, rrd) described below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_ring_header {
    pub /: *mut *mut *mut void desc; / virtual address,
    pub address*/: *mut *mut dma_addr_t dma; / physical,
    pub /: *mut *mut unsigned int size; / length in bytes,
}

//
// atl1c_buffer is wrapper around a pointer to a socket buffer
// so a DMA handle can be stored along with the skb
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_buffer {
    pub /: *mut *mut *mut sk_buff skb; / socket buffer,
    pub /: *mut *mut u16 length; / rx buffer length,
    pub /: *mut *mut u16 flags; / information of buffer,
pub const ATL1C_BUFFER_FREE: c_uint = 0x0001;
pub const ATL1C_BUFFER_BUSY: c_uint = 0x0002;
pub const ATL1C_BUFFER_STATE_MASK: c_uint = 0x0003;
pub const ATL1C_PCIMAP_SINGLE: c_uint = 0x0004;
pub const ATL1C_PCIMAP_PAGE: c_uint = 0x0008;
pub const ATL1C_PCIMAP_TYPE_MASK: c_uint = 0x000C;
pub const ATL1C_PCIMAP_TODEVICE: c_uint = 0x0010;
pub const ATL1C_PCIMAP_FROMDEVICE: c_uint = 0x0020;
pub const ATL1C_PCIMAP_DIRECTION_MASK: c_uint = 0x0030;
    pub dma: dma_addr_t,
}

// transimit packet descriptor (tpd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_tpd_ring {
    pub adapter: *mut atl1c_adapter,
    pub /: *mut *mut *mut void desc; / descriptor ring virtual address,
    pub /: *mut *mut dma_addr_t dma; / descriptor ring physical address,
    pub num: u16,
    pub /: *mut *mut u16 size; / descriptor ring length in bytes,
    pub /: *mut *mut u16 count; / number of descriptors in the ring,
    pub next_to_use: u16,
    pub next_to_clean: core::sync::atomic::AtomicI32,
    pub buffer_info: *mut atl1c_buffer,
    pub napi: napi_struct,
}

// receive free descriptor (rfd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_rfd_ring {
    pub /: *mut *mut *mut void desc; / descriptor ring virtual address,
    pub /: *mut *mut dma_addr_t dma; / descriptor ring physical address,
    pub /: *mut *mut u16 size; / descriptor ring length in bytes,
    pub /: *mut *mut u16 count; / number of descriptors in the ring,
    pub next_to_use: u16,
    pub next_to_clean: u16,
    pub buffer_info: *mut atl1c_buffer,
}

// receive return descriptor (rrd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_rrd_ring {
    pub adapter: *mut atl1c_adapter,
    pub /: *mut *mut *mut void desc; / descriptor ring virtual address,
    pub /: *mut *mut dma_addr_t dma; / descriptor ring physical address,
    pub num: u16,
    pub /: *mut *mut u16 size; / descriptor ring length in bytes,
    pub /: *mut *mut u16 count; / number of descriptors in the ring,
    pub next_to_use: u16,
    pub next_to_clean: u16,
    pub napi: napi_struct,
}

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl1c_adapter {
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub hw: atl1c_hw,
    pub hw_stats: atl1c_hw_stats,
    pub /: *mut *mut mii_if_info mii; / MII interface info,
    pub rx_buffer_len: u16,
    pub tx_queue_count: c_uint,
    pub rx_queue_count: c_uint,
    pub flags: c_ulong,
pub const __AT_TESTING: c_uint = 0x0001;
pub const __AT_RESETTING: c_uint = 0x0002;
pub const __AT_DOWN: c_uint = 0x0003;
    pub work_event: c_ulong,
pub const ATL1C_WORK_EVENT_RESET: c_int = 0;
pub const ATL1C_WORK_EVENT_LINK_CHANGE: c_int = 1;
    pub msg_enable: u32,
    pub have_msi: bool,
    pub wol: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub mdio_lock: spinlock_t,
    pub irq_sem: core::sync::atomic::AtomicI32,
    pub common_task: work_struct,
    pub watchdog_timer: timer_list,
    pub phy_config_timer: timer_list,
// All Descriptor memory
    pub ring_header: atl1c_ring_header,
    pub tpd_ring: [atl1c_tpd_ring; AT_MAX_TRANSMIT_QUEUE],
    pub rfd_ring: [atl1c_rfd_ring; AT_MAX_RECEIVE_QUEUE],
    pub rrd_ring: [atl1c_rrd_ring; AT_MAX_RECEIVE_QUEUE],
    pub number;*/: *mut *mut u32 bd_number; / board,
}

// (u32 *)pdata = readl((a)->hw_addr + reg);	\

// (u16 *)pdata = readw((a)->hw_addr + reg);	\

extern "C" {
    pub fn atl1c_reinit_locked(adapter: *mut atl1c_adapter);
}
extern "C" {
    pub fn atl1c_reset_hw(hw: *mut atl1c_hw) -> i32;
}
extern "C" {
    pub fn atl1c_set_ethtool_ops(netdev: *mut net_device);
}
