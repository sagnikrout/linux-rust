//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_lan_tx_rx.h
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
// Copyright (c) 2018, Intel Corporation.
#[repr(C)]
#[derive(Copy, Clone)]
pub union ice_32byte_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
// bit 0 of hdr_addr is DD bit
    pub rsvd1: __le64,
    pub rsvd2: __le64,
    pub read: },
    pub mirroring_status: __le16,
    pub l2tag1: __le16,
    pub lo_dword: },
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le32 fd_id; / Flow Director filter ID,
    pub hi_dword: },
    pub qword0: },
// status/error/PTYPE/length
    pub status_error_len: __le64,
    pub qword1: },
    pub /: *mut *mut __le16 ext_status; / extended status,
    pub rsvd: __le16,
    pub l2tag2_1: __le16,
    pub l2tag2_2: __le16,
    pub qword2: },
    pub reserved: __le32,
    pub fd_id: __le32,
    pub qword3: },
    pub /: *mut *mut } wb; / writeback,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fltr_desc {
    pub qidx_compq_space_stat: __le64,
    pub dtype_cmd_vsi_fdid: __le64,
}

pub const ICE_FXD_FLTR_QW0_QINDEX_S: c_int = 0;

pub const ICE_FXD_FLTR_QW0_COMP_Q_S: c_int = 11;

pub const ICE_FXD_FLTR_QW0_COMP_Q_ZERO: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW0_COMP_REPORT_S: c_int = 12;

pub const ICE_FXD_FLTR_QW0_COMP_REPORT_SW_FAIL: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW0_COMP_REPORT_SW: c_uint = 0x2ULL;
pub const ICE_FXD_FLTR_QW0_FD_SPACE_S: c_int = 14;

pub const ICE_FXD_FLTR_QW0_FD_SPACE_GUAR_BEST: c_uint = 0x2ULL;
pub const ICE_FXD_FLTR_QW0_STAT_CNT_S: c_int = 16;

pub const ICE_FXD_FLTR_QW0_STAT_ENA_S: c_int = 29;

pub const ICE_FXD_FLTR_QW0_STAT_ENA_PKTS: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW0_EVICT_ENA_S: c_int = 31;

pub const ICE_FXD_FLTR_QW0_EVICT_ENA_FALSE: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW0_EVICT_ENA_TRUE: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW0_TO_Q_S: c_int = 32;

pub const ICE_FXD_FLTR_QW0_TO_Q_EQUALS_QINDEX: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW0_TO_Q_PRI_S: c_int = 35;

pub const ICE_FXD_FLTR_QW0_TO_Q_PRIO1: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW0_DPU_RECIPE_S: c_int = 38;

pub const ICE_FXD_FLTR_QW0_DPU_RECIPE_DFLT: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW0_DROP_S: c_int = 40;

pub const ICE_FXD_FLTR_QW0_DROP_NO: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW0_DROP_YES: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW0_FLEX_PRI_S: c_int = 41;

pub const ICE_FXD_FLTR_QW0_FLEX_PRI_NONE: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW0_FLEX_MDID_S: c_int = 44;

pub const ICE_FXD_FLTR_QW0_FLEX_MDID0: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW0_FLEX_VAL_S: c_int = 48;

pub const ICE_FXD_FLTR_QW0_FLEX_VAL0: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW1_DTYPE_S: c_int = 0;

pub const ICE_FXD_FLTR_QW1_PCMD_S: c_int = 4;

pub const ICE_FXD_FLTR_QW1_PCMD_ADD: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW1_PCMD_REMOVE: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW1_PROF_PRI_S: c_int = 5;

pub const ICE_FXD_FLTR_QW1_PROF_PRIO_ZERO: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW1_PROF_S: c_int = 8;

pub const ICE_FXD_FLTR_QW1_PROF_ZERO: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW1_FD_VSI_S: c_int = 14;

pub const ICE_FXD_FLTR_QW1_SWAP_S: c_int = 24;

pub const ICE_FXD_FLTR_QW1_SWAP_NOT_SET: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_QW1_SWAP_SET: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW1_FDID_PRI_S: c_int = 25;

pub const ICE_FXD_FLTR_QW1_FDID_PRI_ONE: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_QW1_FDID_PRI_THREE: c_uint = 0x3ULL;
pub const ICE_FXD_FLTR_QW1_FDID_MDID_S: c_int = 28;

pub const ICE_FXD_FLTR_QW1_FDID_MDID_FD: c_uint = 0x05ULL;
pub const ICE_FXD_FLTR_QW1_FDID_S: c_int = 32;

