//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/bnxt_re.h
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


//
// Broadcom NetXtreme-E RoCE driver.
//
// Copyright (c) 2016 - 2017, Broadcom. All rights reserved.  The term
// Broadcom refers to Broadcom Limited and/or its subsidiaries.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// BSD license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS''
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS
// BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
// OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN
// IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Description: Slow Path Operators (header)
//

pub const BNXT_RE_PAGE_SIZE_SUPPORTED: c_uint = 0x7FFFF000 /* 4kb - 1G */;

// Number of MRs to reserve for PF, leaving remainder for VFs

pub const BNXT_RE_MAX_GID_PER_VF: c_int = 128;
//
// Percentage of resources of each type reserved for PF.
// Remaining resources are divided equally among VFs.
// [0, 100]
//
pub const BNXT_RE_PCT_RSVD_FOR_PF: c_int = 50;
pub const BNXT_RE_UD_QP_HW_STALL: c_uint = 0x400000;
pub const BNXT_RE_RQ_WQE_THRESHOLD: c_int = 32;
//
// Setting the default ack delay value to 16, which means
// the default timeout is approx. 260ms(4 usec * 2 ^(timeout))
//
pub const BNXT_RE_DEFAULT_ACK_DELAY: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_ring_attr {
    pub dma_arr: *mut dma_addr_t,
    pub pages: c_int,
    pub type: c_int,
    pub depth: u32,
    pub /: *mut *mut u32 lrid; / Logical ring id,
    pub mode: u8,
}

//
// Data structure and defines to handle
// recovery
//
pub const BNXT_RE_PRE_RECOVERY_REMOVE: c_uint = 0x1;
pub const BNXT_RE_COMPLETE_REMOVE: c_uint = 0x2;
pub const BNXT_RE_POST_RECOVERY_INIT: c_uint = 0x4;
pub const BNXT_RE_COMPLETE_INIT: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_sqp_entries {
    pub sge: bnxt_qplib_sge,
    pub wrid: u64,
// For storing the actual qp1 cqe
    pub cqe: bnxt_qplib_cqe,
    pub qp1_qp: *mut bnxt_re_qp,
}

