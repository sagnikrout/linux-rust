//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/virtchnl.h
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
// Copyright (c) 2013-2022, Intel Corporation.

// Description:
// This header file describes the Virtual Function (VF) - Physical Function
// (PF) communication protocol used by the drivers for all devices starting
// from our 40G product line
//
// Admin queue buffer usage:
// desc->opcode is always aqc_opc_send_msg_to_pf
// flags, retval, datalen, and data addr are all used normally.
// The Firmware copies the cookie fields when sending messages between the
// PF and VF, but uses all other fields internally. Due to this limitation,
// we must send all messages as "indirect", i.e. using an external buffer.
//
// All the VSI indexes are relative to the VF. Each VF can have maximum of
// three VSIs. All the queue indexes are relative to the VSI.  Each VF can
// have a maximum of sixteen queues for all of its VSIs.
//
// The PF is required to return a status code in v_retval for all messages
// except RESET_VF, which does not require any response. The returned value
// is of virtchnl_status_code type, defined here.
//
// In general, VF driver initialization should roughly follow the order of
// these opcodes. The VF driver must first validate the API version of the
// PF driver, then request a reset, then get resources, then configure
// queues and interrupts. After these operations are complete, the VF
// driver may start its queues, optionally add MAC and VLAN filters, and
// process traffic.
//
// START GENERIC DEFINES
// Need to ensure the following enums and defines hold the same meaning and
// value in current and future projects
//
// Error Codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_status_code {
    VIRTCHNL_STATUS_SUCCESS				= 0,
    VIRTCHNL_STATUS_ERR_PARAM			= -5,
    VIRTCHNL_STATUS_ERR_NO_MEMORY			= -18,
    VIRTCHNL_STATUS_ERR_OPCODE_MISMATCH		= -38,
    VIRTCHNL_STATUS_ERR_CQP_COMPL_ERROR		= -39,
    VIRTCHNL_STATUS_ERR_INVALID_VF_ID		= -40,
    VIRTCHNL_STATUS_ERR_ADMIN_QUEUE_ERROR		= -53,
    VIRTCHNL_STATUS_ERR_NOT_SUPPORTED		= -64,
}

// Backward compatibility

pub const VIRTCHNL_LINK_SPEED_2_5GB_SHIFT: c_uint = 0x0;
pub const VIRTCHNL_LINK_SPEED_100MB_SHIFT: c_uint = 0x1;
pub const VIRTCHNL_LINK_SPEED_1000MB_SHIFT: c_uint = 0x2;
pub const VIRTCHNL_LINK_SPEED_10GB_SHIFT: c_uint = 0x3;
pub const VIRTCHNL_LINK_SPEED_40GB_SHIFT: c_uint = 0x4;
pub const VIRTCHNL_LINK_SPEED_20GB_SHIFT: c_uint = 0x5;
pub const VIRTCHNL_LINK_SPEED_25GB_SHIFT: c_uint = 0x6;
pub const VIRTCHNL_LINK_SPEED_5GB_SHIFT: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_link_speed {
    VIRTCHNL_LINK_SPEED_UNKNOWN	= 0,
    VIRTCHNL_LINK_SPEED_100MB	= BIT(VIRTCHNL_LINK_SPEED_100MB_SHIFT),
    VIRTCHNL_LINK_SPEED_1GB		= BIT(VIRTCHNL_LINK_SPEED_1000MB_SHIFT),
    VIRTCHNL_LINK_SPEED_10GB	= BIT(VIRTCHNL_LINK_SPEED_10GB_SHIFT),
    VIRTCHNL_LINK_SPEED_40GB	= BIT(VIRTCHNL_LINK_SPEED_40GB_SHIFT),
    VIRTCHNL_LINK_SPEED_20GB	= BIT(VIRTCHNL_LINK_SPEED_20GB_SHIFT),
    VIRTCHNL_LINK_SPEED_25GB	= BIT(VIRTCHNL_LINK_SPEED_25GB_SHIFT),
    VIRTCHNL_LINK_SPEED_2_5GB	= BIT(VIRTCHNL_LINK_SPEED_2_5GB_SHIFT),
    VIRTCHNL_LINK_SPEED_5GB		= BIT(VIRTCHNL_LINK_SPEED_5GB_SHIFT),
}

// for hsplit_0 field of Rx HMC context
// deprecated with AVF 1.0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_rx_hsplit {
    VIRTCHNL_RX_HSPLIT_NO_SPLIT      = 0,
    VIRTCHNL_RX_HSPLIT_SPLIT_L2      = 1,
    VIRTCHNL_RX_HSPLIT_SPLIT_IP      = 2,
    VIRTCHNL_RX_HSPLIT_SPLIT_TCP_UDP = 4,
    VIRTCHNL_RX_HSPLIT_SPLIT_SCTP    = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_bw_limit_type {
    VIRTCHNL_BW_SHAPER = 0,
}

// END GENERIC DEFINES
// Opcodes for VF-PF communication. These are placed in the v_opcode field
// of the virtchnl_msg structure.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_ops {
// The PF sends status change events to VFs using
// the VIRTCHNL_OP_EVENT opcode.
// VFs send requests to the PF using the other ops.
// Use of "advanced opcode" features must be negotiated as part of capabilities
// exchange and are not considered part of base mode feature set.
//
    VIRTCHNL_OP_UNKNOWN = 0,
    VIRTCHNL_OP_VERSION = 1, /* must ALWAYS be 1 */
    VIRTCHNL_OP_RESET_VF = 2,
    VIRTCHNL_OP_GET_VF_RESOURCES = 3,
    VIRTCHNL_OP_CONFIG_TX_QUEUE = 4,
    VIRTCHNL_OP_CONFIG_RX_QUEUE = 5,
    VIRTCHNL_OP_CONFIG_VSI_QUEUES = 6,
    VIRTCHNL_OP_CONFIG_IRQ_MAP = 7,
    VIRTCHNL_OP_ENABLE_QUEUES = 8,
    VIRTCHNL_OP_DISABLE_QUEUES = 9,
    VIRTCHNL_OP_ADD_ETH_ADDR = 10,
    VIRTCHNL_OP_DEL_ETH_ADDR = 11,
    VIRTCHNL_OP_ADD_VLAN = 12,
    VIRTCHNL_OP_DEL_VLAN = 13,
    VIRTCHNL_OP_CONFIG_PROMISCUOUS_MODE = 14,
    VIRTCHNL_OP_GET_STATS = 15,
    VIRTCHNL_OP_RSVD = 16,
    VIRTCHNL_OP_EVENT = 17, /* must ALWAYS be 17 */
    VIRTCHNL_OP_CONFIG_RSS_HFUNC = 18,
// opcode 19 is reserved
    VIRTCHNL_OP_IWARP = 20, /* advanced opcode */
    VIRTCHNL_OP_RDMA = VIRTCHNL_OP_IWARP,
    VIRTCHNL_OP_CONFIG_IWARP_IRQ_MAP = 21, /* advanced opcode */
    VIRTCHNL_OP_CONFIG_RDMA_IRQ_MAP = VIRTCHNL_OP_CONFIG_IWARP_IRQ_MAP,
    VIRTCHNL_OP_RELEASE_IWARP_IRQ_MAP = 22, /* advanced opcode */
    VIRTCHNL_OP_RELEASE_RDMA_IRQ_MAP = VIRTCHNL_OP_RELEASE_IWARP_IRQ_MAP,
    VIRTCHNL_OP_CONFIG_RSS_KEY = 23,
    VIRTCHNL_OP_CONFIG_RSS_LUT = 24,
    VIRTCHNL_OP_GET_RSS_HASHCFG_CAPS = 25,
    VIRTCHNL_OP_SET_RSS_HASHCFG = 26,
    VIRTCHNL_OP_ENABLE_VLAN_STRIPPING = 27,
    VIRTCHNL_OP_DISABLE_VLAN_STRIPPING = 28,
    VIRTCHNL_OP_REQUEST_QUEUES = 29,
    VIRTCHNL_OP_ENABLE_CHANNELS = 30,
    VIRTCHNL_OP_DISABLE_CHANNELS = 31,
    VIRTCHNL_OP_ADD_CLOUD_FILTER = 32,
    VIRTCHNL_OP_DEL_CLOUD_FILTER = 33,
