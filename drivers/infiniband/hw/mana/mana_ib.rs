//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mana/mana_ib.h
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
// Copyright (c) 2022 Microsoft Corporation. All rights reserved.
//

// MANA doesn't have any limit for MR size

// Send queue ID mask

// Queue ID encodes type in the lower 2 bits
pub const MANA_QID_SUBTYPE_MASK: c_uint = 0x3;
//
// The hardware limit of number of MRs is greater than maximum number of MRs
// that can possibly represent in 24 bits
//
pub const MANA_IB_MAX_MR: c_uint = 0xFFFFFFu;
//
// The CA timeout is approx. 260ms (4us * 2^(DELAY))
//
pub const MANA_CA_ACK_DELAY: c_int = 16;
//
// The buffer used for writing AV
//
pub const MANA_AV_BUFFER_SIZE: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_adapter_caps {
    pub max_sq_id: u32,
    pub max_rq_id: u32,
    pub max_cq_id: u32,
    pub max_qp_count: u32,
    pub max_cq_count: u32,
    pub max_mr_count: u32,
    pub max_pd_count: u32,
    pub max_inbound_read_limit: u32,
    pub max_outbound_read_limit: u32,
    pub mw_count: u32,
    pub max_srq_count: u32,
    pub max_qp_wr: u32,
    pub max_send_sge_count: u32,
    pub max_recv_sge_count: u32,
    pub max_inline_data_size: u32,
    pub feature_flags: u64,
    pub page_size_cap: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_queue {
    pub umem: *mut ib_umem,
    pub kmem: *mut gdma_queue,
    pub gdma_region: u64,
    pub id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_dev {
    pub ib_dev: ib_device,
    pub gdma_dev: *mut gdma_dev,
    pub adapter_handle: mana_handle_t,
    pub fatal_err_eq: *mut gdma_queue,
    pub eqs: *mut gdma_queue,
    pub qp_table_wq: xarray,
    pub adapter_caps: mana_ib_adapter_caps,
    pub av_pool: *mut dma_pool,
    pub dev_tracker: netdevice_tracker,
    pub nb: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_wq {
    pub ibwq: ib_wq,
    pub queue: mana_ib_queue,
    pub wqe: c_int,
    pub wq_buf_size: u32,
    pub rx_object: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_pd {
    pub ibpd: ib_pd,
    pub pdn: u32,
    pub pd_handle: mana_handle_t,
// Mutex for sharing access to vport_use_count
    pub vport_mutex: mutex,
    pub vport_use_count: c_int,
// Port bound to this PD for raw QP usage. Only valid when
// vport_use_count > 0. A PD can only be associated with a
// single physical port because per-port EQs and vport
// configuration are tied to the PD's refcount.
//
    pub vport_port: u32,
// Only one RSS QP is allowed per vport because each RSS QP
// overwrites the vport steering config (indirection table
// hash key) and mana_disable_vport_rx() on destroy would
// blackhole traffic for any other RSS QP on the same vport.
//
    pub has_rss_qp: bool,
    pub tx_shortform_allowed: bool,
    pub tx_vp_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_av {
    pub dest_ip: [u8; 16],
    pub dest_mac: [u8; ETH_ALEN],
    pub udp_src_port: u16,
    pub src_ip: [u8; 16],
    pub 8: u32 hop_limit :,
    pub 12: u32 reserved1 :,
    pub 6: u32 dscp :,
    pub 5: u32 reserved2 :,
    pub 1: u32 is_ipv6 :,
    pub 32: u32 reserved3 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_ah {
    pub ibah: ib_ah,
    pub av: *mut mana_ib_av,
    pub dma_handle: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_mw {
    pub ibmw: ib_mw,
    pub mw_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub mr_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_dm {
    pub ibdm: ib_dm,
    pub dm_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_cq {
    pub ibcq: ib_cq,
    pub queue: mana_ib_queue,
// protects CQ polling
    pub cq_lock: spinlock_t,
    pub list_send_qp: list_head,
    pub list_recv_qp: list_head,
    pub cqe: c_int,
    pub comp_vector: u32,
    pub cq_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_rc_queue_type {
    MANA_RC_SEND_QUEUE_REQUESTER = 0,
    MANA_RC_SEND_QUEUE_RESPONDER,
    MANA_RC_SEND_QUEUE_FMR,
    MANA_RC_RECV_QUEUE_REQUESTER,
    MANA_RC_RECV_QUEUE_RESPONDER,
    MANA_RC_QUEUE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_rc_qp {
    pub queues: [mana_ib_queue; MANA_RC_QUEUE_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_uc_queue_type {
    MANA_UC_SEND_QUEUE_REQUESTER = 0,
    MANA_UC_RECV_QUEUE_RESPONDER,
    MANA_UC_SEND_QUEUE_MMQ,
    MANA_UC_QUEUE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_uc_qp {
    pub queues: [mana_ib_queue; MANA_UC_QUEUE_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ud_queue_type {
    MANA_UD_SEND_QUEUE = 0,
    MANA_UD_RECV_QUEUE,
    MANA_UD_QUEUE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_ud_qp {
    pub queues: [mana_ib_queue; MANA_UD_QUEUE_TYPE_MAX],
    pub sq_psn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_qp {
    pub ibqp: ib_qp,
    pub qp_handle: mana_handle_t,
    pub raw_sq: mana_ib_queue,
    pub rc_qp: mana_ib_rc_qp,
    pub uc_qp: mana_ib_uc_qp,
    pub ud_qp: mana_ib_ud_qp,
}

// The port on the IB device, starting with 1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_ucontext {
    pub ibucontext: ib_ucontext,
    pub doorbell: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_rwq_ind_table {
    pub ib_ind_table: ib_rwq_ind_table,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_command_code {
    MANA_IB_GET_ADAPTER_CAP = 0x30001,
    MANA_IB_CREATE_ADAPTER  = 0x30002,
    MANA_IB_DESTROY_ADAPTER = 0x30003,
    MANA_IB_CONFIG_IP_ADDR	= 0x30004,
    MANA_IB_CONFIG_MAC_ADDR	= 0x30005,
    MANA_IB_CREATE_UD_QP	= 0x30006,
    MANA_IB_DESTROY_UD_QP	= 0x30007,
    MANA_IB_CREATE_CQ       = 0x30008,
    MANA_IB_DESTROY_CQ      = 0x30009,
    MANA_IB_CREATE_RC_QP    = 0x3000a,
    MANA_IB_DESTROY_RNIC_QP = 0x3000b,
    MANA_IB_SET_QP_STATE	= 0x3000d,
    MANA_IB_CREATE_UC_QP    = 0x30020,
    MANA_IB_QUERY_VF_COUNTERS = 0x30022,
    MANA_IB_QUERY_DEVICE_COUNTERS = 0x30023,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_query_adapter_caps_req {
    pub hdr: gdma_req_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_adapter_features {
    MANA_IB_FEATURE_CLIENT_ERROR_CQE_SUPPORT = BIT(4),
    MANA_IB_FEATURE_DEV_COUNTERS_SUPPORT = BIT(5),
    MANA_IB_FEATURE_MULTI_PORTS_SUPPORT = BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_query_adapter_caps_resp {
    pub hdr: gdma_resp_hdr,
    pub max_sq_id: u32,
    pub max_rq_id: u32,
    pub max_cq_id: u32,
    pub max_qp_count: u32,
    pub max_cq_count: u32,
    pub max_mr_count: u32,
    pub max_pd_count: u32,
    pub max_inbound_read_limit: u32,
    pub max_outbound_read_limit: u32,
    pub mw_count: u32,
    pub max_srq_count: u32,
    pub max_requester_sq_size: u32,
    pub max_responder_sq_size: u32,
    pub max_requester_rq_size: u32,
    pub max_responder_rq_size: u32,
    pub max_send_sge_count: u32,
    pub max_recv_sge_count: u32,
    pub max_inline_data_size: u32,
    pub feature_flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_adapter_features_request {
    MANA_IB_FEATURE_CLIENT_ERROR_CQE_REQUEST = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_adapter_req {
    pub hdr: gdma_req_hdr,
    pub notify_eq_id: u32,
    pub reserved: u32,
    pub feature_flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_adapter_resp {
    pub hdr: gdma_resp_hdr,
    pub adapter: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_adapter_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_adapter_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_addr_op {
    ADDR_OP_ADD = 1,
    ADDR_OP_REMOVE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgid_entry_type {
    SGID_TYPE_IPV4 = 1,
    SGID_TYPE_IPV6 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_config_addr_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub op: mana_ib_addr_op,
    pub sgid_type: sgid_entry_type,
    pub ip_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_config_addr_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_config_mac_addr_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub op: mana_ib_addr_op,
    pub mac_addr: [u8; ETH_ALEN],
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_config_mac_addr_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_cq_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub gdma_region: u64,
    pub eq_id: u32,
    pub doorbell_page: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_cq_resp {
    pub hdr: gdma_resp_hdr,
    pub cq_handle: mana_handle_t,
    pub cq_id: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_cq_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub cq_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_cq_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_rnic_create_rc_flags {
    MANA_RC_FLAG_NO_FMR = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_qp_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub pd_handle: mana_handle_t,
    pub send_cq_handle: mana_handle_t,
    pub recv_cq_handle: mana_handle_t,
    pub dma_region: [u64; MANA_RC_QUEUE_TYPE_MAX],
    pub deprecated: [u64; 2],
    pub flags: u64,
    pub doorbell_page: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_qp_resp {
    pub hdr: gdma_resp_hdr,
    pub rc_qp_handle: mana_handle_t,
    pub queue_ids: [u32; MANA_RC_QUEUE_TYPE_MAX],
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_rnic_qp_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub qp_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_rnic_qp_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_uc_qp_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub pd_handle: mana_handle_t,
    pub send_cq_handle: mana_handle_t,
    pub recv_cq_handle: mana_handle_t,
    pub dma_region: [u64; MANA_UC_QUEUE_TYPE_MAX],
    pub flags: u64,
    pub doorbell_page: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_uc_qp_resp {
    pub hdr: gdma_resp_hdr,
    pub qp_handle: mana_handle_t,
    pub queue_ids: [u32; MANA_UC_QUEUE_TYPE_MAX],
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_udqp_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub pd_handle: mana_handle_t,
    pub send_cq_handle: mana_handle_t,
    pub recv_cq_handle: mana_handle_t,
    pub dma_region: [u64; MANA_UD_QUEUE_TYPE_MAX],
    pub qp_type: u32,
    pub doorbell_page: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_create_udqp_resp {
    pub hdr: gdma_resp_hdr,
    pub qp_handle: mana_handle_t,
    pub queue_ids: [u32; MANA_UD_QUEUE_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_udqp_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub qp_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_destroy_udqp_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ib_ah_attr {
    pub src_addr: [u8; 16],
    pub dest_addr: [u8; 16],
    pub src_mac: [u8; ETH_ALEN],
    pub dest_mac: [u8; ETH_ALEN],
    pub src_addr_type: u8,
    pub dest_addr_type: u8,
    pub hop_limit: u8,
    pub traffic_class: u8,
    pub src_port: u16,
    pub dest_port: u16,
    pub flow_label: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_set_qp_state_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
    pub qp_handle: mana_handle_t,
    pub attr_mask: u64,
    pub qp_state: u32,
    pub path_mtu: u32,
    pub rq_psn: u32,
    pub sq_psn: u32,
    pub dest_qpn: u32,
    pub max_dest_rd_atomic: u32,
    pub retry_cnt: u32,
    pub rnr_retry: u32,
    pub min_rnr_timer: u32,
    pub rate_limit: u32,
    pub ah_attr: mana_ib_ah_attr,
    pub reserved1: u64,
    pub qkey: u32,
    pub qp_access_flags: u32,
    pub local_ack_timeout: u8,
    pub max_rd_atomic: u8,
    pub reserved2: u16,
    pub reserved3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_set_qp_state_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WQE_OPCODE_TYPES {
    WQE_TYPE_UD_SEND = 0,
    WQE_TYPE_UD_RECV = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_send_oob {
    pub 5: u32 wqe_type :,
    pub 1: u32 fence :,
    pub 1: u32 signaled :,
    pub 1: u32 solicited :,
    pub 24: u32 psn :,
    pub 24: u32 ssn_or_rqpn :,
    pub 8: u32 reserved1 :,
    pub remote_qkey: u32,
    pub immediate: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub ud_send: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rdma_cqe {
    pub cqe_type: u8,
    pub 1]: u8 data[GDMA_COMP_DATA_SIZE -,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_query_vf_cntrs_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_query_vf_cntrs_resp {
    pub hdr: gdma_resp_hdr,
    pub requester_timeout: u64,
    pub requester_oos_nak: u64,
    pub requester_rnr_nak: u64,
    pub responder_rnr_nak: u64,
    pub responder_oos: u64,
    pub responder_dup_request: u64,
    pub requester_implicit_nak: u64,
    pub requester_readresp_psn_mismatch: u64,
    pub nak_inv_req: u64,
    pub nak_access_err: u64,
    pub nak_opp_err: u64,
    pub nak_inv_read: u64,
    pub responder_local_len_err: u64,
    pub requestor_local_prot_err: u64,
    pub responder_rem_access_err: u64,
    pub responder_local_qp_err: u64,
    pub responder_malformed_wqe: u64,
    pub general_hw_err: u64,
    pub requester_rnr_nak_retries_exceeded: u64,
    pub requester_retries_exceeded: u64,
    pub total_fatal_err: u64,
    pub received_cnps: u64,
    pub num_qps_congested: u64,
    pub rate_inc_events: u64,
    pub num_qps_recovered: u64,
    pub current_rate: u64,
    pub dup_rx_req: u64,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub rx_send_req: u64,
    pub rx_write_req: u64,
    pub rx_read_req: u64,
    pub tx_pkt: u64,
    pub rx_pkt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_query_device_cntrs_req {
    pub hdr: gdma_req_hdr,
    pub adapter: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rnic_query_device_cntrs_resp {
    pub hdr: gdma_resp_hdr,
    pub sent_cnps: u32,
    pub received_ecns: u32,
    pub reserved1: u32,
    pub received_cnp_count: u32,
    pub qp_congested_events: u32,
    pub qp_recovered_events: u32,
    pub rate_inc_events: u32,
    pub reserved2: u32,
}

// Remove subtype bits
extern "C" {
    pub fn mana_ib_install_cq_cb(mdev: *mut mana_ib_dev, cq: *mut mana_ib_cq) -> c_int;
}
extern "C" {
    pub fn mana_ib_remove_cq_cb(mdev: *mut mana_ib_dev, cq: *mut mana_ib_cq);
}
extern "C" {
    pub fn mana_ib_destroy_queue(mdev: *mut mana_ib_dev, queue: *mut mana_ib_queue);
}
extern "C" {
    pub fn mana_ib_destroy_wq(ibwq: *mut ib_wq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mana_ib_destroy_rwq_ind_table(ib_rwq_ind_tbl: *mut ib_rwq_ind_table) -> c_int;
}
extern "C" {
    pub fn mana_ib_dereg_mr(ibmr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mana_ib_destroy_qp(ibqp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mana_ib_destroy_cq(ibcq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mana_ib_alloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mana_ib_dealloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mana_ib_dealloc_ucontext(ibcontext: *mut ib_ucontext);
}
extern "C" {
    pub fn mana_ib_mmap(ibcontext: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn mana_ib_disassociate_ucontext(ibcontext: *mut ib_ucontext);
}
extern "C" {
    pub fn mana_ib_gd_query_adapter_caps(mdev: *mut mana_ib_dev) -> c_int;
}
extern "C" {
    pub fn mana_eth_query_adapter_caps(mdev: *mut mana_ib_dev) -> c_int;
}
extern "C" {
    pub fn mana_ib_create_eqs(mdev: *mut mana_ib_dev) -> c_int;
}
extern "C" {
    pub fn mana_ib_destroy_eqs(mdev: *mut mana_ib_dev);
}
extern "C" {
    pub fn mana_ib_gd_create_rnic_adapter(mdev: *mut mana_ib_dev) -> c_int;
}
extern "C" {
    pub fn mana_ib_gd_destroy_rnic_adapter(mdev: *mut mana_ib_dev) -> c_int;
}
extern "C" {
    pub fn mana_ib_query_pkey(ibdev: *mut ib_device, port: u32, index: u16, pkey: *mut u16) -> c_int;
}
extern "C" {
    pub fn mana_ib_get_link_layer(device: *mut ib_device, port_num: u32) -> rdma_link_layer;
}
extern "C" {
    pub fn mana_ib_gd_add_gid(attr: *const ib_gid_attr, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mana_ib_gd_del_gid(attr: *const ib_gid_attr, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mana_ib_gd_config_mac(mdev: *mut mana_ib_dev, op: mana_ib_addr_op, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn mana_ib_gd_create_cq(mdev: *mut mana_ib_dev, cq: *mut mana_ib_cq, doorbell: u32) -> c_int;
}
extern "C" {
    pub fn mana_ib_gd_destroy_cq(mdev: *mut mana_ib_dev, cq: *mut mana_ib_cq) -> c_int;
}
extern "C" {
    pub fn mana_ib_gd_destroy_rnic_qp(mdev: *mut mana_ib_dev, qp: *mut mana_ib_qp) -> c_int;
}
extern "C" {
    pub fn mana_ib_gd_destroy_ud_qp(mdev: *mut mana_ib_dev, qp: *mut mana_ib_qp) -> c_int;
}
extern "C" {
    pub fn mana_ib_destroy_ah(ah: *mut ib_ah, flags: u32) -> c_int;
}
extern "C" {
    pub fn mana_drain_gsi_sqs(mdev: *mut mana_ib_dev);
}
extern "C" {
    pub fn mana_ib_poll_cq(ibcq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn mana_ib_arm_cq(ibcq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn mana_ib_alloc_mw(mw: *mut ib_mw, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mana_ib_dealloc_mw(mw: *mut ib_mw) -> c_int;
}
extern "C" {
    pub fn mana_ib_dealloc_dm(dm: *mut ib_dm, attrs: *mut uverbs_attr_bundle) -> c_int;
}
