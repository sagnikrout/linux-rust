//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/ib_verbs.h
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
// Description: IB Verbs interpreter (header)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_gid_ctx {
    pub idx: u32,
    pub refcnt: u32,
}

pub const BNXT_RE_FENCE_BYTES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_fence_data {
    pub size: u32,
    pub va: [u8; BNXT_RE_FENCE_BYTES],
    pub dma_addr: dma_addr_t,
    pub mr: *mut bnxt_re_mr,
    pub mw: *mut ib_mw,
    pub bind_wqe: bnxt_qplib_swqe,
    pub bind_rkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_pd {
    pub ib_pd: ib_pd,
    pub rdev: *mut bnxt_re_dev,
    pub qplib_pd: bnxt_qplib_pd,
    pub fence: bnxt_re_fence_data,
    pub pd_db_mmap: *mut rdma_user_mmap_entry,
    pub pd_wcdb_mmap: *mut rdma_user_mmap_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_ah {
    pub ib_ah: ib_ah,
    pub rdev: *mut bnxt_re_dev,
    pub qplib_ah: bnxt_qplib_ah,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_srq {
    pub ib_srq: ib_srq,
    pub rdev: *mut bnxt_re_dev,
    pub srq_limit: u32,
    pub qplib_srq: bnxt_qplib_srq,
    pub umem: *mut ib_umem,
    pub /: *mut *mut spinlock_t lock; / protect srq,
    pub uctx_srq_page: *mut c_void,
    pub toggle_entry: *mut bnxt_re_user_mmap_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_qp {
    pub ib_qp: ib_qp,
    pub list: list_head,
    pub rdev: *mut bnxt_re_dev,
    pub /: *mut *mut spinlock_t sq_lock; / protect sq,
    pub /: *mut *mut spinlock_t rq_lock; / protect rq,
    pub qplib_qp: bnxt_qplib_qp,
    pub sumem: *mut ib_umem,
    pub rumem: *mut ib_umem,
// QP1
    pub send_psn: u32,
    pub qp1_hdr: ib_ud_header,
    pub scq: *mut bnxt_re_cq,
    pub rcq: *mut bnxt_re_cq,
    pub dentry: *mut dentry,
    pub /: *mut *mut *mut bnxt_re_dbr_obj dbr_obj; / doorbell region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_cq {
    pub ib_cq: ib_cq,
    pub rdev: *mut bnxt_re_dev,
    pub /: *mut *mut spinlock_t cq_lock; / protect cq,
    pub cq_count: u16,
    pub cq_period: u16,
    pub qplib_cq: bnxt_qplib_cq,
    pub cql: *mut bnxt_qplib_cqe,
pub const MAX_CQL_PER_POLL: c_int = 1024;
    pub max_cql: u32,
    pub umem: *mut ib_umem,
    pub resize_umem: *mut ib_umem,
    pub resize_cqe: c_int,
    pub uctx_cq_page: *mut c_void,
    pub toggle_entry: *mut bnxt_re_user_mmap_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_mr {
    pub rdev: *mut bnxt_re_dev,
    pub ib_mr: ib_mr,
    pub ib_umem: *mut ib_umem,
    pub qplib_mr: bnxt_qplib_mrw,
    pub npages: u32,
    pub pages: *mut u64,
    pub qplib_frpl: bnxt_qplib_frpl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_frpl {
    pub rdev: *mut bnxt_re_dev,
    pub qplib_frpl: bnxt_qplib_frpl,
    pub page_list: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_mw {
    pub rdev: *mut bnxt_re_dev,
    pub ib_mw: ib_mw,
    pub qplib_mw: bnxt_qplib_mrw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_ucontext {
    pub ib_uctx: ib_ucontext,
    pub rdev: *mut bnxt_re_dev,
    pub dpi: bnxt_qplib_dpi,
    pub wcdpi: bnxt_qplib_dpi,
    pub /: *mut *mut mutex wcdpi_lock; / serialises WC DPI alloc/free,
    pub shpg: *mut c_void,
    pub /: *mut *mut spinlock_t sh_lock; / protect shpg,
    pub shpage_mmap: *mut rdma_user_mmap_entry,
    pub /: *mut *mut xarray cq_xa; / cqid → ib_uobject, per-context toggle page lookup,
    pub /: *mut *mut xarray srq_xa; / srqid → ib_uobject, per-context toggle page lookup,
    pub cmask: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_mmap_flag {
    BNXT_RE_MMAP_SH_PAGE,
    BNXT_RE_MMAP_UC_DB,
    BNXT_RE_MMAP_WC_DB,
    BNXT_RE_MMAP_DBR_PAGE,
    BNXT_RE_MMAP_DBR_BAR,
    BNXT_RE_MMAP_TOGGLE_PAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_user_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub uctx: *mut bnxt_re_ucontext,
    pub mem_offset: u64,
    pub mmap_flag: u8,
    pub dpi_valid: bool,
    pub dpi: bnxt_qplib_dpi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_dbr_obj {
    pub rdev: *mut bnxt_re_dev,
    pub dpi: bnxt_qplib_dpi,
    pub entry: *mut bnxt_re_user_mmap_entry,
    pub /: *mut *mut kref usecnt; / 1 (uobject) + n (QPs using this dbr),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_flow {
    pub ib_flow: ib_flow,
    pub rdev: *mut bnxt_re_dev,
}

extern "C" {
    pub fn sizeof(sq_sge: *mut *mut sq_send_hdr) + nsge  sizeof(struct) -> return;
}
extern "C" {
    pub fn sizeof(sq_sge): *mut *mut rq_wqe_hdr) + (nsge  sizeof(struct) -> return;
}
extern "C" {
    pub fn min(_arg: roundup_pow_of_two(ent), _arg: max) -> return;
}
extern "C" {
    pub fn bnxt_re_query_fw_str(ibdev: *mut ib_device, str: *mut c_char);
}
extern "C" {
    pub fn bnxt_re_del_gid(attr: *const ib_gid_attr, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bnxt_re_add_gid(attr: *const ib_gid_attr, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bnxt_re_alloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn bnxt_re_dealloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn bnxt_re_query_ah(ah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn bnxt_re_destroy_ah(ah: *mut ib_ah, flags: u32) -> c_int;
}
extern "C" {
    pub fn bnxt_re_query_srq(srq: *mut ib_srq, srq_attr: *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn bnxt_re_destroy_srq(srq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn bnxt_re_destroy_qp(qp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn bnxt_re_destroy_cq(cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn bnxt_re_poll_cq(cq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn bnxt_re_req_notify_cq(cq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn bnxt_re_dereg_mr(mr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn bnxt_re_dealloc_mw(mw: *mut ib_mw) -> c_int;
}
extern "C" {
    pub fn bnxt_re_alloc_ucontext(ctx: *mut ib_ucontext, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn bnxt_re_dealloc_ucontext(context: *mut ib_ucontext);
}
extern "C" {
    pub fn bnxt_re_destroy_flow(flow_id: *mut ib_flow) -> c_int;
}
extern "C" {
    pub fn bnxt_re_mmap(context: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn bnxt_re_mmap_free(rdma_entry: *mut rdma_user_mmap_entry);
}
extern "C" {
    pub fn bnxt_re_lock_cqs(qp: *mut bnxt_re_qp) -> c_ulong;
}
extern "C" {
    pub fn bnxt_re_unlock_cqs(qp: *mut bnxt_re_qp, flags: c_ulong);
}
extern "C" {
    pub fn bnxt_re_dbr_kref_release(ref: *mut kref);
}
