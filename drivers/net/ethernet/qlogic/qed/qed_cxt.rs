//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_cxt.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_cxt_info {
    pub p_cxt: *mut c_void,
    pub iid: u32,
    pub type: protocol_type,
}

pub const MAX_TID_BLOCKS: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tid_mem {
    pub tid_size: u32,
    pub num_tids_per_block: u32,
    pub waste: u32,
    pub /: *mut *mut *mut u8 blocks[MAX_TID_BLOCKS]; / 4K,
}

//
// qed_cxt_get_cid_info(): Returns the context info for a specific cidi.
//
// @p_hwfn: HW device data.
// @p_info: In/out.
//
// Return: Int.
//
// qed_cxt_get_tid_mem_info(): Returns the tid mem info.
//
// @p_hwfn: HW device data.
// @p_info: in/out.
//
// Return: int.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_cxt_elem_type {
    QED_ELEM_CXT,
    QED_ELEM_SRQ,
    QED_ELEM_TASK,
    QED_ELEM_XRC_SRQ,
}

//
// qed_cxt_set_pf_params(): Set the PF params for cxt init.
//
// @p_hwfn: HW device data.
// @rdma_tasks: Requested maximum.
//
// Return: int.
//
extern "C" {
    pub fn qed_cxt_set_pf_params(p_hwfn: *mut qed_hwfn, rdma_tasks: u32) -> c_int;
}
//
// qed_cxt_cfg_ilt_compute(): Compute ILT init parameters.
//
// @p_hwfn: HW device data.
// @last_line: Last_line.
//
// Return: Int
//
extern "C" {
    pub fn qed_cxt_cfg_ilt_compute(p_hwfn: *mut qed_hwfn, last_line: *mut u32) -> c_int;
}
//
// qed_cxt_cfg_ilt_compute_excess(): How many lines can be decreased.
//
// @p_hwfn: HW device data.
// @used_lines: Used lines.
//
// Return: Int.
//
extern "C" {
    pub fn qed_cxt_cfg_ilt_compute_excess(p_hwfn: *mut qed_hwfn, used_lines: u32) -> u32;
}
//
// qed_cxt_mngr_alloc(): Allocate and init the context manager struct.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_cxt_mngr_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_cxt_mngr_free() - Context manager free.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_cxt_mngr_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_cxt_tables_alloc(): Allocate ILT shadow, Searcher T2, acquired map.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_cxt_tables_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_cxt_mngr_setup(): Reset the acquired CIDs.
//
// @p_hwfn: HW device data.
//
extern "C" {
    pub fn qed_cxt_mngr_setup(p_hwfn: *mut qed_hwfn);
}
//
// qed_cxt_hw_init_common(): Initailze ILT and DQ, common phase, per path.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_cxt_hw_init_common(p_hwfn: *mut qed_hwfn);
}
//
// qed_cxt_hw_init_pf(): Initailze ILT and DQ, PF phase, per path.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
extern "C" {
    pub fn qed_cxt_hw_init_pf(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt);
}
//
// qed_qm_init_pf(): Initailze the QM PF phase, per path.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @is_pf_loading: Is pf pending.
//
// Return: Void.
//
// qed_qm_reconf(): Reconfigures QM pf on the fly.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
extern "C" {
    pub fn qed_qm_reconf(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}

//
// qed_cxt_release_cid(): Release a cid.
//
// @p_hwfn: HW device data.
// @cid: Cid.
//
// Return: Void.
//
extern "C" {
    pub fn qed_cxt_release_cid(p_hwfn: *mut qed_hwfn, cid: u32);
}
//
// _qed_cxt_release_cid(): Release a cid belonging to a vf-queue.
//
// @p_hwfn: HW device data.
// @cid: Cid.
// @vfid: Engine relative index. QED_CXT_PF_CID if belongs to PF.
//
// Return: Void.
//
extern "C" {
    pub fn _qed_cxt_release_cid(p_hwfn: *mut qed_hwfn, cid: u32, vfid: u8);
}
//
// qed_cxt_acquire_cid(): Acquire a new cid of a specific protocol type.
//
// @p_hwfn: HW device data.
// @type: Type.
// @p_cid: Pointer cid.
//
// Return: Int.
//
// _qed_cxt_acquire_cid(): Acquire a new cid of a specific protocol type
// for a vf-queue.
//
// @p_hwfn: HW device data.
// @type: Type.
// @p_cid: Pointer cid.
// @vfid: Engine relative index. QED_CXT_PF_CID if belongs to PF.
//
// Return: Int.
//
extern "C" {
    pub fn qed_cxt_free_proto_ilt(p_hwfn: *mut qed_hwfn, proto: protocol_type) -> c_int;
}
pub const QED_CTX_WORKING_MEM: c_int = 0;
pub const QED_CTX_FL_MEM: c_int = 1;
// Max number of connection types in HW (DQ/CDU etc.)

pub const NUM_TASK_TYPES: c_int = 2;
pub const NUM_TASK_PF_SEGMENTS: c_int = 4;
pub const NUM_TASK_VF_SEGMENTS: c_int = 1;
// PF per protocl configuration object

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tid_seg {
    pub count: u32,
    pub type: u8,
    pub has_fl_mem: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_conn_type_cfg {
    pub cid_count: u32,
    pub cids_per_vf: u32,
    pub tid_seg: [qed_tid_seg; TASK_SEGMENTS],
}

// ILT Client configuration,
// Per connection type (protocol) resources (cids, tis, vf cids etc.)
// 1 - for connection context (CDUC) and for each task context we need two
// values, for regular task context and for force load memory
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ilt_cfg_pair {
    pub reg: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ilt_cli_blk {
    pub /: *mut *mut u32 total_size; / 0 means not active,
    pub real_size_in_page: u32,
    pub start_line: u32,
    pub dynamic_line_offset: u32,
    pub dynamic_line_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ilt_client_cfg {
    pub active: bool,
// ILT boundaries
    pub first: ilt_cfg_pair,
    pub last: ilt_cfg_pair,
    pub p_size: ilt_cfg_pair,
// ILT client blocks for PF
    pub pf_blks: [qed_ilt_cli_blk; ILT_CLI_PF_BLOCKS],
    pub pf_total_lines: u32,
// ILT client blocks for VFs
    pub vf_blks: [qed_ilt_cli_blk; ILT_CLI_VF_BLOCKS],
    pub vf_total_lines: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_cid_acquired_map {
    pub start_cid: u32,
    pub max_count: u32,
    pub cid_map: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_src_t2 {
    pub dma_mem: *mut phys_mem_desc,
    pub num_pages: u32,
    pub first_free: u64,
    pub last_free: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_cxt_mngr {
// Per protocl configuration
    pub conn_cfg: [qed_conn_type_cfg; MAX_CONN_TYPES],
// computed ILT structure
    pub clients: [qed_ilt_client_cfg; MAX_ILT_CLIENTS],
// Task type sizes
    pub task_type_size: [u32; NUM_TASK_TYPES],
// total number of VFs for this hwfn -
// ALL VFs are symmetric in terms of HW resources
//
    pub vf_count: u32,
    pub first_vf_in_pf: u32,
// Acquired CIDs
    pub acquired: [qed_cid_acquired_map; MAX_CONN_TYPES],
// ILT  shadow table
    pub ilt_shadow: *mut phys_mem_desc,
    pub ilt_shadow_size: u32,
    pub pf_start_line: u32,
// Mutex for a dynamic ILT allocation
    pub mutex: mutex,
// SRC T2
    pub src_t2: qed_src_t2,
// total number of SRQ's for this hwfn
    pub srq_count: u32,
    pub xrc_srq_count: u32,
// Maximal number of L2 steering filters
    pub arfs_count: u32,
    pub iscsi_task_pages: u16,
    pub fcoe_task_pages: u16,
    pub roce_task_pages: u16,
    pub eth_task_pages: u16,
    pub task_ctx_size: u16,
    pub conn_ctx_size: u16,
}

extern "C" {
    pub fn qed_get_cdut_num_pf_init_pages(p_hwfn: *mut qed_hwfn) -> u16;
}
extern "C" {
    pub fn qed_get_cdut_num_vf_init_pages(p_hwfn: *mut qed_hwfn) -> u16;
}
extern "C" {
    pub fn qed_get_cdut_num_pf_work_pages(p_hwfn: *mut qed_hwfn) -> u16;
}
extern "C" {
    pub fn qed_get_cdut_num_vf_work_pages(p_hwfn: *mut qed_hwfn) -> u16;
}
extern "C" {
    pub fn qed_cxt_get_total_srq_count(p_hwfn: *mut qed_hwfn) -> u32;
}
