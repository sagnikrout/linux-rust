//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pensando/ionic/ionic_if.h
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


// SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) OR BSD-2-Clause
// Copyright (c) 2017-2020 Pensando Systems, Inc.  All rights reserved.
pub const IONIC_DEV_INFO_SIGNATURE: c_uint = 0x44455649      /* 'DEVI' */;
pub const IONIC_DEV_INFO_VERSION: c_int = 1;
pub const IONIC_IFNAMSIZ: c_int = 16;
//
// enum ionic_cmd_opcode - Device commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_cmd_opcode {
    IONIC_CMD_NOP				= 0,

// Device commands
    IONIC_CMD_IDENTIFY			= 1,
    IONIC_CMD_INIT				= 2,
    IONIC_CMD_RESET				= 3,
    IONIC_CMD_GETATTR			= 4,
    IONIC_CMD_SETATTR			= 5,

// Port commands
    IONIC_CMD_PORT_IDENTIFY			= 10,
    IONIC_CMD_PORT_INIT			= 11,
    IONIC_CMD_PORT_RESET			= 12,
    IONIC_CMD_PORT_GETATTR			= 13,
    IONIC_CMD_PORT_SETATTR			= 14,

// LIF commands
    IONIC_CMD_LIF_IDENTIFY			= 20,
    IONIC_CMD_LIF_INIT			= 21,
    IONIC_CMD_LIF_RESET			= 22,
    IONIC_CMD_LIF_GETATTR			= 23,
    IONIC_CMD_LIF_SETATTR			= 24,
    IONIC_CMD_LIF_SETPHC			= 25,

    IONIC_CMD_RX_MODE_SET			= 30,
    IONIC_CMD_RX_FILTER_ADD			= 31,
    IONIC_CMD_RX_FILTER_DEL			= 32,

// Queue commands
    IONIC_CMD_Q_IDENTIFY			= 39,
    IONIC_CMD_Q_INIT			= 40,
    IONIC_CMD_Q_CONTROL			= 41,

// RDMA commands
    IONIC_CMD_RDMA_RESET_LIF		= 50,
    IONIC_CMD_RDMA_CREATE_EQ		= 51,
    IONIC_CMD_RDMA_CREATE_CQ		= 52,
    IONIC_CMD_RDMA_CREATE_ADMINQ		= 53,

// SR/IOV commands
    IONIC_CMD_VF_GETATTR			= 60,
    IONIC_CMD_VF_SETATTR			= 61,
    IONIC_CMD_VF_CTRL			= 62,

// CMB command
    IONIC_CMD_DISCOVER_CMB			= 80,

// QoS commands
    IONIC_CMD_QOS_CLASS_IDENTIFY		= 240,
    IONIC_CMD_QOS_CLASS_INIT		= 241,
    IONIC_CMD_QOS_CLASS_RESET		= 242,
    IONIC_CMD_QOS_CLASS_UPDATE		= 243,
    IONIC_CMD_QOS_CLEAR_STATS		= 244,
    IONIC_CMD_QOS_RESET			= 245,

// Firmware commands
    IONIC_CMD_FW_DOWNLOAD                   = 252,
    IONIC_CMD_FW_CONTROL                    = 253,
    IONIC_CMD_FW_DOWNLOAD_V1		= 254,
    IONIC_CMD_FW_CONTROL_V1		        = 255,
}

//
// enum ionic_status_code - Device command return codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_status_code {
    IONIC_RC_SUCCESS	= 0,	/* Success */
    IONIC_RC_EVERSION	= 1,	/* Incorrect version for request */
    IONIC_RC_EOPCODE	= 2,	/* Invalid cmd opcode */
    IONIC_RC_EIO		= 3,	/* I/O error */
    IONIC_RC_EPERM		= 4,	/* Permission denied */
    IONIC_RC_EQID		= 5,	/* Bad qid */
    IONIC_RC_EQTYPE		= 6,	/* Bad qtype */
    IONIC_RC_ENOENT		= 7,	/* No such element */
    IONIC_RC_EINTR		= 8,	/* operation interrupted */
    IONIC_RC_EAGAIN		= 9,	/* Try again */
    IONIC_RC_ENOMEM		= 10,	/* Out of memory */
    IONIC_RC_EFAULT		= 11,	/* Bad address */
    IONIC_RC_EBUSY		= 12,	/* Device or resource busy */
    IONIC_RC_EEXIST		= 13,	/* object already exists */
    IONIC_RC_EINVAL		= 14,	/* Invalid argument */
    IONIC_RC_ENOSPC		= 15,	/* No space left or alloc failure */
    IONIC_RC_ERANGE		= 16,	/* Parameter out of range */
    IONIC_RC_BAD_ADDR	= 17,	/* Descriptor contains a bad ptr */
    IONIC_RC_DEV_CMD	= 18,	/* Device cmd attempted on AdminQ */
    IONIC_RC_ENOSUPP	= 19,	/* Operation not supported */
    IONIC_RC_ERROR		= 29,	/* Generic error */
    IONIC_RC_ERDMA		= 30,	/* Generic RDMA error */
    IONIC_RC_EVFID		= 31,	/* VF ID does not exist */
    IONIC_RC_EBAD_FW	= 32,	/* FW file is invalid or corrupted */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_notifyq_opcode {
    IONIC_EVENT_LINK_CHANGE		= 1,
    IONIC_EVENT_RESET		= 2,
    IONIC_EVENT_HEARTBEAT		= 3,
    IONIC_EVENT_LOG			= 4,
    IONIC_EVENT_XCVR		= 5,
}

//
// struct ionic_admin_cmd - General admin command format
// @opcode:     Opcode for the command
// @rsvd:       reserved byte(s)
// @lif_index:  LIF index
// @cmd_data:   Opcode-specific command bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub lif_index: __le16,
    pub cmd_data: [u8; 60],
}

//
// struct ionic_admin_comp - General admin command completion format
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @cmd_data:   Command-specific bytes
// @color:      Color bit (Always 0 for commands issued to the
// Device Cmd Registers)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub cmd_data: [u8; 11],
    pub color: u8,
pub const IONIC_COMP_COLOR_MASK: c_uint = 0x80;
}

//
// struct ionic_nop_cmd - NOP command
// @opcode: opcode
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_nop_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 63],
}

//
// struct ionic_nop_comp - NOP command completion
// @status: Status of the command (enum ionic_status_code)
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_nop_comp {
    pub status: u8,
    pub rsvd: [u8; 15],
}

//
// struct ionic_dev_init_cmd - Device init command
// @opcode:    opcode
// @type:      Device type
// @rsvd:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_init_cmd {
    pub opcode: u8,
    pub type: u8,
    pub rsvd: [u8; 62],
}

//
// struct ionic_dev_init_comp - Device init command completion
// @status: Status of the command (enum ionic_status_code)
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_init_comp {
    pub status: u8,
    pub rsvd: [u8; 15],
}

//
// struct ionic_dev_reset_cmd - Device reset command
// @opcode: opcode
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_reset_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 63],
}

//
// struct ionic_dev_reset_comp - Reset command completion
// @status: Status of the command (enum ionic_status_code)
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_reset_comp {
    pub status: u8,
    pub rsvd: [u8; 15],
}

pub const IONIC_IDENTITY_VERSION_1: c_int = 1;
pub const IONIC_DEV_IDENTITY_VERSION_2: c_int = 2;
//
// struct ionic_dev_identify_cmd - Driver/device identify command
// @opcode:  opcode
// @ver:     Highest version of identify supported by driver
// @rsvd:    reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_identify_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub rsvd: [u8; 62],
}

//
// struct ionic_dev_identify_comp - Driver/device identify command completion
// @status: Status of the command (enum ionic_status_code)
// @ver:    Version of identify returned by device
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_identify_comp {
    pub status: u8,
    pub ver: u8,
    pub rsvd: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_os_type {
    IONIC_OS_TYPE_LINUX   = 1,
    IONIC_OS_TYPE_WIN     = 2,
    IONIC_OS_TYPE_DPDK    = 3,
    IONIC_OS_TYPE_FREEBSD = 4,
    IONIC_OS_TYPE_IPXE    = 5,
    IONIC_OS_TYPE_ESXI    = 6,
}

//
// union ionic_drv_identity - driver identity information
// @os_type:          OS type (see enum ionic_os_type)
// @os_dist:          OS distribution, numeric format
// @os_dist_str:      OS distribution, string format
// @kernel_ver:       Kernel version, numeric format
// @kernel_ver_str:   Kernel version, string format
// @driver_ver_str:   Driver version, string format
// @words:            word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_drv_identity {
    pub os_type: __le32,
    pub os_dist: __le32,
    pub os_dist_str: [c_char; 128],
    pub kernel_ver: __le32,
    pub kernel_ver_str: [c_char; 32],
    pub driver_ver_str: [c_char; 32],
}

//
// enum ionic_dev_capability - Device capabilities
// @IONIC_DEV_CAP_VF_CTRL:     Device supports VF ctrl operations
// @IONIC_DEV_CAP_DISC_CMB:    Device supports CMB discovery operations
// @IONIC_DEV_CAP_EXTRA_STATS: Device supports extra stats schema
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_dev_capability {
    IONIC_DEV_CAP_VF_CTRL        = BIT(0),
    IONIC_DEV_CAP_DISC_CMB       = BIT(1),
    IONIC_DEV_CAP_EXTRA_STATS    = BIT(4),
}

//
// union ionic_dev_identity - device identity information
// @version:          Version of device identify
// @type:             Identify type (0 for now)
// @rsvd:             reserved byte(s)
// @nports:           Number of ports provisioned
// @rsvd2:            reserved byte(s)
// @nlifs:            Number of LIFs provisioned
// @nintrs:           Number of interrupts provisioned
// @ndbpgs_per_lif:   Number of doorbell pages per LIF
// @intr_coal_mult:   Interrupt coalescing multiplication factor
// Scale user-supplied interrupt coalescing
// value in usecs to device units using:
// device units = usecs * mult / div
// @intr_coal_div:    Interrupt coalescing division factor
// Scale user-supplied interrupt coalescing
// value in usecs to device units using:
// device units = usecs * mult / div
// @eq_count:         Number of shared event queues
// @hwstamp_mask:     Bitmask for subtraction of hardware tick values.
// @hwstamp_mult:     Hardware tick to nanosecond multiplier.
// @hwstamp_shift:    Hardware tick to nanosecond divisor (power of two).
// @capabilities:     Device capabilities
// @words:            word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_dev_identity {
    pub version: u8,
    pub type: u8,
    pub rsvd: [u8; 2],
    pub nports: u8,
    pub rsvd2: [u8; 3],
    pub nlifs: __le32,
    pub nintrs: __le32,
    pub ndbpgs_per_lif: __le32,
    pub intr_coal_mult: __le32,
    pub intr_coal_div: __le32,
    pub eq_count: __le32,
    pub hwstamp_mask: __le64,
    pub hwstamp_mult: __le32,
    pub hwstamp_shift: __le32,
    pub capabilities: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_lif_type {
    IONIC_LIF_TYPE_CLASSIC = 0,
    IONIC_LIF_TYPE_MACVLAN = 1,
    IONIC_LIF_TYPE_NETQUEUE = 2,
}

//
// struct ionic_lif_identify_cmd - LIF identify command
// @opcode:  opcode
// @type:    LIF type (enum ionic_lif_type)
// @ver:     Version of identify returned by device
// @rsvd:    reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_identify_cmd {
    pub opcode: u8,
    pub type: u8,
    pub ver: u8,
    pub rsvd: [u8; 61],
}

//
// struct ionic_lif_identify_comp - LIF identify command completion
// @status:  Status of the command (enum ionic_status_code)
// @ver:     Version of identify returned by device
// @rsvd2:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_identify_comp {
    pub status: u8,
    pub ver: u8,
    pub rsvd2: [u8; 14],
}

//
// enum ionic_lif_capability - LIF capabilities
// @IONIC_LIF_CAP_ETH:     LIF supports Ethernet
// @IONIC_LIF_CAP_RDMA:    LIF supports RDMA
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_lif_capability {
    IONIC_LIF_CAP_ETH        = BIT(0),
    IONIC_LIF_CAP_RDMA       = BIT(1),
}

//
// enum ionic_logical_qtype - Logical Queue Types
// @IONIC_QTYPE_ADMINQ:    Administrative Queue
// @IONIC_QTYPE_NOTIFYQ:   Notify Queue
// @IONIC_QTYPE_RXQ:       Receive Queue
// @IONIC_QTYPE_TXQ:       Transmit Queue
// @IONIC_QTYPE_EQ:        Event Queue
// @IONIC_QTYPE_MAX:       Max queue type supported
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_logical_qtype {
    IONIC_QTYPE_ADMINQ  = 0,
    IONIC_QTYPE_NOTIFYQ = 1,
    IONIC_QTYPE_RXQ     = 2,
    IONIC_QTYPE_TXQ     = 3,
    IONIC_QTYPE_EQ      = 4,
    IONIC_QTYPE_MAX     = 16,
}

//
// enum ionic_q_feature - Common Features for most queue types
//
// Common features use bits 0-15. Per-queue-type features use higher bits.
//
// @IONIC_QIDENT_F_CQ:      Queue has completion ring
// @IONIC_QIDENT_F_SG:      Queue has scatter/gather ring
// @IONIC_QIDENT_F_EQ:      Queue can use event queue
// @IONIC_QIDENT_F_CMB:     Queue is in cmb bar
// @IONIC_Q_F_2X_DESC:      Double main descriptor size
// @IONIC_Q_F_2X_CQ_DESC:   Double cq descriptor size
// @IONIC_Q_F_2X_SG_DESC:   Double sg descriptor size
// @IONIC_Q_F_4X_DESC:      Quadruple main descriptor size
// @IONIC_Q_F_4X_CQ_DESC:   Quadruple cq descriptor size
// @IONIC_Q_F_4X_SG_DESC:   Quadruple sg descriptor size
// @IONIC_QIDENT_F_EXPDB:   Queue supports express doorbell
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_q_feature {
    IONIC_QIDENT_F_CQ		= BIT_ULL(0),
    IONIC_QIDENT_F_SG		= BIT_ULL(1),
    IONIC_QIDENT_F_EQ		= BIT_ULL(2),
    IONIC_QIDENT_F_CMB		= BIT_ULL(3),
    IONIC_Q_F_2X_DESC		= BIT_ULL(4),
    IONIC_Q_F_2X_CQ_DESC		= BIT_ULL(5),
    IONIC_Q_F_2X_SG_DESC		= BIT_ULL(6),
    IONIC_Q_F_4X_DESC		= BIT_ULL(7),
    IONIC_Q_F_4X_CQ_DESC		= BIT_ULL(8),
    IONIC_Q_F_4X_SG_DESC		= BIT_ULL(9),
    IONIC_QIDENT_F_EXPDB		= BIT_ULL(10),
}

