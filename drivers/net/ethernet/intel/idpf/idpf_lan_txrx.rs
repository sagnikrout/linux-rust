//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/idpf_lan_txrx.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_rss_hash {
    IDPF_HASH_INVALID			= 0,
// Values 1 - 28 are reserved for future use
    IDPF_HASH_NONF_UNICAST_IPV4_UDP		= 29,
    IDPF_HASH_NONF_MULTICAST_IPV4_UDP,
    IDPF_HASH_NONF_IPV4_UDP,
    IDPF_HASH_NONF_IPV4_TCP_SYN_NO_ACK,
    IDPF_HASH_NONF_IPV4_TCP,
    IDPF_HASH_NONF_IPV4_SCTP,
    IDPF_HASH_NONF_IPV4_OTHER,
    IDPF_HASH_FRAG_IPV4,
// Values 37-38 are reserved
    IDPF_HASH_NONF_UNICAST_IPV6_UDP		= 39,
    IDPF_HASH_NONF_MULTICAST_IPV6_UDP,
    IDPF_HASH_NONF_IPV6_UDP,
    IDPF_HASH_NONF_IPV6_TCP_SYN_NO_ACK,
    IDPF_HASH_NONF_IPV6_TCP,
    IDPF_HASH_NONF_IPV6_SCTP,
    IDPF_HASH_NONF_IPV6_OTHER,
    IDPF_HASH_FRAG_IPV6,
    IDPF_HASH_NONF_RSVD47,
    IDPF_HASH_NONF_FCOE_OX,
    IDPF_HASH_NONF_FCOE_RX,
    IDPF_HASH_NONF_FCOE_OTHER,
// Values 51-62 are reserved
    IDPF_HASH_L2_PAYLOAD			= 63,

    IDPF_HASH_MAX
}

// Supported RSS offloads

// For idpf_splitq_base_tx_compl_desc
pub const IDPF_TXD_COMPLQ_GEN_S: c_int = 15;

pub const IDPF_TXD_COMPLQ_COMPL_TYPE_S: c_int = 11;

pub const IDPF_TXD_COMPLQ_QID_S: c_int = 0;

// For base mode TX descriptors
pub const IDPF_TXD_CTX_QW0_TUNN_L4T_CS_S: c_int = 23;

pub const IDPF_TXD_CTX_QW0_TUNN_DECTTL_S: c_int = 19;

pub const IDPF_TXD_CTX_QW0_TUNN_NATLEN_S: c_int = 12;

pub const IDPF_TXD_CTX_QW0_TUNN_EIP_NOINC_S: c_int = 11;

pub const IDPF_TXD_CTX_QW0_TUNN_NATT_S: c_int = 9;

pub const IDPF_TXD_CTX_QW0_TUNN_EXT_IPLEN_S: c_int = 2;

pub const IDPF_TXD_CTX_QW0_TUNN_EXT_IP_S: c_int = 0;

pub const IDPF_TXD_CTX_QW1_MSS_S: c_int = 50;

pub const IDPF_TXD_CTX_QW1_TSO_LEN_S: c_int = 30;

pub const IDPF_TXD_CTX_QW1_CMD_S: c_int = 4;

pub const IDPF_TXD_CTX_QW1_DTYPE_S: c_int = 0;

pub const IDPF_TXD_QW1_L2TAG1_S: c_int = 48;

pub const IDPF_TXD_QW1_TX_BUF_SZ_S: c_int = 34;

pub const IDPF_TXD_QW1_OFFSET_S: c_int = 16;

pub const IDPF_TXD_QW1_CMD_S: c_int = 4;

pub const IDPF_TXD_QW1_DTYPE_S: c_int = 0;

