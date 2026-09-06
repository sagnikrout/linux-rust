//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/mlx5_ib.h
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
// Copyright (c) 2013-2020, Mellanox Technologies inc. All rights reserved.
// Copyright (c) 2020, Intel Corporation. All rights reserved.
//

pub const MLX5_IB_DEFAULT_UIDX: c_uint = 0xffffff;

//
// Despite a command allowing it, the device does not support lower than
// 4k page size.
//
extern "C" {
    pub fn GENMASK(_arg: largest_pg_shift, _arg: pgsz_shift) -> return;
}
extern "C" {
    pub fn GENMASK(_arg: largest_offset_shift, _arg: offset_shift) -> return;
}
//
// QP/CQ/WQ/etc type commands take a page offset that satisifies:
// page_offset_quantized * (page_size/scale) = page_offset
// Which restricts allowed page sizes to ones that satisify the above.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_mad_ifc_flags {
    MLX5_MAD_IFC_IGNORE_MKEY	= 1,
    MLX5_MAD_IFC_IGNORE_BKEY	= 2,
    MLX5_MAD_IFC_NET_VIEW		= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_mmap_type {
    MLX5_IB_MMAP_TYPE_MEMIC = 1,
    MLX5_IB_MMAP_TYPE_VAR = 2,
    MLX5_IB_MMAP_TYPE_UAR_WC = 3,
    MLX5_IB_MMAP_TYPE_UAR_NC = 4,
    MLX5_IB_MMAP_TYPE_MEMIC_OP = 5,
    MLX5_IB_MMAP_TYPE_TLP_VAR = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bfreg_info {
    pub sys_pages: *mut u32,
    pub num_low_latency_bfregs: c_int,
    pub count: *mut c_uint,
//
// protect bfreg allocation data structs
//
    pub lock: mutex,
    pub ver: u32,
    pub 1: u8 lib_uar_4k :,
    pub 1: u8 lib_uar_dyn :,
    pub num_sys_pages: u32,
    pub num_static_sys_pages: u32,
    pub total_num_bfregs: u32,
    pub num_dyn_bfregs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_ucontext {
    pub ibucontext: ib_ucontext,
    pub db_page_list: list_head,
// protect doorbell record alloc/free
//
    pub db_page_mutex: mutex,
    pub bfregi: mlx5_bfreg_info,
    pub cqe_version: u8,
// Transport Domain number
    pub tdn: u32,
    pub lib_caps: u64,
    pub devx_uid: u16,
// For RoCE LAG TX affinity
    pub tx_port_affinity: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn container_of(_arg: ibucontext, mlx5_ib_ucontext: struct, _arg: ibucontext) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_pd {
    pub ibpd: ib_pd,
    pub pdn: u32,
    pub uid: u16,
}

pub const MLX5_IB_NUM_SNIFFER_FTS: c_int = 2;
pub const MLX5_IB_NUM_EGRESS_FTS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_anchor {
    pub ft: *mut mlx5_flow_table,
    pub fg_goto_table: *mut mlx5_flow_group,
    pub fg_drop: *mut mlx5_flow_group,
    pub rule_goto_table: *mut mlx5_flow_handle,
    pub rule_drop: *mut mlx5_flow_handle,
    pub rule_goto_table_ref: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_flow_prio {
    pub flow_table: *mut mlx5_flow_table,
    pub anchor: mlx5_ib_anchor,
    pub refcount: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_flow_handler {
    pub list: list_head,
    pub ibflow: ib_flow,
    pub prio: *mut mlx5_ib_flow_prio,
    pub rule: *mut mlx5_flow_handle,
    pub ibcounters: *mut ib_counters,
    pub dev: *mut mlx5_ib_dev,
    pub flow_matcher: *mut mlx5_ib_flow_matcher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_flow_matcher {
    pub matcher_mask: mlx5_ib_match_params,
    pub mask_len: c_int,
    pub flow_type: mlx5_ib_flow_type,
    pub ns_type: mlx5_flow_namespace_type,
    pub priority: u16,
    pub mdev: *mut mlx5_core_dev,
    pub usecnt: core::sync::atomic::AtomicI32,
    pub match_criteria_enable: u8,
    pub ib_port: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_steering_anchor {
    pub ft_prio: *mut mlx5_ib_flow_prio,
    pub dev: *mut mlx5_ib_dev,
    pub usecnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_pp {
    pub index: u16,
    pub mdev: *mut mlx5_core_dev,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_optional_counter_type {
    MLX5_IB_OPCOUNTER_CC_RX_CE_PKTS,
    MLX5_IB_OPCOUNTER_CC_RX_CNP_PKTS,
    MLX5_IB_OPCOUNTER_CC_TX_CNP_PKTS,
    MLX5_IB_OPCOUNTER_RDMA_TX_PACKETS,
    MLX5_IB_OPCOUNTER_RDMA_TX_BYTES,
    MLX5_IB_OPCOUNTER_RDMA_RX_PACKETS,
    MLX5_IB_OPCOUNTER_RDMA_RX_BYTES,

    MLX5_IB_OPCOUNTER_CC_RX_CE_PKTS_PER_QP,
    MLX5_IB_OPCOUNTER_CC_RX_CNP_PKTS_PER_QP,
    MLX5_IB_OPCOUNTER_CC_TX_CNP_PKTS_PER_QP,
    MLX5_IB_OPCOUNTER_RDMA_TX_PACKETS_PER_QP,
    MLX5_IB_OPCOUNTER_RDMA_TX_BYTES_PER_QP,
    MLX5_IB_OPCOUNTER_RDMA_RX_PACKETS_PER_QP,
    MLX5_IB_OPCOUNTER_RDMA_RX_BYTES_PER_QP,

    MLX5_IB_OPCOUNTER_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_flow_db {
    pub prios: [mlx5_ib_flow_prio; MLX5_IB_NUM_FLOW_FT],
    pub egress_prios: [mlx5_ib_flow_prio; MLX5_IB_NUM_FLOW_FT],
    pub sniffer: [mlx5_ib_flow_prio; MLX5_IB_NUM_SNIFFER_FTS],
    pub egress: [mlx5_ib_flow_prio; MLX5_IB_NUM_EGRESS_FTS],
    pub fdb: [mlx5_ib_flow_prio; MLX5_IB_NUM_FDB_FTS],
    pub rdma_rx: [mlx5_ib_flow_prio; MLX5_IB_NUM_FLOW_FT],
    pub rdma_tx: [mlx5_ib_flow_prio; MLX5_IB_NUM_FLOW_FT],
    pub opfcs: [mlx5_ib_flow_prio; MLX5_IB_OPCOUNTER_MAX],
    pub rdma_transport_rx: [*mut mlx5_ib_flow_prio; MLX5_RDMA_TRANSPORT_BYPASS_PRIO],
    pub rdma_transport_tx: [*mut mlx5_ib_flow_prio; MLX5_RDMA_TRANSPORT_BYPASS_PRIO],
// Protect flow steering bypass flow tables
// when add/del flow rules.
// only single add/removal of flow steering rule could be done
// simultaneously.
//
    pub lock: mutex,
}

// Use macros here so that don't have to duplicate
// enum ib_qp_type for low-level driver
//

//
// IB_QPT_GSI creates the software wrapper around GSI, and MLX5_IB_QPT_HW_GSI
// creates the actual hardware QP.
//

//
// A valid pdn is required when flags include MLX5_IB_UPD_XLT_ENABLE,
// MLX5_IB_UPD_XLT_PD or MLX5_IB_UPD_XLT_ACCESS.
//

// Private QP creation flags to be passed in ib_qp_init_attr.create_flags.
//
// These flags are intended for internal use by the mlx5_ib driver, and they
// rely on the range reserved for that use in the ib_qp_create_flags enum.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wr_list {
    pub opcode: u16,
    pub next: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_rq_flags {
    MLX5_IB_RQ_CVLAN_STRIPPING	= 1 << 0,
    MLX5_IB_RQ_PCI_WRITE_END_PADDING	= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_wq {
    pub fbc: mlx5_frag_buf_ctrl,
    pub wrid: *mut u64,
    pub wr_data: *mut u32,
    pub w_list: *mut wr_list,
    pub wqe_head: *mut unsigned,
    pub unsig_count: u16,
// serialize post to the work queue
//
    pub lock: spinlock_t,
    pub wqe_cnt: c_int,
    pub max_post: c_int,
    pub max_gs: c_int,
    pub offset: c_int,
    pub wqe_shift: c_int,
    pub head: unsigned,
    pub tail: unsigned,
    pub cur_post: u16,
    pub last_poll: u16,
    pub cur_edge: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_wq_flags {
    MLX5_IB_WQ_FLAGS_DELAY_DROP = 0x1,
    MLX5_IB_WQ_FLAGS_STRIDING_RQ = 0x2,
}

pub const MLX5_MIN_SINGLE_WQE_LOG_NUM_STRIDES: c_int = 9;
pub const MLX5_MAX_SINGLE_WQE_LOG_NUM_STRIDES: c_int = 16;
pub const MLX5_MIN_SINGLE_STRIDE_LOG_NUM_BYTES: c_int = 6;
pub const MLX5_MAX_SINGLE_STRIDE_LOG_NUM_BYTES: c_int = 13;
pub const MLX5_EXT_MIN_SINGLE_WQE_LOG_NUM_STRIDES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_rwq {
    pub ibwq: ib_wq,
    pub core_qp: mlx5_core_qp,
    pub rq_num_pas: u32,
    pub log_rq_stride: u32,
    pub log_rq_size: u32,
    pub rq_page_offset: u32,
    pub log_page_size: u32,
    pub log_num_strides: u32,
    pub two_byte_shift_en: u32,
    pub single_stride_log_num_of_bytes: u32,
    pub umem: *mut ib_umem,
    pub buf_size: usize,
    pub page_shift: c_uint,
    pub db: mlx5_db,
    pub user_index: u32,
    pub wqe_count: u32,
    pub wqe_shift: u32,
    pub wq_sig: c_int,
    pub /: *mut *mut u32 create_flags; / Use enum mlx5_ib_wq_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_rwq_ind_table {
    pub ib_rwq_ind_tbl: ib_rwq_ind_table,
    pub rqtn: u32,
    pub uid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_ubuffer {
    pub umem: *mut ib_umem,
    pub buf_size: c_int,
    pub buf_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_qp_base {
    pub container_mibqp: *mut mlx5_ib_qp,
    pub mqp: mlx5_core_qp,
    pub ubuffer: mlx5_ib_ubuffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_qp_trans {
    pub base: mlx5_ib_qp_base,
    pub xrcdn: u16,
    pub alt_port: u32,
    pub atomic_rd_en: u8,
    pub resp_depth: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_rss_qp {
    pub tirn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_rq {
    pub base: mlx5_ib_qp_base,
    pub rq: *mut mlx5_ib_wq,
    pub ubuffer: mlx5_ib_ubuffer,
    pub doorbell: *mut mlx5_db,
    pub tirn: u32,
    pub state: u8,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_sq {
    pub base: mlx5_ib_qp_base,
    pub sq: *mut mlx5_ib_wq,
    pub ubuffer: mlx5_ib_ubuffer,
    pub doorbell: *mut mlx5_db,
    pub flow_rule: *mut mlx5_flow_handle,
    pub tisn: u32,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_raw_packet_qp {
    pub sq: mlx5_ib_sq,
    pub rq: mlx5_ib_rq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_bf {
    pub buf_size: c_int,
    pub offset: c_ulong,
    pub bfreg: *mut mlx5_sq_bfreg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dct {
    pub mdct: mlx5_core_dct,
    pub in: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_gsi_qp {
    pub rx_qp: *mut ib_qp,
    pub port_num: u32,
    pub cap: ib_qp_cap,
    pub cq: *mut ib_cq,
    pub outstanding_wrs: *mut mlx5_ib_gsi_wr,
    pub outstanding_ci: u32 outstanding_pi,,
    pub num_qps: c_int,
// Protects access to the tx_qps. Post send operations synchronize
// with tx_qp creation in setup_qp(). Also protects the
// outstanding_wrs array and indices.
//
    pub lock: spinlock_t,
    pub tx_qps: *mut ib_qp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_qp {
    pub ibqp: ib_qp,
    pub trans_qp: mlx5_ib_qp_trans,
    pub raw_packet_qp: mlx5_ib_raw_packet_qp,
    pub rss_qp: mlx5_ib_rss_qp,
    pub dct: mlx5_ib_dct,
    pub gsi: mlx5_ib_gsi_qp,
}

// serialize qp state modifications
//
// cached variant of create_flags from struct ib_qp_init_attr
// only for user space QPs. For kernel
// we have it from the bf object
//
// IB/core doesn't store low-level QP types, so
// store both MLX and IBTA types in the field below.
//
// A flag to indicate if there's a new counter is configured
// but not take effective
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_cq_buf {
    pub fbc: mlx5_frag_buf_ctrl,
    pub frag_buf: mlx5_frag_buf,
    pub umem: *mut ib_umem,
    pub cqe_size: c_int,
    pub nent: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_cq_pr_flags {
    MLX5_IB_CQ_PR_FLAGS_CQE_128_PAD	= 1 << 0,
    MLX5_IB_CQ_PR_FLAGS_REAL_TIME_TS = 1 << 1,
    MLX5_IB_CQ_PR_TIMESTAMP_COMPLETION = 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_cq {
    pub ibcq: ib_cq,
    pub mcq: mlx5_core_cq,
    pub buf: mlx5_ib_cq_buf,
    pub db: mlx5_db,
// serialize access to the CQ
//
    pub lock: spinlock_t,
// protect resize cq
//
    pub resize_mutex: mutex,
    pub resize_buf: *mut mlx5_ib_cq_buf,
    pub resize_umem: *mut ib_umem,
    pub cqe_size: c_int,
    pub list_send_qp: list_head,
    pub list_recv_qp: list_head,
    pub wc_list: list_head,
    pub notify_flags: ib_cq_notify_flags,
    pub notify_work: work_struct,
    pub /: *mut *mut u16 private_flags; / Use mlx5_ib_cq_pr_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_wc {
    pub wc: ib_wc,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_srq {
    pub ibsrq: ib_srq,
    pub msrq: mlx5_core_srq,
    pub buf: mlx5_frag_buf,
    pub db: mlx5_db,
    pub fbc: mlx5_frag_buf_ctrl,
    pub wrid: *mut u64,
// protect SRQ hanlding
//
    pub lock: spinlock_t,
    pub head: c_int,
    pub tail: c_int,
    pub wqe_ctr: u16,
    pub umem: *mut ib_umem,
// serialize arming a SRQ
//
    pub mutex: mutex,
    pub wq_sig: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_xrcd {
    pub ibxrcd: ib_xrcd,
    pub xrcdn: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_mtt_access_flags {
    MLX5_IB_MTT_READ  = (1 << 0),
    MLX5_IB_MTT_WRITE = (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_user_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub mmap_flag: u8,
    pub address: u64,
    pub page_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_mkey_type {
    MLX5_MKEY_MR = 1,
    MLX5_MKEY_MW,
    MLX5_MKEY_INDIRECT_DEVX,
    MLX5_MKEY_NULL,
    MLX5_MKEY_IMPLICIT_CHILD,
}

// Used for non-existent ph value
pub const MLX5_IB_NO_PH: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_mkey {
    pub key: u32,
    pub type: mlx5_mkey_type,
    pub ndescs: c_uint,
    pub wait: wait_queue_head,
    pub usecount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_mr {
    pub ibmr: ib_mr,
    pub mmkey: mlx5_ib_mkey,
    pub umem: *mut ib_umem,
// The mr is data direct related
    pub :1: u8 data_direct,
// Used only by kernel MRs (umem == NULL)
    pub descs: *mut c_void,
    pub descs_alloc: *mut c_void,
    pub desc_map: dma_addr_t,
    pub max_descs: c_int,
    pub desc_size: c_int,
    pub access_mode: c_int,
// For Kernel IB_MR_TYPE_INTEGRITY
    pub sig: *mut mlx5_core_sig_ctx,
    pub pi_mr: *mut mlx5_ib_mr,
    pub klm_mr: *mut mlx5_ib_mr,
    pub mtt_mr: *mut mlx5_ib_mr,
    pub data_iova: u64,
    pub pi_iova: u64,
    pub meta_ndescs: c_int,
    pub meta_length: c_int,
    pub data_length: c_int,
}

// Used only by User MRs (umem != NULL)
// Current access_flags
// For User ODP
// The affilated data direct crossed mr
// Indicates previous dmabuf page fault occurred
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_mw {
    pub ibmw: ib_mw,
    pub mmkey: mlx5_ib_mkey,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_umr_context {
    pub cqe: ib_cqe,
    pub status: ib_wc_status,
    pub done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umr_common {
    pub pd: *mut ib_pd,
    pub cq: *mut ib_cq,
    pub qp: *mut ib_qp,
// Protects from UMR QP overflow
//
    pub sem: semaphore,
// Protects from using UMR while the UMR is not active
//
    pub lock: mutex,
    pub state: c_uint,
// Protects from repeat UMR QP creation
    pub init_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_port_resources {
    pub gsi: *mut mlx5_ib_gsi_qp,
    pub pkey_change_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_data_direct_resources {
    pub pdn: u32,
    pub mkey: u32,
    pub mkey_ro: u32,
    pub :1: u8 mkey_ro_valid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_resources {
    pub c0: *mut ib_cq,
    pub cq_lock: mutex,
    pub xrcdn0: u32,
    pub xrcdn1: u32,
    pub p0: *mut ib_pd,
    pub s0: *mut ib_srq,
    pub s1: *mut ib_srq,
    pub srq_lock: mutex,
    pub ports: [mlx5_ib_port_resources; 2],
}

pub const MAX_OPFC_RULES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_op_fc {
    pub fc: *mut mlx5_fc,
    pub rule: [*mut mlx5_flow_handle; MAX_OPFC_RULES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_counters {
    pub descs: *mut rdma_stat_desc,
    pub offsets: *mut usize,
    pub num_q_counters: u32,
    pub num_cong_counters: u32,
    pub num_ext_ppcnt_counters: u32,
    pub num_op_counters: u32,
    pub set_id: u16,
    pub opfcs: [mlx5_ib_op_fc; MLX5_IB_OPCOUNTER_MAX],
}

extern "C" {
    pub fn mlx5r_fs_unbind_op_fc(qp: *mut ib_qp, qpn_opfc_xa: *mut xarray);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_multiport {
    pub mpi: *mut mlx5_ib_multiport_info,
// To be held when accessing the multiport info
    pub mpi_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_roce {
// Protect mlx5_ib_get_netdev from invoking dev_hold() with a NULL
// netdev pointer
//
    pub nb: notifier_block,
    pub nn: netdev_net_notifier,
    pub mdev_nb: notifier_block,
    pub tracking_netdev: *mut net_device,
    pub tx_port_affinity: core::sync::atomic::AtomicI32,
    pub last_port_state: ib_port_state,
    pub dev: *mut mlx5_ib_dev,
    pub native_port_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_port {
    pub cnts: mlx5_ib_counters,
    pub mp: mlx5_ib_multiport,
    pub dbg_cc_params: *mut mlx5_ib_dbg_cc_params,
    pub roce: mlx5_roce,
    pub rep: *mut mlx5_eswitch_rep,

    pub reserved_gids: *mut mlx5_reserved_gids,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dbg_param {
    pub offset: c_int,
    pub dev: *mut mlx5_ib_dev,
    pub dentry: *mut dentry,
    pub port_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_dbg_cc_types {
    MLX5_IB_DBG_CC_RP_CLAMP_TGT_RATE,
    MLX5_IB_DBG_CC_RP_CLAMP_TGT_RATE_ATI,
    MLX5_IB_DBG_CC_RP_TIME_RESET,
    MLX5_IB_DBG_CC_RP_BYTE_RESET,
    MLX5_IB_DBG_CC_RP_THRESHOLD,
    MLX5_IB_DBG_CC_RP_AI_RATE,
    MLX5_IB_DBG_CC_RP_MAX_RATE,
    MLX5_IB_DBG_CC_RP_HAI_RATE,
    MLX5_IB_DBG_CC_RP_MIN_DEC_FAC,
    MLX5_IB_DBG_CC_RP_MIN_RATE,
    MLX5_IB_DBG_CC_RP_RATE_TO_SET_ON_FIRST_CNP,
    MLX5_IB_DBG_CC_RP_DCE_TCP_G,
    MLX5_IB_DBG_CC_RP_DCE_TCP_RTT,
    MLX5_IB_DBG_CC_RP_RATE_REDUCE_MONITOR_PERIOD,
    MLX5_IB_DBG_CC_RP_INITIAL_ALPHA_VALUE,
    MLX5_IB_DBG_CC_RP_GD,
    MLX5_IB_DBG_CC_NP_MIN_TIME_BETWEEN_CNPS,
    MLX5_IB_DBG_CC_NP_CNP_DSCP,
    MLX5_IB_DBG_CC_NP_CNP_PRIO_MODE,
    MLX5_IB_DBG_CC_NP_CNP_PRIO,
    MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP_VALID,
    MLX5_IB_DBG_CC_GENERAL_RTT_RESP_DSCP,
    MLX5_IB_DBG_CC_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dbg_cc_params {
    pub root: *mut dentry,
    pub params: [mlx5_ib_dbg_param; MLX5_IB_DBG_CC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_delay_drop {
    pub dev: *mut mlx5_ib_dev,
    pub delay_drop_work: work_struct,
// serialize setting of delay drop
    pub lock: mutex,
    pub timeout: u32,
    pub activate: bool,
    pub events_cnt: core::sync::atomic::AtomicI32,
    pub rqs_cnt: core::sync::atomic::AtomicI32,
    pub dir_debugfs: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_stages {
    MLX5_IB_STAGE_INIT,
    MLX5_IB_STAGE_FS,
    MLX5_IB_STAGE_CAPS,
    MLX5_IB_STAGE_NON_DEFAULT_CB,
    MLX5_IB_STAGE_ROCE,
    MLX5_IB_STAGE_QP,
    MLX5_IB_STAGE_SRQ,
    MLX5_IB_STAGE_DEVICE_RESOURCES,
    MLX5_IB_STAGE_ODP,
    MLX5_IB_STAGE_COUNTERS,
    MLX5_IB_STAGE_CONG_DEBUGFS,
    MLX5_IB_STAGE_BFREG,
    MLX5_IB_STAGE_PRE_IB_REG_UMR,
    MLX5_IB_STAGE_WHITELIST_UID,
    MLX5_IB_STAGE_SYS_ERROR_NOTIFIER,
    MLX5_IB_STAGE_IB_REG,
    MLX5_IB_STAGE_DEVICE_NOTIFIER,
    MLX5_IB_STAGE_POST_IB_REG_UMR,
    MLX5_IB_STAGE_DELAY_DROP,
    MLX5_IB_STAGE_RESTRACK,
    MLX5_IB_STAGE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_stage {
    pub dev): *mut *mut int (init)(struct mlx5_ib_dev,
    pub dev): *mut *mut void (cleanup)(struct mlx5_ib_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_profile {
    pub stage: [mlx5_ib_stage; MLX5_IB_STAGE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_multiport_info {
    pub list: list_head,
    pub ibdev: *mut mlx5_ib_dev,
    pub mdev: *mut mlx5_core_dev,
    pub mdev_events: notifier_block,
    pub unref_comp: completion,
    pub sys_image_guid: u64,
    pub mdev_refcnt: u32,
    pub is_master: bool,
    pub unaffiliate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_flow_action {
    pub ib_action: ib_flow_action,
    pub ib_flags: u64,
    pub ctx: *mut mlx5_accel_esp_xfrm,
    pub esp_aes_gcm: },
    pub dev: *mut mlx5_ib_dev,
    pub sub_type: u32,
    pub modify_hdr: *mut mlx5_modify_hdr,
    pub pkt_reformat: *mut mlx5_pkt_reformat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_dm {
    pub dev: *mut mlx5_core_dev,
// This lock is used to protect the access to the shared
// allocation map when concurrent requests by different
// processes are handled.
//
    pub lock: spinlock_t,
    pub MLX5_MAX_MEMIC_PAGES): DECLARE_BITMAP(memic_alloc_pages,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_read_counters_attr {
    pub hw_cntrs_hndl: *mut mlx5_fc,
    pub out: *mut u64,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ib_counters_type {
    MLX5_IB_COUNTERS_FLOW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_mcounters {
    pub ibcntrs: ib_counters,
    pub type: mlx5_ib_counters_type,
// number of counters supported for this counters type
    pub counters_num: u32,
    pub hw_cntrs_hndl: *mut mlx5_fc,
// read function for this counters type
    pub read_attr): *mut mlx5_read_counters_attr,
// max index set as part of create_flow
    pub cntrs_max_index: u32,
// number of counters data entries (<description,index> pair)
    pub ncounters: u32,
// counters data array for descriptions and indexes
    pub counters_data: *mut mlx5_ib_flow_counters_desc,
// protects access to mcounters internal data
    pub mcntrs_mutex: mutex,
}

extern "C" {
    pub fn container_of(_arg: ibcntrs, mlx5_ib_mcounters: struct, _arg: ibcntrs) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_lb_state {
// protect the user_td
    pub mutex: mutex,
    pub user_td: u32,
    pub qps: c_int,
    pub enabled: bool,
    pub force_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_pf_eq {
    pub irq_nb: notifier_block,
    pub dev: *mut mlx5_ib_dev,
    pub core: *mut mlx5_eq,
    pub work: work_struct,
    pub /: *mut *mut spinlock_t lock; / Pagefaults spinlock,
    pub wq: *mut workqueue_struct,
    pub pool: *mut mempool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_devx_event_table {
    pub devx_nb: mlx5_nb,
// serialize updating the event_xa
    pub event_xa_lock: mutex,
    pub event_xa: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_var_region {
// serialize updating the bitmap
    pub bitmap_lock: mutex,
    pub bitmap: *mut c_ulong,
    pub hw_start_addr: u64,
    pub stride_size: u32,
    pub num_var_hw_entries: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_var_table {
    pub var_region: mlx5_var_region,
    pub tlp_var_region: mlx5_var_region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_port_caps {
    pub has_smi: bool,
    pub ext_port_cap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_special_mkeys {
    pub dump_fill_mkey: u32,
    pub null_mkey: __be32,
    pub terminate_scatter_list_mkey: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_macsec {
    pub /: *mut *mut mutex lock; / Protects mlx5_macsec internal contexts,
    pub macsec_devices_list: list_head,
    pub blocking_events_nb: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dev {
    pub ib_dev: ib_device,
    pub mdev: *mut mlx5_core_dev,
    pub data_direct_dev: *mut mlx5_data_direct_dev,
// protect accessing data_direct_dev
    pub data_direct_lock: mutex,
    pub mdev_events: notifier_block,
    pub sys_error_events: notifier_block,
    pub lag_events: notifier_block,
    pub num_ports: c_int,
// serialize update of capability mask
//
    pub cap_mask_mutex: mutex,
    pub ib_active:1: u8,
    pub is_rep:1: u8,
    pub lag_active:1: u8,
    pub fill_delay: u8,
    pub umrc: umr_common,
// sync used page count stats
//
    pub devr: mlx5_ib_resources,
    pub mkey_var: core::sync::atomic::AtomicI32,
// Prevents soft lock on massive reg MRs
    pub slow_path_mutex: mutex,
    pub odp_caps: ib_odp_caps,
    pub odp_max_size: u64,
    pub odp_eq_mutex: mutex,
    pub odp_pf_eq: mlx5_ib_pf_eq,
    pub odp_mkeys: xarray,
    pub flow_db: *mut mlx5_ib_flow_db,
// protect resources needed as part of reset flow
    pub reset_flow_resource_lock: spinlock_t,
    pub qp_list: list_head,
    pub data_direct_mr_list: list_head,
// Array with num_ports elements
    pub port: *mut mlx5_ib_port,
    pub bfreg: mlx5_sq_bfreg,
    pub fp_bfreg: mlx5_sq_bfreg,
    pub delay_drop: mlx5_ib_delay_drop,
    pub profile: *const mlx5_ib_profile,
    pub lb: mlx5_ib_lb_state,
    pub umr_fence: u8,
    pub ib_dev_list: list_head,
    pub sys_image_guid: u64,
    pub dm: mlx5_dm,
    pub devx_whitelist_uid: u16,
    pub srq_table: mlx5_srq_table,
    pub qp_table: mlx5_qp_table,
    pub async_ctx: mlx5_async_ctx,
    pub devx_event_table: mlx5_devx_event_table,
    pub var_table: mlx5_var_table,
    pub sig_mrs: xarray,
    pub port_caps: [mlx5_port_caps; MLX5_MAX_PORTS],
    pub pkey_table_len: u16,
    pub lag_ports: u8,
    pub mkeys: mlx5_special_mkeys,
    pub ddr: mlx5_data_direct_resources,

    pub macsec: mlx5_macsec,

    pub num_plane: u8,
    pub smi_dev: *mut mlx5_ib_dev,
    pub sub_dev_name: *const c_char,
}

extern "C" {
    pub fn container_of(_arg: mcq, mlx5_ib_cq: struct, _arg: mcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibxrcd, mlx5_ib_xrcd: struct, _arg: ibxrcd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibdev, mlx5_ib_dev: struct, _arg: ib_dev) -> return;
}
extern "C" {
    pub fn to_mdev(_arg: mr->ibmr.device) -> return;
}
extern "C" {
    pub fn to_mdev(_arg: context->ibucontext.device) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, mlx5_ib_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: core_qp, mlx5_ib_rwq: struct, _arg: core_qp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, mlx5_ib_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, mlx5_ib_srq: struct, _arg: ibsrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, mlx5_ib_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibwq, mlx5_ib_rwq: struct, _arg: ibwq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ib_rwq_ind_tbl, mlx5_ib_rwq_ind_table: struct, _arg: ib_rwq_ind_tbl) -> return;
}
extern "C" {
    pub fn container_of(_arg: msrq, mlx5_ib_srq: struct, _arg: msrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, mlx5_ib_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmw, mlx5_ib_mw: struct, _arg: ibmw) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibact, mlx5_ib_flow_action: struct, _arg: ib_action) -> return;
}
extern "C" {
    pub fn mlx5_ib_dev_res_cq_init(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_dev_res_srq_init(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_db_unmap_user(context: *mut mlx5_ib_ucontext, db: *mut mlx5_db);
}
extern "C" {
    pub fn __mlx5_ib_cq_clean(cq: *mut mlx5_ib_cq, qpn: u32, srq: *mut mlx5_ib_srq);
}
extern "C" {
    pub fn mlx5_ib_cq_clean(cq: *mut mlx5_ib_cq, qpn: u32, srq: *mut mlx5_ib_srq);
}
extern "C" {
    pub fn mlx5_ib_free_srq_wqe(srq: *mut mlx5_ib_srq, wqe_index: c_int);
}
extern "C" {
    pub fn mlx5_ib_query_ah(ibah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_query_srq(ibsrq: *mut ib_srq, srq_attr: *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_destroy_srq(srq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_enable_lb(dev: *mut mlx5_ib_dev, td: bool, qp: bool) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_disable_lb(dev: *mut mlx5_ib_dev, td: bool, qp: bool);
}
extern "C" {
    pub fn mlx5_ib_destroy_qp(qp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_drain_sq(qp: *mut ib_qp);
}
extern "C" {
    pub fn mlx5_ib_drain_rq(qp: *mut ib_qp);
}
extern "C" {
    pub fn mlx5_ib_destroy_cq(cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_poll_cq(ibcq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_pre_destroy_cq(cq: *mut ib_cq) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_post_destroy_cq(cq: *mut ib_cq);
}
extern "C" {
    pub fn mlx5_ib_arm_cq(ibcq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_modify_cq(cq: *mut ib_cq, cq_count: u16, cq_period: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_alloc_mw(mw: *mut ib_mw, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_dealloc_mw(mw: *mut ib_mw) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_free_odp_mr(mr: *mut mlx5_ib_mr);
}
extern "C" {
    pub fn mlx5_ib_dereg_mr(ibmr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_alloc_xrcd(xrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_dealloc_xrcd(xrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_query_ext_port_caps(dev: *mut mlx5_ib_dev, port: c_uint) -> c_int;
}
extern "C" {
    pub fn mlx5_query_mad_ifc_node_desc(dev: *mut mlx5_ib_dev, node_desc: *mut c_char) -> c_int;
}
extern "C" {
    pub fn mlx5_query_mad_ifc_node_guid(dev: *mut mlx5_ib_dev, node_guid: *mut __be64) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_get_cqe_size(ibcq: *mut ib_cq) -> c_int;
}
extern "C" {
    pub fn mlx5r_frmr_pools_init(device: *mut ib_device) -> c_int;
}
extern "C" {
    pub fn mlx5r_frmr_pools_cleanup(device: *mut ib_device);
}
extern "C" {
    pub fn mlx5_ib_destroy_wq(wq: *mut ib_wq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_destroy_rwq_ind_table(wq_ind_table: *mut ib_rwq_ind_table) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_data_direct_unbind(ibdev: *mut mlx5_ib_dev);
}
extern "C" {
    pub fn mlx5_ib_revoke_data_direct_mrs(dev: *mut mlx5_ib_dev);
}

extern "C" {
    pub fn mlx5_ib_odp_init_one(ibdev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5r_odp_create_eq(dev: *mut mlx5_ib_dev, eq: *mut mlx5_ib_pf_eq) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_odp_cleanup_one(ibdev: *mut mlx5_ib_dev);
}
extern "C" {
    pub fn mlx5_ib_odp_init() -> int __init;
}
extern "C" {
    pub fn mlx5_ib_odp_cleanup();
}
extern "C" {
    pub fn mlx5_odp_init_mkey_cache(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_init_odp_mr(mr: *mut mlx5_ib_mr, pd: *mut ib_pd) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_init_dmabuf_mr(mr: *mut mlx5_ib_mr, pd: *mut ib_pd) -> c_int;
}

// Needed for rep profile
extern "C" {
    pub fn mlx5_ib_cleanup_cong_debugfs(dev: *mut mlx5_ib_dev, port_num: u32);
}
extern "C" {
    pub fn mlx5_ib_init_cong_debugfs(dev: *mut mlx5_ib_dev, port_num: u32);
}
// GSI QP helper functions
extern "C" {
    pub fn mlx5_ib_destroy_gsi(mqp: *mut mlx5_ib_qp) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_gsi_pkey_change(gsi: *mut mlx5_ib_gsi_qp);
}
extern "C" {
    pub fn mlx5_ib_generate_wc(ibcq: *mut ib_cq, wc: *mut ib_wc) -> c_int;
}
//
// It returns non-zero value for unsupported CQ
// create flags, otherwise it returns zero.
//
// user_index = cmd_uidx;
// user_index = MLX5_IB_DEFAULT_UIDX;
extern "C" {
    pub fn verify_assign_uidx(_arg: cqe_version, _arg: ucmd->uidx, _arg: user_index) -> return;
}
extern "C" {
    pub fn verify_assign_uidx(_arg: cqe_version, _arg: ucmd->uidx, _arg: user_index) -> return;
}
// deref an mkey that can participate in ODP flow
// deref an mkey that can participate in ODP flow and wait for relese
//
// If the driver is in hash mode and the port_select_flow_table_bypass cap
// is supported, it means that the driver no longer needs to assign the port
// affinity by default. If a user wants to set the port affinity explicitly,
// the user has a dedicated API to do that, so there is no need to assign
// the port affinity by default.
//
// PCI Peer to Peer is a trainwreck. If no switch is present then things
// sometimes work, depending on the pci_distance_p2p logic for excluding broken
// root complexes. However if a switch is present in the path, then things get
// really ugly depending on how the switch is setup. This table assumes that the
// root complex is strict and is validating that all req/reps are matches
// perfectly - so any scenario where it sees only half the transaction is a
// failure.
//
// CR/RR/DT  ATS RO P2P
// 00X       X   X  OK
// 010       X   X  fails (request is routed to root but root never sees comp)
// 011       0   X  fails (request is routed to root but root never sees comp)
// 011       1   X  OK
// 10X       X   1  OK
// 101       X   0  fails (completion is routed to root but root didn't see req)
// 110       X   0  SLOW
// 111       0   0  SLOW
// 111       1   0  fails (completion is routed to root but root didn't see req)
// 111       1   1  OK
//
// Unfortunately we cannot reliably know if a switch is present or what the
// CR/RR/DT ACS settings are, as in a VM that is all hidden. Assume that
// CR/RR/DT is 111 if the ATS cap is enabled and follow the last three rows.
//
// For now assume if the umem is a dma_buf then it is P2P.
//
// For mkc users, instead of a page_offset the command has a start_iova which
// specifies both the page_offset and the on-the-wire IOVA
//
// In KSM mode HW requires IOVA and mkey's page size to be aligned
extern "C" {
    pub fn ib_umem_find_best_pgsz(_arg: umem, _arg: bitmap, _arg: iova) -> return;
}
