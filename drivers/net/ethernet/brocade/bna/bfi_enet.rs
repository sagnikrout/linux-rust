//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfi_enet.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//
// BNA Hardware and Firmware Interface
// Skipping statistics collection to avoid clutter.
// Command is no longer needed:
// MTU
// TxQ Stop
// RxQ Stop
// RxF Enable/Disable
//
// HDS-off request is dynamic
// keep structures as multiple of 32-bit fields for alignment.
// All values must be written in big-endian.
//

pub const BFI_ENET_TXQ_PRIO_MAX: c_int = 8;
pub const BFI_ENET_RX_QSET_MAX: c_int = 16;
pub const BFI_ENET_TXQ_WI_VECT_MAX: c_int = 4;
pub const BFI_ENET_VLAN_ID_MAX: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_addr_be_u {
    pub /: *mut *mut u32 addr_hi; / Most Significant 32-bits,
    pub /: *mut *mut u32 addr_lo; / Least Significant 32-Bits,
    pub a32: } __packed,
    pub __packed: },
// T X   Q U E U E   D E F I N E S
// TxQ Vector (a.k.a. Tx-Buffer Descriptor)
// TxQ Entry Opcodes

// TxQ Entry Control Flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_txq_wi_base {
    pub reserved: u8,
    pub /: *mut *mut u8 num_vectors; / number of vectors present,
    pub opcode: u16,
// BFI_ENET_TXQ_WI_SEND or BFI_ENET_TXQ_WI_SEND_LSO
    pub /: *mut *mut u16 flags; / OR of all the flags,
    pub l4_hdr_size_n_offset: u16,
    pub vlan_tag: u16,
    pub /: *mut *mut u16 lso_mss; / Only 14 LSB are valid,
    pub /: *mut *mut u32 frame_length; / Only 24 LSB are valid,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_txq_wi_ext {
    pub reserved: u16,
    pub /: *mut *mut u16 opcode; / BFI_ENET_TXQ_WI_EXTENSION,
    pub reserved2: [u32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_txq_wi_vector {
    pub reserved: u16,
    pub /: *mut *mut u16 length; / Only 14 LSB are valid,
    pub addr: bfi_addr_be_u,
    pub __packed: },
// TxQ Entry Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_txq_entry {
    pub base: bfi_enet_txq_wi_base,
    pub ext: bfi_enet_txq_wi_ext,
    pub wi: } __packed,
    pub vector: [bfi_enet_txq_wi_vector; BFI_ENET_TXQ_WI_VECT_MAX],
    pub __packed: },

// R X   Q U E U E   D E F I N E S
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rxq_entry {
    pub rx_buffer: bfi_addr_be_u,
    pub __packed: },
// R X   C O M P L E T I O N   Q U E U E   D E F I N E S
// CQ Entry Flags

// CQ Entry Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_cq_entry {
    pub flags: u32,
    pub vlan_tag: u16,
    pub length: u16,
    pub rss_hash: u32,
    pub valid: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub rxq_id: u8,
    pub __packed: },
