//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa_admin_cmds_defs.h
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
// EFA admin queue opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_aq_opcode {
    EFA_ADMIN_CREATE_QP                         = 1,
    EFA_ADMIN_MODIFY_QP                         = 2,
    EFA_ADMIN_QUERY_QP                          = 3,
    EFA_ADMIN_DESTROY_QP                        = 4,
    EFA_ADMIN_CREATE_AH                         = 5,
    EFA_ADMIN_DESTROY_AH                        = 6,
    EFA_ADMIN_REG_MR                            = 7,
    EFA_ADMIN_DEREG_MR                          = 8,
    EFA_ADMIN_CREATE_CQ                         = 9,
    EFA_ADMIN_DESTROY_CQ                        = 10,
    EFA_ADMIN_GET_FEATURE                       = 11,
    EFA_ADMIN_SET_FEATURE                       = 12,
    EFA_ADMIN_GET_STATS                         = 13,
    EFA_ADMIN_ALLOC_PD                          = 14,
    EFA_ADMIN_DEALLOC_PD                        = 15,
    EFA_ADMIN_ALLOC_UAR                         = 16,
    EFA_ADMIN_DEALLOC_UAR                       = 17,
    EFA_ADMIN_CREATE_EQ                         = 18,
    EFA_ADMIN_DESTROY_EQ                        = 19,
    EFA_ADMIN_ALLOC_MR                          = 20,
    EFA_ADMIN_SERVICE                           = 21,
    EFA_ADMIN_CREATE_EVENT_COUNTER              = 25,
    EFA_ADMIN_DESTROY_EVENT_COUNTER             = 26,
    EFA_ADMIN_ATTACH_EVENT_COUNTER              = 27,
    EFA_ADMIN_MODIFY_EVENT_COUNTER              = 28,
    EFA_ADMIN_DETACH_EVENT_COUNTER              = 29,
    EFA_ADMIN_MAX_OPCODE                        = 29,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_aq_feature_id {
    EFA_ADMIN_DEVICE_ATTR                       = 1,
    EFA_ADMIN_AENQ_CONFIG                       = 2,
    EFA_ADMIN_NETWORK_ATTR                      = 3,
    EFA_ADMIN_QUEUE_ATTR_1                      = 4,
    EFA_ADMIN_HW_HINTS                          = 5,
    EFA_ADMIN_HOST_INFO                         = 6,
    EFA_ADMIN_EVENT_QUEUE_ATTR                  = 7,
    EFA_ADMIN_QUEUE_ATTR_2                      = 9,
}

// QP transport type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_qp_type {
// Unreliable Datagram
    EFA_ADMIN_QP_TYPE_UD                        = 1,
// Scalable Reliable Datagram
    EFA_ADMIN_QP_TYPE_SRD                       = 2,
}