//
// enum ionic_rxq_feature - RXQ-specific Features
//
// Per-queue-type features use bits 16 and higher.
//
// @IONIC_RXQ_F_HWSTAMP:   Queue supports Hardware Timestamping
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_rxq_feature {
    IONIC_RXQ_F_HWSTAMP		= BIT_ULL(16),
}

//
// enum ionic_txq_feature - TXQ-specific Features
//
// Per-queue-type features use bits 16 and higher.
//
// @IONIC_TXQ_F_HWSTAMP:   Queue supports Hardware Timestamping
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_txq_feature {
    IONIC_TXQ_F_HWSTAMP		= BIT(16),
}

//
// enum ionic_hwstamp_bits - Hardware timestamp decoding bits
// @IONIC_HWSTAMP_INVALID:          Invalid hardware timestamp value
// @IONIC_HWSTAMP_CQ_NEGOFFSET:     Timestamp field negative offset
// from the base cq descriptor.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_hwstamp_bits {
    IONIC_HWSTAMP_INVALID	    = ~0ull,
    IONIC_HWSTAMP_CQ_NEGOFFSET  = 8,
}

//
// struct ionic_lif_logical_qtype - Descriptor of logical to HW queue type
// @qtype:          Hardware Queue Type
// @rsvd:           reserved byte(s)
// @qid_count:      Number of Queue IDs of the logical type
// @qid_base:       Minimum Queue ID of the logical type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_logical_qtype {
    pub qtype: u8,
    pub rsvd: [u8; 3],
    pub qid_count: __le32,
    pub qid_base: __le32,
}

//
// enum ionic_lif_state - LIF state
// @IONIC_LIF_DISABLE:     LIF disabled
// @IONIC_LIF_ENABLE:      LIF enabled
// @IONIC_LIF_QUIESCE:     LIF Quiesced
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_lif_state {
    IONIC_LIF_QUIESCE	= 0,
    IONIC_LIF_ENABLE	= 1,
    IONIC_LIF_DISABLE	= 2,
}

//
// union ionic_lif_config - LIF configuration
// @state:          LIF state (enum ionic_lif_state)
// @rsvd:           reserved byte(s)
// @name:           LIF name
// @mtu:            MTU
// @mac:            Station MAC address
// @vlan:           Default Vlan ID
// @features:       Features (enum ionic_eth_hw_features)
// @queue_count:    Queue counts per queue-type
// @words:          word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_lif_config {
    pub state: u8,
    pub rsvd: [u8; 3],
    pub name: [c_char; IONIC_IFNAMSIZ],
    pub mtu: __le32,
    pub mac: [u8; 6],
    pub vlan: __le16,
    pub features: __le64,
    pub queue_count: [__le32; IONIC_QTYPE_MAX],
    pub __packed: },
    pub words: [__le32; 64],
}

//
// enum ionic_lif_rdma_cap_stats - LIF stat type
// @IONIC_LIF_RDMA_STAT_GLOBAL:     Global stats
// @IONIC_LIF_RDMA_STAT_QP:         Queue pair stats
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_lif_rdma_cap_stats {
    IONIC_LIF_RDMA_STAT_GLOBAL = BIT(0),
    IONIC_LIF_RDMA_STAT_QP = BIT(1),
}

//
// struct ionic_lif_identity - LIF identity information (type-specific)
//
// @capabilities:        LIF capabilities
//
// @eth:                    Ethernet identify structure
// @eth.version:            Ethernet identify structure version
// @eth.rsvd:               reserved byte(s)
// @eth.max_ucast_filters:  Number of perfect unicast addresses supported
// @eth.max_mcast_filters:  Number of perfect multicast addresses supported
// @eth.min_frame_size:     Minimum size of frames to be sent
// @eth.max_frame_size:     Maximum size of frames to be sent
// @eth.rsvd2:              reserved byte(s)
// @eth.hwstamp_tx_modes:   Bitmask of BIT_ULL(enum ionic_txstamp_mode)
// @eth.hwstamp_rx_filters: Bitmask of enum ionic_pkt_class
// @eth.rsvd3:              reserved byte(s)
// @eth.config:             LIF config struct with features, mtu, mac, q counts
//
// @rdma:                RDMA identify structure
// @rdma.version:         RDMA capability version
// @rdma.qp_opcodes:      Number of RDMA queue pair opcodes supported
// @rdma.admin_opcodes:   Number of RDMA admin opcodes supported
// @rdma.minor_version:   RDMA capability minor version
// @rdma.npts_per_lif:    Page table size per LIF
// @rdma.nmrs_per_lif:    Number of memory regions per LIF
// @rdma.nahs_per_lif:    Number of address handles per LIF
// @rdma.max_stride:      Max work request stride
// @rdma.cl_stride:       Cache line stride
// @rdma.pte_stride:      Page table entry stride
// @rdma.rrq_stride:      Remote RQ work request stride
// @rdma.rsq_stride:      Remote SQ work request stride
// @rdma.dcqcn_profiles:  Number of DCQCN profiles
// @rdma.udma_shift:      Log2 number of queues per queue group
// @rdma.rsvd_dimensions: Reserved byte
// @rdma.page_size_cap:   Supported page sizes
// @rdma.aq_qtype:        RDMA Admin Qtype
// @rdma.sq_qtype:        RDMA Send Qtype
// @rdma.rq_qtype:        RDMA Receive Qtype
// @rdma.cq_qtype:        RDMA Completion Qtype
// @rdma.eq_qtype:        RDMA Event Qtype
// @rdma.stats_type:      Supported statistics type
// (enum ionic_lif_rdma_cap_stats)
// @rdma.rsvd:            Reserved byte
// @rdma.rcq_sign_bit:    RCQ sign bit
// @rdma.rsvd1:           Reserved byte(s)
// @words:               word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_lif_identity {
    pub capabilities: __le64,
    pub version: u8,
    pub rsvd: [u8; 3],
    pub max_ucast_filters: __le32,
    pub max_mcast_filters: __le32,
    pub rss_ind_tbl_sz: __le16,
    pub min_frame_size: __le32,
    pub max_frame_size: __le32,
    pub rsvd2: [u8; 2],
    pub hwstamp_tx_modes: __le64,
    pub hwstamp_rx_filters: __le64,
    pub rsvd3: [u8; 88],
    pub config: ionic_lif_config,
    pub eth: } __packed,
    pub version: u8,
    pub qp_opcodes: u8,
    pub admin_opcodes: u8,
    pub minor_version: u8,
    pub npts_per_lif: __le32,
    pub nmrs_per_lif: __le32,
    pub nahs_per_lif: __le32,
    pub max_stride: u8,
    pub cl_stride: u8,
    pub pte_stride: u8,
    pub rrq_stride: u8,
    pub rsq_stride: u8,
    pub dcqcn_profiles: u8,
    pub udma_shift: u8,
    pub rsvd_dimensions: u8,
    pub page_size_cap: __le64,
    pub aq_qtype: ionic_lif_logical_qtype,
    pub sq_qtype: ionic_lif_logical_qtype,
    pub rq_qtype: ionic_lif_logical_qtype,
    pub cq_qtype: ionic_lif_logical_qtype,
    pub eq_qtype: ionic_lif_logical_qtype,
    pub stats_type: __le16,
    pub rsvd: u8,
    pub rcq_sign_bit: u8,
    pub rsvd1: [u8; 160],
    pub rdma: } __packed,
    pub __packed: },
    pub words: [__le32; 478],
}

//
// struct ionic_lif_init_cmd - LIF init command
// @opcode:       Opcode
// @type:         LIF type (enum ionic_lif_type)
// @index:        LIF index
// @rsvd:         reserved byte(s)
// @info_pa:      Destination address for LIF info (struct ionic_lif_info)
// @rsvd2:        reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_init_cmd {
    pub opcode: u8,
    pub type: u8,
    pub index: __le16,
    pub rsvd: __le32,
    pub info_pa: __le64,
    pub rsvd2: [u8; 48],
}

//
// struct ionic_lif_init_comp - LIF init command completion
// @status:	Status of the command (enum ionic_status_code)
// @rsvd:	reserved byte(s)
// @hw_index:	Hardware index of the initialized LIF
// @rsvd2:	reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_init_comp {
    pub status: u8,
    pub rsvd: u8,
    pub hw_index: __le16,
    pub rsvd2: [u8; 12],
}

//
// struct ionic_q_identify_cmd - queue identify command
// @opcode:     opcode
// @rsvd:       reserved byte(s)
// @lif_type:   LIF type (enum ionic_lif_type)
// @type:       Logical queue type (enum ionic_logical_qtype)
// @ver:        Highest queue type version that the driver supports
// @rsvd2:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_q_identify_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub lif_type: __le16,
    pub type: u8,
    pub ver: u8,
    pub rsvd2: [u8; 58],
}

//
// struct ionic_q_identify_comp - queue identify command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @ver:        Queue type version that can be used with FW
// @rsvd2:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_q_identify_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub ver: u8,
    pub rsvd2: [u8; 11],
}

//
// union ionic_q_identity - queue identity information
// @version:        Queue type version that can be used with FW
// @supported:      Bitfield of queue versions, first bit = ver 0
// @rsvd:           reserved byte(s)
// @features:       Queue features (enum ionic_q_feature, etc)
// @desc_sz:        Descriptor size
// @comp_sz:        Completion descriptor size
// @sg_desc_sz:     Scatter/Gather descriptor size
// @max_sg_elems:   Maximum number of Scatter/Gather elements
// @sg_desc_stride: Number of Scatter/Gather elements per descriptor
// @words:          word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_q_identity {
    pub version: u8,
    pub supported: u8,
    pub rsvd: [u8; 6],
    pub features: __le64,
    pub desc_sz: __le16,
    pub comp_sz: __le16,
    pub sg_desc_sz: __le16,
    pub max_sg_elems: __le16,
    pub sg_desc_stride: __le16,
}

//
// struct ionic_q_init_cmd - Queue init command
// @opcode:       opcode
// @rsvd:         reserved byte(s)
// @type:         Logical queue type
// @ver:          Queue type version
// @rsvd1:        reserved byte(s)
// @lif_index:    LIF index
// @index:        (LIF, qtype) relative admin queue index
// @intr_index:   Interrupt control register index, or Event queue index
// @pid:          Process ID
// @flags:
// IRQ:        Interrupt requested on completion
// ENA:        Enable the queue.  If ENA=0 the queue is initialized
// but remains disabled, to be later enabled with the
// Queue Enable command.  If ENA=1, then queue is
// initialized and then enabled.
// SG:         Enable Scatter-Gather on the queue.
// in number of descs.  The actual ring size is
// (1 << ring_size).  For example, to
// select a ring size of 64 descriptors write
// ring_size = 6.  The minimum ring_size value is 2
// for a ring size of 4 descriptors.  The maximum
// ring_size value is 16 for a ring size of 64k
// descriptors.  Values of ring_size <2 and >16 are
// reserved.
// EQ:         Enable the Event Queue
// @cos:          Class of service for this queue
// @ring_size:    Queue ring size, encoded as a log2(size)
// @ring_base:    Queue ring base address
// @cq_ring_base: Completion queue ring base address
// @sg_ring_base: Scatter/Gather ring base address
// @rsvd2:        reserved byte(s)
// @features:     Mask of queue features to enable, if not in the flags above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_q_init_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub lif_index: __le16,
    pub type: u8,
    pub ver: u8,
    pub rsvd1: [u8; 2],
    pub index: __le32,
    pub pid: __le16,
    pub intr_index: __le16,
    pub flags: __le16,
pub const IONIC_QINIT_F_IRQ: c_uint = 0x01	/* Request interrupt on completion */;
pub const IONIC_QINIT_F_ENA: c_uint = 0x02	/* Enable the queue */;
pub const IONIC_QINIT_F_SG: c_uint = 0x04	/* Enable scatter/gather on the queue */;
pub const IONIC_QINIT_F_EQ: c_uint = 0x08	/* Enable event queue */;
pub const IONIC_QINIT_F_CMB: c_uint = 0x10	/* Enable cmb-based queue */;
pub const IONIC_QINIT_F_DEBUG: c_uint = 0x80	/* Enable queue debugging */;
    pub cos: u8,
    pub ring_size: u8,
    pub ring_base: __le64,
    pub cq_ring_base: __le64,
    pub sg_ring_base: __le64,
    pub rsvd2: [u8; 12],
    pub features: __le64,
    pub __packed: },
//
// struct ionic_q_init_comp - Queue init command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @hw_index:   Hardware Queue ID
// @hw_type:    Hardware Queue type
// @rsvd2:      reserved byte(s)
// @color:      Color
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_q_init_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub hw_index: __le32,
    pub hw_type: u8,
    pub rsvd2: [u8; 6],
    pub color: u8,
}

// the device's internal addressing uses up to 52 bits
pub const IONIC_ADDR_LEN: c_int = 52;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_txq_desc_opcode {
    IONIC_TXQ_DESC_OPCODE_CSUM_NONE = 0,
    IONIC_TXQ_DESC_OPCODE_CSUM_PARTIAL = 1,
    IONIC_TXQ_DESC_OPCODE_CSUM_HW = 2,
    IONIC_TXQ_DESC_OPCODE_TSO = 3,
}

