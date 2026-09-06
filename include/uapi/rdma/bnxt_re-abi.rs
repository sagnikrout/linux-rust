//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/bnxt_re-abi.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause)
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
// Description: Uverbs ABI header file
//

pub const BNXT_RE_ABI_VERSION: c_int = 1;
pub const BNXT_RE_CHIP_ID0_CHIP_NUM_SFT: c_uint = 0x00;
pub const BNXT_RE_CHIP_ID0_CHIP_REV_SFT: c_uint = 0x10;
pub const BNXT_RE_CHIP_ID0_CHIP_MET_SFT: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_wqe_mode {
    BNXT_QPLIB_WQE_MODE_STATIC	= 0x00,
    BNXT_QPLIB_WQE_MODE_VARIABLE	= 0x01,
    BNXT_QPLIB_WQE_MODE_INVALID	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_uctx_req {
    pub comp_mask: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_uctx_resp {
    pub dev_id: __u32,
    pub max_qp: __u32,
    pub pg_size: __u32,
    pub cqe_sz: __u32,
    pub max_cqd: __u32,
    pub rsvd: __u32,
    pub comp_mask: __aligned_u64,
    pub chip_id0: __u32,
    pub chip_id1: __u32,
    pub mode: __u32,
    pub /: *mut *mut __u32 rsvd1; / padding,
}

//
// This struct is placed after the ib_uverbs_alloc_pd_resp struct, which is
// not 8 byted aligned. To avoid undesired padding in various cases we have to
// set this struct to packed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_pd_resp {
    pub pdid: __u32,
    pub dpi: __u32,
    pub dbr: __u64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_cq_req {
    pub cq_va: __aligned_u64,
    pub cq_handle: __aligned_u64,
    pub comp_mask: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_resp_cq_mask {
    BNXT_RE_CQ_TOGGLE_PAGE_SUPPORT = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_req_cq_mask {
    BNXT_RE_CQ_FIXED_NUM_CQE_ENABLE = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_cq_resp {
    pub cqid: __u32,
    pub tail: __u32,
    pub phase: __u32,
    pub rsvd: __u32,
    pub comp_mask: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_resize_cq_req {
    pub cq_va: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_qp_mask {
    BNXT_RE_QP_REQ_MASK_FIXED_QUE_ATTR = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_qp_req {
    pub qpsva: __aligned_u64,
    pub qprva: __aligned_u64,
    pub qp_handle: __aligned_u64,
    pub comp_mask: __aligned_u64,
    pub sq_slots: __u32,
    pub sq_npsn: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_create_qp_attrs {
    BNXT_RE_CREATE_QP_ATTR_DBR_HANDLE = UVERBS_ID_DRIVER_NS_WITH_UHW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_qp_resp {
    pub qpid: __u32,
    pub rsvd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_srq_req {
    pub srqva: __aligned_u64,
    pub srq_handle: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_srq_mask {
    BNXT_RE_SRQ_TOGGLE_PAGE_SUPPORT = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_srq_resp {
    pub srqid: __u32,
    pub /: *mut *mut __u32 rsvd; / padding,
    pub comp_mask: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_shpg_offt {
    BNXT_RE_BEG_RESV_OFFT	= 0x00,
    BNXT_RE_AVID_OFFT	= 0x10,
    BNXT_RE_AVID_SIZE	= 0x04,
    BNXT_RE_END_RESV_OFFT	= 0xFF0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_objects {
    BNXT_RE_OBJECT_ALLOC_PAGE = (1U << UVERBS_ID_NS_SHIFT),
    BNXT_RE_OBJECT_NOTIFY_DRV,
    BNXT_RE_OBJECT_GET_TOGGLE_MEM,
    BNXT_RE_OBJECT_DBR,
    BNXT_RE_OBJECT_DEFAULT_DBR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_alloc_page_type {
    BNXT_RE_ALLOC_WC_PAGE = 0,
    BNXT_RE_ALLOC_DBR_BAR_PAGE,
    BNXT_RE_ALLOC_DBR_PAGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_var_alloc_page_attrs {
    BNXT_RE_ALLOC_PAGE_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    BNXT_RE_ALLOC_PAGE_TYPE,
    BNXT_RE_ALLOC_PAGE_DPI,
    BNXT_RE_ALLOC_PAGE_MMAP_OFFSET,
    BNXT_RE_ALLOC_PAGE_MMAP_LENGTH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_alloc_page_attrs {
    BNXT_RE_DESTROY_PAGE_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_alloc_page_methods {
    BNXT_RE_METHOD_ALLOC_PAGE = (1U << UVERBS_ID_NS_SHIFT),
    BNXT_RE_METHOD_DESTROY_PAGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_notify_drv_methods {
    BNXT_RE_METHOD_NOTIFY_DRV = (1U << UVERBS_ID_NS_SHIFT),
}

// Toggle mem
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_get_toggle_mem_type {
    BNXT_RE_CQ_TOGGLE_MEM = 0,
    BNXT_RE_SRQ_TOGGLE_MEM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_var_toggle_mem_attrs {
    BNXT_RE_TOGGLE_MEM_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    BNXT_RE_TOGGLE_MEM_TYPE,
    BNXT_RE_TOGGLE_MEM_RES_ID,
    BNXT_RE_TOGGLE_MEM_MMAP_PAGE,
    BNXT_RE_TOGGLE_MEM_MMAP_OFFSET,
    BNXT_RE_TOGGLE_MEM_MMAP_LENGTH,
    BNXT_RE_TOGGLE_MEM_CQ_HANDLE,
    BNXT_RE_TOGGLE_MEM_SRQ_HANDLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_toggle_mem_attrs {
    BNXT_RE_RELEASE_TOGGLE_MEM_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_toggle_mem_methods {
    BNXT_RE_METHOD_GET_TOGGLE_MEM = (1U << UVERBS_ID_NS_SHIFT),
    BNXT_RE_METHOD_RELEASE_TOGGLE_MEM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_packet_pacing_caps {
    pub qp_rate_limit_min: __u32,
    pub /: *mut *mut __u32 qp_rate_limit_max; / In kbps,
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported, e.g.
// supported_qpts |= 1 << IB_QPT_RC
//
    pub supported_qpts: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_query_device_ex_resp {
    pub packet_pacing_caps: bnxt_re_packet_pacing_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_db_region {
    pub dpi: __u32,
    pub reserved: __u32,
    pub umdbr: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_obj_dbr_alloc_attrs {
    BNXT_RE_ALLOC_DBR_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
    BNXT_RE_ALLOC_DBR_ATTR,
    BNXT_RE_ALLOC_DBR_OFFSET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_obj_dbr_free_attrs {
    BNXT_RE_FREE_DBR_HANDLE = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_obj_default_dbr_attrs {
    BNXT_RE_DEFAULT_DBR_ATTR = (1U << UVERBS_ID_NS_SHIFT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_obj_dpi_methods {
    BNXT_RE_METHOD_DBR_ALLOC = (1U << UVERBS_ID_NS_SHIFT),
    BNXT_RE_METHOD_DBR_FREE,
    BNXT_RE_METHOD_GET_DEFAULT_DBR,
}