// opcode 34 - 43 are reserved
    VIRTCHNL_OP_GET_SUPPORTED_RXDIDS = 44,
    VIRTCHNL_OP_ADD_RSS_CFG = 45,
    VIRTCHNL_OP_DEL_RSS_CFG = 46,
    VIRTCHNL_OP_ADD_FDIR_FILTER = 47,
    VIRTCHNL_OP_DEL_FDIR_FILTER = 48,
    VIRTCHNL_OP_GET_OFFLOAD_VLAN_V2_CAPS = 51,
    VIRTCHNL_OP_ADD_VLAN_V2 = 52,
    VIRTCHNL_OP_DEL_VLAN_V2 = 53,
    VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2 = 54,
    VIRTCHNL_OP_DISABLE_VLAN_STRIPPING_V2 = 55,
    VIRTCHNL_OP_ENABLE_VLAN_INSERTION_V2 = 56,
    VIRTCHNL_OP_DISABLE_VLAN_INSERTION_V2 = 57,
// opcode 58 and 59 are reserved
    VIRTCHNL_OP_1588_PTP_GET_CAPS = 60,
    VIRTCHNL_OP_1588_PTP_GET_TIME = 61,
// opcode 62 - 65 are reserved
    VIRTCHNL_OP_GET_QOS_CAPS = 66,
// opcode 68 through 111 are reserved
    VIRTCHNL_OP_CONFIG_QUEUE_BW = 112,
    VIRTCHNL_OP_CONFIG_QUANTA = 113,
    VIRTCHNL_OP_MAX,
}

// These macros are used to generate compilation errors if a structure/union
// is not exactly the correct length. It gives a divide by zero error if the
// structure/union is not of the correct size, otherwise it creates an enum
// that is never used.
//

// Message descriptions and data structures.
// VIRTCHNL_OP_VERSION
// VF posts its version number to the PF. PF responds with its version number
// in the same format, along with a return code.
// Reply from PF has its major/minor versions also in param0 and param1.
// If there is a major version mismatch, then the VF cannot operate.
// If there is a minor version mismatch, then the VF can operate but should
// add a warning to the system log.
//
// This enum element MUST always be specified as == 1, regardless of other
// changes in the API. The PF must always respond to this message without
// error regardless of version mismatch.
//
pub const VIRTCHNL_VERSION_MAJOR: c_int = 1;
pub const VIRTCHNL_VERSION_MINOR: c_int = 1;
pub const VIRTCHNL_VERSION_MINOR_NO_VF_CAPS: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_version_info {
    pub major: u32,
    pub minor: u32,
}

// VIRTCHNL_OP_RESET_VF
// VF sends this request to PF with no parameters
// PF does NOT respond! VF driver must delay then poll VFGEN_RSTAT register
// until reset completion is indicated. The admin queue must be reinitialized
// after this operation.
//
// When reset is complete, PF must ensure that all queues in all VSIs associated
// with the VF are stopped, all queue configurations in the HMC are set to 0,
// and all MAC and VLAN filters (except the default MAC address) on all VSIs
// are cleared.
//
// VSI types that use VIRTCHNL interface for VF-PF communication. VSI_SRIOV
// vsi_type should always be 6 for backward compatibility. Add other fields
// as needed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_vsi_type {
    VIRTCHNL_VSI_TYPE_INVALID = 0,
    VIRTCHNL_VSI_SRIOV = 6,
}

// VIRTCHNL_OP_GET_VF_RESOURCES
// Version 1.0 VF sends this request to PF with no parameters
// Version 1.1 VF sends this request to PF with u32 bitmap of its capabilities
// PF responds with an indirect message containing
// virtchnl_vf_resource and one or more
// virtchnl_vsi_resource structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vsi_resource {
    pub vsi_id: u16,
    pub num_queue_pairs: u16,
// see enum virtchnl_vsi_type
    pub vsi_type: i32,
    pub qset_handle: u16,
    pub default_mac_addr: [u8; ETH_ALEN],
}

// VF capability flags
// VIRTCHNL_VF_OFFLOAD_L2 flag is inclusive of base mode L2 offloads including
// TX/RX Checksum offloading and TSO for non-tunnelled packets.
//

// used to negotiate communicating link speeds in Mbps

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vf_resource {
    pub num_vsis: u16,
    pub num_queue_pairs: u16,
    pub max_vectors: u16,
    pub max_mtu: u16,
    pub vf_cap_flags: u32,
    pub rss_key_size: u32,
    pub rss_lut_size: u32,
    pub vsi_res: [virtchnl_vsi_resource; ],
}

pub const virtchnl_vf_resource_LEGACY_SIZEOF: c_int = 36;
// VIRTCHNL_OP_CONFIG_TX_QUEUE
// VF sends this message to set up parameters for one TX queue.
// External data buffer contains one instance of virtchnl_txq_info.
// PF configures requested queue and returns a status code.
//
// Tx queue config info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_txq_info {
    pub vsi_id: u16,
    pub queue_id: u16,
    pub /: *mut *mut u16 ring_len; / number of descriptors, multiple of 8,
    pub /: *mut *mut u16 headwb_enabled; / deprecated with AVF 1.0,
    pub dma_ring_addr: u64,
    pub /: *mut *mut u64 dma_headwb_addr; / deprecated with AVF 1.0,
}

// RX descriptor IDs (range from 0 to 63)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_rx_desc_ids {
    VIRTCHNL_RXDID_0_16B_BASE		= 0,
    VIRTCHNL_RXDID_1_32B_BASE		= 1,
    VIRTCHNL_RXDID_2_FLEX_SQ_NIC		= 2,
    VIRTCHNL_RXDID_3_FLEX_SQ_SW		= 3,
    VIRTCHNL_RXDID_4_FLEX_SQ_NIC_VEB	= 4,
    VIRTCHNL_RXDID_5_FLEX_SQ_NIC_ACL	= 5,
    VIRTCHNL_RXDID_6_FLEX_SQ_NIC_2		= 6,
    VIRTCHNL_RXDID_7_HW_RSVD		= 7,
// 8 through 15 are reserved
    VIRTCHNL_RXDID_16_COMMS_GENERIC		= 16,
    VIRTCHNL_RXDID_17_COMMS_AUX_VLAN	= 17,
    VIRTCHNL_RXDID_18_COMMS_AUX_IPV4	= 18,
    VIRTCHNL_RXDID_19_COMMS_AUX_IPV6	= 19,
    VIRTCHNL_RXDID_20_COMMS_AUX_FLOW	= 20,
    VIRTCHNL_RXDID_21_COMMS_AUX_TCP		= 21,
// 22 through 63 are reserved
}

// RX descriptor ID bitmasks
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_rx_desc_id_bitmasks {
    VIRTCHNL_RXDID_0_16B_BASE_M		= VIRTCHNL_RXDID_BIT(0_16B_BASE),
    VIRTCHNL_RXDID_1_32B_BASE_M		= VIRTCHNL_RXDID_BIT(1_32B_BASE),
    VIRTCHNL_RXDID_2_FLEX_SQ_NIC_M		= VIRTCHNL_RXDID_BIT(2_FLEX_SQ_NIC),
    VIRTCHNL_RXDID_3_FLEX_SQ_SW_M		= VIRTCHNL_RXDID_BIT(3_FLEX_SQ_SW),
    VIRTCHNL_RXDID_4_FLEX_SQ_NIC_VEB_M	= VIRTCHNL_RXDID_BIT(4_FLEX_SQ_NIC_VEB),
    VIRTCHNL_RXDID_5_FLEX_SQ_NIC_ACL_M	= VIRTCHNL_RXDID_BIT(5_FLEX_SQ_NIC_ACL),
    VIRTCHNL_RXDID_6_FLEX_SQ_NIC_2_M	= VIRTCHNL_RXDID_BIT(6_FLEX_SQ_NIC_2),
    VIRTCHNL_RXDID_7_HW_RSVD_M		= VIRTCHNL_RXDID_BIT(7_HW_RSVD),
// 8 through 15 are reserved
    VIRTCHNL_RXDID_16_COMMS_GENERIC_M	= VIRTCHNL_RXDID_BIT(16_COMMS_GENERIC),
    VIRTCHNL_RXDID_17_COMMS_AUX_VLAN_M	= VIRTCHNL_RXDID_BIT(17_COMMS_AUX_VLAN),
    VIRTCHNL_RXDID_18_COMMS_AUX_IPV4_M	= VIRTCHNL_RXDID_BIT(18_COMMS_AUX_IPV4),
    VIRTCHNL_RXDID_19_COMMS_AUX_IPV6_M	= VIRTCHNL_RXDID_BIT(19_COMMS_AUX_IPV6),
    VIRTCHNL_RXDID_20_COMMS_AUX_FLOW_M	= VIRTCHNL_RXDID_BIT(20_COMMS_AUX_FLOW),
    VIRTCHNL_RXDID_21_COMMS_AUX_TCP_M	= VIRTCHNL_RXDID_BIT(21_COMMS_AUX_TCP),
// 22 through 63 are reserved
}