//
// struct ionic_txq_desc - Ethernet Tx queue descriptor format
// @cmd:          Tx operation, see IONIC_TXQ_DESC_OPCODE_*:
//
// IONIC_TXQ_DESC_OPCODE_CSUM_NONE:
// Non-offload send.  No segmentation,
// fragmentation or checksum calc/insertion is
// performed by device; packet is prepared
// to send by software stack and requires
// no further manipulation from device.
//
// IONIC_TXQ_DESC_OPCODE_CSUM_PARTIAL:
// Offload 16-bit L4 checksum
// calculation/insertion.  The device will
// calculate the L4 checksum value and
// insert the result in the packet's L4
// header checksum field.  The L4 checksum
// is calculated starting at @csum_start bytes
// into the packet to the end of the packet.
// The checksum insertion position is given
// in @csum_offset, which is the offset from
// @csum_start to the checksum field in the L4
// header.  This feature is only applicable to
// protocols such as TCP, UDP and ICMP where a
// standard (i.e. the 'IP-style' checksum)
// one's complement 16-bit checksum is used,
// using an IP pseudo-header to seed the
// calculation.  Software will preload the L4
// checksum field with the IP pseudo-header
// checksum.
//
// For tunnel encapsulation, @csum_start and
// @csum_offset refer to the inner L4
// header.  Supported tunnels encapsulations
// are: IPIP, GRE, and UDP.  If the @encap
// is clear, no further processing by the
// device is required; software will
// calculate the outer header checksums.  If
// the @encap is set, the device will
// offload the outer header checksums using
// LCO (local checksum offload) (see
// Documentation/networking/checksum-offloads.rst
// for more info).
//
// IONIC_TXQ_DESC_OPCODE_CSUM_HW:
// Offload 16-bit checksum computation to hardware.
// If @csum_l3 is set then the packet's L3 checksum is
// updated. Similarly, if @csum_l4 is set the L4
// checksum is updated. If @encap is set then encap header
// checksums are also updated.
//
// IONIC_TXQ_DESC_OPCODE_TSO:
// Device performs TCP segmentation offload
// (TSO).  @hdr_len is the number of bytes
// to the end of TCP header (the offset to
// the TCP payload).  @mss is the desired
// MSS, the TCP payload length for each
// segment.  The device will calculate
// insert IP (IPv4 only) and TCP checksums
// for each segment.  In the first data
// buffer containing the header template,
// the driver will set IPv4 checksum to 0
// and preload TCP checksum with the IP
// pseudo header calculated with IP length = 0.
//
// Supported tunnel encapsulations are IPIP,
// layer-3 GRE, and UDP. @hdr_len includes
// both outer and inner headers.  The driver
// will set IPv4 checksum to zero and
// preload TCP checksum with IP pseudo
// header on the inner header.
//
// TCP ECN offload is supported.  The device
// will set CWR flag in the first segment if
// CWR is set in the template header, and
// clear CWR in remaining segments.
// flags:
// vlan:
// Insert an L2 VLAN header using @vlan_tci
// encap:
// Calculate encap header checksum
// csum_l3:
// Compute L3 header checksum
// csum_l4:
// Compute L4 header checksum
// tso_sot:
// TSO start
// tso_eot:
// TSO end
// num_sg_elems: Number of scatter-gather elements in SG
// descriptor
// addr:       First data buffer's DMA address
// (Subsequent data buffers are on txq_sg_desc)
// @len:          First data buffer's length, in bytes
// @vlan_tci:     VLAN tag to insert in the packet (if requested
// by @V-bit).  Includes .1p and .1q tags
// @hword0:       half word padding
// @hdr_len:      Length of packet headers, including
// encapsulating outer header, if applicable
// Valid for opcodes IONIC_TXQ_DESC_OPCODE_CALC_CSUM and
// IONIC_TXQ_DESC_OPCODE_TSO.  Should be set to zero for
// all other modes.  For
// IONIC_TXQ_DESC_OPCODE_CALC_CSUM, @hdr_len is length
// of headers up to inner-most L4 header.  For
// IONIC_TXQ_DESC_OPCODE_TSO, @hdr_len is up to
// inner-most L4 payload, so inclusive of
// inner-most L4 header.
// @hword1:       half word padding
// @mss:          Desired MSS value for TSO; only applicable for
// IONIC_TXQ_DESC_OPCODE_TSO
// @csum_start:   Offset from packet to first byte checked in L4 checksum
// @csum_offset:  Offset from csum_start to L4 checksum field
// @hword2:       half word padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_txq_desc {
    pub cmd: __le64,
pub const IONIC_TXQ_DESC_OPCODE_MASK: c_uint = 0xf;
pub const IONIC_TXQ_DESC_OPCODE_SHIFT: c_int = 4;
pub const IONIC_TXQ_DESC_FLAGS_MASK: c_uint = 0xf;
pub const IONIC_TXQ_DESC_FLAGS_SHIFT: c_int = 0;
pub const IONIC_TXQ_DESC_NSGE_MASK: c_uint = 0xf;
pub const IONIC_TXQ_DESC_NSGE_SHIFT: c_int = 8;

pub const IONIC_TXQ_DESC_ADDR_SHIFT: c_int = 12;
// common flags
pub const IONIC_TXQ_DESC_FLAG_VLAN: c_uint = 0x1;
pub const IONIC_TXQ_DESC_FLAG_ENCAP: c_uint = 0x2;
// flags for csum_hw opcode
pub const IONIC_TXQ_DESC_FLAG_CSUM_L3: c_uint = 0x4;
pub const IONIC_TXQ_DESC_FLAG_CSUM_L4: c_uint = 0x8;
// flags for tso opcode
pub const IONIC_TXQ_DESC_FLAG_TSO_SOT: c_uint = 0x4;
pub const IONIC_TXQ_DESC_FLAG_TSO_EOT: c_uint = 0x8;
    pub len: __le16,
    pub vlan_tci: __le16,
    pub hword0: __le16,
}

// opcode = (cmd >> IONIC_TXQ_DESC_OPCODE_SHIFT) & IONIC_TXQ_DESC_OPCODE_MASK;
// flags = (cmd >> IONIC_TXQ_DESC_FLAGS_SHIFT) & IONIC_TXQ_DESC_FLAGS_MASK;
// nsge = (cmd >> IONIC_TXQ_DESC_NSGE_SHIFT) & IONIC_TXQ_DESC_NSGE_MASK;
// addr = (cmd >> IONIC_TXQ_DESC_ADDR_SHIFT) & IONIC_TXQ_DESC_ADDR_MASK;
//
// struct ionic_txq_sg_elem - Transmit scatter-gather (SG) descriptor element
// @addr:      DMA address of SG element data buffer
// @len:       Length of SG element data buffer, in bytes
// @rsvd:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_txq_sg_elem {
    pub addr: __le64,
    pub len: __le16,
    pub rsvd: [__le16; 3],
}

//
// struct ionic_txq_sg_desc - Transmit scatter-gather (SG) list
// @elems:     Scatter-gather elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_txq_sg_desc {
pub const IONIC_TX_MAX_SG_ELEMS: c_int = 8;
pub const IONIC_TX_SG_DESC_STRIDE: c_int = 8;
    pub elems: [ionic_txq_sg_elem; IONIC_TX_MAX_SG_ELEMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_txq_sg_desc_v1 {
pub const IONIC_TX_MAX_SG_ELEMS_V1: c_int = 15;
pub const IONIC_TX_SG_DESC_STRIDE_V1: c_int = 16;
    pub elems: [ionic_txq_sg_elem; IONIC_TX_SG_DESC_STRIDE_V1],
}

//
// struct ionic_txq_comp - Ethernet transmit queue completion descriptor
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @rsvd2:      reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_txq_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub rsvd2: [u8; 11],
    pub color: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_rxq_desc_opcode {
    IONIC_RXQ_DESC_OPCODE_SIMPLE = 0,
    IONIC_RXQ_DESC_OPCODE_SG = 1,
}

//
// struct ionic_rxq_desc - Ethernet Rx queue descriptor format
// @opcode:       Rx operation, see IONIC_RXQ_DESC_OPCODE_*:
//
// IONIC_RXQ_DESC_OPCODE_SIMPLE:
// Receive full packet into data buffer
// starting at @addr.  Results of
// receive, including actual bytes received,
// are recorded in Rx completion descriptor.
//
// @rsvd:         reserved byte(s)
// @len:          Data buffer's length, in bytes
// @addr:         Data buffer's DMA address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rxq_desc {
    pub opcode: u8,
    pub rsvd: [u8; 5],
    pub len: __le16,
    pub addr: __le64,
}

//
// struct ionic_rxq_sg_elem - Receive scatter-gather (SG) descriptor element
// @addr:      DMA address of SG element data buffer
// @len:       Length of SG element data buffer, in bytes
// @rsvd:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rxq_sg_elem {
    pub addr: __le64,
    pub len: __le16,
    pub rsvd: [__le16; 3],
}

//
// struct ionic_rxq_sg_desc - Receive scatter-gather (SG) list
// @elems:     Scatter-gather elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rxq_sg_desc {
pub const IONIC_RX_MAX_SG_ELEMS: c_int = 8;
pub const IONIC_RX_SG_DESC_STRIDE: c_int = 8;
    pub elems: [ionic_rxq_sg_elem; IONIC_RX_SG_DESC_STRIDE],
}

