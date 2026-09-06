//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_defs_fcs.h
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

//
// VF states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_vf_state {
    BFA_VF_UNINIT    = 0,	/*  fabric is not yet initialized */
    BFA_VF_LINK_DOWN = 1,	/*  link is down */
    BFA_VF_FLOGI     = 2,	/*  flogi is in progress */
    BFA_VF_AUTH      = 3,	/*  authentication in progress */
    BFA_VF_NOFABRIC  = 4,	/*  fabric is not present */
    BFA_VF_ONLINE    = 5,	/*  login to fabric is complete */
    BFA_VF_EVFP      = 6,	/*  EVFP is in progress */
    BFA_VF_ISOLATED  = 7,	/*  port isolated due to vf_id mismatch */
}

//
// VF statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_vf_stats_s {
    pub /: *mut *mut u32 flogi_sent; / Num FLOGIs sent,
    pub /: *mut *mut u32 flogi_rsp_err; / FLOGI response errors,
    pub /: *mut *mut u32 flogi_acc_err; / FLOGI accept errors,
    pub /: *mut *mut u32 flogi_accepts; / FLOGI accepts received,
    pub /: *mut *mut u32 flogi_rejects; / FLOGI rejects received,
    pub /: *mut *mut u32 flogi_unknown_rsp; / Unknown responses for FLOGI,
    pub /: *mut *mut u32 flogi_alloc_wait; / Allocation waits prior to sending FLOGI,
    pub /: *mut *mut u32 flogi_rcvd; / FLOGIs received,
    pub /: *mut *mut u32 flogi_rejected; / Incoming FLOGIs rejected,
    pub sent: *mut *mut u32 fabric_onlines; / Internal fabric online notification,
// to other modules
    pub sent: *mut *mut u32 fabric_offlines; / Internal fabric offline notification,
// to other modules
    pub /: *mut *mut u32 resvd; / padding for 64 bit alignment,
}

//
// VF attributes returned in queries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_vf_attr_s {
    pub /: *mut *mut bfa_vf_state state; / VF state,
    pub rsvd: u32,
    pub /: *mut *mut wwn_t fabric_name; / fabric name,
}

pub const BFA_FCS_MAX_LPORTS: c_int = 256;
pub const BFA_FCS_FABRIC_IPADDR_SZ: c_int = 16;
//
// symbolic names for base port/virtual port
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lport_symname_s {
    pub symname: [c_char; BFA_SYMNAME_MAXLEN],
}

//
// Roles of FCS port:
// - FCP IM and FCP TM roles cannot be enabled together for a FCS port
// - Create multiple ports if both IM and TM functions required.
// - Atleast one role must be specified.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_lport_role {
    BFA_LPORT_ROLE_FCP_IM	= 0x01,	/*  FCP initiator role */
    BFA_LPORT_ROLE_FCP_MAX	= BFA_LPORT_ROLE_FCP_IM,
}

//
// FCS port configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lport_cfg_s {
    pub /: *mut *mut wwn_t pwwn; / port wwn,
    pub /: *mut *mut wwn_t nwwn; / node wwn,
    pub /: *mut *mut bfa_lport_symname_s sym_name; / vm port symbolic name,
    pub /: *mut *mut bfa_lport_symname_s node_sym_name; / Node symbolic name,
    pub /: *mut *mut bfa_lport_role roles; / FCS port roles,
    pub rsvd: u32,
    pub /: *mut *mut bfa_boolean_t preboot_vp; / vport created from PBC,
    pub /: *mut *mut u8 tag[16]; / opaque tag from application,
    pub padding: [u8; 4],
}

//
// FCS port states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_lport_state {
    BFA_LPORT_UNINIT  = 0,	/*  PORT is not yet initialized */
    BFA_LPORT_FDISC   = 1,	/*  FDISC is in progress */
    BFA_LPORT_ONLINE  = 2,	/*  login to fabric is complete */
    BFA_LPORT_OFFLINE = 3,	/*  No login to fabric */
}

//
// FCS port type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_lport_type {
    BFA_LPORT_TYPE_PHYSICAL = 0,
    BFA_LPORT_TYPE_VIRTUAL,
}