// virtchnl_rxq_info_flags - definition of bits in the flags field of the
// virtchnl_rxq_info structure.
//
// @VIRTCHNL_PTP_RX_TSTAMP: request to enable Rx timestamping
//
// Other flag bits are currently reserved and they may be extended in the
// future.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_rxq_info_flags {
    VIRTCHNL_PTP_RX_TSTAMP = BIT(0),
}

// VIRTCHNL_OP_CONFIG_RX_QUEUE
// VF sends this message to set up parameters for one RX queue.
// External data buffer contains one instance of virtchnl_rxq_info.
// PF configures requested queue and returns a status code. The
// crc_disable flag disables CRC stripping on the VF. Setting
// the crc_disable flag to 1 will disable CRC stripping for each
// queue in the VF where the flag is set. The VIRTCHNL_VF_OFFLOAD_CRC
// offload must have been set prior to sending this info or the PF
// will ignore the request. This flag should be set the same for
// all of the queues for a VF.
//
// Rx queue config info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rxq_info {
    pub vsi_id: u16,
    pub queue_id: u16,
    pub /: *mut *mut u32 ring_len; / number of descriptors, multiple of 32,
    pub hdr_size: u16,
    pub /: *mut *mut u16 splithdr_enabled; / deprecated with AVF 1.0,
    pub databuffer_size: u32,
    pub max_pkt_size: u32,
    pub crc_disable: u8,
// see enum virtchnl_rx_desc_ids;
// only used when VIRTCHNL_VF_OFFLOAD_RX_FLEX_DESC is supported. Note
// that when the offload is not supported, the descriptor format aligns
// with VIRTCHNL_RXDID_1_32B_BASE.
//
    pub rxdid:8: virtchnl_rx_desc_ids,
    pub /: *mut *mut virtchnl_rxq_info_flags flags:8; / see virtchnl_rxq_info_flags,
    pub pad1: u8,
    pub dma_ring_addr: u64,
// see enum virtchnl_rx_hsplit; deprecated with AVF 1.0
    pub rx_split_pos: i32,
    pub pad2: u32,
}

// VIRTCHNL_OP_CONFIG_VSI_QUEUES
// VF sends this message to set parameters for all active TX and RX queues
// associated with the specified VSI.
// PF configures queues and returns status.
// If the number of queues specified is greater than the number of queues
// associated with the VSI, an error is returned and no queues are configured.
// NOTE: The VF is not required to configure all queues in a single request.
// It may send multiple messages. PF drivers must correctly handle all VF
// requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_queue_pair_info {
// NOTE: vsi_id and queue_id should be identical for both queues.
    pub txq: virtchnl_txq_info,
    pub rxq: virtchnl_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vsi_queue_config_info {
    pub vsi_id: u16,
    pub num_queue_pairs: u16,
    pub pad: u32,
    pub qpair: [virtchnl_queue_pair_info; ],
}

pub const virtchnl_vsi_queue_config_info_LEGACY_SIZEOF: c_int = 72;
// VIRTCHNL_OP_REQUEST_QUEUES
// VF sends this message to request the PF to allocate additional queues to
// this VF.  Each VF gets a guaranteed number of queues on init but asking for
// additional queues must be negotiated.  This is a best effort request as it
// is possible the PF does not have enough queues left to support the request.
// If the PF cannot support the number requested it will respond with the
// maximum number it is able to support.  If the request is successful, PF will
// then reset the VF to institute required changes.
//
// VF resource request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vf_res_request {
    pub num_queue_pairs: u16,
}

// VIRTCHNL_OP_CONFIG_IRQ_MAP
// VF uses this message to map vectors to queues.
// The rxq_map and txq_map fields are bitmaps used to indicate which queues
// are to be associated with the specified vector.
// The "other" causes are always mapped to vector 0. The VF may not request
// that vector 0 be used for traffic.
// PF configures interrupt mapping and returns status.
// NOTE: due to hardware requirements, all active queues (both TX and RX)
// should be mapped to interrupts, even if the driver intends to operate
// only in polling mode. In this case the interrupt may be disabled, but
// the ITR timer will still run to trigger writebacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vector_map {
    pub vsi_id: u16,
    pub vector_id: u16,
    pub rxq_map: u16,
    pub txq_map: u16,
    pub rxitr_idx: u16,
    pub txitr_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_irq_map_info {
    pub num_vectors: u16,
    pub vecmap: [virtchnl_vector_map; ],
}

pub const virtchnl_irq_map_info_LEGACY_SIZEOF: c_int = 14;
// VIRTCHNL_OP_ENABLE_QUEUES
// VIRTCHNL_OP_DISABLE_QUEUES
// VF sends these message to enable or disable TX/RX queue pairs.
// The queues fields are bitmaps indicating which queues to act upon.
// (Currently, we only support 16 queues per VF, but we make the field
// u32 to allow for expansion.)
// PF performs requested action and returns status.
// NOTE: The VF is not required to enable/disable all queues in a single
// request. It may send multiple messages.
// PF drivers must correctly handle all VF requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_queue_select {
    pub vsi_id: u16,
    pub pad: u16,
    pub rx_queues: u32,
    pub tx_queues: u32,
}

// VIRTCHNL_OP_ADD_ETH_ADDR
// VF sends this message in order to add one or more unicast or multicast
// address filters for the specified VSI.
// PF adds the filters and returns status.
//
// VIRTCHNL_OP_DEL_ETH_ADDR
// VF sends this message in order to remove one or more unicast or multicast
// filters for the specified VSI.
// PF removes the filters and returns status.
//
// VIRTCHNL_ETHER_ADDR_LEGACY
// Prior to adding the @type member to virtchnl_ether_addr, there were 2 pad
// bytes. Moving forward all VF drivers should not set type to
// VIRTCHNL_ETHER_ADDR_LEGACY. This is only here to not break previous/legacy
// behavior. The control plane function (i.e. PF) can use a best effort method
// of tracking the primary/device unicast in this case, but there is no
// guarantee and functionality depends on the implementation of the PF.
//
// VIRTCHNL_ETHER_ADDR_PRIMARY
// All VF drivers should set @type to VIRTCHNL_ETHER_ADDR_PRIMARY for the
// primary/device unicast MAC address filter for VIRTCHNL_OP_ADD_ETH_ADDR and
// VIRTCHNL_OP_DEL_ETH_ADDR. This allows for the underlying control plane
// function (i.e. PF) to accurately track and use this MAC address for
// displaying on the host and for VM/function reset.
//
// VIRTCHNL_ETHER_ADDR_EXTRA
// All VF drivers should set @type to VIRTCHNL_ETHER_ADDR_EXTRA for any extra
// unicast and/or multicast filters that are being added/deleted via
// VIRTCHNL_OP_DEL_ETH_ADDR/VIRTCHNL_OP_ADD_ETH_ADDR respectively.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_ether_addr {
    pub addr: [u8; ETH_ALEN],
    pub type: u8,
pub const VIRTCHNL_ETHER_ADDR_LEGACY: c_int = 0;
pub const VIRTCHNL_ETHER_ADDR_PRIMARY: c_int = 1;
pub const VIRTCHNL_ETHER_ADDR_EXTRA: c_int = 2;

    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_ether_addr_list {
    pub vsi_id: u16,
    pub num_elements: u16,
    pub list: [virtchnl_ether_addr; ],
}

pub const virtchnl_ether_addr_list_LEGACY_SIZEOF: c_int = 12;
// VIRTCHNL_OP_ADD_VLAN
// VF sends this message to add one or more VLAN tag filters for receives.
// PF adds the filters and returns status.
// If a port VLAN is configured by the PF, this operation will return an
// error to the VF.
//
// VIRTCHNL_OP_DEL_VLAN
// VF sends this message to remove one or more VLAN tag filters for receives.
// PF removes the filters and returns status.
// If a port VLAN is configured by the PF, this operation will return an
// error to the VF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_filter_list {
    pub vsi_id: u16,
    pub num_elements: u16,
    pub vlan_id: [u16; ],
}