pub const ICE_FXD_FLTR_QW1_FDID_ZERO: c_uint = 0x0ULL;
// definition for FD filter programming status descriptor WB format
pub const ICE_FXD_FLTR_WB_QW1_DD_S: c_int = 0;

pub const ICE_FXD_FLTR_WB_QW1_DD_YES: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_WB_QW1_PROG_ID_S: c_int = 1;

pub const ICE_FXD_FLTR_WB_QW1_PROG_ADD: c_uint = 0x0ULL;
pub const ICE_FXD_FLTR_WB_QW1_PROG_DEL: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_WB_QW1_FAIL_S: c_int = 4;

pub const ICE_FXD_FLTR_WB_QW1_FAIL_YES: c_uint = 0x1ULL;
pub const ICE_FXD_FLTR_WB_QW1_FAIL_PROF_S: c_int = 5;

pub const ICE_FXD_FLTR_WB_QW1_FAIL_PROF_YES: c_uint = 0x1ULL;
// Rx Flex Descriptor
// This descriptor is used instead of the legacy version descriptor when
// ice_rlan_ctx.adv_desc is set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ice_32b_rx_flex_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
// bit 0 of hdr_addr is DD bit
    pub rsvd1: __le64,
    pub rsvd2: __le64,
    pub read: },
// Qword 0
    pub /: *mut *mut u8 rxdid; / descriptor builder profile ID,
    pub /: *mut *mut u8 mir_id_umb_cast; / mirror=[5:0], umb=[7:6],
    pub /: *mut *mut __le16 ptype_flex_flags0; / ptype=[9:0], ff0=[15:10],
    pub /: *mut *mut __le16 pkt_len; / [15:14] are reserved,
    pub /: *mut *mut __le16 hdr_len_sph_flex_flags1; / header=[10:0],
// sph=[11:11]
// ff1/ext=[15:12]
// Qword 1
    pub status_error0: __le16,
    pub l2tag1: __le16,
    pub flex_meta0: __le16,
    pub flex_meta1: __le16,
// Qword 2
    pub status_error1: __le16,
    pub flex_flags2: u8,
    pub time_stamp_low: u8,
    pub l2tag2_1st: __le16,
    pub l2tag2_2nd: __le16,
// Qword 3
    pub flex_meta2: __le16,
    pub flex_meta3: __le16,
    pub flex_meta4: __le16,
    pub flex_meta5: __le16,
    pub flex: },
    pub ts_high: __le32,
    pub flex_ts: },
    pub /: *mut *mut } wb; / writeback,
}

// Rx Flex Descriptor NIC Profile
// This descriptor corresponds to RxDID 2 which contains
// metadata fields for RSS, flow ID and timestamp info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_32b_rx_flex_desc_nic {
// Qword 0
    pub rxdid: u8,
    pub mir_id_umb_cast: u8,
    pub ptype_flexi_flags0: __le16,
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
    pub raw_csum: __le16,
    pub l2tag2_2nd: __le16,
// Qword 3
    pub flow_id: __le32,
    pub vlan_id: __le16,
    pub flow_id_ipv6: __le16,
    pub flex: },
    pub ts_high: __le32,
    pub flex_ts: },
}

// Rx Flex Descriptor NIC Profile
// RxDID Profile ID 6
// Flex-field 0: RSS hash lower 16-bits
// Flex-field 1: RSS hash upper 16-bits
// Flex-field 2: Flow ID lower 16-bits
// Flex-field 3: Source VSI
// Flex-field 4: reserved, VLAN ID taken from L2Tag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_32b_rx_flex_desc_nic_2 {
// Qword 0
    pub rxdid: u8,
    pub mir_id_umb_cast: u8,
    pub ptype_flexi_flags0: __le16,
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
    pub flow_id: __le16,
    pub src_vsi: __le16,
    pub rsvd: __le16,
    pub flow_id_ipv6: __le16,
    pub flex: },
    pub ts_high: __le32,
    pub flex_ts: },
}

// Receive Flex Descriptor profile IDs: There are a total
// of 64 profiles where profile IDs 0/1 are for legacy; and
// profiles 2-63 are flex profiles that can be programmed
// with a specific metadata (profile 7 reserved for HW)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rxdid {
    ICE_RXDID_LEGACY_0		= 0,
    ICE_RXDID_LEGACY_1		= 1,
    ICE_RXDID_FLEX_NIC		= 2,
    ICE_RXDID_FLEX_NIC_2		= 6,
    ICE_RXDID_HW			= 7,
    ICE_RXDID_LAST			= 63,
}