//
// struct ionic_rxq_comp - Ethernet receive queue completion descriptor
// @status:       Status of the command (enum ionic_status_code)
// @num_sg_elems: Number of SG elements used by this descriptor
// @comp_index:   Index in the descriptor ring for which this is the completion
// @rss_hash:     32-bit RSS hash
// @csum:         16-bit sum of the packet's L2 payload
// If the packet's L2 payload is odd length, an extra
// zero-value byte is included in the @csum calculation but
// not included in @len.
// @vlan_tci:     VLAN tag stripped from the packet.  Valid if @VLAN is
// set.  Includes .1p and .1q tags.
// @len:          Received packet length, in bytes.  Excludes FCS.
// @csum_calc     L2 payload checksum is computed or not
// @csum_flags:   See IONIC_RXQ_COMP_CSUM_F_*:
//
// IONIC_RXQ_COMP_CSUM_F_TCP_OK:
// The TCP checksum calculated by the device
// matched the checksum in the receive packet's
// TCP header.
//
// IONIC_RXQ_COMP_CSUM_F_TCP_BAD:
// The TCP checksum calculated by the device did
// not match the checksum in the receive packet's
// TCP header.
//
// IONIC_RXQ_COMP_CSUM_F_UDP_OK:
// The UDP checksum calculated by the device
// matched the checksum in the receive packet's
// UDP header
//
// IONIC_RXQ_COMP_CSUM_F_UDP_BAD:
// The UDP checksum calculated by the device did
// not match the checksum in the receive packet's
// UDP header.
//
// IONIC_RXQ_COMP_CSUM_F_IP_OK:
// The IPv4 checksum calculated by the device
// matched the checksum in the receive packet's
// first IPv4 header.  If the receive packet
// contains both a tunnel IPv4 header and a
// transport IPv4 header, the device validates the
// checksum for both IPv4 headers.
//
// IONIC_RXQ_COMP_CSUM_F_IP_BAD:
// The IPv4 checksum calculated by the device did
// not match the checksum in the receive packet's
// first IPv4 header. If the receive packet
// contains both a tunnel IPv4 header and a
// transport IPv4 header, the device validates the
// checksum for both IP headers.
//
// IONIC_RXQ_COMP_CSUM_F_VLAN:
// The VLAN header was stripped and placed in @vlan_tci.
//
// IONIC_RXQ_COMP_CSUM_F_CALC:
// The checksum was calculated by the device.
//
// @pkt_type_color: Packet type and color bit; see IONIC_RXQ_COMP_PKT_TYPE_MASK
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rxq_comp {
    pub status: u8,
    pub num_sg_elems: u8,
    pub comp_index: __le16,
    pub rss_hash: __le32,
    pub csum: __le16,
    pub vlan_tci: __le16,
    pub len: __le16,
    pub csum_flags: u8,
pub const IONIC_RXQ_COMP_CSUM_F_TCP_OK: c_uint = 0x01;
pub const IONIC_RXQ_COMP_CSUM_F_TCP_BAD: c_uint = 0x02;
pub const IONIC_RXQ_COMP_CSUM_F_UDP_OK: c_uint = 0x04;
pub const IONIC_RXQ_COMP_CSUM_F_UDP_BAD: c_uint = 0x08;
pub const IONIC_RXQ_COMP_CSUM_F_IP_OK: c_uint = 0x10;
pub const IONIC_RXQ_COMP_CSUM_F_IP_BAD: c_uint = 0x20;
pub const IONIC_RXQ_COMP_CSUM_F_VLAN: c_uint = 0x40;
pub const IONIC_RXQ_COMP_CSUM_F_CALC: c_uint = 0x80;
    pub pkt_type_color: u8,
pub const IONIC_RXQ_COMP_PKT_TYPE_MASK: c_uint = 0x7f;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_pkt_type {
    IONIC_PKT_TYPE_NON_IP		= 0x00,
    IONIC_PKT_TYPE_IPV4		= 0x01,
    IONIC_PKT_TYPE_IPV4_TCP		= 0x03,
    IONIC_PKT_TYPE_IPV4_UDP		= 0x05,
    IONIC_PKT_TYPE_IPV6		= 0x08,
    IONIC_PKT_TYPE_IPV6_TCP		= 0x18,
    IONIC_PKT_TYPE_IPV6_UDP		= 0x28,
// below types are only used if encap offloads are enabled on lif
    IONIC_PKT_TYPE_ENCAP_NON_IP	= 0x40,
    IONIC_PKT_TYPE_ENCAP_IPV4	= 0x41,
    IONIC_PKT_TYPE_ENCAP_IPV4_TCP	= 0x43,
    IONIC_PKT_TYPE_ENCAP_IPV4_UDP	= 0x45,
    IONIC_PKT_TYPE_ENCAP_IPV6	= 0x48,
    IONIC_PKT_TYPE_ENCAP_IPV6_TCP	= 0x58,
    IONIC_PKT_TYPE_ENCAP_IPV6_UDP	= 0x68,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_eth_hw_features {
    IONIC_ETH_HW_VLAN_TX_TAG	= BIT(0),
    IONIC_ETH_HW_VLAN_RX_STRIP	= BIT(1),
    IONIC_ETH_HW_VLAN_RX_FILTER	= BIT(2),
    IONIC_ETH_HW_RX_HASH		= BIT(3),
    IONIC_ETH_HW_RX_CSUM		= BIT(4),
    IONIC_ETH_HW_TX_SG		= BIT(5),
    IONIC_ETH_HW_RX_SG		= BIT(6),
    IONIC_ETH_HW_TX_CSUM		= BIT(7),
    IONIC_ETH_HW_TSO		= BIT(8),
    IONIC_ETH_HW_TSO_IPV6		= BIT(9),
    IONIC_ETH_HW_TSO_ECN		= BIT(10),
    IONIC_ETH_HW_TSO_GRE		= BIT(11),
    IONIC_ETH_HW_TSO_GRE_CSUM	= BIT(12),
    IONIC_ETH_HW_TSO_IPXIP4		= BIT(13),
    IONIC_ETH_HW_TSO_IPXIP6		= BIT(14),
    IONIC_ETH_HW_TSO_UDP		= BIT(15),
    IONIC_ETH_HW_TSO_UDP_CSUM	= BIT(16),
    IONIC_ETH_HW_RX_CSUM_GENEVE	= BIT(17),
    IONIC_ETH_HW_TX_CSUM_GENEVE	= BIT(18),
    IONIC_ETH_HW_TSO_GENEVE		= BIT(19),
    IONIC_ETH_HW_TIMESTAMP		= BIT(20),
    IONIC_ETH_HW_RDMA_TIMESTAMP	= BIT(21),
}

//
// enum ionic_pkt_class - Packet classification mask.
//
// Used with rx steering filter, packets indicated by the mask can be steered
// toward a specific receive queue.
//
// @IONIC_PKT_CLS_NTP_ALL:          All NTP packets.
// @IONIC_PKT_CLS_PTP1_SYNC:        PTPv1 sync
// @IONIC_PKT_CLS_PTP1_DREQ:        PTPv1 delay-request
// @IONIC_PKT_CLS_PTP1_ALL:         PTPv1 all packets
// @IONIC_PKT_CLS_PTP2_L4_SYNC:     PTPv2-UDP sync
// @IONIC_PKT_CLS_PTP2_L4_DREQ:     PTPv2-UDP delay-request
// @IONIC_PKT_CLS_PTP2_L4_ALL:      PTPv2-UDP all packets
// @IONIC_PKT_CLS_PTP2_L2_SYNC:     PTPv2-ETH sync
// @IONIC_PKT_CLS_PTP2_L2_DREQ:     PTPv2-ETH delay-request
// @IONIC_PKT_CLS_PTP2_L2_ALL:      PTPv2-ETH all packets
// @IONIC_PKT_CLS_PTP2_SYNC:        PTPv2 sync
// @IONIC_PKT_CLS_PTP2_DREQ:        PTPv2 delay-request
// @IONIC_PKT_CLS_PTP2_ALL:         PTPv2 all packets
// @IONIC_PKT_CLS_PTP_SYNC:         PTP sync
// @IONIC_PKT_CLS_PTP_DREQ:         PTP delay-request
// @IONIC_PKT_CLS_PTP_ALL:          PTP all packets
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_pkt_class {
    IONIC_PKT_CLS_NTP_ALL		= BIT(0),

    IONIC_PKT_CLS_PTP1_SYNC		= BIT(1),
    IONIC_PKT_CLS_PTP1_DREQ		= BIT(2),
    IONIC_PKT_CLS_PTP1_ALL		= BIT(3) |
    IONIC_PKT_CLS_PTP1_SYNC | IONIC_PKT_CLS_PTP1_DREQ,

    IONIC_PKT_CLS_PTP2_L4_SYNC	= BIT(4),
    IONIC_PKT_CLS_PTP2_L4_DREQ	= BIT(5),
    IONIC_PKT_CLS_PTP2_L4_ALL	= BIT(6) |
    IONIC_PKT_CLS_PTP2_L4_SYNC | IONIC_PKT_CLS_PTP2_L4_DREQ,

    IONIC_PKT_CLS_PTP2_L2_SYNC	= BIT(7),
    IONIC_PKT_CLS_PTP2_L2_DREQ	= BIT(8),
    IONIC_PKT_CLS_PTP2_L2_ALL	= BIT(9) |
    IONIC_PKT_CLS_PTP2_L2_SYNC | IONIC_PKT_CLS_PTP2_L2_DREQ,

    IONIC_PKT_CLS_PTP2_SYNC		=
    IONIC_PKT_CLS_PTP2_L4_SYNC | IONIC_PKT_CLS_PTP2_L2_SYNC,
    IONIC_PKT_CLS_PTP2_DREQ		=
    IONIC_PKT_CLS_PTP2_L4_DREQ | IONIC_PKT_CLS_PTP2_L2_DREQ,
    IONIC_PKT_CLS_PTP2_ALL		=
    IONIC_PKT_CLS_PTP2_L4_ALL | IONIC_PKT_CLS_PTP2_L2_ALL,

    IONIC_PKT_CLS_PTP_SYNC		=
    IONIC_PKT_CLS_PTP1_SYNC | IONIC_PKT_CLS_PTP2_SYNC,
    IONIC_PKT_CLS_PTP_DREQ		=
    IONIC_PKT_CLS_PTP1_DREQ | IONIC_PKT_CLS_PTP2_DREQ,
    IONIC_PKT_CLS_PTP_ALL		=
    IONIC_PKT_CLS_PTP1_ALL | IONIC_PKT_CLS_PTP2_ALL,
}

//
// struct ionic_q_control_cmd - Queue control command
// @opcode:     opcode
// @type:       Queue type
// @lif_index:  LIF index
// @index:      Queue index
// @oper:       Operation (enum ionic_q_control_oper)
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_q_control_cmd {
    pub opcode: u8,
    pub type: u8,
    pub lif_index: __le16,
    pub index: __le32,
    pub oper: u8,
    pub rsvd: [u8; 55],
}

pub type ionic_q_control_comp = ionic_admin_comp;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_q_control_oper {
    IONIC_Q_DISABLE		= 0,
    IONIC_Q_ENABLE		= 1,
    IONIC_Q_HANG_RESET	= 2,
}

//
// enum ionic_phy_type - Physical connection type
// @IONIC_PHY_TYPE_NONE:    No PHY installed
// @IONIC_PHY_TYPE_COPPER:  Copper PHY
// @IONIC_PHY_TYPE_FIBER:   Fiber PHY
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_phy_type {
    IONIC_PHY_TYPE_NONE	= 0,
    IONIC_PHY_TYPE_COPPER	= 1,
    IONIC_PHY_TYPE_FIBER	= 2,
}

//
// enum ionic_xcvr_state - Transceiver status
// @IONIC_XCVR_STATE_REMOVED:        Transceiver removed
// @IONIC_XCVR_STATE_INSERTED:       Transceiver inserted
// @IONIC_XCVR_STATE_PENDING:        Transceiver pending
// @IONIC_XCVR_STATE_SPROM_READ:     Transceiver data read
// @IONIC_XCVR_STATE_SPROM_READ_ERR: Transceiver data read error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_xcvr_state {
    IONIC_XCVR_STATE_REMOVED	 = 0,
    IONIC_XCVR_STATE_INSERTED	 = 1,
    IONIC_XCVR_STATE_PENDING	 = 2,
    IONIC_XCVR_STATE_SPROM_READ	 = 3,
    IONIC_XCVR_STATE_SPROM_READ_ERR	 = 4,
}

//
// enum ionic_xcvr_pid - Supported link modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_xcvr_pid {
    IONIC_XCVR_PID_UNKNOWN           = 0,

// CU
    IONIC_XCVR_PID_QSFP_100G_CR4     = 1,
    IONIC_XCVR_PID_QSFP_40GBASE_CR4  = 2,
    IONIC_XCVR_PID_SFP_25GBASE_CR_S  = 3,
    IONIC_XCVR_PID_SFP_25GBASE_CR_L  = 4,
    IONIC_XCVR_PID_SFP_25GBASE_CR_N  = 5,
    IONIC_XCVR_PID_QSFP_50G_CR2_FC   = 6,
    IONIC_XCVR_PID_QSFP_50G_CR2      = 7,
    IONIC_XCVR_PID_QSFP_200G_CR4     = 8,
    IONIC_XCVR_PID_QSFP_400G_CR4     = 9,
// Fiber
    IONIC_XCVR_PID_QSFP_100G_AOC    = 50,
    IONIC_XCVR_PID_QSFP_100G_ACC    = 51,
    IONIC_XCVR_PID_QSFP_100G_SR4    = 52,
    IONIC_XCVR_PID_QSFP_100G_LR4    = 53,
    IONIC_XCVR_PID_QSFP_100G_ER4    = 54,
    IONIC_XCVR_PID_QSFP_40GBASE_ER4 = 55,
    IONIC_XCVR_PID_QSFP_40GBASE_SR4 = 56,
    IONIC_XCVR_PID_QSFP_40GBASE_LR4 = 57,
    IONIC_XCVR_PID_QSFP_40GBASE_AOC = 58,
    IONIC_XCVR_PID_SFP_25GBASE_SR   = 59,
    IONIC_XCVR_PID_SFP_25GBASE_LR   = 60,
    IONIC_XCVR_PID_SFP_25GBASE_ER   = 61,
    IONIC_XCVR_PID_SFP_25GBASE_AOC  = 62,
    IONIC_XCVR_PID_SFP_10GBASE_SR   = 63,
    IONIC_XCVR_PID_SFP_10GBASE_LR   = 64,
    IONIC_XCVR_PID_SFP_10GBASE_LRM  = 65,
    IONIC_XCVR_PID_SFP_10GBASE_ER   = 66,
    IONIC_XCVR_PID_SFP_10GBASE_AOC  = 67,
    IONIC_XCVR_PID_SFP_10GBASE_CU   = 68,
    IONIC_XCVR_PID_QSFP_100G_CWDM4  = 69,
    IONIC_XCVR_PID_QSFP_100G_PSM4   = 70,
    IONIC_XCVR_PID_SFP_25GBASE_ACC  = 71,
    IONIC_XCVR_PID_SFP_10GBASE_T    = 72,
    IONIC_XCVR_PID_SFP_1000BASE_T   = 73,
    IONIC_XCVR_PID_QSFP_200G_AOC    = 74,
    IONIC_XCVR_PID_QSFP_200G_FR4    = 75,
    IONIC_XCVR_PID_QSFP_200G_DR4    = 76,
    IONIC_XCVR_PID_QSFP_200G_SR4    = 77,
    IONIC_XCVR_PID_QSFP_200G_ACC    = 78,
    IONIC_XCVR_PID_QSFP_400G_FR4    = 79,
    IONIC_XCVR_PID_QSFP_400G_DR4    = 80,
    IONIC_XCVR_PID_QSFP_400G_SR4    = 81,
    IONIC_XCVR_PID_QSFP_400G_VR4    = 82,
    IONIC_XCVR_PID_QSFP_400G_AOC    = 83,
    IONIC_XCVR_PID_QSFP_400G_AEC    = 84,
    IONIC_XCVR_PID_QSFP_200G_AEC    = 85,
    IONIC_XCVR_PID_QSFP_400G_LPO    = 86,
    IONIC_XCVR_PID_QSFP_100G_FR4    = 87,
    IONIC_XCVR_PID_QSFP_100G_DR4    = 88,
}

//
// enum ionic_port_type - Port types
// @IONIC_PORT_TYPE_NONE:           Port type not configured
// @IONIC_PORT_TYPE_ETH:            Port carries ethernet traffic (inband)
// @IONIC_PORT_TYPE_MGMT:           Port carries mgmt traffic (out-of-band)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_port_type {
    IONIC_PORT_TYPE_NONE = 0,
    IONIC_PORT_TYPE_ETH  = 1,
    IONIC_PORT_TYPE_MGMT = 2,
}

//
// enum ionic_port_admin_state - Port config state
// @IONIC_PORT_ADMIN_STATE_NONE:    Port admin state not configured
// @IONIC_PORT_ADMIN_STATE_DOWN:    Port admin disabled
// @IONIC_PORT_ADMIN_STATE_UP:      Port admin enabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_port_admin_state {
    IONIC_PORT_ADMIN_STATE_NONE = 0,
    IONIC_PORT_ADMIN_STATE_DOWN = 1,
    IONIC_PORT_ADMIN_STATE_UP   = 2,
}

//
// enum ionic_port_oper_status - Port operational status
// @IONIC_PORT_OPER_STATUS_NONE:    Port disabled
// @IONIC_PORT_OPER_STATUS_UP:      Port link status up
// @IONIC_PORT_OPER_STATUS_DOWN:    Port link status down
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_port_oper_status {
    IONIC_PORT_OPER_STATUS_NONE  = 0,
    IONIC_PORT_OPER_STATUS_UP    = 1,
    IONIC_PORT_OPER_STATUS_DOWN  = 2,
}

//
// enum ionic_port_fec_type - Ethernet Forward error correction (FEC) modes
// @IONIC_PORT_FEC_TYPE_NONE:       FEC Disabled
// @IONIC_PORT_FEC_TYPE_FC:         FireCode FEC
// @IONIC_PORT_FEC_TYPE_RS:         ReedSolomon FEC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_port_fec_type {
    IONIC_PORT_FEC_TYPE_NONE = 0,
    IONIC_PORT_FEC_TYPE_FC   = 1,
    IONIC_PORT_FEC_TYPE_RS   = 2,
}

//
// enum ionic_port_pause_type - Ethernet pause (flow control) modes
// @IONIC_PORT_PAUSE_TYPE_NONE:     Disable Pause
// @IONIC_PORT_PAUSE_TYPE_LINK:     Link level pause
// @IONIC_PORT_PAUSE_TYPE_PFC:      Priority-Flow Control
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_port_pause_type {
    IONIC_PORT_PAUSE_TYPE_NONE = 0,
    IONIC_PORT_PAUSE_TYPE_LINK = 1,
    IONIC_PORT_PAUSE_TYPE_PFC  = 2,
}

//
// enum ionic_port_loopback_mode - Loopback modes
// @IONIC_PORT_LOOPBACK_MODE_NONE:  Disable loopback
// @IONIC_PORT_LOOPBACK_MODE_MAC:   MAC loopback
// @IONIC_PORT_LOOPBACK_MODE_PHY:   PHY/SerDes loopback
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_port_loopback_mode {
    IONIC_PORT_LOOPBACK_MODE_NONE = 0,
    IONIC_PORT_LOOPBACK_MODE_MAC  = 1,
    IONIC_PORT_LOOPBACK_MODE_PHY  = 2,
}

//
// struct ionic_xcvr_status - Transceiver Status information
// @state:    Transceiver status (enum ionic_xcvr_state)
// @phy:      Physical connection type (enum ionic_phy_type)
// @pid:      Transceiver link mode (enum ionic_xcvr_pid)
// @sprom:    Transceiver sprom contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_xcvr_status {
    pub state: u8,
    pub phy: u8,
    pub pid: __le16,
    pub sprom: [u8; 256],
}

//
// union ionic_port_config - Port configuration
// @speed:              port speed (in Mbps)
// @mtu:                mtu
// @state:              port admin state (enum ionic_port_admin_state)
// @an_enable:          autoneg enable
// @fec_type:           fec type (enum ionic_port_fec_type)
// @pause_type:         pause type (enum ionic_port_pause_type)
// @loopback_mode:      loopback mode (enum ionic_port_loopback_mode)
// @words:              word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_port_config {

    pub speed: __le32,
    pub mtu: __le32,
    pub state: u8,
    pub an_enable: u8,
    pub fec_type: u8,
pub const IONIC_PAUSE_TYPE_MASK: c_uint = 0x0f;
pub const IONIC_PAUSE_FLAGS_MASK: c_uint = 0xf0;
pub const IONIC_PAUSE_F_TX: c_uint = 0x10;
pub const IONIC_PAUSE_F_RX: c_uint = 0x20;
    pub pause_type: u8,
    pub loopback_mode: u8,
}

//
// struct ionic_port_status - Port Status information
// @status:             link status (enum ionic_port_oper_status)
// @id:                 port id
// @speed:              link speed (in Mbps)
// @link_down_count:    number of times link went from up to down
// @fec_type:           fec type (enum ionic_port_fec_type)
// @rsvd:               reserved byte(s)
// @xcvr:               transceiver status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_status {
    pub id: __le32,
    pub speed: __le32,
    pub status: u8,
    pub link_down_count: __le16,
    pub fec_type: u8,
    pub rsvd: [u8; 48],
    pub xcvr: ionic_xcvr_status,
    pub __packed: },
//
// struct ionic_port_identify_cmd - Port identify command
// @opcode:     opcode
// @index:      port index
// @ver:        Highest version of identify supported by driver
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_identify_cmd {
    pub opcode: u8,
    pub index: u8,
    pub ver: u8,
    pub rsvd: [u8; 61],
}

//
// struct ionic_port_identify_comp - Port identify command completion
// @status: Status of the command (enum ionic_status_code)
// @ver:    Version of identify returned by device
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_identify_comp {
    pub status: u8,
    pub ver: u8,
    pub rsvd: [u8; 14],
}

//
// struct ionic_port_init_cmd - Port initialization command
// @opcode:     opcode
// @index:      port index
// @rsvd:       reserved byte(s)
// @info_pa:    destination address for port info (struct ionic_port_info)
// @rsvd2:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_init_cmd {
    pub opcode: u8,
    pub index: u8,
    pub rsvd: [u8; 6],
    pub info_pa: __le64,
    pub rsvd2: [u8; 48],
}