//
// FCS port offline reason.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_lport_offline_reason {
    BFA_LPORT_OFFLINE_UNKNOWN = 0,
    BFA_LPORT_OFFLINE_LINKDOWN,
    BFA_LPORT_OFFLINE_FAB_UNSUPPORTED,	/*  NPIV not supported by the
// fabric
    BFA_LPORT_OFFLINE_FAB_NORESOURCES,
    BFA_LPORT_OFFLINE_FAB_LOGOUT,
}

//
// FCS port statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lport_stats_s {
    pub ns_plogi_sent: u32,
    pub ns_plogi_rsp_err: u32,
    pub ns_plogi_acc_err: u32,
    pub ns_plogi_accepts: u32,
    pub /: *mut *mut u32 ns_rejects; / NS command rejects,
    pub ns_plogi_unknown_rsp: u32,
    pub ns_plogi_alloc_wait: u32,
    pub /: *mut *mut u32 ns_retries; / NS command retries,
    pub /: *mut *mut u32 ns_timeouts; / NS command timeouts,
    pub ns_rspnid_sent: u32,
    pub ns_rspnid_accepts: u32,
    pub ns_rspnid_rsp_err: u32,
    pub ns_rspnid_rejects: u32,
    pub ns_rspnid_alloc_wait: u32,
    pub ns_rftid_sent: u32,
    pub ns_rftid_accepts: u32,
    pub ns_rftid_rsp_err: u32,
    pub ns_rftid_rejects: u32,
    pub ns_rftid_alloc_wait: u32,
    pub ns_rffid_sent: u32,
    pub ns_rffid_accepts: u32,
    pub ns_rffid_rsp_err: u32,
    pub ns_rffid_rejects: u32,
    pub ns_rffid_alloc_wait: u32,
    pub ns_gidft_sent: u32,
    pub ns_gidft_accepts: u32,
    pub ns_gidft_rsp_err: u32,
    pub ns_gidft_rejects: u32,
    pub ns_gidft_unknown_rsp: u32,
    pub ns_gidft_alloc_wait: u32,
    pub ns_rnnid_sent: u32,
    pub ns_rnnid_accepts: u32,
    pub ns_rnnid_rsp_err: u32,
    pub ns_rnnid_rejects: u32,
    pub ns_rnnid_alloc_wait: u32,
    pub ns_rsnn_nn_sent: u32,
    pub ns_rsnn_nn_accepts: u32,
    pub ns_rsnn_nn_rsp_err: u32,
    pub ns_rsnn_nn_rejects: u32,
    pub ns_rsnn_nn_alloc_wait: u32,
//
// Mgmt Server stats
//
    pub /: *mut *mut u32 ms_retries; / MS command retries,
    pub /: *mut *mut u32 ms_timeouts; / MS command timeouts,
    pub ms_plogi_sent: u32,
    pub ms_plogi_rsp_err: u32,
    pub ms_plogi_acc_err: u32,
    pub ms_plogi_accepts: u32,
    pub /: *mut *mut u32 ms_rejects; / MS command rejects,
    pub ms_plogi_unknown_rsp: u32,
    pub ms_plogi_alloc_wait: u32,
    pub /: *mut *mut u32 num_rscn; / Num of RSCN received,
    pub RSCN: *mut *mut u32 num_portid_rscn;/ Num portid format,
// received
    pub /: *mut *mut u32 uf_recvs; / Unsolicited recv frames,
    pub /: *mut *mut u32 uf_recv_drops; / Dropped received frames,
    pub /: *mut *mut u32 plogi_rcvd; / Received plogi,
    pub /: *mut *mut u32 prli_rcvd; / Received prli,
    pub /: *mut *mut u32 adisc_rcvd; / Received adisc,
    pub /: *mut *mut u32 prlo_rcvd; / Received prlo,
    pub /: *mut *mut u32 logo_rcvd; / Received logo,
    pub /: *mut *mut u32 rpsc_rcvd; / Received rpsc,
    pub /: *mut *mut u32 un_handled_els_rcvd; / Received unhandled ELS,
    pub /: *mut *mut u32 rport_plogi_timeouts; / Rport plogi retry timeout count,
    pub rport: *mut *mut u32 rport_del_max_plogi_retry; / Deleted,
// (max retry of plogi)
}

//
// BFA port attribute returned in queries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lport_attr_s {
    pub /: *mut *mut bfa_lport_state state; / port state,
    pub /: *mut *mut u32 pid; / port ID,
    pub /: *mut *mut bfa_lport_cfg_s port_cfg; / port configuration,
    pub /: *mut *mut bfa_port_type port_type; / current topology,
    pub /: *mut *mut u32 loopback; / cable is externally looped back,
    pub /: *mut *mut wwn_t fabric_name; / attached switch's nwwn,
    pub attached: *mut *mut u8 fabric_ip_addr[BFA_FCS_FABRIC_IPADDR_SZ]; /,
// fabric's ip addr
    pub /: *mut *mut mac_t fpma_mac; / Lport's FPMA Mac address,
    pub /: *mut *mut u16 authfail; / auth failed state,
}

//
// VPORT states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_vport_state {
    BFA_FCS_VPORT_UNINIT		= 0,
    BFA_FCS_VPORT_CREATED		= 1,
    BFA_FCS_VPORT_OFFLINE		= 1,
    BFA_FCS_VPORT_FDISC_SEND	= 2,
    BFA_FCS_VPORT_FDISC		= 3,
    BFA_FCS_VPORT_FDISC_RETRY	= 4,
    BFA_FCS_VPORT_FDISC_RSP_WAIT	= 5,
    BFA_FCS_VPORT_ONLINE		= 6,
    BFA_FCS_VPORT_DELETING		= 7,
    BFA_FCS_VPORT_CLEANUP		= 8,
    BFA_FCS_VPORT_LOGO_SEND		= 9,
    BFA_FCS_VPORT_LOGO		= 10,
    BFA_FCS_VPORT_ERROR		= 11,
    BFA_FCS_VPORT_MAX_STATE,
}

//
// vport statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_vport_stats_s {
    pub /: *mut *mut bfa_lport_stats_s port_stats; / base class (port) stats,
//
// TODO - remove
//
    pub /: *mut *mut u32 fdisc_sent; / num fdisc sent,
    pub /: *mut *mut u32 fdisc_accepts; / fdisc accepts,
    pub /: *mut *mut u32 fdisc_retries; / fdisc retries,
    pub /: *mut *mut u32 fdisc_timeouts; / fdisc timeouts,
    pub /: *mut *mut u32 fdisc_rsp_err; / fdisc response error,
    pub /: *mut *mut u32 fdisc_acc_bad; / bad fdisc accepts,
    pub /: *mut *mut u32 fdisc_rejects; / fdisc rejects,
    pub fdisc_unknown_rsp: u32,
//
// !< fdisc rsp unknown error
//
    pub /: *mut *mut u32 fdisc_alloc_wait;/ fdisc req (fcxp)alloc wait,
    pub /: *mut *mut u32 logo_alloc_wait;/ logo req (fcxp) alloc wait,
    pub /: *mut *mut u32 logo_sent; / logo sent,
    pub /: *mut *mut u32 logo_accepts; / logo accepts,
    pub /: *mut *mut u32 logo_rejects; / logo rejects,
    pub /: *mut *mut u32 logo_rsp_err; / logo rsp errors,
    pub logo_unknown_rsp: u32,
// logo rsp unknown errors
    pub /: *mut *mut u32 fab_no_npiv; / fabric does not support npiv,
    pub /: *mut *mut u32 fab_offline; / offline events from fab SM,
    pub /: *mut *mut u32 fab_online; / online events from fab SM,
    pub /: *mut *mut u32 fab_cleanup; / cleanup request from fab SM,
    pub rsvd: u32,
}

//
// BFA vport attribute returned in queries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_vport_attr_s {
    pub /: *mut *mut bfa_lport_attr_s port_attr; / base class (port) attributes,
    pub /: *mut *mut bfa_vport_state vport_state; / vport state,
    pub rsvd: u32,
}

//
// FCS remote port states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_rport_state {
    BFA_RPORT_UNINIT	= 0,	/*  PORT is not yet initialized */
    BFA_RPORT_OFFLINE	= 1,	/*  rport is offline */
    BFA_RPORT_PLOGI		= 2,	/*  PLOGI to rport is in progress */
    BFA_RPORT_ONLINE	= 3,	/*  login to rport is complete */
    BFA_RPORT_PLOGI_RETRY	= 4,	/*  retrying login to rport */
    BFA_RPORT_NSQUERY	= 5,	/*  nameserver query */
    BFA_RPORT_ADISC		= 6,	/*  ADISC authentication */
    BFA_RPORT_LOGO		= 7,	/*  logging out with rport */
    BFA_RPORT_LOGORCV	= 8,	/*  handling LOGO from rport */
    BFA_RPORT_NSDISC	= 9,	/*  re-discover rport */
}

