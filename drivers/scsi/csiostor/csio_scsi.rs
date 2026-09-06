//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_scsi.h
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
// This file is part of the Chelsio FCoE driver for Linux.
//
// Copyright (c) 2008-2012 Chelsio Communications, Inc. All rights reserved.
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

//
// NOTE
// How do we calculate MAX FCoE SCSI SGEs? Here is the math:
// Max Egress WR size = 512 bytes
// One SCSI egress WR has the following fixed no of bytes:
// 48 (sizeof(struct fw_scsi_write[read]_wr)) - FW WR
// + 32 (sizeof(struct fc_fcp_cmnd)) - Immediate FCP_CMD
// ------
// 80
// ------
// That leaves us with 512 - 96 = 432 bytes for data SGE. Using
// struct ulptx_sgl header for the SGE consumes:
// - 4 bytes for cmnd_sge.
// - 12 bytes for the first SGL.
// That leaves us with 416 bytes for the remaining SGE pairs. Which is
// is 416 / 24 (size(struct ulptx_sge_pair)) = 17 SGE pairs,
// or 34 SGEs. Adding the first SGE fetches us 35 SGEs.
//
pub const CSIO_SCSI_MAX_SGE: c_int = 35;
pub const CSIO_SCSI_ABRT_TMO_MS: c_int = 60000;
pub const CSIO_SCSI_LUNRST_TMO_MS: c_int = 60000;

// all TM timeouts.
//
pub const CSIO_SCSI_IQ_WRSZ: c_int = 128;

pub const CSIO_MAX_SNS_LEN: c_int = 128;

// Reference to scsi_cmnd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_scsi_stats {
    pub /: *mut *mut uint64_t n_tot_success; / Total number of good I/Os,
    pub remote-node-not-: *mut *mut uint32_t n_rn_nr_error; / No. of,
// ready errors
//
    pub hw-module-not-: *mut *mut uint32_t n_hw_nr_error; / No. of,
// ready errors
//
    pub /: *mut *mut uint32_t n_dmamap_error; / No. of DMA map erros,
    pub too-many-SGes: *mut *mut uint32_t n_unsupp_sge_error; / No. of,
// errors.
//
    pub /: *mut *mut uint32_t n_no_req_error; / No. of Out-of-ioreqs error,
    pub /: *mut *mut uint32_t n_busy_error; / No. of -EBUSY errors,
    pub /: *mut *mut uint32_t n_hosterror; / No. of FW_HOSTERROR I/O,
    pub /: *mut *mut uint32_t n_rsperror; / No. of response errors,
    pub /: *mut *mut uint32_t n_autosense; / No. of auto sense replies,
    pub /: *mut *mut uint32_t n_ovflerror; / No. of overflow errors,
    pub /: *mut *mut uint32_t n_unflerror; / No. of underflow errors,
    pub not: *mut *mut uint32_t n_rdev_nr_error;/ No. of rdev,
// ready errors
//
    pub /: *mut *mut uint32_t n_rdev_lost_error;/ No. of rdev lost errors,
    pub /: *mut *mut uint32_t n_rdev_logo_error;/ No. of rdev logo errors,
    pub /: *mut *mut uint32_t n_link_down_error;/ No. of link down errors,
    pub /: *mut *mut uint32_t n_no_xchg_error; / No. no exchange error,
    pub /: *mut *mut uint32_t n_unknown_error;/ No. of unhandled errors,
    pub /: *mut *mut uint32_t n_aborted; / No. of aborted I/Os,
    pub /: *mut *mut uint32_t n_abrt_timedout; / No. of abort timedouts,
    pub /: *mut *mut uint32_t n_abrt_fail; / No. of abort failures,
    pub /: *mut *mut uint32_t n_abrt_dups; / No. of duplicate aborts,
    pub raced: *mut *mut uint32_t n_abrt_race_comp; / No. of aborts that,
// with completions.
//
    pub failures: *mut *mut uint32_t n_abrt_busy_error;/ No. of abort,
