//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/snic_io.h
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.

// SG descriptor for snic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_sg_desc {
    pub addr: __le64,
    pub len: __le32,
    pub _resvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_dflt_sgl {
    pub sg_desc: [snic_sg_desc; SNIC_DFLT_SG_DESC_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_max_sgl {
    pub sg_desc: [snic_sg_desc; SNIC_MAX_SG_DESC_CNT],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_req_cache_type {
    SNIC_REQ_CACHE_DFLT_SGL = 0,	/* cache with default size sgl */
    SNIC_REQ_CACHE_MAX_SGL,		/* cache with max size sgl */
    SNIC_REQ_TM_CACHE,		/* cache for task mgmt reqs contains
    snic_host_req objects only*/
    SNIC_REQ_MAX_CACHES		/* number of sgl caches */
}

// Per IO internal state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_internal_io_state {
    pub rqi: *mut c_char,
    pub flags: u64,
    pub state: u32,
    pub /: *mut *mut u32 abts_status; / Abort completion status,
    pub /: *mut *mut u32 lr_status; / device reset completion status,
}

// IO state machine
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_ioreq_state {
    SNIC_IOREQ_NOT_INITED = 0,
    SNIC_IOREQ_PENDING,
    SNIC_IOREQ_ABTS_PENDING,
    SNIC_IOREQ_ABTS_COMPLETE,
    SNIC_IOREQ_LR_PENDING,
    SNIC_IOREQ_LR_COMPLETE,
    SNIC_IOREQ_COMPLETE,
}

//
// snic_req_info : Contains info about IO, one per scsi command.
// Notes: Make sure that the structure is aligned to 16 B
// this helps in easy access to snic_req_info from snic_host_req
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_req_info {
    pub list: list_head,
    pub req: *mut snic_host_req,
    pub /: *mut *mut u64 start_time; / start time in jiffies,
    pub /: *mut *mut u16 rq_pool_type; / noticion of request pool type,
    pub sgl)*/: *mut *mut u16 req_len; / buf len passing to fw (req +,
    pub tgt_id: u32,
    pub tm_tag: u32,
    pub /: *mut *mut u8 io_cmpl:1; / sets to 1 when fw completes IO,
    pub resvd: [u8; 3],
    pub /: *mut *mut *mut scsi_cmnd sc; / Associated scsi cmd,
    pub /: *mut *mut *mut snic snic; / Associated snic,
    pub /: *mut *mut ulong sge_va; / Pointer to Resp Buffer,
    pub snsbuf_va: u64,
    pub abort_req: *mut snic_host_req,
    pub abts_done: *mut completion,
    pub dr_req: *mut snic_host_req,
    pub dr_done: *mut completion,
}

extern "C" {
    pub fn snic_req_free(: *mut snic, : *mut snic_req_info);
}
extern "C" {
    pub fn snic_calc_io_process_time(: *mut snic, : *mut snic_req_info);
}
extern "C" {
    pub fn snic_pci_unmap_rsp_buf(: *mut snic, : *mut snic_req_info);
}