// E N E T   C O N T R O L   P A T H   C O M M A N D S
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_q {
    pub pg_tbl: bfi_addr_u,
    pub first_entry: bfi_addr_u,
    pub /: *mut *mut u16 pages; / # of pages,
    pub page_sz: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_txq {
    pub q: bfi_enet_q,
    pub priority: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rxq {
    pub q: bfi_enet_q,
    pub rx_buffer_size: u16,
    pub rsvd: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_cq {
    pub q: bfi_enet_q,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_ib_cfg {
    pub int_pkt_dma: u8,
    pub int_enabled: u8,
    pub int_pkt_enabled: u8,
    pub continuous_coalescing: u8,
    pub msix: u8,
    pub rsvd: [u8; 3],
    pub coalescing_timeout: u32,
    pub inter_pkt_timeout: u32,
    pub inter_pkt_count: u8,
    pub rsvd1: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_ib {
    pub index_addr: bfi_addr_u,
    pub msix_index: u16,
    pub intx_bitmask: u16,
    pub intr: } __packed,
    pub rsvd: u16,
    pub __packed: },
// ENET command messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_enet_h2i_msgs {
// Rx Commands
    BFI_ENET_H2I_RX_CFG_SET_REQ = 1,
    BFI_ENET_H2I_RX_CFG_CLR_REQ = 2,

    BFI_ENET_H2I_RIT_CFG_REQ = 3,
    BFI_ENET_H2I_RSS_CFG_REQ = 4,
    BFI_ENET_H2I_RSS_ENABLE_REQ = 5,
    BFI_ENET_H2I_RX_PROMISCUOUS_REQ = 6,
    BFI_ENET_H2I_RX_DEFAULT_REQ = 7,

    BFI_ENET_H2I_MAC_UCAST_SET_REQ = 8,
    BFI_ENET_H2I_MAC_UCAST_CLR_REQ = 9,
    BFI_ENET_H2I_MAC_UCAST_ADD_REQ = 10,
    BFI_ENET_H2I_MAC_UCAST_DEL_REQ = 11,

    BFI_ENET_H2I_MAC_MCAST_ADD_REQ = 12,
    BFI_ENET_H2I_MAC_MCAST_DEL_REQ = 13,
    BFI_ENET_H2I_MAC_MCAST_FILTER_REQ = 14,

    BFI_ENET_H2I_RX_VLAN_SET_REQ = 15,
    BFI_ENET_H2I_RX_VLAN_STRIP_ENABLE_REQ = 16,

// Tx Commands
    BFI_ENET_H2I_TX_CFG_SET_REQ = 17,
    BFI_ENET_H2I_TX_CFG_CLR_REQ = 18,

// Port Commands
    BFI_ENET_H2I_PORT_ADMIN_UP_REQ = 19,
    BFI_ENET_H2I_SET_PAUSE_REQ = 20,
    BFI_ENET_H2I_DIAG_LOOPBACK_REQ = 21,

// Get Attributes Command
    BFI_ENET_H2I_GET_ATTR_REQ = 22,

// Statistics Commands
    BFI_ENET_H2I_STATS_GET_REQ = 23,
    BFI_ENET_H2I_STATS_CLR_REQ = 24,

    BFI_ENET_H2I_WOL_MAGIC_REQ = 25,
    BFI_ENET_H2I_WOL_FRAME_REQ = 26,

    BFI_ENET_H2I_MAX = 27,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_enet_i2h_msgs {
// Rx Responses
    BFI_ENET_I2H_RX_CFG_SET_RSP =
    BFA_I2HM(BFI_ENET_H2I_RX_CFG_SET_REQ),
    BFI_ENET_I2H_RX_CFG_CLR_RSP =
    BFA_I2HM(BFI_ENET_H2I_RX_CFG_CLR_REQ),

    BFI_ENET_I2H_RIT_CFG_RSP =
    BFA_I2HM(BFI_ENET_H2I_RIT_CFG_REQ),
    BFI_ENET_I2H_RSS_CFG_RSP =
    BFA_I2HM(BFI_ENET_H2I_RSS_CFG_REQ),
    BFI_ENET_I2H_RSS_ENABLE_RSP =
    BFA_I2HM(BFI_ENET_H2I_RSS_ENABLE_REQ),
    BFI_ENET_I2H_RX_PROMISCUOUS_RSP =
    BFA_I2HM(BFI_ENET_H2I_RX_PROMISCUOUS_REQ),
    BFI_ENET_I2H_RX_DEFAULT_RSP =
    BFA_I2HM(BFI_ENET_H2I_RX_DEFAULT_REQ),

    BFI_ENET_I2H_MAC_UCAST_SET_RSP =
    BFA_I2HM(BFI_ENET_H2I_MAC_UCAST_SET_REQ),
    BFI_ENET_I2H_MAC_UCAST_CLR_RSP =
    BFA_I2HM(BFI_ENET_H2I_MAC_UCAST_CLR_REQ),
    BFI_ENET_I2H_MAC_UCAST_ADD_RSP =
    BFA_I2HM(BFI_ENET_H2I_MAC_UCAST_ADD_REQ),
    BFI_ENET_I2H_MAC_UCAST_DEL_RSP =
    BFA_I2HM(BFI_ENET_H2I_MAC_UCAST_DEL_REQ),

    BFI_ENET_I2H_MAC_MCAST_ADD_RSP =
    BFA_I2HM(BFI_ENET_H2I_MAC_MCAST_ADD_REQ),
    BFI_ENET_I2H_MAC_MCAST_DEL_RSP =
    BFA_I2HM(BFI_ENET_H2I_MAC_MCAST_DEL_REQ),
    BFI_ENET_I2H_MAC_MCAST_FILTER_RSP =
    BFA_I2HM(BFI_ENET_H2I_MAC_MCAST_FILTER_REQ),

    BFI_ENET_I2H_RX_VLAN_SET_RSP =
    BFA_I2HM(BFI_ENET_H2I_RX_VLAN_SET_REQ),

    BFI_ENET_I2H_RX_VLAN_STRIP_ENABLE_RSP =
    BFA_I2HM(BFI_ENET_H2I_RX_VLAN_STRIP_ENABLE_REQ),

// Tx Responses
    BFI_ENET_I2H_TX_CFG_SET_RSP =
    BFA_I2HM(BFI_ENET_H2I_TX_CFG_SET_REQ),
    BFI_ENET_I2H_TX_CFG_CLR_RSP =
    BFA_I2HM(BFI_ENET_H2I_TX_CFG_CLR_REQ),

// Port Responses
    BFI_ENET_I2H_PORT_ADMIN_RSP =
    BFA_I2HM(BFI_ENET_H2I_PORT_ADMIN_UP_REQ),

    BFI_ENET_I2H_SET_PAUSE_RSP =
    BFA_I2HM(BFI_ENET_H2I_SET_PAUSE_REQ),
    BFI_ENET_I2H_DIAG_LOOPBACK_RSP =
    BFA_I2HM(BFI_ENET_H2I_DIAG_LOOPBACK_REQ),

// Attributes Response
    BFI_ENET_I2H_GET_ATTR_RSP =
    BFA_I2HM(BFI_ENET_H2I_GET_ATTR_REQ),

// Statistics Responses
    BFI_ENET_I2H_STATS_GET_RSP =
    BFA_I2HM(BFI_ENET_H2I_STATS_GET_REQ),
    BFI_ENET_I2H_STATS_CLR_RSP =
    BFA_I2HM(BFI_ENET_H2I_STATS_CLR_REQ),

    BFI_ENET_I2H_WOL_MAGIC_RSP =
    BFA_I2HM(BFI_ENET_H2I_WOL_MAGIC_REQ),
    BFI_ENET_I2H_WOL_FRAME_RSP =
    BFA_I2HM(BFI_ENET_H2I_WOL_FRAME_REQ),

// AENs
    BFI_ENET_I2H_LINK_DOWN_AEN = BFA_I2HM(BFI_ENET_H2I_MAX),
    BFI_ENET_I2H_LINK_UP_AEN = BFA_I2HM(BFI_ENET_H2I_MAX + 1),

    BFI_ENET_I2H_PORT_ENABLE_AEN = BFA_I2HM(BFI_ENET_H2I_MAX + 2),
    BFI_ENET_I2H_PORT_DISABLE_AEN = BFA_I2HM(BFI_ENET_H2I_MAX + 3),

    BFI_ENET_I2H_BW_UPDATE_AEN = BFA_I2HM(BFI_ENET_H2I_MAX + 4),
}

// The following error codes can be returned by the enet commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_enet_err {
    BFI_ENET_CMD_OK		= 0,
    BFI_ENET_CMD_FAIL	= 1,
    BFI_ENET_CMD_DUP_ENTRY	= 2,	/* !< Duplicate entry in CAM */
    BFI_ENET_CMD_CAM_FULL	= 3,	/* !< CAM is full */
    BFI_ENET_CMD_NOT_OWNER	= 4,	/* !< Not permitted, b'cos not owner */
    BFI_ENET_CMD_NOT_EXEC	= 5,	/* !< Was not sent to f/w at all */
    BFI_ENET_CMD_WAITING	= 6,	/* !< Waiting for completion */
    BFI_ENET_CMD_PORT_DISABLED = 7,	/* !< port in disabled state */
}

// Generic Request
//
// bfi_enet_req is used by:
// BFI_ENET_H2I_RX_CFG_CLR_REQ
// BFI_ENET_H2I_TX_CFG_CLR_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_req {
    pub mh: bfi_msgq_mhdr,
    pub __packed: },
// Enable/Disable Request
//
// bfi_enet_enable_req is used by:
// BFI_ENET_H2I_RSS_ENABLE_REQ	(enet_id must be zero)
// BFI_ENET_H2I_RX_PROMISCUOUS_REQ (enet_id must be zero)
// BFI_ENET_H2I_RX_DEFAULT_REQ	(enet_id must be zero)
// BFI_ENET_H2I_RX_MAC_MCAST_FILTER_REQ
// BFI_ENET_H2I_PORT_ADMIN_UP_REQ	(enet_id must be zero)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_enable_req {
    pub mh: bfi_msgq_mhdr,
    pub /: *mut *mut u8 enable; / 1 = enable; 0 = disable,
    pub rsvd: [u8; 3],
    pub __packed: },
// Generic Response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rsp {
    pub mh: bfi_msgq_mhdr,
    pub /: *mut *mut u8 error; /!< if error see cmd_offset,
    pub rsvd: u8,
    pub /: *mut *mut u16 cmd_offset; /!< offset to invalid parameter,
    pub __packed: },
