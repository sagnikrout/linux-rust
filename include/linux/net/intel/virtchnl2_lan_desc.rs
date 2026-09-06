//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/virtchnl2_lan_desc.h
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
// Copyright (C) 2023 Intel Corporation

// This is an interface definition file where existing enums and their values
// must remain unchanged over time, so we specify explicit values for all enums.
//
// Transmit descriptor ID flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_tx_desc_ids {
    VIRTCHNL2_TXDID_DATA				= BIT(0),
    VIRTCHNL2_TXDID_CTX				= BIT(1),
// TXDID bit 2 is reserved
// TXDID bit 3 is free for future use
// TXDID bit 4 is reserved
//
    VIRTCHNL2_TXDID_FLEX_TSO_CTX			= BIT(5),
// TXDID bit 6 is reserved
    VIRTCHNL2_TXDID_FLEX_L2TAG1_L2TAG2		= BIT(7),
// TXDID bits 8 and 9 are free for future use
// TXDID bit 10 is reserved
// TXDID bit 11 is free for future use
//
    VIRTCHNL2_TXDID_FLEX_FLOW_SCHED			= BIT(12),
// TXDID bits 13 and 14 are free for future use
    VIRTCHNL2_TXDID_DESC_DONE			= BIT(15),
}

// Receive descriptor IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_desc_ids {
    VIRTCHNL2_RXDID_1_32B_BASE	= 1,
// FLEX_SQ_NIC and FLEX_SPLITQ share desc ids because they can be
// differentiated based on queue model; e.g. single queue model can
// only use FLEX_SQ_NIC and split queue model can only use FLEX_SPLITQ
// for DID 2.
//
    VIRTCHNL2_RXDID_2_FLEX_SPLITQ	= 2,
    VIRTCHNL2_RXDID_2_FLEX_SQ_NIC	= VIRTCHNL2_RXDID_2_FLEX_SPLITQ,
// 3 through 6 are reserved
    VIRTCHNL2_RXDID_7_HW_RSVD	= 7,
// 8 through 15 are free
}

// Receive descriptor ID bitmasks

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_desc_id_bitmasks {
    VIRTCHNL2_RXDID_1_32B_BASE_M	= VIRTCHNL2_RXDID_M(1_32B_BASE),
    VIRTCHNL2_RXDID_2_FLEX_SPLITQ_M	= VIRTCHNL2_RXDID_M(2_FLEX_SPLITQ),
    VIRTCHNL2_RXDID_2_FLEX_SQ_NIC_M	= VIRTCHNL2_RXDID_M(2_FLEX_SQ_NIC),
    VIRTCHNL2_RXDID_7_HW_RSVD_M	= VIRTCHNL2_RXDID_M(7_HW_RSVD),
}

// For splitq virtchnl2_rx_flex_desc_adv_nic_3 desc members

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_RAW_CSUM_INV_S: c_int = 12;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_GEN_S: c_int = 14;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_BUFQ_ID_S: c_int = 15;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_RSC_S: c_int = 10;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_SPH_S: c_int = 11;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_FF1_S: c_int = 12;

pub const VIRTCHNL2_RX_FLEX_DESC_ADV_MISS_S: c_int = 15;

// Bitmasks for splitq virtchnl2_rx_flex_desc_adv_nic_3
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchl2_rx_flex_desc_adv_status_error_0_qw1_bits {
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_DD_M			= BIT(0),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_EOF_M		= BIT(1),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_HBO_M		= BIT(2),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_L3L4P_M		= BIT(3),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_IPE_M		= BIT(4),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_L4E_M		= BIT(5),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_EIPE_M		= BIT(6),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XSUM_EUDPE_M		= BIT(7),
}

// Bitmasks for splitq virtchnl2_rx_flex_desc_adv_nic_3
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_flex_desc_adv_status_error_0_qw0_bits {
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_LPBK_M		= BIT(0),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_IPV6EXADD_M		= BIT(1),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_RXE_M		= BIT(2),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_CRCP_M		= BIT(3),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_RSS_VALID_M		= BIT(4),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_L2TAG1P_M		= BIT(5),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XTRMD0_VALID_M	= BIT(6),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS0_XTRMD1_VALID_M	= BIT(7),
}

// Bitmasks for splitq virtchnl2_rx_flex_desc_adv_nic_3
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_flex_desc_adv_status_error_1_bits {
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_RSVD_M		= GENMASK(1, 0),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_ATRAEFAIL_M		= BIT(2),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_L2TAG2P_M		= BIT(3),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD2_VALID_M	= BIT(4),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD3_VALID_M	= BIT(5),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD4_VALID_M	= BIT(6),
    VIRTCHNL2_RX_FLEX_DESC_ADV_STATUS1_XTRMD5_VALID_M	= BIT(7),
}

