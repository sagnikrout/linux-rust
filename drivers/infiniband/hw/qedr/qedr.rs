//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/qedr/qedr.h
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


// QLogic qedr NIC Driver
// Copyright (c) 2015-2016  QLogic Corporation
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and /or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_cnq {
    pub dev: *mut qedr_dev,
    pub pbl: qed_chain,
    pub sb: *mut qed_sb_info,
    pub name: [c_char; 32],
    pub n_comp: u64,
    pub hw_cons_ptr: *mut __le16,
    pub index: u8,
}

pub const QEDR_MAX_SGID: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_device_attr {
    pub vendor_id: u32,
    pub vendor_part_id: u32,
    pub hw_ver: u32,
    pub fw_ver: u64,
    pub node_guid: u64,
    pub sys_image_guid: u64,
    pub max_cnq: u8,
    pub max_sge: u8,
    pub max_inline: u16,
    pub max_sqe: u32,
    pub max_rqe: u32,
    pub max_qp_resp_rd_atomic_resc: u8,
    pub max_qp_req_rd_atomic_resc: u8,
    pub max_dev_resp_rd_atomic_resc: u64,
    pub max_cq: u32,
    pub max_qp: u32,
    pub max_mr: u32,
    pub max_mr_size: u64,
    pub max_cqe: u32,
    pub max_mw: u32,
    pub max_mr_mw_fmr_pbl: u32,
    pub max_mr_mw_fmr_size: u64,
    pub max_pd: u32,
    pub max_ah: u32,
    pub max_pkey: u8,
    pub max_srq: u32,
    pub max_srq_wr: u32,
    pub max_srq_sge: u8,
    pub max_stats_queues: u8,
    pub dev_caps: u32,
    pub page_size_caps: u64,
    pub dev_ack_delay: u8,
    pub reserved_lkey: u32,
    pub bad_pkey_counter: u32,
    pub events: qed_rdma_events,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_dev {
    pub ibdev: ib_device,
    pub cdev: *mut qed_dev,
    pub pdev: *mut pci_dev,
    pub ndev: *mut net_device,
    pub atomic_cap: ib_atomic_cap,
    pub rdma_ctx: *mut c_void,
    pub attr: qedr_device_attr,
    pub ops: *const qed_rdma_ops,
    pub int_info: qed_int_info,
    pub sb_array: *mut qed_sb_info,
    pub cnq_array: *mut qedr_cnq,
    pub num_cnq: c_int,
    pub sb_start: c_int,
    pub db_addr: *mut void __iomem,
    pub db_phys_addr: u64,
    pub db_size: u32,
    pub dpi: u16,
    pub sgid_tbl: *mut ib_gid,
// Lock for sgid table
    pub sgid_lock: spinlock_t,
    pub guid: u64,
    pub dp_module: u32,
    pub dp_level: u8,
    pub num_hwfns: u8,

    pub affin_hwfn_idx: u8,
    pub gsi_ll2_handle: u8,
    pub wq_multiplier: c_uint,
    pub gsi_ll2_mac_address: [u8; ETH_ALEN],
    pub gsi_qp_created: c_int,
    pub gsi_sqcq: *mut qedr_cq,
    pub gsi_rqcq: *mut qedr_cq,
    pub gsi_qp: *mut qedr_qp,
    pub rdma_type: qed_rdma_type,
    pub qps: xarray,
    pub srqs: xarray,
    pub iwarp_wq: *mut workqueue_struct,
    pub iwarp_max_mtu: u16,
    pub enet_state: c_ulong,
    pub user_dpm_enabled: u8,
}

// RQ

pub const QEDR_ROCE_PKEY_TABLE_LEN: c_int = 1;
pub const QEDR_ROCE_PKEY_DEFAULT: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_pbl {
    pub list_entry: list_head,
    pub va: *mut c_void,
    pub pa: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_ucontext {
    pub ibucontext: ib_ucontext,
    pub dev: *mut qedr_dev,
    pub pd: *mut qedr_pd,
    pub dpi_addr: *mut void __iomem,
    pub db_mmap_entry: *mut rdma_user_mmap_entry,
    pub dpi_phys_addr: u64,
    pub dpi_size: u32,
    pub dpi: u16,
    pub db_rec: bool,
    pub edpm_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union db_prod32 {
    pub data: rdma_pwm_val16_data,
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union db_prod64 {
    pub data: rdma_pwm_val32_data,
    pub raw: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedr_cq_type {
    QEDR_CQ_TYPE_GSI,
    QEDR_CQ_TYPE_KERNEL,
    QEDR_CQ_TYPE_USER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_pbl_info {
    pub num_pbls: u32,
    pub num_pbes: u32,
    pub pbl_size: u32,
    pub pbe_size: u32,
    pub two_layered: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_userq {
    pub umem: *mut ib_umem,
    pub pbl_info: qedr_pbl_info,
    pub pbl_tbl: *mut qedr_pbl,
    pub buf_addr: u64,
    pub buf_len: usize,
// doorbell recovery
    pub db_addr: *mut void __iomem,
    pub db_rec_data: *mut qedr_user_db_rec,
    pub db_mmap_entry: *mut rdma_user_mmap_entry,
    pub db_rec_db2_addr: *mut void __iomem,
    pub db_rec_db2_data: db_prod32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_cq {
    pub ibcq: ib_cq,
    pub cq_type: qedr_cq_type,
    pub sig: u32,
    pub icid: u16,
// Lock to protect multiplem CQ's
    pub cq_lock: spinlock_t,
    pub arm_flags: u8,
    pub pbl: qed_chain,
    pub db_addr: *mut void __iomem,
    pub db: db_prod64,
    pub pbl_toggle: u8,
    pub latest_cqe: *mut rdma_cqe,
    pub toggle_cqe: *mut rdma_cqe,
    pub cq_cons: u32,
    pub q: qedr_userq,
    pub destroyed: u8,
    pub cnq_notif: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_pd {
    pub ibpd: ib_pd,
    pub pd_id: u32,
    pub uctx: *mut qedr_ucontext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_xrcd {
    pub ibxrcd: ib_xrcd,
    pub xrcd_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_qp_hwq_info {
// WQE Elements
    pub pbl: qed_chain,
    pub p_phys_addr_tbl: u64,
    pub max_sges: u32,
// WQE
    pub prod: u16,
    pub cons: u16,
    pub wqe_cons: u16,
    pub gsi_cons: u16,
    pub max_wr: u16,
// DB
    pub db: *mut void __iomem,
    pub db_data: db_prod32,
    pub iwarp_db2: *mut void __iomem,
    pub iwarp_db2_data: db_prod32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_srq_hwq_info {
    pub max_sges: u32,
    pub max_wr: u32,
    pub pbl: qed_chain,
    pub p_phys_addr_tbl: u64,
    pub wqe_prod: u32,
    pub sge_prod: u32,
    pub wr_prod_cnt: u32,
    pub wr_cons_cnt: core::sync::atomic::AtomicI32,
    pub num_elems: u32,
    pub virt_prod_pair_addr: *mut rdma_srq_producers,
    pub phy_prod_pair_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_srq {
    pub ibsrq: ib_srq,
    pub dev: *mut qedr_dev,
    pub usrq: qedr_userq,
    pub hw_srq: qedr_srq_hwq_info,
    pub prod_umem: *mut ib_umem,
    pub srq_id: u16,
    pub srq_limit: u32,
    pub is_xrc: bool,
// lock to protect srq recv post
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedr_qp_err_bitmap {
    QEDR_QP_ERR_SQ_FULL = 1,
    QEDR_QP_ERR_RQ_FULL = 2,
    QEDR_QP_ERR_BAD_SR = 4,
    QEDR_QP_ERR_BAD_RR = 8,
    QEDR_QP_ERR_SQ_PBL_FULL = 16,
    QEDR_QP_ERR_RQ_PBL_FULL = 32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedr_qp_create_type {
    QEDR_QP_CREATE_NONE,
    QEDR_QP_CREATE_USER,
    QEDR_QP_CREATE_KERNEL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedr_iwarp_cm_flags {
    QEDR_IWARP_CM_WAIT_FOR_CONNECT    = BIT(0),
    QEDR_IWARP_CM_WAIT_FOR_DISCONNECT = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_qp {
    pub /: *mut *mut ib_qp ibqp; / must be first,
    pub dev: *mut qedr_dev,
    pub sq: qedr_qp_hwq_info,
    pub rq: qedr_qp_hwq_info,
    pub max_inline_data: u32,
// Lock for QP's
    pub q_lock: spinlock_t,
    pub sq_cq: *mut qedr_cq,
    pub rq_cq: *mut qedr_cq,
    pub srq: *mut qedr_srq,
    pub state: qed_roce_qp_state,
    pub id: u32,
    pub pd: *mut qedr_pd,
    pub qp_type: ib_qp_type,
    pub create_type: qedr_qp_create_type,
    pub qed_qp: *mut qed_rdma_qp,
    pub qp_id: u32,
    pub icid: u16,
    pub mtu: u16,
    pub sgid_idx: c_int,
    pub rq_psn: u32,
    pub sq_psn: u32,
    pub qkey: u32,
    pub dest_qp_num: u32,
    pub timeout: u8,
// Relevant to qps created from kernel space only (ULPs)
    pub prev_wqe_size: u8,
    pub wqe_cons: u16,
    pub err_bitmap: u32,
    pub signaled: bool,
// SQ shadow
    pub wr_id: u64,
    pub opcode: ib_wc_opcode,
    pub bytes_len: u32,
    pub wqe_size: u8,
    pub signaled: bool,
    pub icrc_mapping: dma_addr_t,
    pub icrc: *mut u32,
    pub mr: *mut qedr_mr,
    pub wqe_wr_id: *mut },
// RQ shadow
    pub wr_id: u64,
    pub sg_list: [ib_sge; RDMA_MAX_SGE_PER_RQ_WQE],
    pub wqe_size: u8,
    pub smac: [u8; ETH_ALEN],
    pub vlan: u16,
    pub rc: c_int,
    pub rqe_wr_id: *mut },
// Relevant to qps created from user space only (applications)
    pub usq: qedr_userq,
    pub urq: qedr_userq,
// synchronization objects used with iwarp ep
    pub refcnt: kref,
    pub iwarp_cm_comp: completion,
    pub qp_rel_comp: completion,
    pub /: *mut *mut unsigned long iwarp_cm_flags; / enum iwarp_cm_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_ah {
    pub ibah: ib_ah,
    pub attr: rdma_ah_attr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedr_mr_type {
    QEDR_MR_USER,
    QEDR_MR_KERNEL,
    QEDR_MR_DMA,
    QEDR_MR_FRMR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_info {
    pub pbl_table: *mut qedr_pbl,
    pub pbl_info: qedr_pbl_info,
    pub free_pbl_list: list_head,
    pub inuse_pbl_list: list_head,
    pub completed: u32,
    pub completed_handled: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub hw_mr: qed_rdma_register_tid_in_params,
    pub type: qedr_mr_type,
    pub dev: *mut qedr_dev,
    pub info: mr_info,
    pub pages: *mut u64,
    pub npages: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_user_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub dev: *mut qedr_dev,
    pub io_address: u64,
    pub address: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_iw_listener {
    pub dev: *mut qedr_dev,
    pub cm_id: *mut iw_cm_id,
    pub backlog: c_int,
    pub qed_handle: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_iw_ep {
    pub dev: *mut qedr_dev,
    pub cm_id: *mut iw_cm_id,
    pub qp: *mut qedr_qp,
    pub qed_context: *mut c_void,
    pub refcnt: kref,
}

extern "C" {
    pub fn container_of(_arg: ibucontext, qedr_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibdev, qedr_dev: struct, _arg: ibdev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, qedr_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibxrcd, qedr_xrcd: struct, _arg: ibxrcd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, qedr_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, qedr_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, qedr_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, qedr_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, qedr_srq: struct, _arg: ibsrq) -> return;
}
