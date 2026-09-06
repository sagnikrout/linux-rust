//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_qp_ctxt.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_SQ_CTXT_CEQ_ATTR_GLOBAL_SQ_ID_SHIFT: c_int = 13;
pub const HINIC_SQ_CTXT_CEQ_ATTR_EN_SHIFT: c_int = 23;
pub const HINIC_SQ_CTXT_CEQ_ATTR_GLOBAL_SQ_ID_MASK: c_uint = 0x3FF;
pub const HINIC_SQ_CTXT_CEQ_ATTR_EN_MASK: c_uint = 0x1;

pub const HINIC_SQ_CTXT_CI_IDX_SHIFT: c_int = 11;
pub const HINIC_SQ_CTXT_CI_WRAPPED_SHIFT: c_int = 23;
pub const HINIC_SQ_CTXT_CI_IDX_MASK: c_uint = 0xFFF;
pub const HINIC_SQ_CTXT_CI_WRAPPED_MASK: c_uint = 0x1;

pub const HINIC_SQ_CTXT_WQ_PAGE_HI_PFN_SHIFT: c_int = 0;
pub const HINIC_SQ_CTXT_WQ_PAGE_PI_SHIFT: c_int = 20;
pub const HINIC_SQ_CTXT_WQ_PAGE_HI_PFN_MASK: c_uint = 0xFFFFF;
pub const HINIC_SQ_CTXT_WQ_PAGE_PI_MASK: c_uint = 0xFFF;

pub const HINIC_SQ_CTXT_PREF_CACHE_THRESHOLD_SHIFT: c_int = 0;
pub const HINIC_SQ_CTXT_PREF_CACHE_MAX_SHIFT: c_int = 14;
pub const HINIC_SQ_CTXT_PREF_CACHE_MIN_SHIFT: c_int = 25;
pub const HINIC_SQ_CTXT_PREF_CACHE_THRESHOLD_MASK: c_uint = 0x3FFF;
pub const HINIC_SQ_CTXT_PREF_CACHE_MAX_MASK: c_uint = 0x7FF;
pub const HINIC_SQ_CTXT_PREF_CACHE_MIN_MASK: c_uint = 0x7F;
pub const HINIC_SQ_CTXT_PREF_WQ_HI_PFN_SHIFT: c_int = 0;
pub const HINIC_SQ_CTXT_PREF_CI_SHIFT: c_int = 20;
pub const HINIC_SQ_CTXT_PREF_WQ_HI_PFN_MASK: c_uint = 0xFFFFF;
pub const HINIC_SQ_CTXT_PREF_CI_MASK: c_uint = 0xFFF;

pub const HINIC_SQ_CTXT_WQ_BLOCK_HI_PFN_SHIFT: c_int = 0;
pub const HINIC_SQ_CTXT_WQ_BLOCK_HI_PFN_MASK: c_uint = 0x7FFFFF;

pub const HINIC_RQ_CTXT_CEQ_ATTR_EN_SHIFT: c_int = 0;
pub const HINIC_RQ_CTXT_CEQ_ATTR_WRAPPED_SHIFT: c_int = 1;
pub const HINIC_RQ_CTXT_CEQ_ATTR_EN_MASK: c_uint = 0x1;
pub const HINIC_RQ_CTXT_CEQ_ATTR_WRAPPED_MASK: c_uint = 0x1;

pub const HINIC_RQ_CTXT_PI_IDX_SHIFT: c_int = 0;
pub const HINIC_RQ_CTXT_PI_INTR_SHIFT: c_int = 22;
pub const HINIC_RQ_CTXT_PI_IDX_MASK: c_uint = 0xFFF;
pub const HINIC_RQ_CTXT_PI_INTR_MASK: c_uint = 0x3FF;

pub const HINIC_RQ_CTXT_WQ_PAGE_HI_PFN_SHIFT: c_int = 0;
pub const HINIC_RQ_CTXT_WQ_PAGE_CI_SHIFT: c_int = 20;
pub const HINIC_RQ_CTXT_WQ_PAGE_HI_PFN_MASK: c_uint = 0xFFFFF;
pub const HINIC_RQ_CTXT_WQ_PAGE_CI_MASK: c_uint = 0xFFF;

pub const HINIC_RQ_CTXT_PREF_CACHE_THRESHOLD_SHIFT: c_int = 0;
pub const HINIC_RQ_CTXT_PREF_CACHE_MAX_SHIFT: c_int = 14;
pub const HINIC_RQ_CTXT_PREF_CACHE_MIN_SHIFT: c_int = 25;
pub const HINIC_RQ_CTXT_PREF_CACHE_THRESHOLD_MASK: c_uint = 0x3FFF;
pub const HINIC_RQ_CTXT_PREF_CACHE_MAX_MASK: c_uint = 0x7FF;
pub const HINIC_RQ_CTXT_PREF_CACHE_MIN_MASK: c_uint = 0x7F;
pub const HINIC_RQ_CTXT_PREF_WQ_HI_PFN_SHIFT: c_int = 0;
pub const HINIC_RQ_CTXT_PREF_CI_SHIFT: c_int = 20;
pub const HINIC_RQ_CTXT_PREF_WQ_HI_PFN_MASK: c_uint = 0xFFFFF;
pub const HINIC_RQ_CTXT_PREF_CI_MASK: c_uint = 0xFFF;

pub const HINIC_RQ_CTXT_WQ_BLOCK_HI_PFN_SHIFT: c_int = 0;
pub const HINIC_RQ_CTXT_WQ_BLOCK_HI_PFN_MASK: c_uint = 0x7FFFFF;

pub const HINIC_WQ_PAGE_PFN_SHIFT: c_int = 12;
pub const HINIC_WQ_BLOCK_PFN_SHIFT: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_qp_ctxt_type {
    HINIC_QP_CTXT_TYPE_SQ,
    HINIC_QP_CTXT_TYPE_RQ
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_qp_ctxt_header {
    pub num_queues: u16,
    pub queue_type: u16,
    pub addr_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sq_ctxt {
    pub ceq_attr: u32,
    pub ci_wrapped: u32,
    pub wq_hi_pfn_pi: u32,
    pub wq_lo_pfn: u32,
    pub pref_cache: u32,
    pub pref_wrapped: u32,
    pub pref_wq_hi_pfn_ci: u32,
    pub pref_wq_lo_pfn: u32,
    pub rsvd0: u32,
    pub rsvd1: u32,
    pub wq_block_hi_pfn: u32,
    pub wq_block_lo_pfn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_ctxt {
    pub ceq_attr: u32,
    pub pi_intr_attr: u32,
    pub wq_hi_pfn_ci: u32,
    pub wq_lo_pfn: u32,
    pub pref_cache: u32,
    pub pref_wrapped: u32,
    pub pref_wq_hi_pfn_ci: u32,
    pub pref_wq_lo_pfn: u32,
    pub pi_paddr_hi: u32,
    pub pi_paddr_lo: u32,
    pub wq_block_hi_pfn: u32,
    pub wq_block_lo_pfn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_clean_queue_ctxt {
    pub cmdq_hdr: hinic_qp_ctxt_header,
    pub ctxt_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sq_ctxt_block {
    pub hdr: hinic_qp_ctxt_header,
    pub sq_ctxt: [hinic_sq_ctxt; HINIC_Q_CTXT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_ctxt_block {
    pub hdr: hinic_qp_ctxt_header,
    pub rq_ctxt: [hinic_rq_ctxt; HINIC_Q_CTXT_MAX],
}
