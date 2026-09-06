//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi/mpi30_ioc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016-2023 Broadcom Inc. All rights reserved.
//
pub const MPI30_IOC_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ioc_init_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub mpi_version: mpi3_version_union,
    pub time_stamp: __le64,
    pub reserved18: u8,
    pub who_init: u8,
    pub reserved1a: __le16,
    pub reply_free_queue_depth: __le16,
    pub reserved1e: __le16,
    pub reply_free_queue_address: __le64,
    pub reserved28: __le32,
    pub sense_buffer_free_queue_depth: __le16,
    pub sense_buffer_length: __le16,
    pub sense_buffer_free_queue_address: __le64,
    pub driver_information_address: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ioc_facts_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub reserved0c: __le32,
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ioc_facts_data {
    pub ioc_facts_data_length: __le16,
    pub reserved02: __le16,
    pub mpi_version: mpi3_version_union,
    pub fw_version: mpi3_comp_image_version,
    pub ioc_capabilities: __le32,
    pub ioc_number: u8,
    pub who_init: u8,
    pub max_msix_vectors: __le16,
    pub max_outstanding_requests: __le16,
    pub product_id: __le16,
    pub ioc_request_frame_size: __le16,
    pub reply_frame_size: __le16,
    pub ioc_exceptions: __le16,
    pub max_persistent_id: __le16,
    pub sge_modifier_mask: u8,
    pub sge_modifier_value: u8,
    pub sge_modifier_shift: u8,
    pub protocol_flags: u8,
    pub max_sas_initiators: __le16,
    pub max_data_length: __le16,
    pub max_sas_expanders: __le16,
    pub max_enclosures: __le16,
    pub min_dev_handle: __le16,
    pub max_dev_handle: __le16,
    pub max_pcie_switches: __le16,
    pub max_nvme: __le16,
    pub reserved38: __le16,
    pub max_vds: __le16,
    pub max_host_pds: __le16,
    pub max_adv_host_pds: __le16,
    pub max_raid_pds: __le16,
    pub max_posted_cmd_buffers: __le16,
    pub flags: __le32,
    pub max_operational_request_queues: __le16,
    pub max_operational_reply_queues: __le16,
    pub shutdown_timeout: __le16,
    pub reserved4e: __le16,
    pub diag_trace_size: __le32,
    pub diag_fw_size: __le32,
    pub diag_driver_size: __le32,
    pub max_host_pd_ns_count: u8,
    pub max_adv_host_pd_ns_count: u8,
    pub max_raidpd_ns_count: u8,
    pub max_devices_per_throttle_group: u8,
    pub io_throttle_data_length: __le16,
    pub max_io_throttle_group: __le16,
    pub io_throttle_low: __le16,
    pub io_throttle_high: __le16,
    pub diag_fdl_size: __le32,
    pub diag_tty_size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_mgmt_passthrough_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub reserved0c: [__le32; 5],
    pub command_sgl: mpi3_sge_union,
    pub response_sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_create_request_queue_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub flags: u8,
    pub burst: u8,
    pub size: __le16,
    pub queue_id: __le16,
    pub reply_queue_id: __le16,
    pub reserved12: __le16,
    pub reserved14: __le32,
    pub base_address: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_delete_request_queue_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub queue_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_create_reply_queue_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub flags: u8,
    pub reserved0b: u8,
    pub size: __le16,
    pub queue_id: __le16,
    pub msix_index: __le16,
    pub reserved12: __le16,
    pub reserved14: __le32,
    pub base_address: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_delete_reply_queue_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub queue_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_port_enable_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_notification_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub sas_broadcast_primitive_masks: __le16,
    pub sas_notify_primitive_masks: __le16,
    pub event_masks: [__le32; MPI3_EVENT_NOTIFY_EVENTMASK_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_notification_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub event_data_length: u8,
    pub event: u8,
    pub ioc_change_count: __le16,
    pub event_context: __le32,
    pub event_data: [__le32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_gpio_interrupt {
    pub gpio_num: u8,
    pub reserved01: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_cable_management {
    pub active_cable_power_requirement: __le32,
    pub status: u8,
    pub receptacle_id: u8,
    pub reserved06: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_ack_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub event: u8,
    pub reserved0d: [u8; 3],
    pub event_context: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_prepare_for_reset {
    pub reason_code: u8,
    pub reserved01: u8,
    pub reserved02: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_comp_image_activation {
    pub reserved00: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_device_status_change {
    pub task_tag: __le16,
    pub reason_code: u8,
    pub io_unit_port: u8,
    pub parent_dev_handle: __le16,
    pub dev_handle: __le16,
    pub wwid: __le64,
    pub lun: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_energy_pack_change {
    pub reserved00: __le32,
    pub shutdown_timeout: __le16,
    pub reserved06: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_discovery {
    pub flags: u8,
    pub reason_code: u8,
    pub io_unit_port: u8,
    pub reserved03: u8,
    pub discovery_status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_broadcast_primitive {
    pub phy_num: u8,
    pub io_unit_port: u8,
    pub port_width: u8,
    pub primitive: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_notify_primitive {
    pub phy_num: u8,
    pub io_unit_port: u8,
    pub reserved02: u8,
    pub primitive: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_sas_topo_phy_entry {
    pub attached_dev_handle: __le16,
    pub link_rate: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_topology_change_list {
    pub enclosure_handle: __le16,
    pub expander_dev_handle: __le16,
    pub num_phys: u8,
    pub reserved05: [u8; 3],
    pub num_entries: u8,
    pub start_phy_num: u8,
    pub exp_status: u8,
    pub io_unit_port: u8,
    pub __counted_by(num_entries): mpi3_event_sas_topo_phy_entry phy_entry[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_phy_counter {
    pub time_stamp: __le64,
    pub reserved08: __le32,
    pub phy_event_code: u8,
    pub phy_num: u8,
    pub reserved0e: __le16,
    pub phy_event_info: __le32,
    pub counter_type: u8,
    pub threshold_window: u8,
    pub time_units: u8,
    pub reserved17: u8,
    pub event_threshold: __le32,
    pub threshold_flags: __le16,
    pub reserved1e: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_device_disc_err {
    pub dev_handle: __le16,
    pub reason_code: u8,
    pub io_unit_port: u8,
    pub reserved04: __le32,
    pub sas_address: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_pcie_enumeration {
    pub flags: u8,
    pub reason_code: u8,
    pub io_unit_port: u8,
    pub reserved03: u8,
    pub enumeration_status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_pcie_topo_port_entry {
    pub attached_dev_handle: __le16,
    pub port_status: u8,
    pub reserved03: u8,
    pub current_port_info: u8,
    pub reserved05: u8,
    pub previous_port_info: u8,
    pub reserved07: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_pcie_topology_change_list {
    pub enclosure_handle: __le16,
    pub switch_dev_handle: __le16,
    pub num_ports: u8,
    pub reserved05: [u8; 3],
    pub num_entries: u8,
    pub start_port_num: u8,
    pub switch_status: u8,
    pub io_unit_port: u8,
    pub reserved0c: __le32,
    pub __counted_by(num_entries): mpi3_event_pcie_topo_port_entry port_entry[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_pcie_error_threshold {
    pub timestamp: __le64,
    pub reason_code: u8,
    pub port: u8,
    pub switch_dev_handle: __le16,
    pub error: u8,
    pub action: u8,
    pub threshold_count: __le16,
    pub attached_dev_handle: __le16,
    pub reserved12: __le16,
    pub reserved14: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_init_dev_status_change {
    pub reason_code: u8,
    pub io_unit_port: u8,
    pub dev_handle: __le16,
    pub reserved04: __le32,
    pub sas_address: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_sas_init_table_overflow {
    pub max_init: __le16,
    pub current_init: __le16,
    pub reserved04: __le32,
    pub sas_address: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_hard_reset_received {
    pub reserved00: u8,
    pub io_unit_port: u8,
    pub reserved02: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_event_data_diag_buffer_status_change {
    pub type: u8,
    pub reason_code: u8,
    pub reserved02: __le16,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_seq {
    pub newest: __le32,
    pub oldest: __le32,
    pub clear: __le32,
    pub shutdown: __le32,
    pub boot: __le32,
    pub last_acknowledged: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_entry {
    pub time_stamp: __le64,
    pub sequence_number: __le32,
    pub log_code: __le16,
    pub arg_type: __le16,
    pub locale: __le16,
    pub class: u8,
    pub flags: u8,
    pub ext_num: u8,
    pub num_exts: u8,
    pub arg_data_size: u8,
    pub fixed_format_strings_size: u8,
    pub reserved18: [__le32; 2],
    pub pel_info: [__le32; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_list {
    pub log_count: __le32,
    pub reserved04: __le32,
    pub entry: [mpi3_pel_entry; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_arg_map {
    pub arg_type: u8,
    pub length: u8,
    pub start_location: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_print_string {
    pub log_code: __le16,
    pub string_length: __le16,
    pub num_arg_map: u8,
    pub reserved05: [u8; 3],
    pub arg_map: [mpi3_pel_arg_map; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_print_string_list {
    pub num_print_strings: __le32,
    pub residual_bytes_remain: __le32,
    pub reserved08: [__le32; 2],
    pub print_string: [mpi3_pel_print_string; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub action_specific: [__le32; MPI3_PEL_ACTION_SPECIFIC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_get_sequence_numbers {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub reserved0c: [__le32; 5],
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_clear_log_marker {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub clear_type: u8,
    pub reserved0d: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_get_log {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub starting_sequence_number: __le32,
    pub locale: __le16,
    pub class: u8,
    pub reserved13: u8,
    pub reserved14: [__le32; 3],
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_get_count {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub starting_sequence_number: __le32,
    pub locale: __le16,
    pub class: u8,
    pub reserved13: u8,
    pub reserved14: [__le32; 3],
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_wait {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub starting_sequence_number: __le32,
    pub locale: __le16,
    pub class: u8,
    pub reserved13: u8,
    pub wait_time: __le16,
    pub reserved16: __le16,
    pub reserved18: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_abort {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub reserved0c: __le32,
    pub abort_host_tag: __le16,
    pub reserved12: __le16,
    pub reserved14: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_get_print_strings {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub reserved0c: __le32,
    pub start_log_code: __le16,
    pub reserved12: __le16,
    pub reserved14: [__le32; 3],
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_req_action_acknowledge {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub sequence_number: __le32,
    pub reserved10: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pel_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub action: u8,
    pub reserved11: u8,
    pub reserved12: __le16,
    pub pe_log_status: __le16,
    pub reserved16: __le16,
    pub transfer_length: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ci_download_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub action: u8,
    pub reserved0b: u8,
    pub signature1: __le32,
    pub total_image_size: __le32,
    pub image_offset: __le32,
    pub segment_size: __le32,
    pub reserved1c: __le32,
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ci_download_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub flags: u8,
    pub cache_dirty: u8,
    pub pending_count: u8,
    pub reserved13: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ci_upload_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: __le16,
    pub signature1: __le32,
    pub reserved10: __le32,
    pub image_offset: __le32,
    pub segment_size: __le32,
    pub reserved1c: __le32,
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_iounit_control_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub reserved0a: u8,
    pub operation: u8,
    pub reserved0c: __le32,
    pub param64: [__le64; 2],
    pub param32: [__le32; 4],
    pub param16: [__le16; 4],
    pub param8: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_iounit_control_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub value64: [__le64; 2],
    pub value32: [__le32; 4],
    pub value16: [__le16; 4],
    pub value8: [u8; 8],
}