// GLOBAL CONFIGURATION
// bfi_enet_attr_req is used by:
// BFI_ENET_H2I_GET_ATTR_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_attr_req {
    pub mh: bfi_msgq_mhdr,
    pub __packed: },
// bfi_enet_attr_rsp is used by:
// BFI_ENET_I2H_GET_ATTR_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_attr_rsp {
    pub mh: bfi_msgq_mhdr,
    pub /: *mut *mut u8 error; /!< if error see cmd_offset,
    pub rsvd: u8,
    pub /: *mut *mut u16 cmd_offset; /!< offset to invalid parameter,
    pub max_cfg: u32,
    pub max_ucmac: u32,
    pub rit_size: u32,
    pub __packed: },
// Tx Configuration
//
// bfi_enet_tx_cfg is used by:
// BFI_ENET_H2I_TX_CFG_SET_REQ
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_enet_tx_vlan_mode {
    BFI_ENET_TX_VLAN_NOP	= 0,
    BFI_ENET_TX_VLAN_INS	= 1,
    BFI_ENET_TX_VLAN_WI	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_tx_cfg {
    pub /: *mut *mut u8 vlan_mode; /!< processing mode,
    pub rsvd: u8,
    pub vlan_id: u16,
    pub admit_tagged_frame: u8,
    pub apply_vlan_filter: u8,
    pub add_to_vswitch: u8,
    pub rsvd1: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_tx_cfg_req {
    pub mh: bfi_msgq_mhdr,
    pub /: *mut *mut u8 num_queues; / # of Tx Queues,
    pub rsvd: [u8; 3],
    pub q: bfi_enet_txq,
    pub ib: bfi_enet_ib,
    pub q_cfg: [} __packed; BFI_ENET_TXQ_PRIO_MAX],
    pub ib_cfg: bfi_enet_ib_cfg,
    pub tx_cfg: bfi_enet_tx_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_tx_cfg_rsp {
    pub mh: bfi_msgq_mhdr,
    pub error: u8,
    pub /: *mut *mut u8 hw_id; / For debugging,
    pub rsvd: [u8; 2],
    pub /: *mut *mut u32 q_dbell; / PCI base address offset,
    pub /: *mut *mut u32 i_dbell; / PCI base address offset,
    pub /: *mut *mut u8 hw_qid; / For debugging,
    pub rsvd: [u8; 3],
    pub q_handles: [} __packed; BFI_ENET_TXQ_PRIO_MAX],
}

// Rx Configuration
//
// bfi_enet_rx_cfg is used by:
// BFI_ENET_H2I_RX_CFG_SET_REQ
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_enet_rxq_type {
    BFI_ENET_RXQ_SINGLE		= 1,
    BFI_ENET_RXQ_LARGE_SMALL	= 2,
    BFI_ENET_RXQ_HDS		= 3,
    BFI_ENET_RXQ_HDS_OPT_BASED	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_enet_hds_type {
    BFI_ENET_HDS_FORCED	= 0x01,
    BFI_ENET_HDS_IPV6_UDP	= 0x02,
    BFI_ENET_HDS_IPV6_TCP	= 0x04,
    BFI_ENET_HDS_IPV4_TCP	= 0x08,
    BFI_ENET_HDS_IPV4_UDP	= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rx_cfg {
    pub rxq_type: u8,
    pub rsvd: [u8; 1],
    pub frame_size: u16,
    pub max_header_size: u8,
    pub force_offset: u8,
    pub type: u8,
    pub rsvd1: u8,
    pub hds: } __packed,
    pub multi_buffer: u8,
    pub strip_vlan: u8,
    pub drop_untagged: u8,
    pub rsvd2: u8,
    pub __packed: },
//
// Multicast frames are received on the ql of q-set index zero.
// On the completion queue.  RxQ ID = even is for large/data buffer queues
// and RxQ ID = odd is for small/header buffer queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rx_cfg_req {
    pub mh: bfi_msgq_mhdr,
    pub /: *mut *mut u8 num_queue_sets; / # of Rx Queue Sets,
    pub rsvd: [u8; 3],
    pub /: *mut *mut bfi_enet_rxq ql; / large/data/single buffers,
    pub /: *mut *mut bfi_enet_rxq qs; / small/header buffers,
    pub cq: bfi_enet_cq,
    pub ib: bfi_enet_ib,
    pub q_cfg: [} __packed; BFI_ENET_RX_QSET_MAX],
    pub ib_cfg: bfi_enet_ib_cfg,
    pub rx_cfg: bfi_enet_rx_cfg,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rx_cfg_rsp {
    pub mh: bfi_msgq_mhdr,
    pub error: u8,
    pub /: *mut *mut u8 hw_id; / For debugging,
    pub rsvd: [u8; 2],
    pub /: *mut *mut u32 ql_dbell; / PCI base address offset,
    pub /: *mut *mut u32 qs_dbell; / PCI base address offset,
    pub /: *mut *mut u32 i_dbell; / PCI base address offset,
    pub /: *mut *mut u8 hw_lqid; / For debugging,
    pub /: *mut *mut u8 hw_sqid; / For debugging,
    pub /: *mut *mut u8 hw_cqid; / For debugging,
    pub rsvd: u8,
    pub q_handles: [} __packed; BFI_ENET_RX_QSET_MAX],
    pub __packed: },
