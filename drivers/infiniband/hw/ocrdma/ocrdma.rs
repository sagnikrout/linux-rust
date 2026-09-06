//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ocrdma/ocrdma.h
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


// This file is part of the Emulex RoCE Device Driver for
// RoCE (RDMA over Converged Ethernet) adapters.
// Copyright (C) 2012-2015 Emulex. All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.emulex.com
//
// This software is available to you under a choice of one of two licenses.
// You may choose to be licensed under the terms of the GNU General Public
// License (GPL) Version 2, available from the file COPYING in the main
// directory of this source tree, or the BSD license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// - Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//

pub const OC_SKH_DEVICE_PF: c_uint = 0x720;
pub const OC_SKH_DEVICE_VF: c_uint = 0x728;
pub const OCRDMA_MAX_AH: c_int = 512;

pub const EQ_INTR_PER_SEC_THRSH_HI: c_int = 150000;
pub const EQ_INTR_PER_SEC_THRSH_LOW: c_int = 100000;
pub const EQ_AIC_MAX_EQD: c_int = 20;
pub const EQ_AIC_MIN_EQD: c_int = 0;
extern "C" {
    pub fn ocrdma_eqd_set_task(work: *mut work_struct);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dev_attr {
    pub fw_ver: [u8; 32],
    pub vendor_id: u32,
    pub device_id: u32,
    pub max_pd: u16,
    pub max_dpp_pds: u16,
    pub max_cq: u16,
    pub max_cqe: u16,
    pub max_qp: u16,
    pub max_wqe: u16,
    pub max_rqe: u16,
    pub max_srq: u16,
    pub max_inline_data: u32,
    pub max_send_sge: c_int,
    pub max_recv_sge: c_int,
    pub max_srq_sge: c_int,
    pub max_rdma_sge: c_int,
    pub max_mr: c_int,
    pub max_mr_size: u64,
    pub max_num_mr_pbl: u32,
    pub max_mw: c_int,
    pub max_map_per_fmr: c_int,
    pub max_pages_per_frmr: c_int,
    pub max_ord_per_qp: u16,
    pub max_ird_per_qp: u16,
    pub device_cap_flags: c_int,
    pub cq_overflow_detect: u8,
    pub srq_supported: u8,
    pub wqe_size: u32,
    pub rqe_size: u32,
    pub ird_page_size: u32,
    pub local_ca_ack_delay: u8,
    pub ird: u8,
    pub num_ird_pages: u8,
    pub udp_encap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dma_mem {
    pub va: *mut c_void,
    pub pa: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_pbl {
    pub va: *mut c_void,
    pub pa: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_queue_info {
    pub va: *mut c_void,
    pub dma: dma_addr_t,
    pub size: u32,
    pub len: u16,
    pub /: *mut *mut u16 entry_size; / Size of an element in the queue,
    pub /: *mut *mut u16 id; / qid, where to ring the doorbell.,
    pub tail: u16 head,,
    pub created: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_aic_obj {
    pub prev_eqd: u32,
    pub eq_intr_cnt: u64,
    pub prev_eq_intr_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_eq {
    pub q: ocrdma_queue_info,
    pub vector: u32,
    pub cq_cnt: c_int,
    pub dev: *mut ocrdma_dev,
    pub irq_name: [c_char; 32],
    pub aic_obj: ocrdma_aic_obj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mq {
    pub sq: ocrdma_queue_info,
    pub cq: ocrdma_queue_info,
    pub rearm_cq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mqe_ctx {
    pub /: *mut *mut mutex lock; / for serializing mailbox commands on MQ,
    pub cmd_wait: wait_queue_head_t,
    pub tag: u32,
    pub cqe_status: u16,
    pub ext_status: u16,
    pub cmd_done: bool,
    pub fw_error_state: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_hw_mr {
    pub lkey: u32,
    pub fr_mr: u8,
    pub remote_atomic: u8,
    pub remote_rd: u8,
    pub remote_wr: u8,
    pub local_rd: u8,
    pub local_wr: u8,
    pub mw_bind: u8,
    pub rsvd: u8,
    pub len: u64,
    pub pbl_table: *mut ocrdma_pbl,
    pub num_pbls: u32,
    pub num_pbes: u32,
    pub pbl_size: u32,
    pub pbe_size: u32,
    pub va: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub hwmr: ocrdma_hw_mr,
    pub npages: u32,
    pub pages: [u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_stats {
    pub type: u8,
    pub dev: *mut ocrdma_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_pd_resource_mgr {
    pub pd_norm_start: u32,
    pub pd_norm_count: u16,
    pub pd_norm_thrsh: u16,
    pub max_normal_pd: u16,
    pub pd_dpp_start: u32,
    pub pd_dpp_count: u16,
    pub pd_dpp_thrsh: u16,
    pub max_dpp_pd: u16,
    pub dpp_page_index: u16,
    pub pd_norm_bitmap: *mut c_ulong,
    pub pd_dpp_bitmap: *mut c_ulong,
    pub pd_prealloc_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_mem {
    pub mqe: ocrdma_mqe,
    pub va: *mut c_void,
    pub pa: dma_addr_t,
    pub size: u32,
    pub debugfs_mem: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_info {
    pub auto_speeds_supported: u16,
    pub fixed_speeds_supported: u16,
    pub phy_type: u16,
    pub interface_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocrdma_flags {
    OCRDMA_FLAGS_LINK_STATUS_INIT = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_dev {
    pub ibdev: ib_device,
    pub attr: ocrdma_dev_attr,
    pub /: *mut *mut mutex dev_lock; / provides syncronise access to device data,
    pub ____cacheline_aligned: spinlock_t flush_q_lock,
    pub cq_tbl: *mut ocrdma_cq,
    pub qp_tbl: *mut ocrdma_qp,
    pub eq_tbl: *mut ocrdma_eq,
    pub eq_cnt: c_int,
    pub eqd_work: delayed_work,
    pub base_eqid: u16,
    pub max_eq: u16,
// provided synchronization to sgid table for
// updating gid entries triggered by notifier.
//
    pub sgid_lock: spinlock_t,
    pub gsi_qp_created: c_int,
    pub gsi_sqcq: *mut ocrdma_cq,
    pub gsi_rqcq: *mut ocrdma_cq,
    pub va: *mut ocrdma_av,
    pub pa: dma_addr_t,
    pub size: u32,
    pub num_ah: u32,
// provide synchronization for av
// entry allocations.
//
    pub lock: spinlock_t,
    pub ahid: u32,
    pub pbl: ocrdma_pbl,
    pub av_tbl: },
    pub mbx_cmd: *mut c_void,
    pub mq: ocrdma_mq,
    pub mqe_ctx: mqe_ctx,
    pub nic_info: be_dev_info,
    pub phy: phy_info,
    pub model_number: [c_char; 32],
    pub hba_port_num: u32,
    pub entry: list_head,
    pub id: c_int,
    pub stag_arr: *mut u64,
    pub /: *mut *mut u8 sl; / service level,
    pub pfc_state: bool,
    pub update_sl: core::sync::atomic::AtomicI32,
    pub pvid: u16,
    pub asic_id: u32,
    pub flags: u32,
    pub last_stats_time: c_ulong,
    pub /: *mut *mut mutex stats_lock; / provide synch for debugfs operations,
    pub stats_mem: stats_mem,
    pub rsrc_stats: ocrdma_stats,
    pub rx_stats: ocrdma_stats,
    pub wqe_stats: ocrdma_stats,
    pub tx_stats: ocrdma_stats,
    pub db_err_stats: ocrdma_stats,
    pub tx_qp_err_stats: ocrdma_stats,
    pub rx_qp_err_stats: ocrdma_stats,
    pub tx_dbg_stats: ocrdma_stats,
    pub rx_dbg_stats: ocrdma_stats,
    pub driver_stats: ocrdma_stats,
    pub reset_stats: ocrdma_stats,
    pub dir: *mut dentry,
    pub async_err_stats: [core::sync::atomic::AtomicI32; OCRDMA_MAX_ASYNC_ERRORS],
    pub cqe_err_stats: [core::sync::atomic::AtomicI32; OCRDMA_MAX_CQE_ERR],
    pub pd_mgr: *mut ocrdma_pd_resource_mgr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_cq {
    pub ibcq: ib_cq,
    pub va: *mut ocrdma_cqe,
    pub phase: u32,
    pub to: *mut *mut u32 getp; / pointer to pending wrs,
// return to stack, wrap arounds
// at max_hw_cqe
//
    pub max_hw_cqe: u32,
    pub phase_change: bool,
    pub synchronization: *mut *mut spinlock_t cq_lock ____cacheline_aligned; / provide,
// to cq polling
//
// syncronizes cq completion handler invoked from multiple context
    pub ____cacheline_aligned: spinlock_t comp_handler_lock,
    pub id: u16,
    pub eqn: u16,
    pub ucontext: *mut ocrdma_ucontext,
    pub pa: dma_addr_t,
    pub len: u32,
    pub cqe_cnt: u32,
// head of all qp's sq and rq for which cqes need to be flushed
// by the software.
//
    pub rq_head: list_head sq_head,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_pd {
    pub ibpd: ib_pd,
    pub uctx: *mut ocrdma_ucontext,
    pub id: u32,
    pub num_dpp_qp: c_int,
    pub dpp_page: u32,
    pub dpp_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ah {
    pub ibah: ib_ah,
    pub av: *mut ocrdma_av,
    pub sgid_index: u16,
    pub id: u32,
    pub hdr_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_qp_hwq_info {
    pub /: *mut *mut *mut u8 va; / virtual address,
    pub max_sges: u32,
    pub tail: u32 head,,
    pub entry_size: u32,
    pub max_cnt: u32,
    pub max_wqe_idx: u32,
    pub /: *mut *mut u16 dbid; / qid, where to ring the doorbell.,
    pub len: u32,
    pub pa: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_srq {
    pub ibsrq: ib_srq,
    pub db: *mut u8 __iomem,
    pub rq: ocrdma_qp_hwq_info,
    pub rqe_wr_id_tbl: *mut u64,
    pub idx_bit_fields: *mut u32,
    pub bit_fields_len: u32,
// provide synchronization to multiple context(s) posting rqe
    pub ____cacheline_aligned: spinlock_t q_lock,
    pub pd: *mut ocrdma_pd,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_qp {
    pub ibqp: ib_qp,
    pub sq_db: *mut u8 __iomem,
    pub sq: ocrdma_qp_hwq_info,
    pub wrid: u64,
    pub dpp_wqe_idx: u16,
    pub dpp_wqe: u16,
    pub signaled: u8,
    pub rsvd: [u8; 3],
    pub wqe_wr_id_tbl: *mut },
    pub max_inline_data: u32,
// provide synchronization to multiple context(s) posting wqe, rqe
    pub ____cacheline_aligned: spinlock_t q_lock,
    pub sq_cq: *mut ocrdma_cq,
// list maintained per CQ to flush SQ errors
    pub sq_entry: list_head,
    pub rq_db: *mut u8 __iomem,
    pub rq: ocrdma_qp_hwq_info,
    pub rqe_wr_id_tbl: *mut u64,
    pub rq_cq: *mut ocrdma_cq,
    pub srq: *mut ocrdma_srq,
// list maintained per CQ to flush RQ errors
    pub rq_entry: list_head,
    pub /: *mut *mut ocrdma_qp_state state; / QP state,
    pub cap_flags: c_int,
    pub max_ird: u32 max_ord,,
    pub id: u32,
    pub pd: *mut ocrdma_pd,
    pub qp_type: ib_qp_type,
    pub sgid_idx: c_int,
    pub qkey: u32,
    pub dpp_enabled: bool,
    pub ird_q_va: *mut u8,
    pub signaled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_ucontext {
    pub ibucontext: ib_ucontext,
    pub mm_head: list_head,
    pub /: *mut *mut mutex mm_list_lock; / protects list entries of mm type,
    pub cntxt_pd: *mut ocrdma_pd,
    pub pd_in_use: c_int,
    pub va: *mut u32,
    pub pa: dma_addr_t,
    pub len: u32,
    pub ah_tbl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_mm {
    pub phy_addr: u64,
    pub len: c_ulong,
    pub key: },
    pub entry: list_head,
}

extern "C" {
    pub fn container_of(_arg: ibdev, ocrdma_dev: struct, _arg: ibdev) -> return;
}
// ibucontext)
extern "C" {
    pub fn container_of(_arg: ibucontext, ocrdma_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, ocrdma_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, ocrdma_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, ocrdma_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, ocrdma_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, ocrdma_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, ocrdma_srq: struct, _arg: ibsrq) -> return;
}
// Appl-state and Logical-link-state in future.
//