//
// struct ionic_port_init_comp - Port initialization command completion
// @status: Status of the command (enum ionic_status_code)
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_init_comp {
    pub status: u8,
    pub rsvd: [u8; 15],
}

//
// struct ionic_port_reset_cmd - Port reset command
// @opcode:     opcode
// @index:      port index
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_reset_cmd {
    pub opcode: u8,
    pub index: u8,
    pub rsvd: [u8; 62],
}

//
// struct ionic_port_reset_comp - Port reset command completion
// @status: Status of the command (enum ionic_status_code)
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_reset_comp {
    pub status: u8,
    pub rsvd: [u8; 15],
}

//
// enum ionic_stats_ctl_cmd - List of commands for stats control
// @IONIC_STATS_CTL_RESET:      Reset statistics
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_stats_ctl_cmd {
    IONIC_STATS_CTL_RESET		= 0,
}

//
// enum ionic_txstamp_mode - List of TX Timestamping Modes
// @IONIC_TXSTAMP_OFF:           Disable TX hardware timetamping.
// @IONIC_TXSTAMP_ON:            Enable local TX hardware timetamping.
// @IONIC_TXSTAMP_ONESTEP_SYNC:  Modify TX PTP Sync packets.
// @IONIC_TXSTAMP_ONESTEP_P2P:   Modify TX PTP Sync and PDelayResp.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_txstamp_mode {
    IONIC_TXSTAMP_OFF		= 0,
    IONIC_TXSTAMP_ON		= 1,
    IONIC_TXSTAMP_ONESTEP_SYNC	= 2,
    IONIC_TXSTAMP_ONESTEP_P2P	= 3,
}

//
// enum ionic_port_attr - List of device attributes
// @IONIC_PORT_ATTR_STATE:      Port state attribute
// @IONIC_PORT_ATTR_SPEED:      Port speed attribute
// @IONIC_PORT_ATTR_MTU:        Port MTU attribute
// @IONIC_PORT_ATTR_AUTONEG:    Port autonegotiation attribute
// @IONIC_PORT_ATTR_FEC:        Port FEC attribute
// @IONIC_PORT_ATTR_PAUSE:      Port pause attribute
// @IONIC_PORT_ATTR_LOOPBACK:   Port loopback attribute
// @IONIC_PORT_ATTR_STATS_CTRL: Port statistics control attribute
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_port_attr {
    IONIC_PORT_ATTR_STATE		= 0,
    IONIC_PORT_ATTR_SPEED		= 1,
    IONIC_PORT_ATTR_MTU		= 2,
    IONIC_PORT_ATTR_AUTONEG		= 3,
    IONIC_PORT_ATTR_FEC		= 4,
    IONIC_PORT_ATTR_PAUSE		= 5,
    IONIC_PORT_ATTR_LOOPBACK	= 6,
    IONIC_PORT_ATTR_STATS_CTRL	= 7,
}

//
// struct ionic_port_setattr_cmd - Set port attributes on the NIC
// @opcode:         Opcode
// @index:          Port index
// @attr:           Attribute type (enum ionic_port_attr)
// @rsvd:           reserved byte(s)
// @state:          Port state
// @speed:          Port speed
// @mtu:            Port MTU
// @an_enable:      Port autonegotiation setting
// @fec_type:       Port FEC type setting
// @pause_type:     Port pause type setting
// @loopback_mode:  Port loopback mode
// @stats_ctl:      Port stats setting
// @rsvd2:          reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_setattr_cmd {
    pub opcode: u8,
    pub index: u8,
    pub attr: u8,
    pub rsvd: u8,
    pub state: u8,
    pub speed: __le32,
    pub mtu: __le32,
    pub an_enable: u8,
    pub fec_type: u8,
    pub pause_type: u8,
    pub loopback_mode: u8,
    pub stats_ctl: u8,
    pub rsvd2: [u8; 60],
}

//
// struct ionic_port_setattr_comp - Port set attr command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_setattr_comp {
    pub status: u8,
    pub rsvd: [u8; 14],
    pub color: u8,
}

//
// struct ionic_port_getattr_cmd - Get port attributes from the NIC
// @opcode:     Opcode
// @index:      port index
// @attr:       Attribute type (enum ionic_port_attr)
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_getattr_cmd {
    pub opcode: u8,
    pub index: u8,
    pub attr: u8,
    pub rsvd: [u8; 61],
}

//
// struct ionic_port_getattr_comp - Port get attr command completion
// @status:         Status of the command (enum ionic_status_code)
// @rsvd:           reserved byte(s)
// @state:          Port state
// @speed:          Port speed
// @mtu:            Port MTU
// @an_enable:      Port autonegotiation setting
// @fec_type:       Port FEC type setting
// @pause_type:     Port pause type setting
// @loopback_mode:  Port loopback mode
// @rsvd2:          reserved byte(s)
// @color:          Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_getattr_comp {
    pub status: u8,
    pub rsvd: [u8; 3],
    pub state: u8,
    pub speed: __le32,
    pub mtu: __le32,
    pub an_enable: u8,
    pub fec_type: u8,
    pub pause_type: u8,
    pub loopback_mode: u8,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

//
// struct ionic_lif_status - LIF status register
// @eid:             most recent NotifyQ event id
// @port_num:        port the LIF is connected to
// @rsvd:            reserved byte(s)
// @link_status:     port status (enum ionic_port_oper_status)
// @link_speed:      speed of link in Mbps
// @link_down_count: number of times link went from up to down
// @rsvd2:           reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_status {
    pub eid: __le64,
    pub port_num: u8,
    pub rsvd: u8,
    pub link_status: __le16,
    pub /: *mut *mut __le32 link_speed; / units of 1Mbps: eg 10000 = 10Gbps,
    pub link_down_count: __le16,
    pub rsvd2: [u8; 46],
}

//
// struct ionic_lif_reset_cmd - LIF reset command
// @opcode:    opcode
// @rsvd:      reserved byte(s)
// @index:     LIF index
// @rsvd2:     reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_reset_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub index: __le16,
    pub rsvd2: [__le32; 15],
}

pub type ionic_lif_reset_comp = ionic_admin_comp;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_dev_state {
    IONIC_DEV_DISABLE	= 0,
    IONIC_DEV_ENABLE	= 1,
    IONIC_DEV_HANG_RESET	= 2,
}

//
// enum ionic_dev_attr - List of device attributes
// @IONIC_DEV_ATTR_STATE:     Device state attribute
// @IONIC_DEV_ATTR_NAME:      Device name attribute
// @IONIC_DEV_ATTR_FEATURES:  Device feature attributes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_dev_attr {
    IONIC_DEV_ATTR_STATE    = 0,
    IONIC_DEV_ATTR_NAME     = 1,
    IONIC_DEV_ATTR_FEATURES = 2,
}

//
// struct ionic_dev_setattr_cmd - Set Device attributes on the NIC
// @opcode:     Opcode
// @attr:       Attribute type (enum ionic_dev_attr)
// @rsvd:       reserved byte(s)
// @state:      Device state (enum ionic_dev_state)
// @name:       The bus info, e.g. PCI slot-device-function, 0 terminated
// @features:   Device features
// @rsvd2:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_setattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub rsvd: __le16,
    pub state: u8,
    pub name: [c_char; IONIC_IFNAMSIZ],
    pub features: __le64,
    pub rsvd2: [u8; 60],
    pub __packed: },
}

//
// struct ionic_dev_setattr_comp - Device set attr command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @features:   Device features
// @rsvd2:      reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_setattr_comp {
    pub status: u8,
    pub rsvd: [u8; 3],
    pub features: __le64,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

//
// struct ionic_dev_getattr_cmd - Get Device attributes from the NIC
// @opcode:     opcode
// @attr:       Attribute type (enum ionic_dev_attr)
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_getattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub rsvd: [u8; 62],
}

//
// struct ionic_dev_getattr_comp - Device set attr command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @features:   Device features
// @rsvd2:      reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_getattr_comp {
    pub status: u8,
    pub rsvd: [u8; 3],
    pub features: __le64,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

//
// RSS parameters
//
pub const IONIC_RSS_HASH_KEY_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_rss_hash_types {
    IONIC_RSS_TYPE_IPV4	= BIT(0),
    IONIC_RSS_TYPE_IPV4_TCP	= BIT(1),
    IONIC_RSS_TYPE_IPV4_UDP	= BIT(2),
    IONIC_RSS_TYPE_IPV6	= BIT(3),
    IONIC_RSS_TYPE_IPV6_TCP	= BIT(4),
    IONIC_RSS_TYPE_IPV6_UDP	= BIT(5),
}

//
// enum ionic_lif_attr - List of LIF attributes
// @IONIC_LIF_ATTR_STATE:       LIF state attribute
// @IONIC_LIF_ATTR_NAME:        LIF name attribute
// @IONIC_LIF_ATTR_MTU:         LIF MTU attribute
// @IONIC_LIF_ATTR_MAC:         LIF MAC attribute
// @IONIC_LIF_ATTR_FEATURES:    LIF features attribute
// @IONIC_LIF_ATTR_RSS:         LIF RSS attribute
// @IONIC_LIF_ATTR_STATS_CTRL:  LIF statistics control attribute
// @IONIC_LIF_ATTR_TXSTAMP:     LIF TX timestamping mode
// @IONIC_LIF_ATTR_MAX:         maximum attribute value
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_lif_attr {
    IONIC_LIF_ATTR_STATE        = 0,
    IONIC_LIF_ATTR_NAME         = 1,
    IONIC_LIF_ATTR_MTU          = 2,
    IONIC_LIF_ATTR_MAC          = 3,
    IONIC_LIF_ATTR_FEATURES     = 4,
    IONIC_LIF_ATTR_RSS          = 5,
    IONIC_LIF_ATTR_STATS_CTRL   = 6,
    IONIC_LIF_ATTR_TXSTAMP      = 7,
    IONIC_LIF_ATTR_MAX          = 255,
}

//
// struct ionic_lif_setattr_cmd - Set LIF attributes on the NIC
// @opcode:     Opcode
// @attr:       Attribute type (enum ionic_lif_attr)
// @index:      LIF index
// @state:      LIF state (enum ionic_lif_state)
// @name:       The netdev name string, 0 terminated
// @mtu:        Mtu
// @mac:        Station mac
// @features:   Features (enum ionic_eth_hw_features)
// @rss:        RSS properties
// @rss.types:     The hash types to enable (see rss_hash_types)
// @rss.key:       The hash secret key
// @rss.rsvd:      reserved byte(s)
// @rss.addr:      Address for the indirection table shared memory
// @stats_ctl:  stats control commands (enum ionic_stats_ctl_cmd)
// @txstamp_mode:    TX Timestamping Mode (enum ionic_txstamp_mode)
// @rsvd:        reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_setattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub index: __le16,
    pub state: u8,
    pub name: [c_char; IONIC_IFNAMSIZ],
    pub mtu: __le32,
    pub mac: [u8; 6],
    pub features: __le64,
    pub types: __le16,
    pub key: [u8; IONIC_RSS_HASH_KEY_SIZE],
    pub rsvd: [u8; 6],
    pub addr: __le64,
    pub rss: },
    pub stats_ctl: u8,
    pub txstamp_mode: __le16,
    pub rsvd: [u8; 60],
    pub __packed: },
}

//
// struct ionic_lif_setattr_comp - LIF set attr command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @features:   features (enum ionic_eth_hw_features)
// @rsvd2:      reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_setattr_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub features: __le64,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

//
// struct ionic_lif_getattr_cmd - Get LIF attributes from the NIC
// @opcode:     Opcode
// @attr:       Attribute type (enum ionic_lif_attr)
// @index:      LIF index
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_getattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub index: __le16,
    pub rsvd: [u8; 60],
}

//
// struct ionic_lif_getattr_comp - LIF get attr command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @state:      LIF state (enum ionic_lif_state)
// @mtu:        Mtu
// @mac:        Station mac
// @features:   Features (enum ionic_eth_hw_features)
// @txstamp_mode:    TX Timestamping Mode (enum ionic_txstamp_mode)
// @rsvd2:      reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_getattr_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub state: u8,
    pub mtu: __le32,
    pub mac: [u8; 6],
    pub features: __le64,
    pub txstamp_mode: __le16,
    pub rsvd2: [u8; 11],
    pub __packed: },
    pub color: u8,
}

//
// struct ionic_lif_setphc_cmd - Set LIF PTP Hardware Clock
// @opcode:     Opcode
// @rsvd1:      reserved byte(s)
// @lif_index:  LIF index
// @rsvd2:      reserved byte(s)
// @tick:       Hardware stamp tick of an instant in time.
// @nsec:       Nanosecond stamp of the same instant.
// @frac:       Fractional nanoseconds at the same instant.
// @mult:       Cycle to nanosecond multiplier.
// @shift:      Cycle to nanosecond divisor (power of two).
// @rsvd3:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_setphc_cmd {
    pub opcode: u8,
    pub rsvd1: u8,
    pub lif_index: __le16,
    pub rsvd2: [u8; 4],
    pub tick: __le64,
    pub nsec: __le64,
    pub frac: __le64,
    pub mult: __le32,
    pub shift: __le32,
    pub rsvd3: [u8; 24],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_rx_mode {
    IONIC_RX_MODE_F_UNICAST		= BIT(0),
    IONIC_RX_MODE_F_MULTICAST	= BIT(1),
    IONIC_RX_MODE_F_BROADCAST	= BIT(2),
    IONIC_RX_MODE_F_PROMISC		= BIT(3),
    IONIC_RX_MODE_F_ALLMULTI	= BIT(4),
    IONIC_RX_MODE_F_RDMA_SNIFFER	= BIT(5),
}

//
// struct ionic_rx_mode_set_cmd - Set LIF's Rx mode command
// @opcode:     opcode
// @rsvd:       reserved byte(s)
// @lif_index:  LIF index
// @rx_mode:    Rx mode flags:
// IONIC_RX_MODE_F_UNICAST: Accept known unicast packets
// IONIC_RX_MODE_F_MULTICAST: Accept known multicast packets
// IONIC_RX_MODE_F_BROADCAST: Accept broadcast packets
// IONIC_RX_MODE_F_PROMISC: Accept any packets
// IONIC_RX_MODE_F_ALLMULTI: Accept any multicast packets
// IONIC_RX_MODE_F_RDMA_SNIFFER: Sniff RDMA packets
// @rsvd2:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_mode_set_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub lif_index: __le16,
    pub rx_mode: __le16,
    pub rsvd2: [__le16; 29],
}

pub type ionic_rx_mode_set_comp = ionic_admin_comp;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_rx_filter_match_type {
    IONIC_RX_FILTER_MATCH_VLAN	= 0x0,
    IONIC_RX_FILTER_MATCH_MAC	= 0x1,
    IONIC_RX_FILTER_MATCH_MAC_VLAN	= 0x2,
    IONIC_RX_FILTER_STEER_PKTCLASS	= 0x10,
}