//
// Rport Scsi Function : Initiator/Target.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_rport_function {
    BFA_RPORT_INITIATOR	= 0x01,	/*  SCSI Initiator	*/
    BFA_RPORT_TARGET	= 0x02,	/*  SCSI Target	*/
}

//
// port/node symbolic names for rport
//
pub const BFA_RPORT_SYMNAME_MAXLEN: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_symname_s {
    pub symname: [c_char; BFA_RPORT_SYMNAME_MAXLEN],
}

//
// FCS remote port statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_stats_s {
    pub /: *mut *mut u32 offlines; / remote port offline count,
    pub /: *mut *mut u32 onlines; / remote port online count,
    pub /: *mut *mut u32 rscns; / RSCN affecting rport,
    pub /: *mut *mut u32 plogis; / plogis sent,
    pub /: *mut *mut u32 plogi_accs; / plogi accepts,
    pub /: *mut *mut u32 plogi_timeouts; / plogi timeouts,
    pub /: *mut *mut u32 plogi_rejects; / rcvd plogi rejects,
    pub /: *mut *mut u32 plogi_failed; / local failure,
    pub /: *mut *mut u32 plogi_rcvd; / plogis rcvd,
    pub /: *mut *mut u32 prli_rcvd; / inbound PRLIs,
    pub /: *mut *mut u32 adisc_rcvd; / ADISCs received,
    pub /: *mut *mut u32 adisc_rejects; / recvd ADISC rejects,
    pub /: *mut *mut u32 adisc_sent; / ADISC requests sent,
    pub /: *mut *mut u32 adisc_accs; / ADISC accepted by rport,
    pub /: *mut *mut u32 adisc_failed; / ADISC failed (no response),
    pub /: *mut *mut u32 adisc_rejected; / ADISC rejected by us,
    pub /: *mut *mut u32 logos; / logos sent,
    pub /: *mut *mut u32 logo_accs; / LOGO accepts from rport,
    pub /: *mut *mut u32 logo_failed; / LOGO failures,
    pub /: *mut *mut u32 logo_rejected; / LOGO rejects from rport,
    pub /: *mut *mut u32 logo_rcvd; / LOGO from remote port,
    pub /: *mut *mut u32 rpsc_rcvd; / RPSC received,
    pub /: *mut *mut u32 rpsc_rejects; / recvd RPSC rejects,
    pub /: *mut *mut u32 rpsc_sent; / RPSC requests sent,
    pub /: *mut *mut u32 rpsc_accs; / RPSC accepted by rport,
    pub /: *mut *mut u32 rpsc_failed; / RPSC failed (no response),
    pub /: *mut *mut u32 rpsc_rejected; / RPSC rejected by us,
    pub /: *mut *mut u32 rjt_insuff_res; / LS RJT with insuff resources,
    pub /: *mut *mut bfa_rport_hal_stats_s hal_stats; / BFA rport stats,
}