// For singleq (flex) virtchnl2_rx_flex_desc fields
// For virtchnl2_rx_flex_desc.ptype_flex_flags0 member
//

// For virtchnl2_rx_flex_desc.pkt_len member

// Bitmasks for singleq (flex) virtchnl2_rx_flex_desc
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_flex_desc_status_error_0_bits {
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_DD_M			= BIT(0),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_EOF_M			= BIT(1),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_HBO_M			= BIT(2),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_L3L4P_M			= BIT(3),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_IPE_M		= BIT(4),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_L4E_M		= BIT(5),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_EIPE_M		= BIT(6),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_XSUM_EUDPE_M		= BIT(7),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_LPBK_M			= BIT(8),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_IPV6EXADD_M		= BIT(9),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_RXE_M			= BIT(10),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_CRCP_M			= BIT(11),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_RSS_VALID_M		= BIT(12),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_L2TAG1P_M		= BIT(13),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_XTRMD0_VALID_M		= BIT(14),
    VIRTCHNL2_RX_FLEX_DESC_STATUS0_XTRMD1_VALID_M		= BIT(15),
}

// Bitmasks for singleq (flex) virtchnl2_rx_flex_desc
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_flex_desc_status_error_1_bits {
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_CPM_M			= GENMASK(3, 0),
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_NAT_M			= BIT(4),
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_CRYPTO_M			= BIT(5),
// [10:6] reserved
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_L2TAG2P_M		= BIT(11),
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD2_VALID_M		= BIT(12),
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD3_VALID_M		= BIT(13),
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD4_VALID_M		= BIT(14),
    VIRTCHNL2_RX_FLEX_DESC_STATUS1_XTRMD5_VALID_M		= BIT(15),
}

// For virtchnl2_rx_flex_desc.ts_low member

// For singleq (non flex) virtchnl2_singleq_base_rx_desc legacy desc members

// Bitmasks for singleq (base) virtchnl2_rx_base_desc
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_base_desc_status_bits {
    VIRTCHNL2_RX_BASE_DESC_STATUS_DD_M		= BIT(0),
    VIRTCHNL2_RX_BASE_DESC_STATUS_EOF_M		= BIT(1),
    VIRTCHNL2_RX_BASE_DESC_STATUS_L2TAG1P_M		= BIT(2),
    VIRTCHNL2_RX_BASE_DESC_STATUS_L3L4P_M		= BIT(3),
    VIRTCHNL2_RX_BASE_DESC_STATUS_CRCP_M		= BIT(4),
    VIRTCHNL2_RX_BASE_DESC_STATUS_RSVD_M		= GENMASK(7, 5),
    VIRTCHNL2_RX_BASE_DESC_STATUS_EXT_UDP_0_M	= BIT(8),
    VIRTCHNL2_RX_BASE_DESC_STATUS_UMBCAST_M		= GENMASK(10, 9),
    VIRTCHNL2_RX_BASE_DESC_STATUS_FLM_M		= BIT(11),
    VIRTCHNL2_RX_BASE_DESC_STATUS_FLTSTAT_M		= GENMASK(13, 12),
    VIRTCHNL2_RX_BASE_DESC_STATUS_LPBK_M		= BIT(14),
    VIRTCHNL2_RX_BASE_DESC_STATUS_IPV6EXADD_M	= BIT(15),
    VIRTCHNL2_RX_BASE_DESC_STATUS_RSVD1_M		= GENMASK(17, 16),
    VIRTCHNL2_RX_BASE_DESC_STATUS_INT_UDP_0_M	= BIT(18),
}

// Bitmasks for singleq (base) virtchnl2_rx_base_desc
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl2_rx_base_desc_error_bits {
    VIRTCHNL2_RX_BASE_DESC_ERROR_RXE_M		= BIT(0),
    VIRTCHNL2_RX_BASE_DESC_ERROR_ATRAEFAIL_M	= BIT(1),
    VIRTCHNL2_RX_BASE_DESC_ERROR_HBO_M		= BIT(2),
    VIRTCHNL2_RX_BASE_DESC_ERROR_L3L4E_M		= GENMASK(5, 3),
    VIRTCHNL2_RX_BASE_DESC_ERROR_IPE_M		= BIT(3),
    VIRTCHNL2_RX_BASE_DESC_ERROR_L4E_M		= BIT(4),
    VIRTCHNL2_RX_BASE_DESC_ERROR_EIPE_M		= BIT(5),
    VIRTCHNL2_RX_BASE_DESC_ERROR_OVERSIZE_M		= BIT(6),
    VIRTCHNL2_RX_BASE_DESC_ERROR_PPRS_M		= BIT(7),
}