pub const virtchnl_vlan_filter_list_LEGACY_SIZEOF: c_int = 6;
// This enum is used for all of the VIRTCHNL_VF_OFFLOAD_VLAN_V2_CAPS related
// structures and opcodes.
//
// VIRTCHNL_VLAN_UNSUPPORTED - This field is not supported and if a VF driver
// populates it the PF should return VIRTCHNL_STATUS_ERR_NOT_SUPPORTED.
//
// VIRTCHNL_VLAN_ETHERTYPE_8100 - This field supports 0x8100 ethertype.
// VIRTCHNL_VLAN_ETHERTYPE_88A8 - This field supports 0x88A8 ethertype.
// VIRTCHNL_VLAN_ETHERTYPE_9100 - This field supports 0x9100 ethertype.
//
// VIRTCHNL_VLAN_ETHERTYPE_AND - Used when multiple ethertypes can be supported
// by the PF concurrently. For example, if the PF can support
// VIRTCHNL_VLAN_ETHERTYPE_8100 AND VIRTCHNL_VLAN_ETHERTYPE_88A8 filters it
// would OR the following bits:
//
// VIRTHCNL_VLAN_ETHERTYPE_8100 |
// VIRTCHNL_VLAN_ETHERTYPE_88A8 |
// VIRTCHNL_VLAN_ETHERTYPE_AND;
//
// The VF would interpret this as VLAN filtering can be supported on both 0x8100
// and 0x88A8 VLAN ethertypes.
//
// VIRTCHNL_ETHERTYPE_XOR - Used when only a single ethertype can be supported
// by the PF concurrently. For example if the PF can support
// VIRTCHNL_VLAN_ETHERTYPE_8100 XOR VIRTCHNL_VLAN_ETHERTYPE_88A8 stripping
// offload it would OR the following bits:
//
// VIRTCHNL_VLAN_ETHERTYPE_8100 |
// VIRTCHNL_VLAN_ETHERTYPE_88A8 |
// VIRTCHNL_VLAN_ETHERTYPE_XOR;
//
// The VF would interpret this as VLAN stripping can be supported on either
// 0x8100 or 0x88a8 VLAN ethertypes. So when requesting VLAN stripping via
// VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2 the specified ethertype will override
// the previously set value.
//
// VIRTCHNL_VLAN_TAG_LOCATION_L2TAG1 - Used to tell the VF to insert and/or
// strip the VLAN tag using the L2TAG1 field of the Tx/Rx descriptors.
//
// VIRTCHNL_VLAN_TAG_LOCATION_L2TAG2 - Used to tell the VF to insert hardware
// offloaded VLAN tags using the L2TAG2 field of the Tx descriptor.
//
// VIRTCHNL_VLAN_TAG_LOCATION_L2TAG2 - Used to tell the VF to strip hardware
// offloaded VLAN tags using the L2TAG2_2 field of the Rx descriptor.
//
// VIRTCHNL_VLAN_PRIO - This field supports VLAN priority bits. This is used for
// VLAN filtering if the underlying PF supports it.
//
// VIRTCHNL_VLAN_TOGGLE_ALLOWED - This field is used to say whether a
// certain VLAN capability can be toggled. For example if the underlying PF/CP
// allows the VF to toggle VLAN filtering, stripping, and/or insertion it should
// set this bit along with the supported ethertypes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_vlan_support {
    VIRTCHNL_VLAN_UNSUPPORTED =		0,
    VIRTCHNL_VLAN_ETHERTYPE_8100 =		BIT(0),
    VIRTCHNL_VLAN_ETHERTYPE_88A8 =		BIT(1),
    VIRTCHNL_VLAN_ETHERTYPE_9100 =		BIT(2),
    VIRTCHNL_VLAN_TAG_LOCATION_L2TAG1 =	BIT(8),
    VIRTCHNL_VLAN_TAG_LOCATION_L2TAG2 =	BIT(9),
    VIRTCHNL_VLAN_TAG_LOCATION_L2TAG2_2 =	BIT(10),
    VIRTCHNL_VLAN_PRIO =			BIT(24),
    VIRTCHNL_VLAN_FILTER_MASK =		BIT(28),
    VIRTCHNL_VLAN_ETHERTYPE_AND =		BIT(29),
    VIRTCHNL_VLAN_ETHERTYPE_XOR =		BIT(30),
    VIRTCHNL_VLAN_TOGGLE =			BIT(31),
}

// This structure is used as part of the VIRTCHNL_OP_GET_OFFLOAD_VLAN_V2_CAPS
// for filtering, insertion, and stripping capabilities.
//
// If only outer capabilities are supported (for filtering, insertion, and/or
// stripping) then this refers to the outer most or single VLAN from the VF's
// perspective.
//
// If only inner capabilities are supported (for filtering, insertion, and/or
// stripping) then this refers to the outer most or single VLAN from the VF's
// perspective. Functionally this is the same as if only outer capabilities are
// supported. The VF driver is just forced to use the inner fields when
// adding/deleting filters and enabling/disabling offloads (if supported).
//
// If both outer and inner capabilities are supported (for filtering, insertion,
// and/or stripping) then outer refers to the outer most or single VLAN and
// inner refers to the second VLAN, if it exists, in the packet.
//
// There is no support for tunneled VLAN offloads, so outer or inner are never
// referring to a tunneled packet from the VF's perspective.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_supported_caps {
    pub outer: u32,
    pub inner: u32,
}

// The PF populates these fields based on the supported VLAN filtering. If a
// field is VIRTCHNL_VLAN_UNSUPPORTED then it's not supported and the PF will
// reject any VIRTCHNL_OP_ADD_VLAN_V2 or VIRTCHNL_OP_DEL_VLAN_V2 messages using
// the unsupported fields.
//
// Also, a VF is only allowed to toggle its VLAN filtering setting if the
// VIRTCHNL_VLAN_TOGGLE bit is set.
//
// The ethertype(s) specified in the ethertype_init field are the ethertypes
// enabled for VLAN filtering. VLAN filtering in this case refers to the outer
// most VLAN from the VF's perspective. If both inner and outer filtering are
// allowed then ethertype_init only refers to the outer most VLAN as only
// VLAN ethertype supported for inner VLAN filtering is
// VIRTCHNL_VLAN_ETHERTYPE_8100. By default, inner VLAN filtering is disabled
// when both inner and outer filtering are allowed.
//
// The max_filters field tells the VF how many VLAN filters it's allowed to have
// at any one time. If it exceeds this amount and tries to add another filter,
// then the request will be rejected by the PF. To prevent failures, the VF
// should keep track of how many VLAN filters it has added and not attempt to
// add more than max_filters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_filtering_caps {
    pub filtering_support: virtchnl_vlan_supported_caps,
    pub ethertype_init: u32,
    pub max_filters: u16,
    pub pad: [u8; 2],
}

// This enum is used for the virtchnl_vlan_offload_caps structure to specify
// if the PF supports a different ethertype for stripping and insertion.
//
// VIRTCHNL_ETHERTYPE_STRIPPING_MATCHES_INSERTION - The ethertype(s) specified
// for stripping affect the ethertype(s) specified for insertion and visa versa
// as well. If the VF tries to configure VLAN stripping via
// VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2 with VIRTCHNL_VLAN_ETHERTYPE_8100 then
// that will be the ethertype for both stripping and insertion.
//
// VIRTCHNL_ETHERTYPE_MATCH_NOT_REQUIRED - The ethertype(s) specified for
// stripping do not affect the ethertype(s) specified for insertion and visa
// versa.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_vlan_ethertype_match {
    VIRTCHNL_ETHERTYPE_STRIPPING_MATCHES_INSERTION = 0,
    VIRTCHNL_ETHERTYPE_MATCH_NOT_REQUIRED = 1,
}

// The PF populates these fields based on the supported VLAN offloads. If a
// field is VIRTCHNL_VLAN_UNSUPPORTED then it's not supported and the PF will
// reject any VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2 or
// VIRTCHNL_OP_DISABLE_VLAN_STRIPPING_V2 messages using the unsupported fields.
//
// Also, a VF is only allowed to toggle its VLAN offload setting if the
// VIRTCHNL_VLAN_TOGGLE_ALLOWED bit is set.
//
// The VF driver needs to be aware of how the tags are stripped by hardware and
// inserted by the VF driver based on the level of offload support. The PF will
// populate these fields based on where the VLAN tags are expected to be
// offloaded via the VIRTHCNL_VLAN_TAG_LOCATION_* bits. The VF will need to
// interpret these fields. See the definition of the
// VIRTCHNL_VLAN_TAG_LOCATION_* bits above the virtchnl_vlan_support
// enumeration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_offload_caps {
    pub stripping_support: virtchnl_vlan_supported_caps,
    pub insertion_support: virtchnl_vlan_supported_caps,
    pub ethertype_init: u32,
    pub ethertype_match: u8,
    pub pad: [u8; 3],
}

