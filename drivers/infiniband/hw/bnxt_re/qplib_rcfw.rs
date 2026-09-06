//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/qplib_rcfw.h
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
// Description: RDMA Controller HW interface (header)
//

pub const RCFW_CMDQ_TRIG_VAL: c_int = 1;
pub const RCFW_COMM_PCI_BAR_REGION: c_int = 0;
pub const RCFW_COMM_CONS_PCI_BAR_REGION: c_int = 2;
pub const RCFW_COMM_BASE_OFFSET: c_uint = 0x600;
pub const RCFW_PF_VF_COMM_PROD_OFFSET: c_uint = 0xc;
pub const RCFW_COMM_TRIG_OFFSET: c_uint = 0x100;
pub const RCFW_COMM_SIZE: c_uint = 0x104;
pub const RCFW_DBR_PCI_BAR_REGION: c_int = 2;
pub const RCFW_DBR_BASE_PAGE_SHIFT: c_int = 12;
pub const RCFW_FW_STALL_MAX_TIMEOUT: c_int = 40;
// Cmdq contains a fix number of a 16-Byte slots
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cmdqe {
    pub data: [u8; 16],
}

// Shadow queue depth for non blocking command
pub const RCFW_CMD_NON_BLOCKING_SHADOW_QD: c_int = 64;

// CMDQ elements
pub const BNXT_QPLIB_CMDQE_MAX_CNT: c_int = 8192;

// Get the number of command units required for the req. The
// function returns correct value only if called before
// setting using bnxt_qplib_set_cmd_slots
//

pub const RCFW_CMD_IS_BLOCKING: c_uint = 0x8000;
pub const HWRM_VERSION_DEV_ATTR_MAX_DPI: c_uint = 0x1000A0000000DULL;
// HWRM version 1.10.3.18
pub const HWRM_VERSION_READ_CTX: c_uint = 0x1000A00030012;
// Crsq buf is 1024-Byte
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_crsbe {
    pub data: [u8; 1024],
}

// CREQ
// Allocate 1 per QP for async error notification for now

pub const CREQ_ENTRY_POLL_BUDGET: c_uint = 0x100;
// HWQ
extern "C" {
    pub fn int(: *mut *mut aeq_handler_t)(struct bnxt_qplib_rcfw, : *mut c_void, : *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_crsqe {
    pub resp: *mut creq_qp_event,
    pub req_size: u32,
// Free slots at the time of submission
    pub free_slots: u32,
    pub opcode: u8,
    pub is_waiter_alive: bool,
    pub is_internal_cmd: bool,
    pub is_in_used: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_rcfw_sbuf {
    pub sb: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_qp_node {
    pub /: *mut *mut u32 qp_id; / QP id,
    pub /: *mut *mut *mut void qp_handle; / ptr to qplib_qp,
}

pub const BNXT_QPLIB_OOS_COUNT_MASK: c_uint = 0xFFFFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cmdq_mbox {
    pub reg: bnxt_qplib_reg_desc,
    pub prod: *mut void __iomem,
    pub db: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cmdq_ctx {
    pub hwq: bnxt_qplib_hwq,
    pub cmdq_mbox: bnxt_qplib_cmdq_mbox,
    pub waitq: wait_queue_head_t,
    pub flags: c_ulong,
    pub last_seen: c_ulong,
    pub seq_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_creq_db {
    pub reg: bnxt_qplib_reg_desc,
    pub dbinfo: bnxt_qplib_db_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_creq_stat {
    pub creq_qp_event_processed: u64,
    pub creq_func_event_processed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_creq_ctx {
    pub hwq: bnxt_qplib_hwq,
    pub creq_db: bnxt_qplib_creq_db,
    pub stats: bnxt_qplib_creq_stat,
    pub creq_tasklet: tasklet_struct,
    pub aeq_handler: aeq_handler_t,
    pub ring_id: u16,
    pub msix_vec: c_int,
    pub /: *mut *mut bool requested; /irq handler installed,
    pub irq_name: *mut c_char,
}

// RCFW Communication Channels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_rcfw {
    pub pdev: *mut pci_dev,
    pub res: *mut bnxt_qplib_res,
    pub cmdq: bnxt_qplib_cmdq_ctx,
    pub creq: bnxt_qplib_creq_ctx,
    pub crsqe_tbl: *mut bnxt_qplib_crsqe,
    pub qp_tbl_size: c_int,
    pub qp_tbl: *mut bnxt_qplib_qp_node,
// To synchronize the qp-handle hash table
    pub tbl_lock: spinlock_t,
    pub oos_prev: u64,
    pub init_oos_stats: u32,
    pub cmdq_depth: u32,
    pub rcfw_intr_enabled: core::sync::atomic::AtomicI32,
    pub rcfw_inflight: semaphore,
    pub timeout_send: core::sync::atomic::AtomicI32,
// cached from chip cctx for quick reference in slow path
    pub max_timeout: u16,
    pub roce_mirror: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_qplib_cmdqmsg {
    pub req: *mut cmdq_base,
    pub resp: *mut creq_base,
    pub sb: *mut c_void,
    pub req_sz: u32,
    pub res_sz: u32,
    pub block: u8,
}

extern "C" {
    pub fn bnxt_qplib_free_rcfw_channel(rcfw: *mut bnxt_qplib_rcfw);
}
extern "C" {
    pub fn bnxt_qplib_rcfw_stop_irq(rcfw: *mut bnxt_qplib_rcfw, kill: bool);
}
extern "C" {
    pub fn bnxt_qplib_disable_rcfw_channel(rcfw: *mut bnxt_qplib_rcfw);
}
extern "C" {
    pub fn bnxt_qplib_deinit_rcfw(rcfw: *mut bnxt_qplib_rcfw) -> c_int;
}
extern "C" {
    pub fn bnxt_qplib_mark_qp_error(qp_handle: *mut c_void);
}
// Last index of the qp_tbl is for QP1 ie. qp_tbl_size - 1