pub const BNXT_RE_MAX_GSI_SQP_ENTRIES: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_gsi_context {
    pub gsi_qp: *mut bnxt_re_qp,
    pub gsi_sqp: *mut bnxt_re_qp,
    pub gsi_sah: *mut bnxt_re_ah,
    pub sqp_tbl: *mut bnxt_re_sqp_entries,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_en_dev_info {
    pub en_dev: *mut bnxt_en_dev,
    pub rdev: *mut bnxt_re_dev,
}

pub const BNXT_RE_AEQ_IDX: c_int = 0;
pub const BNXT_RE_NQ_IDX: c_int = 1;
pub const BNXT_RE_GEN_P5_MAX_VF: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_pacing {
    pub dbr_db_fifo_reg_off: u64,
    pub dbr_page: *mut c_void,
    pub dbr_bar_addr: u64,
    pub pacing_algo_th: u32,
    pub do_pacing_save: u32,
    pub /: *mut *mut u32 dbq_pacing_time; / ms,
    pub dbr_def_do_pacing: u32,
    pub dbr_pacing: bool,
    pub /: *mut *mut mutex dbq_lock; / synchronize db pacing algo,
}

pub const BNXT_RE_MAX_DBR_DO_PACING: c_uint = 0xFFFF;

// Default do_pacing value when there is no congestion
pub const BNXT_RE_DBR_DO_PACING_NO_CONGESTION: c_uint = 0x7F /* 1 in 512 probability */;
pub const BNXT_RE_MAX_FIFO_DEPTH_P5: c_uint = 0x2c00;
pub const BNXT_RE_MAX_FIFO_DEPTH_P7: c_uint = 0x8000;

pub const BNXT_RE_GRC_FIFO_REG_BASE: c_uint = 0x2000;
pub const BNXT_RE_MIN_MSIX: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_nq_record {
    pub msix_entries: [bnxt_msix_entry; BNXT_RE_MAX_MSIX],
    pub nq: [bnxt_qplib_nq; BNXT_RE_MAX_MSIX],
    pub num_msix: c_int,
// serialize NQ access
    pub load_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_dev {
    pub ibdev: ib_device,
    pub list: list_head,
    pub flags: c_ulong,
pub const BNXT_RE_FLAG_NETDEV_REGISTERED: c_int = 0;
pub const BNXT_RE_FLAG_STATS_CTX3_ALLOC: c_int = 1;
pub const BNXT_RE_FLAG_HAVE_L2_REF: c_int = 3;
pub const BNXT_RE_FLAG_RCFW_CHANNEL_EN: c_int = 4;
pub const BNXT_RE_FLAG_RESOURCES_ALLOCATED: c_int = 7;
pub const BNXT_RE_FLAG_RESOURCES_INITIALIZED: c_int = 8;
pub const BNXT_RE_FLAG_ERR_DEVICE_DETACHED: c_int = 17;
pub const BNXT_RE_FLAG_ISSUE_ROCE_STATS: c_int = 29;
    pub netdev: *mut net_device,
    pub adev: *mut auxiliary_device,
    pub minor: unsigned int version, major,,
    pub chip_ctx: *mut bnxt_qplib_chip_ctx,
    pub en_dev: *mut bnxt_en_dev,
    pub id: c_int,
// RCFW Channel
    pub rcfw: bnxt_qplib_rcfw,
// NQ record
    pub nqr: *mut bnxt_re_nq_record,
// Device Resources
    pub dev_attr: *mut bnxt_qplib_dev_attr,
    pub qplib_ctx: bnxt_qplib_ctx,
    pub qplib_res: bnxt_qplib_res,
    pub dpi_privileged: bnxt_qplib_dpi,
    pub cq_coalescing: bnxt_qplib_cq_coal_param,
    pub /: *mut *mut mutex qp_lock; / protect qp list,
    pub qp_list: list_head,
// Max of 2 lossless traffic class supported per port
    pub cosq: [u16; 2],
// QP for handling QP1 packets
    pub gsi_ctx: bnxt_re_gsi_context,
    pub stats: bnxt_re_stats,
    pub nq_alloc_cnt: core::sync::atomic::AtomicI32,
    pub is_virtfn: u32,
    pub num_vfs: u32,
    pub pacing: bnxt_re_pacing,
    pub dbq_fifo_check_work: work_struct,
    pub dbq_pacing_work: delayed_work,
    pub dbg_root: *mut dentry,
    pub qp_debugfs: *mut dentry,
    pub event_bitmap: c_ulong,
    pub cc_param: bnxt_qplib_cc_param,
    pub dcb_wq: *mut workqueue_struct,
    pub cc_config: *mut dentry,
    pub cc_config_params: *mut bnxt_re_dbg_cc_config_params,
    pub cq_coal_cfg: *mut dentry,
    pub cq_coal_cfg_params: *mut bnxt_re_dbg_cq_coal_params,
pub const BNXT_VPD_FLD_LEN: c_int = 32;
    pub board_partno: [c_char; BNXT_VPD_FLD_LEN],
// RoCE mirror
    pub mirror_vnic_id: u16,
    pub ugid: ib_gid,
    pub ugid_index: u32,
    pub 1: u8 sniffer_flow_created :,
}

pub const BNXT_RE_ROCE_V1_PACKET: c_int = 0;
pub const BNXT_RE_ROCEV2_IPV4_PACKET: c_int = 2;
pub const BNXT_RE_ROCEV2_IPV6_PACKET: c_int = 3;

extern "C" {
    pub fn bnxt_re_pacing_alert(rdev: *mut bnxt_re_dev);
}
extern "C" {
    pub fn bnxt_re_assign_pma_port_counters(rdev: *mut bnxt_re_dev, out_mad: *mut ib_mad) -> c_int;
}
extern "C" {
    pub fn bnxt_re_hwrm_free_vnic(rdev: *mut bnxt_re_dev);
}
extern "C" {
    pub fn bnxt_re_hwrm_alloc_vnic(rdev: *mut bnxt_re_dev) -> c_int;
}
extern "C" {
    pub fn bnxt_re_hwrm_cfg_vnic(rdev: *mut bnxt_re_dev, qp_id: u32) -> c_int;
}
pub const BNXT_RE_CONTEXT_TYPE_QPC_SIZE_P5: c_int = 1088;
pub const BNXT_RE_CONTEXT_TYPE_CQ_SIZE_P5: c_int = 128;
pub const BNXT_RE_CONTEXT_TYPE_MRW_SIZE_P5: c_int = 128;
pub const BNXT_RE_CONTEXT_TYPE_SRQ_SIZE_P5: c_int = 192;
pub const BNXT_RE_CONTEXT_TYPE_QPC_SIZE_P7: c_int = 1088;
pub const BNXT_RE_CONTEXT_TYPE_CQ_SIZE_P7: c_int = 192;
pub const BNXT_RE_CONTEXT_TYPE_MRW_SIZE_P7: c_int = 192;
pub const BNXT_RE_CONTEXT_TYPE_SRQ_SIZE_P7: c_int = 192;

