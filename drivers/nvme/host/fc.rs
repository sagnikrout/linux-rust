//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvme/host/fc.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2016, Avago Technologies
//
pub const _NVME_FC_TRANSPORT_H: c_int = 1;
//
// Common definitions between the nvme_fc (host) transport and
// nvmet_fc (target) transport implementation.
//
// ******************  FC-NVME LS HANDLING
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmefc_ls_requests {
    pub w0: fcnvme_ls_rqst_w0,
    pub rq_cr_assoc: fcnvme_ls_cr_assoc_rqst,
    pub rq_cr_conn: fcnvme_ls_cr_conn_rqst,
    pub rq_dis_assoc: fcnvme_ls_disconnect_assoc_rqst,
    pub rq_dis_conn: fcnvme_ls_disconnect_conn_rqst,
    pub /: *mut *mut } __aligned(128); / alignment for other things alloc'd with,
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmefc_ls_responses {
    pub rsp_rjt: fcnvme_ls_rjt,
    pub rsp_cr_assoc: fcnvme_ls_cr_assoc_acc,
    pub rsp_cr_conn: fcnvme_ls_cr_conn_acc,
    pub rsp_dis_assoc: fcnvme_ls_disconnect_assoc_acc,
    pub rsp_dis_conn: fcnvme_ls_disconnect_conn_acc,
    pub /: *mut *mut } __aligned(128); / alignment for other things alloc'd with,
    pub buf: *mut *mut fcnvme_ls_acc_hdr acc =,
    pub ls_cmd: acc->w0.ls_cmd =,
    pub desc_len: acc->desc_list_len =,
    pub cpu_to_be32(FCNVME_LSDESC_RQST): acc->rqst.desc_tag =,
    pub fcnvme_lsdesc_rqst)): fcnvme_lsdesc_len(sizeof(struct,
    pub rqst_ls_cmd: acc->rqst.w0.ls_cmd =,
    pub buf: *mut *mut fcnvme_ls_rjt rjt =,
    pub cpu_to_be32(FCNVME_LSDESC_RJT): rjt->rjt.desc_tag =,
    pub fcnvme_lsdesc_rjt)): rjt->rjt.desc_len = fcnvme_lsdesc_len(sizeof(struct,
    pub reason: rjt->rjt.reason_code =,
    pub explanation: rjt->rjt.reason_explanation =,
    pub vendor: rjt->rjt.vendor =,
    pub fcnvme_ls_rjt): return sizeof(struct,
// Validation Error indexes into the string table below
}

//
// As the standard changed on the LS, check if old format and scope
// something other than Association (e.g. 0).
//
