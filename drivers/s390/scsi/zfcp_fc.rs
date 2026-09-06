//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_fc.h
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
// zfcp device driver
//
// Fibre Channel related definitions and inline functions for the zfcp
// device driver
//
// Copyright IBM Corp. 2009, 2017
//

//
// struct zfcp_fc_event - FC HBAAPI event for internal queueing from irq context
// @code: Event code
// @data: Event data
// @list: list_head for zfcp_fc_events list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_event {
    pub code: fc_host_event_code,
    pub data: u32,
    pub list: list_head,
}

//
// struct zfcp_fc_events - Infrastructure for posting FC events from irq context
// @list: List for queueing of events from irq context to workqueue
// @list_lock: Lock for event list
// @work: work_struct for forwarding events in workqueue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_events {
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub work: work_struct,
}

//
// struct zfcp_fc_gid_pn_req - container for ct header plus gid_pn request
// @ct_hdr: FC GS common transport header
// @gid_pn: GID_PN request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_gid_pn_req {
    pub ct_hdr: fc_ct_hdr,
    pub gid_pn: fc_ns_gid_pn,
    pub __packed: },
//
// struct zfcp_fc_gid_pn_rsp - container for ct header plus gid_pn response
// @ct_hdr: FC GS common transport header
// @gid_pn: GID_PN response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_gid_pn_rsp {
    pub ct_hdr: fc_ct_hdr,
    pub gid_pn: fc_gid_pn_resp,
    pub __packed: },
//
// struct zfcp_fc_gpn_ft - container for ct header plus gpn_ft request
// @ct_hdr: FC GS common transport header
// @gpn_ft: GPN_FT request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_gpn_ft_req {
    pub ct_hdr: fc_ct_hdr,
    pub gpn_ft: fc_ns_gid_ft,
    pub __packed: },
//
// struct zfcp_fc_gspn_req - container for ct header plus GSPN_ID request
// @ct_hdr: FC GS common transport header
// @gspn: GSPN_ID request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_gspn_req {
    pub ct_hdr: fc_ct_hdr,
    pub gspn: fc_gid_pn_resp,
    pub __packed: },
//
// struct zfcp_fc_gspn_rsp - container for ct header plus GSPN_ID response
// @ct_hdr: FC GS common transport header
// @gspn: GSPN_ID response
// @name: The name string of the GSPN_ID response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_gspn_rsp {
    pub ct_hdr: fc_ct_hdr,
    pub gspn: fc_gspn_resp,
    pub name: [c_char; FC_SYMBOLIC_NAME_SIZE],
    pub __packed: },
//
// struct zfcp_fc_rspn_req - container for ct header plus RSPN_ID request
// @ct_hdr: FC GS common transport header
// @rspn: RSPN_ID request
// @name: The name string of the RSPN_ID request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_rspn_req {
    pub ct_hdr: fc_ct_hdr,
    pub rspn: fc_ns_rspn,
    pub name: [c_char; FC_SYMBOLIC_NAME_SIZE],
    pub __packed: },
