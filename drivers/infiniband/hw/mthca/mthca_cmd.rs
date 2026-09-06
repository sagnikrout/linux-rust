//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mthca/mthca_cmd.h
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
// Copyright (c) 2004, 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
// Copyright (c) 2006 Cisco Systems.  All rights reserved.
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
// disclaimer in the documentation and/or other materials
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

pub const MTHCA_MAILBOX_SIZE: c_int = 4096;
// command completed successfully:
// Internal error (such as a bus error) occurred while processing command:
// Operation/command not supported or opcode modifier not supported:
// Parameter not supported or parameter out of range:
// System not enabled or bad system state:
// Attempt to access reserved or unallocaterd resource:
// Requested resource is currently executing a command, or is otherwise busy:
// memory error:
// Required capability exceeds device limits:
// Resource is not in the appropriate state or ownership:
// Index out of range:
// FW image corrupted:
// Attempt to modify a QP/EE which is not in the presumed state:
// Bad segment parameters (Address/Size):
// Memory Region has Memory Windows bound to:
// HCA local attached memory not present:
// Bad management packet (silently discarded):
// More outstanding CQEs in CQ than new CQ size:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_mailbox {
    pub dma: dma_addr_t,
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_dev_lim {
    pub max_srq_sz: c_int,
    pub max_qp_sz: c_int,
    pub reserved_qps: c_int,
    pub max_qps: c_int,
    pub reserved_srqs: c_int,
    pub max_srqs: c_int,
    pub reserved_eecs: c_int,
    pub max_eecs: c_int,
    pub max_cq_sz: c_int,
    pub reserved_cqs: c_int,
    pub max_cqs: c_int,
    pub max_mpts: c_int,
    pub reserved_eqs: c_int,
    pub max_eqs: c_int,
    pub reserved_mtts: c_int,
    pub max_mrw_sz: c_int,
    pub reserved_mrws: c_int,
    pub max_mtt_seg: c_int,
    pub max_requester_per_qp: c_int,
    pub max_responder_per_qp: c_int,
    pub max_rdma_global: c_int,
    pub local_ca_ack_delay: c_int,
    pub max_mtu: c_int,
    pub max_port_width: c_int,
    pub max_vl: c_int,
    pub num_ports: c_int,
    pub max_gids: c_int,
    pub stat_rate_support: u16,
    pub max_pkeys: c_int,
    pub flags: u32,
    pub reserved_uars: c_int,
    pub uar_size: c_int,
    pub min_page_sz: c_int,
    pub max_sg: c_int,
    pub max_desc_sz: c_int,
    pub max_qp_per_mcg: c_int,
    pub reserved_mgms: c_int,
    pub max_mcgs: c_int,
    pub reserved_pds: c_int,
    pub max_pds: c_int,
    pub reserved_rdds: c_int,
    pub max_rdds: c_int,
    pub eec_entry_sz: c_int,
    pub qpc_entry_sz: c_int,
    pub eeec_entry_sz: c_int,
    pub eqpc_entry_sz: c_int,
    pub eqc_entry_sz: c_int,
    pub cqc_entry_sz: c_int,
    pub srq_entry_sz: c_int,
    pub uar_scratch_entry_sz: c_int,
    pub mpt_entry_sz: c_int,
    pub max_avs: c_int,
    pub tavor: },
    pub resize_srq: c_int,
    pub max_pbl_sz: c_int,
    pub bmme_flags: u8,
    pub reserved_lkey: u32,
    pub lam_required: c_int,
    pub max_icm_sz: u64,
    pub arbel: },
    pub hca: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_adapter {
    pub vendor_id: u32,
    pub device_id: u32,
    pub revision_id: u32,
    pub board_id: [c_char; MTHCA_BOARD_ID_LEN],
    pub inta_pin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_init_hca_param {
    pub qpc_base: u64,
    pub eec_base: u64,
    pub srqc_base: u64,
    pub cqc_base: u64,
    pub eqpc_base: u64,
    pub eeec_base: u64,
    pub eqc_base: u64,
    pub rdb_base: u64,
    pub mc_base: u64,
    pub mpt_base: u64,
    pub mtt_base: u64,
    pub uar_scratch_base: u64,
    pub uarc_base: u64,
    pub log_mc_entry_sz: u16,
    pub mc_hash_sz: u16,
    pub log_num_qps: u8,
    pub log_num_eecs: u8,
    pub log_num_srqs: u8,
    pub log_num_cqs: u8,
    pub log_num_eqs: u8,
    pub log_mc_table_sz: u8,
    pub mtt_seg_sz: u8,
    pub log_mpt_sz: u8,
    pub log_uar_sz: u8,
    pub log_uarc_sz: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_init_ib_param {
    pub port_width: c_int,
    pub vl_cap: c_int,
    pub mtu_cap: c_int,
    pub gid_cap: u16,
    pub pkey_cap: u16,
    pub set_guid0: c_int,
    pub guid0: u64,
    pub set_node_guid: c_int,
    pub node_guid: u64,
    pub set_si_guid: c_int,
    pub si_guid: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_set_ib_param {
    pub set_si_guid: c_int,
    pub reset_qkey_viol: c_int,
    pub si_guid: u64,
    pub cap_mask: u32,
}

extern "C" {
    pub fn mthca_cmd_init(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_cmd_cleanup(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cmd_use_events(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_cmd_use_polling(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_free_mailbox(dev: *mut mthca_dev, mailbox: *mut mthca_mailbox);
}
extern "C" {
    pub fn mthca_SYS_EN(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_SYS_DIS(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_MAP_FA(dev: *mut mthca_dev, icm: *mut mthca_icm) -> c_int;
}
extern "C" {
    pub fn mthca_UNMAP_FA(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_RUN_FW(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_QUERY_FW(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_ENABLE_LAM(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_DISABLE_LAM(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_QUERY_DDR(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_CLOSE_IB(dev: *mut mthca_dev, port: c_int) -> c_int;
}
extern "C" {
    pub fn mthca_CLOSE_HCA(dev: *mut mthca_dev, panic: c_int) -> c_int;
}
extern "C" {
    pub fn mthca_MAP_ICM(dev: *mut mthca_dev, icm: *mut mthca_icm, virt: u64) -> c_int;
}
extern "C" {
    pub fn mthca_MAP_ICM_page(dev: *mut mthca_dev, dma_addr: u64, virt: u64) -> c_int;
}
extern "C" {
    pub fn mthca_UNMAP_ICM(dev: *mut mthca_dev, virt: u64, page_count: u32) -> c_int;
}
extern "C" {
    pub fn mthca_MAP_ICM_AUX(dev: *mut mthca_dev, icm: *mut mthca_icm) -> c_int;
}
extern "C" {
    pub fn mthca_UNMAP_ICM_AUX(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_SET_ICM_SIZE(dev: *mut mthca_dev, icm_size: u64, aux_pages: *mut u64) -> c_int;
}
extern "C" {
    pub fn mthca_SYNC_TPT(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_RESIZE_CQ(dev: *mut mthca_dev, cq_num: c_int, lkey: u32, log_size: u8) -> c_int;
}
extern "C" {
    pub fn mthca_ARM_SRQ(dev: *mut mthca_dev, srq_num: c_int, limit: c_int) -> c_int;
}
extern "C" {
    pub fn mthca_CONF_SPECIAL_QP(dev: *mut mthca_dev, type: c_int, qpn: u32) -> c_int;
}
extern "C" {
    pub fn mthca_NOP(dev: *mut mthca_dev) -> c_int;
}