// Bitmasks for singleq (base) virtchnl2_rx_base_desc

//
// struct virtchnl2_splitq_rx_buf_desc - SplitQ RX buffer descriptor format
// @qword0: RX buffer struct.
// @qword0.buf_id: Buffer identifier.
// @qword0.rsvd0: Reserved.
// @qword0.rsvd1: Reserved.
// @pkt_addr: Packet buffer address.
// @hdr_addr: Header buffer address.
// @rsvd2: Rerserved.
//
// Receive Descriptors
// SplitQ buffer
// |                                       16|                   0|
// ----------------------------------------------------------------
// | RSV                                     | Buffer ID          |
// ----------------------------------------------------------------
// | Rx packet buffer address                                     |
// ----------------------------------------------------------------
// | Rx header buffer address                                     |
// ----------------------------------------------------------------
// | RSV                                                          |
// ----------------------------------------------------------------
// |                                                             0|
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl2_splitq_rx_buf_desc {
    pub buf_id: __le16,
    pub rsvd0: __le16,
    pub rsvd1: __le32,
    pub qword0: },
    pub pkt_addr: __le64,
    pub hdr_addr: __le64,
    pub rsvd2: __le64,
}

//
// struct virtchnl2_singleq_rx_buf_desc - SingleQ RX buffer descriptor format.
// @pkt_addr: Packet buffer address.
// @hdr_addr: Header buffer address.
// @rsvd1: Reserved.
// @rsvd2: Reserved.
//
// SingleQ buffer
// |                                                             0|
// ----------------------------------------------------------------
// | Rx packet buffer address                                     |
// ----------------------------------------------------------------
// | Rx header buffer address                                     |
// ----------------------------------------------------------------
// | RSV                                                          |
// ----------------------------------------------------------------
// | RSV                                                          |
// ----------------------------------------------------------------
// |                                                             0|
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl2_singleq_rx_buf_desc {
    pub pkt_addr: __le64,
    pub hdr_addr: __le64,
    pub rsvd1: __le64,
    pub rsvd2: __le64,
}

//
// struct virtchnl2_singleq_base_rx_desc - RX descriptor writeback format.
// @qword0: First quad word struct.
// @qword0.lo_dword: Lower dual word struct.
// @qword0.lo_dword.mirroring_status: Mirrored packet status.
// @qword0.lo_dword.l2tag1: Stripped L2 tag from the received packet.
// @qword0.hi_dword: High dual word union.
// @qword0.hi_dword.rss: RSS hash.
// @qword0.hi_dword.fd_id: Flow director filter id.
// @qword1: Second quad word struct.
// @qword1.status_error_ptype_len: Status/error/PTYPE/length.
// @qword2: Third quad word struct.
// @qword2.ext_status: Extended status.
// @qword2.rsvd: Reserved.
// @qword2.l2tag2_1: Extracted L2 tag 2 from the packet.
// @qword2.l2tag2_2: Reserved.
// @qword3: Fourth quad word struct.
// @qword3.reserved: Reserved.
// @qword3.fd_id: Flow director filter id.
//
// Profile ID 0x1, SingleQ, base writeback format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl2_singleq_base_rx_desc {
    pub mirroring_status: __le16,
    pub l2tag1: __le16,
    pub lo_dword: },
    pub rss: __le32,
    pub fd_id: __le32,
    pub hi_dword: },
    pub qword0: },
    pub status_error_ptype_len: __le64,
    pub qword1: },
    pub ext_status: __le16,
    pub rsvd: __le16,
    pub l2tag2_1: __le16,
    pub l2tag2_2: __le16,
    pub qword2: },
    pub reserved: __le32,
    pub fd_id: __le32,
    pub qword3: },
}

//
// struct virtchnl2_rx_flex_desc_nic - RX descriptor writeback format.
//
// @rxdid: Descriptor builder profile id.
// @mir_id_umb_cast: umb_cast=[7:6], mirror=[5:0]
// @ptype_flex_flags0: ff0=[15:10], ptype=[9:0]
// @pkt_len: Packet length, [15:14] are reserved.
// @hdr_len_sph_flex_flags1: ff1/ext=[15:12], sph=[11], header=[10:0].
// @status_error0: Status/Error section 0.
// @l2tag1: Stripped L2 tag from the received packet
// @rss_hash: RSS hash.
// @status_error1: Status/Error section 1.
// @flexi_flags2: Flexible flags section 2.
// @ts_low: Lower word of timestamp value.
// @l2tag2_1st: First L2TAG2.
// @l2tag2_2nd: Second L2TAG2.
// @flow_id: Flow id.
// @flex_ts: Timestamp and flexible flow id union.
// @flex_ts.ts_high: Timestamp higher word of the timestamp value.
// @flex_ts.flex.rsvd: Reserved.
// @flex_ts.flex.flow_id_ipv6: IPv6 flow id.
//
// Profile ID 0x2, SingleQ, flex writeback format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl2_rx_flex_desc_nic {
// Qword 0
    pub rxdid: u8,
    pub mir_id_umb_cast: u8,
    pub ptype_flex_flags0: __le16,
    pub pkt_len: __le16,
    pub hdr_len_sph_flex_flags1: __le16,
