//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_82575.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2007 - 2018 Intel Corporation.
extern "C" {
    pub fn igb_shutdown_serdes_link_82575(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_power_up_serdes_link_82575(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_power_down_phy_copper_82575(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_rx_fifo_flush_82575(hw: *mut e1000_hw);
}

pub const E1000_RAR_ENTRIES_82575: c_int = 16;
pub const E1000_RAR_ENTRIES_82576: c_int = 24;
pub const E1000_RAR_ENTRIES_82580: c_int = 24;
pub const E1000_RAR_ENTRIES_I350: c_int = 32;
pub const E1000_SW_SYNCH_MB: c_uint = 0x00000100;
pub const E1000_STAT_DEV_RST_SET: c_uint = 0x00100000;
pub const E1000_CTRL_DEV_RST: c_uint = 0x20000000;
// SRRCTL bit definitions

pub const E1000_SRRCTL_DESCTYPE_ADV_ONEBUF: c_uint = 0x02000000;
pub const E1000_SRRCTL_DESCTYPE_HDR_SPLIT_ALWAYS: c_uint = 0x0A000000;
pub const E1000_SRRCTL_DROP_EN: c_uint = 0x80000000;
pub const E1000_SRRCTL_TIMESTAMP: c_uint = 0x40000000;
pub const E1000_MRQC_ENABLE_RSS_MQ: c_uint = 0x00000002;
pub const E1000_MRQC_ENABLE_VMDQ: c_uint = 0x00000003;
pub const E1000_MRQC_RSS_FIELD_IPV4_UDP: c_uint = 0x00400000;
pub const E1000_MRQC_ENABLE_VMDQ_RSS_MQ: c_uint = 0x00000005;
pub const E1000_MRQC_RSS_FIELD_IPV6_UDP: c_uint = 0x00800000;
pub const E1000_MRQC_RSS_FIELD_IPV6_UDP_EX: c_uint = 0x01000000;

// Immediate Interrupt Rx (A.K.A. Low Latency Interrupt)
pub const E1000_IMIREXT_SIZE_BP: c_uint = 0x00001000  /* Packet size bypass */;
pub const E1000_IMIREXT_CTRL_BP: c_uint = 0x00080000  /* Bypass check of ctrl bits */;
// Receive Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_adv_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub read: },
    pub /: *mut *mut __le16 pkt_info; / RSS type, Packet type,
    pub /: *mut *mut __le16 hdr_info; / Split Head, buf len,
    pub lo_dword: },
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub /: *mut *mut __le16 length; / Packet length,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub upper: },
    pub /: *mut *mut } wb; / writeback,
}

pub const E1000_RSS_TYPE_NO_HASH: c_int = 0;
pub const E1000_RSS_TYPE_HASH_TCP_IPV4: c_int = 1;
pub const E1000_RSS_TYPE_HASH_IPV4: c_int = 2;
pub const E1000_RSS_TYPE_HASH_TCP_IPV6: c_int = 3;
pub const E1000_RSS_TYPE_HASH_IPV6_EX: c_int = 4;
pub const E1000_RSS_TYPE_HASH_IPV6: c_int = 5;
pub const E1000_RSS_TYPE_HASH_TCP_IPV6_EX: c_int = 6;
pub const E1000_RSS_TYPE_HASH_UDP_IPV4: c_int = 7;
pub const E1000_RSS_TYPE_HASH_UDP_IPV6: c_int = 8;
pub const E1000_RSS_TYPE_HASH_UDP_IPV6_EX: c_int = 9;

pub const E1000_RXDADV_HDRBUFLEN_MASK: c_uint = 0x7FE0;
pub const E1000_RXDADV_HDRBUFLEN_SHIFT: c_int = 5;
pub const E1000_RXDADV_STAT_TS: c_uint = 0x10000 /* Pkt was time stamped */;
pub const E1000_RXDADV_STAT_TSIP: c_uint = 0x08000 /* timestamp in packet */;
// Transmit Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_adv_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_len: __le32,
    pub olinfo_status: __le32,
    pub read: },
    pub /: *mut *mut __le64 rsvd; / Reserved,
    pub nxtseq_seed: __le32,
    pub status: __le32,
    pub wb: },
}