// RIT
//
// bfi_enet_rit_req is used by:
// BFI_ENET_H2I_RIT_CFG_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rit_req {
    pub mh: bfi_msgq_mhdr,
    pub /: *mut *mut u16 size; / number of table-entries used,
    pub rsvd: [u8; 2],
    pub table: [u8; BFI_ENET_RSS_RIT_MAX],
    pub __packed: },
// RSS
//
// bfi_enet_rss_cfg_req is used by:
// BFI_ENET_H2I_RSS_CFG_REQ
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_enet_rss_type {
    BFI_ENET_RSS_IPV6	= 0x01,
    BFI_ENET_RSS_IPV6_TCP	= 0x02,
    BFI_ENET_RSS_IPV4	= 0x04,
    BFI_ENET_RSS_IPV4_TCP	= 0x08
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rss_cfg {
    pub type: u8,
    pub mask: u8,
    pub rsvd: [u8; 2],
    pub key: [u32; BFI_ENET_RSS_KEY_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rss_cfg_req {
    pub mh: bfi_msgq_mhdr,
    pub cfg: bfi_enet_rss_cfg,
    pub __packed: },
// MAC Unicast
//
// bfi_enet_rx_vlan_req is used by:
// BFI_ENET_H2I_MAC_UCAST_SET_REQ
// BFI_ENET_H2I_MAC_UCAST_CLR_REQ
// BFI_ENET_H2I_MAC_UCAST_ADD_REQ
// BFI_ENET_H2I_MAC_UCAST_DEL_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_ucast_req {
    pub mh: bfi_msgq_mhdr,
    pub mac_addr: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
    pub __packed: },
// MAC Unicast + VLAN
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_mac_n_vlan_req {
    pub mh: bfi_msgq_mhdr,
    pub vlan_id: u16,
    pub mac_addr: [u8; ETH_ALEN],
    pub __packed: },
// MAC Multicast
//
// bfi_enet_mac_mfilter_add_req is used by:
// BFI_ENET_H2I_MAC_MCAST_ADD_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_mcast_add_req {
    pub mh: bfi_msgq_mhdr,
    pub mac_addr: [u8; ETH_ALEN],
    pub rsvd: [u8; 2],
    pub __packed: },
// bfi_enet_mac_mfilter_add_rsp is used by:
// BFI_ENET_I2H_MAC_MCAST_ADD_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_mcast_add_rsp {
    pub mh: bfi_msgq_mhdr,
    pub error: u8,
    pub rsvd: u8,
    pub cmd_offset: u16,
    pub handle: u16,
    pub rsvd1: [u8; 2],
    pub __packed: },
