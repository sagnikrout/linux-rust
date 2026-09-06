//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_fcs.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//

pub const BFA_FCS_OS_STR_LEN: c_int = 64;
//
// !!! Only append to the enums defined here to avoid any versioning
// !!! needed between trace utility and driver version
//

pub const BFA_FCS_BRCD_SWITCH_OUI: c_uint = 0x051e;
pub const N2N_LOCAL_PID: c_uint = 0x010000;
pub const N2N_REMOTE_PID: c_uint = 0x020000;
pub const BFA_FCS_RETRY_TIMEOUT: c_int = 2000;
pub const BFA_FCS_MAX_NS_RETRIES: c_int = 5;

pub const BFA_FCS_MAX_RPORT_LOGINS: c_int = 1024;
//
// VPort NS State Machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vport_ns_event {
    NSSM_EVENT_PORT_ONLINE = 1,
    NSSM_EVENT_PORT_OFFLINE = 2,
    NSSM_EVENT_PLOGI_SENT = 3,
    NSSM_EVENT_RSP_OK = 4,
    NSSM_EVENT_RSP_ERROR = 5,
    NSSM_EVENT_TIMEOUT = 6,
    NSSM_EVENT_NS_QUERY = 7,
    NSSM_EVENT_RSPNID_SENT = 8,
    NSSM_EVENT_RFTID_SENT = 9,
    NSSM_EVENT_RFFID_SENT = 10,
    NSSM_EVENT_GIDFT_SENT = 11,
    NSSM_EVENT_RNNID_SENT = 12,
    NSSM_EVENT_RSNN_NN_SENT = 13,
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_fcs_lport_ns_sm_t)(struct bfa_fcs_lport_ns_s, vport_ns_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_ns_s {
    pub /: *mut *mut bfa_fcs_lport_ns_sm_t sm; / state machine,
    pub timer: bfa_timer_s,
    pub /: *mut *mut *mut bfa_fcs_lport_s port; / parent port,
    pub fcxp: *mut bfa_fcxp_s,
    pub fcxp_wqe: bfa_fcxp_wqe_s,
    pub num_rnnid_retries: u8,
    pub num_rsnn_nn_retries: u8,
}

//
// VPort SCN State Machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_scn_event {
    SCNSM_EVENT_PORT_ONLINE = 1,
    SCNSM_EVENT_PORT_OFFLINE = 2,
    SCNSM_EVENT_RSP_OK = 3,
    SCNSM_EVENT_RSP_ERROR = 4,
    SCNSM_EVENT_TIMEOUT = 5,
    SCNSM_EVENT_SCR_SENT = 6,
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_fcs_lport_scn_sm_t)(struct bfa_fcs_lport_scn_s, port_scn_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_scn_s {
    pub /: *mut *mut bfa_fcs_lport_scn_sm_t sm; / state machine,
    pub timer: bfa_timer_s,
    pub /: *mut *mut *mut bfa_fcs_lport_s port; / parent port,
    pub fcxp: *mut bfa_fcxp_s,
    pub fcxp_wqe: bfa_fcxp_wqe_s,
}

//
// FDMI State Machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_fdmi_event {
    FDMISM_EVENT_PORT_ONLINE = 1,
    FDMISM_EVENT_PORT_OFFLINE = 2,
    FDMISM_EVENT_RSP_OK = 4,
    FDMISM_EVENT_RSP_ERROR = 5,
    FDMISM_EVENT_TIMEOUT = 6,
    FDMISM_EVENT_RHBA_SENT = 7,
    FDMISM_EVENT_RPRT_SENT = 8,
    FDMISM_EVENT_RPA_SENT = 9,
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_fcs_lport_fdmi_sm_t)(struct bfa_fcs_lport_fdmi_s, port_fdmi_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_fdmi_s {
    pub /: *mut *mut bfa_fcs_lport_fdmi_sm_t sm; / state machine,
    pub timer: bfa_timer_s,
    pub /: *mut *mut *mut bfa_fcs_lport_ms_s ms; / parent ms,
    pub fcxp: *mut bfa_fcxp_s,
    pub fcxp_wqe: bfa_fcxp_wqe_s,
    pub /: *mut *mut u8 retry_cnt; / retry count,
    pub rsvd: [u8; 3],
}

//
// MS State Machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_ms_event {
    MSSM_EVENT_PORT_ONLINE = 1,
    MSSM_EVENT_PORT_OFFLINE = 2,
    MSSM_EVENT_RSP_OK = 3,
    MSSM_EVENT_RSP_ERROR = 4,
    MSSM_EVENT_TIMEOUT = 5,
    MSSM_EVENT_FCXP_SENT = 6,
    MSSM_EVENT_PORT_FABRIC_RSCN = 7
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_fcs_lport_ms_sm_t)(struct bfa_fcs_lport_ms_s, port_ms_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_ms_s {
    pub /: *mut *mut bfa_fcs_lport_ms_sm_t sm; / state machine,
    pub timer: bfa_timer_s,
    pub /: *mut *mut *mut bfa_fcs_lport_s port; / parent port,
    pub fcxp: *mut bfa_fcxp_s,
    pub fcxp_wqe: bfa_fcxp_wqe_s,
    pub /: *mut *mut bfa_fcs_lport_fdmi_s fdmi; / FDMI component of MS,
    pub /: *mut *mut u8 retry_cnt; / retry count,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_fab_s {
    pub /: *mut *mut bfa_fcs_lport_ns_s ns; / NS component of port,
    pub /: *mut *mut bfa_fcs_lport_scn_s scn; / scn component of port,
    pub /: *mut *mut bfa_fcs_lport_ms_s ms; / MS component of port,
}

pub const MAX_ALPA_COUNT: c_int = 127;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_loop_s {
    pub /: *mut *mut u8 num_alpa; / Num of ALPA entries in the map,
    pub /: *mut *mut u8 alpabm_valid; / alpa bitmap valid or not (1 or 0),
    pub /: *mut *mut u8 alpa_pos_map[MAX_ALPA_COUNT]; / ALPA Positional Map,
    pub /: *mut *mut *mut bfa_fcs_lport_s port; / parent port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_n2n_s {
    pub rsvd: u32,
    pub be: *mut *mut __be16 reply_oxid; / ox_id from the req flogi to,
// used in flogi acc
    pub /: *mut *mut wwn_t rem_port_wwn; / Attached port's wwn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfa_fcs_lport_topo_u {
    pub pfab: bfa_fcs_lport_fab_s,
    pub ploop: bfa_fcs_lport_loop_s,
    pub pn2n: bfa_fcs_lport_n2n_s,
}

//
// fcs_port_sm FCS logical port state machine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fcs_lport_event {
    BFA_FCS_PORT_SM_CREATE = 1,
    BFA_FCS_PORT_SM_ONLINE = 2,
    BFA_FCS_PORT_SM_OFFLINE = 3,
    BFA_FCS_PORT_SM_DELETE = 4,
    BFA_FCS_PORT_SM_DELRPORT = 5,
    BFA_FCS_PORT_SM_STOP = 6,
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_fcs_lport_sm_t)(struct bfa_fcs_lport_s, bfa_fcs_lport_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_lport_s {
    pub /: *mut *mut list_head qe; / used by port/vport,
    pub /: *mut *mut bfa_fcs_lport_sm_t sm; / state machine,
    pub /: *mut *mut *mut bfa_fcs_fabric_s fabric; / parent fabric,
    pub /: *mut *mut bfa_lport_cfg_s port_cfg; / port configuration,
    pub /: *mut *mut bfa_timer_s link_timer; / timer for link offline,
    pub /: *mut *mut u32 pid:24; / FC address,
    pub /: *mut *mut u8 lp_tag; / lport tag,
    pub /: *mut *mut u16 num_rports; / Num of r-ports,
    pub /: *mut *mut list_head rport_q; / queue of discovered r-ports,
    pub /: *mut *mut *mut bfa_fcs_s fcs; / FCS instance,
    pub /: *mut *mut bfa_fcs_lport_topo_u port_topo; / fabric/loop/n2n details,
    pub /: *mut *mut *mut bfad_port_s bfad_port; / driver peer instance,
    pub /: *mut *mut *mut bfa_fcs_vport_s vport; / NULL for base ports,
    pub fcxp: *mut bfa_fcxp_s,
    pub fcxp_wqe: bfa_fcxp_wqe_s,
    pub stats: bfa_lport_stats_s,
    pub /: *mut *mut bfa_wc_s wc; / waiting counter for events,
}

//
// forward declaration
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fcs_fabric_type {
    BFA_FCS_FABRIC_UNKNOWN = 0,
    BFA_FCS_FABRIC_SWITCHED = 1,
    BFA_FCS_FABRIC_N2N = 2,
    BFA_FCS_FABRIC_LOOP = 3,
}

//
// Fabric state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fcs_fabric_event {
    BFA_FCS_FABRIC_SM_CREATE        = 1,    /*  create from driver        */
    BFA_FCS_FABRIC_SM_DELETE        = 2,    /*  delete from driver        */
    BFA_FCS_FABRIC_SM_LINK_DOWN     = 3,    /*  link down from port      */
    BFA_FCS_FABRIC_SM_LINK_UP       = 4,    /*  link up from port         */
    BFA_FCS_FABRIC_SM_CONT_OP       = 5,    /*  flogi/auth continue op   */
    BFA_FCS_FABRIC_SM_RETRY_OP      = 6,    /*  flogi/auth retry op      */
    BFA_FCS_FABRIC_SM_NO_FABRIC     = 7,    /*  from flogi/auth           */
    BFA_FCS_FABRIC_SM_PERF_EVFP     = 8,    /*  from flogi/auth           */
    BFA_FCS_FABRIC_SM_ISOLATE       = 9,    /*  from EVFP processing     */
    BFA_FCS_FABRIC_SM_NO_TAGGING    = 10,   /*  no VFT tagging from EVFP */
    BFA_FCS_FABRIC_SM_DELAYED       = 11,   /*  timeout delay event      */
    BFA_FCS_FABRIC_SM_AUTH_FAILED   = 12,   /*  auth failed       */
    BFA_FCS_FABRIC_SM_AUTH_SUCCESS  = 13,   /*  auth successful           */
    BFA_FCS_FABRIC_SM_DELCOMP       = 14,   /*  all vports deleted event */
    BFA_FCS_FABRIC_SM_LOOPBACK      = 15,   /*  Received our own FLOGI   */
    BFA_FCS_FABRIC_SM_START         = 16,   /*  from driver       */
    BFA_FCS_FABRIC_SM_STOP		= 17,	/*  Stop from driver	*/
    BFA_FCS_FABRIC_SM_STOPCOMP	= 18,	/*  Stop completion	*/
    BFA_FCS_FABRIC_SM_LOGOCOMP	= 19,	/*  FLOGO completion	*/
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_fcs_fabric_sm_t)(struct bfa_fcs_fabric_s, bfa_fcs_fabric_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_fabric_s {
    pub /: *mut *mut list_head qe; / queue element,
    pub /: *mut *mut bfa_fcs_fabric_sm_t sm; / state machine,
    pub /: *mut *mut *mut bfa_fcs_s fcs; / FCS instance,
    pub /: *mut *mut bfa_fcs_lport_s bport; / base logical port,
    pub /: *mut *mut bfa_fcs_fabric_type fab_type; / fabric type,
    pub /: *mut *mut bfa_port_type oper_type; / current link topology,
    pub /: *mut *mut u8 is_vf; / is virtual fabric?,
    pub /: *mut *mut u8 is_npiv; / is NPIV supported ?,
    pub /: *mut *mut u8 is_auth; / is Security/Auth supported ?,
    pub /: *mut *mut u16 bb_credit; / BB credit from fabric,
    pub /: *mut *mut u16 vf_id; / virtual fabric ID,
    pub /: *mut *mut u16 num_vports; / num vports,
    pub rsvd: u16,
    pub /: *mut *mut list_head vport_q; / queue of virtual ports,
    pub /: *mut *mut list_head vf_q; / queue of virtual fabrics,
    pub /: *mut *mut *mut bfad_vf_s vf_drv; / driver vf structure,
    pub /: *mut *mut bfa_timer_s link_timer; / Link Failure timer. Vport,
    pub /: *mut *mut wwn_t fabric_name; / attached fabric name,
    pub /: *mut *mut bfa_boolean_t auth_reqd; / authentication required,
    pub /: *mut *mut bfa_timer_s delay_timer; / delay timer,
    pub /: *mut *mut u16 swp_vfid;/ switch port VF id,
    pub event_arg: },
    pub /: *mut *mut bfa_wc_s wc; / wait counter for delete,
    pub /: *mut *mut bfa_vf_stats_s stats; / fabric/vf stats,
    pub /: *mut *mut *mut bfa_lps_s lps; / lport login services,
    pub fabric_ip_addr: [u8; BFA_FCS_FABRIC_IPADDR_SZ],
// attached fabric's ip addr
    pub /: *mut *mut bfa_wc_s stop_wc; / wait counter for stop,
}

//
// The design calls for a single implementation of base fabric and vf.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_vf_event_s {
    pub undefined: u32,
}

//
// @todo : need to move to a global config file.
// Maximum Rports supported per port (physical/logical).
//

//
// Symbolic Name related defines
// Total bytes 255.
// Physical Port's symbolic name 128 bytes.
// For Vports, Vport's symbolic name is appended to the Physical port's
// Symbolic Name.
//
// Physical Port's symbolic name Format : (Total 128 bytes)
// Adapter Model number/name : 16 bytes
// Driver Version     : 10 bytes
// Host Machine Name  : 30 bytes
// Host OS Info	   : 44 bytes
// Host OS PATCH Info : 16 bytes
// ( remaining 12 bytes reserved to be used for separator)
//

pub const BFA_FCS_PORT_SYMBNAME_MODEL_SZ: c_int = 16;
pub const BFA_FCS_PORT_SYMBNAME_VERSION_SZ: c_int = 10;
pub const BFA_FCS_PORT_SYMBNAME_MACHINENAME_SZ: c_int = 30;
pub const BFA_FCS_PORT_SYMBNAME_OSINFO_SZ: c_int = 44;
pub const BFA_FCS_PORT_SYMBNAME_OSPATCH_SZ: c_int = 16;
//
// Get FC port ID for a logical port.
//

//
// bfa fcs port public functions
//
extern "C" {
    pub fn bfa_fcs_lport_is_online(port: *mut bfa_fcs_lport_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_fcs_lport_set_symname(port: *mut bfa_fcs_lport_s, symname: *mut c_char);
}
extern "C" {
    pub fn bfa_fcs_lport_clear_stats(fcs_port: *mut bfa_fcs_lport_s);
}
// MS FCS routines
extern "C" {
    pub fn bfa_fcs_lport_ms_init(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_ms_offline(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_ms_online(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_ms_fabric_rscn(port: *mut bfa_fcs_lport_s);
}
// FDMI FCS routines
extern "C" {
    pub fn bfa_fcs_lport_fdmi_init(ms: *mut bfa_fcs_lport_ms_s);
}
extern "C" {
    pub fn bfa_fcs_lport_fdmi_offline(ms: *mut bfa_fcs_lport_ms_s);
}
extern "C" {
    pub fn bfa_fcs_lport_fdmi_online(ms: *mut bfa_fcs_lport_ms_s);
}
extern "C" {
    pub fn bfa_fcs_lport_online(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_offline(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_delete(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_stop(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_ns_init(vport: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_ns_offline(vport: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_ns_online(vport: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_ns_query(port: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_scn_init(vport: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_scn_offline(vport: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_fab_scn_online(vport: *mut bfa_fcs_lport_s);
}
extern "C" {
    pub fn bfa_fcs_lport_lip_scn_online(port: *mut bfa_fcs_lport_t);
}
//
// VPort State Machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fcs_vport_event {
    BFA_FCS_VPORT_SM_CREATE = 1,	/*  vport create event */
    BFA_FCS_VPORT_SM_DELETE = 2,	/*  vport delete event */
    BFA_FCS_VPORT_SM_START = 3,	/*  vport start request */
    BFA_FCS_VPORT_SM_STOP = 4,	/*  stop: unsupported */
    BFA_FCS_VPORT_SM_ONLINE = 5,	/*  fabric online */
    BFA_FCS_VPORT_SM_OFFLINE = 6,	/*  fabric offline event */
    BFA_FCS_VPORT_SM_FRMSENT = 7,	/*  fdisc/logo sent events */
    BFA_FCS_VPORT_SM_RSP_OK = 8,	/*  good response */
    BFA_FCS_VPORT_SM_RSP_ERROR = 9,	/*  error/bad response */
    BFA_FCS_VPORT_SM_TIMEOUT = 10,	/*  delay timer event */
    BFA_FCS_VPORT_SM_DELCOMP = 11,	/*  lport delete completion */
    BFA_FCS_VPORT_SM_RSP_DUP_WWN = 12,	/*  Dup wnn error*/
    BFA_FCS_VPORT_SM_RSP_FAILED = 13,	/*  non-retryable failure */
    BFA_FCS_VPORT_SM_STOPCOMP = 14,	/* vport delete completion */
    BFA_FCS_VPORT_SM_FABRIC_MAX = 15, /* max vports on fabric */
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_fcs_vport_sm_t)(struct bfa_fcs_vport_s, bfa_fcs_vport_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_vport_s {
    pub /: *mut *mut list_head qe; / queue elem,
    pub /: *mut *mut bfa_fcs_vport_sm_t sm; / state machine,
    pub /: *mut *mut bfa_fcs_lport_t lport; / logical port,
    pub timer: bfa_timer_s,
    pub /: *mut *mut *mut bfad_vport_s vport_drv; / Driver private,
    pub /: *mut *mut bfa_vport_stats_s vport_stats; / vport statistics,
    pub service*/: *mut *mut *mut bfa_lps_s lps; / Lport login,
    pub fdisc_retries: c_int,
}

//
// bfa fcs vport public functions
//
extern "C" {
    pub fn bfa_fcs_vport_delete(vport: *mut bfa_fcs_vport_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcs_vport_start(vport: *mut bfa_fcs_vport_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcs_vport_stop(vport: *mut bfa_fcs_vport_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcs_vport_online(vport: *mut bfa_fcs_vport_s);
}
extern "C" {
    pub fn bfa_fcs_vport_offline(vport: *mut bfa_fcs_vport_s);
}
extern "C" {
    pub fn bfa_fcs_vport_delete_comp(vport: *mut bfa_fcs_vport_s);
}
extern "C" {
    pub fn bfa_fcs_vport_fcs_delete(vport: *mut bfa_fcs_vport_s);
}
extern "C" {
    pub fn bfa_fcs_vport_fcs_stop(vport: *mut bfa_fcs_vport_s);
}
extern "C" {
    pub fn bfa_fcs_vport_stop_comp(vport: *mut bfa_fcs_vport_s);
}

//
// forward declarations
//
// fcs_rport_ftrs_sm FCS rport state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpf_event {
    RPFSM_EVENT_RPORT_OFFLINE  = 1, /* Rport offline		*/
    RPFSM_EVENT_RPORT_ONLINE   = 2,	/* Rport online			*/
    RPFSM_EVENT_FCXP_SENT      = 3,	/* Frame from has been sent	*/
    RPFSM_EVENT_TIMEOUT	   = 4, /* Rport SM timeout event	*/
    RPFSM_EVENT_RPSC_COMP      = 5,
    RPFSM_EVENT_RPSC_FAIL      = 6,
    RPFSM_EVENT_RPSC_ERROR     = 7,
}

extern "C" {
    pub fn void(: *mut *mut bfa_fcs_rpf_sm_t)(struct bfa_fcs_rpf_s, rpf_event: enum) -> typedef;
}
// Rport Features (RPF)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_rpf_s {
    pub /: *mut *mut bfa_fcs_rpf_sm_t sm; / state machine,
    pub /: *mut *mut *mut bfa_fcs_rport_s rport; / parent rport,
    pub /: *mut *mut bfa_timer_s timer; / general purpose timer,
    pub /: *mut *mut *mut bfa_fcxp_s fcxp; / FCXP needed for discarding,
    pub /: *mut *mut bfa_fcxp_wqe_s fcxp_wqe; / fcxp wait queue element,
    pub /: *mut *mut int rpsc_retries; / max RPSC retry attempts,
    pub rpsc_speed: bfa_port_speed,
// Current Speed from RPSC. O if RPSC fails
    pub assigned_speed: bfa_port_speed,
//
// Speed assigned by the user.  will be used if RPSC is
// not supported by the rport.
//
}

//
// fcs_rport_sm FCS rport state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rport_event {
    RPSM_EVENT_PLOGI_SEND   = 1,    /*  new rport; start with PLOGI */
    RPSM_EVENT_PLOGI_RCVD   = 2,    /*  Inbound PLOGI from remote port */
    RPSM_EVENT_PLOGI_COMP   = 3,    /*  PLOGI completed to rport    */
    RPSM_EVENT_LOGO_RCVD    = 4,    /*  LOGO from remote device     */
    RPSM_EVENT_LOGO_IMP     = 5,    /*  implicit logo for SLER      */
    RPSM_EVENT_FCXP_SENT    = 6,    /*  Frame from has been sent    */
    RPSM_EVENT_DELETE       = 7,    /*  RPORT delete request        */
    RPSM_EVENT_FAB_SCN	= 8,    /*  state change notification   */
    RPSM_EVENT_ACCEPTED     = 9,    /*  Good response from remote device */
    RPSM_EVENT_FAILED       = 10,   /*  Request to rport failed.    */
    RPSM_EVENT_TIMEOUT      = 11,   /*  Rport SM timeout event      */
    RPSM_EVENT_HCB_ONLINE  = 12,    /*  BFA rport online callback   */
    RPSM_EVENT_HCB_OFFLINE = 13,    /*  BFA rport offline callback  */
    RPSM_EVENT_FC4_OFFLINE = 14,    /*  FC-4 offline complete       */
    RPSM_EVENT_ADDRESS_CHANGE = 15, /*  Rport's PID has changed     */
    RPSM_EVENT_ADDRESS_DISC = 16,   /*  Need to Discover rport's PID */
    RPSM_EVENT_PRLO_RCVD   = 17,    /*  PRLO from remote device     */
    RPSM_EVENT_PLOGI_RETRY = 18,    /*  Retry PLOGI continuously */
    RPSM_EVENT_SCN_OFFLINE = 19,	/* loop scn offline		*/
    RPSM_EVENT_SCN_ONLINE   = 20,	/* loop scn online		*/
    RPSM_EVENT_FC4_FCS_ONLINE = 21, /* FC-4 FCS online complete */
}

extern "C" {
    pub fn void(: *mut *mut bfa_fcs_rport_sm_t)(struct bfa_fcs_rport_s, rport_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_rport_s {
    pub /: *mut *mut list_head qe; / used by port/vport,
    pub /: *mut *mut *mut bfa_fcs_lport_s port; / parent FCS port,
    pub /: *mut *mut *mut bfa_fcs_s fcs; / fcs instance,
    pub /: *mut *mut *mut bfad_rport_s rp_drv; / driver peer instance,
    pub /: *mut *mut u32 pid; / port ID of rport,
    pub /: *mut *mut u32 old_pid; / PID before rport goes offline,
    pub /: *mut *mut u16 maxfrsize; / maximum frame size,
    pub /: *mut *mut __be16 reply_oxid; / OX_ID of inbound requests,
    pub /: *mut *mut fc_cos fc_cos; / FC classes of service supp,
    pub /: *mut *mut bfa_boolean_t cisc; / CISC capable device,
    pub /: *mut *mut bfa_boolean_t prlo; / processing prlo or LOGO,
    pub /: *mut *mut bfa_boolean_t plogi_pending; / Rx Plogi Pending,
    pub /: *mut *mut wwn_t pwwn; / port wwn of rport,
    pub /: *mut *mut wwn_t nwwn; / node wwn of rport,
    pub /: *mut *mut bfa_rport_symname_s psym_name; / port symbolic name,
    pub /: *mut *mut bfa_fcs_rport_sm_t sm; / state machine,
    pub /: *mut *mut bfa_timer_s timer; / general purpose timer,
    pub /: *mut *mut *mut bfa_fcs_itnim_s itnim; / ITN initiator mode role,
    pub /: *mut *mut *mut bfa_fcs_tin_s tin; / ITN initiator mode role,
    pub /: *mut *mut *mut bfa_fcs_iprp_s iprp; / IP/FC role,
    pub /: *mut *mut *mut bfa_rport_s bfa_rport; / BFA Rport,
    pub /: *mut *mut *mut bfa_fcxp_s fcxp; / FCXP needed for discarding,
    pub /: *mut *mut int plogi_retries; / max plogi retry attempts,
    pub /: *mut *mut int ns_retries; / max NS query retry attempts,
    pub /: *mut *mut bfa_fcxp_wqe_s fcxp_wqe; / fcxp wait queue element,
    pub /: *mut *mut bfa_rport_stats_s stats; / rport stats,
    pub /: *mut *mut bfa_rport_function scsi_function; / Initiator/Target,
    pub /: *mut *mut bfa_fcs_rpf_s rpf; / Rport features module,
    pub /: *mut *mut bfa_boolean_t scn_online; / SCN online flag,
}

//
// bfa fcs rport API functions
//
extern "C" {
    pub fn bfa_fcs_rport_set_del_timeout(rport_tmo: u8);
}
extern "C" {
    pub fn bfa_fcs_rport_set_max_logins(max_logins: u32);
}
extern "C" {
    pub fn bfa_fcs_rport_scn(rport: *mut bfa_fcs_rport_s);
}
extern "C" {
    pub fn bfa_fcs_rport_prlo(rport: *mut bfa_fcs_rport_s, ox_id: __be16);
}
extern "C" {
    pub fn bfa_fcs_rport_itntm_ack(rport: *mut bfa_fcs_rport_s);
}
extern "C" {
    pub fn bfa_fcs_rport_fcptm_offline_done(rport: *mut bfa_fcs_rport_s);
}
extern "C" {
    pub fn bfa_fcs_rport_get_state(rport: *mut bfa_fcs_rport_s) -> c_int;
}
extern "C" {
    pub fn bfa_fcs_rpf_init(rport: *mut bfa_fcs_rport_s);
}
extern "C" {
    pub fn bfa_fcs_rpf_rport_online(rport: *mut bfa_fcs_rport_s);
}
extern "C" {
    pub fn bfa_fcs_rpf_rport_offline(rport: *mut bfa_fcs_rport_s);
}
//
// fcs_itnim_sm FCS itnim state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fcs_itnim_event {
    BFA_FCS_ITNIM_SM_FCS_ONLINE = 1,        /*  rport online event */
    BFA_FCS_ITNIM_SM_OFFLINE = 2,   /*  rport offline */
    BFA_FCS_ITNIM_SM_FRMSENT = 3,   /*  prli frame is sent */
    BFA_FCS_ITNIM_SM_RSP_OK = 4,    /*  good response */
    BFA_FCS_ITNIM_SM_RSP_ERROR = 5, /*  error response */
    BFA_FCS_ITNIM_SM_TIMEOUT = 6,   /*  delay timeout */
    BFA_FCS_ITNIM_SM_HCB_OFFLINE = 7, /*  BFA online callback */
    BFA_FCS_ITNIM_SM_HCB_ONLINE = 8, /*  BFA offline callback */
    BFA_FCS_ITNIM_SM_INITIATOR = 9, /*  rport is initiator */
    BFA_FCS_ITNIM_SM_DELETE = 10,   /*  delete event from rport */
    BFA_FCS_ITNIM_SM_PRLO = 11,     /*  delete event from rport */
    BFA_FCS_ITNIM_SM_RSP_NOT_SUPP = 12, /* cmd not supported rsp */
    BFA_FCS_ITNIM_SM_HAL_ONLINE = 13, /* bfa rport online event */
}

extern "C" {
    pub fn void(: *mut *mut bfa_fcs_itnim_sm_t)(struct bfa_fcs_itnim_s, bfa_fcs_itnim_event: enum) -> typedef;
}
//
// forward declarations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_itnim_s {
    pub /: *mut *mut bfa_fcs_itnim_sm_t sm; / state machine,
    pub /: *mut *mut *mut bfa_fcs_rport_s rport; / parent remote rport,
    pub /: *mut *mut *mut bfad_itnim_s itnim_drv; / driver peer instance,
    pub /: *mut *mut *mut bfa_fcs_s fcs; / fcs instance,
    pub /: *mut *mut bfa_timer_s timer; / timer functions,
    pub /: *mut *mut *mut bfa_itnim_s bfa_itnim; / BFA itnim struct,
    pub /: *mut *mut u32 prli_retries; / max prli retry attempts,
    pub /: *mut *mut bfa_boolean_t seq_rec; / seq recovery support,
    pub /: *mut *mut bfa_boolean_t rec_support; / REC supported,
    pub /: *mut *mut bfa_boolean_t conf_comp; / FCP_CONF support,
    pub /: *mut *mut bfa_boolean_t task_retry_id; / task retry id supp,
    pub /: *mut *mut bfa_fcxp_wqe_s fcxp_wqe; / wait qelem for fcxp,
    pub /: *mut *mut *mut bfa_fcxp_s fcxp; / FCXP in use,
    pub /: *mut *mut bfa_itnim_stats_s stats; / itn statistics,
}

//
// bfa fcs FCP Initiator mode API functions
//
extern "C" {
    pub fn bfa_fcs_itnim_delete(itnim: *mut bfa_fcs_itnim_s);
}
extern "C" {
    pub fn bfa_fcs_itnim_rport_offline(itnim: *mut bfa_fcs_itnim_s);
}
extern "C" {
    pub fn bfa_fcs_itnim_brp_online(itnim: *mut bfa_fcs_itnim_s);
}
extern "C" {
    pub fn bfa_fcs_itnim_get_online_state(itnim: *mut bfa_fcs_itnim_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fcs_itnim_is_initiator(itnim: *mut bfa_fcs_itnim_s);
}

pub const BFA_FCS_FDMI_VENDOR_INFO_LEN: c_int = 8;
pub const BFA_FCS_FDMI_FC4_TYPE_LEN: c_int = 32;
//
// HBA Attribute Block : BFA internal representation. Note : Some variable
// sizes have been trimmed to suit BFA For Ex : Model will be "QLogic ". Based
// on this the size has been reduced to 16 bytes from the standard's 64 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_fdmi_hba_attr_s {
    pub node_name: wwn_t,
    pub manufacturer: [u8; 64],
    pub serial_num: [u8; 64],
    pub model: [u8; 16],
    pub model_desc: [u8; 128],
    pub hw_version: [u8; 8],
    pub driver_version: [u8; BFA_VERSION_LEN],
    pub option_rom_ver: [u8; BFA_VERSION_LEN],
    pub fw_version: [u8; BFA_VERSION_LEN],
    pub os_name: [u8; 256],
    pub max_ct_pyld: __be32,
    pub node_sym_name: bfa_lport_symname_s,
    pub vendor_info: [u8; BFA_FCS_FDMI_VENDOR_INFO_LEN],
    pub num_ports: __be32,
    pub fabric_name: wwn_t,
    pub bios_ver: [u8; BFA_VERSION_LEN],
}

//
// Port Attribute Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_fdmi_port_attr_s {
    pub supp_fc4_types: [u8; BFA_FCS_FDMI_FC4_TYPE_LEN],
    pub /: *mut *mut __be32 supp_speed; / supported speed,
    pub /: *mut *mut __be32 curr_speed; / current Speed,
    pub /: *mut *mut __be32 max_frm_size; / max frame size,
    pub /: *mut *mut u8 os_device_name[256]; / OS device Name,
    pub /: *mut *mut u8 host_name[256]; / host name,
    pub port_name: wwn_t,
    pub node_name: wwn_t,
    pub port_sym_name: bfa_lport_symname_s,
    pub port_type: __be32,
    pub scos: fc_cos,
    pub port_fabric_name: wwn_t,
    pub port_act_fc4_type: [u8; BFA_FCS_FDMI_FC4_TYPE_LEN],
    pub port_state: __be32,
    pub num_ports: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_stats_s {
    pub /: *mut *mut u32 untagged; / untagged receive frames,
    pub /: *mut *mut u32 tagged; / tagged receive frames,
    pub /: *mut *mut u32 vfid_unknown; / VF id is unknown,
    pub uf: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_driver_info_s {
    pub /: *mut *mut u8 version[BFA_VERSION_LEN]; / Driver Version,
    pub host_machine_name: [u8; BFA_FCS_OS_STR_LEN],
    pub /: *mut *mut u8 host_os_name[BFA_FCS_OS_STR_LEN]; / OS name and version,
    pub /: *mut *mut u8 host_os_patch[BFA_FCS_OS_STR_LEN]; / patch or service pack,
    pub /: *mut *mut u8 os_device_name[BFA_FCS_OS_STR_LEN]; / Driver Device Name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcs_s {
    pub /: *mut *mut *mut bfa_s bfa; / corresponding BFA bfa instance,
    pub /: *mut *mut *mut bfad_s bfad; / corresponding BDA driver instance,
    pub /: *mut *mut *mut bfa_trc_mod_s trcmod; / tracing module,
    pub /: *mut *mut bfa_boolean_t vf_enabled; / VF mode is enabled,
    pub /: *mut *mut bfa_boolean_t fdmi_enabled; / FDMI is enabled,
    pub /: *mut *mut bfa_boolean_t min_cfg; / min cfg enabled/disabled,
    pub /: *mut *mut u16 port_vfid; / port default VF ID,
    pub driver_info: bfa_fcs_driver_info_s,
    pub /: *mut *mut bfa_fcs_fabric_s fabric; / base fabric state machine,
    pub /: *mut *mut bfa_fcs_stats_s stats; / FCS statistics,
    pub /: *mut *mut bfa_wc_s wc; / waiting counter,
    pub fcs_aen_seq: c_int,
    pub num_rport_logins: u32,
}

//
// fcs_fabric_sm fabric state machine functions
//
// bfa fcs API functions
//
extern "C" {
    pub fn bfa_fcs_init(fcs: *mut bfa_fcs_s);
}
extern "C" {
    pub fn bfa_fcs_pbc_vport_init(fcs: *mut bfa_fcs_s);
}
extern "C" {
    pub fn bfa_fcs_update_cfg(fcs: *mut bfa_fcs_s);
}
extern "C" {
    pub fn bfa_fcs_exit(fcs: *mut bfa_fcs_s);
}
extern "C" {
    pub fn bfa_fcs_stop(fcs: *mut bfa_fcs_s);
}
//
// bfa fcs vf public functions
//
extern "C" {
    pub fn bfa_fcs_vf_get_ports(vf: *mut bfa_fcs_vf_t, vpwwn[]: wwn_t, nports: *mut c_int);
}
//
// fabric protected interface functions
//
extern "C" {
    pub fn bfa_fcs_fabric_modinit(fcs: *mut bfa_fcs_s);
}
extern "C" {
    pub fn bfa_fcs_fabric_link_up(fabric: *mut bfa_fcs_fabric_s);
}
extern "C" {
    pub fn bfa_fcs_fabric_link_down(fabric: *mut bfa_fcs_fabric_s);
}
extern "C" {
    pub fn bfa_fcs_fabric_modstart(fcs: *mut bfa_fcs_s);
}
extern "C" {
    pub fn bfa_fcs_fabric_psymb_init(fabric: *mut bfa_fcs_fabric_s);
}
extern "C" {
    pub fn bfa_fcs_fabric_nsymb_init(fabric: *mut bfa_fcs_fabric_s);
}
extern "C" {
    pub fn bfa_fcs_fabric_get_switch_oui(fabric: *mut bfa_fcs_fabric_s) -> u16;
}
extern "C" {
    pub fn bfa_fcs_fabric_modstop(fcs: *mut bfa_fcs_s);
}
//
// BFA FCS callback interfaces
//
// fcb Main fcs callbacks
//
// lport callbacks
//
// vport callbacks
//
extern "C" {
    pub fn bfa_fcb_pbc_vport_create(bfad: *mut bfad_s, bfi_pbc_vport_s: struct);
}
//
// rport callbacks
//
// itnim callbacks
//
extern "C" {
    pub fn bfa_fcb_itnim_online(itnim_drv: *mut bfad_itnim_s);
}
extern "C" {
    pub fn bfa_fcb_itnim_offline(itnim_drv: *mut bfad_itnim_s);
}
