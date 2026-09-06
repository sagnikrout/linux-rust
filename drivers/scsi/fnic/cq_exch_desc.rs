//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/cq_exch_desc.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

// Exchange completion queue descriptor: 16B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_exch_wq_desc {
    pub completed_index: u16,
    pub q_number: u16,
    pub exchange_id: u16,
    pub tmpl: u8,
    pub reserved0: u8,
    pub reserved1: u32,
    pub exch_status: u8,
    pub reserved2: [u8; 2],
    pub type_color: u8,
}

pub const CQ_EXCH_WQ_STATUS_BITS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cq_exch_status_types {
    CQ_EXCH_WQ_STATUS_TYPE_COMPLETE = 0,
    CQ_EXCH_WQ_STATUS_TYPE_ABORT = 1,
    CQ_EXCH_WQ_STATUS_TYPE_SGL_EOF = 2,
    CQ_EXCH_WQ_STATUS_TYPE_TMPL_ERR = 3,
}

// exch_status = desc_ptr->exch_status & CQ_EXCH_WQ_STATUS_MASK;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_fcp_rq_desc {
    pub completed_index_eop_sop_prt: u16,
    pub q_number: u16,
    pub exchange_id: u16,
    pub tmpl: u16,
    pub bytes_written: u16,
    pub vlan: u16,
    pub sof: u8,
    pub eof: u8,
    pub fcs_fer_fck: u8,
    pub type_color: u8,
}

pub const CQ_FCP_RQ_DESC_TMPL_MASK: c_uint = 0x1f;
pub const CQ_FCP_RQ_DESC_BYTES_WRITTEN_MASK: c_uint = 0x3fff;
pub const CQ_FCP_RQ_DESC_PACKET_ERR_SHIFT: c_int = 14;

pub const CQ_FCP_RQ_DESC_VS_STRIPPED_SHIFT: c_int = 15;

pub const CQ_FCP_RQ_DESC_FC_CRC_OK_MASK: c_uint = 0x1;
pub const CQ_FCP_RQ_DESC_FCOE_ERR_SHIFT: c_int = 1;

pub const CQ_FCP_RQ_DESC_FCS_OK_SHIFT: c_int = 7;

// eop = (desc_ptr->completed_index_eop_sop_prt &
// sop = (desc_ptr->completed_index_eop_sop_prt &
// ingress_port =
// exchange_id = desc_ptr->exchange_id;
// tmpl = desc_ptr->tmpl & CQ_FCP_RQ_DESC_TMPL_MASK;
// bytes_written =
// packet_err =
// vlan_stripped =
// vlan = desc_ptr->vlan;
// sof = desc_ptr->sof;
// fck = desc_ptr->fcs_fer_fck & CQ_FCP_RQ_DESC_FC_CRC_OK_MASK;
// fcoe_err = (desc_ptr->fcs_fer_fck & CQ_FCP_RQ_DESC_FCOE_ERR_MASK) >>
// eof = desc_ptr->eof;
// fcs_ok =
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_sgl_desc {
    pub exchange_id: u16,
    pub q_number: u16,
    pub active_burst_offset: u32,
    pub tot_data_bytes: u32,
    pub tmpl: u16,
    pub sgl_err: u8,
    pub type_color: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cq_sgl_err_types {
    CQ_SGL_ERR_NO_ERROR = 0,
    CQ_SGL_ERR_OVERFLOW,         /* data ran beyond end of SGL */
    CQ_SGL_ERR_SGL_LCL_ADDR_ERR, /* sgl access to local vnic addr illegal*/
    CQ_SGL_ERR_ADDR_RSP_ERR,     /* sgl address error */
    CQ_SGL_ERR_DATA_RSP_ERR,     /* sgl data rsp error */
    CQ_SGL_ERR_CNT_ZERO_ERR,     /* SGL count is 0 */
    CQ_SGL_ERR_CNT_MAX_ERR,      /* SGL count is larger than supported */
    CQ_SGL_ERR_ORDER_ERR,        /* frames recv on both ports, order err */
    CQ_SGL_ERR_DATA_LCL_ADDR_ERR,/* sgl data buf to local vnic addr ill */
    CQ_SGL_ERR_HOST_CQ_ERR,      /* host cq entry to local vnic addr ill */
}

pub const CQ_SGL_SGL_ERR_MASK: c_uint = 0x1f;
pub const CQ_SGL_TMPL_MASK: c_uint = 0x1f;
// Cheat a little by assuming exchange_id is the same as completed
// active_burst_offset = desc_ptr->active_burst_offset;
// tot_data_bytes = desc_ptr->tot_data_bytes;
// tmpl = desc_ptr->tmpl & CQ_SGL_TMPL_MASK;
// sgl_err = desc_ptr->sgl_err & CQ_SGL_SGL_ERR_MASK;
