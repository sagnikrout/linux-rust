//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbevf/defines.h
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
// Copyright(c) 1999 - 2024 Intel Corporation.
// Device IDs
pub const IXGBE_DEV_ID_82599_VF: c_uint = 0x10ED;
pub const IXGBE_DEV_ID_X540_VF: c_uint = 0x1515;
pub const IXGBE_DEV_ID_X550_VF: c_uint = 0x1565;
pub const IXGBE_DEV_ID_X550EM_X_VF: c_uint = 0x15A8;
pub const IXGBE_DEV_ID_X550EM_A_VF: c_uint = 0x15C5;
pub const IXGBE_DEV_ID_82599_VF_HV: c_uint = 0x152E;
pub const IXGBE_DEV_ID_X540_VF_HV: c_uint = 0x1530;
pub const IXGBE_DEV_ID_X550_VF_HV: c_uint = 0x1564;
pub const IXGBE_DEV_ID_X550EM_X_VF_HV: c_uint = 0x15A9;
pub const IXGBE_DEV_ID_E610_VF: c_uint = 0x57AD;
pub const IXGBE_SUBDEV_ID_E610_VF_HV: c_uint = 0x00FF;
pub const IXGBE_VF_IRQ_CLEAR_MASK: c_int = 7;
pub const IXGBE_VF_MAX_TX_QUEUES: c_int = 8;
pub const IXGBE_VF_MAX_RX_QUEUES: c_int = 8;
// DCB define
pub const IXGBE_VF_MAX_TRAFFIC_CLASS: c_int = 8;
// Link speed
pub type ixgbe_link_speed = u32;
pub const IXGBE_LINK_SPEED_UNKNOWN: c_int = 0;
pub const IXGBE_LINK_SPEED_1GB_FULL: c_uint = 0x0020;
pub const IXGBE_LINK_SPEED_10GB_FULL: c_uint = 0x0080;
pub const IXGBE_LINK_SPEED_100_FULL: c_uint = 0x0008;
pub const IXGBE_CTRL_RST: c_uint = 0x04000000 /* Reset (SW) */;
pub const IXGBE_RXDCTL_ENABLE: c_uint = 0x02000000 /* Enable specific Rx Queue */;
pub const IXGBE_TXDCTL_ENABLE: c_uint = 0x02000000 /* Enable specific Tx Queue */;
pub const IXGBE_LINKS_UP: c_uint = 0x40000000;
pub const IXGBE_LINKS_SPEED_82599: c_uint = 0x30000000;
pub const IXGBE_LINKS_SPEED_10G_82599: c_uint = 0x30000000;
pub const IXGBE_LINKS_SPEED_1G_82599: c_uint = 0x20000000;
pub const IXGBE_LINKS_SPEED_100_82599: c_uint = 0x10000000;
// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const IXGBE_REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const IXGBE_REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const IXGBE_REQ_TX_BUFFER_GRANULARITY: c_int = 1024;
// Interrupt Vector Allocation Registers
pub const IXGBE_IVAR_ALLOC_VAL: c_uint = 0x80 /* Interrupt Allocation valid */;

// Receive Config masks
pub const IXGBE_RXCTRL_RXEN: c_uint = 0x00000001  /* Enable Receiver */;
pub const IXGBE_RXCTRL_DMBYPS: c_uint = 0x00000002  /* Descriptor Monitor Bypass */;
pub const IXGBE_RXDCTL_ENABLE: c_uint = 0x02000000  /* Enable specific Rx Queue */;
pub const IXGBE_RXDCTL_VME: c_uint = 0x40000000  /* VLAN mode enable */;
pub const IXGBE_RXDCTL_RLPMLMASK: c_uint = 0x00003FFF  /* Only supported on the X540 */;
pub const IXGBE_RXDCTL_RLPML_EN: c_uint = 0x00008000;
// DCA Control

// PSRTYPE bit definitions
pub const IXGBE_PSRTYPE_TCPHDR: c_uint = 0x00000010;
pub const IXGBE_PSRTYPE_UDPHDR: c_uint = 0x00000020;
pub const IXGBE_PSRTYPE_IPV4HDR: c_uint = 0x00000100;
pub const IXGBE_PSRTYPE_IPV6HDR: c_uint = 0x00000200;
pub const IXGBE_PSRTYPE_L2HDR: c_uint = 0x00001000;
// SRRCTL bit definitions

