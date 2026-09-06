//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_lnode.h
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

pub const CSIO_FCOE_MAX_NPIV: c_int = 128;
pub const CSIO_FCOE_MAX_RNODES: c_int = 2048;
// FDMI port attribute unknown speed
pub const CSIO_HBA_PORTSPEED_UNKNOWN: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_fcf_info {
    pub list: list_head,
    pub priority: u8,
    pub mac: [u8; 6],
    pub name_id: [u8; 8],
    pub fabric: [u8; 8],
    pub vf_id: u16,
    pub vlan_id: u8,
    pub max_fcoe_size: u16,
    pub fc_map: [u8; 3],
    pub fka_adv: u32,
    pub fcfi: u32,
    pub get_next:1: u8,
    pub link_aff:1: u8,
    pub fpma:1: u8,
    pub spma:1: u8,
    pub login:1: u8,
    pub portid: u8,
    pub spma_mac: [u8; 6],
    pub kref: kref,
}

// Defines for flags
pub const CSIO_LNF_FIPSUPP: c_uint = 0x00000001	/* Fip Supported */;
pub const CSIO_LNF_NPIVSUPP: c_uint = 0x00000002	/* NPIV supported */;
pub const CSIO_LNF_LINK_ENABLE: c_uint = 0x00000004	/* Link enabled */;
pub const CSIO_LNF_FDMI_ENABLE: c_uint = 0x00000008	/* FDMI support */;
// Transport events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_ln_fc_evt {
    CSIO_LN_FC_LINKUP = 1,
    CSIO_LN_FC_LINKDOWN,
    CSIO_LN_FC_RSCN,
    CSIO_LN_FC_ATTRIB_UPDATE,
}

// Lnode stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_lnode_stats {
    pub /: *mut *mut uint32_t n_link_up; / Link down,
    pub /: *mut *mut uint32_t n_link_down; / Link up,
    pub /: *mut *mut uint32_t n_err; / error,
    pub /: *mut *mut uint32_t n_err_nomem; / memory not available,
    pub /: *mut *mut uint32_t n_inval_parm; / Invalid parameters,
    pub /: *mut *mut uint32_t n_evt_unexp; / unexpected event,
    pub /: *mut *mut uint32_t n_evt_drop; / dropped event,
    pub /: *mut *mut uint32_t n_rnode_match; / matched rnode,
    pub /: *mut *mut uint32_t n_dev_loss_tmo; / Device loss timeout,
    pub /: *mut *mut uint32_t n_fdmi_err; / fdmi err,
    pub /: *mut *mut uint32_t n_evt_fw[PROTO_ERR_IMPL_LOGO + 1]; / fw events,
    pub /: *mut *mut csio_ln_ev n_evt_sm[CSIO_LNE_MAX_EVENT]; / State m/c events,
    pub /: *mut *mut uint32_t n_rnode_alloc; / rnode allocated,
    pub /: *mut *mut uint32_t n_rnode_free; / rnode freed,
    pub /: *mut *mut uint32_t n_rnode_nomem; / rnode alloc failure,
    pub /: *mut *mut uint32_t n_input_requests; / Input Requests,
    pub /: *mut *mut uint32_t n_output_requests; / Output Requests,
    pub /: *mut *mut uint32_t n_control_requests; / Control Requests,
    pub /: *mut *mut uint32_t n_input_bytes; / Input Bytes,
    pub /: *mut *mut uint32_t n_output_bytes; / Output Bytes,
    pub rsvd1: u32,
}

// Common Lnode params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_lnode_params {
    pub ra_tov: u32,
    pub fcfi: u32,
    pub /: *mut *mut uint32_t log_level; / Module level for debugging,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_service_parms {
    pub /: *mut *mut fc_els_csp csp; / Common service parms,
    pub /: *mut *mut uint8_t wwpn[8]; / WWPN,
    pub /: *mut *mut uint8_t wwnn[8]; / WWNN,
    pub /: *mut *mut fc_els_cssp clsp[4]; / Class service params,
    pub /: *mut *mut uint8_t vvl[16]; / Vendor version level,
}

