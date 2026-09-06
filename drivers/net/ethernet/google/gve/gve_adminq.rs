//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/google/gve/gve_adminq.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2015-2021 Google, Inc.
//

// Admin queue opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_adminq_opcodes {
    GVE_ADMINQ_DESCRIBE_DEVICE		= 0x1,
    GVE_ADMINQ_CONFIGURE_DEVICE_RESOURCES	= 0x2,
    GVE_ADMINQ_REGISTER_PAGE_LIST		= 0x3,
    GVE_ADMINQ_UNREGISTER_PAGE_LIST		= 0x4,
    GVE_ADMINQ_CREATE_TX_QUEUE		= 0x5,
    GVE_ADMINQ_CREATE_RX_QUEUE		= 0x6,
    GVE_ADMINQ_DESTROY_TX_QUEUE		= 0x7,
    GVE_ADMINQ_DESTROY_RX_QUEUE		= 0x8,
    GVE_ADMINQ_DECONFIGURE_DEVICE_RESOURCES	= 0x9,
    GVE_ADMINQ_CONFIGURE_RSS		= 0xA,
    GVE_ADMINQ_SET_DRIVER_PARAMETER		= 0xB,
    GVE_ADMINQ_REPORT_STATS			= 0xC,
    GVE_ADMINQ_REPORT_LINK_SPEED		= 0xD,
    GVE_ADMINQ_GET_PTYPE_MAP		= 0xE,
    GVE_ADMINQ_VERIFY_DRIVER_COMPATIBILITY	= 0xF,
    GVE_ADMINQ_QUERY_FLOW_RULES		= 0x10,
    GVE_ADMINQ_REPORT_NIC_TIMESTAMP		= 0x11,
    GVE_ADMINQ_QUERY_RSS			= 0x12,

// For commands that are larger than 56 bytes
    GVE_ADMINQ_EXTENDED_COMMAND		= 0xFF,
}

// The normal adminq command is restricted to be 56 bytes at maximum. For the
// longer adminq command, it is wrapped by GVE_ADMINQ_EXTENDED_COMMAND with
// inner opcode of gve_adminq_extended_cmd_opcodes specified. The inner command
// is written in the dma memory allocated by GVE_ADMINQ_EXTENDED_COMMAND.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_adminq_extended_cmd_opcodes {
    GVE_ADMINQ_CONFIGURE_FLOW_RULE	= 0x101,
}

// Admin queue status codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_adminq_statuses {
    GVE_ADMINQ_COMMAND_UNSET			= 0x0,
    GVE_ADMINQ_COMMAND_PASSED			= 0x1,
    GVE_ADMINQ_COMMAND_ERROR_ABORTED		= 0xFFFFFFF0,
    GVE_ADMINQ_COMMAND_ERROR_ALREADY_EXISTS		= 0xFFFFFFF1,
    GVE_ADMINQ_COMMAND_ERROR_CANCELLED		= 0xFFFFFFF2,
    GVE_ADMINQ_COMMAND_ERROR_DATALOSS		= 0xFFFFFFF3,
    GVE_ADMINQ_COMMAND_ERROR_DEADLINE_EXCEEDED	= 0xFFFFFFF4,
    GVE_ADMINQ_COMMAND_ERROR_FAILED_PRECONDITION	= 0xFFFFFFF5,
    GVE_ADMINQ_COMMAND_ERROR_INTERNAL_ERROR		= 0xFFFFFFF6,
    GVE_ADMINQ_COMMAND_ERROR_INVALID_ARGUMENT	= 0xFFFFFFF7,
    GVE_ADMINQ_COMMAND_ERROR_NOT_FOUND		= 0xFFFFFFF8,
    GVE_ADMINQ_COMMAND_ERROR_OUT_OF_RANGE		= 0xFFFFFFF9,
    GVE_ADMINQ_COMMAND_ERROR_PERMISSION_DENIED	= 0xFFFFFFFA,
    GVE_ADMINQ_COMMAND_ERROR_UNAUTHENTICATED	= 0xFFFFFFFB,
    GVE_ADMINQ_COMMAND_ERROR_RESOURCE_EXHAUSTED	= 0xFFFFFFFC,
    GVE_ADMINQ_COMMAND_ERROR_UNAVAILABLE		= 0xFFFFFFFD,
    GVE_ADMINQ_COMMAND_ERROR_UNIMPLEMENTED		= 0xFFFFFFFE,
    GVE_ADMINQ_COMMAND_ERROR_UNKNOWN_ERROR		= 0xFFFFFFFF,
}

