//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_admin_defs.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright 2015-2020 Amazon.com, Inc. or its affiliates. All rights reserved.
//
pub const ENA_ADMIN_RSS_KEY_PARTS: c_int = 10;
pub const ENA_ADMIN_CUSTOMER_METRICS_SUPPORT_MASK: c_uint = 0x3F;
pub const ENA_ADMIN_CUSTOMER_METRICS_MIN_SUPPORT_MASK: c_uint = 0x1F;
// customer metrics - in correlation with
// ENA_ADMIN_CUSTOMER_METRICS_SUPPORT_MASK
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_customer_metrics_id {
    ENA_ADMIN_BW_IN_ALLOWANCE_EXCEEDED         = 0,
    ENA_ADMIN_BW_OUT_ALLOWANCE_EXCEEDED        = 1,
    ENA_ADMIN_PPS_ALLOWANCE_EXCEEDED           = 2,
    ENA_ADMIN_CONNTRACK_ALLOWANCE_EXCEEDED     = 3,
    ENA_ADMIN_LINKLOCAL_ALLOWANCE_EXCEEDED     = 4,
    ENA_ADMIN_CONNTRACK_ALLOWANCE_AVAILABLE    = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_aq_opcode {
    ENA_ADMIN_CREATE_SQ                         = 1,
    ENA_ADMIN_DESTROY_SQ                        = 2,
    ENA_ADMIN_CREATE_CQ                         = 3,
    ENA_ADMIN_DESTROY_CQ                        = 4,
    ENA_ADMIN_GET_FEATURE                       = 8,
    ENA_ADMIN_SET_FEATURE                       = 9,
    ENA_ADMIN_GET_STATS                         = 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_aq_completion_status {
    ENA_ADMIN_SUCCESS                           = 0,
    ENA_ADMIN_RESOURCE_ALLOCATION_FAILURE       = 1,
    ENA_ADMIN_BAD_OPCODE                        = 2,
    ENA_ADMIN_UNSUPPORTED_OPCODE                = 3,
    ENA_ADMIN_MALFORMED_REQUEST                 = 4,
// Additional status is provided in ACQ entry extended_status
    ENA_ADMIN_ILLEGAL_PARAMETER                 = 5,
    ENA_ADMIN_UNKNOWN_ERROR                     = 6,
    ENA_ADMIN_RESOURCE_BUSY                     = 7,
}

// subcommands for the set/get feature admin commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_aq_feature_id {
    ENA_ADMIN_DEVICE_ATTRIBUTES                 = 1,
    ENA_ADMIN_MAX_QUEUES_NUM                    = 2,
    ENA_ADMIN_HW_HINTS                          = 3,
    ENA_ADMIN_LLQ                               = 4,
    ENA_ADMIN_MAX_QUEUES_EXT                    = 7,
    ENA_ADMIN_RSS_HASH_FUNCTION                 = 10,
    ENA_ADMIN_STATELESS_OFFLOAD_CONFIG          = 11,
    ENA_ADMIN_RSS_INDIRECTION_TABLE_CONFIG      = 12,
    ENA_ADMIN_MTU                               = 14,
    ENA_ADMIN_RSS_HASH_INPUT                    = 18,
    ENA_ADMIN_INTERRUPT_MODERATION              = 20,
    ENA_ADMIN_AENQ_CONFIG                       = 26,
    ENA_ADMIN_LINK_CONFIG                       = 27,
    ENA_ADMIN_HOST_ATTR_CONFIG                  = 28,
    ENA_ADMIN_PHC_CONFIG                        = 29,
    ENA_ADMIN_FEATURES_OPCODE_NUM               = 32,
}

// device capabilities
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_aq_caps_id {
    ENA_ADMIN_ENI_STATS                         = 0,
// ENA SRD customer metrics
    ENA_ADMIN_ENA_SRD_INFO                      = 1,
    ENA_ADMIN_CUSTOMER_METRICS                  = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_placement_policy_type {
// descriptors and headers are in host memory
    ENA_ADMIN_PLACEMENT_POLICY_HOST             = 1,
// descriptors and headers are in device memory (a.k.a Low Latency
// Queue)
//
    ENA_ADMIN_PLACEMENT_POLICY_DEV              = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_link_types {
    ENA_ADMIN_LINK_SPEED_1G                     = 0x1,
    ENA_ADMIN_LINK_SPEED_2_HALF_G               = 0x2,
    ENA_ADMIN_LINK_SPEED_5G                     = 0x4,
    ENA_ADMIN_LINK_SPEED_10G                    = 0x8,
    ENA_ADMIN_LINK_SPEED_25G                    = 0x10,
    ENA_ADMIN_LINK_SPEED_40G                    = 0x20,
    ENA_ADMIN_LINK_SPEED_50G                    = 0x40,
    ENA_ADMIN_LINK_SPEED_100G                   = 0x80,
    ENA_ADMIN_LINK_SPEED_200G                   = 0x100,
    ENA_ADMIN_LINK_SPEED_400G                   = 0x200,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_completion_policy_type {
// completion queue entry for each sq descriptor
    ENA_ADMIN_COMPLETION_POLICY_DESC            = 0,
// completion queue entry upon request in sq descriptor
    ENA_ADMIN_COMPLETION_POLICY_DESC_ON_DEMAND  = 1,
// current queue head pointer is updated in OS memory upon sq
// descriptor request
//
    ENA_ADMIN_COMPLETION_POLICY_HEAD_ON_DEMAND  = 2,
// current queue head pointer is updated in OS memory for each sq
// descriptor
//
    ENA_ADMIN_COMPLETION_POLICY_HEAD            = 3,
}

// basic stats return ena_admin_basic_stats while extanded stats return a
// buffer (string format) with additional statistics per queue and per
// device id
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_get_stats_type {
    ENA_ADMIN_GET_STATS_TYPE_BASIC              = 0,
    ENA_ADMIN_GET_STATS_TYPE_EXTENDED           = 1,
// extra HW stats for specific network interface
    ENA_ADMIN_GET_STATS_TYPE_ENI                = 2,
// extra HW stats for ENA SRD
    ENA_ADMIN_GET_STATS_TYPE_ENA_SRD            = 3,
    ENA_ADMIN_GET_STATS_TYPE_CUSTOMER_METRICS   = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_get_stats_scope {
    ENA_ADMIN_SPECIFIC_QUEUE                    = 0,
    ENA_ADMIN_ETH_TRAFFIC                       = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_phc_type {
    ENA_ADMIN_PHC_TYPE_READLESS                 = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_phc_error_flags {
    ENA_ADMIN_PHC_ERROR_FLAG_TIMESTAMP   = BIT(0),
}

// ENA SRD configuration for ENI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_ena_srd_flags {
// Feature enabled
    ENA_ADMIN_ENA_SRD_ENABLED                   = BIT(0),
// UDP support enabled
    ENA_ADMIN_ENA_SRD_UDP_ENABLED               = BIT(1),
// Bypass Rx UDP ordering
    ENA_ADMIN_ENA_SRD_UDP_ORDERING_BYPASS_ENABLED = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aq_common_desc {
// 11:0 : command_id
// 15:12 : reserved12
//
    pub command_id: u16,
// as appears in ena_admin_aq_opcode
    pub opcode: u8,
// 0 : phase
// 1 : ctrl_data - control buffer address valid
// 2 : ctrl_data_indirect - control buffer address
// points to list of pages with addresses of control
// buffers
// 7:3 : reserved3
//
    pub flags: u8,
}

// used in ena_admin_aq_entry. Can point directly to control data, or to a
// page list chunk. Used also at the end of indirect mode page list chunks,
// for chaining.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_ctrl_buff_info {
    pub length: u32,
    pub address: ena_common_mem_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_sq {
    pub sq_idx: u16,
// 4:0 : reserved
// 7:5 : sq_direction - 0x1 - Tx; 0x2 - Rx
//
    pub sq_identity: u8,
    pub reserved1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aq_entry {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
    pub inline_data_w1: [u32; 3],
    pub control_buffer: ena_admin_ctrl_buff_info,
    pub u: },
    pub inline_data_w4: [u32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_acq_common_desc {
// command identifier to associate it with the aq descriptor
// 11:0 : command_id
// 15:12 : reserved12
//
    pub command: u16,
    pub status: u8,
// 0 : phase
// 7:1 : reserved1
//
    pub flags: u8,
    pub extended_status: u16,
// indicates to the driver which AQ entry has been consumed by the
// device and could be reused
//
    pub sq_head_indx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_acq_entry {
    pub acq_common_descriptor: ena_admin_acq_common_desc,
    pub response_specific_data: [u32; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aq_create_sq_cmd {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
// 4:0 : reserved0_w1
// 7:5 : sq_direction - 0x1 - Tx, 0x2 - Rx
//
    pub sq_identity: u8,
    pub reserved8_w1: u8,
// 3:0 : placement_policy - Describing where the SQ
// descriptor ring and the SQ packet headers reside:
// 0x1 - descriptors and headers are in OS memory,
// 0x3 - descriptors and headers in device memory
// (a.k.a Low Latency Queue)
// 6:4 : completion_policy - Describing what policy
// to use for generation completion entry (cqe) in
// the CQ associated with this SQ: 0x0 - cqe for each
// sq descriptor, 0x1 - cqe upon request in sq
// descriptor, 0x2 - current queue head pointer is
// updated in OS memory upon sq descriptor request
// 0x3 - current queue head pointer is updated in OS
// memory for each sq descriptor
// 7 : reserved15_w1
//
    pub sq_caps_2: u8,
// 0 : is_physically_contiguous - Described if the
// queue ring memory is allocated in physical
// contiguous pages or split.
// 7:1 : reserved17_w1
//
    pub sq_caps_3: u8,
// associated completion queue id. This CQ must be created prior to SQ
// creation
//
    pub cq_idx: u16,
// submission queue depth in entries
    pub sq_depth: u16,
// SQ physical base address in OS memory. This field should not be
// used for Low Latency queues. Has to be page aligned.
//
    pub sq_ba: ena_common_mem_addr,
// specifies queue head writeback location in OS memory. Valid if
// completion_policy is set to completion_policy_head_on_demand or
// completion_policy_head. Has to be cache aligned
//
    pub sq_head_writeback: ena_common_mem_addr,
    pub reserved0_w7: u32,
    pub reserved0_w8: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_sq_direction {
    ENA_ADMIN_SQ_DIRECTION_TX                   = 1,
    ENA_ADMIN_SQ_DIRECTION_RX                   = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_acq_create_sq_resp_desc {
    pub acq_common_desc: ena_admin_acq_common_desc,
    pub sq_idx: u16,
    pub reserved: u16,
// queue doorbell address as an offset to PCIe MMIO REG BAR
    pub sq_doorbell_offset: u32,
// low latency queue ring base address as an offset to PCIe MMIO
// LLQ_MEM BAR
//
    pub llq_descriptors_offset: u32,
// low latency queue headers' memory as an offset to PCIe MMIO
// LLQ_MEM BAR
//
    pub llq_headers_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aq_destroy_sq_cmd {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
    pub sq: ena_admin_sq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_acq_destroy_sq_resp_desc {
    pub acq_common_desc: ena_admin_acq_common_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aq_create_cq_cmd {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
// 4:0 : reserved5
// 5 : interrupt_mode_enabled - if set, cq operates
// in interrupt mode, otherwise - polling
// 7:6 : reserved6
//
    pub cq_caps_1: u8,
// 4:0 : cq_entry_size_words - size of CQ entry in
// 32-bit words, valid values: 4, 8.
// 7:5 : reserved7
//
    pub cq_caps_2: u8,
// completion queue depth in # of entries. must be power of 2
    pub cq_depth: u16,
// msix vector assigned to this cq
    pub msix_vector: u32,
// cq physical base address in OS memory. CQ must be physically
// contiguous
//
    pub cq_ba: ena_common_mem_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_acq_create_cq_resp_desc {
    pub acq_common_desc: ena_admin_acq_common_desc,
    pub cq_idx: u16,
// actual cq depth in number of entries
    pub cq_actual_depth: u16,
    pub numa_node_register_offset: u32,
    pub cq_head_db_register_offset: u32,
    pub cq_interrupt_unmask_register_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aq_destroy_cq_cmd {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
    pub cq_idx: u16,
    pub reserved1: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_acq_destroy_cq_resp_desc {
    pub acq_common_desc: ena_admin_acq_common_desc,
}

// ENA AQ Get Statistics command. Extended statistics are placed in control
// buffer pointed by AQ entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aq_get_stats_cmd {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
// command specific inline data
    pub inline_data_w1: [u32; 3],
    pub control_buffer: ena_admin_ctrl_buff_info,
    pub u: },
// stats type as defined in enum ena_admin_get_stats_type
    pub type: u8,
// stats scope defined in enum ena_admin_get_stats_scope
    pub scope: u8,
    pub reserved3: u16,
// queue id. used when scope is specific_queue
    pub queue_idx: u16,
// device id, value 0xFFFF means mine. only privileged device can get
// stats of other device
//
    pub device_id: u16,
// a bitmap representing the requested metric values
    pub requested_metrics: u64,
}

// Basic Statistics Command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_basic_stats {
    pub tx_bytes_low: u32,
    pub tx_bytes_high: u32,
    pub tx_pkts_low: u32,
    pub tx_pkts_high: u32,
    pub rx_bytes_low: u32,
    pub rx_bytes_high: u32,
    pub rx_pkts_low: u32,
    pub rx_pkts_high: u32,
    pub rx_drops_low: u32,
    pub rx_drops_high: u32,
    pub tx_drops_low: u32,
    pub tx_drops_high: u32,
}

// ENI Statistics Command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_eni_stats {
// The number of packets shaped due to inbound aggregate BW
// allowance being exceeded
//
    pub bw_in_allowance_exceeded: u64,
// The number of packets shaped due to outbound aggregate BW
// allowance being exceeded
//
    pub bw_out_allowance_exceeded: u64,
// The number of packets shaped due to PPS allowance being exceeded
    pub pps_allowance_exceeded: u64,
// The number of packets shaped due to connection tracking
// allowance being exceeded and leading to failure in establishment
// of new connections
//
    pub conntrack_allowance_exceeded: u64,
// The number of packets shaped due to linklocal packet rate
// allowance being exceeded
//
    pub linklocal_allowance_exceeded: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_ena_srd_stats {
// Number of packets transmitted over ENA SRD
    pub ena_srd_tx_pkts: u64,
// Number of packets transmitted or could have been
// transmitted over ENA SRD
//
    pub ena_srd_eligible_tx_pkts: u64,
// Number of packets received over ENA SRD
    pub ena_srd_rx_pkts: u64,
// Percentage of the ENA SRD resources that is in use
    pub ena_srd_resource_utilization: u64,
}

// ENA SRD Statistics Command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_ena_srd_info {
// ENA SRD configuration bitmap. See ena_admin_ena_srd_flags for
// details
//
    pub flags: u64,
    pub ena_srd_stats: ena_admin_ena_srd_stats,
}

// Customer Metrics Command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_customer_metrics {
// A bitmap representing the reported customer metrics according to
// the order they are reported
//
    pub reported_metrics: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_acq_get_stats_resp {
    pub acq_common_desc: ena_admin_acq_common_desc,
    pub raw: [u64; 7],
    pub basic_stats: ena_admin_basic_stats,
    pub eni_stats: ena_admin_eni_stats,
    pub ena_srd_info: ena_admin_ena_srd_info,
    pub customer_metrics: ena_admin_customer_metrics,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_get_set_feature_common_desc {
// 1:0 : select - 0x1 - current value; 0x3 - default
// value
// 7:3 : reserved3
//
    pub flags: u8,
// as appears in ena_admin_aq_feature_id
    pub feature_id: u8,
// The driver specifies the max feature version it supports and the
// device responds with the currently supported feature version. The
// field is zero based
//
    pub feature_version: u8,
    pub reserved8: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_device_attr_feature_desc {
    pub impl_id: u32,
    pub device_version: u32,
// bitmap of ena_admin_aq_feature_id, which represents supported
// subcommands for the set/get feature admin commands.
//
    pub supported_features: u32,
// bitmap of ena_admin_aq_caps_id, which represents device
// capabilities.
//
    pub capabilities: u32,
// Indicates how many bits are used physical address access.
    pub phys_addr_width: u32,
// Indicates how many bits are used virtual address access.
    pub virt_addr_width: u32,
// unicast MAC address (in Network byte order)
    pub mac_addr: [u8; 6],
    pub reserved7: [u8; 2],
    pub max_mtu: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_llq_header_location {
// header is in descriptor list
    ENA_ADMIN_INLINE_HEADER                     = 1,
// header in a separate ring, implies 16B descriptor list entry
    ENA_ADMIN_HEADER_RING                       = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_llq_ring_entry_size {
    ENA_ADMIN_LIST_ENTRY_SIZE_128B              = 1,
    ENA_ADMIN_LIST_ENTRY_SIZE_192B              = 2,
    ENA_ADMIN_LIST_ENTRY_SIZE_256B              = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_llq_num_descs_before_header {
    ENA_ADMIN_LLQ_NUM_DESCS_BEFORE_HEADER_0     = 0,
    ENA_ADMIN_LLQ_NUM_DESCS_BEFORE_HEADER_1     = 1,
    ENA_ADMIN_LLQ_NUM_DESCS_BEFORE_HEADER_2     = 2,
    ENA_ADMIN_LLQ_NUM_DESCS_BEFORE_HEADER_4     = 4,
    ENA_ADMIN_LLQ_NUM_DESCS_BEFORE_HEADER_8     = 8,
}

// packet descriptor list entry always starts with one or more descriptors,
// followed by a header. The rest of the descriptors are located in the
// beginning of the subsequent entry. Stride refers to how the rest of the
// descriptors are placed. This field is relevant only for inline header
// mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_llq_stride_ctrl {
    ENA_ADMIN_SINGLE_DESC_PER_ENTRY             = 1,
    ENA_ADMIN_MULTIPLE_DESCS_PER_ENTRY          = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_accel_mode_feat {
    ENA_ADMIN_DISABLE_META_CACHING              = 0,
    ENA_ADMIN_LIMIT_TX_BURST                    = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_accel_mode_get {
// bit field of enum ena_admin_accel_mode_feat
    pub supported_flags: u16,
// maximum burst size between two doorbells. The size is in bytes
    pub max_tx_burst_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_accel_mode_set {
// bit field of enum ena_admin_accel_mode_feat
    pub enabled_flags: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_accel_mode_req {
    pub raw: [u32; 2],
    pub get: ena_admin_accel_mode_get,
    pub set: ena_admin_accel_mode_set,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_llq_desc {
    pub max_llq_num: u32,
    pub max_llq_depth: u32,
// specify the header locations the device supports. bitfield of enum
// ena_admin_llq_header_location.
//
    pub header_location_ctrl_supported: u16,
// the header location the driver selected to use.
    pub header_location_ctrl_enabled: u16,
// if inline header is specified - this is the size of descriptor list
// entry. If header in a separate ring is specified - this is the size
// of header ring entry. bitfield of enum ena_admin_llq_ring_entry_size.
// specify the entry sizes the device supports
//
    pub entry_size_ctrl_supported: u16,
// the entry size the driver selected to use.
    pub entry_size_ctrl_enabled: u16,
// valid only if inline header is specified. First entry associated with
// the packet includes descriptors and header. Rest of the entries
// occupied by descriptors. This parameter defines the max number of
// descriptors precedding the header in the first entry. The field is
// bitfield of enum ena_admin_llq_num_descs_before_header and specify
// the values the device supports
//
    pub desc_num_before_header_supported: u16,
// the desire field the driver selected to use
    pub desc_num_before_header_enabled: u16,
// valid only if inline was chosen. bitfield of enum
// ena_admin_llq_stride_ctrl
//
    pub descriptors_stride_ctrl_supported: u16,
// the stride control the driver selected to use
    pub descriptors_stride_ctrl_enabled: u16,
// reserved
    pub reserved1: u32,
// accelerated low latency queues requirement. driver needs to
// support those requirements in order to use accelerated llq
//
    pub accel_mode: ena_admin_accel_mode_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_queue_ext_feature_fields {
    pub max_tx_sq_num: u32,
    pub max_tx_cq_num: u32,
    pub max_rx_sq_num: u32,
    pub max_rx_cq_num: u32,
    pub max_tx_sq_depth: u32,
    pub max_tx_cq_depth: u32,
    pub max_rx_sq_depth: u32,
    pub max_rx_cq_depth: u32,
    pub max_tx_header_size: u32,
// Maximum Descriptors number, including meta descriptor, allowed for a
// single Tx packet
//
    pub max_per_packet_tx_descs: u16,
// Maximum Descriptors number allowed for a single Rx packet
    pub max_per_packet_rx_descs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_queue_feature_desc {
    pub max_sq_num: u32,
    pub max_sq_depth: u32,
    pub max_cq_num: u32,
    pub max_cq_depth: u32,
    pub max_legacy_llq_num: u32,
    pub max_legacy_llq_depth: u32,
    pub max_header_size: u32,
// Maximum Descriptors number, including meta descriptor, allowed for a
// single Tx packet
//
    pub max_packet_tx_descs: u16,
// Maximum Descriptors number allowed for a single Rx packet
    pub max_packet_rx_descs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_set_feature_mtu_desc {
// exclude L2
    pub mtu: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_set_feature_host_attr_desc {
// host OS info base address in OS memory. host info is 4KB of
// physically contiguous
//
    pub os_info_ba: ena_common_mem_addr,
// host debug area base address in OS memory. debug area must be
// physically contiguous
//
    pub debug_ba: ena_common_mem_addr,
// debug area size
    pub debug_area_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_intr_moder_desc {
// interrupt delay granularity in usec
    pub intr_delay_resolution: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_get_feature_link_desc {
// Link speed in Mb
    pub speed: u32,
// bit field of enum ena_admin_link types
    pub supported: u32,
// 0 : autoneg
// 1 : duplex - Full Duplex
// 31:2 : reserved2
//
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_aenq_desc {
// bitmask for AENQ groups the device can report
    pub supported_groups: u32,
// bitmask for AENQ groups to report
    pub enabled_groups: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_offload_desc {
// 0 : TX_L3_csum_ipv4
// 1 : TX_L4_ipv4_csum_part - The checksum field
// should be initialized with pseudo header checksum
// 2 : TX_L4_ipv4_csum_full
// 3 : TX_L4_ipv6_csum_part - The checksum field
// should be initialized with pseudo header checksum
// 4 : TX_L4_ipv6_csum_full
// 5 : tso_ipv4
// 6 : tso_ipv6
// 7 : tso_ecn
//
    pub tx: u32,
// Receive side supported stateless offload
// 0 : RX_L3_csum_ipv4 - IPv4 checksum
// 1 : RX_L4_ipv4_csum - TCP/UDP/IPv4 checksum
// 2 : RX_L4_ipv6_csum - TCP/UDP/IPv6 checksum
// 3 : RX_hash - Hash calculation
//
    pub rx_supported: u32,
    pub rx_enabled: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_hash_functions {
    ENA_ADMIN_TOEPLITZ                          = 1,
    ENA_ADMIN_CRC32                             = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_rss_flow_hash_control {
    pub key_parts: u32,
    pub reserved: u32,
    pub key: [u32; ENA_ADMIN_RSS_KEY_PARTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_rss_flow_hash_function {
// 7:0 : funcs - bitmask of ena_admin_hash_functions
    pub supported_func: u32,
// 7:0 : selected_func - bitmask of
// ena_admin_hash_functions
//
    pub selected_func: u32,
// initial value
    pub init_val: u32,
}

// RSS flow hash protocols
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_flow_hash_proto {
    ENA_ADMIN_RSS_TCP4                          = 0,
    ENA_ADMIN_RSS_UDP4                          = 1,
    ENA_ADMIN_RSS_TCP6                          = 2,
    ENA_ADMIN_RSS_UDP6                          = 3,
    ENA_ADMIN_RSS_IP4                           = 4,
    ENA_ADMIN_RSS_IP6                           = 5,
    ENA_ADMIN_RSS_IP4_FRAG                      = 6,
    ENA_ADMIN_RSS_NOT_IP                        = 7,
// TCPv6 with extension header
    ENA_ADMIN_RSS_TCP6_EX                       = 8,
// IPv6 with extension header
    ENA_ADMIN_RSS_IP6_EX                        = 9,
    ENA_ADMIN_RSS_PROTO_NUM                     = 16,
}

// RSS flow hash fields
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_flow_hash_fields {
// Ethernet Dest Addr
    ENA_ADMIN_RSS_L2_DA                         = BIT(0),
// Ethernet Src Addr
    ENA_ADMIN_RSS_L2_SA                         = BIT(1),
// ipv4/6 Dest Addr
    ENA_ADMIN_RSS_L3_DA                         = BIT(2),
// ipv4/6 Src Addr
    ENA_ADMIN_RSS_L3_SA                         = BIT(3),
// tcp/udp Dest Port
    ENA_ADMIN_RSS_L4_DP                         = BIT(4),
// tcp/udp Src Port
    ENA_ADMIN_RSS_L4_SP                         = BIT(5),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_proto_input {
// flow hash fields (bitwise according to ena_admin_flow_hash_fields)
    pub fields: u16,
    pub reserved2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_rss_hash_control {
    pub supported_fields: [ena_admin_proto_input; ENA_ADMIN_RSS_PROTO_NUM],
    pub selected_fields: [ena_admin_proto_input; ENA_ADMIN_RSS_PROTO_NUM],
    pub reserved2: [ena_admin_proto_input; ENA_ADMIN_RSS_PROTO_NUM],
    pub reserved3: [ena_admin_proto_input; ENA_ADMIN_RSS_PROTO_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_rss_flow_hash_input {
// supported hash input sorting
// 1 : L3_sort - support swap L3 addresses if DA is
// smaller than SA
// 2 : L4_sort - support swap L4 ports if DP smaller
// SP
//
    pub supported_input_sort: u16,
// enabled hash input sorting
// 1 : enable_L3_sort - enable swap L3 addresses if
// DA smaller than SA
// 2 : enable_L4_sort - enable swap L4 ports if DP
// smaller than SP
//
    pub enabled_input_sort: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_os_type {
    ENA_ADMIN_OS_LINUX                          = 1,
    ENA_ADMIN_OS_WIN                            = 2,
    ENA_ADMIN_OS_DPDK                           = 3,
    ENA_ADMIN_OS_FREEBSD                        = 4,
    ENA_ADMIN_OS_IPXE                           = 5,
    ENA_ADMIN_OS_ESXI                           = 6,
    ENA_ADMIN_OS_GROUPS_NUM                     = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_host_info {
// defined in enum ena_admin_os_type
    pub os_type: u32,
// os distribution string format
    pub os_dist_str: [u8; 128],
// OS distribution numeric format
    pub os_dist: u32,
// kernel version string format
    pub kernel_ver_str: [u8; 32],
// Kernel version numeric format
    pub kernel_ver: u32,
// 7:0 : major
// 15:8 : minor
// 23:16 : sub_minor
// 31:24 : module_type
//
    pub driver_version: u32,
// features bitmap
    pub supported_network_features: [u32; 2],
// ENA spec version of driver
    pub ena_spec_version: u16,
// ENA device's Bus, Device and Function
// 2:0 : function
// 7:3 : device
// 15:8 : bus
//
    pub bdf: u16,
// Number of CPUs
    pub num_cpus: u16,
    pub reserved: u16,
// 0 : reserved
// 1 : rx_offset
// 2 : interrupt_moderation
// 3 : rx_buf_mirroring
// 4 : rss_configurable_function_key
// 5 : reserved
// 6 : rx_page_reuse
// 7 : reserved
// 8 : phc
// 31:9 : reserved
//
    pub driver_supported_features: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_rss_ind_table_entry {
    pub cq_idx: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_rss_ind_table {
// min supported table size (2^min_size)
    pub min_size: u16,
// max supported table size (2^max_size)
    pub max_size: u16,
// table size (2^size)
    pub size: u16,
    pub reserved: u16,
// index of the inline entry. 0xFFFFFFFF means invalid
    pub inline_index: u32,
// used for updating single entry, ignored when setting the entire
// table through the control buffer.
//
    pub inline_entry: ena_admin_rss_ind_table_entry,
}

// When hint value is 0, driver should use its own predefined value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_ena_hw_hints {
// value in ms
    pub mmio_read_timeout: u16,
// value in ms
    pub driver_watchdog_timeout: u16,
// Per packet tx completion timeout. value in ms
    pub missing_tx_completion_timeout: u16,
    pub missed_tx_completion_count_threshold_to_reset: u16,
// value in ms
    pub admin_completion_tx_timeout: u16,
    pub netdev_wd_timeout: u16,
    pub max_tx_sgl_size: u16,
    pub max_rx_sgl_size: u16,
    pub reserved: [u16; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_get_feat_cmd {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
    pub control_buffer: ena_admin_ctrl_buff_info,
    pub feat_common: ena_admin_get_set_feature_common_desc,
    pub raw: [u32; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_queue_ext_feature_desc {
// version
    pub version: u8,
    pub reserved1: [u8; 3],
    pub max_queue_ext: ena_admin_queue_ext_feature_fields,
    pub raw: [u32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_feature_phc_desc {
// PHC type as defined in enum ena_admin_get_phc_type,
// used only for GET command.
//
    pub type: u8,
// Reserved - MBZ
    pub reserved1: [u8; 3],
// PHC doorbell address as an offset to PCIe MMIO REG BAR,
// used only for GET command.
//
    pub doorbell_offset: u32,
// Max time for valid PHC retrieval, passing this threshold will
// fail the get-time request and block PHC requests for
// block_timeout_usec, used only for GET command.
//
    pub expire_timeout_usec: u32,
// PHC requests block period, blocking starts if PHC request expired
// in order to prevent floods on busy device,
// used only for GET command.
//
    pub block_timeout_usec: u32,
// Shared PHC physical address (ena_admin_phc_resp),
// used only for SET command.
//
    pub output_address: ena_common_mem_addr,
// Shared PHC Size (ena_admin_phc_resp),
// used only for SET command.
//
    pub output_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_get_feat_resp {
    pub acq_common_desc: ena_admin_acq_common_desc,
    pub raw: [u32; 14],
    pub dev_attr: ena_admin_device_attr_feature_desc,
    pub llq: ena_admin_feature_llq_desc,
    pub max_queue: ena_admin_queue_feature_desc,
    pub max_queue_ext: ena_admin_queue_ext_feature_desc,
    pub aenq: ena_admin_feature_aenq_desc,
    pub link: ena_admin_get_feature_link_desc,
    pub offload: ena_admin_feature_offload_desc,
    pub flow_hash_func: ena_admin_feature_rss_flow_hash_function,
    pub flow_hash_input: ena_admin_feature_rss_flow_hash_input,
    pub ind_table: ena_admin_feature_rss_ind_table,
    pub intr_moderation: ena_admin_feature_intr_moder_desc,
    pub hw_hints: ena_admin_ena_hw_hints,
    pub phc: ena_admin_feature_phc_desc,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_set_feat_cmd {
    pub aq_common_descriptor: ena_admin_aq_common_desc,
    pub control_buffer: ena_admin_ctrl_buff_info,
    pub feat_common: ena_admin_get_set_feature_common_desc,
    pub raw: [u32; 11],
// mtu size
    pub mtu: ena_admin_set_feature_mtu_desc,
// host attributes
    pub host_attr: ena_admin_set_feature_host_attr_desc,
// AENQ configuration
    pub aenq: ena_admin_feature_aenq_desc,
// rss flow hash function
    pub flow_hash_func: ena_admin_feature_rss_flow_hash_function,
// rss flow hash input
    pub flow_hash_input: ena_admin_feature_rss_flow_hash_input,
// rss indirection table
    pub ind_table: ena_admin_feature_rss_ind_table,
// LLQ configuration
    pub llq: ena_admin_feature_llq_desc,
// PHC configuration
    pub phc: ena_admin_feature_phc_desc,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_set_feat_resp {
    pub acq_common_desc: ena_admin_acq_common_desc,
    pub raw: [u32; 14],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aenq_common_desc {
    pub group: u16,
    pub syndrome: u16,
// 0 : phase
// 7:1 : reserved - MBZ
//
    pub flags: u8,
    pub reserved1: [u8; 3],
    pub timestamp_low: u32,
    pub timestamp_high: u32,
}

// asynchronous event notification groups
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_aenq_group {
    ENA_ADMIN_LINK_CHANGE                       = 0,
    ENA_ADMIN_FATAL_ERROR                       = 1,
    ENA_ADMIN_WARNING                           = 2,
    ENA_ADMIN_NOTIFICATION                      = 3,
    ENA_ADMIN_KEEP_ALIVE                        = 4,
    ENA_ADMIN_AENQ_GROUPS_NUM                   = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_admin_aenq_notification_syndrome {
    ENA_ADMIN_UPDATE_HINTS                      = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aenq_entry {
    pub aenq_common_desc: ena_admin_aenq_common_desc,
// command specific inline data
    pub inline_data_w4: [u32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aenq_link_change_desc {
    pub aenq_common_desc: ena_admin_aenq_common_desc,
// 0 : link_status
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_aenq_keep_alive_desc {
    pub aenq_common_desc: ena_admin_aenq_common_desc,
    pub rx_drops_low: u32,
    pub rx_drops_high: u32,
    pub tx_drops_low: u32,
    pub tx_drops_high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_ena_mmio_req_read_less_resp {
    pub req_id: u16,
    pub reg_off: u16,
// value is valid when poll is cleared
    pub reg_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_admin_phc_resp {
// Request Id, received from DB register
    pub req_id: u16,
    pub reserved1: [u8; 6],
// PHC timestamp (nsec)
    pub timestamp: u64,
    pub reserved2: [u8; 12],
// Bit field of enum ena_admin_phc_error_flags
    pub error_flags: u32,
    pub reserved3: [u8; 32],
}

// aq_common_desc

pub const ENA_ADMIN_AQ_COMMON_DESC_CTRL_DATA_SHIFT: c_int = 1;

pub const ENA_ADMIN_AQ_COMMON_DESC_CTRL_DATA_INDIRECT_SHIFT: c_int = 2;

// sq
pub const ENA_ADMIN_SQ_SQ_DIRECTION_SHIFT: c_int = 5;

// acq_common_desc

// aq_create_sq_cmd
pub const ENA_ADMIN_AQ_CREATE_SQ_CMD_SQ_DIRECTION_SHIFT: c_int = 5;

pub const ENA_ADMIN_AQ_CREATE_SQ_CMD_COMPLETION_POLICY_SHIFT: c_int = 4;

// aq_create_cq_cmd
pub const ENA_ADMIN_AQ_CREATE_CQ_CMD_INTERRUPT_MODE_ENABLED_SHIFT: c_int = 5;

// get_set_feature_common_desc

// get_feature_link_desc

pub const ENA_ADMIN_GET_FEATURE_LINK_DESC_DUPLEX_SHIFT: c_int = 1;

// feature_offload_desc

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_TX_L4_IPV4_CSUM_PART_SHIFT: c_int = 1;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_TX_L4_IPV4_CSUM_FULL_SHIFT: c_int = 2;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_TX_L4_IPV6_CSUM_PART_SHIFT: c_int = 3;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_TX_L4_IPV6_CSUM_FULL_SHIFT: c_int = 4;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_TSO_IPV4_SHIFT: c_int = 5;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_TSO_IPV6_SHIFT: c_int = 6;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_TSO_ECN_SHIFT: c_int = 7;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_RX_L4_IPV4_CSUM_SHIFT: c_int = 1;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_RX_L4_IPV6_CSUM_SHIFT: c_int = 2;

pub const ENA_ADMIN_FEATURE_OFFLOAD_DESC_RX_HASH_SHIFT: c_int = 3;

// feature_rss_flow_hash_function

// feature_rss_flow_hash_input
pub const ENA_ADMIN_FEATURE_RSS_FLOW_HASH_INPUT_L3_SORT_SHIFT: c_int = 1;

pub const ENA_ADMIN_FEATURE_RSS_FLOW_HASH_INPUT_L4_SORT_SHIFT: c_int = 2;

pub const ENA_ADMIN_FEATURE_RSS_FLOW_HASH_INPUT_ENABLE_L3_SORT_SHIFT: c_int = 1;

pub const ENA_ADMIN_FEATURE_RSS_FLOW_HASH_INPUT_ENABLE_L4_SORT_SHIFT: c_int = 2;

// host_info

pub const ENA_ADMIN_HOST_INFO_MINOR_SHIFT: c_int = 8;

pub const ENA_ADMIN_HOST_INFO_SUB_MINOR_SHIFT: c_int = 16;

pub const ENA_ADMIN_HOST_INFO_MODULE_TYPE_SHIFT: c_int = 24;

pub const ENA_ADMIN_HOST_INFO_DEVICE_SHIFT: c_int = 3;

pub const ENA_ADMIN_HOST_INFO_BUS_SHIFT: c_int = 8;

pub const ENA_ADMIN_HOST_INFO_RX_OFFSET_SHIFT: c_int = 1;

pub const ENA_ADMIN_HOST_INFO_INTERRUPT_MODERATION_SHIFT: c_int = 2;

pub const ENA_ADMIN_HOST_INFO_RX_BUF_MIRRORING_SHIFT: c_int = 3;

pub const ENA_ADMIN_HOST_INFO_RSS_CONFIGURABLE_FUNCTION_KEY_SHIFT: c_int = 4;

pub const ENA_ADMIN_HOST_INFO_RX_PAGE_REUSE_SHIFT: c_int = 6;

pub const ENA_ADMIN_HOST_INFO_PHC_SHIFT: c_int = 8;

// aenq_common_desc

// aenq_link_change_desc