// QP state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_qp_state {
    EFA_ADMIN_QP_STATE_RESET                    = 0,
    EFA_ADMIN_QP_STATE_INIT                     = 1,
    EFA_ADMIN_QP_STATE_RTR                      = 2,
    EFA_ADMIN_QP_STATE_RTS                      = 3,
    EFA_ADMIN_QP_STATE_SQD                      = 4,
    EFA_ADMIN_QP_STATE_SQE                      = 5,
    EFA_ADMIN_QP_STATE_ERR                      = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_get_stats_type {
    EFA_ADMIN_GET_STATS_TYPE_BASIC              = 0,
    EFA_ADMIN_GET_STATS_TYPE_MESSAGES           = 1,
    EFA_ADMIN_GET_STATS_TYPE_RDMA_READ          = 2,
    EFA_ADMIN_GET_STATS_TYPE_RDMA_WRITE         = 3,
    EFA_ADMIN_GET_STATS_TYPE_NETWORK            = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_get_stats_scope {
    EFA_ADMIN_GET_STATS_SCOPE_ALL               = 0,
    EFA_ADMIN_GET_STATS_SCOPE_QUEUE             = 1,
}

//
// QP allocation sizes, converted by fabric QueuePair (QP) create command
// from QP capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_qp_alloc_size {
// Send descriptor ring size in bytes
    pub send_queue_ring_size: u32,
// Max number of WQEs that can be outstanding on send queue.
    pub send_queue_depth: u32,
//
// Recv descriptor ring size in bytes, sufficient for user-provided
// number of WQEs
//
    pub recv_queue_ring_size: u32,
// Max number of WQEs that can be outstanding on recv queue
    pub recv_queue_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_qp_cmd {
// Protection Domain associated with this QP
    pub pd: u16,
// QP type
    pub qp_type: u8,
//
// 0 : sq_virt - If set, SQ ring base address is
// virtual (IOVA returned by MR registration)
// 1 : rq_virt - If set, RQ ring base address is
// virtual (IOVA returned by MR registration)
// 2 : unsolicited_write_recv - If set, work requests
// will not be consumed for incoming RDMA write with
// immediate
// 3 : sq_64_bit_req_id - If set, requests posted on
// SQ will use 64-bit ids. The corresponding CQ must
// also have 64-bit ids enabled.
// 7:4 : reserved - MBZ
//
    pub flags: u8,
//
// Send queue (SQ) ring base physical address. This field is not
// used if this is a Low Latency Queue(LLQ).
//
    pub sq_base_addr: u64,
// Receive queue (RQ) ring base address.
    pub rq_base_addr: u64,
// Index of CQ to be associated with Send Queue completions
    pub send_cq_idx: u32,
// Index of CQ to be associated with Recv Queue completions
    pub recv_cq_idx: u32,
//
// Memory registration key for the SQ ring, used only when not in
// LLQ mode and base address is virtual
//
    pub sq_l_key: u32,
//
// Memory registration key for the RQ ring, used only when base
// address is virtual
//
    pub rq_l_key: u32,
// Requested QP allocation sizes
    pub qp_alloc_size: efa_admin_qp_alloc_size,
// UAR number
    pub uar: u16,
// Requested service level for the QP, 0 is the default SL
    pub sl: u8,
// MBZ
    pub reserved: u8,
// MBZ
    pub reserved2: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_qp_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
//
// Opaque handle to be used for consequent admin operations on the
// QP
//
    pub qp_handle: u32,
//
// QP number in the given EFA virtual device. Least-significant bits (as
// needed according to max_qp) carry unique QP ID
//
    pub qp_num: u16,
// MBZ
    pub reserved: u16,
// Index of sub-CQ for Send Queue completions
    pub send_sub_cq_idx: u16,
// Index of sub-CQ for Receive Queue completions
    pub recv_sub_cq_idx: u16,
// SQ doorbell address, as offset to PCIe DB BAR
    pub sq_db_offset: u32,
// RQ doorbell address, as offset to PCIe DB BAR
    pub rq_db_offset: u32,
//
// low latency send queue ring base address as an offset to PCIe
// MMIO LLQ_MEM BAR
//
    pub llq_descriptors_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_modify_qp_cmd {
//
// Mask indicating which fields should be updated
// 0 : qp_state
// 1 : cur_qp_state
// 2 : qkey
// 3 : sq_psn
// 4 : sq_drained_async_notify
// 5 : rnr_retry
// 31:6 : reserved
//
    pub modify_mask: u32,
// QP handle returned by create_qp command
    pub qp_handle: u32,
// QP state
    pub qp_state: u32,
// Override current QP state (before applying the transition)
    pub cur_qp_state: u32,
// QKey
    pub qkey: u32,
// SQ PSN
    pub sq_psn: u32,
// Enable async notification when SQ is drained
    pub sq_drained_async_notify: u8,
// Number of RNR retries (valid only for SRD QPs)
    pub rnr_retry: u8,
// MBZ
    pub reserved2: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_modify_qp_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_query_qp_cmd {
// QP handle returned by create_qp command
    pub qp_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_query_qp_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
// QP state
    pub qp_state: u32,
// QKey
    pub qkey: u32,
// SQ PSN
    pub sq_psn: u32,
// Indicates that draining is in progress
    pub sq_draining: u8,
// Number of RNR retries (valid only for SRD QPs)
    pub rnr_retry: u8,
// MBZ
    pub reserved2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_qp_cmd {
// QP handle returned by create_qp command
    pub qp_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_qp_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
}

//
// Create Address Handle command parameters. Must not be called more than
// once for the same destination
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_ah_cmd {
// Destination address in network byte order
    pub dest_addr: [u8; 16],
// PD number
    pub pd: u16,
// MBZ
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_ah_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
// Target interface address handle (opaque)
    pub ah: u16,
// MBZ
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_ah_cmd {
// Target interface address handle (opaque)
    pub ah: u16,
// PD number
    pub pd: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_ah_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
}

//
// Registration of MemoryRegion, required for QP working with Virtual
// Addresses. In standard verbs semantics, region length is limited to 2GB
// space, but EFA offers larger MR support for large memory space, to ease
// on users working with very large datasets (i.e. full GPU memory mapping).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_reg_mr_cmd {
// Protection Domain
    pub pd: u16,
// MBZ
    pub reserved16_w1: u16,
// Physical Buffer List, each element is page-aligned.
//
// Inline array of guest-physical page addresses of user
// memory pages (optimization for short region
// registrations)
//
    pub inline_pbl_array: [u64; 4],
// points to PBL (direct or indirect, chained if needed)
    pub pbl: efa_admin_ctrl_buff_info,
    pub pbl: },
// Memory region length, in bytes.
    pub mr_length: u64,
//
// flags and page size
// 5:0 : phys_page_size_shift - page size is (1 <<
// phys_page_size_shift). Page size is used for
// building the Virtual to Physical address mapping
// 6 : reserved - MBZ
// 7 : mem_addr_phy_mode_en - Enable bit for physical
// memory registration (no translation), can be used
// only by privileged clients. If set, PBL must
// contain a single entry.
//
    pub flags: u8,
//
// permissions
// 0 : local_write_enable - Local write permissions:
// must be set for RQ buffers and buffers posted for
// RDMA Read requests
// 1 : remote_write_enable - Remote write
// permissions: must be set to enable RDMA write to
// the region
// 2 : remote_read_enable - Remote read permissions:
// must be set to enable RDMA read from the region
// 7:3 : reserved2 - MBZ
//
    pub permissions: u8,
// MBZ
    pub reserved16_w5: u16,
// number of pages in PBL (redundant, could be calculated)
    pub page_num: u32,
//
// IO Virtual Address associated with this MR. If
// mem_addr_phy_mode_en is set, contains the physical address of
// the region.
//
    pub iova: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_reg_mr_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
//
// L_Key, to be used in conjunction with local buffer references in
// SQ and RQ WQE, or with virtual RQ/CQ rings
//
    pub l_key: u32,
//
// R_Key, to be used in RDMA messages to refer to remotely accessed
// memory region
//
    pub r_key: u32,
//
// Mask indicating which fields have valid values
// 0 : recv_ic_id
// 1 : rdma_read_ic_id
// 2 : rdma_recv_ic_id
//
    pub validity: u8,
//
// Physical interconnect used by the device to reach the MR for receive
// operation
//
    pub recv_ic_id: u8,
//
// Physical interconnect used by the device to reach the MR for RDMA
// read operation
//
    pub rdma_read_ic_id: u8,
//
// Physical interconnect used by the device to reach the MR for RDMA
// write receive
//
    pub rdma_recv_ic_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_dereg_mr_cmd {
// L_Key, memory region's l_key
    pub l_key: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_dereg_mr_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
}

//
// Allocation of MemoryRegion, required for QP working with Virtual
// Addresses in kernel verbs semantics, ready for fast registration use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_alloc_mr_cmd {
// Protection Domain
    pub pd: u16,
// MBZ
    pub reserved1: u16,
// Maximum number of pages this MR supports.
    pub max_pages: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_alloc_mr_resp {
// Common Admin Queue completion descriptor
    pub acq_common_desc: efa_admin_acq_common_desc,
//
// L_Key, to be used in conjunction with local buffer references in
// SQ and RQ WQE, or with virtual RQ/CQ rings
//
    pub l_key: u32,
//
// R_Key, to be used in RDMA messages to refer to remotely accessed
// memory region
//
    pub r_key: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_cq_cmd {
//
// 4:0 : reserved5 - MBZ
// 5 : interrupt_mode_enabled - if set, cq operates
// in interrupt mode (i.e. CQ events and EQ elements
// are generated), otherwise - polling
// 6 : virt - If set, ring base address is virtual
// (IOVA returned by MR registration)
// 7 : reserved6 - MBZ
//
    pub cq_caps_1: u8,
//
// 4:0 : cq_entry_size_words - size of CQ entry in
// 32-bit words, valid values: 4, 8.
// 5 : set_src_addr - If set, source address will be
// filled on RX completions from unknown senders.
// Requires 8 words CQ entry size.
// 6 : sq_comp_64_bit_req_id - If set, send
// completions will use 64-bit work request ids
// 7 : reserved7 - MBZ
//
    pub cq_caps_2: u8,
// Sub completion queue depth in # of entries. must be power of 2
    pub sub_cq_depth: u16,
// EQ number assigned to this cq
    pub eqn: u16,
// MBZ
    pub reserved: u16,
//
// CQ ring base address, virtual or physical depending on 'virt'
// flag
//
    pub cq_ba: efa_common_mem_addr,
//
// Memory registration key for the ring, used only when base
// address is virtual
//
    pub l_key: u32,
//
// number of sub cqs - must be equal to sub_cqs_per_cq of queue
// attributes.
//
    pub num_sub_cqs: u16,
// UAR number
    pub uar: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_cq_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
    pub cq_idx: u16,
// actual sub cq depth in number of entries
    pub sub_cq_actual_depth: u16,
// CQ doorbell address, as offset to PCIe DB BAR
    pub db_offset: u32,
//
// 0 : db_valid - If set, doorbell offset is valid.
// Always set when interrupts are requested.
//
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_cq_cmd {
    pub cq_idx: u16,
// MBZ
    pub reserved1: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_cq_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
}

//
// EFA AQ Get Statistics command. Extended statistics are placed in control
// buffer pointed by AQ entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_aq_get_stats_cmd {
    pub control_buffer: efa_admin_ctrl_buff_info,
// stats type as defined in enum efa_admin_get_stats_type
    pub type: u8,
// stats scope defined in enum efa_admin_get_stats_scope
    pub scope: u8,
    pub scope_modifier: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_basic_stats {
    pub tx_bytes: u64,
    pub tx_pkts: u64,
    pub rx_bytes: u64,
    pub rx_pkts: u64,
    pub rx_drops: u64,
    pub qkey_viol: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_messages_stats {
    pub send_bytes: u64,
    pub send_wrs: u64,
    pub recv_bytes: u64,
    pub recv_wrs: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_rdma_read_stats {
    pub read_wrs: u64,
    pub read_bytes: u64,
    pub read_wr_err: u64,
    pub read_resp_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_rdma_write_stats {
    pub write_wrs: u64,
    pub write_bytes: u64,
    pub write_wr_err: u64,
    pub write_recv_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_network_stats {
    pub retrans_bytes: u64,
    pub retrans_pkts: u64,
    pub retrans_timeout_events: u64,
    pub unresponsive_remote_events: u64,
    pub impaired_remote_conn_events: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_acq_get_stats_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
    pub basic_stats: efa_admin_basic_stats,
    pub messages_stats: efa_admin_messages_stats,
    pub rdma_read_stats: efa_admin_rdma_read_stats,
    pub rdma_write_stats: efa_admin_rdma_write_stats,
    pub network_stats: efa_admin_network_stats,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_get_set_feature_common_desc {
// MBZ
    pub reserved0: u8,
// as appears in efa_admin_aq_feature_id
    pub feature_id: u8,
// MBZ
    pub reserved16: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_feature_device_attr_desc {
// Bitmap of efa_admin_aq_feature_id
    pub supported_features: u64,
// Bitmap of supported page sizes in MR registrations
    pub page_size_cap: u64,
    pub fw_version: u32,
    pub admin_api_version: u32,
    pub device_version: u32,
// Bar used for SQ and RQ doorbells
    pub db_bar: u16,
// Indicates how many bits are used on physical address access
    pub phys_addr_width: u8,
// Indicates how many bits are used on virtual address access
    pub virt_addr_width: u8,
//
// 0 : rdma_read - If set, RDMA Read is supported on
// TX queues
// 1 : rnr_retry - If set, RNR retry is supported on
// modify QP command
// 2 : data_polling_128 - If set, 128 bytes data
// polling is supported
// 3 : rdma_write - If set, RDMA Write is supported
// on TX queues
// 4 : unsolicited_write_recv - If set, unsolicited
// write with imm. receive is supported
// 5 : event_counters - If set, event counters are
// supported
// 9:6 : reserved1 - MBZ
// 10 : sq_64_bit_req_id - If set, SQ can use 64-bit
// work request ids
// 31:11 : reserved2 - MBZ
//
    pub device_caps: u32,
// Max RDMA transfer size in bytes
    pub max_rdma_size: u32,
// Unique global ID for an EFA device
    pub guid: u64,
// The device maximum link speed in Gbit/sec
    pub max_link_speed_gbps: u16,
// MBZ
    pub reserved0: u16,
// MBZ
    pub reserved1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_feature_queue_attr_desc_1 {
// The maximum number of queue pairs supported
    pub max_qp: u32,
// Maximum number of WQEs per Send Queue
    pub max_sq_depth: u32,
//
// Maximum size of data that can be sent inline in a Send WQE
// (deprecated by
// efa_admin_feature_queue_attr_desc_2::inline_buf_size_ex on
// supporting devices)
//
    pub inline_buf_size: u32,
// Maximum number of buffer descriptors per Recv Queue
    pub max_rq_depth: u32,
// The maximum number of completion queues supported per VF
    pub max_cq: u32,
// Maximum number of CQEs per Completion Queue
    pub max_cq_depth: u32,
// Number of sub-CQs to be created for each CQ
    pub sub_cqs_per_cq: u16,
// Minimum number of WQEs per SQ
    pub min_sq_depth: u16,
// Maximum number of SGEs (buffers) allowed for a single send WQE
    pub max_wr_send_sges: u16,
// Maximum number of SGEs allowed for a single recv WQE
    pub max_wr_recv_sges: u16,
// The maximum number of memory regions supported
    pub max_mr: u32,
// The maximum number of pages can be registered
    pub max_mr_pages: u32,
// The maximum number of protection domains supported
    pub max_pd: u32,
// The maximum number of address handles supported
    pub max_ah: u32,
// The maximum size of LLQ in bytes
    pub max_llq_size: u32,
// Maximum number of SGEs for a single RDMA read/write WQE
    pub max_wr_rdma_sges: u16,
//
// Maximum number of bytes that can be written to SQ between two
// consecutive doorbells (in units of 64B). Driver must ensure that only
// complete WQEs are written to queue before issuing a doorbell.
// Examples: max_tx_batch=16 and WQE size = 64B, means up to 16 WQEs can
// be written to SQ between two consecutive doorbells. max_tx_batch=11
// and WQE size = 128B, means up to 5 WQEs can be written to SQ between
// two consecutive doorbells. Zero means unlimited.
//
    pub max_tx_batch: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_feature_queue_attr_desc_2 {
// Maximum size of data that can be sent inline in a Send WQE
    pub inline_buf_size_ex: u16,
// MBZ
    pub reserved: [u8; 6],
//
// Supported counter QP events
// 0 : send_comp
// 1 : send_comp_err
// 2 : recv_comp
// 3 : recv_comp_err
// 4 : read_comp
// 5 : read_comp_err
// 6 : write_comp
// 7 : write_comp_err
// 8 : remote_read_comp
// 9 : remote_write_comp
// 31:10 : reserved - MBZ
//
    pub supported_event_counter_qp_events: u32,
// Maximum number of counters
    pub max_event_counters: u32,
//
// Maximum counter value, counter wraps around to 0 after reaching
// this value
//
    pub event_counter_max_val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_event_queue_attr_desc {
// The maximum number of event queues supported
    pub max_eq: u32,
// Maximum number of EQEs per Event Queue
    pub max_eq_depth: u32,
// Supported events bitmask
    pub event_bitmask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_feature_aenq_desc {
// bitmask for AENQ groups the device can report
    pub supported_groups: u32,
// bitmask for AENQ groups to report
    pub enabled_groups: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_feature_network_attr_desc {
// Raw address data in network byte order
    pub addr: [u8; 16],
// max packet payload size in bytes
    pub mtu: u32,
}

//
// When hint value is 0, hints capabilities are not supported or driver
// should use its own predefined value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_hw_hints {
// value in ms
    pub mmio_read_timeout: u16,
// value in ms
    pub driver_watchdog_timeout: u16,
// value in ms
    pub admin_completion_timeout: u16,
// poll interval in ms
    pub poll_interval: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_get_feature_cmd {
    pub control_buffer: efa_admin_ctrl_buff_info,
    pub feature_common: efa_admin_get_set_feature_common_desc,
    pub raw: [u32; 11],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_get_feature_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
    pub raw: [u32; 14],
    pub device_attr: efa_admin_feature_device_attr_desc,
    pub aenq: efa_admin_feature_aenq_desc,
    pub network_attr: efa_admin_feature_network_attr_desc,
    pub queue_attr_1: efa_admin_feature_queue_attr_desc_1,
    pub queue_attr_2: efa_admin_feature_queue_attr_desc_2,
    pub event_queue_attr: efa_admin_event_queue_attr_desc,
    pub hw_hints: efa_admin_hw_hints,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_set_feature_cmd {
    pub control_buffer: efa_admin_ctrl_buff_info,
    pub feature_common: efa_admin_get_set_feature_common_desc,
    pub raw: [u32; 11],
// AENQ configuration
    pub aenq: efa_admin_feature_aenq_desc,
    pub u: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_set_feature_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
    pub raw: [u32; 14],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_alloc_pd_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
// PD number
    pub pd: u16,
// MBZ
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_dealloc_pd_cmd {
// PD number
    pub pd: u16,
// MBZ
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_dealloc_pd_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_alloc_uar_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
// UAR number
    pub uar: u16,
// MBZ
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_dealloc_uar_cmd {
// UAR number
    pub uar: u16,
// MBZ
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_dealloc_uar_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_eq_cmd {
// Size of the EQ in entries, must be power of 2
    pub depth: u16,
// MSI-X table entry index
    pub msix_vec: u8,
//
// 4:0 : entry_size_words - size of EQ entry in
// 32-bit words
// 7:5 : reserved - MBZ
//
    pub caps: u8,
// EQ ring base address
    pub ba: efa_common_mem_addr,
//
// Enabled events on this EQ
// 0 : completion_events - Enable completion events
// 31:1 : reserved - MBZ
//
    pub event_bitmask: u32,
// MBZ
    pub reserved: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_eq_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
// EQ number
    pub eqn: u16,
// MBZ
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_eq_cmd {
// EQ number
    pub eqn: u16,
// MBZ
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_eq_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
}

// asynchronous event notification groups
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_aenq_group {
    EFA_ADMIN_FATAL_ERROR                       = 1,
    EFA_ADMIN_WARNING                           = 2,
    EFA_ADMIN_NOTIFICATION                      = 3,
    EFA_ADMIN_KEEP_ALIVE                        = 4,
    EFA_ADMIN_AENQ_GROUPS_NUM                   = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_mmio_req_read_less_resp {
    pub req_id: u16,
    pub reg_off: u16,
// value is valid when poll is cleared
    pub reg_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_os_type {
    EFA_ADMIN_OS_LINUX                          = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_host_info {
// OS distribution string format
    pub os_dist_str: [u8; 128],
// Defined in enum efa_admin_os_type
    pub os_type: u32,
// Kernel version string format
    pub kernel_ver_str: [u8; 32],
// Kernel version numeric format
    pub kernel_ver: u32,
//
// 7:0 : driver_module_type
// 15:8 : driver_sub_minor
// 23:16 : driver_minor
// 31:24 : driver_major
//
    pub driver_ver: u32,
//
// Device's Bus, Device and Function
// 2:0 : function
// 7:3 : device
// 15:8 : bus
//
    pub bdf: u16,
//
// Spec version
// 7:0 : spec_minor
// 15:8 : spec_major
//
    pub spec_ver: u16,
//
// 0 : intree - Intree driver
// 1 : gdr - GPUDirect RDMA supported
// 31:2 : reserved2
//
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_service_cmd {
    pub buffer: [u8; 60],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_service_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
    pub buffer: [u8; 56],
}

// Create Counter command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_event_counter_cmd {
// UAR number
    pub uar: u16,
// MBZ
    pub reserved: u16,
// Counter physical address
    pub paddr: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_create_event_counter_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
// Counter handle
    pub cntr_handle: u32,
// MBZ
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_event_counter_cmd {
// Counter handle
    pub cntr_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_destroy_event_counter_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_event_counter_attach_type {
    EFA_ADMIN_EVENT_COUNTER_ATTACH_QP_EVENTS    = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_event_counter_attach_qp_events {
// QP handle
    pub qp_handle: u32,
//
// Bitmask of counter QP events
// 0 : send_comp
// 1 : send_comp_err
// 2 : recv_comp
// 3 : recv_comp_err
// 4 : read_comp
// 5 : read_comp_err
// 6 : write_comp
// 7 : write_comp_err
// 8 : remote_read_comp
// 9 : remote_write_comp
// 31:10 : reserved - MBZ
//
    pub events: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_attach_detach_event_counter_cmd {
// Counter handle
    pub cntr_handle: u32,
// efa_admin_event_counter_attach_type
    pub attach_type: u8,
// MBZ
    pub reserved: [u8; 3],
    pub qp_events: efa_admin_event_counter_attach_qp_events,
    pub u: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_attach_detach_event_counter_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
}

// Counter modify operations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_event_counter_modify_ops {
// Set counter value
    EFA_ADMIN_EVENT_COUNTER_MODIFY_SET          = 0,
// Add to counter value
    EFA_ADMIN_EVENT_COUNTER_MODIFY_ADD          = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_modify_event_counter_cmd {
// Counter handle
    pub cntr_handle: u32,
// Counter operation type (efa_admin_event_counter_modify_ops)
    pub operation: u8,
// MBZ
    pub reserved: [u8; 7],
// Value for SET or ADD
    pub value: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_modify_event_counter_resp {
    pub acq_common_desc: efa_admin_acq_common_desc,
}

// create_qp_cmd

pub const EFA_ADMIN_CREATE_QP_CMD_SQ_64_BIT_REQ_ID_SHIFT: c_int = 3;

// modify_qp_cmd

// reg_mr_cmd

// reg_mr_resp

// create_cq_cmd

pub const EFA_ADMIN_CREATE_CQ_CMD_SQ_COMP_64_BIT_REQ_ID_SHIFT: c_int = 6;

// create_cq_resp

// feature_device_attr_desc

pub const EFA_ADMIN_FEATURE_DEVICE_ATTR_DESC_SQ_64_BIT_REQ_ID_SHIFT: c_int = 10;

// feature_queue_attr_desc_2

// create_eq_cmd

// host_info

// counter_attach_qp_events

