//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_type.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

// IAVF_MASK is a macro used on 32 bit registers

pub const IAVF_MAX_VSI_QP: c_int = 16;
pub const IAVF_MAX_VF_VSI: c_int = 3;
pub const IAVF_MAX_CHAINED_RX_BUFFERS: c_int = 5;
// forward declaration
extern "C" {
    pub fn void(: *mut *mut IAVF_ADMINQ_CALLBACK)(struct iavf_hw, : *mut libie_aq_desc) -> typedef;
}
// Data type manipulation macros.

// bitfields for Tx queue mapping in QTX_CTL
pub const IAVF_QTX_CTL_VF_QUEUE: c_uint = 0x0;
pub const IAVF_QTX_CTL_VM_QUEUE: c_uint = 0x1;
pub const IAVF_QTX_CTL_PF_QUEUE: c_uint = 0x2;
// debug masks - set these bits in hw->debug_mask to control output
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_debug_mask {
    IAVF_DEBUG_INIT			= 0x00000001,
    IAVF_DEBUG_RELEASE		= 0x00000002,

    IAVF_DEBUG_LINK			= 0x00000010,
    IAVF_DEBUG_PHY			= 0x00000020,
    IAVF_DEBUG_HMC			= 0x00000040,
    IAVF_DEBUG_NVM			= 0x00000080,
    IAVF_DEBUG_LAN			= 0x00000100,
    IAVF_DEBUG_FLOW			= 0x00000200,
    IAVF_DEBUG_DCB			= 0x00000400,
    IAVF_DEBUG_DIAG			= 0x00000800,
    IAVF_DEBUG_FD			= 0x00001000,
    IAVF_DEBUG_PACKAGE		= 0x00002000,

    IAVF_DEBUG_AQ_MESSAGE		= 0x01000000,
    IAVF_DEBUG_AQ_DESCRIPTOR	= 0x02000000,
    IAVF_DEBUG_AQ_DESC_BUFFER	= 0x04000000,
    IAVF_DEBUG_AQ_COMMAND		= 0x06000000,
    IAVF_DEBUG_AQ			= 0x0F000000,

    IAVF_DEBUG_USER			= 0xF0000000,

    IAVF_DEBUG_ALL			= 0xFFFFFFFF
}

// These are structs for managing the hardware information and the operations.
// The structures of function pointers are filled out at init time when we
// know for sure exactly which hardware we're working with.  This gives us the
// flexibility of using the same main driver code but adapting to slightly
// different hardware needs as new parts are developed.  For this architecture,
// the Firmware and AdminQ are intended to insulate the driver from most of the
// future changes, but these structures will also do part of the job.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_vsi_type {
    IAVF_VSI_MAIN	= 0,
    IAVF_VSI_VMDQ1	= 1,
    IAVF_VSI_VMDQ2	= 2,
    IAVF_VSI_CTRL	= 3,
    IAVF_VSI_FCOE	= 4,
    IAVF_VSI_MIRROR	= 5,
    IAVF_VSI_SRIOV	= 6,
    IAVF_VSI_FDIR	= 7,
    IAVF_VSI_TYPE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_queue_type {
    IAVF_QUEUE_TYPE_RX = 0,
    IAVF_QUEUE_TYPE_TX,
    IAVF_QUEUE_TYPE_PE_CEQ,
    IAVF_QUEUE_TYPE_UNKNOWN
}

pub const IAVF_HW_CAP_MAX_GPIO: c_int = 30;
// Capabilities of a PF or a VF or the whole device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_hw_capabilities {
    pub dcb: bool,
    pub fcoe: bool,
    pub num_vsis: u32,
    pub num_rx_qp: u32,
    pub num_tx_qp: u32,
    pub base_queue: u32,
    pub num_msix_vectors_vf: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_mac_info {
    pub addr: [u8; ETH_ALEN],
    pub perm_addr: [u8; ETH_ALEN],
}

// PCI bus types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_bus_type {
    iavf_bus_type_unknown = 0,
    iavf_bus_type_pci,
    iavf_bus_type_pcix,
    iavf_bus_type_pci_express,
    iavf_bus_type_reserved
}