pub const IXGBE_SRRCTL_RDMTS_SHIFT: c_int = 22;
pub const IXGBE_SRRCTL_RDMTS_MASK: c_uint = 0x01C00000;
pub const IXGBE_SRRCTL_DROP_EN: c_uint = 0x10000000;
pub const IXGBE_SRRCTL_BSIZEPKT_MASK: c_uint = 0x0000007F;
pub const IXGBE_SRRCTL_BSIZEHDR_MASK: c_uint = 0x00003F00;
pub const IXGBE_SRRCTL_DESCTYPE_LEGACY: c_uint = 0x00000000;
pub const IXGBE_SRRCTL_DESCTYPE_ADV_ONEBUF: c_uint = 0x02000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_SPLIT: c_uint = 0x04000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_REPLICATION_LARGE_PKT: c_uint = 0x08000000;
pub const IXGBE_SRRCTL_DESCTYPE_HDR_SPLIT_ALWAYS: c_uint = 0x0A000000;
pub const IXGBE_SRRCTL_DESCTYPE_MASK: c_uint = 0x0E000000;
// Receive Descriptor bit definitions
pub const IXGBE_RXD_STAT_DD: c_uint = 0x01    /* Descriptor Done */;
pub const IXGBE_RXD_STAT_EOP: c_uint = 0x02    /* End of Packet */;
pub const IXGBE_RXD_STAT_FLM: c_uint = 0x04    /* FDir Match */;
pub const IXGBE_RXD_STAT_VP: c_uint = 0x08    /* IEEE VLAN Packet */;
pub const IXGBE_RXDADV_NEXTP_MASK: c_uint = 0x000FFFF0 /* Next Descriptor Index */;
pub const IXGBE_RXDADV_NEXTP_SHIFT: c_uint = 0x00000004;
pub const IXGBE_RXD_STAT_UDPCS: c_uint = 0x10    /* UDP xsum calculated */;
pub const IXGBE_RXD_STAT_L4CS: c_uint = 0x20    /* L4 xsum calculated */;
pub const IXGBE_RXD_STAT_IPCS: c_uint = 0x40    /* IP xsum calculated */;
pub const IXGBE_RXD_STAT_PIF: c_uint = 0x80    /* passed in-exact filter */;
pub const IXGBE_RXD_STAT_CRCV: c_uint = 0x100   /* Speculative CRC Valid */;
pub const IXGBE_RXD_STAT_VEXT: c_uint = 0x200   /* 1st VLAN found */;
pub const IXGBE_RXD_STAT_UDPV: c_uint = 0x400   /* Valid UDP checksum */;
pub const IXGBE_RXD_STAT_DYNINT: c_uint = 0x800   /* Pkt caused INT via DYNINT */;
pub const IXGBE_RXD_STAT_TS: c_uint = 0x10000 /* Time Stamp */;
pub const IXGBE_RXD_STAT_SECP: c_uint = 0x20000 /* Security Processing */;
pub const IXGBE_RXD_STAT_LB: c_uint = 0x40000 /* Loopback Status */;
pub const IXGBE_RXD_STAT_ACK: c_uint = 0x8000  /* ACK Packet indication */;
pub const IXGBE_RXD_ERR_CE: c_uint = 0x01    /* CRC Error */;
pub const IXGBE_RXD_ERR_LE: c_uint = 0x02    /* Length Error */;
pub const IXGBE_RXD_ERR_PE: c_uint = 0x08    /* Packet Error */;
pub const IXGBE_RXD_ERR_OSE: c_uint = 0x10    /* Oversize Error */;
pub const IXGBE_RXD_ERR_USE: c_uint = 0x20    /* Undersize Error */;
pub const IXGBE_RXD_ERR_TCPE: c_uint = 0x40    /* TCP/UDP Checksum Error */;
pub const IXGBE_RXD_ERR_IPE: c_uint = 0x80    /* IP Checksum Error */;
pub const IXGBE_RXDADV_ERR_MASK: c_uint = 0xFFF00000 /* RDESC.ERRORS mask */;