// VIRTCHNL_OP_GET_OFFLOAD_VLAN_V2_CAPS
// VF sends this message to determine its VLAN capabilities.
//
// PF will mark which capabilities it supports based on hardware support and
// current configuration. For example, if a port VLAN is configured the PF will
// not allow outer VLAN filtering, stripping, or insertion to be configured so
// it will block these features from the VF.
//
// The VF will need to cross reference its capabilities with the PFs
// capabilities in the response message from the PF to determine the VLAN
// support.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_caps {
    pub filtering: virtchnl_vlan_filtering_caps,
    pub offloads: virtchnl_vlan_offload_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan {
    pub /: *mut *mut u16 tci; / tci[15:13] = PCP and tci[11:0] = VID,
    pub in: *mut *mut u16 tci_mask; / only valid if VIRTCHNL_VLAN_FILTER_MASK set,
// filtering caps
//
    pub in: *mut *mut u16 tpid; / 0x8100, 0x88a8, etc. and only type(s) set,
// filtering caps. Note that tpid here does not refer to
// VIRTCHNL_VLAN_ETHERTYPE_*, but it refers to the
// actual 2-byte VLAN TPID
//
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_filter {
    pub inner: virtchnl_vlan,
    pub outer: virtchnl_vlan,
    pub pad: [u8; 16],
}

// VIRTCHNL_OP_ADD_VLAN_V2
// VIRTCHNL_OP_DEL_VLAN_V2
//
// VF sends these messages to add/del one or more VLAN tag filters for Rx
// traffic.
//
// The PF attempts to add the filters and returns status.
//
// The VF should only ever attempt to add/del virtchnl_vlan_filter(s) using the
// supported fields negotiated via VIRTCHNL_OP_GET_OFFLOAD_VLAN_V2_CAPS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_filter_list_v2 {
    pub vport_id: u16,
    pub num_elements: u16,
    pub pad: [u8; 4],
    pub filters: [virtchnl_vlan_filter; ],
}

pub const virtchnl_vlan_filter_list_v2_LEGACY_SIZEOF: c_int = 40;
// VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2
// VIRTCHNL_OP_DISABLE_VLAN_STRIPPING_V2
// VIRTCHNL_OP_ENABLE_VLAN_INSERTION_V2
// VIRTCHNL_OP_DISABLE_VLAN_INSERTION_V2
//
// VF sends this message to enable or disable VLAN stripping or insertion. It
// also needs to specify an ethertype. The VF knows which VLAN ethertypes are
// allowed and whether or not it's allowed to enable/disable the specific
// offload via the VIRTCHNL_OP_GET_OFFLOAD_VLAN_V2_CAPS message. The VF needs to
// parse the virtchnl_vlan_caps.offloads fields to determine which offload
// messages are allowed.
//
// For example, if the PF populates the virtchnl_vlan_caps.offloads in the
// following manner the VF will be allowed to enable and/or disable 0x8100 inner
// VLAN insertion and/or stripping via the opcodes listed above. Inner in this
// case means the outer most or single VLAN from the VF's perspective. This is
// because no outer offloads are supported. See the comments above the
// virtchnl_vlan_supported_caps structure for more details.
//
// virtchnl_vlan_caps.offloads.stripping_support.inner =
// VIRTCHNL_VLAN_TOGGLE |
// VIRTCHNL_VLAN_ETHERTYPE_8100;
//
// virtchnl_vlan_caps.offloads.insertion_support.inner =
// VIRTCHNL_VLAN_TOGGLE |
// VIRTCHNL_VLAN_ETHERTYPE_8100;
//
// In order to enable inner (again note that in this case inner is the outer
// most or single VLAN from the VF's perspective) VLAN stripping for 0x8100
// VLANs, the VF would populate the virtchnl_vlan_setting structure in the
// following manner and send the VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2 message.
//
// virtchnl_vlan_setting.inner_ethertype_setting =
// VIRTCHNL_VLAN_ETHERTYPE_8100;
//
// virtchnl_vlan_setting.vport_id = vport_id or vsi_id assigned to the VF on
// initialization.
//
// The reason that VLAN TPID(s) are not being used for the
// outer_ethertype_setting and inner_ethertype_setting fields is because it's
// possible a device could support VLAN insertion and/or stripping offload on
// multiple ethertypes concurrently, so this method allows a VF to request
// multiple ethertypes in one message using the virtchnl_vlan_support
// enumeration.
//
// For example, if the PF populates the virtchnl_vlan_caps.offloads in the
// following manner the VF will be allowed to enable 0x8100 and 0x88a8 outer
// VLAN insertion and stripping simultaneously. The
// virtchnl_vlan_caps.offloads.ethertype_match field will also have to be
// populated based on what the PF can support.
//
// virtchnl_vlan_caps.offloads.stripping_support.outer =
// VIRTCHNL_VLAN_TOGGLE |
// VIRTCHNL_VLAN_ETHERTYPE_8100 |
// VIRTCHNL_VLAN_ETHERTYPE_88A8 |
// VIRTCHNL_VLAN_ETHERTYPE_AND;
//
// virtchnl_vlan_caps.offloads.insertion_support.outer =
// VIRTCHNL_VLAN_TOGGLE |
// VIRTCHNL_VLAN_ETHERTYPE_8100 |
// VIRTCHNL_VLAN_ETHERTYPE_88A8 |
// VIRTCHNL_VLAN_ETHERTYPE_AND;
//
// In order to enable outer VLAN stripping for 0x8100 and 0x88a8 VLANs, the VF
// would populate the virthcnl_vlan_offload_structure in the following manner
// and send the VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2 message.
//
// virtchnl_vlan_setting.outer_ethertype_setting =
// VIRTHCNL_VLAN_ETHERTYPE_8100 |
// VIRTHCNL_VLAN_ETHERTYPE_88A8;
//
// virtchnl_vlan_setting.vport_id = vport_id or vsi_id assigned to the VF on
// initialization.
//
// There is also the case where a PF and the underlying hardware can support
// VLAN offloads on multiple ethertypes, but not concurrently. For example, if
// the PF populates the virtchnl_vlan_caps.offloads in the following manner the
// VF will be allowed to enable and/or disable 0x8100 XOR 0x88a8 outer VLAN
// offloads. The ethertypes must match for stripping and insertion.
//
// virtchnl_vlan_caps.offloads.stripping_support.outer =
// VIRTCHNL_VLAN_TOGGLE |
// VIRTCHNL_VLAN_ETHERTYPE_8100 |
// VIRTCHNL_VLAN_ETHERTYPE_88A8 |
// VIRTCHNL_VLAN_ETHERTYPE_XOR;
//
// virtchnl_vlan_caps.offloads.insertion_support.outer =
// VIRTCHNL_VLAN_TOGGLE |
// VIRTCHNL_VLAN_ETHERTYPE_8100 |
// VIRTCHNL_VLAN_ETHERTYPE_88A8 |
// VIRTCHNL_VLAN_ETHERTYPE_XOR;
//
// virtchnl_vlan_caps.offloads.ethertype_match =
// VIRTCHNL_ETHERTYPE_STRIPPING_MATCHES_INSERTION;
//
// In order to enable outer VLAN stripping for 0x88a8 VLANs, the VF would
// populate the virtchnl_vlan_setting structure in the following manner and send
// the VIRTCHNL_OP_ENABLE_VLAN_STRIPPING_V2. Also, this will change the
// ethertype for VLAN insertion if it's enabled. So, for completeness, a
// VIRTCHNL_OP_ENABLE_VLAN_INSERTION_V2 with the same ethertype should be sent.
//
// virtchnl_vlan_setting.outer_ethertype_setting = VIRTHCNL_VLAN_ETHERTYPE_88A8;
//
// virtchnl_vlan_setting.vport_id = vport_id or vsi_id assigned to the VF on
// initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_vlan_setting {
    pub outer_ethertype_setting: u32,
    pub inner_ethertype_setting: u32,
    pub vport_id: u16,
    pub pad: [u8; 6],
}

// VIRTCHNL_OP_CONFIG_PROMISCUOUS_MODE
// VF sends VSI id and flags.
// PF returns status code in retval.
// Note: we assume that broadcast accept mode is always enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_promisc_info {
    pub vsi_id: u16,
    pub flags: u16,
}