//
// FCS remote port attributes returned in queries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_attr_s {
    pub /: *mut *mut wwn_t nwwn; / node wwn,
    pub /: *mut *mut wwn_t pwwn; / port wwn,
    pub /: *mut *mut fc_cos cos_supported; / supported class of services,
    pub /: *mut *mut u32 pid; / port ID,
    pub /: *mut *mut u32 df_sz; / Max payload size,
    pub /: *mut *mut bfa_rport_state state; / Rport State machine state,
    pub /: *mut *mut fc_cos fc_cos; / FC classes of services,
    pub /: *mut *mut bfa_boolean_t cisc; / CISC capable device,
    pub /: *mut *mut bfa_rport_symname_s symname; / Symbolic Name,
    pub /: *mut *mut bfa_rport_function scsi_function; / Initiator/Target,
    pub /: *mut *mut bfa_rport_qos_attr_s qos_attr; / qos attributes,
    pub from: *mut *mut bfa_port_speed curr_speed; / operating speed got,
// RPSC ELS. UNKNOWN, if RPSC
// is not supported
    pub /: *mut *mut bfa_boolean_t trl_enforced; / TRL enforced ? TRUE/FALSE,
    pub user.: *mut *mut bfa_port_speed assigned_speed; / Speed assigned by the,