// Qword 1
    pub status_error0: __le16,
    pub l2tag1: __le16,
    pub rss_hash: __le32,
// Qword 2
    pub status_error1: __le16,
    pub flexi_flags2: u8,
    pub ts_low: u8,
    pub l2tag2_1st: __le16,
    pub l2tag2_2nd: __le16,
// Qword 3
    pub flow_id: __le32,
    pub rsvd: __le16,
    pub flow_id_ipv6: __le16,
    pub flex: },
    pub ts_high: __le32,
    pub flex_ts: },
}

//
// struct virtchnl2_rx_flex_desc_adv_nic_3 - RX descriptor writeback format.
// @rxdid_ucast: ucast=[7:6], rsvd=[5:4], profile_id=[3:0].
// @status_err0_qw0: Status/Error section 0 in quad word 0.
// @ptype_err_fflags0: ff0=[15:12], udp_len_err=[11], ip_hdr_err=[10],
// ptype=[9:0].
// @pktlen_gen_bufq_id: bufq_id=[15] only in splitq, gen=[14] only in splitq,
// plen=[13:0].
// @hdrlen_flags: miss_prepend=[15], trunc_mirr=[14], int_udp_0=[13],
// ext_udp0=[12], sph=[11] only in splitq, rsc=[10]
// only in splitq, header=[9:0].
// @status_err0_qw1: Status/Error section 0 in quad word 1.
// @status_err1: Status/Error section 1.
// @fflags1: Flexible flags section 1.
// @ts_low: Lower word of timestamp value.
// @buf_id: Buffer identifier. Only in splitq mode.
// @misc: Union.
// @misc.raw_cs: Raw checksum.
// @misc.l2tag1: Stripped L2 tag from the received packet
// @misc.rscseglen:
// @hash1: Lower bits of Rx hash value.
// @ff2_mirrid_hash2: Union.
// @ff2_mirrid_hash2.fflags2: Flexible flags section 2.
// @ff2_mirrid_hash2.mirrorid: Mirror id.
// @ff2_mirrid_hash2.rscseglen: RSC segment length.
// @hash3: Upper bits of Rx hash value.
// @l2tag2: Extracted L2 tag 2 from the packet.
// @fmd4: Flexible metadata container 4.
// @l2tag1: Stripped L2 tag from the received packet
// @fmd6: Flexible metadata container 6.
// @ts_high: Timestamp higher word of the timestamp value.
//
// Profile ID 0x2, SplitQ, flex writeback format
//
// Flex-field 0: BufferID
// Flex-field 1: Raw checksum/L2TAG1/RSC Seg Len (determined by HW)
// Flex-field 2: Hash[15:0]
// Flex-flags 2: Hash[23:16]
// Flex-field 3: L2TAG2
// Flex-field 5: L2TAG1
// Flex-field 7: Timestamp (upper 32 bits)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl2_rx_flex_desc_adv_nic_3 {
// Qword 0
    pub rxdid_ucast: u8,
    pub status_err0_qw0: u8,
    pub ptype_err_fflags0: __le16,
    pub pktlen_gen_bufq_id: __le16,
    pub hdrlen_flags: __le16,
// Qword 1
    pub status_err0_qw1: u8,
    pub status_err1: u8,
    pub fflags1: u8,
    pub ts_low: u8,
    pub buf_id: __le16,
    pub raw_cs: __le16,
    pub l2tag1: __le16,
    pub rscseglen: __le16,
    pub misc: },
// Qword 2
    pub hash1: __le16,
    pub fflags2: u8,
    pub mirrorid: u8,
    pub hash2: u8,
    pub ff2_mirrid_hash2: },
    pub hash3: u8,
    pub l2tag2: __le16,
    pub fmd4: __le16,
// Qword 3
    pub l2tag1: __le16,
    pub fmd6: __le16,
    pub ts_high: __le32,
}

// Common union for accessing descriptor format structs
#[repr(C)]
#[derive(Copy, Clone)]
pub union virtchnl2_rx_desc {
    pub base_wb: virtchnl2_singleq_base_rx_desc,
    pub flex_nic_wb: virtchnl2_rx_flex_desc_nic,
    pub flex_adv_nic_3_wb: virtchnl2_rx_flex_desc_adv_nic_3,
}