// bfi_enet_mac_mfilter_del_req is used by:
// BFI_ENET_H2I_MAC_MCAST_DEL_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_mcast_del_req {
    pub mh: bfi_msgq_mhdr,
    pub handle: u16,
    pub rsvd: [u8; 2],
    pub __packed: },
// VLAN
//
// bfi_enet_rx_vlan_req is used by:
// BFI_ENET_H2I_RX_VLAN_SET_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_rx_vlan_req {
    pub mh: bfi_msgq_mhdr,
    pub block_idx: u8,
    pub rsvd: [u8; 3],
    pub bit_mask: [u32; BFI_ENET_VLAN_WORDS_MAX],
    pub __packed: },
// PAUSE
//
// bfi_enet_set_pause_req is used by:
// BFI_ENET_H2I_SET_PAUSE_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_set_pause_req {
    pub mh: bfi_msgq_mhdr,
    pub rsvd: [u8; 2],
    pub /: *mut *mut u8 tx_pause; / 1 = enable; 0 = disable,
    pub /: *mut *mut u8 rx_pause; / 1 = enable; 0 = disable,
    pub __packed: },
// DIAGNOSTICS
//
// bfi_enet_diag_lb_req is used by:
// BFI_ENET_H2I_DIAG_LOOPBACK
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_diag_lb_req {
    pub mh: bfi_msgq_mhdr,
    pub rsvd: [u8; 2],
    pub /: *mut *mut u8 mode; / cable or Serdes,
    pub /: *mut *mut u8 enable; / 1 = enable; 0 = disable,
    pub __packed: },