// Receive Flex Descriptor Rx opcode values
pub const ICE_RX_OPC_MDID: c_uint = 0x01;
// Receive Descriptor MDID values that access packet flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flex_mdid_pkt_flags {
    ICE_RX_MDID_PKT_FLAGS_15_0	= 20,
    ICE_RX_MDID_PKT_FLAGS_31_16,
    ICE_RX_MDID_PKT_FLAGS_47_32,
    ICE_RX_MDID_PKT_FLAGS_63_48,
}

// Receive Descriptor MDID values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flex_rx_mdid {
    ICE_RX_MDID_FLOW_ID_LOWER	= 5,
    ICE_RX_MDID_FLOW_ID_HIGH,
    ICE_RX_MDID_SRC_VSI		= 19,
    ICE_RX_MDID_HASH_LOW		= 56,
    ICE_RX_MDID_HASH_HIGH,
}

// Rx/Tx Flag64 packet flag bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_flg64_bits {
    ICE_FLG_PKT_DSI		= 0,
    ICE_FLG_EVLAN_x8100	= 14,
    ICE_FLG_EVLAN_x9100,
    ICE_FLG_VLAN_x8100,
    ICE_FLG_TNL_MAC		= 22,
    ICE_FLG_TNL_VLAN,
    ICE_FLG_PKT_FRG,
    ICE_FLG_FIN		= 32,
    ICE_FLG_SYN,
    ICE_FLG_RST,
    ICE_FLG_TNL0		= 38,
    ICE_FLG_TNL1,
    ICE_FLG_TNL2,
    ICE_FLG_UDP_GRE,
    ICE_FLG_RSVD		= 63
}

// for ice_32byte_rx_flex_desc.ptype_flexi_flags0 member

// for ice_32byte_rx_flex_desc.pkt_length member

// ice_32byte_rx_flex_desc::hdr_len_sph_flex_flags1

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rx_flex_desc_status_error_0_bits {
// Note: These are predefined bit offsets
    ICE_RX_FLEX_DESC_STATUS0_DD_S = 0,
    ICE_RX_FLEX_DESC_STATUS0_EOF_S,
    ICE_RX_FLEX_DESC_STATUS0_HBO_S,
    ICE_RX_FLEX_DESC_STATUS0_L3L4P_S,
    ICE_RX_FLEX_DESC_STATUS0_XSUM_IPE_S,
    ICE_RX_FLEX_DESC_STATUS0_XSUM_L4E_S,
    ICE_RX_FLEX_DESC_STATUS0_XSUM_EIPE_S,
    ICE_RX_FLEX_DESC_STATUS0_XSUM_EUDPE_S,
    ICE_RX_FLEX_DESC_STATUS0_LPBK_S,
    ICE_RX_FLEX_DESC_STATUS0_IPV6EXADD_S,
    ICE_RX_FLEX_DESC_STATUS0_RXE_S,
    ICE_RX_FLEX_DESC_STATUS0_CRCP_S,
    ICE_RX_FLEX_DESC_STATUS0_RSS_VALID_S,
    ICE_RX_FLEX_DESC_STATUS0_L2TAG1P_S,
    ICE_RX_FLEX_DESC_STATUS0_XTRMD0_VALID_S,
    ICE_RX_FLEX_DESC_STATUS0_XTRMD1_VALID_S,
    ICE_RX_FLEX_DESC_STATUS0_LAST /* this entry must be last!!! */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rx_flex_desc_status_error_1_bits {
// Note: These are predefined bit offsets
    ICE_RX_FLEX_DESC_STATUS1_NAT_S = 4,
// [10:5] reserved
    ICE_RX_FLEX_DESC_STATUS1_L2TAG2P_S = 11,
    ICE_RX_FLEX_DESC_STATUS1_LAST /* this entry must be last!!! */
}

pub const ICE_TX_CMPLTNQ_CTX_SIZE_DWORDS: c_int = 22;
pub const ICE_TX_DRBELL_Q_CTX_SIZE_DWORDS: c_int = 5;

// RLAN Rx queue context data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rlan_ctx {
    pub head: u16,
    pub cpuid: u8,
pub const ICE_RLAN_BASE_S: c_int = 7;
    pub base: u64,
    pub qlen: u16,
pub const ICE_RLAN_CTX_DBUF_S: c_int = 7;
    pub dbuf: u8,
pub const ICE_RLAN_CTX_HBUF_S: c_int = 6;
    pub hbuf: u8,
    pub dtype: u8,
    pub dsize: u8,
    pub crcstrip: u8,
    pub l2tsel: u8,
    pub hsplit_0: u8,
    pub hsplit_1: u8,
    pub showiv: u8,
    pub rxmax: u16,
    pub tphrdesc_ena: u8,
    pub tphwdesc_ena: u8,
    pub tphdata_ena: u8,
    pub tphhead_ena: u8,
    pub lrxqthresh: u8,
    pub /: *mut *mut u8 prefena; / NOTE: normally must be set to 1 at init,
}