pub const IXGBE_RXDADV_ERR_HBO: c_uint = 0x00800000 /*Header Buffer Overflow */;
pub const IXGBE_RXDADV_ERR_CE: c_uint = 0x01000000 /* CRC Error */;
pub const IXGBE_RXDADV_ERR_LE: c_uint = 0x02000000 /* Length Error */;
pub const IXGBE_RXDADV_ERR_PE: c_uint = 0x08000000 /* Packet Error */;
pub const IXGBE_RXDADV_ERR_OSE: c_uint = 0x10000000 /* Oversize Error */;
pub const IXGBE_RXDADV_ERR_USE: c_uint = 0x20000000 /* Undersize Error */;
pub const IXGBE_RXDADV_ERR_TCPE: c_uint = 0x40000000 /* TCP/UDP Checksum Error */;
pub const IXGBE_RXDADV_ERR_IPE: c_uint = 0x80000000 /* IP Checksum Error */;
pub const IXGBE_RXD_VLAN_ID_MASK: c_uint = 0x0FFF  /* VLAN ID is in lower 12 bits */;
pub const IXGBE_RXD_PRI_MASK: c_uint = 0xE000  /* Priority is in upper 3 bits */;
pub const IXGBE_RXD_PRI_SHIFT: c_int = 13;
pub const IXGBE_RXD_CFI_MASK: c_uint = 0x1000  /* CFI is bit 12 */;
pub const IXGBE_RXD_CFI_SHIFT: c_int = 12;

pub const IXGBE_RXDADV_STAT_MASK: c_uint = 0x000FFFFF /* Stat/NEXTP: bit 0-19 */;
pub const IXGBE_RXDADV_STAT_FCEOFS: c_uint = 0x00000040 /* FCoE EOF/SOF Stat */;
pub const IXGBE_RXDADV_STAT_FCSTAT: c_uint = 0x00000030 /* FCoE Pkt Stat */;
pub const IXGBE_RXDADV_STAT_FCSTAT_NOMTCH: c_uint = 0x00000000 /* 00: No Ctxt Match */;
pub const IXGBE_RXDADV_STAT_FCSTAT_NODDP: c_uint = 0x00000010 /* 01: Ctxt w/o DDP */;
pub const IXGBE_RXDADV_STAT_FCSTAT_FCPRSP: c_uint = 0x00000020 /* 10: Recv. FCP_RSP */;
pub const IXGBE_RXDADV_STAT_FCSTAT_DDP: c_uint = 0x00000030 /* 11: Ctxt w/ DDP */;
pub const IXGBE_RXDADV_STAT_SECP: c_uint = 0x00020000 /* IPsec/MACsec pkt found */;
pub const IXGBE_RXDADV_RSSTYPE_MASK: c_uint = 0x0000000F;
pub const IXGBE_RXDADV_PKTTYPE_MASK: c_uint = 0x0000FFF0;
pub const IXGBE_RXDADV_PKTTYPE_IPV4: c_uint = 0x00000010 /* IPv4 hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_IPV6: c_uint = 0x00000040 /* IPv6 hdr present */;
pub const IXGBE_RXDADV_PKTTYPE_IPSEC_ESP: c_uint = 0x00001000 /* IPSec ESP */;
pub const IXGBE_RXDADV_PKTTYPE_IPSEC_AH: c_uint = 0x00002000 /* IPSec AH */;
pub const IXGBE_RXDADV_PKTTYPE_MASK_EX: c_uint = 0x0001FFF0;
pub const IXGBE_RXDADV_HDRBUFLEN_MASK: c_uint = 0x00007FE0;
pub const IXGBE_RXDADV_RSCCNT_MASK: c_uint = 0x001E0000;
pub const IXGBE_RXDADV_RSCCNT_SHIFT: c_int = 17;
pub const IXGBE_RXDADV_HDRBUFLEN_SHIFT: c_int = 5;
pub const IXGBE_RXDADV_SPLITHEADER_EN: c_uint = 0x00001000;
pub const IXGBE_RXDADV_SPH: c_uint = 0x8000;
// RSS Hash results
pub const IXGBE_RXDADV_RSSTYPE_NONE: c_uint = 0x00000000;
pub const IXGBE_RXDADV_RSSTYPE_IPV4_TCP: c_uint = 0x00000001;
pub const IXGBE_RXDADV_RSSTYPE_IPV4: c_uint = 0x00000002;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_TCP: c_uint = 0x00000003;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_EX: c_uint = 0x00000004;
pub const IXGBE_RXDADV_RSSTYPE_IPV6: c_uint = 0x00000005;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_TCP_EX: c_uint = 0x00000006;
pub const IXGBE_RXDADV_RSSTYPE_IPV4_UDP: c_uint = 0x00000007;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_UDP: c_uint = 0x00000008;
pub const IXGBE_RXDADV_RSSTYPE_IPV6_UDP_EX: c_uint = 0x00000009;