// enum for Loopback opmodes
}

// STATISTICS
//
// bfi_enet_stats_req is used by:
// BFI_ENET_H2I_STATS_GET_REQ
// BFI_ENET_I2H_STATS_CLR_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_req {
    pub mh: bfi_msgq_mhdr,
    pub stats_mask: u16,
    pub rsvd: [u8; 2],
    pub rx_enet_mask: u32,
    pub tx_enet_mask: u32,
    pub host_buffer: bfi_addr_u,
    pub __packed: },
// defines for "stats_mask" above.

pub const BFI_ENET_STATS_ALL: c_uint = 0x1f;
// TxF Frame Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_txf {
    pub ucast_octets: u64,
    pub ucast: u64,
    pub ucast_vlan: u64,
    pub mcast_octets: u64,
    pub mcast: u64,
    pub mcast_vlan: u64,
    pub bcast_octets: u64,
    pub bcast: u64,
    pub bcast_vlan: u64,
    pub errors: u64,
    pub /: *mut *mut u64 filter_vlan; / frames filtered due to VLAN,
    pub /: *mut *mut u64 filter_mac_sa; / frames filtered due to SA check,
    pub __packed: },
// RxF Frame Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_rxf {
    pub ucast_octets: u64,
    pub ucast: u64,
    pub ucast_vlan: u64,
    pub mcast_octets: u64,
    pub mcast: u64,
    pub mcast_vlan: u64,
    pub bcast_octets: u64,
    pub bcast: u64,
    pub bcast_vlan: u64,
    pub frame_drops: u64,
    pub __packed: },
// FC Tx Frame Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_fc_tx {
    pub txf_ucast_octets: u64,
    pub txf_ucast: u64,
    pub txf_ucast_vlan: u64,
    pub txf_mcast_octets: u64,
    pub txf_mcast: u64,
    pub txf_mcast_vlan: u64,
    pub txf_bcast_octets: u64,
    pub txf_bcast: u64,
    pub txf_bcast_vlan: u64,
    pub txf_parity_errors: u64,
    pub txf_timeout: u64,
    pub txf_fid_parity_errors: u64,
    pub __packed: },