// for hsplit_0 field of Rx RLAN context
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rlan_ctx_rx_hsplit_0 {
    ICE_RLAN_RX_HSPLIT_0_NO_SPLIT		= 0,
    ICE_RLAN_RX_HSPLIT_0_SPLIT_L2		= 1,
    ICE_RLAN_RX_HSPLIT_0_SPLIT_IP		= 2,
    ICE_RLAN_RX_HSPLIT_0_SPLIT_TCP_UDP	= 4,
    ICE_RLAN_RX_HSPLIT_0_SPLIT_SCTP		= 8,
}

// for hsplit_1 field of Rx RLAN context
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rlan_ctx_rx_hsplit_1 {
    ICE_RLAN_RX_HSPLIT_1_NO_SPLIT		= 0,
    ICE_RLAN_RX_HSPLIT_1_SPLIT_L2		= 1,
    ICE_RLAN_RX_HSPLIT_1_SPLIT_ALWAYS	= 2,
}

// Tx Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tx_desc {
    pub /: *mut *mut __le64 buf_addr; / Address of descriptor's data buf,
    pub cmd_type_offset_bsz: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_desc_dtype_value {
    ICE_TX_DESC_DTYPE_DATA		= 0x0,
    ICE_TX_DESC_DTYPE_CTX		= 0x1,
    ICE_TX_DESC_DTYPE_FLTR_PROG	= 0x8,
// DESC_DONE - HW has completed write-back of descriptor
    ICE_TX_DESC_DTYPE_DESC_DONE	= 0xF,
}

pub const ICE_TXD_QW1_CMD_S: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_desc_cmd_bits {
    ICE_TX_DESC_CMD_EOP			= 0x0001,
    ICE_TX_DESC_CMD_RS			= 0x0002,
    ICE_TX_DESC_CMD_IL2TAG1			= 0x0008,
    ICE_TX_DESC_CMD_DUMMY			= 0x0010,
    ICE_TX_DESC_CMD_IIPT_IPV6		= 0x0020,
    ICE_TX_DESC_CMD_IIPT_IPV4		= 0x0040,
    ICE_TX_DESC_CMD_IIPT_IPV4_CSUM		= 0x0060,
    ICE_TX_DESC_CMD_L4T_EOFT_TCP		= 0x0100,
    ICE_TX_DESC_CMD_L4T_EOFT_SCTP		= 0x0200,
    ICE_TX_DESC_CMD_L4T_EOFT_UDP		= 0x0300,
    ICE_TX_DESC_CMD_RE			= 0x0400,
}

pub const ICE_TXD_QW1_OFFSET_S: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_desc_len_fields {
// Note: These are predefined bit offsets
    ICE_TX_DESC_LEN_MACLEN_S	= 0, /* 7 BITS */
    ICE_TX_DESC_LEN_IPLEN_S	= 7, /* 7 BITS */
    ICE_TX_DESC_LEN_L4_LEN_S	= 14 /* 4 BITS */
}

// Tx descriptor field limits in bytes

pub const ICE_TXD_QW1_TX_BUF_SZ_S: c_int = 34;
pub const ICE_TXD_QW1_L2TAG1_S: c_int = 48;
// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tx_ctx_desc {
    pub tunneling_params: __le32,
    pub l2tag2: __le16,
    pub gcs: __le16,
    pub qw1: __le64,
}

pub const ICE_TX_GCS_DESC_CSUM_PSH: c_int = 1;
pub const ICE_TXD_CTX_QW1_CMD_S: c_int = 4;

pub const ICE_TXD_CTX_QW1_TSO_LEN_S: c_int = 30;

