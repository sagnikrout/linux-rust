//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/qplib_res.h
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
// Description: QPLib resource manager (header)
//

pub const CHIP_NUM_57508: c_uint = 0x1750;
pub const CHIP_NUM_57504: c_uint = 0x1751;
pub const CHIP_NUM_57502: c_uint = 0x1752;
pub const CHIP_NUM_58818: c_uint = 0xd818;
pub const CHIP_NUM_57608: c_uint = 0x1760;

pub const BNXT_QPLIB_DBR_EPOCH_SHIFT: c_int = 24;
pub const BNXT_QPLIB_DBR_TOGGLE_SHIFT: c_int = 25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_drv_modes {
    pub wqe_mode: u8,
    pub db_push: bool,
    pub dbr_pacing: bool,
    pub toggle_bits: u32,
    pub roce_mirror: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_toggle_modes {
    BNXT_QPLIB_CQ_TOGGLE_BIT = 0x1,
    BNXT_QPLIB_SRQ_TOGGLE_BIT = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_chip_ctx {
    pub chip_num: u16,
    pub chip_rev: u8,
    pub chip_metal: u8,
    pub hw_stats_size: u16,
    pub hwrm_cmd_max_timeout: u16,
    pub modes: bnxt_qplib_drv_modes,
    pub hwrm_intf_ver: u64,
    pub dbr_stat_db_fifo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_db_pacing_data {
    pub do_pacing: u32,
    pub pacing_th: u32,
    pub alarm_th: u32,
    pub fifo_max_depth: u32,
    pub fifo_room_mask: u32,
    pub fifo_room_shift: u32,
    pub grc_reg_offset: u32,
    pub dev_err_state: u32,
}

pub const BNXT_QPLIB_DBR_PF_DB_OFFSET: c_uint = 0x10000;
pub const BNXT_QPLIB_DBR_VF_DB_OFFSET: c_uint = 0x4000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_qplib_hwq_type {
    HWQ_TYPE_CTX,
    HWQ_TYPE_QUEUE,
    HWQ_TYPE_L2_CMPL,
    HWQ_TYPE_MR
}

pub const MAX_PBL_LVL_0_PGS: c_int = 1;
pub const MAX_PBL_LVL_1_PGS: c_int = 512;
pub const MAX_PBL_LVL_1_PGS_SHIFT: c_int = 9;
pub const MAX_PBL_LVL_1_PGS_FOR_LVL_2: c_int = 256;

pub const MAX_PDL_LVL_SHIFT: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_qplib_pbl_lvl {
    PBL_LVL_0,
    PBL_LVL_1,
    PBL_LVL_2,
    PBL_LVL_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_qplib_hwrm_pg_size {
    BNXT_QPLIB_HWRM_PG_SIZE_4K	= 0,
    BNXT_QPLIB_HWRM_PG_SIZE_8K	= 1,
    BNXT_QPLIB_HWRM_PG_SIZE_64K	= 2,
    BNXT_QPLIB_HWRM_PG_SIZE_2M	= 3,
    BNXT_QPLIB_HWRM_PG_SIZE_8M	= 4,
    BNXT_QPLIB_HWRM_PG_SIZE_1G	= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_reg_desc {
    pub bar_id: u8,
    pub bar_base: resource_size_t,
    pub offset: c_ulong,
    pub bar_reg: *mut void __iomem,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_pbl {
    pub pg_count: u32,
    pub pg_size: u32,
    pub pg_arr: *mut c_void,
    pub pg_map_arr: *mut dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_sg_info {
    pub umem: *mut ib_umem,
    pub npages: u32,
    pub pgshft: u32,
    pub pgsize: u32,
    pub nopte: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_hwq_attr {
    pub res: *mut bnxt_qplib_res,
    pub sginfo: *mut bnxt_qplib_sg_info,
    pub type: bnxt_qplib_hwq_type,
    pub depth: u32,
    pub stride: u32,
    pub aux_stride: u32,
    pub aux_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_hwq {
    pub pdev: *mut pci_dev,
// lock to protect qplib_hwq
    pub lock: spinlock_t,
    pub 1]: bnxt_qplib_pbl pbl[PBL_LVL_MAX +,
    pub /: *mut *mut bnxt_qplib_pbl_lvl level; / 0, 1, or 2,
// ptr for easy access to the PBL entries
    pub pbl_ptr: *mut c_void,
// ptr for easy access to the dma_addr
    pub pbl_dma_ptr: *mut dma_addr_t,
    pub max_elements: u32,
    pub depth: u32,
    pub /: *mut *mut u16 element_size; / Size of each entry,
    pub /: *mut *mut u16 qe_ppg; / queue entry per page,
    pub /: *mut *mut u32 prod; / raw,
    pub /: *mut *mut u32 cons; / raw,
    pub cp_bit: u8,
    pub is_user: u8,
    pub pg_sz_lvl: u8,
    pub pad_pg: *mut u64,
    pub pad_stride: u32,
    pub pad_pgofft: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_db_info {
    pub db: *mut void __iomem,
    pub priv_db: *mut void __iomem,
    pub hwq: *mut bnxt_qplib_hwq,
    pub xid: u32,
    pub max_slot: u32,
    pub flags: u32,
    pub toggle: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_qplib_db_info_flags_mask {
    BNXT_QPLIB_FLAG_EPOCH_CONS_SHIFT        = 0x0UL,
    BNXT_QPLIB_FLAG_EPOCH_PROD_SHIFT        = 0x1UL,
    BNXT_QPLIB_FLAG_EPOCH_CONS_MASK         = 0x1UL,
    BNXT_QPLIB_FLAG_EPOCH_PROD_MASK         = 0x2UL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_qplib_db_epoch_flag_shift {
    BNXT_QPLIB_DB_EPOCH_CONS_SHIFT  = BNXT_QPLIB_DBR_EPOCH_SHIFT,
    BNXT_QPLIB_DB_EPOCH_PROD_SHIFT  = (BNXT_QPLIB_DBR_EPOCH_SHIFT - 1),
}

// Tables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_pd_tbl {
    pub tbl: *mut c_ulong,
    pub max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_sgid_tbl {
    pub tbl: *mut bnxt_qplib_gid_info,
    pub hw_id: *mut u16,
    pub max: u16,
    pub active: u16,
    pub ctx: *mut c_void,
    pub vlan: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_dpi {
    pub dpi: u32,
    pub bit: u32,
    pub dbr: *mut void __iomem,
    pub umdbr: u64,
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_dpi_tbl {
    pub app_tbl: *mut c_void,
    pub tbl: *mut c_ulong,
    pub max: u16,
    pub /: *mut *mut bnxt_qplib_reg_desc ucreg; / Hold entire DB bar.,
    pub wcreg: bnxt_qplib_reg_desc,
    pub priv_db: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_stats {
    pub dma_map: dma_addr_t,
    pub dma: *mut c_void,
    pub size: u32,
    pub fw_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_vf_res {
    pub max_qp_per_vf: u32,
    pub max_mrw_per_vf: u32,
    pub max_srq_per_vf: u32,
    pub max_cq_per_vf: u32,
    pub max_gid_per_vf: u32,
}

pub const BNXT_QPLIB_MAX_QP_CTX_ENTRY_SIZE: c_int = 448;
pub const BNXT_QPLIB_MAX_SRQ_CTX_ENTRY_SIZE: c_int = 64;
pub const BNXT_QPLIB_MAX_CQ_CTX_ENTRY_SIZE: c_int = 64;
pub const BNXT_QPLIB_MAX_MRW_CTX_ENTRY_SIZE: c_int = 128;
pub const MAX_TQM_ALLOC_REQ: c_int = 48;
pub const MAX_TQM_ALLOC_BLK_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_tqm_ctx {
    pub pde: bnxt_qplib_hwq,
    pub /: *mut *mut u8 pde_level; / Original level,
    pub qtbl: [bnxt_qplib_hwq; MAX_TQM_ALLOC_REQ],
    pub qcount: [u8; MAX_TQM_ALLOC_REQ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_ctx {
    pub qpc_count: u32,
    pub qpc_tbl: bnxt_qplib_hwq,
    pub mrw_count: u32,
    pub mrw_tbl: bnxt_qplib_hwq,
    pub srqc_count: u32,
    pub srqc_tbl: bnxt_qplib_hwq,
    pub cq_count: u32,
    pub cq_tbl: bnxt_qplib_hwq,
    pub tim_tbl: bnxt_qplib_hwq,
    pub tqm_ctx: bnxt_qplib_tqm_ctx,
    pub stats: bnxt_qplib_stats,
    pub stats3: bnxt_qplib_stats,
    pub vf_res: bnxt_qplib_vf_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_res {
    pub pdev: *mut pci_dev,
    pub cctx: *mut bnxt_qplib_chip_ctx,
    pub dattr: *mut bnxt_qplib_dev_attr,
    pub netdev: *mut net_device,
    pub en_dev: *mut bnxt_en_dev,
    pub rcfw: *mut bnxt_qplib_rcfw,
    pub pd_tbl: bnxt_qplib_pd_tbl,
// To protect the pd table bit map
    pub pd_tbl_lock: mutex,
    pub sgid_tbl: bnxt_qplib_sgid_tbl,
    pub dpi_tbl: bnxt_qplib_dpi_tbl,
// To protect the dpi table bit map
    pub dpi_tbl_lock: mutex,
    pub prio: bool,
    pub is_vf: bool,
    pub pacing_data: *mut bnxt_qplib_db_pacing_data,
}

extern "C" {
    pub fn bnxt_qplib_is_chip_gen_p5(bnxt_qplib_is_chip_gen_p7(cctx: cctx) ||) -> return;
}
// pg = (u64)&hwq->pbl_ptr[pg_num];
extern "C" {
    pub fn bnxt_qplib_get_qe(_arg: hwq, _arg: idx, _arg: NULL) -> return;
}

extern "C" {
    pub fn bnxt_qplib_cleanup_res(res: *mut bnxt_qplib_res);
}
extern "C" {
    pub fn bnxt_qplib_init_res(res: *mut bnxt_qplib_res) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_free_res(res: *mut bnxt_qplib_res);
}
extern "C" {
    pub fn bnxt_qplib_alloc_res(res: *mut bnxt_qplib_res, netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_map_db_bar(res: *mut bnxt_qplib_res) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_unmap_db_bar(res: *mut bnxt_qplib_res);
}
extern "C" {
    pub fn bnxt_qplib_determine_atomics(dev: *mut pci_dev) -> c_int;
}
// move prod and update toggle/epoch if wrap around
// move cons and update toggle/epoch if wrap around
// cons += cnt;
// cons %= max_elements;
// dbinfo_flags ^= 1UL << BNXT_QPLIB_FLAG_EPOCH_CONS_SHIFT;

// Index always at 0
// ext stats supported if cap flag is set AND is a PF OR a Thor2 VF