//
// struct ionic_rx_filter_add_cmd - Add LIF Rx filter command
// @opcode:     opcode
// @qtype:      Queue type
// @lif_index:  LIF index
// @qid:        Queue ID
// @match:      Rx filter match type (see IONIC_RX_FILTER_MATCH_xxx)
// @vlan:       VLAN filter
// @vlan.vlan:  VLAN ID
// @mac:        MAC filter
// @mac.addr:  MAC address (network-byte order)
// @mac_vlan:   MACVLAN filter
// @mac_vlan.vlan:  VLAN ID
// @mac_vlan.addr:  MAC address (network-byte order)
// @pkt_class:  Packet classification filter
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_filter_add_cmd {
    pub opcode: u8,
    pub qtype: u8,
    pub lif_index: __le16,
    pub qid: __le32,
    pub match: __le16,
    pub vlan: __le16,
    pub vlan: },
    pub addr: [u8; 6],
    pub mac: },
    pub vlan: __le16,
    pub addr: [u8; 6],
    pub mac_vlan: },
    pub pkt_class: __le64,
    pub rsvd: [u8; 54],
    pub __packed: },
}

//
// struct ionic_rx_filter_add_comp - Add LIF Rx filter command completion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @filter_id:  Filter ID
// @rsvd2:      reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_filter_add_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub filter_id: __le32,
    pub rsvd2: [u8; 7],
    pub color: u8,
}

//
// struct ionic_rx_filter_del_cmd - Delete LIF Rx filter command
// @opcode:     opcode
// @rsvd:       reserved byte(s)
// @lif_index:  LIF index
// @filter_id:  Filter ID
// @rsvd2:      reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_filter_del_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub lif_index: __le16,
    pub filter_id: __le32,
    pub rsvd2: [u8; 56],
}

pub type ionic_rx_filter_del_comp = ionic_admin_comp;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_vf_attr {
    IONIC_VF_ATTR_SPOOFCHK	= 1,
    IONIC_VF_ATTR_TRUST	= 2,
    IONIC_VF_ATTR_MAC	= 3,
    IONIC_VF_ATTR_LINKSTATE	= 4,
    IONIC_VF_ATTR_VLAN	= 5,
    IONIC_VF_ATTR_RATE	= 6,
    IONIC_VF_ATTR_STATSADDR	= 7,
}

//
// enum ionic_vf_link_status - Virtual Function link status
// @IONIC_VF_LINK_STATUS_AUTO:   Use link state of the uplink
// @IONIC_VF_LINK_STATUS_UP:     Link always up
// @IONIC_VF_LINK_STATUS_DOWN:   Link always down
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_vf_link_status {
    IONIC_VF_LINK_STATUS_AUTO = 0,
    IONIC_VF_LINK_STATUS_UP   = 1,
    IONIC_VF_LINK_STATUS_DOWN = 2,
}

//
// struct ionic_vf_setattr_cmd - Set VF attributes on the NIC
// @opcode:     Opcode
// @attr:       Attribute type (enum ionic_vf_attr)
// @vf_index:   VF index
// @macaddr:	mac address
// @vlanid:	vlan ID
// @maxrate:	max Tx rate in Mbps
// @spoofchk:	enable address spoof checking
// @trust:		enable VF trust
// @linkstate:	set link up or down
// @stats_pa:	set DMA address for VF stats
// @pad:           reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vf_setattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub vf_index: __le16,
    pub macaddr: [u8; 6],
    pub vlanid: __le16,
    pub maxrate: __le32,
    pub spoofchk: u8,
    pub trust: u8,
    pub linkstate: u8,
    pub stats_pa: __le64,
    pub pad: [u8; 60],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vf_setattr_comp {
    pub status: u8,
    pub attr: u8,
    pub vf_index: __le16,
    pub comp_index: __le16,
    pub rsvd: [u8; 9],
    pub color: u8,
}

//
// struct ionic_vf_getattr_cmd - Get VF attributes from the NIC
// @opcode:     Opcode
// @attr:       Attribute type (enum ionic_vf_attr)
// @vf_index:   VF index
// @rsvd:       reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vf_getattr_cmd {
    pub opcode: u8,
    pub attr: u8,
    pub vf_index: __le16,
    pub rsvd: [u8; 60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vf_getattr_comp {
    pub status: u8,
    pub attr: u8,
    pub vf_index: __le16,
    pub macaddr: [u8; 6],
    pub vlanid: __le16,
    pub maxrate: __le32,
    pub spoofchk: u8,
    pub trust: u8,
    pub linkstate: u8,
    pub stats_pa: __le64,
    pub pad: [u8; 11],
    pub __packed: },
    pub color: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_vf_ctrl_opcode {
    IONIC_VF_CTRL_START_ALL	= 0,
    IONIC_VF_CTRL_START	= 1,
}

//
// struct ionic_vf_ctrl_cmd - VF control command
// @opcode:         Opcode for the command
// @ctrl_opcode:    VF control operation type
// @vf_index:       VF Index. It is unused if op START_ALL is used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vf_ctrl_cmd {
    pub opcode: u8,
    pub ctrl_opcode: u8,
    pub vf_index: __le16,
// private:
    pub rsvd1: [u8; 60],
}

//
// struct ionic_vf_ctrl_comp - VF_CTRL command completion.
// @status:     Status of the command (enum ionic_status_code)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vf_ctrl_comp {
    pub status: u8,
// private:
    pub rsvd: [u8; 15],
}

//
// struct ionic_discover_cmb_cmd - CMB discovery command
// @opcode: Opcode for the command
// @rsvd:   Reserved bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_discover_cmb_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 63],
}

//
// struct ionic_discover_cmb_comp - CMB discover command completion.
// @status: Status of the command (enum ionic_status_code)
// @rsvd:   Reserved bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_discover_cmb_comp {
    pub status: u8,
    pub rsvd: [u8; 15],
}

pub const IONIC_MAX_CMB_REGIONS: c_int = 16;
pub const IONIC_CMB_SHIFT_64K: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_cmb_type {
    IONIC_CMB_TYPE_DEVMEM	= 0,
    IONIC_CMB_TYPE_EXPDB64	= 1,
    IONIC_CMB_TYPE_EXPDB128	= 2,
    IONIC_CMB_TYPE_EXPDB256	= 3,
    IONIC_CMB_TYPE_EXPDB512	= 4,
}

//
// union ionic_cmb_region - Configuration for CMB region
// @bar_num:	CMB mapping number from FW
// @cmb_type:	Type of CMB this region describes (enum ionic_cmb_type)
// @rsvd:	Reserved
// @offset:	Offset within BAR in 64KB pages
// @length:	Length of the CMB region
// @words:	32-bit words for direct access to the entire region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_cmb_region {
    pub bar_num: u8,
    pub cmb_type: u8,
    pub rsvd: [u8; 6],
    pub offset: __le32,
    pub length: __le32,
    pub __packed: },
    pub words: [__le32; 4],
}

//
// union ionic_discover_cmb_identity - CMB layout identity structure
// @num_regions:    Number of CMB regions, up to 16
// @flags:          Feature and capability bits (0 for express
// doorbell, 1 for 4K alignment indicator,
// 31-24 for version information)
// @region:         CMB mappings region, entry 0 for regular
// mapping, entries 1-7 for WQE sizes 64,
// 128, 256, 512, 1024, 2048 and 4096 bytes
// @words:          Full union buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_discover_cmb_identity {
    pub num_regions: __le32,

pub const IONIC_CMB_FLAG_VERSION: c_uint = 0xff000000;
    pub flags: __le32,
    pub region: [ionic_cmb_region; IONIC_MAX_CMB_REGIONS],
}

//
// struct ionic_qos_identify_cmd - QoS identify command
// @opcode:  opcode
// @ver:     Highest version of identify supported by driver
// @rsvd:    reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qos_identify_cmd {
    pub opcode: u8,
    pub ver: u8,
    pub rsvd: [u8; 62],
}

//
// struct ionic_qos_identify_comp - QoS identify command completion
// @status: Status of the command (enum ionic_status_code)
// @ver:    Version of identify returned by device
// @rsvd:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qos_identify_comp {
    pub status: u8,
    pub ver: u8,
    pub rsvd: [u8; 14],
}

pub const IONIC_QOS_TC_MAX: c_int = 8;
pub const IONIC_QOS_ALL_TC: c_uint = 0xFF;
// Capri max supported, should be renamed.
pub const IONIC_QOS_CLASS_MAX: c_int = 7;
pub const IONIC_QOS_PCP_MAX: c_int = 8;
pub const IONIC_QOS_CLASS_NAME_SZ: c_int = 32;
pub const IONIC_QOS_DSCP_MAX: c_int = 64;
pub const IONIC_QOS_ALL_PCP: c_uint = 0xFF;
pub const IONIC_DSCP_BLOCK_SIZE: c_int = 8;
//
// enum ionic_qos_class
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_qos_class {
    IONIC_QOS_CLASS_DEFAULT		= 0,
    IONIC_QOS_CLASS_USER_DEFINED_1	= 1,
    IONIC_QOS_CLASS_USER_DEFINED_2	= 2,
    IONIC_QOS_CLASS_USER_DEFINED_3	= 3,
    IONIC_QOS_CLASS_USER_DEFINED_4	= 4,
    IONIC_QOS_CLASS_USER_DEFINED_5	= 5,
    IONIC_QOS_CLASS_USER_DEFINED_6	= 6,
}

//
// enum ionic_qos_class_type - Traffic classification criteria
// @IONIC_QOS_CLASS_TYPE_NONE:    No QoS
// @IONIC_QOS_CLASS_TYPE_PCP:     Dot1Q PCP
// @IONIC_QOS_CLASS_TYPE_DSCP:    IP DSCP
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_qos_class_type {
    IONIC_QOS_CLASS_TYPE_NONE	= 0,
    IONIC_QOS_CLASS_TYPE_PCP	= 1,
    IONIC_QOS_CLASS_TYPE_DSCP	= 2,
}

//
// enum ionic_qos_sched_type - QoS class scheduling type
// @IONIC_QOS_SCHED_TYPE_STRICT:  Strict priority
// @IONIC_QOS_SCHED_TYPE_DWRR:    Deficit weighted round-robin
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_qos_sched_type {
    IONIC_QOS_SCHED_TYPE_STRICT	= 0,
    IONIC_QOS_SCHED_TYPE_DWRR	= 1,
}

//
// union ionic_qos_config - QoS configuration structure
// @flags:		Configuration flags
// IONIC_QOS_CONFIG_F_ENABLE		enable
// IONIC_QOS_CONFIG_F_NO_DROP		drop/nodrop
// IONIC_QOS_CONFIG_F_RW_DOT1Q_PCP		enable dot1q pcp rewrite
// IONIC_QOS_CONFIG_F_RW_IP_DSCP		enable ip dscp rewrite
// IONIC_QOS_CONFIG_F_NON_DISRUPTIVE	Non-disruptive TC update
// @sched_type:		QoS class scheduling type (enum ionic_qos_sched_type)
// @class_type:		QoS class type (enum ionic_qos_class_type)
// @pause_type:		QoS pause type (enum ionic_qos_pause_type)
// @name:		QoS class name
// @mtu:		MTU of the class
// @pfc_cos:		Priority-Flow Control class of service
// @dwrr_weight:	QoS class scheduling weight
// @strict_rlmt:	Rate limit for strict priority scheduling
// @rw_dot1q_pcp:	Rewrite dot1q pcp to value (valid iff F_RW_DOT1Q_PCP)
// @rw_ip_dscp:		Rewrite ip dscp to value (valid iff F_RW_IP_DSCP)
// @dot1q_pcp:		Dot1q pcp value
// @ndscp:		Number of valid dscp values in the ip_dscp field
// @ip_dscp:		IP dscp values
// @words:		word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_qos_config {

// Used to rewrite PCP or DSCP value.

// Non-disruptive TC update

    pub flags: u8,
    pub sched_type: u8,
    pub class_type: u8,
    pub pause_type: u8,
    pub name: [c_char; IONIC_QOS_CLASS_NAME_SZ],
    pub mtu: __le32,
// flow control
    pub pfc_cos: u8,
// scheduler
    pub dwrr_weight: u8,
    pub strict_rlmt: __le64,
}

// marking
// Used to rewrite PCP or DSCP value.
// classification
//
// union ionic_qos_identity - QoS identity structure
// @version:	Version of the identify structure
// @type:	QoS system type
// @rsvd:	reserved byte(s)
// @config:	Current configuration of classes
// @words:	word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_qos_identity {
    pub version: u8,
    pub type: u8,
    pub rsvd: [u8; 62],
    pub config: [ionic_qos_config; IONIC_QOS_CLASS_MAX],
}

//
// struct ionic_qos_init_cmd - QoS config init command
// @opcode:	Opcode
// @group:	QoS class id
// @rsvd:	reserved byte(s)
// @info_pa:	destination address for qos info
// @rsvd1:	reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qos_init_cmd {
    pub opcode: u8,
    pub group: u8,
    pub rsvd: [u8; 6],
    pub info_pa: __le64,
    pub rsvd1: [u8; 48],
}

pub type ionic_qos_init_comp = ionic_admin_comp;
//
// struct ionic_qos_reset_cmd - QoS config reset command
// @opcode:	Opcode
// @group:	QoS class id
// @rsvd:	reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qos_reset_cmd {
    pub opcode: u8,
    pub group: u8,
    pub rsvd: [u8; 62],
}

//
// struct ionic_qos_clear_stats_cmd - Qos config reset command
// @opcode:	Opcode
// @group_bitmap: bitmap of groups to be cleared
// @rsvd:	reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qos_clear_stats_cmd {
    pub opcode: u8,
    pub group_bitmap: u8,
    pub rsvd: [u8; 62],
}

pub type ionic_qos_reset_comp = ionic_admin_comp;
//
// struct ionic_fw_download_cmd - Firmware download command
// @opcode:	opcode
// @rsvd:	reserved byte(s)
// @addr:	dma address of the firmware buffer
// @offset:	offset of the firmware buffer within the full image
// @length:	number of valid bytes in the firmware buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_fw_download_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub offset: __le32,
    pub addr: __le64,
    pub length: __le32,
}

pub type ionic_fw_download_comp = ionic_admin_comp;
//
// enum ionic_fw_control_oper - FW control operations
// @IONIC_FW_RESET:		Reset firmware
// @IONIC_FW_INSTALL:		Install firmware
// @IONIC_FW_ACTIVATE:		Activate firmware
// @IONIC_FW_INSTALL_ASYNC:	Install firmware asynchronously
// @IONIC_FW_INSTALL_STATUS:	Firmware installation status
// @IONIC_FW_ACTIVATE_ASYNC:	Activate firmware asynchronously
// @IONIC_FW_ACTIVATE_STATUS:	Firmware activate status
// @IONIC_FW_UPDATE_CLEANUP:	Clean up after an interrupted fw update
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_fw_control_oper {
    IONIC_FW_RESET			= 0,
    IONIC_FW_INSTALL		= 1,
    IONIC_FW_ACTIVATE		= 2,
    IONIC_FW_INSTALL_ASYNC		= 3,
    IONIC_FW_INSTALL_STATUS		= 4,
    IONIC_FW_ACTIVATE_ASYNC		= 5,
    IONIC_FW_ACTIVATE_STATUS	= 6,
    IONIC_FW_UPDATE_CLEANUP		= 7,
}