// Lnode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_lnode {
    pub sibling: *mut *mut csio_sm sm; / State machine +,
// lnode list.
//
    pub /: *mut *mut *mut csio_hw hwp; / Pointer to the HW module,
    pub /: *mut *mut uint8_t portid; / Port ID,
    pub rsvd1: u8,
    pub rsvd2: u16,
    pub /: *mut *mut uint32_t dev_num; / Device number,
    pub /: *mut *mut uint32_t flags; / Flags,
    pub /: *mut *mut list_head fcf_lsthead; / FCF entries,
    pub /: *mut *mut *mut csio_fcf_info fcfinfo; / FCF in use,
    pub /: *mut *mut *mut csio_ioreq mgmt_req; / MGMT request,
// FCoE identifiers
    pub mac: [u8; 6],
    pub nport_id: u32,
    pub /: *mut *mut csio_service_parms ln_sparm; / Service parms,
// Firmware identifiers
    pub /: *mut *mut uint32_t fcf_flowid; /fcf flowid,
    pub vnp_flowid: u32,
    pub /: *mut *mut uint16_t ssn_cnt; / Registered Session,
    pub /: *mut *mut uint8_t cur_evt; / Current event,
    pub /: *mut *mut uint8_t prev_evt; / Previous event,
// Children
    pub lnode: *mut *mut list_head cln_head; / Head of the children,
// list.
//
    pub LNodes*/: *mut *mut uint32_t num_vports; / Total NPIV/children,
    pub child: *mut *mut *mut csio_lnode pln; / Parent lnode of,
// lnodes.
//
    pub /: *mut *mut list_head cmpl_q; / Pending I/Os on this lnode,
// Remote node information
    pub /: *mut *mut list_head rnhead; / Head of rnode list,
    pub registered: *mut *mut uint32_t num_reg_rnodes; / Number of rnodes,
// with the host.
//
    pub targets: *mut *mut uint32_t n_scsi_tgts; / Number of scsi,
// found
//
    pub targets: *mut *mut uint32_t last_scan_ntgts;/ Number of scsi,
// found per last scan.
//
    pub after: *mut *mut uint32_t tgt_scan_tick; / timer started,
// new tgt found
//
// FC transport data
    pub fc_vport: *mut fc_vport,
    pub fch_stats: fc_host_statistics,
    pub /: *mut *mut csio_lnode_stats stats; / Common lnode stats,
    pub /: *mut *mut csio_lnode_params params; / Common lnode params,
}

// HW->Lnode notifications
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_ln_notify {
    CSIO_LN_NOTIFY_HWREADY = 1,
    CSIO_LN_NOTIFY_HWSTOP,
    CSIO_LN_NOTIFY_HWREMOVE,
    CSIO_LN_NOTIFY_HWRESET,
}

extern "C" {
    pub fn csio_fcoe_fwevt_handler(: *mut csio_hw, cpl_op: __u8, : *mut __be64);
}
extern "C" {
    pub fn csio_is_lnode_ready(: *mut csio_lnode) -> c_int;
}
extern "C" {
    pub fn csio_lnode_state_to_str(ln: *mut csio_lnode, str: *mut i8);
}
extern "C" {
    pub fn csio_notify_lnodes(: *mut csio_hw, csio_ln_notify: enum);
}
extern "C" {
    pub fn csio_disable_lnodes(: *mut csio_hw, _arg: u8, _arg: bool);
}
extern "C" {
    pub fn csio_lnode_async_event(: *mut csio_lnode, csio_ln_fc_evt: enum);
}
extern "C" {
    pub fn csio_ln_fdmi_start(: *mut csio_lnode, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn csio_lnode_start(: *mut csio_lnode) -> c_int;
}
extern "C" {
    pub fn csio_lnode_stop(: *mut csio_lnode);
}
extern "C" {
    pub fn csio_lnode_close(: *mut csio_lnode);
}
extern "C" {
    pub fn csio_lnode_exit(: *mut csio_lnode);
}
