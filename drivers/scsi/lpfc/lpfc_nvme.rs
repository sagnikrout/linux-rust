//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_nvme.h
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
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2022 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2004-2016 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
// Portions Copyright (C) 2004-2005 Christoph Hellwig
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//

pub const LPFC_NVME_ERSP_LEN: c_uint = 0x20;
pub const LPFC_NVME_WAIT_TMO: c_int = 10;
pub const LPFC_NVME_EXPEDITE_XRICNT: c_int = 8;
pub const LPFC_NVME_FB_SHIFT: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvme_qhandle {
    pub /: *mut *mut uint32_t index; / WQ index to use,
    pub /: *mut *mut uint32_t qidx; / queue index passed to create,
    pub /: *mut *mut uint32_t cpu_id; / current cpu id at time of create,
}

// Declare nvme-based local and remote port definitions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvme_lport {
    pub vport: *mut lpfc_vport,
    pub lport_unreg_cmp: *mut completion,
// Add stats counters here
    pub fc4NvmeLsRequests: core::sync::atomic::AtomicI32,
    pub fc4NvmeLsCmpls: core::sync::atomic::AtomicI32,
    pub xmt_fcp_noxri: core::sync::atomic::AtomicI32,
    pub xmt_fcp_bad_ndlp: core::sync::atomic::AtomicI32,
    pub xmt_fcp_qdepth: core::sync::atomic::AtomicI32,
    pub xmt_fcp_wqerr: core::sync::atomic::AtomicI32,
    pub xmt_fcp_err: core::sync::atomic::AtomicI32,
    pub xmt_fcp_abort: core::sync::atomic::AtomicI32,
    pub xmt_ls_abort: core::sync::atomic::AtomicI32,
    pub xmt_ls_err: core::sync::atomic::AtomicI32,
    pub cmpl_fcp_xb: core::sync::atomic::AtomicI32,
    pub cmpl_fcp_err: core::sync::atomic::AtomicI32,
    pub cmpl_ls_xb: core::sync::atomic::AtomicI32,
    pub cmpl_ls_err: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvme_rport {
    pub lport: *mut lpfc_nvme_lport,
    pub remoteport: *mut nvme_fc_remote_port,
    pub ndlp: *mut lpfc_nodelist,
    pub rport_unreg_done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvme_fcpreq_priv {
    pub nvme_buf: *mut lpfc_io_buf,
}

//
// set NVME LS request timeouts to 30s. It is larger than the 2*R_A_TOV
// set by the spec, which appears to have issues with some devices.
//
pub const LPFC_NVME_LS_TIMEOUT: c_int = 30;

pub const LPFC_NVMET_RQE_MIN_POST: c_int = 128;
pub const LPFC_NVMET_RQE_DEF_POST: c_int = 512;
pub const LPFC_NVMET_RQE_DEF_COUNT: c_int = 2048;
pub const LPFC_NVMET_SUCCESS_LEN: c_int = 12;
pub const LPFC_NVMET_MRQ_AUTO: c_int = 0;
pub const LPFC_NVMET_MRQ_MAX: c_int = 16;

// Used for NVME Target
pub const LPFC_NVMET_INV_HOST_ACTIVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvmet_tgtport {
    pub phba: *mut lpfc_hba,
    pub tport_unreg_cmp: *mut completion,
    pub /: *mut *mut atomic_t state; / tracks nvmet hosthandle invalidation,
// Stats counters - lpfc_nvmet_unsol_ls_buffer
    pub rcv_ls_req_in: core::sync::atomic::AtomicI32,
    pub rcv_ls_req_out: core::sync::atomic::AtomicI32,
    pub rcv_ls_req_drop: core::sync::atomic::AtomicI32,
    pub xmt_ls_abort: core::sync::atomic::AtomicI32,
    pub xmt_ls_abort_cmpl: core::sync::atomic::AtomicI32,
// Stats counters - lpfc_nvmet_xmt_ls_rsp
    pub xmt_ls_rsp: core::sync::atomic::AtomicI32,
    pub xmt_ls_drop: core::sync::atomic::AtomicI32,
// Stats counters - lpfc_nvmet_xmt_ls_rsp_cmp
    pub xmt_ls_rsp_error: core::sync::atomic::AtomicI32,
    pub xmt_ls_rsp_aborted: core::sync::atomic::AtomicI32,
    pub xmt_ls_rsp_xb_set: core::sync::atomic::AtomicI32,
    pub xmt_ls_rsp_cmpl: core::sync::atomic::AtomicI32,
// Stats counters - lpfc_nvmet_unsol_fcp_buffer
    pub rcv_fcp_cmd_in: core::sync::atomic::AtomicI32,
    pub rcv_fcp_cmd_out: core::sync::atomic::AtomicI32,
    pub rcv_fcp_cmd_drop: core::sync::atomic::AtomicI32,
    pub rcv_fcp_cmd_defer: core::sync::atomic::AtomicI32,
    pub xmt_fcp_release: core::sync::atomic::AtomicI32,
// Stats counters - lpfc_nvmet_xmt_fcp_op
    pub xmt_fcp_drop: core::sync::atomic::AtomicI32,
    pub xmt_fcp_read_rsp: core::sync::atomic::AtomicI32,
    pub xmt_fcp_read: core::sync::atomic::AtomicI32,
    pub xmt_fcp_write: core::sync::atomic::AtomicI32,
    pub xmt_fcp_rsp: core::sync::atomic::AtomicI32,
// Stats counters - lpfc_nvmet_xmt_fcp_op_cmp
    pub xmt_fcp_rsp_xb_set: core::sync::atomic::AtomicI32,
    pub xmt_fcp_rsp_cmpl: core::sync::atomic::AtomicI32,
    pub xmt_fcp_rsp_error: core::sync::atomic::AtomicI32,
    pub xmt_fcp_rsp_aborted: core::sync::atomic::AtomicI32,
    pub xmt_fcp_rsp_drop: core::sync::atomic::AtomicI32,
// Stats counters - lpfc_nvmet_xmt_fcp_abort
    pub xmt_fcp_xri_abort_cqe: core::sync::atomic::AtomicI32,
    pub xmt_fcp_abort: core::sync::atomic::AtomicI32,
    pub xmt_fcp_abort_cmpl: core::sync::atomic::AtomicI32,
    pub xmt_abort_sol: core::sync::atomic::AtomicI32,
    pub xmt_abort_unsol: core::sync::atomic::AtomicI32,
    pub xmt_abort_rsp: core::sync::atomic::AtomicI32,
    pub xmt_abort_rsp_error: core::sync::atomic::AtomicI32,
// Stats counters - defer IO
    pub defer_ctx: core::sync::atomic::AtomicI32,
    pub defer_fod: core::sync::atomic::AtomicI32,
    pub defer_wqfull: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvmet_ctx_info {
    pub nvmet_ctx_list: list_head,
    pub /: *mut *mut spinlock_t nvmet_ctx_list_lock; / lock per CPU,
    pub nvmet_ctx_next_cpu: *mut lpfc_nvmet_ctx_info,
    pub nvmet_ctx_start_cpu: *mut lpfc_nvmet_ctx_info,
    pub nvmet_ctx_list_cnt: u16,
    pub /: *mut *mut char pad[16]; / pad to a cache-line,
}

// This retrieves the context info associated with the specified cpu / mrq

// Values for state field of struct lpfc_async_xchg_ctx
pub const LPFC_NVME_STE_LS_RCV: c_int = 1;
pub const LPFC_NVME_STE_LS_ABORT: c_int = 2;
pub const LPFC_NVME_STE_LS_RSP: c_int = 3;
pub const LPFC_NVME_STE_RCV: c_int = 4;
pub const LPFC_NVME_STE_DATA: c_int = 5;
pub const LPFC_NVME_STE_ABORT: c_int = 6;
pub const LPFC_NVME_STE_DONE: c_int = 7;
pub const LPFC_NVME_STE_FREE: c_uint = 0xff;
// Values for flag field of struct lpfc_async_xchg_ctx
pub const LPFC_NVME_IO_INP: c_uint = 0x1  /* IO is in progress on exchange */;
pub const LPFC_NVME_ABORT_OP: c_uint = 0x2  /* Abort WQE issued on exchange */;
pub const LPFC_NVME_XBUSY: c_uint = 0x4  /* XB bit set on IO cmpl */;
pub const LPFC_NVME_CTX_RLS: c_uint = 0x8  /* ctx free requested */;
pub const LPFC_NVME_ABTS_RCV: c_uint = 0x10  /* ABTS received on exchange */;
pub const LPFC_NVME_CTX_REUSE_WQ: c_uint = 0x20  /* ctx reused via WQ */;
pub const LPFC_NVME_DEFER_WQFULL: c_uint = 0x40  /* Waiting on a free WQE */;
pub const LPFC_NVME_TNOTIFY: c_uint = 0x80  /* notify transport of abts */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_async_xchg_ctx {
    pub fcp_req: nvmefc_tgt_fcp_req,
    pub hdlrctx: },
    pub list: list_head,
    pub phba: *mut lpfc_hba,
    pub ndlp: *mut lpfc_nodelist,
    pub ls_req: *mut nvmefc_ls_req,
    pub ls_rsp: nvmefc_ls_rsp,
    pub wqeq: *mut lpfc_iocbq,
    pub abort_wqeq: *mut lpfc_iocbq,
    pub /: *mut *mut spinlock_t ctxlock; / protect flag access,
    pub sid: u32,
    pub offset: u32,
    pub oxid: u16,
    pub size: u16,
    pub entry_cnt: u16,
    pub cpu: u16,
    pub idx: u16,
    pub state: u16,
    pub flag: u16,
    pub payload: *mut c_void,
    pub rqb_buffer: *mut rqb_dmabuf,
    pub ctxbuf: *mut lpfc_nvmet_ctxbuf,
    pub hdwq: *mut lpfc_sli4_hdw_queue,

    pub ts_isr_cmd: u64,
    pub ts_cmd_nvme: u64,
    pub ts_nvme_data: u64,
    pub ts_data_wqput: u64,
    pub ts_isr_data: u64,
    pub ts_data_nvme: u64,
    pub ts_nvme_status: u64,
    pub ts_status_wqput: u64,
    pub ts_isr_status: u64,
    pub ts_status_nvme: u64,

}

// routines found in lpfc_nvme.c
// routines found in lpfc_nvmet.c