//
// struct ionic_fw_control_cmd - Firmware control command
// @opcode:    opcode
// @rsvd:      reserved byte(s)
// @oper:      firmware control operation (enum ionic_fw_control_oper)
// @slot:      slot to activate
// @rsvd1:     reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_fw_control_cmd {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub oper: u8,
    pub slot: u8,
    pub rsvd1: [u8; 58],
}

//
// struct ionic_fw_control_comp - Firmware control copletion
// @status:     Status of the command (enum ionic_status_code)
// @rsvd:       reserved byte(s)
// @comp_index: Index in the descriptor ring for which this is the completion
// @slot:       Slot where the firmware was installed
// @rsvd1:      reserved byte(s)
// @color:      Color bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_fw_control_comp {
    pub status: u8,
    pub rsvd: u8,
    pub comp_index: __le16,
    pub slot: u8,
    pub rsvd1: [u8; 10],
    pub color: u8,
}

//
// RDMA Commands
//
// struct ionic_rdma_reset_cmd - Reset RDMA LIF cmd
// @opcode:        opcode
// @rsvd:          reserved byte(s)
// @lif_index:     LIF index
// @rsvd2:         reserved byte(s)
//
// There is no RDMA specific dev command completion struct.  Completion uses
// the common struct ionic_admin_comp.  Only the status is indicated.
// Nonzero status means the LIF does not support RDMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rdma_reset_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub lif_index: __le16,
    pub rsvd2: [u8; 60],
}

//
// struct ionic_rdma_queue_cmd - Create RDMA Queue command
// @opcode:        opcode, 52, 53
// @rsvd:          reserved byte(s)
// @lif_index:     LIF index
// @qid_ver:       (qid | (RDMA version << 24))
// @cid:           intr, eq_id, or cq_id
// @dbid:          doorbell page id
// @depth_log2:    log base two of queue depth
// @stride_log2:   log base two of queue stride
// @dma_addr:      address of the queue memory
// @rsvd2:         reserved byte(s)
//
// The same command struct is used to create an RDMA event queue, completion
// queue, or RDMA admin queue.  The cid is an interrupt number for an event
// queue, an event queue id for a completion queue, or a completion queue id
// for an RDMA admin queue.
//
// The queue created via a dev command must be contiguous in dma space.
//
// The dev commands are intended only to be used during driver initialization,
// to create queues supporting the RDMA admin queue.  Other queues, and other
// types of RDMA resources like memory regions, will be created and registered
// via the RDMA admin queue, and will support a more complete interface
// providing scatter gather lists for larger, scattered queue buffers and
// memory registration.
//
// There is no RDMA specific dev command completion struct.  Completion uses
// the common struct ionic_admin_comp.  Only the status is indicated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rdma_queue_cmd {
    pub opcode: u8,
    pub rsvd: u8,
    pub lif_index: __le16,
    pub qid_ver: __le32,
    pub cid: __le32,
    pub dbid: __le16,
    pub depth_log2: u8,
    pub stride_log2: u8,
    pub dma_addr: __le64,
    pub rsvd2: [u8; 40],
}

//
// Notify Events
//
// struct ionic_notifyq_event - Generic event reporting structure
// @eid:   event number
// @ecode: event code
// @data:  unspecified data about the event
//
// This is the generic event report struct from which the other
// actual events will be formed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_notifyq_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub data: [u8; 54],
}

//
// struct ionic_link_change_event - Link change event notification
// @eid:		event number
// @ecode:		event code = IONIC_EVENT_LINK_CHANGE
// @link_status:	link up/down, with error bits (enum ionic_port_status)
// @link_speed:		speed of the network link
// @rsvd:		reserved byte(s)
//
// Sent when the network link state changes between UP and DOWN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_link_change_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub link_status: __le16,
    pub /: *mut *mut __le32 link_speed; / units of 1Mbps: e.g. 10000 = 10Gbps,
    pub rsvd: [u8; 48],
}

//
// struct ionic_reset_event - Reset event notification
// @eid:		event number
// @ecode:		event code = IONIC_EVENT_RESET
// @reset_code:		reset type
// @state:		0=pending, 1=complete, 2=error
// @rsvd:		reserved byte(s)
//
// Sent when the NIC or some subsystem is going to be or
// has been reset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_reset_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub reset_code: u8,
    pub state: u8,
    pub rsvd: [u8; 52],
}

//
// struct ionic_heartbeat_event - Sent periodically by NIC to indicate health
// @eid:	event number
// @ecode:	event code = IONIC_EVENT_HEARTBEAT
// @rsvd:	reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_heartbeat_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub rsvd: [u8; 54],
}

//
// struct ionic_log_event - Sent to notify the driver of an internal error
// @eid:	event number
// @ecode:	event code = IONIC_EVENT_LOG
// @data:	log data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_log_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub data: [u8; 54],
}

//
// struct ionic_xcvr_event - Transceiver change event
// @eid:	event number
// @ecode:	event code = IONIC_EVENT_XCVR
// @rsvd:	reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_xcvr_event {
    pub eid: __le64,
    pub ecode: __le16,
    pub rsvd: [u8; 54],
}

//
// struct ionic_port_stats - Port statistics structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_stats {
    pub frames_rx_ok: __le64,
    pub frames_rx_all: __le64,
    pub frames_rx_bad_fcs: __le64,
    pub frames_rx_bad_all: __le64,
    pub octets_rx_ok: __le64,
    pub octets_rx_all: __le64,
    pub frames_rx_unicast: __le64,
    pub frames_rx_multicast: __le64,
    pub frames_rx_broadcast: __le64,
    pub frames_rx_pause: __le64,
    pub frames_rx_bad_length: __le64,
    pub frames_rx_undersized: __le64,
    pub frames_rx_oversized: __le64,
    pub frames_rx_fragments: __le64,
    pub frames_rx_jabber: __le64,
    pub frames_rx_pripause: __le64,
    pub frames_rx_stomped_crc: __le64,
    pub frames_rx_too_long: __le64,
    pub frames_rx_vlan_good: __le64,
    pub frames_rx_dropped: __le64,
    pub frames_rx_less_than_64b: __le64,
    pub frames_rx_64b: __le64,
    pub frames_rx_65b_127b: __le64,
    pub frames_rx_128b_255b: __le64,
    pub frames_rx_256b_511b: __le64,
    pub frames_rx_512b_1023b: __le64,
    pub frames_rx_1024b_1518b: __le64,
    pub frames_rx_1519b_2047b: __le64,
    pub frames_rx_2048b_4095b: __le64,
    pub frames_rx_4096b_8191b: __le64,
    pub frames_rx_8192b_9215b: __le64,
    pub frames_rx_other: __le64,
    pub frames_tx_ok: __le64,
    pub frames_tx_all: __le64,
    pub frames_tx_bad: __le64,
    pub octets_tx_ok: __le64,
    pub octets_tx_total: __le64,
    pub frames_tx_unicast: __le64,
    pub frames_tx_multicast: __le64,
    pub frames_tx_broadcast: __le64,
    pub frames_tx_pause: __le64,
    pub frames_tx_pripause: __le64,
    pub frames_tx_vlan: __le64,
    pub frames_tx_less_than_64b: __le64,
    pub frames_tx_64b: __le64,
    pub frames_tx_65b_127b: __le64,
    pub frames_tx_128b_255b: __le64,
    pub frames_tx_256b_511b: __le64,
    pub frames_tx_512b_1023b: __le64,
    pub frames_tx_1024b_1518b: __le64,
    pub frames_tx_1519b_2047b: __le64,
    pub frames_tx_2048b_4095b: __le64,
    pub frames_tx_4096b_8191b: __le64,
    pub frames_tx_8192b_9215b: __le64,
    pub frames_tx_other: __le64,
    pub frames_tx_pri_0: __le64,
    pub frames_tx_pri_1: __le64,
    pub frames_tx_pri_2: __le64,
    pub frames_tx_pri_3: __le64,
    pub frames_tx_pri_4: __le64,
    pub frames_tx_pri_5: __le64,
    pub frames_tx_pri_6: __le64,
    pub frames_tx_pri_7: __le64,
    pub frames_rx_pri_0: __le64,
    pub frames_rx_pri_1: __le64,
    pub frames_rx_pri_2: __le64,
    pub frames_rx_pri_3: __le64,
    pub frames_rx_pri_4: __le64,
    pub frames_rx_pri_5: __le64,
    pub frames_rx_pri_6: __le64,
    pub frames_rx_pri_7: __le64,
    pub tx_pripause_0_1us_count: __le64,
    pub tx_pripause_1_1us_count: __le64,
    pub tx_pripause_2_1us_count: __le64,
    pub tx_pripause_3_1us_count: __le64,
    pub tx_pripause_4_1us_count: __le64,
    pub tx_pripause_5_1us_count: __le64,
    pub tx_pripause_6_1us_count: __le64,
    pub tx_pripause_7_1us_count: __le64,
    pub rx_pripause_0_1us_count: __le64,
    pub rx_pripause_1_1us_count: __le64,
    pub rx_pripause_2_1us_count: __le64,
    pub rx_pripause_3_1us_count: __le64,
    pub rx_pripause_4_1us_count: __le64,
    pub rx_pripause_5_1us_count: __le64,
    pub rx_pripause_6_1us_count: __le64,
    pub rx_pripause_7_1us_count: __le64,
    pub rx_pause_1us_count: __le64,
    pub frames_tx_truncated: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_mgmt_port_stats {
    pub frames_rx_ok: __le64,
    pub frames_rx_all: __le64,
    pub frames_rx_bad_fcs: __le64,
    pub frames_rx_bad_all: __le64,
    pub octets_rx_ok: __le64,
    pub octets_rx_all: __le64,
    pub frames_rx_unicast: __le64,
    pub frames_rx_multicast: __le64,
    pub frames_rx_broadcast: __le64,
    pub frames_rx_pause: __le64,
    pub frames_rx_bad_length: __le64,
    pub frames_rx_undersized: __le64,
    pub frames_rx_oversized: __le64,
    pub frames_rx_fragments: __le64,
    pub frames_rx_jabber: __le64,
    pub frames_rx_64b: __le64,
    pub frames_rx_65b_127b: __le64,
    pub frames_rx_128b_255b: __le64,
    pub frames_rx_256b_511b: __le64,
    pub frames_rx_512b_1023b: __le64,
    pub frames_rx_1024b_1518b: __le64,
    pub frames_rx_gt_1518b: __le64,
    pub frames_rx_fifo_full: __le64,
    pub frames_tx_ok: __le64,
    pub frames_tx_all: __le64,
    pub frames_tx_bad: __le64,
    pub octets_tx_ok: __le64,
    pub octets_tx_total: __le64,
    pub frames_tx_unicast: __le64,
    pub frames_tx_multicast: __le64,
    pub frames_tx_broadcast: __le64,
    pub frames_tx_pause: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_extra_stats {
    pub rsfec_correctable_blocks: __le64,
    pub rsfec_uncorrectable_blocks: __le64,
    pub fec_corrected_bits_total: __le64,
    pub rx_bits_phy: __le64,
    pub fec_codeword_error_bin: [__le64; 16],
}

//
// struct ionic_port_identity - port identity structure
// @version:        identity structure version
// @type:           type of port (enum ionic_port_type)
// @num_lanes:      number of lanes for the port
// @autoneg:        autoneg supported
// @min_frame_size: minimum frame size supported
// @max_frame_size: maximum frame size supported
// @fec_type:       supported fec types
// @pause_type:     supported pause types
// @loopback_mode:  supported loopback mode
// @speeds:         supported speeds
// @rsvd2:          reserved byte(s)
// @config:         current port configuration
// @words:          word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_port_identity {
    pub version: u8,
    pub type: u8,
    pub num_lanes: u8,
    pub autoneg: u8,
    pub min_frame_size: __le32,
    pub max_frame_size: __le32,
    pub fec_type: [u8; 4],
    pub pause_type: [u8; 2],
    pub loopback_mode: [u8; 2],
    pub speeds: [__le32; 16],
    pub rsvd2: [u8; 44],
    pub config: ionic_port_config,
}

//
// struct ionic_port_info - port info structure
// @config:          Port configuration data
// @status:          Port status data
// @stats:           Port statistics data
// @mgmt_stats:      Port management statistics data
// @sprom_epage:     Extended Transceiver sprom
// @sprom_page1:     Extended Transceiver sprom, page 1
// @sprom_page2:     Extended Transceiver sprom, page 2
// @sprom_page17:    Extended Transceiver sprom, page 17
// @rsvd:            reserved byte(s)
// @extra_stats:     Extra port statistics data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_port_info {
    pub config: ionic_port_config,
    pub status: ionic_port_status,
    pub stats: ionic_port_stats,
    pub mgmt_stats: ionic_mgmt_port_stats,
}

//
// struct ionic_lif_stats - LIF statistics structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_stats {
// RX
    pub rx_ucast_bytes: __le64,
    pub rx_ucast_packets: __le64,
    pub rx_mcast_bytes: __le64,
    pub rx_mcast_packets: __le64,
    pub rx_bcast_bytes: __le64,
    pub rx_bcast_packets: __le64,
    pub rsvd0: __le64,
    pub rsvd1: __le64,
// RX drops
    pub rx_ucast_drop_bytes: __le64,
    pub rx_ucast_drop_packets: __le64,
    pub rx_mcast_drop_bytes: __le64,
    pub rx_mcast_drop_packets: __le64,
    pub rx_bcast_drop_bytes: __le64,
    pub rx_bcast_drop_packets: __le64,
    pub rx_dma_error: __le64,
    pub rsvd2: __le64,
// TX
    pub tx_ucast_bytes: __le64,
    pub tx_ucast_packets: __le64,
    pub tx_mcast_bytes: __le64,
    pub tx_mcast_packets: __le64,
    pub tx_bcast_bytes: __le64,
    pub tx_bcast_packets: __le64,
    pub rsvd3: __le64,
    pub rsvd4: __le64,
// TX drops
    pub tx_ucast_drop_bytes: __le64,
    pub tx_ucast_drop_packets: __le64,
    pub tx_mcast_drop_bytes: __le64,
    pub tx_mcast_drop_packets: __le64,
    pub tx_bcast_drop_bytes: __le64,
    pub tx_bcast_drop_packets: __le64,
    pub tx_dma_error: __le64,
    pub rsvd5: __le64,