pub const IXGBE_TXD_POPTS_IXSM: c_uint = 0x01       /* Insert IP checksum */;
pub const IXGBE_TXD_POPTS_TXSM: c_uint = 0x02       /* Insert TCP/UDP checksum */;
pub const IXGBE_TXD_CMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const IXGBE_TXD_CMD_IFCS: c_uint = 0x02000000 /* Insert FCS (Ethernet CRC) */;
pub const IXGBE_TXD_CMD_IC: c_uint = 0x04000000 /* Insert Checksum */;
pub const IXGBE_TXD_CMD_RS: c_uint = 0x08000000 /* Report Status */;
pub const IXGBE_TXD_CMD_DEXT: c_uint = 0x20000000 /* Descriptor ext (0 = legacy) */;
pub const IXGBE_TXD_CMD_VLE: c_uint = 0x40000000 /* Add VLAN tag */;
pub const IXGBE_TXD_STAT_DD: c_uint = 0x00000001 /* Descriptor Done */;

// Transmit Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_len: __le32,
    pub olinfo_status: __le32,
    pub read: },
    pub /: *mut *mut __le64 rsvd; / Reserved,
    pub nxtseq_seed: __le32,
    pub status: __le32,
    pub wb: },
}

// Receive Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union ixgbe_adv_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub read: },
    pub data: __le32,
    pub /: *mut *mut __le16 pkt_info; / RSS, Pkt type,
    pub /: *mut *mut __le16 hdr_info; / Splithdr, hdrlen,
    pub hs_rss: },
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

// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_adv_tx_context_desc {
    pub vlan_macip_lens: __le32,
    pub fceof_saidx: __le32,
    pub type_tucmd_mlhl: __le32,
    pub mss_l4len_idx: __le32,
}

// Adv Transmit Descriptor Config Masks
pub const IXGBE_ADVTXD_DTYP_MASK: c_uint = 0x00F00000 /* DTYP mask */;
pub const IXGBE_ADVTXD_DTYP_CTXT: c_uint = 0x00200000 /* Advanced Context Desc */;
pub const IXGBE_ADVTXD_DTYP_DATA: c_uint = 0x00300000 /* Advanced Data Descriptor */;

pub const IXGBE_ADVTXD_DCMD_TSE: c_uint = 0x80000000 /* TCP Seg enable */;

pub const IXGBE_ADVTXD_TUCMD_IPV4: c_uint = 0x00000400  /* IP Packet Type: 1=IPv4 */;
pub const IXGBE_ADVTXD_TUCMD_IPV6: c_uint = 0x00000000  /* IP Packet Type: 0=IPv6 */;
pub const IXGBE_ADVTXD_TUCMD_L4T_UDP: c_uint = 0x00000000  /* L4 Packet TYPE of UDP */;
pub const IXGBE_ADVTXD_TUCMD_L4T_TCP: c_uint = 0x00000800  /* L4 Packet TYPE of TCP */;
pub const IXGBE_ADVTXD_TUCMD_L4T_SCTP: c_uint = 0x00001000  /* L4 Packet TYPE of SCTP */;
pub const IXGBE_ADVTXD_TUCMD_IPSEC_TYPE_ESP: c_uint = 0x00002000 /* IPSec Type ESP */;
pub const IXGBE_ADVTXD_TUCMD_IPSEC_ENCRYPT_EN: c_uint = 0x00004000 /* ESP Encrypt Enable */;

pub const IXGBE_ADVTXD_CC: c_uint = 0x00000080 /* Check Context */;

pub const IXGBE_ADVTXD_POPTS_IPSEC: c_uint = 0x00000400 /* IPSec offload request */;

// Interrupt register bitmasks
pub const IXGBE_EITR_CNT_WDIS: c_uint = 0x80000000;
pub const IXGBE_MAX_EITR: c_uint = 0x00000FF8;
pub const IXGBE_MIN_EITR: c_int = 8;
// Error Codes

// Transmit Config masks
pub const IXGBE_TXDCTL_ENABLE: c_uint = 0x02000000 /* Ena specific Tx Queue */;
pub const IXGBE_TXDCTL_SWFLSH: c_uint = 0x04000000 /* Tx Desc. wr-bk flushing */;