pub const GVE_ADMINQ_DEVICE_DESCRIPTOR_VERSION: c_int = 1;
// All AdminQ command structs should be naturally packed. The static_assert
// calls make sure this is the case at compile time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_describe_device {
    pub device_descriptor_addr: __be64,
    pub device_descriptor_version: __be32,
    pub available_length: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_descriptor {
    pub max_registered_pages: __be64,
    pub reserved1: __be16,
    pub tx_queue_entries: __be16,
    pub rx_queue_entries: __be16,
    pub default_num_queues: __be16,
    pub mtu: __be16,
    pub counters: __be16,
    pub tx_pages_per_qpl: __be16,
    pub rx_pages_per_qpl: __be16,
    pub mac: [u8; ETH_ALEN],
    pub num_device_options: __be16,
    pub total_length: __be16,
    pub reserved2: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option {
    pub option_id: __be16,
    pub option_length: __be16,
    pub required_features_mask: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_gqi_rda {
    pub supported_features_mask: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_gqi_qpl {
    pub supported_features_mask: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_dqo_rda {
    pub supported_features_mask: __be32,
    pub reserved: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_dqo_qpl {
    pub supported_features_mask: __be32,
    pub tx_pages_per_qpl: __be16,
    pub rx_pages_per_qpl: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_jumbo_frames {
    pub supported_features_mask: __be32,
    pub max_mtu: __be16,
    pub padding: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_buffer_sizes {
// GVE_SUP_BUFFER_SIZES_MASK bit should be set
    pub supported_features_mask: __be32,
    pub packet_buffer_size: __be16,
    pub header_buffer_size: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_modify_ring {
    pub supported_featured_mask: __be32,
    pub max_rx_ring_size: __be16,
    pub max_tx_ring_size: __be16,
    pub min_rx_ring_size: __be16,
    pub min_tx_ring_size: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_flow_steering {
    pub supported_features_mask: __be32,
    pub reserved: __be32,
    pub max_flow_rules: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_rss_config {
    pub supported_features_mask: __be32,
    pub hash_key_size: __be16,
    pub hash_lut_size: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_device_option_nic_timestamp {
    pub supported_features_mask: __be32,
}

// Terminology:
//
// RDA - Raw DMA Addressing - Buffers associated with SKBs are directly DMA
// mapped and read/updated by the device.
//
// QPL - Queue Page Lists - Driver uses bounce buffers which are DMA mapped with
// the device for read/write and data is copied from/to SKBs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_dev_opt_id {
    GVE_DEV_OPT_ID_GQI_RAW_ADDRESSING	= 0x1,
    GVE_DEV_OPT_ID_GQI_RDA			= 0x2,
    GVE_DEV_OPT_ID_GQI_QPL			= 0x3,
    GVE_DEV_OPT_ID_DQO_RDA			= 0x4,
    GVE_DEV_OPT_ID_MODIFY_RING		= 0x6,
    GVE_DEV_OPT_ID_DQO_QPL			= 0x7,
    GVE_DEV_OPT_ID_JUMBO_FRAMES		= 0x8,
    GVE_DEV_OPT_ID_BUFFER_SIZES		= 0xa,
    GVE_DEV_OPT_ID_FLOW_STEERING		= 0xb,
    GVE_DEV_OPT_ID_NIC_TIMESTAMP		= 0xd,
    GVE_DEV_OPT_ID_RSS_CONFIG		= 0xe,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_dev_opt_req_feat_mask {
    GVE_DEV_OPT_REQ_FEAT_MASK_GQI_RAW_ADDRESSING	= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_GQI_RDA		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_GQI_QPL		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_DQO_RDA		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_JUMBO_FRAMES		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_DQO_QPL		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_BUFFER_SIZES		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_MODIFY_RING		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_FLOW_STEERING		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_RSS_CONFIG		= 0x0,
    GVE_DEV_OPT_REQ_FEAT_MASK_NIC_TIMESTAMP		= 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_sup_feature_mask {
    GVE_SUP_MODIFY_RING_MASK	= 1 << 0,
    GVE_SUP_JUMBO_FRAMES_MASK	= 1 << 2,
    GVE_SUP_BUFFER_SIZES_MASK	= 1 << 4,
    GVE_SUP_FLOW_STEERING_MASK	= 1 << 5,
    GVE_SUP_RSS_CONFIG_MASK		= 1 << 7,
    GVE_SUP_NIC_TIMESTAMP_MASK	= 1 << 8,
}

pub const GVE_DEV_OPT_LEN_GQI_RAW_ADDRESSING: c_uint = 0x0;
pub const GVE_VERSION_STR_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_driver_capbility {
    gve_driver_capability_gqi_qpl = 0,
    gve_driver_capability_gqi_rda = 1,
    gve_driver_capability_dqo_qpl = 2, /* reserved for future use */
    gve_driver_capability_dqo_rda = 3,
    gve_driver_capability_alt_miss_compl = 4,
    gve_driver_capability_flexible_buffer_size = 5,
    gve_driver_capability_flexible_rss_size = 6,
}

pub const GVE_DRIVER_CAPABILITY_FLAGS2: c_uint = 0x0;
pub const GVE_DRIVER_CAPABILITY_FLAGS3: c_uint = 0x0;
pub const GVE_DRIVER_CAPABILITY_FLAGS4: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_extended_command {
    pub inner_opcode: __be32,
    pub inner_length: __be32,
    pub inner_command_addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_driver_info {
    pub /: *mut *mut u8 os_type; / 0x01 = Linux,
    pub driver_major: u8,
    pub driver_minor: u8,
    pub driver_sub: u8,
    pub os_version_major: __be32,
    pub os_version_minor: __be32,
    pub os_version_sub: __be32,
    pub driver_capability_flags: [__be64; 4],
    pub os_version_str1: [u8; GVE_VERSION_STR_LEN],
    pub os_version_str2: [u8; GVE_VERSION_STR_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_verify_driver_compatibility {
    pub driver_info_len: __be64,
    pub driver_info_addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_configure_device_resources {
    pub counter_array: __be64,
    pub irq_db_addr: __be64,
    pub num_counters: __be32,
    pub num_irq_dbs: __be32,
    pub irq_db_stride: __be32,
    pub ntfy_blk_msix_base_idx: __be32,
    pub queue_format: u8,
    pub padding: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_register_page_list {
    pub page_list_id: __be32,
    pub num_pages: __be32,
    pub page_address_list_addr: __be64,
    pub page_size: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_unregister_page_list {
    pub page_list_id: __be32,
}

pub const GVE_RAW_ADDRESSING_QPL_ID: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_create_tx_queue {
    pub queue_id: __be32,
    pub reserved: __be32,
    pub queue_resources_addr: __be64,
    pub tx_ring_addr: __be64,
    pub queue_page_list_id: __be32,
    pub ntfy_id: __be32,
    pub tx_comp_ring_addr: __be64,
    pub tx_ring_size: __be16,
    pub tx_comp_ring_size: __be16,
    pub padding: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_create_rx_queue {
    pub queue_id: __be32,
    pub index: __be32,
    pub reserved: __be32,
    pub ntfy_id: __be32,
    pub queue_resources_addr: __be64,
    pub rx_desc_ring_addr: __be64,
    pub rx_data_ring_addr: __be64,
    pub queue_page_list_id: __be32,
    pub rx_ring_size: __be16,
    pub packet_buffer_size: __be16,
    pub rx_buff_ring_size: __be16,
    pub enable_rsc: u8,
    pub padding1: u8,
    pub header_buffer_size: __be16,
    pub padding2: [u8; 2],
}

// Queue resources that are shared with the device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_queue_resources {
    pub /: *mut *mut __be32 db_index; / Device -> Guest,
    pub /: *mut *mut __be32 counter_index; / Device -> Guest,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_destroy_tx_queue {
    pub queue_id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_destroy_rx_queue {
    pub queue_id: __be32,
}

// GVE Set Driver Parameter Types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_set_driver_param_types {
    GVE_SET_PARAM_MTU	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_set_driver_parameter {
    pub parameter_type: __be32,
    pub reserved: [u8; 4],
    pub parameter_value: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_report_stats {
    pub stats_report_len: __be64,
    pub stats_report_addr: __be64,
    pub interval: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_report_link_speed {
    pub link_speed_address: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_report_nic_ts {
    pub nic_ts_report_len: __be64,
    pub nic_ts_report_addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_nic_ts_report {
    pub /: *mut *mut __be64 nic_timestamp; / NIC clock in nanoseconds,
    pub reserved1: __be64,
    pub reserved2: __be64,
    pub reserved3: __be64,
    pub reserved4: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats {
    pub stat_name: __be32,
    pub queue_id: __be32,
    pub value: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_stats_report {
    pub written_count: __be64,
    pub stats: [stats; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_stat_names {
// stats from gve
    TX_WAKE_CNT			= 1,
    TX_STOP_CNT			= 2,
    TX_FRAMES_SENT			= 3,
    TX_BYTES_SENT			= 4,
    TX_LAST_COMPLETION_PROCESSED	= 5,
    RX_NEXT_EXPECTED_SEQUENCE	= 6,
    RX_BUFFERS_POSTED		= 7,
    TX_TIMEOUT_CNT			= 8,
// stats from NIC
    RX_QUEUE_DROP_CNT		= 65,
    RX_NO_BUFFERS_POSTED		= 66,
    RX_DROPS_PACKET_OVER_MRU	= 67,
    RX_DROPS_INVALID_CHECKSUM	= 68,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_l3_type {
// Must be zero so zero initialized LUT is unknown.
    GVE_L3_TYPE_UNKNOWN = 0,
    GVE_L3_TYPE_OTHER,
    GVE_L3_TYPE_IPV4,
    GVE_L3_TYPE_IPV6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_l4_type {
// Must be zero so zero initialized LUT is unknown.
    GVE_L4_TYPE_UNKNOWN = 0,
    GVE_L4_TYPE_OTHER,
    GVE_L4_TYPE_TCP,
    GVE_L4_TYPE_UDP,
    GVE_L4_TYPE_ICMP,
    GVE_L4_TYPE_SCTP,
}

// These are control path types for PTYPE which are the same as the data path
// types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_ptype_entry {
    pub l3_type: u8,
    pub l4_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_ptype_map {
    pub /: *mut *mut gve_ptype_entry ptypes[GVE_NUM_PTYPES]; / PTYPES are always 10 bits.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_get_ptype_map {
    pub ptype_map_len: __be64,
    pub ptype_map_addr: __be64,
}

// Flow-steering related definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_adminq_flow_rule_cfg_opcode {
    GVE_FLOW_RULE_CFG_ADD	= 0,
    GVE_FLOW_RULE_CFG_DEL	= 1,
    GVE_FLOW_RULE_CFG_RESET	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_adminq_flow_rule_query_opcode {
    GVE_FLOW_RULE_QUERY_RULES	= 0,
    GVE_FLOW_RULE_QUERY_IDS		= 1,
    GVE_FLOW_RULE_QUERY_STATS	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_adminq_flow_type {
    GVE_FLOW_TYPE_TCPV4,
    GVE_FLOW_TYPE_UDPV4,
    GVE_FLOW_TYPE_SCTPV4,
    GVE_FLOW_TYPE_AHV4,
    GVE_FLOW_TYPE_ESPV4,
    GVE_FLOW_TYPE_TCPV6,
    GVE_FLOW_TYPE_UDPV6,
    GVE_FLOW_TYPE_SCTPV6,
    GVE_FLOW_TYPE_AHV6,
    GVE_FLOW_TYPE_ESPV6,
}

// Flow-steering command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_flow_rule {
    pub flow_type: __be16,
    pub /: *mut *mut __be16 action; / RX queue id,
    pub key: gve_flow_spec,
    pub mask: gve_flow_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_configure_flow_rule {
    pub opcode: __be16,
    pub padding: [u8; 2],
    pub rule: gve_adminq_flow_rule,
    pub location: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_query_flow_rules_descriptor {
    pub num_flow_rules: __be32,
    pub max_flow_rules: __be32,
    pub num_queried_rules: __be32,
    pub total_length: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_queried_flow_rule {
    pub location: __be32,
    pub flow_rule: gve_adminq_flow_rule,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_query_flow_rules {
    pub opcode: __be16,
    pub padding: [u8; 2],
    pub starting_rule_id: __be32,
    pub /: *mut *mut __be64 available_length; / The dma memory length that the driver allocated,
    pub /: *mut *mut __be64 rule_descriptor_addr; / The dma memory address,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gve_rss_hash_type {
    GVE_RSS_HASH_IPV4,
    GVE_RSS_HASH_TCPV4,
    GVE_RSS_HASH_IPV6,
    GVE_RSS_HASH_IPV6_EX,
    GVE_RSS_HASH_TCPV6,
    GVE_RSS_HASH_TCPV6_EX,
    GVE_RSS_HASH_UDPV4,
    GVE_RSS_HASH_UDPV6,
    GVE_RSS_HASH_UDPV6_EX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_configure_rss {
    pub hash_types: __be16,
    pub hash_alg: u8,
    pub reserved: u8,
    pub hash_key_size: __be16,
    pub hash_lut_size: __be16,
    pub hash_key_addr: __be64,
    pub hash_lut_addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_query_rss_descriptor {
    pub total_length: __be32,
    pub hash_types: __be16,
    pub hash_alg: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_adminq_query_rss {
    pub available_length: __be64,
    pub rss_descriptor_addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gve_adminq_command {
    pub opcode: __be32,
    pub status: __be32,
    pub create_tx_queue: gve_adminq_create_tx_queue,
    pub create_rx_queue: gve_adminq_create_rx_queue,
    pub destroy_tx_queue: gve_adminq_destroy_tx_queue,
    pub destroy_rx_queue: gve_adminq_destroy_rx_queue,
    pub describe_device: gve_adminq_describe_device,
    pub reg_page_list: gve_adminq_register_page_list,
    pub unreg_page_list: gve_adminq_unregister_page_list,
    pub set_driver_param: gve_adminq_set_driver_parameter,
    pub report_stats: gve_adminq_report_stats,
    pub report_link_speed: gve_adminq_report_link_speed,
    pub get_ptype_map: gve_adminq_get_ptype_map,
    pub query_flow_rules: gve_adminq_query_flow_rules,
    pub configure_rss: gve_adminq_configure_rss,
    pub query_rss: gve_adminq_query_rss,
    pub report_nic_ts: gve_adminq_report_nic_ts,
    pub extended_command: gve_adminq_extended_command,
}

extern "C" {
    pub fn gve_adminq_alloc(dev: *mut device, priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_adminq_free(priv: *mut gve_priv);
}
extern "C" {
    pub fn gve_adminq_release(priv: *mut gve_priv);
}
extern "C" {
    pub fn gve_adminq_describe_device(priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_adminq_deconfigure_device_resources(priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_adminq_create_tx_queues(priv: *mut gve_priv, start_id: u32, num_queues: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_destroy_tx_queues(priv: *mut gve_priv, start_id: u32, num_queues: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_create_single_rx_queue(priv: *mut gve_priv, queue_index: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_create_rx_queues(priv: *mut gve_priv, num_queues: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_destroy_single_rx_queue(priv: *mut gve_priv, queue_index: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_destroy_rx_queues(priv: *mut gve_priv, queue_id: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_unregister_page_list(priv: *mut gve_priv, page_list_id: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_report_link_speed(priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_adminq_add_flow_rule(priv: *mut gve_priv, rule: *mut gve_adminq_flow_rule, loc: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_del_flow_rule(priv: *mut gve_priv, loc: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_reset_flow_rules(priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_adminq_query_flow_rules(priv: *mut gve_priv, query_opcode: u16, starting_loc: u32) -> c_int;
}
extern "C" {
    pub fn gve_adminq_configure_rss(priv: *mut gve_priv, rxfh: *mut ethtool_rxfh_param) -> c_int;
}
extern "C" {
    pub fn gve_adminq_query_rss_config(priv: *mut gve_priv, rxfh: *mut ethtool_rxfh_param) -> c_int;
}
extern "C" {
    pub fn gve_set_num_ntfy_blks(priv: *mut gve_priv) -> c_int;
}
extern "C" {
    pub fn gve_set_num_queues(priv: *mut gve_priv);
}