// Rx Queue/Ring drops
    pub rx_queue_disabled: __le64,
    pub rx_queue_empty: __le64,
    pub rx_queue_error: __le64,
    pub rx_desc_fetch_error: __le64,
    pub rx_desc_data_error: __le64,
    pub rsvd6: __le64,
    pub rsvd7: __le64,
    pub rsvd8: __le64,
// Tx Queue/Ring drops
    pub tx_queue_disabled: __le64,
    pub tx_queue_error: __le64,
    pub tx_desc_fetch_error: __le64,
    pub tx_desc_data_error: __le64,
    pub tx_queue_empty: __le64,
    pub rsvd10: __le64,
    pub rsvd11: __le64,
    pub rsvd12: __le64,
// RDMA/ROCE TX
    pub tx_rdma_ucast_bytes: __le64,
    pub tx_rdma_ucast_packets: __le64,
    pub tx_rdma_mcast_bytes: __le64,
    pub tx_rdma_mcast_packets: __le64,
    pub tx_rdma_cnp_packets: __le64,
    pub rsvd13: __le64,
    pub rsvd14: __le64,
    pub rsvd15: __le64,
// RDMA/ROCE RX
    pub rx_rdma_ucast_bytes: __le64,
    pub rx_rdma_ucast_packets: __le64,
    pub rx_rdma_mcast_bytes: __le64,
    pub rx_rdma_mcast_packets: __le64,
    pub rx_rdma_cnp_packets: __le64,
    pub rx_rdma_ecn_packets: __le64,
    pub rsvd16: __le64,
    pub rsvd17: __le64,
    pub rsvd18: __le64,
    pub rsvd19: __le64,
    pub rsvd20: __le64,
    pub rsvd21: __le64,
    pub rsvd22: __le64,
    pub rsvd23: __le64,
    pub rsvd24: __le64,
    pub rsvd25: __le64,
    pub rsvd26: __le64,
    pub rsvd27: __le64,
    pub rsvd28: __le64,
    pub rsvd29: __le64,
    pub rsvd30: __le64,
    pub rsvd31: __le64,
    pub rsvd32: __le64,
    pub rsvd33: __le64,
    pub rsvd34: __le64,
    pub rsvd35: __le64,
    pub rsvd36: __le64,
    pub rsvd37: __le64,
    pub rsvd38: __le64,
    pub rsvd39: __le64,
    pub rsvd40: __le64,
    pub rsvd41: __le64,
    pub rsvd42: __le64,
    pub rsvd43: __le64,
    pub rsvd44: __le64,
    pub rsvd45: __le64,
    pub rsvd46: __le64,
    pub rsvd47: __le64,
    pub rsvd48: __le64,
    pub rsvd49: __le64,
// RDMA/ROCE REQ Error/Debugs (768 - 895)
    pub rdma_req_rx_pkt_seq_err: __le64,
    pub rdma_req_rx_rnr_retry_err: __le64,
    pub rdma_req_rx_remote_access_err: __le64,
    pub rdma_req_rx_remote_inv_req_err: __le64,
    pub rdma_req_rx_remote_oper_err: __le64,
    pub rdma_req_rx_implied_nak_seq_err: __le64,
    pub rdma_req_rx_cqe_err: __le64,
    pub rdma_req_rx_cqe_flush_err: __le64,
    pub rdma_req_rx_dup_responses: __le64,
    pub rdma_req_rx_invalid_packets: __le64,
    pub rdma_req_tx_local_access_err: __le64,
    pub rdma_req_tx_local_oper_err: __le64,
    pub rdma_req_tx_memory_mgmt_err: __le64,
    pub rsvd52: __le64,
    pub rsvd53: __le64,
    pub rsvd54: __le64,
// RDMA/ROCE RESP Error/Debugs (896 - 1023)
    pub rdma_resp_rx_dup_requests: __le64,
    pub rdma_resp_rx_out_of_buffer: __le64,
    pub rdma_resp_rx_out_of_seq_pkts: __le64,
    pub rdma_resp_rx_cqe_err: __le64,
    pub rdma_resp_rx_cqe_flush_err: __le64,
    pub rdma_resp_rx_local_len_err: __le64,
    pub rdma_resp_rx_inv_request_err: __le64,
    pub rdma_resp_rx_local_qp_oper_err: __le64,
    pub rdma_resp_rx_out_of_atomic_resource: __le64,
    pub rdma_resp_tx_pkt_seq_err: __le64,
    pub rdma_resp_tx_remote_inv_req_err: __le64,
    pub rdma_resp_tx_remote_access_err: __le64,
    pub rdma_resp_tx_remote_oper_err: __le64,
    pub rdma_resp_tx_rnr_retry_err: __le64,
    pub rsvd57: __le64,
    pub rsvd58: __le64,
}

//
// struct ionic_lif_info - LIF info structure
// @config:	LIF configuration structure
// @status:	LIF status structure
// @stats:	LIF statistics structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_info {
    pub config: ionic_lif_config,
    pub status: ionic_lif_status,
    pub stats: ionic_lif_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_dev_cmd {
    pub words: [u32; 16],
    pub cmd: ionic_admin_cmd,
    pub nop: ionic_nop_cmd,
    pub identify: ionic_dev_identify_cmd,
    pub init: ionic_dev_init_cmd,
    pub reset: ionic_dev_reset_cmd,
    pub getattr: ionic_dev_getattr_cmd,
    pub setattr: ionic_dev_setattr_cmd,
    pub port_identify: ionic_port_identify_cmd,
    pub port_init: ionic_port_init_cmd,
    pub port_reset: ionic_port_reset_cmd,
    pub port_getattr: ionic_port_getattr_cmd,
    pub port_setattr: ionic_port_setattr_cmd,
    pub vf_setattr: ionic_vf_setattr_cmd,
    pub vf_getattr: ionic_vf_getattr_cmd,
    pub vf_ctrl: ionic_vf_ctrl_cmd,
    pub discover_cmb: ionic_discover_cmb_cmd,
    pub lif_identify: ionic_lif_identify_cmd,
    pub lif_init: ionic_lif_init_cmd,
    pub lif_reset: ionic_lif_reset_cmd,
    pub qos_identify: ionic_qos_identify_cmd,
    pub qos_init: ionic_qos_init_cmd,
    pub qos_reset: ionic_qos_reset_cmd,
    pub qos_clear_stats: ionic_qos_clear_stats_cmd,
    pub q_identify: ionic_q_identify_cmd,
    pub q_init: ionic_q_init_cmd,
    pub q_control: ionic_q_control_cmd,
    pub fw_download: ionic_fw_download_cmd,
    pub fw_control: ionic_fw_control_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_dev_cmd_comp {
    pub words: [u32; 4],
    pub status: u8,
    pub comp: ionic_admin_comp,
    pub nop: ionic_nop_comp,
    pub identify: ionic_dev_identify_comp,
    pub init: ionic_dev_init_comp,
    pub reset: ionic_dev_reset_comp,
    pub getattr: ionic_dev_getattr_comp,
    pub setattr: ionic_dev_setattr_comp,
    pub port_identify: ionic_port_identify_comp,
    pub port_init: ionic_port_init_comp,
    pub port_reset: ionic_port_reset_comp,
    pub port_getattr: ionic_port_getattr_comp,
    pub port_setattr: ionic_port_setattr_comp,
    pub vf_setattr: ionic_vf_setattr_comp,
    pub vf_getattr: ionic_vf_getattr_comp,
    pub vf_ctrl: ionic_vf_ctrl_comp,
    pub discover_cmb: ionic_discover_cmb_comp,
    pub lif_identify: ionic_lif_identify_comp,
    pub lif_init: ionic_lif_init_comp,
    pub lif_reset: ionic_lif_reset_comp,
    pub qos_identify: ionic_qos_identify_comp,
    pub qos_init: ionic_qos_init_comp,
    pub qos_reset: ionic_qos_reset_comp,
    pub q_identify: ionic_q_identify_comp,
    pub q_init: ionic_q_init_comp,
    pub fw_download: ionic_fw_download_comp,
    pub fw_control: ionic_fw_control_comp,
}

//
// struct ionic_hwstamp_regs - Hardware current timestamp registers
// @tick_low:        Low 32 bits of hardware timestamp
// @tick_high:       High 32 bits of hardware timestamp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_hwstamp_regs {
    pub tick_low: u32,
    pub tick_high: u32,
}

//
// union ionic_dev_info_regs - Device info register format (read-only)
// @signature:       Signature value of 0x44455649 ('DEVI')
// @version:         Current version of info
// @asic_type:       Asic type
// @asic_rev:        Asic revision
// @fw_status:       Firmware status
// bit 0   - 1 = fw running
// bit 4-7 - 4 bit generation number, changes on fw restart
// @fw_heartbeat:    Firmware heartbeat counter
// @serial_num:      Serial number
// @rsvd_pad1024:    reserved byte(s)
// @fw_version:      Firmware version
// @hwstamp:         Hardware current timestamp registers
// @words:           word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_dev_info_regs {
pub const IONIC_DEVINFO_FWVERS_BUFLEN: c_int = 32;
pub const IONIC_DEVINFO_SERIAL_BUFLEN: c_int = 32;
    pub signature: u32,
    pub version: u8,
    pub asic_type: u8,
    pub asic_rev: u8,
pub const IONIC_FW_STS_F_RUNNING: c_uint = 0x01;
pub const IONIC_FW_STS_F_GENERATION: c_uint = 0xF0;
    pub fw_status: u8,
    pub fw_heartbeat: u32,
    pub fw_version: [c_char; IONIC_DEVINFO_FWVERS_BUFLEN],
    pub serial_num: [c_char; IONIC_DEVINFO_SERIAL_BUFLEN],
    pub rsvd_pad1024: [u8; 948],
    pub hwstamp: ionic_hwstamp_regs,
}

//
// union ionic_dev_cmd_regs - Device command register format (read-write)
// @doorbell:        Device Cmd Doorbell, write-only
// Write a 1 to signal device to process cmd,
// poll done for completion.
// @done:            Done indicator, bit 0 == 1 when command is complete
// @cmd:             Opcode-specific command bytes
// @comp:            Opcode-specific response bytes
// @rsvd:            reserved byte(s)
// @data:            Opcode-specific side-data
// @words:           word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_dev_cmd_regs {
    pub doorbell: u32,
    pub done: u32,
    pub cmd: ionic_dev_cmd,
    pub comp: ionic_dev_cmd_comp,
    pub rsvd: [u8; 48],
    pub data: [u32; 478],
    pub __packed: },
    pub words: [u32; 512],
}

//
// union ionic_dev_regs - Device register format for bar 0 page 0
// @info:            Device info registers
// @devcmd:          Device command registers
// @words:           word access to struct contents
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_dev_regs {
    pub info: ionic_dev_info_regs,
    pub devcmd: ionic_dev_cmd_regs,
    pub __packed: },
    pub words: [__le32; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_adminq_cmd {
    pub cmd: ionic_admin_cmd,
    pub nop: ionic_nop_cmd,
    pub q_identify: ionic_q_identify_cmd,
    pub q_init: ionic_q_init_cmd,
    pub q_control: ionic_q_control_cmd,
    pub lif_setattr: ionic_lif_setattr_cmd,
    pub lif_getattr: ionic_lif_getattr_cmd,
    pub lif_setphc: ionic_lif_setphc_cmd,
    pub rx_mode_set: ionic_rx_mode_set_cmd,
    pub rx_filter_add: ionic_rx_filter_add_cmd,
    pub rx_filter_del: ionic_rx_filter_del_cmd,
    pub rdma_reset: ionic_rdma_reset_cmd,
    pub rdma_queue: ionic_rdma_queue_cmd,
    pub fw_download: ionic_fw_download_cmd,
    pub fw_control: ionic_fw_control_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_adminq_comp {
    pub comp: ionic_admin_comp,
    pub nop: ionic_nop_comp,
    pub q_identify: ionic_q_identify_comp,
    pub q_init: ionic_q_init_comp,
    pub lif_setattr: ionic_lif_setattr_comp,
    pub lif_getattr: ionic_lif_getattr_comp,
    pub lif_setphc: ionic_admin_comp,
    pub rx_filter_add: ionic_rx_filter_add_comp,
    pub fw_control: ionic_fw_control_comp,
}

pub const IONIC_BARS_MAX: c_int = 6;
pub const IONIC_PCI_BAR_DBELL: c_int = 1;
pub const IONIC_PCI_BAR_CMB: c_int = 2;
pub const IONIC_BAR0_SIZE: c_uint = 0x8000;
pub const IONIC_BAR2_SIZE: c_uint = 0x800000;
pub const IONIC_BAR0_DEV_INFO_REGS_OFFSET: c_uint = 0x0000;
pub const IONIC_BAR0_DEV_CMD_REGS_OFFSET: c_uint = 0x0800;
pub const IONIC_BAR0_DEV_CMD_DATA_REGS_OFFSET: c_uint = 0x0c00;
pub const IONIC_BAR0_INTR_STATUS_OFFSET: c_uint = 0x1000;
pub const IONIC_BAR0_INTR_CTRL_OFFSET: c_uint = 0x2000;
// BAR2
pub const IONIC_BAR2_CMB_ENTRY_SIZE: c_uint = 0x800000;
pub const IONIC_DEV_CMD_DONE: c_uint = 0x00000001;
pub const IONIC_ASIC_TYPE_NONE: c_int = 0;
pub const IONIC_ASIC_TYPE_CAPRI: c_int = 1;
pub const IONIC_ASIC_TYPE_ELBA: c_int = 2;
pub const IONIC_ASIC_TYPE_GIGLIO: c_int = 3;
pub const IONIC_ASIC_TYPE_SALINA: c_int = 4;
//
// struct ionic_doorbell - Doorbell register layout
// @p_index: Producer index
// @ring:    Selects the specific ring of the queue to update
// Type-specific meaning:
// ring=0: Default producer/consumer queue
// ring=1: (CQ, EQ) Re-Arm queue.  RDMA CQs
// send events to EQs when armed.  EQs send
// interrupts when armed.
// @qid_lo:  Queue destination for the producer index and flags (low bits)
// @qid_hi:  Queue destination for the producer index and flags (high bits)
// @rsvd2:   reserved byte(s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_doorbell {
    pub p_index: __le16,
    pub ring: u8,
    pub qid_lo: u8,
    pub qid_hi: __le16,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_intr_status {
    pub status: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_notifyq_cmd {
    pub /: *mut *mut __le32 data; / Not used but needed for qcq structure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ionic_notifyq_comp {
    pub event: ionic_notifyq_event,
    pub link_change: ionic_link_change_event,
    pub reset: ionic_reset_event,
    pub heartbeat: ionic_heartbeat_event,
    pub log: ionic_log_event,
}

// Deprecate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_identity {
    pub drv: ionic_drv_identity,
    pub dev: ionic_dev_identity,
    pub lif: ionic_lif_identity,
    pub port: ionic_port_identity,
    pub qos: ionic_qos_identity,
    pub txq: ionic_q_identity,
    pub cmb_layout: ionic_discover_cmb_identity,
}