// Adv Transmit Descriptor Config Masks
pub const E1000_ADVTXD_MAC_TSTAMP: c_uint = 0x00080000 /* IEEE1588 Timestamp packet */;
pub const E1000_ADVTXD_DTYP_CTXT: c_uint = 0x00200000 /* Advanced Context Descriptor */;
pub const E1000_ADVTXD_DTYP_DATA: c_uint = 0x00300000 /* Advanced Data Descriptor */;
pub const E1000_ADVTXD_DCMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const E1000_ADVTXD_DCMD_IFCS: c_uint = 0x02000000 /* Insert FCS (Ethernet CRC) */;
pub const E1000_ADVTXD_DCMD_RS: c_uint = 0x08000000 /* Report Status */;
pub const E1000_ADVTXD_DCMD_DEXT: c_uint = 0x20000000 /* Descriptor extension (1=Adv) */;
pub const E1000_ADVTXD_DCMD_VLE: c_uint = 0x40000000 /* VLAN pkt enable */;
pub const E1000_ADVTXD_DCMD_TSE: c_uint = 0x80000000 /* TCP Seg enable */;

// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_adv_tx_context_desc {
    pub vlan_macip_lens: __le32,
    pub seqnum_seed: __le32,
    pub type_tucmd_mlhl: __le32,
    pub mss_l4len_idx: __le32,
}

pub const E1000_ADVTXD_TUCMD_L4T_UDP: c_uint = 0x00000000  /* L4 Packet TYPE of UDP */;
pub const E1000_ADVTXD_TUCMD_IPV4: c_uint = 0x00000400  /* IP Packet Type: 1=IPv4 */;
pub const E1000_ADVTXD_TUCMD_L4T_TCP: c_uint = 0x00000800  /* L4 Packet TYPE of TCP */;
pub const E1000_ADVTXD_TUCMD_L4T_SCTP: c_uint = 0x00001000 /* L4 packet TYPE of SCTP */;
// IPSec Encrypt Enable for ESP

// Adv ctxt IPSec SA IDX mask
// Adv ctxt IPSec ESP len mask
// Additional Transmit Descriptor Control definitions
pub const E1000_TXDCTL_QUEUE_ENABLE: c_uint = 0x02000000 /* Enable specific Tx Queue */;
// Tx Queue Arbitration Priority 0=low, 1=high
// Additional Receive Descriptor Control definitions
pub const E1000_RXDCTL_QUEUE_ENABLE: c_uint = 0x02000000 /* Enable specific Rx Queue */;
// Direct Cache Access (DCA) definitions
pub const E1000_DCA_CTRL_DCA_MODE_DISABLE: c_uint = 0x01 /* DCA Disable */;
pub const E1000_DCA_CTRL_DCA_MODE_CB2: c_uint = 0x02 /* DCA Mode CB2 */;
pub const E1000_DCA_RXCTRL_CPUID_MASK: c_uint = 0x0000001F /* Rx CPUID Mask */;

pub const E1000_DCA_TXCTRL_CPUID_MASK: c_uint = 0x0000001F /* Tx CPUID Mask */;

// Additional DCA related definitions, note change in position of CPUID
pub const E1000_DCA_TXCTRL_CPUID_MASK_82576: c_uint = 0xFF000000 /* Tx CPUID Mask */;
pub const E1000_DCA_RXCTRL_CPUID_MASK_82576: c_uint = 0xFF000000 /* Rx CPUID Mask */;

// ETQF register bit definitions

pub const E1000_ETQF_QUEUE_SHIFT: c_int = 16;
pub const E1000_ETQF_QUEUE_MASK: c_uint = 0x00070000;
pub const E1000_ETQF_ETYPE_MASK: c_uint = 0x0000FFFF;
// FTQF register bit definitions
pub const E1000_FTQF_VF_BP: c_uint = 0x00008000;
pub const E1000_FTQF_1588_TIME_STAMP: c_uint = 0x08000000;
pub const E1000_FTQF_MASK: c_uint = 0xF0000000;
pub const E1000_FTQF_MASK_PROTO_BP: c_uint = 0x10000000;
pub const E1000_FTQF_MASK_SOURCE_PORT_BP: c_uint = 0x80000000;
pub const E1000_NVM_APME_82575: c_uint = 0x0400;
pub const MAX_NUM_VFS: c_int = 8;
pub const E1000_DTXSWC_MAC_SPOOF_MASK: c_uint = 0x000000FF /* Per VF MAC spoof control */;
pub const E1000_DTXSWC_VLAN_SPOOF_MASK: c_uint = 0x0000FF00 /* Per VF VLAN spoof control */;
pub const E1000_DTXSWC_LLE_MASK: c_uint = 0x00FF0000 /* Per VF Local LB enables */;
pub const E1000_DTXSWC_VLAN_SPOOF_SHIFT: c_int = 8;

// Easy defines for setting default pool, would normally be left a zero
pub const E1000_VT_CTL_DEFAULT_POOL_SHIFT: c_int = 7;

// Other useful VMD_CTL register defines