// PCI bus speeds
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_bus_speed {
    iavf_bus_speed_unknown	= 0,
    iavf_bus_speed_33	= 33,
    iavf_bus_speed_66	= 66,
    iavf_bus_speed_100	= 100,
    iavf_bus_speed_120	= 120,
    iavf_bus_speed_133	= 133,
    iavf_bus_speed_2500	= 2500,
    iavf_bus_speed_5000	= 5000,
    iavf_bus_speed_8000	= 8000,
    iavf_bus_speed_reserved
}

// PCI bus widths
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_bus_width {
    iavf_bus_width_unknown	= 0,
    iavf_bus_width_pcie_x1	= 1,
    iavf_bus_width_pcie_x2	= 2,
    iavf_bus_width_pcie_x4	= 4,
    iavf_bus_width_pcie_x8	= 8,
    iavf_bus_width_32	= 32,
    iavf_bus_width_64	= 64,
    iavf_bus_width_reserved
}

// Bus parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_bus_info {
    pub speed: iavf_bus_speed,
    pub width: iavf_bus_width,
    pub type: iavf_bus_type,
    pub func: u16,
    pub device: u16,
    pub lan_id: u16,
    pub bus_id: u16,
}

pub const IAVF_MAX_USER_PRIORITY: c_int = 8;
// Port hardware description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_hw {
    pub hw_addr: *mut u8 __iomem,
    pub back: *mut c_void,
// subsystem structs
    pub mac: iavf_mac_info,
    pub bus: iavf_bus_info,
// pci info
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_device_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
// capabilities for entire device and PCI func
    pub dev_caps: iavf_hw_capabilities,
// Admin Queue info
    pub aq: iavf_adminq_info,
// debug mask
    pub debug_mask: u32,
    pub err_str: [c_char; 16],
}

//
// struct iavf_rx_desc - Receive descriptor (both legacy and flexible)
// @qw0: quad word 0 fields:
// Legacy: Descriptor Type; Mirror ID; L2TAG1P (S-TAG); Filter Status
// Flex: Descriptor Type; Mirror ID; UMBCAST; Packet Type; Flexible Flags
// Section 0; Packet Length; Header Length; Split Header Flag;
// Flexible Flags section 1 / Extended Status
// @qw1: quad word 1 fields:
// Legacy: Status Field; Error Field; Packet Type; Packet Length (packet,
// header, Split Header Flag)
// Flex: Status / Error 0 Field; L2TAG1P (S-TAG); Flexible Metadata
// Container #0; Flexible Metadata Container #1
// @qw2: quad word 2 fields:
// Legacy: Extended Status; 1st L2TAG2P (C-TAG); 2nd L2TAG2P (C-TAG)
// Flex: Status / Error 1 Field; Flexible Flags section 2; Timestamp Low;
// 1st L2TAG2 (C-TAG); 2nd L2TAG2 (C-TAG)
// @qw3: quad word 3 fields:
// Legacy: FD Filter ID / Flexible Bytes
// Flex: Flexible Metadata Container #2; Flexible Metadata Container #3;
// Flexible Metadata Container #4 / Timestamp High 0; Flexible
// Metadata Container #5 / Timestamp High 1;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_rx_desc {
    pub qw0: aligned_le64,
// The hash signature (RSS)

// Stripped C-TAG VLAN from the receive packet

// Packet type

// Packet length

    pub qw1: aligned_le64,
// Descriptor done indication flag.

// End of packet. Set to 1 if this descriptor is the last one of the packet

// L2 TAG 1 presence indication

// Detectable L3 and L4 integrity check is processed by the HW

// Set when an IPv6 packet contains a Destination Options Header or a Routing
// Header.
//

// Receive MAC Errors: CRC; Alignment; Oversize; Undersizes; Length error

// Checksum reports:
// - IPE: IP checksum error
// - L4E: L4 integrity error
// - EIPE: External IP header (tunneled packets)
//

// Set for packets that skip checksum calculation in pre-parser

// Indicates the content in the Filter Status field

// Packet type

// Packet length

// Descriptor done indication flag

// End of packet. Set to 1 if this descriptor is the last one of the packet

// Detectable L3 and L4 integrity check is processed by the HW