pub const ICE_TXD_CTX_QW1_MSS_S: c_int = 50;
pub const ICE_TXD_CTX_MIN_MSS: c_int = 64;
pub const ICE_TXD_CTX_QW1_VSI_S: c_int = 50;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_ctx_desc_cmd_bits {
    ICE_TX_CTX_DESC_TSO		= 0x01,
    ICE_TX_CTX_DESC_TSYN		= 0x02,
    ICE_TX_CTX_DESC_IL2TAG2		= 0x04,
    ICE_TX_CTX_DESC_IL2TAG2_IL2H	= 0x08,
    ICE_TX_CTX_DESC_SWTCH_NOTAG	= 0x00,
    ICE_TX_CTX_DESC_SWTCH_UPLINK	= 0x10,
    ICE_TX_CTX_DESC_SWTCH_LOCAL	= 0x20,
    ICE_TX_CTX_DESC_SWTCH_VSI	= 0x30,
    ICE_TX_CTX_DESC_RESERVED	= 0x40
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_ctx_desc_eipt_offload {
    ICE_TX_CTX_EIPT_NONE		= 0x0,
    ICE_TX_CTX_EIPT_IPV6		= 0x1,
    ICE_TX_CTX_EIPT_IPV4_NO_CSUM	= 0x2,
    ICE_TX_CTX_EIPT_IPV4		= 0x3
}

pub const ICE_TXD_CTX_QW0_EIPLEN_S: c_int = 2;
pub const ICE_TXD_CTX_QW0_L4TUNT_S: c_int = 9;

pub const ICE_TXD_CTX_QW0_NATLEN_S: c_int = 12;
pub const ICE_TXD_CTX_QW0_L4T_CS_S: c_int = 23;

pub const ICE_LAN_TXQ_MAX_QGRPS: c_int = 127;
pub const ICE_LAN_TXQ_MAX_QDIS: c_int = 1023;
// Tx queue context data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tlan_ctx {
pub const ICE_TLAN_CTX_BASE_S: c_int = 7;
    pub /: *mut *mut u64 base; / base is defined in 128-byte units,
    pub port_num: u8,
    pub cgd_num: u8,
    pub pf_num: u8,
    pub vmvf_num: u16,
    pub vmvf_type: u8,
pub const ICE_TLAN_CTX_VMVF_TYPE_VF: c_int = 0;
pub const ICE_TLAN_CTX_VMVF_TYPE_VMQ: c_int = 1;
pub const ICE_TLAN_CTX_VMVF_TYPE_PF: c_int = 2;
    pub src_vsi: u16,
    pub tsyn_ena: u8,
    pub internal_usage_flag: u8,
    pub alt_vlan: u8,
    pub cpuid: u8,
    pub wb_mode: u8,
    pub tphrd_desc: u8,
    pub tphrd: u8,
    pub tphwr_desc: u8,
    pub cmpq_id: u16,
    pub qnum_in_func: u16,
    pub itr_notification_mode: u8,
    pub adjust_prof_id: u8,
    pub qlen: u16,
    pub quanta_prof_idx: u8,
    pub tso_ena: u8,
    pub tso_qnum: u16,
    pub legacy_int: u8,
    pub drop_ena: u8,
    pub cache_prof_idx: u8,
    pub pkt_shaper_prof_idx: u8,
}

// Tx time stamp descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ts_desc {
    pub tx_desc_idx_tstamp: __le32,
}

pub const ICE_TXTIME_MAX_QUEUE: c_int = 2047;
pub const ICE_SET_TXTIME_MAX_Q_AMOUNT: c_int = 127;
pub const ICE_TXTIME_FETCH_TS_DESC_DFLT: c_int = 8;
pub const ICE_TXTIME_FETCH_PROFILE_CNT: c_int = 16;
// Tx Time queue context data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_txtime_ctx {
pub const ICE_TXTIME_CTX_BASE_S: c_int = 7;
    pub /: *mut *mut u64 base; / base is defined in 128-byte units,
    pub pf_num: u8,
    pub vmvf_num: u16,
    pub vmvf_type: u8,
    pub src_vsi: u16,
    pub cpuid: u8,
    pub tphrd_desc: u8,
    pub qlen: u16,
    pub timer_num: u8,
    pub txtime_ena_q: u8,
    pub drbell_mode_32: u8,
pub const ICE_TXTIME_CTX_DRBELL_MODE_32: c_int = 1;
    pub ts_res: u8,
pub const ICE_TXTIME_CTX_RESOLUTION_128NS: c_int = 7;
    pub ts_round_type: u8,
    pub ts_pacing_slot: u8,
pub const ICE_TXTIME_CTX_FETCH_PROF_ID_0: c_int = 0;
    pub merging_ena: u8,
    pub ts_fetch_prof_id: u8,
    pub ts_fetch_cache_line_aln_thld: u8,
    pub tx_pipe_delay_mode: u8,
}