// will be used if RPSC is not
// supported by the rport
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_remote_link_stats_s {
    pub /: *mut *mut u32 lfc; / Link Failure Count,
    pub /: *mut *mut u32 lsyc; / Loss of Synchronization Count,
    pub /: *mut *mut u32 lsic; / Loss of Signal Count,
    pub /: *mut *mut u32 pspec; / Primitive Sequence Protocol Error Count,
    pub /: *mut *mut u32 itwc; / Invalid Transmission Word Count,
    pub /: *mut *mut u32 icc; / Invalid CRC Count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_qualifier_s {
    pub /: *mut *mut wwn_t pwwn; / Port WWN,
    pub /: *mut *mut u32 pid; / port ID,
    pub rsvd: u32,
}

pub const BFA_MAX_IO_INDEX: c_int = 7;
pub const BFA_NO_IO_INDEX: c_int = 9;
//
// FCS itnim states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_itnim_state {
    BFA_ITNIM_OFFLINE	= 0,	/*  offline */
    BFA_ITNIM_PRLI_SEND	= 1,	/*  prli send */
    BFA_ITNIM_PRLI_SENT	= 2,	/*  prli sent */
    BFA_ITNIM_PRLI_RETRY	= 3,	/*  prli retry */
    BFA_ITNIM_HCB_ONLINE	= 4,	/*  online callback */
    BFA_ITNIM_ONLINE	= 5,	/*  online */
    BFA_ITNIM_HCB_OFFLINE	= 6,	/*  offline callback */
    BFA_ITNIM_INITIATIOR	= 7,	/*  initiator */
}

//
// FCS remote port statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itnim_stats_s {
    pub /: *mut *mut u32 onlines; / num rport online,
    pub /: *mut *mut u32 offlines; / num rport offline,
    pub /: *mut *mut u32 prli_sent; / num prli sent out,
    pub /: *mut *mut u32 fcxp_alloc_wait;/ num fcxp alloc waits,
    pub /: *mut *mut u32 prli_rsp_err; / num prli rsp errors,
    pub /: *mut *mut u32 prli_rsp_acc; / num prli rsp accepts,
    pub /: *mut *mut u32 initiator; / rport is an initiator,
    pub /: *mut *mut u32 prli_rsp_parse_err; / prli rsp parsing errors,
    pub /: *mut *mut u32 prli_rsp_rjt; / num prli rsp rejects,
    pub /: *mut *mut u32 timeout; / num timeouts detected,
    pub /: *mut *mut u32 sler; / num sler notification from BFA,
    pub /: *mut *mut u32 rsvd; / padding for 64 bit alignment,
}

//
// FCS itnim attributes returned in queries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itnim_attr_s {
    pub /: *mut *mut bfa_itnim_state state; / FCS itnim state,
    pub /: *mut *mut u8 retry; / data retransmision support,
    pub /: *mut *mut u8 task_retry_id; / task retry ident support,
    pub /: *mut *mut u8 rec_support; / REC supported,
    pub /: *mut *mut u8 conf_comp; / confirmed completion supp,
}