// Checksum reports:
// - IPE: IP checksum error
// - L4E: L4 integrity error
// - EIPE: External IP header (tunneled packets)
// - EUDPE: External UDP checksum error (tunneled packets)
//

// Set when an IPv6 packet contains a Destination Options Header or a Routing
// Header.
//

// Receive MAC Errors: CRC; Alignment; Oversize; Undersizes; Length error

// Indicates that the RSS/HASH result is valid

// L2 TAG 1 presence indication

// Stripped L2 Tag from the receive packet

// The hash signature (RSS)

    pub qw2: aligned_le64,
// L2 Tag 2 Presence

// Stripped S-TAG VLAN from the receive packet

// Stripped S-TAG VLAN from the receive packet

// The packet is a UDP tunneled packet

// L2 Tag 2 Presence

    pub qw3: aligned_le64,

    pub sizeof(__le64)): *mut *mut } __aligned(4,
    pub 32): static_assert(sizeof(struct iavf_rx_desc) ==,

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_rx_desc_fltstat_values {
    IAVF_RX_DESC_FLTSTAT_NO_DATA	= 0,
    IAVF_RX_DESC_FLTSTAT_RSV_FD_ID	= 1, /* 16byte desc? FD_ID : RSV */
    IAVF_RX_DESC_FLTSTAT_RSV	= 2,
    IAVF_RX_DESC_FLTSTAT_RSS_HASH	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_rx_desc_error_l3l4e_fcoe_masks {
    IAVF_RX_DESC_ERROR_L3L4E_NONE		= 0,
    IAVF_RX_DESC_ERROR_L3L4E_PROT		= 1,
    IAVF_RX_DESC_ERROR_L3L4E_FC		= 2,
    IAVF_RX_DESC_ERROR_L3L4E_DMAC_ERR	= 3,
    IAVF_RX_DESC_ERROR_L3L4E_DMAC_WARN	= 4
}

pub const IAVF_RXD_QW1_LENGTH_HBUF_SHIFT: c_int = 52;

pub const IAVF_RXD_QW1_LENGTH_SPH_SHIFT: c_int = 63;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_rx_desc_ext_status_bits {
// Note: These are predefined bit offsets
    IAVF_RX_DESC_EXT_STATUS_L2TAG2P_SHIFT	= 0,
    IAVF_RX_DESC_EXT_STATUS_L2TAG3P_SHIFT	= 1,
    IAVF_RX_DESC_EXT_STATUS_FLEXBL_SHIFT	= 2, /* 2 BITS */
    IAVF_RX_DESC_EXT_STATUS_FLEXBH_SHIFT	= 4, /* 2 BITS */
    IAVF_RX_DESC_EXT_STATUS_FDLONGB_SHIFT	= 9,
    IAVF_RX_DESC_EXT_STATUS_FCOELONGB_SHIFT	= 10,
    IAVF_RX_DESC_EXT_STATUS_PELONGB_SHIFT	= 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_rx_desc_pe_status_bits {
// Note: These are predefined bit offsets
    IAVF_RX_DESC_PE_STATUS_QPID_SHIFT	= 0, /* 18 BITS */
    IAVF_RX_DESC_PE_STATUS_L4PORT_SHIFT	= 0, /* 16 BITS */
    IAVF_RX_DESC_PE_STATUS_IPINDEX_SHIFT	= 16, /* 8 BITS */
    IAVF_RX_DESC_PE_STATUS_QPIDHIT_SHIFT	= 24,
    IAVF_RX_DESC_PE_STATUS_APBVTHIT_SHIFT	= 25,
    IAVF_RX_DESC_PE_STATUS_PORTV_SHIFT	= 26,
    IAVF_RX_DESC_PE_STATUS_URG_SHIFT	= 27,
    IAVF_RX_DESC_PE_STATUS_IPFRAG_SHIFT	= 28,
    IAVF_RX_DESC_PE_STATUS_IPOPT_SHIFT	= 29
}

pub const IAVF_RX_PROG_STATUS_DESC_LENGTH_SHIFT: c_int = 38;
pub const IAVF_RX_PROG_STATUS_DESC_LENGTH: c_uint = 0x2000000;
pub const IAVF_RX_PROG_STATUS_DESC_QW1_PROGID_SHIFT: c_int = 2;

pub const IAVF_RX_PROG_STATUS_DESC_QW1_ERROR_SHIFT: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_rx_prog_status_desc_status_bits {
// Note: These are predefined bit offsets
    IAVF_RX_PROG_STATUS_DESC_DD_SHIFT	= 0,
    IAVF_RX_PROG_STATUS_DESC_PROG_ID_SHIFT	= 2 /* 3 BITS */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_rx_prog_status_desc_prog_id_masks {
    IAVF_RX_PROG_STATUS_DESC_FD_FILTER_STATUS	= 1,
    IAVF_RX_PROG_STATUS_DESC_FCOE_CTXT_PROG_STATUS	= 2,
    IAVF_RX_PROG_STATUS_DESC_FCOE_CTXT_INVL_STATUS	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_rx_prog_status_desc_error_bits {
// Note: These are predefined bit offsets
    IAVF_RX_PROG_STATUS_DESC_FD_TBL_FULL_SHIFT	= 0,
    IAVF_RX_PROG_STATUS_DESC_NO_FD_ENTRY_SHIFT	= 1,
    IAVF_RX_PROG_STATUS_DESC_FCOE_TBL_FULL_SHIFT	= 2,
    IAVF_RX_PROG_STATUS_DESC_FCOE_CONFLICT_SHIFT	= 3
}

// TX Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_offset_bsz: __le64,
}

pub const IAVF_TXD_QW1_DTYPE_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_tx_desc_dtype_value {
    IAVF_TX_DESC_DTYPE_DATA		= 0x0,
    IAVF_TX_DESC_DTYPE_NOP		= 0x1, /* same as Context desc */
    IAVF_TX_DESC_DTYPE_CONTEXT	= 0x1,
    IAVF_TX_DESC_DTYPE_FCOE_CTX	= 0x2,
    IAVF_TX_DESC_DTYPE_FILTER_PROG	= 0x8,
    IAVF_TX_DESC_DTYPE_DDP_CTX	= 0x9,
    IAVF_TX_DESC_DTYPE_FLEX_DATA	= 0xB,
    IAVF_TX_DESC_DTYPE_FLEX_CTX_1	= 0xC,
    IAVF_TX_DESC_DTYPE_FLEX_CTX_2	= 0xD,
    IAVF_TX_DESC_DTYPE_DESC_DONE	= 0xF
}

pub const IAVF_TXD_QW1_CMD_SHIFT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_tx_desc_cmd_bits {
    IAVF_TX_DESC_CMD_EOP			= 0x0001,
    IAVF_TX_DESC_CMD_RS			= 0x0002,
    IAVF_TX_DESC_CMD_ICRC			= 0x0004,
    IAVF_TX_DESC_CMD_IL2TAG1		= 0x0008,
    IAVF_TX_DESC_CMD_DUMMY			= 0x0010,
    IAVF_TX_DESC_CMD_IIPT_NONIP		= 0x0000, /* 2 BITS */
    IAVF_TX_DESC_CMD_IIPT_IPV6		= 0x0020, /* 2 BITS */
    IAVF_TX_DESC_CMD_IIPT_IPV4		= 0x0040, /* 2 BITS */
    IAVF_TX_DESC_CMD_IIPT_IPV4_CSUM		= 0x0060, /* 2 BITS */
    IAVF_TX_DESC_CMD_FCOET			= 0x0080,
    IAVF_TX_DESC_CMD_L4T_EOFT_UNK		= 0x0000, /* 2 BITS */
    IAVF_TX_DESC_CMD_L4T_EOFT_TCP		= 0x0100, /* 2 BITS */
    IAVF_TX_DESC_CMD_L4T_EOFT_SCTP		= 0x0200, /* 2 BITS */
    IAVF_TX_DESC_CMD_L4T_EOFT_UDP		= 0x0300, /* 2 BITS */
    IAVF_TX_DESC_CMD_L4T_EOFT_EOF_N		= 0x0000, /* 2 BITS */
    IAVF_TX_DESC_CMD_L4T_EOFT_EOF_T		= 0x0100, /* 2 BITS */
    IAVF_TX_DESC_CMD_L4T_EOFT_EOF_NI	= 0x0200, /* 2 BITS */
    IAVF_TX_DESC_CMD_L4T_EOFT_EOF_A		= 0x0300, /* 2 BITS */
}

pub const IAVF_TXD_QW1_OFFSET_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_tx_desc_length_fields {
// Note: These are predefined bit offsets
    IAVF_TX_DESC_LENGTH_MACLEN_SHIFT	= 0, /* 7 BITS */
    IAVF_TX_DESC_LENGTH_IPLEN_SHIFT		= 7, /* 7 BITS */
    IAVF_TX_DESC_LENGTH_L4_FC_LEN_SHIFT	= 14 /* 4 BITS */
}

pub const IAVF_TXD_QW1_TX_BUF_SZ_SHIFT: c_int = 34;

pub const IAVF_TXD_QW1_L2TAG1_SHIFT: c_int = 48;

// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_tx_context_desc {
    pub tunneling_params: __le32,
    pub l2tag2: __le16,
    pub rsvd: __le16,
    pub type_cmd_tso_mss: __le64,
}

pub const IAVF_TXD_CTX_QW1_CMD_SHIFT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_tx_ctx_desc_cmd_bits {
    IAVF_TX_CTX_DESC_TSO		= 0x01,
    IAVF_TX_CTX_DESC_TSYN		= 0x02,
    IAVF_TX_CTX_DESC_IL2TAG2	= 0x04,
    IAVF_TX_CTX_DESC_IL2TAG2_IL2H	= 0x08,
    IAVF_TX_CTX_DESC_SWTCH_NOTAG	= 0x00,
    IAVF_TX_CTX_DESC_SWTCH_UPLINK	= 0x10,
    IAVF_TX_CTX_DESC_SWTCH_LOCAL	= 0x20,
    IAVF_TX_CTX_DESC_SWTCH_VSI	= 0x30,
    IAVF_TX_CTX_DESC_SWPE		= 0x40
}

pub const IAVF_TXD_CTX_QW1_TSO_LEN_SHIFT: c_int = 30;

pub const IAVF_TXD_CTX_QW1_MSS_SHIFT: c_int = 50;

pub const IAVF_TXD_CTX_QW1_VSI_SHIFT: c_int = 50;

pub const IAVF_TXD_CTX_QW0_EXT_IP_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_tx_ctx_desc_eipt_offload {
    IAVF_TX_CTX_EXT_IP_NONE		= 0x0,
    IAVF_TX_CTX_EXT_IP_IPV6		= 0x1,
    IAVF_TX_CTX_EXT_IP_IPV4_NO_CSUM	= 0x2,
    IAVF_TX_CTX_EXT_IP_IPV4		= 0x3
}

pub const IAVF_TXD_CTX_QW0_EXT_IPLEN_SHIFT: c_int = 2;

pub const IAVF_TXD_CTX_QW0_NATT_SHIFT: c_int = 9;

pub const IAVF_TXD_CTX_QW0_EIP_NOINC_SHIFT: c_int = 11;

pub const IAVF_TXD_CTX_QW0_NATLEN_SHIFT: c_int = 12;

pub const IAVF_TXD_CTX_QW0_DECTTL_SHIFT: c_int = 19;

pub const IAVF_TXD_CTX_QW0_L4T_CS_SHIFT: c_int = 23;

// Statistics collected by each port, VSI, VEB, and S-channel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_eth_stats {
    pub /: *mut *mut u64 rx_bytes; / gorc,
    pub /: *mut *mut u64 rx_unicast; / uprc,
    pub /: *mut *mut u64 rx_multicast; / mprc,
    pub /: *mut *mut u64 rx_broadcast; / bprc,
    pub /: *mut *mut u64 rx_discards; / rdpc,
    pub /: *mut *mut u64 rx_unknown_protocol; / rupp,
    pub /: *mut *mut u64 tx_bytes; / gotc,
    pub /: *mut *mut u64 tx_unicast; / uptc,
    pub /: *mut *mut u64 tx_multicast; / mptc,
    pub /: *mut *mut u64 tx_broadcast; / bptc,
    pub /: *mut *mut u64 tx_discards; / tdpc,
    pub /: *mut *mut u64 tx_errors; / tepc,
}