// FC Rx Frame Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_fc_rx {
    pub rxf_ucast_octets: u64,
    pub rxf_ucast: u64,
    pub rxf_ucast_vlan: u64,
    pub rxf_mcast_octets: u64,
    pub rxf_mcast: u64,
    pub rxf_mcast_vlan: u64,
    pub rxf_bcast_octets: u64,
    pub rxf_bcast: u64,
    pub rxf_bcast_vlan: u64,
    pub __packed: },
// RAD Frame Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_rad {
    pub rx_frames: u64,
    pub rx_octets: u64,
    pub rx_vlan_frames: u64,
    pub rx_ucast: u64,
    pub rx_ucast_octets: u64,
    pub rx_ucast_vlan: u64,
    pub rx_mcast: u64,
    pub rx_mcast_octets: u64,
    pub rx_mcast_vlan: u64,
    pub rx_bcast: u64,
    pub rx_bcast_octets: u64,
    pub rx_bcast_vlan: u64,
    pub rx_drops: u64,
    pub __packed: },
// BPC Tx Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_bpc {
// transmit stats
    pub tx_pause: [u64; 8],
    pub /: *mut *mut u64 tx_zero_pause[8]; /!< Pause cancellation,
// !<Pause initiation rather than retention
    pub tx_first_pause: [u64; 8],
// receive stats
    pub rx_pause: [u64; 8],
    pub /: *mut *mut u64 rx_zero_pause[8]; /!< Pause cancellation,
// !<Pause initiation rather than retention
    pub rx_first_pause: [u64; 8],
    pub __packed: },
// MAC Rx Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats_mac {
    pub /: *mut *mut u64 stats_clr_cnt; / times this stats cleared,
    pub /: *mut *mut u64 frame_64; / both rx and tx counter,
    pub /: *mut *mut u64 frame_65_127; / both rx and tx counter,
    pub /: *mut *mut u64 frame_128_255; / both rx and tx counter,
    pub /: *mut *mut u64 frame_256_511; / both rx and tx counter,
    pub /: *mut *mut u64 frame_512_1023; / both rx and tx counter,
    pub /: *mut *mut u64 frame_1024_1518; / both rx and tx counter,
    pub /: *mut *mut u64 frame_1519_1522; / both rx and tx counter,
// receive stats
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_fcs_error: u64,
    pub rx_multicast: u64,
    pub rx_broadcast: u64,
    pub rx_control_frames: u64,
    pub rx_pause: u64,
    pub rx_unknown_opcode: u64,
    pub rx_alignment_error: u64,
    pub rx_frame_length_error: u64,
    pub rx_code_error: u64,
    pub rx_carrier_sense_error: u64,
    pub rx_undersize: u64,
    pub rx_oversize: u64,
    pub rx_fragments: u64,
    pub rx_jabber: u64,
    pub rx_drop: u64,
// transmit stats
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_multicast: u64,
    pub tx_broadcast: u64,
    pub tx_pause: u64,
    pub tx_deferral: u64,
    pub tx_excessive_deferral: u64,
    pub tx_single_collision: u64,
    pub tx_muliple_collision: u64,
    pub tx_late_collision: u64,
    pub tx_excessive_collision: u64,
    pub tx_total_collision: u64,
    pub tx_pause_honored: u64,
    pub tx_drop: u64,
    pub tx_jabber: u64,
    pub tx_fcs_error: u64,
    pub tx_control_frame: u64,
    pub tx_oversize: u64,
    pub tx_undersize: u64,
    pub tx_fragments: u64,
    pub __packed: },
// Complete statistics, DMAed from fw to host followed by
// BFI_ENET_I2H_STATS_GET_RSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_enet_stats {
    pub mac_stats: bfi_enet_stats_mac,
    pub bpc_stats: bfi_enet_stats_bpc,
    pub rad_stats: bfi_enet_stats_rad,
    pub rlb_stats: bfi_enet_stats_rad,
    pub fc_rx_stats: bfi_enet_stats_fc_rx,
    pub fc_tx_stats: bfi_enet_stats_fc_tx,
    pub rxf_stats: [bfi_enet_stats_rxf; BFI_ENET_CFG_MAX],
    pub txf_stats: [bfi_enet_stats_txf; BFI_ENET_CFG_MAX],
    pub __packed: },