// due to -EBUSY.
//
    pub /: *mut *mut uint32_t n_closed; / No. of closed I/Os,
    pub failures: *mut *mut uint32_t n_cls_busy_error; / No. of close,
// due to -EBUSY.
//
    pub /: *mut *mut uint32_t n_active; / No. of IOs in active_q,
    pub /: *mut *mut uint32_t n_tm_active; / No. of TMs in active_q,
    pub worker: *mut *mut uint32_t n_wcbfn; / No. of I/Os in,
// cbfn q
//
    pub /: *mut *mut uint32_t n_free_ioreq; / No. of freelist entries,
    pub /: *mut *mut uint32_t n_free_ddp; / No. of DDP freelist,
    pub /: *mut *mut uint32_t n_unaligned; / No. of Unaligned SGls,
    pub /: *mut *mut uint32_t n_inval_cplop; / No. invalid CPL op's in IQ,
    pub IQ*/: *mut *mut uint32_t n_inval_scsiop; / No. invalid scsi op's in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_scsim {
    pub /: *mut *mut *mut csio_hw hw; / Pointer to HW moduel,
    pub /: *mut *mut uint8_t max_sge; / Max SGE,
    pub SCSI: *mut *mut uint8_t proto_cmd_len; / Proto specific,
// cmd length
//
    pub SCSI: *mut *mut uint16_t proto_rsp_len; / Proto specific,
// response length
//
    pub /: *mut *mut spinlock_t freelist_lock; / Lock for ioreq freelist,
    pub /: *mut *mut list_head active_q; / Outstanding SCSI I/Os,
    pub /: *mut *mut list_head ioreq_freelist; / Free list of ioreq's,
    pub /: *mut *mut list_head ddp_freelist; / DDP descriptor freelist,
    pub /: *mut *mut csio_scsi_stats stats; / This module's statistics,
}

// State machine defines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_scsi_ev {
    CSIO_SCSIE_START_IO = 1,		/* Start a regular SCSI IO */
    CSIO_SCSIE_START_TM,			/* Start a TM IO */
    CSIO_SCSIE_COMPLETED,			/* IO Completed */
    CSIO_SCSIE_ABORT,			/* Abort IO */
    CSIO_SCSIE_ABORTED,			/* IO Aborted */
    CSIO_SCSIE_CLOSE,			/* Close exchange */
    CSIO_SCSIE_CLOSED,			/* Exchange closed */
    CSIO_SCSIE_DRVCLEANUP,			/* Driver wants to manually
// cleanup this I/O.
//
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_scsi_lev {
    CSIO_LEV_ALL = 1,
    CSIO_LEV_LNODE,
    CSIO_LEV_RNODE,
    CSIO_LEV_LUN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_scsi_level_data {
    pub level: csio_scsi_lev,
    pub rnode: *mut csio_rnode,
    pub lnode: *mut csio_lnode,
    pub oslun: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_cmd_priv {
    pub /: *mut *mut uint8_t fc_tm_flags; / task management flags,
    pub wr_status: u16,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
//
// csio_scsi_start_io - Kick starts the IO SM.
// @req: io request SM.
//
// needs to be called with lock held.
//
// csio_scsi_start_tm - Kicks off the Task management IO SM.
// @req: io request SM.
//
// needs to be called with lock held.
//
// csio_scsi_abort - Abort an IO request
// @req: io request SM.
//
// needs to be called with lock held.
//
// csio_scsi_close - Close an IO request
// @req: io request SM.
//
// needs to be called with lock held.
//
extern "C" {
    pub fn csio_scsi_cleanup_io_q(: *mut csio_scsim, : *mut list_head);
}
extern "C" {
    pub fn csio_scsim_cleanup_io(: *mut csio_scsim, abort: bool) -> c_int;
}
extern "C" {
    pub fn csio_scsi_qconfig(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_scsim_init(: *mut csio_scsim, : *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_scsim_exit(: *mut csio_scsim);
}