//
// struct zfcp_fc_req - Container for FC ELS and CT requests sent from zfcp
// @ct_els: data required for issuing fsf command
// @sg_req: scatterlist entry for request data, refers to embedded @u submember
// @sg_rsp: scatterlist entry for response data, refers to embedded @u submember
// @u: request and response specific data
// @u.adisc: ADISC specific data
// @u.adisc.req: ADISC request
// @u.adisc.rsp: ADISC response
// @u.gid_pn: GID_PN specific data
// @u.gid_pn.req: GID_PN request
// @u.gid_pn.rsp: GID_PN response
// @u.gpn_ft: GPN_FT specific data
// @u.gpn_ft.sg_rsp2: GPN_FT response, not embedded here, allocated elsewhere
// @u.gpn_ft.req: GPN_FT request
// @u.gspn: GSPN specific data
// @u.gspn.req: GSPN request
// @u.gspn.rsp: GSPN response
// @u.rspn: RSPN specific data
// @u.rspn.req: RSPN request
// @u.rspn.rsp: RSPN response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_req {
    pub ct_els: zfcp_fsf_ct_els,
    pub sg_req: scatterlist,
    pub sg_rsp: scatterlist,
    pub req: fc_els_adisc,
    pub rsp: fc_els_adisc,
    pub adisc: },
    pub req: zfcp_fc_gid_pn_req,
    pub rsp: zfcp_fc_gid_pn_rsp,
    pub gid_pn: },
    pub 1]: scatterlist sg_rsp2[ZFCP_FC_GPN_FT_NUM_BUFS -,
    pub req: zfcp_fc_gpn_ft_req,
    pub gpn_ft: },
    pub req: zfcp_fc_gspn_req,
    pub rsp: zfcp_fc_gspn_rsp,
    pub gspn: },
    pub req: zfcp_fc_rspn_req,
    pub rsp: fc_ct_hdr,
    pub rspn: },
    pub u: },
}

//
// enum zfcp_fc_wka_status - FC WKA port status in zfcp
// @ZFCP_FC_WKA_PORT_OFFLINE: Port is closed and not in use
// @ZFCP_FC_WKA_PORT_CLOSING: The FSF "close port" request is pending
// @ZFCP_FC_WKA_PORT_OPENING: The FSF "open port" request is pending
// @ZFCP_FC_WKA_PORT_ONLINE: The port is open and the port handle is valid
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_fc_wka_status {
    ZFCP_FC_WKA_PORT_OFFLINE,
    ZFCP_FC_WKA_PORT_CLOSING,
    ZFCP_FC_WKA_PORT_OPENING,
    ZFCP_FC_WKA_PORT_ONLINE,
}

//
// struct zfcp_fc_wka_port - representation of well-known-address (WKA) FC port
// @adapter: Pointer to adapter structure this WKA port belongs to
// @opened: Wait for completion of open command
// @closed: Wait for completion of close command
// @status: Current status of WKA port
// @refcount: Reference count to keep port open as long as it is in use
// @d_id: FC destination id or well-known-address
// @handle: FSF handle for the open WKA port
// @mutex: Mutex used during opening/closing state changes
// @work: For delaying the closing of the WKA port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_wka_port {
    pub adapter: *mut zfcp_adapter,
    pub opened: wait_queue_head_t,
    pub closed: wait_queue_head_t,
    pub status: zfcp_fc_wka_status,
    pub refcount: core::sync::atomic::AtomicI32,
    pub d_id: u32,
    pub handle: u32,
    pub mutex: mutex,
    pub work: delayed_work,
}

//
// struct zfcp_fc_wka_ports - Data structures for FC generic services
// @ms: FC Management service
// @ts: FC time service
// @ds: FC directory service
// @as: FC alias service
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_fc_wka_ports {
    pub ms: zfcp_fc_wka_port,
    pub ts: zfcp_fc_wka_port,
    pub ds: zfcp_fc_wka_port,
    pub as: zfcp_fc_wka_port,
}

//
// zfcp_fc_scsi_to_fcp - setup FCP command with data from scsi_cmnd
// @fcp: fcp_cmnd to setup
// @scsi: scsi_cmnd where to get LUN, task attributes/flags and CDB
//
// zfcp_fc_fcp_tm() - Setup FCP command as task management command.
// @fcp: Pointer to FCP_CMND IU to set up.
// @dev: Pointer to SCSI_device where to send the task management command.
// @tm_flags: Task management flags to setup tm command.
//
// zfcp_fc_evap_fcp_rsp - evaluate FCP RSP IU and update scsi_cmnd accordingly
// @fcp_rsp: FCP RSP IU to evaluate
// @scsi: SCSI command where to update status and sense buffer
//
// FCP_DL was not sufficient for SCSI data length