// Per VM Offload register setup
pub const E1000_VMOLR_RLPML_MASK: c_uint = 0x00003FFF /* Long Packet Maximum Length mask */;
pub const E1000_VMOLR_LPE: c_uint = 0x00010000 /* Accept Long packet */;
pub const E1000_VMOLR_RSSE: c_uint = 0x00020000 /* Enable RSS */;
pub const E1000_VMOLR_AUPE: c_uint = 0x01000000 /* Accept untagged packets */;
pub const E1000_VMOLR_ROMPE: c_uint = 0x02000000 /* Accept overflow multicast */;
pub const E1000_VMOLR_ROPE: c_uint = 0x04000000 /* Accept overflow unicast */;
pub const E1000_VMOLR_BAM: c_uint = 0x08000000 /* Accept Broadcast packets */;
pub const E1000_VMOLR_MPME: c_uint = 0x10000000 /* Multicast promiscuous mode */;
pub const E1000_VMOLR_STRVLAN: c_uint = 0x40000000 /* Vlan stripping enable */;
pub const E1000_VMOLR_STRCRC: c_uint = 0x80000000 /* CRC stripping enable */;
pub const E1000_DVMOLR_HIDEVLAN: c_uint = 0x20000000 /* Hide vlan enable */;
pub const E1000_DVMOLR_STRVLAN: c_uint = 0x40000000 /* Vlan stripping enable */;
pub const E1000_DVMOLR_STRCRC: c_uint = 0x80000000 /* CRC stripping enable */;
pub const E1000_VLVF_ARRAY_SIZE: c_int = 32;
pub const E1000_VLVF_VLANID_MASK: c_uint = 0x00000FFF;
pub const E1000_VLVF_POOLSEL_SHIFT: c_int = 12;

pub const E1000_VLVF_LVLAN: c_uint = 0x00100000;
pub const E1000_VLVF_VLANID_ENABLE: c_uint = 0x80000000;
pub const E1000_VMVIR_VLANA_DEFAULT: c_uint = 0x40000000 /* Always use default VLAN */;
pub const E1000_VMVIR_VLANA_NEVER: c_uint = 0x80000000 /* Never insert VLAN tag */;
pub const E1000_IOVCTL: c_uint = 0x05BBC;
pub const E1000_IOVCTL_REUSE_VFQ: c_uint = 0x00000001;
pub const E1000_RPLOLR_STRVLAN: c_uint = 0x40000000;
pub const E1000_RPLOLR_STRCRC: c_uint = 0x80000000;
pub const E1000_DTXCTL_8023LL: c_uint = 0x0004;
pub const E1000_DTXCTL_VLAN_ADDED: c_uint = 0x0008;
pub const E1000_DTXCTL_OOS_ENABLE: c_uint = 0x0010;
pub const E1000_DTXCTL_MDP_EN: c_uint = 0x0020;
pub const E1000_DTXCTL_SPOOF_INT: c_uint = 0x0040;

pub const ALL_QUEUES: c_uint = 0xFFFF;
// RX packet buffer size defines
pub const E1000_RXPBS_SIZE_MASK_82576: c_uint = 0x0000007F;
extern "C" {
    pub fn igb_vmdq_set_anti_spoofing_pf(: *mut e1000_hw, _arg: bool, _arg: c_int);
}
extern "C" {
    pub fn igb_vmdq_set_loopback_pf(: *mut e1000_hw, _arg: bool);
}
extern "C" {
    pub fn igb_vmdq_set_replication_pf(: *mut e1000_hw, _arg: bool);
}
extern "C" {
    pub fn igb_rxpbs_adjust_82580(data: u32) -> u16;
}
extern "C" {
    pub fn igb_read_emi_reg(: *mut e1000_hw, addr: u16, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_set_eee_i350(: *mut e1000_hw, adv1G: bool, adv100M: bool) -> i32;
}
extern "C" {
    pub fn igb_set_eee_i354(: *mut e1000_hw, adv1G: bool, adv100M: bool) -> i32;
}
extern "C" {
    pub fn igb_get_eee_status_i354(hw: *mut e1000_hw, status: *mut bool) -> i32;
}
pub const E1000_I2C_THERMAL_SENSOR_ADDR: c_uint = 0xF8;
pub const E1000_EMC_INTERNAL_DATA: c_uint = 0x00;
pub const E1000_EMC_INTERNAL_THERM_LIMIT: c_uint = 0x20;
pub const E1000_EMC_DIODE1_DATA: c_uint = 0x01;
pub const E1000_EMC_DIODE1_THERM_LIMIT: c_uint = 0x19;
pub const E1000_EMC_DIODE2_DATA: c_uint = 0x23;
pub const E1000_EMC_DIODE2_THERM_LIMIT: c_uint = 0x1A;
pub const E1000_EMC_DIODE3_DATA: c_uint = 0x2A;
pub const E1000_EMC_DIODE3_THERM_LIMIT: c_uint = 0x30;
