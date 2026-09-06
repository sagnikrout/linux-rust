//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_rnode.h
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

// State machine evets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_rn_ev {
    CSIO_RNFE_NONE = (uint32_t)0,			/* None */
    CSIO_RNFE_LOGGED_IN,				/* [N/F]Port login
// complete.
//
    CSIO_RNFE_PRLI_DONE,				/* PRLI completed */
    CSIO_RNFE_PLOGI_RECV,				/* Received PLOGI */
    CSIO_RNFE_PRLI_RECV,				/* Received PLOGI */
    CSIO_RNFE_LOGO_RECV,				/* Received LOGO */
    CSIO_RNFE_PRLO_RECV,				/* Received PRLO */
    CSIO_RNFE_DOWN,					/* Rnode is down */
    CSIO_RNFE_CLOSE,				/* Close rnode */
    CSIO_RNFE_NAME_MISSING,				/* Rnode name missing
// in name server.
//
    CSIO_RNFE_MAX_EVENT,
}

// rnode stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_rnode_stats {
    pub /: *mut *mut uint32_t n_err; / error,
    pub /: *mut *mut uint32_t n_err_inval; / invalid parameter,
    pub /: *mut *mut uint32_t n_err_nomem; / error nomem,
    pub /: *mut *mut uint32_t n_evt_unexp; / unexpected event,
    pub /: *mut *mut uint32_t n_evt_drop; / unexpected event,
    pub /: *mut *mut uint32_t n_evt_fw[PROTO_ERR_IMPL_LOGO + 1]; / fw events,
    pub /: *mut *mut csio_rn_ev n_evt_sm[CSIO_RNFE_MAX_EVENT]; / State m/c events,
    pub of: *mut *mut uint32_t n_lun_rst; / Number of resets,
// of LUNs under this
// target
//
    pub reset: *mut *mut uint32_t n_lun_rst_fail; / Number of LUN,
// failures.
//
    pub /: *mut *mut uint32_t n_tgt_rst; / Number of target resets,
    pub reset: *mut *mut uint32_t n_tgt_rst_fail; / Number of target,
// failures.
//
}

// Defines for rnode role
pub const CSIO_RNFR_INITIATOR: c_uint = 0x1;
pub const CSIO_RNFR_TARGET: c_uint = 0x2;
pub const CSIO_RNFR_FABRIC: c_uint = 0x4;
pub const CSIO_RNFR_NS: c_uint = 0x8;
pub const CSIO_RNFR_NPORT: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_rnode {
    pub -: *mut *mut csio_sm sm; / State machine,
// should be the
// 1st member
//
    pub owning: *mut *mut *mut csio_lnode lnp; / Pointer to,
// Lnode
    pub /: *mut *mut uint32_t flowid; / Firmware ID,
    pub IOs: *mut *mut list_head host_cmpl_q; / SCSI,
// pending to completed
// to Mid-layer.
//
// FC identifiers for remote node
    pub nport_id: u32,
    pub /: *mut *mut uint16_t fcp_flags; / FCP Flags,
    pub /: *mut *mut uint8_t cur_evt; / Current event,
    pub /: *mut *mut uint8_t prev_evt; / Previous event,
    pub Fabric/Target/: *mut *mut uint32_t role; /,
// Initiator/NS
//
    pub /: *mut *mut *mut fcoe_rdev_entry rdev_entry; / Rdev entry,
    pub rn_sparm: csio_service_parms,
// FC transport attributes
    pub /: *mut *mut *mut fc_rport rport; / FC transport rport,
    pub /: *mut *mut uint32_t supp_classes; / Supported FC classes,
    pub /: *mut *mut uint32_t maxframe_size; / Max Frame size,
    pub /: *mut *mut uint32_t scsi_id; / Transport given SCSI id,
    pub /: *mut *mut csio_rnode_stats stats; / Common rnode stats,
}

extern "C" {
    pub fn csio_is_rnode_ready(rn: *mut csio_rnode) -> c_int;
}
extern "C" {
    pub fn csio_rnode_state_to_str(rn: *mut csio_rnode, str: *mut i8);
}
extern "C" {
    pub fn csio_rnode_fwevt_handler(rn: *mut csio_rnode, fwevt: u8);
}
extern "C" {
    pub fn csio_put_rnode(ln: *mut csio_lnode, rn: *mut csio_rnode);
}
extern "C" {
    pub fn csio_reg_rnode(: *mut csio_rnode);
}
extern "C" {
    pub fn csio_unreg_rnode(: *mut csio_rnode);
}
extern "C" {
    pub fn csio_rnode_devloss_handler(: *mut csio_rnode);
}
