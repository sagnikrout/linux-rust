//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_rdma_if.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

// rdma interface
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_roce_qp_state {
    QED_ROCE_QP_STATE_RESET,
    QED_ROCE_QP_STATE_INIT,
    QED_ROCE_QP_STATE_RTR,
    QED_ROCE_QP_STATE_RTS,
    QED_ROCE_QP_STATE_SQD,
    QED_ROCE_QP_STATE_ERR,
    QED_ROCE_QP_STATE_SQE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_rdma_qp_type {
    QED_RDMA_QP_TYPE_RC,
    QED_RDMA_QP_TYPE_XRC_INI,
    QED_RDMA_QP_TYPE_XRC_TGT,
    QED_RDMA_QP_TYPE_INVAL = 0xffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_rdma_tid_type {
    QED_RDMA_TID_REGISTERED_MR,
    QED_RDMA_TID_FMR,
    QED_RDMA_TID_MW
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_events {
    pub context: *mut c_void,
    pub fw_handle): *mut c_void,
    pub event_code): *mut *mut *mut void (unaffiliated_event)(void context, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_device {
    pub vendor_id: u32,
    pub vendor_part_id: u32,
    pub hw_ver: u32,
    pub fw_ver: u64,
    pub node_guid: u64,
    pub sys_image_guid: u64,
    pub max_cnq: u8,
    pub max_sge: u8,
    pub max_srq_sge: u8,
    pub max_inline: u16,
    pub max_wqe: u32,
    pub max_srq_wqe: u32,
    pub max_qp_resp_rd_atomic_resc: u8,
    pub max_qp_req_rd_atomic_resc: u8,
    pub max_dev_resp_rd_atomic_resc: u64,
    pub max_cq: u32,
    pub max_qp: u32,
    pub max_srq: u32,
    pub max_mr: u32,
    pub max_mr_size: u64,
    pub max_cqe: u32,
    pub max_mw: u32,
    pub max_mr_mw_fmr_pbl: u32,
    pub max_mr_mw_fmr_size: u64,
    pub max_pd: u32,
    pub max_ah: u32,
    pub max_pkey: u8,
    pub max_srq_wr: u16,
    pub max_stats_queues: u8,
    pub dev_caps: u32,
// Abilty to support RNR-NAK generation
pub const QED_RDMA_DEV_CAP_RNR_NAK_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_RNR_NAK_SHIFT: c_int = 0;
// Abilty to support shutdown port
pub const QED_RDMA_DEV_CAP_SHUTDOWN_PORT_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_SHUTDOWN_PORT_SHIFT: c_int = 1;
// Abilty to support port active event
pub const QED_RDMA_DEV_CAP_PORT_ACTIVE_EVENT_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_PORT_ACTIVE_EVENT_SHIFT: c_int = 2;
// Abilty to support port change event
pub const QED_RDMA_DEV_CAP_PORT_CHANGE_EVENT_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_PORT_CHANGE_EVENT_SHIFT: c_int = 3;
// Abilty to support system image GUID
pub const QED_RDMA_DEV_CAP_SYS_IMAGE_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_SYS_IMAGE_SHIFT: c_int = 4;
// Abilty to support bad P_Key counter support
pub const QED_RDMA_DEV_CAP_BAD_PKEY_CNT_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_BAD_PKEY_CNT_SHIFT: c_int = 5;
// Abilty to support atomic operations
pub const QED_RDMA_DEV_CAP_ATOMIC_OP_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_ATOMIC_OP_SHIFT: c_int = 6;
pub const QED_RDMA_DEV_CAP_RESIZE_CQ_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_RESIZE_CQ_SHIFT: c_int = 7;
// Abilty to support modifying the maximum number of
// outstanding work requests per QP
//
pub const QED_RDMA_DEV_CAP_RESIZE_MAX_WR_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_RESIZE_MAX_WR_SHIFT: c_int = 8;
// Abilty to support automatic path migration
pub const QED_RDMA_DEV_CAP_AUTO_PATH_MIG_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_AUTO_PATH_MIG_SHIFT: c_int = 9;
// Abilty to support the base memory management extensions
pub const QED_RDMA_DEV_CAP_BASE_MEMORY_EXT_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_BASE_MEMORY_EXT_SHIFT: c_int = 10;
pub const QED_RDMA_DEV_CAP_BASE_QUEUE_EXT_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_BASE_QUEUE_EXT_SHIFT: c_int = 11;
// Abilty to support multipile page sizes per memory region
pub const QED_RDMA_DEV_CAP_MULTI_PAGE_PER_MR_EXT_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_MULTI_PAGE_PER_MR_EXT_SHIFT: c_int = 12;
// Abilty to support block list physical buffer list
pub const QED_RDMA_DEV_CAP_BLOCK_MODE_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_BLOCK_MODE_SHIFT: c_int = 13;
// Abilty to support zero based virtual addresses
pub const QED_RDMA_DEV_CAP_ZBVA_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_ZBVA_SHIFT: c_int = 14;
// Abilty to support local invalidate fencing
pub const QED_RDMA_DEV_CAP_LOCAL_INV_FENCE_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_LOCAL_INV_FENCE_SHIFT: c_int = 15;
// Abilty to support Loopback on QP
pub const QED_RDMA_DEV_CAP_LB_INDICATOR_MASK: c_uint = 0x1;
pub const QED_RDMA_DEV_CAP_LB_INDICATOR_SHIFT: c_int = 16;
    pub page_size_caps: u64,
    pub dev_ack_delay: u8,
    pub reserved_lkey: u32,
    pub bad_pkey_counter: u32,
    pub events: qed_rdma_events,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_port_state {
    QED_RDMA_PORT_UP,
    QED_RDMA_PORT_DOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_roce_capability {
    QED_ROCE_V1 = 1 << 0,
    QED_ROCE_V2 = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_port {
    pub port_state: qed_port_state,
    pub link_speed: c_int,
    pub max_msg_size: u64,
    pub source_gid_table_len: u8,
    pub source_gid_table_ptr: *mut c_void,
    pub pkey_table_len: u8,
    pub pkey_table_ptr: *mut c_void,
    pub pkey_bad_counter: u32,
    pub capability: qed_roce_capability,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_cnq_params {
    pub num_pbl_pages: u8,
    pub pbl_ptr: u64,
}

// The CQ Mode affects the CQ doorbell transaction size.
// 64/32 bit machines should configure to 32/16 bits respectively.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_rdma_cq_mode {
    QED_RDMA_CQ_MODE_16_BITS,
    QED_RDMA_CQ_MODE_32_BITS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_roce_dcqcn_params {
    pub notification_point: u8,
    pub reaction_point: u8,
// fields for notification point
    pub cnp_send_timeout: u32,
// fields for reaction point
    pub rl_bc_rate: u32,
    pub rl_max_rate: u16,
    pub rl_r_ai: u16,
    pub rl_r_hai: u16,
    pub dcqcn_g: u16,
    pub dcqcn_k_us: u32,
    pub dcqcn_timeout_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_start_in_params {
    pub events: *mut qed_rdma_events,
    pub cnq_pbl_list: [qed_rdma_cnq_params; 128],
    pub desired_cnq: u8,
    pub cq_mode: qed_rdma_cq_mode,
    pub dcqcn_params: qed_roce_dcqcn_params,
    pub max_mtu: u16,
    pub mac_addr: [u8; ETH_ALEN],
    pub iwarp_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_add_user_out_params {
    pub dpi: u16,
    pub dpi_addr: *mut void __iomem,
    pub dpi_phys_addr: u64,
    pub dpi_size: u32,
    pub wid_count: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roce_mode {
    ROCE_V1,
    ROCE_V2_IPV4,
    ROCE_V2_IPV6,
    MAX_ROCE_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union qed_gid {
    pub bytes: [u8; 16],
    pub words: [u16; 8],
    pub dwords: [u32; 4],
    pub qwords: [u64; 2],
    pub ipv4_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_register_tid_in_params {
    pub itid: u32,
    pub tid_type: qed_rdma_tid_type,
    pub key: u8,
    pub pd: u16,
    pub local_read: bool,
    pub local_write: bool,
    pub remote_read: bool,
    pub remote_write: bool,
    pub remote_atomic: bool,
    pub mw_bind: bool,
    pub pbl_ptr: u64,
    pub pbl_two_level: bool,
    pub pbl_page_size_log: u8,
    pub page_size_log: u8,
    pub length: u64,
    pub vaddr: u64,
    pub phy_mr: bool,
    pub dma_mr: bool,
    pub dif_enabled: bool,
    pub dif_error_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_create_cq_in_params {
    pub cq_handle_lo: u32,
    pub cq_handle_hi: u32,
    pub cq_size: u32,
    pub dpi: u16,
    pub pbl_two_level: bool,
    pub pbl_ptr: u64,
    pub pbl_num_pages: u16,
    pub pbl_page_size_log: u8,
    pub cnq_id: u8,
    pub int_timeout: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_create_srq_in_params {
    pub pbl_base_addr: u64,
    pub prod_pair_addr: u64,
    pub num_pages: u16,
    pub pd_id: u16,
    pub page_size: u16,
// XRC related only
    pub reserved_key_en: bool,
    pub is_xrc: bool,
    pub cq_cid: u32,
    pub xrcd_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_destroy_cq_in_params {
    pub icid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_destroy_cq_out_params {
    pub num_cq_notif: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_create_qp_in_params {
    pub qp_handle_lo: u32,
    pub qp_handle_hi: u32,
    pub qp_handle_async_lo: u32,
    pub qp_handle_async_hi: u32,
    pub use_srq: bool,
    pub signal_all: bool,
    pub fmr_and_reserved_lkey: bool,
    pub pd: u16,
    pub dpi: u16,
    pub sq_cq_id: u16,
    pub sq_num_pages: u16,
    pub sq_pbl_ptr: u64,
    pub max_sq_sges: u8,
    pub rq_cq_id: u16,
    pub rq_num_pages: u16,
    pub rq_pbl_ptr: u64,
    pub srq_id: u16,
    pub xrcd_id: u16,
    pub stats_queue: u8,
    pub qp_type: qed_rdma_qp_type,
    pub flags: u8,
pub const QED_ROCE_EDPM_MODE_MASK: c_uint = 0x1;
pub const QED_ROCE_EDPM_MODE_SHIFT: c_int = 0;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_create_qp_out_params {
    pub qp_id: u32,
    pub icid: u16,
    pub rq_pbl_virt: *mut c_void,
    pub rq_pbl_phys: dma_addr_t,
    pub sq_pbl_virt: *mut c_void,
    pub sq_pbl_phys: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_modify_qp_in_params {
    pub modify_flags: u32,
pub const QED_RDMA_MODIFY_QP_VALID_NEW_STATE_MASK: c_uint = 0x1;
pub const QED_RDMA_MODIFY_QP_VALID_NEW_STATE_SHIFT: c_int = 0;
pub const QED_ROCE_MODIFY_QP_VALID_PKEY_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_PKEY_SHIFT: c_int = 1;
pub const QED_RDMA_MODIFY_QP_VALID_RDMA_OPS_EN_MASK: c_uint = 0x1;
pub const QED_RDMA_MODIFY_QP_VALID_RDMA_OPS_EN_SHIFT: c_int = 2;
pub const QED_ROCE_MODIFY_QP_VALID_DEST_QP_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_DEST_QP_SHIFT: c_int = 3;
pub const QED_ROCE_MODIFY_QP_VALID_ADDRESS_VECTOR_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_ADDRESS_VECTOR_SHIFT: c_int = 4;
pub const QED_ROCE_MODIFY_QP_VALID_RQ_PSN_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_RQ_PSN_SHIFT: c_int = 5;
pub const QED_ROCE_MODIFY_QP_VALID_SQ_PSN_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_SQ_PSN_SHIFT: c_int = 6;
pub const QED_RDMA_MODIFY_QP_VALID_MAX_RD_ATOMIC_REQ_MASK: c_uint = 0x1;
pub const QED_RDMA_MODIFY_QP_VALID_MAX_RD_ATOMIC_REQ_SHIFT: c_int = 7;
pub const QED_RDMA_MODIFY_QP_VALID_MAX_RD_ATOMIC_RESP_MASK: c_uint = 0x1;
pub const QED_RDMA_MODIFY_QP_VALID_MAX_RD_ATOMIC_RESP_SHIFT: c_int = 8;
pub const QED_ROCE_MODIFY_QP_VALID_ACK_TIMEOUT_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_ACK_TIMEOUT_SHIFT: c_int = 9;
pub const QED_ROCE_MODIFY_QP_VALID_RETRY_CNT_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_RETRY_CNT_SHIFT: c_int = 10;
pub const QED_ROCE_MODIFY_QP_VALID_RNR_RETRY_CNT_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_RNR_RETRY_CNT_SHIFT: c_int = 11;
pub const QED_ROCE_MODIFY_QP_VALID_MIN_RNR_NAK_TIMER_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_MIN_RNR_NAK_TIMER_SHIFT: c_int = 12;
pub const QED_ROCE_MODIFY_QP_VALID_E2E_FLOW_CONTROL_EN_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_E2E_FLOW_CONTROL_EN_SHIFT: c_int = 13;
pub const QED_ROCE_MODIFY_QP_VALID_ROCE_MODE_MASK: c_uint = 0x1;
pub const QED_ROCE_MODIFY_QP_VALID_ROCE_MODE_SHIFT: c_int = 14;
    pub new_state: qed_roce_qp_state,
    pub pkey: u16,
    pub incoming_rdma_read_en: bool,
    pub incoming_rdma_write_en: bool,
    pub incoming_atomic_en: bool,
    pub e2e_flow_control_en: bool,
    pub dest_qp: u32,
    pub lb_indication: bool,
    pub mtu: u16,
    pub traffic_class_tos: u8,
    pub hop_limit_ttl: u8,
    pub flow_label: u32,
    pub sgid: qed_gid,
    pub dgid: qed_gid,
    pub udp_src_port: u16,
    pub vlan_id: u16,
    pub rq_psn: u32,
    pub sq_psn: u32,
    pub max_rd_atomic_resp: u8,
    pub max_rd_atomic_req: u8,
    pub ack_timeout: u32,
    pub retry_cnt: u8,
    pub rnr_retry_cnt: u8,
    pub min_rnr_nak_timer: u8,
    pub sqd_async: bool,
    pub remote_mac_addr: [u8; 6],
    pub local_mac_addr: [u8; 6],
    pub use_local_mac: bool,
    pub roce_mode: roce_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_query_qp_out_params {
    pub state: qed_roce_qp_state,
    pub rq_psn: u32,
    pub sq_psn: u32,
    pub draining: bool,
    pub mtu: u16,
    pub dest_qp: u32,
    pub incoming_rdma_read_en: bool,
    pub incoming_rdma_write_en: bool,
    pub incoming_atomic_en: bool,
    pub e2e_flow_control_en: bool,
    pub sgid: qed_gid,
    pub dgid: qed_gid,
    pub flow_label: u32,
    pub hop_limit_ttl: u8,
    pub traffic_class_tos: u8,
    pub timeout: u32,
    pub rnr_retry: u8,
    pub retry_cnt: u8,
    pub min_rnr_nak_timer: u8,
    pub pkey_index: u16,
    pub max_rd_atomic: u8,
    pub max_dest_rd_atomic: u8,
    pub sqd_async: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_create_srq_out_params {
    pub srq_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_destroy_srq_in_params {
    pub srq_id: u16,
    pub is_xrc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_modify_srq_in_params {
    pub wqe_limit: u32,
    pub srq_id: u16,
    pub is_xrc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_stats_out_params {
    pub sent_bytes: u64,
    pub sent_pkts: u64,
    pub rcv_bytes: u64,
    pub rcv_pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_counters_out_params {
    pub pd_count: u64,
    pub max_pd: u64,
    pub dpi_count: u64,
    pub max_dpi: u64,
    pub cq_count: u64,
    pub max_cq: u64,
    pub qp_count: u64,
    pub max_qp: u64,
    pub tid_count: u64,
    pub max_tid: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_iwarp_event_type {
    QED_IWARP_EVENT_MPA_REQUEST,	  /* Passive side request received */
    QED_IWARP_EVENT_PASSIVE_COMPLETE, /* ack on mpa response */
    QED_IWARP_EVENT_ACTIVE_COMPLETE,  /* Active side reply received */
    QED_IWARP_EVENT_DISCONNECT,
    QED_IWARP_EVENT_CLOSE,
    QED_IWARP_EVENT_IRQ_FULL,
    QED_IWARP_EVENT_RQ_EMPTY,
    QED_IWARP_EVENT_LLP_TIMEOUT,
    QED_IWARP_EVENT_REMOTE_PROTECTION_ERROR,
    QED_IWARP_EVENT_CQ_OVERFLOW,
    QED_IWARP_EVENT_QP_CATASTROPHIC,
    QED_IWARP_EVENT_ACTIVE_MPA_REPLY,
    QED_IWARP_EVENT_LOCAL_ACCESS_ERROR,
    QED_IWARP_EVENT_REMOTE_OPERATION_ERROR,
    QED_IWARP_EVENT_TERMINATE_RECEIVED,
    QED_IWARP_EVENT_SRQ_LIMIT,
    QED_IWARP_EVENT_SRQ_EMPTY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_tcp_ip_version {
    QED_TCP_IPV4,
    QED_TCP_IPV6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_cm_info {
    pub ip_version: qed_tcp_ip_version,
    pub remote_ip: [u32; 4],
    pub local_ip: [u32; 4],
    pub remote_port: u16,
    pub local_port: u16,
    pub vlan: u16,
    pub ord: u8,
    pub ird: u8,
    pub private_data_len: u16,
    pub private_data: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_cm_event_params {
    pub event: qed_iwarp_event_type,
    pub cm_info: *const qed_iwarp_cm_info,
    pub /: *mut *mut *mut void ep_context; / To be passed to accept call,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_connect_in {
    pub event_cb: iwarp_event_handler,
    pub cb_context: *mut c_void,
    pub qp: *mut qed_rdma_qp,
    pub cm_info: qed_iwarp_cm_info,
    pub mss: u16,
    pub remote_mac_addr: [u8; ETH_ALEN],
    pub local_mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_connect_out {
    pub ep_context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_listen_in {
    pub event_cb: iwarp_event_handler,
    pub /: *mut *mut *mut void cb_context; / passed to event_cb,
    pub max_backlog: u32,
    pub ip_version: qed_tcp_ip_version,
    pub ip_addr: [u32; 4],
    pub port: u16,
    pub vlan: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_listen_out {
    pub handle: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_accept_in {
    pub ep_context: *mut c_void,
    pub cb_context: *mut c_void,
    pub qp: *mut qed_rdma_qp,
    pub private_data: *const c_void,
    pub private_data_len: u16,
    pub ord: u8,
    pub ird: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_reject_in {
    pub ep_context: *mut c_void,
    pub cb_context: *mut c_void,
    pub private_data: *const c_void,
    pub private_data_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iwarp_send_rtr_in {
    pub ep_context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_roce_ll2_header {
    pub vaddr: *mut c_void,
    pub baddr: dma_addr_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_roce_ll2_buffer {
    pub baddr: dma_addr_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_roce_ll2_packet {
    pub header: qed_roce_ll2_header,
    pub n_seg: c_int,
    pub payload: [qed_roce_ll2_buffer; RDMA_MAX_SGE_PER_SQ_WQE],
    pub roce_mode: c_int,
    pub tx_dest: qed_ll2_tx_dest,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_rdma_type {
    QED_RDMA_TYPE_ROCE,
    QED_RDMA_TYPE_IWARP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dev_rdma_info {
    pub common: qed_dev_info,
    pub rdma_type: qed_rdma_type,
    pub user_dpm_enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rdma_ops {
    pub common: *const qed_common_ops,
    pub info): *mut qed_dev_rdma_info,
    pub cdev): *mut *mut *mut void (rdma_get_rdma_ctx)(struct qed_dev,
    pub iparams): *mut qed_rdma_start_in_params,
    pub oparams): *mut qed_rdma_add_user_out_params,
    pub dpi): *mut *mut *mut void (rdma_remove_user)(void rdma_cxt, u16,
    pub rdma_cxt): *mut *mut int (rdma_stop)(void,
    pub rdma_cxt): *mut *mut *mut qed_rdma_device (rdma_query_device)(void,
    pub rdma_cxt): *mut *mut *mut qed_rdma_port (rdma_query_port)(void,
    pub cdev): *mut *mut int (rdma_get_start_sb)(struct qed_dev,
    pub cdev): *mut *mut int (rdma_get_min_cnq_msix)(struct qed_dev,
    pub prod): *mut *mut *mut void (rdma_cnq_prod_update)(void rdma_cxt, u8 cnq_index, u16,
    pub info): *mut qed_int_info,
    pub cnt): *mut *mut *mut int (rdma_set_rdma_int)(struct qed_dev cdev, u16,
    pub pd): *mut *mut *mut int (rdma_alloc_pd)(void rdma_cxt, u16,
    pub pd): *mut *mut *mut void (rdma_dealloc_pd)(void rdma_cxt, u16,
    pub xrcd): *mut *mut *mut int (rdma_alloc_xrcd)(void rdma_cxt, u16,
    pub xrcd): *mut *mut *mut void (rdma_dealloc_xrcd)(void rdma_cxt, u16,
    pub icid): *mut u16,
    pub oparams): *mut qed_rdma_destroy_cq_out_params,
    pub oparams): *mut qed_rdma_create_qp_out_params,
    pub iparams): *mut qed_rdma_modify_qp_in_params,
    pub oparams): *mut qed_rdma_query_qp_out_params,
    pub qp): *mut *mut *mut int (rdma_destroy_qp)(void rdma_cxt, struct qed_rdma_qp,
    pub iparams): *mut qed_rdma_register_tid_in_params,
    pub itid): *mut *mut *mut int (rdma_deregister_tid)(void rdma_cxt, u32,
    pub itid): *mut *mut *mut int (rdma_alloc_tid)(void rdma_cxt, u32,
    pub itid): *mut *mut *mut void (rdma_free_tid)(void rdma_cxt, u32,
    pub oparams): *mut qed_rdma_create_srq_out_params,
    pub iparams): *mut qed_rdma_destroy_srq_in_params,
    pub iparams): *mut qed_rdma_modify_srq_in_params,
    pub data): *mut qed_ll2_acquire_data,
    pub connection_handle): *mut *mut *mut int (ll2_establish_connection)(void rdma_cxt, u8,
    pub connection_handle): *mut *mut *mut int (ll2_terminate_connection)(void rdma_cxt, u8,
    pub connection_handle): *mut *mut *mut void (ll2_release_connection)(void rdma_cxt, u8,
    pub notify_fw): bool,
    pub nbytes): u16,
    pub notify_fw): u8,
    pub p_stats): *mut qed_ll2_stats,
    pub new_mac_address): *const u8,
    pub b_reset): *mut *mut *mut int (iwarp_set_engine_affin)(struct qed_dev cdev, bool,
    pub oparams): *mut qed_iwarp_connect_out,
    pub oparams): *mut qed_iwarp_listen_out,
    pub iparams): *mut qed_iwarp_accept_in,
    pub iparams): *mut qed_iwarp_reject_in,
    pub handle): *mut *mut *mut int (iwarp_destroy_listen)(void rdma_cxt, void,
    pub iparams): *mut qed_iwarp_send_rtr_in,
}