pub const FLAG_VF_UNICAST_PROMISC: c_uint = 0x00000001;
pub const FLAG_VF_MULTICAST_PROMISC: c_uint = 0x00000002;
// VIRTCHNL_OP_GET_STATS
// VF sends this message to request stats for the selected VSI. VF uses
// the virtchnl_queue_select struct to specify the VSI. The queue_id
// field is ignored by the PF.
//
// PF replies with struct eth_stats in an external buffer.
//
// VIRTCHNL_OP_CONFIG_RSS_KEY
// VIRTCHNL_OP_CONFIG_RSS_LUT
// VF sends these messages to configure RSS. Only supported if both PF
// and VF drivers set the VIRTCHNL_VF_OFFLOAD_RSS_PF bit during
// configuration negotiation. If this is the case, then the RSS fields in
// the VF resource struct are valid.
// Both the key and LUT are initialized to 0 by the PF, meaning that
// RSS is effectively disabled until set up by the VF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rss_key {
    pub vsi_id: u16,
    pub key_len: u16,
    pub /: *mut *mut u8 key[]; / RSS hash key, packed bytes,
}

pub const virtchnl_rss_key_LEGACY_SIZEOF: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rss_lut {
    pub vsi_id: u16,
    pub lut_entries: u16,
    pub /: *mut *mut u8 lut[]; / RSS lookup table,
}

pub const virtchnl_rss_lut_LEGACY_SIZEOF: c_int = 6;
// VIRTCHNL_OP_GET_RSS_HASHCFG_CAPS
// VIRTCHNL_OP_SET_RSS_HASHCFG
// VF sends these messages to get and set the hash filter configuration for RSS.
// By default, the PF sets these to all possible traffic types that the
// hardware supports. The VF can query this value if it wants to change the
// traffic types that are hashed by the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rss_hashcfg {
// Bits defined by enum libie_filter_pctype
    pub hashcfg: u64,
}

// Type of RSS algorithm
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_rss_algorithm {
    VIRTCHNL_RSS_ALG_TOEPLITZ_ASYMMETRIC	= 0,
    VIRTCHNL_RSS_ALG_R_ASYMMETRIC		= 1,
    VIRTCHNL_RSS_ALG_TOEPLITZ_SYMMETRIC	= 2,
    VIRTCHNL_RSS_ALG_XOR_SYMMETRIC		= 3,
}

// VIRTCHNL_OP_CONFIG_RSS_HFUNC
// VF sends this message to configure the RSS hash function. Only supported
// if both PF and VF drivers set the VIRTCHNL_VF_OFFLOAD_RSS_PF bit during
// configuration negotiation.
// The hash function is initialized to VIRTCHNL_RSS_ALG_TOEPLITZ_ASYMMETRIC
// by the PF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rss_hfunc {
    pub vsi_id: u16,
    pub /: *mut *mut u16 rss_algorithm; / enum virtchnl_rss_algorithm,
    pub reserved: u32,
}

// VIRTCHNL_OP_ENABLE_CHANNELS
// VIRTCHNL_OP_DISABLE_CHANNELS
// VF sends these messages to enable or disable channels based on
// the user specified queue count and queue offset for each traffic class.
// This struct encompasses all the information that the PF needs from
// VF to create a channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_channel_info {
    pub /: *mut *mut u16 count; / number of queues in a channel,
    pub /: *mut *mut u16 offset; / queues in a channel start from 'offset',
    pub pad: u32,
    pub max_tx_rate: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_tc_info {
    pub num_tc: u32,
    pub pad: u32,
    pub list: [virtchnl_channel_info; ],
}

pub const virtchnl_tc_info_LEGACY_SIZEOF: c_int = 24;
// VIRTCHNL_ADD_CLOUD_FILTER
// VIRTCHNL_DEL_CLOUD_FILTER
// VF sends these messages to add or delete a cloud filter based on the
// user specified match and action filters. These structures encompass
// all the information that the PF needs from the VF to add/delete a
// cloud filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_l4_spec {
    pub src_mac: [u8; ETH_ALEN],
    pub dst_mac: [u8; ETH_ALEN],
    pub vlan_id: __be16,
    pub /: *mut *mut __be16 pad; / reserved for future use,
    pub src_ip: [__be32; 4],
    pub dst_ip: [__be32; 4],
    pub src_port: __be16,
    pub dst_port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union virtchnl_flow_spec {
    pub tcp_spec: virtchnl_l4_spec,
    pub /: *mut *mut u8 buffer[128]; / reserved for future use,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_action {
// action types
    VIRTCHNL_ACTION_DROP = 0,
    VIRTCHNL_ACTION_TC_REDIRECT,
    VIRTCHNL_ACTION_PASSTHRU,
    VIRTCHNL_ACTION_QUEUE,
    VIRTCHNL_ACTION_Q_REGION,
    VIRTCHNL_ACTION_MARK,
    VIRTCHNL_ACTION_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_flow_type {
// flow types
    VIRTCHNL_TCP_V4_FLOW = 0,
    VIRTCHNL_TCP_V6_FLOW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_filter {
    pub data: virtchnl_flow_spec,
    pub mask: virtchnl_flow_spec,
// see enum virtchnl_flow_type
    pub flow_type: i32,
// see enum virtchnl_action
    pub action: i32,
    pub action_meta: u32,
    pub field_flags: u8,
    pub pad: [u8; 3],
}

// VIRTCHNL_OP_EVENT
// PF sends this message to inform the VF driver of events that may affect it.
// No direct response is expected from the VF, though it may generate other
// messages in response to this one.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_event_codes {
    VIRTCHNL_EVENT_UNKNOWN = 0,
    VIRTCHNL_EVENT_LINK_CHANGE,
    VIRTCHNL_EVENT_RESET_IMPENDING,
    VIRTCHNL_EVENT_PF_DRIVER_CLOSE,
}

pub const PF_EVENT_SEVERITY_INFO: c_int = 0;
pub const PF_EVENT_SEVERITY_CERTAIN_DOOM: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_pf_event {
// see enum virtchnl_event_codes
    pub event: i32,
// If the PF driver does not support the new speed reporting
// capabilities then use link_event else use link_event_adv to
// get the speed and link information. The ability to understand
// new speeds is indicated by setting the capability flag
// VIRTCHNL_VF_CAP_ADV_LINK_SPEED in vf_cap_flags parameter
// in virtchnl_vf_resource struct and can be used to determine
// which link event struct to use below.
//
    pub link_speed: virtchnl_link_speed,
    pub link_status: bool,
    pub pad: [u8; 3],
    pub link_event: },
// link_speed provided in Mbps
    pub link_speed: u32,
    pub link_status: u8,
    pub pad: [u8; 3],
    pub link_event_adv: },
    pub event_data: },
    pub severity: i32,
}

// used to specify if a ceq_idx or aeq_idx is invalid
pub const VIRTCHNL_RDMA_INVALID_QUEUE_IDX: c_uint = 0xFFFF;
// VIRTCHNL_OP_CONFIG_RDMA_IRQ_MAP
// VF uses this message to request PF to map RDMA vectors to RDMA queues.
// The request for this originates from the VF RDMA driver through
// a client interface between VF LAN and VF RDMA driver.
// A vector could have an AEQ and CEQ attached to it although
// there is a single AEQ per VF RDMA instance in which case
// most vectors will have an VIRTCHNL_RDMA_INVALID_QUEUE_IDX for aeq and valid
// idx for ceqs There will never be a case where there will be multiple CEQs
// attached to a single vector.
// PF configures interrupt mapping and returns status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rdma_qv_info {
    pub /: *mut *mut u32 v_idx; / msix_vector,
    pub /: *mut *mut u16 ceq_idx; / set to VIRTCHNL_RDMA_INVALID_QUEUE_IDX if invalid,
    pub /: *mut *mut u16 aeq_idx; / set to VIRTCHNL_RDMA_INVALID_QUEUE_IDX if invalid,
    pub itr_idx: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rdma_qvlist_info {
    pub num_vectors: u32,
    pub qv_info: [virtchnl_rdma_qv_info; ],
}

pub const virtchnl_rdma_qvlist_info_LEGACY_SIZEOF: c_int = 16;
// VF reset states - these are written into the RSTAT register:
// VFGEN_RSTAT on the VF
// When the PF initiates a reset, it writes 0
// When the reset is complete, it writes 1
// When the PF detects that the VF has recovered, it writes 2
// VF checks this register periodically to determine if a reset has occurred,
// then polls it to know when the reset is complete.
// If either the PF or VF reads the register while the hardware
// is in a reset state, it will return DEADBEEF, which, when masked
// will result in 3.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_vfr_states {
    VIRTCHNL_VFR_INPROGRESS = 0,
    VIRTCHNL_VFR_COMPLETED,
    VIRTCHNL_VFR_VFACTIVE,
}

pub const VIRTCHNL_MAX_NUM_PROTO_HDRS: c_int = 32;
pub const VIRTCHNL_MAX_SIZE_RAW_PACKET: c_int = 1024;
pub const PROTO_HDR_SHIFT: c_int = 5;

