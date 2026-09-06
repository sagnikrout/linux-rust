//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fnic_io.h
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

pub const FNIC_DFLT_SG_DESC_CNT: c_int = 32;

pub const NVME_STAT_ERROR: c_uint = 0x2;
pub const NVME_STAT_TASK_SET_FULL: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_sg_desc {
    pub addr: __le64,
    pub len: __le32,
    pub _resvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_dflt_sgl_list {
    pub sg_desc: [host_sg_desc; FNIC_DFLT_SG_DESC_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_sgl_list {
    pub sg_desc: [host_sg_desc; FNIC_MAX_SG_DESC_CNT],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_sgl_list_type {
    FNIC_SGL_CACHE_DFLT = 0,  /* cache with default size sgl */
    FNIC_SGL_CACHE_MAX,       /* cache with max size sgl */
    FNIC_SGL_NUM_CACHES       /* number of sgl caches */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fnic_ioreq_state {
    FNIC_IOREQ_NOT_INITED = 0,
    FNIC_IOREQ_CMD_PENDING,
    FNIC_IOREQ_ABTS_PENDING,
    FNIC_IOREQ_ABTS_COMPLETE,
    FNIC_IOREQ_CMD_COMPLETE,
    FNIC_IOREQ_RESET_TERM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_io_req {
    pub iport: *mut fnic_iport_s,
    pub tport: *mut fnic_tport_s,
    pub /: *mut *mut *mut host_sg_desc sgl_list; / sgl list,
    pub /: *mut *mut *mut void sgl_list_alloc; / sgl list address used for free,
    pub buffer*/: *mut *mut dma_addr_t sense_buf_pa; / dma address for sense,
    pub /: *mut *mut dma_addr_t sgl_list_pa; / dma address for sgl list,
    pub sgl_cnt: u16,
    pub /: *mut *mut u8 sgl_type; / device DMA descriptor list type,
    pub /: *mut *mut u8 sgl_mapped:1; / set when sgl_list_pa is a valid DMA mapping,
    pub /: *mut *mut u8 io_completed:1; / set to 1 when fw completes IO,
    pub /: *mut *mut u32 port_id; / remote port DID,
    pub /: *mut *mut unsigned long start_time; / in jiffies,
    pub tag: u32,
    pub cmd_state: fnic_ioreq_state,
    pub cmd_flags: u32,
    pub abts_state: u32,
    pub /: *mut *mut *mut completion abts_done; / completion for abts,
    pub /: *mut *mut *mut completion dr_done; / completion for device reset,
    pub /: *mut *mut *mut scsi_cmnd sc; / midlayer's cmd pointer,
    pub wq: *mut vnic_wq_copy,
    pub nvfnic_io_cmpl: llist_node,
    pub fcp_req: *mut nvmefc_fcp_req,
    pub io_req): *mut *mut void (done)(struct fnic_io_req,
    pub /: *mut *mut unsigned long waitq_start_time; / in jiffies,
    pub admin_io_timer: timer_list,
    pub status: u32,
    pub tag_data: *mut fnic_tag_t,
    pub io_evt: fnic_io_event_s,
}
