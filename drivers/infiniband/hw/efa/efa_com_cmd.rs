//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa_com_cmd.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright 2018-2026 Amazon.com, Inc. or its affiliates. All rights reserved.
//

pub const EFA_GID_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_qp_params {
    pub rq_base_addr: u64,
    pub send_cq_idx: u32,
    pub recv_cq_idx: u32,
//
// Send descriptor ring size in bytes,
// sufficient for user-provided number of WQEs and SGL size
//
    pub sq_ring_size_in_bytes: u32,
// Max number of WQEs that will be posted on send queue
    pub sq_depth: u32,
// Recv descriptor ring size in bytes
    pub rq_ring_size_in_bytes: u32,
    pub rq_depth: u32,
    pub pd: u16,
    pub uarn: u16,
    pub qp_type: u8,
    pub sl: u8,
    pub 1: u8 unsolicited_write_recv :,
    pub 1: u8 sq_64_bit_req_id :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_qp_result {
    pub qp_handle: u32,
    pub qp_num: u32,
    pub sq_db_offset: u32,
    pub rq_db_offset: u32,
    pub llq_descriptors_offset: u32,
    pub send_sub_cq_idx: u16,
    pub recv_sub_cq_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_modify_qp_params {
    pub modify_mask: u32,
    pub qp_handle: u32,
    pub qp_state: u32,
    pub cur_qp_state: u32,
    pub qkey: u32,
    pub sq_psn: u32,
    pub sq_drained_async_notify: u8,
    pub rnr_retry: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_query_qp_params {
    pub qp_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_query_qp_result {
    pub qp_state: u32,
    pub qkey: u32,
    pub sq_draining: u32,
    pub sq_psn: u32,
    pub rnr_retry: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_destroy_qp_params {
    pub qp_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_cq_params {
// cq physical base address in OS memory
    pub dma_addr: dma_addr_t,
// completion queue depth in # of entries
    pub sub_cq_depth: u16,
    pub num_sub_cqs: u16,
    pub uarn: u16,
    pub eqn: u16,
    pub entry_size_in_bytes: u8,
    pub 1: u8 interrupt_mode_enabled :,
    pub 1: u8 set_src_addr :,
    pub 1: u8 sq_comp_64_bit_req_id :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_cq_result {
// cq identifier
    pub cq_idx: u16,
// actual cq depth in # of entries
    pub actual_depth: u16,
    pub db_off: u32,
    pub db_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_destroy_cq_params {
    pub cq_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_ah_params {
    pub pdn: u16,
// Destination address in network byte order
    pub dest_addr: [u8; EFA_GID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_ah_result {
    pub ah: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_destroy_ah_params {
    pub ah: u16,
    pub gid: [u8; EFA_GID_SIZE],
    pub pdn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_get_device_attr_result {
    pub addr: [u8; EFA_GID_SIZE],
    pub page_size_cap: u64,
    pub max_mr_pages: u64,
    pub guid: u64,
    pub mtu: u32,
    pub fw_version: u32,
    pub admin_api_version: u32,
    pub device_version: u32,
    pub supported_features: u32,
    pub phys_addr_width: u32,
    pub virt_addr_width: u32,
    pub max_qp: u32,
    pub /: *mut *mut u32 max_sq_depth; / wqes,
    pub /: *mut *mut u32 max_rq_depth; / wqes,
    pub max_cq: u32,
    pub /: *mut *mut u32 max_cq_depth; / cqes,
    pub inline_buf_size: u32,
    pub inline_buf_size_ex: u32,
    pub max_mr: u32,
    pub max_pd: u32,
    pub max_ah: u32,
    pub max_llq_size: u32,
    pub max_rdma_size: u32,
    pub device_caps: u32,
    pub max_eq: u32,
    pub max_eq_depth: u32,
    pub /: *mut *mut u32 event_bitmask; / EQ events bitmask,
    pub sub_cqs_per_cq: u16,
    pub max_sq_sge: u16,
    pub max_rq_sge: u16,
    pub max_wr_rdma_sge: u16,
    pub max_tx_batch: u16,
    pub min_sq_depth: u16,
    pub max_link_speed_gbps: u16,
    pub db_bar: u8,
    pub max_event_counters: u32,
    pub event_counter_max_val: u64,
    pub supported_event_counter_qp_events: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_get_hw_hints_result {
    pub mmio_read_timeout: u16,
    pub driver_watchdog_timeout: u16,
    pub admin_completion_timeout: u16,
    pub poll_interval: u16,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_mem_addr {
    pub mem_addr_low: u32,
    pub mem_addr_high: u32,
}

// Used at indirect mode page list chunks for chaining
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_ctrl_buff_info {
// indicates length of the buffer pointed by control_buffer_address.
    pub length: u32,
// points to control buffer (direct or indirect)
    pub address: efa_com_mem_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_reg_mr_params {
// Memory region length, in bytes.
    pub mr_length_in_bytes: u64,
// IO Virtual Address associated with this MR.
    pub iova: u64,
// words 8:15: Physical Buffer List, each element is page-aligned.
//
// Inline array of physical addresses of app pages
// (optimization for short region reservations)
//
    pub inline_pbl_array: [u64; 4],
//
// Describes the next physically contiguous chunk of indirect
// page list. A page list contains physical addresses of command
// data pages. Data pages are 4KB; page list chunks are
// variable-sized.
//
    pub pbl: efa_com_ctrl_buff_info,
    pub pbl: },
// number of pages in PBL (redundant, could be calculated)
    pub page_num: u32,
// Protection Domain
    pub pd: u16,
//
// phys_page_size_shift - page size is (1 << phys_page_size_shift)
// Page size is used for building the Virtual to Physical
// address mapping
//
    pub page_shift: u8,
// see permissions field of struct efa_admin_reg_mr_cmd
    pub permissions: u8,
    pub inline_pbl: u8,
    pub indirect: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_mr_interconnect_info {
    pub recv_ic_id: u16,
    pub rdma_read_ic_id: u16,
    pub rdma_recv_ic_id: u16,
    pub 1: u8 recv_ic_id_valid :,
    pub 1: u8 rdma_read_ic_id_valid :,
    pub 1: u8 rdma_recv_ic_id_valid :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_reg_mr_result {
//
// To be used in conjunction with local buffers references in SQ and
// RQ WQE
//
    pub l_key: u32,
//
// To be used in incoming RDMA semantics messages to refer to remotely
// accessed memory region
//
    pub r_key: u32,
    pub ic_info: efa_com_mr_interconnect_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_dereg_mr_params {
    pub l_key: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_alloc_pd_result {
    pub pdn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_dealloc_pd_params {
    pub pdn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_alloc_uar_result {
    pub uarn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_dealloc_uar_params {
    pub uarn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_get_stats_params {
// see enum efa_admin_get_stats_type
    pub type: u8,
// see enum efa_admin_get_stats_scope
    pub scope: u8,
    pub scope_modifier: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_basic_stats {
    pub tx_bytes: u64,
    pub tx_pkts: u64,
    pub rx_bytes: u64,
    pub rx_pkts: u64,
    pub rx_drops: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_messages_stats {
    pub send_bytes: u64,
    pub send_wrs: u64,
    pub recv_bytes: u64,
    pub recv_wrs: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_rdma_read_stats {
    pub read_wrs: u64,
    pub read_bytes: u64,
    pub read_wr_err: u64,
    pub read_resp_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_rdma_write_stats {
    pub write_wrs: u64,
    pub write_bytes: u64,
    pub write_wr_err: u64,
    pub write_recv_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_network_stats {
    pub retrans_bytes: u64,
    pub retrans_pkts: u64,
    pub retrans_timeout_events: u64,
    pub unresponsive_remote_events: u64,
    pub impaired_remote_conn_events: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union efa_com_get_stats_result {
    pub basic_stats: efa_com_basic_stats,
    pub messages_stats: efa_com_messages_stats,
    pub rdma_read_stats: efa_com_rdma_read_stats,
    pub rdma_write_stats: efa_com_rdma_write_stats,
    pub network_stats: efa_com_network_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_event_counter_params {
    pub dma_addr: dma_addr_t,
    pub uarn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_event_counter_result {
    pub cntr_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_destroy_event_counter_params {
    pub cntr_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_attach_event_counter_params {
    pub cntr_handle: u32,
    pub qp_handle: u32,
    pub events: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_detach_event_counter_params {
    pub cntr_handle: u32,
    pub qp_handle: u32,
    pub events: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_modify_event_counter_params {
    pub cntr_handle: u32,
    pub operation: u8,
    pub value: u64,
}

extern "C" {
    pub fn efa_com_set_aenq_config(edev: *mut efa_com_dev, groups: u32) -> c_int;
}