// VF use these macros to configure each protocol header.
// Specify which protocol headers and protocol header fields base on
// virtchnl_proto_hdr_type and virtchnl_proto_hdr_field.
// @param hdr: a struct of virtchnl_proto_hdr
// @param hdr_type: ETH/IPV4/TCP, etc
// @param field: SRC/DST/TEID/SPI, etc
//

// Protocol header type within a packet segment. A segment consists of one or
// more protocol headers that make up a logical group of protocol headers. Each
// logical group of protocol headers encapsulates or is encapsulated using/by
// tunneling or encapsulation protocols for network virtualization.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_proto_hdr_type {
    VIRTCHNL_PROTO_HDR_NONE,
    VIRTCHNL_PROTO_HDR_ETH,
    VIRTCHNL_PROTO_HDR_S_VLAN,
    VIRTCHNL_PROTO_HDR_C_VLAN,
    VIRTCHNL_PROTO_HDR_IPV4,
    VIRTCHNL_PROTO_HDR_IPV6,
    VIRTCHNL_PROTO_HDR_TCP,
    VIRTCHNL_PROTO_HDR_UDP,
    VIRTCHNL_PROTO_HDR_SCTP,
    VIRTCHNL_PROTO_HDR_GTPU_IP,
    VIRTCHNL_PROTO_HDR_GTPU_EH,
    VIRTCHNL_PROTO_HDR_GTPU_EH_PDU_DWN,
    VIRTCHNL_PROTO_HDR_GTPU_EH_PDU_UP,
    VIRTCHNL_PROTO_HDR_PPPOE,
    VIRTCHNL_PROTO_HDR_L2TPV3,
    VIRTCHNL_PROTO_HDR_ESP,
    VIRTCHNL_PROTO_HDR_AH,
    VIRTCHNL_PROTO_HDR_PFCP,
    VIRTCHNL_PROTO_HDR_GTPC,
    VIRTCHNL_PROTO_HDR_ECPRI,
    VIRTCHNL_PROTO_HDR_L2TPV2,
    VIRTCHNL_PROTO_HDR_PPP,
// IPv4 and IPv6 Fragment header types are only associated to
// VIRTCHNL_PROTO_HDR_IPV4 and VIRTCHNL_PROTO_HDR_IPV6 respectively,
// cannot be used independently.
//
    VIRTCHNL_PROTO_HDR_IPV4_FRAG,
    VIRTCHNL_PROTO_HDR_IPV6_EH_FRAG,
    VIRTCHNL_PROTO_HDR_GRE,
}

// Protocol header field within a protocol header.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_proto_hdr_field {
// ETHER
    VIRTCHNL_PROTO_HDR_ETH_SRC =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_ETH),
    VIRTCHNL_PROTO_HDR_ETH_DST,
    VIRTCHNL_PROTO_HDR_ETH_ETHERTYPE,
// S-VLAN
    VIRTCHNL_PROTO_HDR_S_VLAN_ID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_S_VLAN),
// C-VLAN
    VIRTCHNL_PROTO_HDR_C_VLAN_ID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_C_VLAN),
// IPV4
    VIRTCHNL_PROTO_HDR_IPV4_SRC =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_IPV4),
    VIRTCHNL_PROTO_HDR_IPV4_DST,
    VIRTCHNL_PROTO_HDR_IPV4_DSCP,
    VIRTCHNL_PROTO_HDR_IPV4_TTL,
    VIRTCHNL_PROTO_HDR_IPV4_PROT,
    VIRTCHNL_PROTO_HDR_IPV4_CHKSUM,
// IPV6
    VIRTCHNL_PROTO_HDR_IPV6_SRC =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_IPV6),
    VIRTCHNL_PROTO_HDR_IPV6_DST,
    VIRTCHNL_PROTO_HDR_IPV6_TC,
    VIRTCHNL_PROTO_HDR_IPV6_HOP_LIMIT,
    VIRTCHNL_PROTO_HDR_IPV6_PROT,
// IPV6 Prefix
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX32_SRC,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX32_DST,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX40_SRC,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX40_DST,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX48_SRC,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX48_DST,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX56_SRC,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX56_DST,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX64_SRC,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX64_DST,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX96_SRC,
    VIRTCHNL_PROTO_HDR_IPV6_PREFIX96_DST,
// TCP
    VIRTCHNL_PROTO_HDR_TCP_SRC_PORT =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_TCP),
    VIRTCHNL_PROTO_HDR_TCP_DST_PORT,
    VIRTCHNL_PROTO_HDR_TCP_CHKSUM,
// UDP
    VIRTCHNL_PROTO_HDR_UDP_SRC_PORT =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_UDP),
    VIRTCHNL_PROTO_HDR_UDP_DST_PORT,
    VIRTCHNL_PROTO_HDR_UDP_CHKSUM,
// SCTP
    VIRTCHNL_PROTO_HDR_SCTP_SRC_PORT =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_SCTP),
    VIRTCHNL_PROTO_HDR_SCTP_DST_PORT,
    VIRTCHNL_PROTO_HDR_SCTP_CHKSUM,
// GTPU_IP
    VIRTCHNL_PROTO_HDR_GTPU_IP_TEID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_GTPU_IP),
// GTPU_EH
    VIRTCHNL_PROTO_HDR_GTPU_EH_PDU =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_GTPU_EH),
    VIRTCHNL_PROTO_HDR_GTPU_EH_QFI,
// PPPOE
    VIRTCHNL_PROTO_HDR_PPPOE_SESS_ID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_PPPOE),
// L2TPV3
    VIRTCHNL_PROTO_HDR_L2TPV3_SESS_ID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_L2TPV3),
// ESP
    VIRTCHNL_PROTO_HDR_ESP_SPI =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_ESP),
// AH
    VIRTCHNL_PROTO_HDR_AH_SPI =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_AH),
// PFCP
    VIRTCHNL_PROTO_HDR_PFCP_S_FIELD =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_PFCP),
    VIRTCHNL_PROTO_HDR_PFCP_SEID,
// GTPC
    VIRTCHNL_PROTO_HDR_GTPC_TEID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_GTPC),
// ECPRI
    VIRTCHNL_PROTO_HDR_ECPRI_MSG_TYPE =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_ECPRI),
    VIRTCHNL_PROTO_HDR_ECPRI_PC_RTC_ID,
// IPv4 Dummy Fragment
    VIRTCHNL_PROTO_HDR_IPV4_FRAG_PKID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_IPV4_FRAG),
// IPv6 Extension Fragment
    VIRTCHNL_PROTO_HDR_IPV6_EH_FRAG_PKID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_IPV6_EH_FRAG),
// GTPU_DWN/UP
    VIRTCHNL_PROTO_HDR_GTPU_DWN_QFI =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_GTPU_EH_PDU_DWN),
    VIRTCHNL_PROTO_HDR_GTPU_UP_QFI =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_GTPU_EH_PDU_UP),
// L2TPv2
    VIRTCHNL_PROTO_HDR_L2TPV2_SESS_ID =
    PROTO_HDR_FIELD_START(VIRTCHNL_PROTO_HDR_L2TPV2),
    VIRTCHNL_PROTO_HDR_L2TPV2_LEN_SESS_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_proto_hdr {
// see enum virtchnl_proto_hdr_type
    pub type: i32,
    pub /: *mut *mut u32 field_selector; / a bit mask to select field for header type,
    pub buffer: [u8; 64],
//
// binary buffer in network order for specific header type.
// For example, if type = VIRTCHNL_PROTO_HDR_IPV4, a IPv4
// header is expected to be copied into the buffer.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_proto_hdrs {
    pub tunnel_level: u8,
    pub pad: [u8; 3],
//
// specify where protocol header start from.
// must be 0 when sending a raw packet request.
// 0 - from the outer layer
// 1 - from the first inner layer
// 2 - from the second inner layer
// ....
//
    pub /: *mut *mut u32 count; / the proto layers must < VIRTCHNL_MAX_NUM_PROTO_HDRS,
    pub pkt_len: u16,
    pub spec: [u8; VIRTCHNL_MAX_SIZE_RAW_PACKET],
    pub mask: [u8; VIRTCHNL_MAX_SIZE_RAW_PACKET],
    pub raw: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_rss_cfg {
    pub /: *mut *mut virtchnl_proto_hdrs proto_hdrs; / protocol headers,
// see enum virtchnl_rss_algorithm; rss algorithm type
    pub rss_algorithm: i32,
    pub /: *mut *mut u8 reserved[128]; / reserve for future,
}

// action configuration for FDIR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_filter_action {
// see enum virtchnl_action type
    pub type: i32,
// used for queue and qgroup action
    pub index: u16,
    pub region: u8,
    pub queue: },