// TX Completion Descriptor Completion Types
pub const IDPF_TXD_COMPLT_ITR_FLUSH: c_int = 0;
// Descriptor completion type 1 is reserved
pub const IDPF_TXD_COMPLT_RS: c_int = 2;
// Descriptor completion type 3 is reserved
pub const IDPF_TXD_COMPLT_RE: c_int = 4;
pub const IDPF_TXD_COMPLT_SW_MARKER: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tx_desc_dtype_value {
    IDPF_TX_DESC_DTYPE_DATA				= 0,
    IDPF_TX_DESC_DTYPE_CTX				= 1,
// DTYPE 2 is reserved
// DTYPE 3 is free for future use
// DTYPE 4 is reserved
//
    IDPF_TX_DESC_DTYPE_FLEX_TSO_CTX			= 5,
// DTYPE 6 is reserved
    IDPF_TX_DESC_DTYPE_FLEX_L2TAG1_L2TAG2		= 7,
// DTYPE 8, 9 are free for future use
// DTYPE 10 is reserved
// DTYPE 11 is free for future use
//
    IDPF_TX_DESC_DTYPE_FLEX_FLOW_SCHE		= 12,
// DTYPE 13, 14 are free for future use

// DESC_DONE - HW has completed write-back of descriptor
    IDPF_TX_DESC_DTYPE_DESC_DONE			= 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tx_ctx_desc_cmd_bits {
    IDPF_TX_CTX_DESC_TSO		= 0x01,
    IDPF_TX_CTX_DESC_TSYN		= 0x02,
    IDPF_TX_CTX_DESC_IL2TAG2	= 0x04,
    IDPF_TX_CTX_DESC_RSVD		= 0x08,
    IDPF_TX_CTX_DESC_SWTCH_NOTAG	= 0x00,
    IDPF_TX_CTX_DESC_SWTCH_UPLINK	= 0x10,
    IDPF_TX_CTX_DESC_SWTCH_LOCAL	= 0x20,
    IDPF_TX_CTX_DESC_SWTCH_VSI	= 0x30,
    IDPF_TX_CTX_DESC_FILT_AU_EN	= 0x40,
    IDPF_TX_CTX_DESC_FILT_AU_EVICT	= 0x80,
    IDPF_TX_CTX_DESC_RSVD1		= 0xF00
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tx_desc_len_fields {
// Note: These are predefined bit offsets
    IDPF_TX_DESC_LEN_MACLEN_S	= 0, /* 7 BITS */
    IDPF_TX_DESC_LEN_IPLEN_S	= 7, /* 7 BITS */
    IDPF_TX_DESC_LEN_L4_LEN_S	= 14 /* 4 BITS */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tx_base_desc_cmd_bits {
    IDPF_TX_DESC_CMD_EOP			= BIT(0),
    IDPF_TX_DESC_CMD_RS			= BIT(1),
// only on VFs else RSVD
    IDPF_TX_DESC_CMD_ICRC			= BIT(2),
    IDPF_TX_DESC_CMD_IL2TAG1		= BIT(3),
    IDPF_TX_DESC_CMD_RSVD1			= BIT(4),
    IDPF_TX_DESC_CMD_IIPT_IPV6		= BIT(5),
    IDPF_TX_DESC_CMD_IIPT_IPV4		= BIT(6),
    IDPF_TX_DESC_CMD_IIPT_IPV4_CSUM		= GENMASK(6, 5),
    IDPF_TX_DESC_CMD_RSVD2			= BIT(7),
    IDPF_TX_DESC_CMD_L4T_EOFT_TCP		= BIT(8),
    IDPF_TX_DESC_CMD_L4T_EOFT_SCTP		= BIT(9),
    IDPF_TX_DESC_CMD_L4T_EOFT_UDP		= GENMASK(9, 8),
    IDPF_TX_DESC_CMD_RSVD3			= BIT(10),
    IDPF_TX_DESC_CMD_RSVD4			= BIT(11),
}

// Transmit descriptors
// splitq tx buf, singleq tx buf and singleq compl desc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_base_tx_desc {
    pub /: *mut *mut __le64 buf_addr; / Address of descriptor's data buf,
    pub /: *mut *mut __le64 qw1; / type_cmd_offset_bsz_l2tag1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_splitq_4b_tx_compl_desc {
// qid=[10:0] comptype=[13:11] rsvd=[14] gen=[15]
    pub qid_comptype_gen: __le16,
    pub /: *mut *mut __le16 q_head; / Queue head,
    pub /: *mut *mut __le16 compl_tag; / Completion tag,
    pub q_head_compl_tag: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_splitq_tx_compl_desc {
    pub common: idpf_splitq_4b_tx_compl_desc,
    pub ts: [u8; 3],
    pub /: *mut *mut u8 rsvd; / Reserved,
}

// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_base_tx_ctx_desc {
    pub tunneling_params: __le32,
    pub l2tag2: __le16,
    pub rsvd1: __le16,
    pub qw0: },
    pub /: *mut *mut __le64 qw1; / type_cmd_tlen_mss/rt_hint,
}

// Common cmd field defines for all desc except Flex Flow Scheduler (0x0C)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tx_flex_desc_cmd_bits {
    IDPF_TX_FLEX_DESC_CMD_EOP			= BIT(0),
    IDPF_TX_FLEX_DESC_CMD_RS			= BIT(1),
    IDPF_TX_FLEX_DESC_CMD_RE			= BIT(2),
    IDPF_TX_FLEX_DESC_CMD_IL2TAG1			= BIT(3),
    IDPF_TX_FLEX_DESC_CMD_DUMMY			= BIT(4),
    IDPF_TX_FLEX_DESC_CMD_CS_EN			= BIT(5),
    IDPF_TX_FLEX_DESC_CMD_FILT_AU_EN		= BIT(6),
    IDPF_TX_FLEX_DESC_CMD_FILT_AU_EVICT		= BIT(7),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_flex_tx_desc {
    pub /: *mut *mut __le64 buf_addr; / Packet buffer address,
pub const IDPF_FLEX_TXD_QW1_DTYPE_S: c_int = 0;

pub const IDPF_FLEX_TXD_QW1_CMD_S: c_int = 5;

    pub cmd_dtype: __le16,
// DTYPE=IDPF_TX_DESC_DTYPE_FLEX_L2TAG1_L2TAG2 (0x07)
    pub l2tag1: __le16,
    pub l2tag2: __le16,
    pub l2tags: },
    pub buf_size: __le16,
    pub qw1: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_flex_tx_sched_desc {
    pub /: *mut *mut __le64 buf_addr; / Packet buffer address,
// DTYPE = IDPF_TX_DESC_DTYPE_FLEX_FLOW_SCHE_16B (0x0C)
    pub cmd_dtype: u8,

// [23:23] Horizon Overflow bit, [22:0] timestamp
    pub ts: [u8; 3],
    pub compl_tag: __le16,
    pub rxr_bufsize: __le16,

    pub qw1: },
}

// Common cmd fields for all flex context descriptors
// Note: these defines already account for the 5 bit dtype in the cmd_dtype
// field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_tx_flex_ctx_desc_cmd_bits {
    IDPF_TX_FLEX_CTX_DESC_CMD_TSO			= BIT(5),
    IDPF_TX_FLEX_CTX_DESC_CMD_TSYN_EN		= BIT(6),
    IDPF_TX_FLEX_CTX_DESC_CMD_L2TAG2		= BIT(7),
    IDPF_TX_FLEX_CTX_DESC_CMD_SWTCH_UPLNK		= BIT(9),
    IDPF_TX_FLEX_CTX_DESC_CMD_SWTCH_LOCAL		= BIT(10),
    IDPF_TX_FLEX_CTX_DESC_CMD_SWTCH_TARGETVSI	= GENMASK(10, 9),
}

// Standard flex descriptor TSO context quad word
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_flex_tx_tso_ctx_qw {
    pub flex_tlen: __le32,

pub const IDPF_TXD_FLEX_TSO_CTX_FLEX_S: c_int = 24;
    pub mss_rt: __le16,

    pub hdr_len: u8,
    pub flex: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union idpf_flex_tx_ctx_desc {
// DTYPE = IDPF_TX_DESC_DTYPE_CTX (0x01)
    pub qw0: __le64,

    pub qw1: __le64,

    pub tsyn: },
// DTYPE = IDPF_TX_DESC_DTYPE_FLEX_TSO_CTX (0x05)
    pub qw0: idpf_flex_tx_tso_ctx_qw,
    pub cmd_dtype: __le16,
    pub flex: [u8; 6],
    pub qw1: },
    pub tso: },
}