// used for count action
// share counter ID with other flow rules
    pub shared: u8,
    pub /: *mut *mut u32 id; / counter ID,
    pub count: },
// used for mark action
    pub mark_id: u32,
    pub reserve: [u8; 32],
    pub act_conf: },
}

pub const VIRTCHNL_MAX_NUM_ACTIONS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_filter_action_set {
// action number must be less then VIRTCHNL_MAX_NUM_ACTIONS
    pub count: u32,
    pub actions: [virtchnl_filter_action; VIRTCHNL_MAX_NUM_ACTIONS],
}

// pattern and action for FDIR rule
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_fdir_rule {
    pub proto_hdrs: virtchnl_proto_hdrs,
    pub action_set: virtchnl_filter_action_set,
}

// Status returned to VF after VF requests FDIR commands
// VIRTCHNL_FDIR_SUCCESS
// VF FDIR related request is successfully done by PF
// The request can be OP_ADD/DEL/QUERY_FDIR_FILTER.
//
// VIRTCHNL_FDIR_FAILURE_RULE_NORESOURCE
// OP_ADD_FDIR_FILTER request is failed due to no Hardware resource.
//
// VIRTCHNL_FDIR_FAILURE_RULE_EXIST
// OP_ADD_FDIR_FILTER request is failed due to the rule is already existed.
//
// VIRTCHNL_FDIR_FAILURE_RULE_CONFLICT
// OP_ADD_FDIR_FILTER request is failed due to conflict with existing rule.
//
// VIRTCHNL_FDIR_FAILURE_RULE_NONEXIST
// OP_DEL_FDIR_FILTER request is failed due to this rule doesn't exist.
//
// VIRTCHNL_FDIR_FAILURE_RULE_INVALID
// OP_ADD_FDIR_FILTER request is failed due to parameters validation
// or HW doesn't support.
//
// VIRTCHNL_FDIR_FAILURE_RULE_TIMEOUT
// OP_ADD/DEL_FDIR_FILTER request is failed due to timing out
// for programming.
//
// VIRTCHNL_FDIR_FAILURE_QUERY_INVALID
// OP_QUERY_FDIR_FILTER request is failed due to parameters validation,
// for example, VF query counter of a rule who has no counter action.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_fdir_prgm_status {
    VIRTCHNL_FDIR_SUCCESS = 0,
    VIRTCHNL_FDIR_FAILURE_RULE_NORESOURCE,
    VIRTCHNL_FDIR_FAILURE_RULE_EXIST,
    VIRTCHNL_FDIR_FAILURE_RULE_CONFLICT,
    VIRTCHNL_FDIR_FAILURE_RULE_NONEXIST,
    VIRTCHNL_FDIR_FAILURE_RULE_INVALID,
    VIRTCHNL_FDIR_FAILURE_RULE_TIMEOUT,
    VIRTCHNL_FDIR_FAILURE_QUERY_INVALID,
}

// VIRTCHNL_OP_ADD_FDIR_FILTER
// VF sends this request to PF by filling out vsi_id,
// validate_only and rule_cfg. PF will return flow_id
// if the request is successfully done and return add_status to VF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_fdir_add {
    pub /: *mut *mut u16 vsi_id; / INPUT,
//
// 1 for validating a fdir rule, 0 for creating a fdir rule.
// Validate and create share one ops: VIRTCHNL_OP_ADD_FDIR_FILTER.
//
    pub /: *mut *mut u16 validate_only; / INPUT,
    pub /: *mut *mut u32 flow_id; / OUTPUT,
    pub /: *mut *mut virtchnl_fdir_rule rule_cfg; / INPUT,
// see enum virtchnl_fdir_prgm_status; OUTPUT
    pub status: i32,
}

// VIRTCHNL_OP_DEL_FDIR_FILTER
// VF sends this request to PF by filling out vsi_id
// and flow_id. PF will return del_status to VF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_fdir_del {
    pub /: *mut *mut u16 vsi_id; / INPUT,
    pub pad: u16,
    pub /: *mut *mut u32 flow_id; / INPUT,
// see enum virtchnl_fdir_prgm_status; OUTPUT
    pub status: i32,
}

//
// struct virtchnl_ptp_caps - Defines the PTP caps available to the VF.
// @caps: On send, VF sets what capabilities it requests. On reply, PF
// indicates what has been enabled for this VF. The PF shall not set
// bits which were not requested by the VF.
// @rsvd: Reserved bits for future extension.
//
// Structure that defines the PTP capabilities available to the VF. The VF
// sends VIRTCHNL_OP_1588_PTP_GET_CAPS, and must fill in the ptp_caps field
// indicating what capabilities it is requesting. The PF will respond with the
// same message with the virtchnl_ptp_caps structure indicating what is
// enabled for the VF.
//
// VIRTCHNL_1588_PTP_CAP_RX_TSTAMP indicates that the VF receive queues have
// receive timestamps enabled in the flexible descriptors. Note that this
// requires a VF to also negotiate to enable advanced flexible descriptors in
// the receive path instead of the default legacy descriptor format.
//
// VIRTCHNL_1588_PTP_CAP_READ_PHC indicates that the VF may read the PHC time
// via the VIRTCHNL_OP_1588_PTP_GET_TIME command.
//
// Note that in the future, additional capability flags may be added which
// indicate additional extended support. All fields marked as reserved by this
// header will be set to zero. VF implementations should verify this to ensure
// that future extensions do not break compatibility.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_ptp_caps {
    pub caps: u32,
    pub rsvd: [u8; 44],
}

//
// struct virtchnl_phc_time - Contains the 64bits of PHC clock time in ns.
// @time: PHC time in nanoseconds
// @rsvd: Reserved for future extension
//
// Structure received with VIRTCHNL_OP_1588_PTP_GET_TIME. Contains the 64bits
// of PHC clock time in nanoseconds.
//
// VIRTCHNL_OP_1588_PTP_GET_TIME may be sent to request the current time of
// the PHC. This op is available in case direct access via the PHC registers
// is not available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_phc_time {
    pub time: u64,
    pub rsvd: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_shaper_bw {
// Unit is Kbps
    pub committed: u32,
    pub peak: u32,
}

// VIRTCHNL_OP_GET_QOS_CAPS
// VF sends this message to get its QoS Caps, such as
// TC number, Arbiter and Bandwidth.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_qos_cap_elem {
    pub tc_num: u8,
    pub tc_prio: u8,
pub const VIRTCHNL_ABITER_STRICT: c_int = 0;
pub const VIRTCHNL_ABITER_ETS: c_int = 2;
    pub arbiter: u8,
pub const VIRTCHNL_STRICT_WEIGHT: c_int = 1;
    pub weight: u8,
    pub type: virtchnl_bw_limit_type,
    pub shaper: virtchnl_shaper_bw,
    pub pad2: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_qos_cap_list {
    pub vsi_id: u16,
    pub num_elem: u16,
    pub cap: [virtchnl_qos_cap_elem; ],
}

pub const virtchnl_qos_cap_list_LEGACY_SIZEOF: c_int = 44;
// VIRTCHNL_OP_CONFIG_QUEUE_BW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_queue_bw {
    pub queue_id: u16,
    pub tc: u8,
    pub pad: u8,
    pub shaper: virtchnl_shaper_bw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_queues_bw_cfg {
    pub vsi_id: u16,
    pub num_queues: u16,
    pub cfg: [virtchnl_queue_bw; ],
}

pub const virtchnl_queues_bw_cfg_LEGACY_SIZEOF: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtchnl_queue_type {
    VIRTCHNL_QUEUE_TYPE_TX			= 0,
    VIRTCHNL_QUEUE_TYPE_RX			= 1,
}

// structure to specify a chunk of contiguous queues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_queue_chunk {
// see enum virtchnl_queue_type
    pub type: i32,
    pub start_queue_id: u16,
    pub num_queues: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtchnl_quanta_cfg {
    pub quanta_size: u16,
    pub pad: u16,
    pub queue_select: virtchnl_queue_chunk,
}

//
// virtchnl_vc_validate_vf_msg
// @ver: Virtchnl version info
// @v_opcode: Opcode for the message
// @msg: pointer to the msg buffer
// @msglen: msg length
//
// validate msg format against struct for each opcode
//
// Validate message length.
// These messages are opaque to us and will be validated in
// the RDMA client code. We just need to check for nonzero
// length. The firmware will enforce max length restrictions.
//
// These are always errors coming from the VF.
// few more checks
