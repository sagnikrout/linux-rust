//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_disc.h
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
// Copyright (C) 2017-2026 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2004-2013 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
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

// This is the protocol dependent definition for a Node List Entry.
// This is used by Fibre Channel protocol to support FCP.
//
// worker thread events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_work_type {
    LPFC_EVT_ONLINE,
    LPFC_EVT_OFFLINE_PREP,
    LPFC_EVT_OFFLINE,
    LPFC_EVT_WARM_START,
    LPFC_EVT_KILL,
    LPFC_EVT_ELS_RETRY,
    LPFC_EVT_DEV_LOSS,
    LPFC_EVT_FASTPATH_MGMT_EVT,
    LPFC_EVT_RESET_HBA,
    LPFC_EVT_RECOVER_PORT
}

// structure used to queue event to the discovery tasklet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_work_evt {
    pub evt_listp: list_head,
    pub evt_arg1: *mut c_void,
    pub evt_arg2: *mut c_void,
    pub evt: lpfc_work_type,
}

// structure used for sending events from fast path
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fast_path_event {
    pub work_evt: lpfc_work_evt,
    pub vport: *mut lpfc_vport,
    pub check_cond_evt: lpfc_scsi_check_condition_event,
    pub queue_depth_evt: lpfc_scsi_varqueuedepth_event,
    pub scsi_evt: lpfc_scsi_event_header,
    pub fabric_evt: lpfc_fabric_event_header,
    pub read_check_error: lpfc_fcprdchkerr_event,
    pub un: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_node_rrqs {
    pub xri_bitmap: [c_ulong; XRI_BITMAP_ULONGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_enc_info {
    pub /: *mut *mut u8 status; / encryption status for session,
    pub /: *mut *mut u8 level; / CNSA encryption level,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_fc4_xpt_flags {
    NLP_XPT_REGD		= 0x1,
    SCSI_XPT_REGD		= 0x2,
    NVME_XPT_REGD		= 0x4,
    NVME_XPT_UNREG_WAIT	= 0x8,
    NLP_XPT_HAS_HH		= 0x10
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_nlp_save_flags {
// devloss occurred during recovery
    NLP_IN_RECOV_POST_DEV_LOSS,
// wait for outstanding LOGO to cmpl
    NLP_WAIT_FOR_LOGO,
// wait for outstanding DA_ID to finish
    NLP_WAIT_FOR_DA_ID
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nodelist {
    pub nlp_listp: list_head,
    pub /: *mut *mut serv_parm fc_sparam; / buffer for service params,
    pub nlp_portname: lpfc_name,
    pub nlp_nodename: lpfc_name,
    pub /: *mut *mut spinlock_t lock; / Node management lock,
    pub /: *mut *mut unsigned long nlp_flag; / entry flags,
    pub /: *mut *mut uint32_t nlp_DID; / FC D_ID of entry,
    pub /: *mut *mut uint32_t nlp_last_elscmd; / Last ELS cmd sent,
    pub nlp_type: u16,
pub const NLP_FC_NODE: c_uint = 0x1			/* entry is an FC node */;
pub const NLP_FABRIC: c_uint = 0x4			/* entry rep a Fabric entity */;
pub const NLP_FCP_TARGET: c_uint = 0x8			/* entry is an FCP target */;
pub const NLP_FCP_INITIATOR: c_uint = 0x10			/* entry is an FCP Initiator */;
pub const NLP_NVME_TARGET: c_uint = 0x20			/* entry is a NVME Target */;
pub const NLP_NVME_INITIATOR: c_uint = 0x40			/* entry is a NVME Initiator */;
pub const NLP_NVME_DISCOVERY: c_uint = 0x80                 /* entry has NVME disc srvc */;
    pub /: *mut *mut uint16_t nlp_fc4_type; / FC types node supports.,
// Assigned from GID_FF, only
// FCP (0x8) and NVME (0x28)
// supported.
//
pub const NLP_FC4_NONE: c_uint = 0x0;
pub const NLP_FC4_FCP: c_uint = 0x1			/* FC4 Type FCP (value x8)) */;
pub const NLP_FC4_NVME: c_uint = 0x2			/* FC4 TYPE NVME (value x28) */;
    pub nlp_rpi: u16,
    pub /: *mut *mut uint16_t nlp_state; / state transition indicator,
    pub /: *mut *mut uint16_t nlp_prev_state; / state transition indicator,
    pub /: *mut *mut uint16_t nlp_xri; / output exchange id for RPI,
    pub /: *mut *mut uint16_t nlp_sid; / scsi id,
pub const NLP_NO_SID: c_uint = 0xffff;
    pub /: *mut *mut uint16_t nlp_maxframe; / Max RCV frame size,
    pub /: *mut *mut uint8_t nlp_class_sup; / Supported Classes,
    pub /: *mut *mut uint8_t nlp_retry; / used for ELS retries,
    pub /: *mut *mut uint8_t nlp_fcp_info; / class info, bits 0-2,
pub const NLP_FCP_CLASS_MASK: c_uint = 0x07			/* class info bitmask */;
pub const NLP_FCP_2_DEVICE: c_uint = 0x10			/* FCP-2 device */;
    pub /: *mut *mut u8 nlp_nvme_info; / NVME NSLER Support,
    pub /: *mut *mut uint8_t vmid_support; / destination VMID support,
pub const NLP_NVME_NSLER: c_uint = 0x1			/* NVME NSLER device */;
    pub /: *mut *mut lpfc_enc_info nlp_enc_info; / Encryption information struct,
    pub /: *mut *mut timer_list nlp_delayfunc; / Used for delayed ELS cmds,
    pub phba: *mut lpfc_hba,
    pub /: *mut *mut *mut fc_rport rport; / scsi_transport_fc port structure,
    pub /: *mut *mut *mut lpfc_nvme_rport nrport; / nvme transport rport struct.,
    pub vport: *mut lpfc_vport,
    pub els_retry_evt: lpfc_work_evt,
    pub dev_loss_evt: lpfc_work_evt,
    pub recovery_evt: lpfc_work_evt,
    pub kref: kref,
    pub cmd_pending: core::sync::atomic::AtomicI32,
    pub cmd_qdepth: u32,
    pub last_change_time: c_ulong,
    pub active_rrqs_xri_bitmap: *mut c_ulong,
    pub fc4_prli_sent: u32,
// flags to keep ndlp alive until special conditions are met
    pub save_flags: c_ulong,
    pub fc4_xpt_flags: lpfc_fc4_xpt_flags,
    pub /: *mut *mut uint32_t nvme_fb_size; / NVME target's supported byte cnt,

    pub nlp_defer_did: u32,
// These wait objects are NPIV specific.  These IOs must complete
// synchronously.
//
    pub logo_waitq: *mut wait_queue_head_t,
    pub da_id_waitq: *mut wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_node_rrq {
    pub list: list_head,
    pub xritag: u16,
    pub send_rrq: u16,
    pub rxid: u16,
    pub /: *mut *mut uint32_t nlp_DID; / FC D_ID of entry,
    pub vport: *mut lpfc_vport,
    pub rrq_stop_time: c_ulong,
}

// nlp_flag mask bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_nlp_flag {
    NLP_IGNR_REG_CMPL  = 0,         /* Rcvd rscn before we cmpl reg login */
    NLP_REG_LOGIN_SEND = 1,         /* sent reglogin to adapter */
    NLP_SUPPRESS_RSP   = 4,         /* Remote NPort supports suppress rsp */
    NLP_PLOGI_SND      = 5,         /* sent PLOGI request for this entry */
    NLP_PRLI_SND       = 6,         /* sent PRLI request for this entry */
    NLP_ADISC_SND      = 7,         /* sent ADISC request for this entry */
    NLP_LOGO_SND       = 8,         /* sent LOGO request for this entry */
    NLP_RNID_SND       = 10,        /* sent RNID request for this entry */
    NLP_NVMET_RECOV    = 12,        /* NVMET auditing node for recovery. */
    NLP_UNREG_INP      = 15,        /* UNREG_RPI cmd is in progress */
    NLP_DROPPED        = 16,        /* Init ref count has been dropped */
    NLP_DELAY_TMO      = 17,        /* delay timeout is running for node */
    NLP_NPR_2B_DISC    = 18,        /* node is included in num_disc_nodes */
    NLP_RCV_PLOGI      = 19,        /* Rcv'ed PLOGI from remote system */
    NLP_LOGO_ACC       = 20,        /* Process LOGO after ACC completes */
    NLP_TGT_NO_SCSIID  = 21,        /* good PRLI but no binding for scsid */
    NLP_ISSUE_LOGO     = 22,        /* waiting to issue a LOGO */
    NLP_IN_DEV_LOSS    = 23,        /* devloss in progress */
    NLP_ACC_REGLOGIN   = 24,        /* Issue Reg Login after successful
    ACC */
    NLP_NPR_ADISC      = 25,        /* Issue ADISC when dq'ed from
    NPR list */
    NLP_RM_DFLT_RPI    = 26,        /* need to remove leftover dflt RPI */
    NLP_NODEV_REMOVE   = 27,        /* Defer removal till discovery ends */
    NLP_FLOGI_DFR_ACC  = 28,        /* FLOGI LS_ACC was Deferred */
    NLP_SC_REQ         = 29,        /* Target requires authentication */
    NLP_FIRSTBURST     = 30,        /* Target supports FirstBurst */
    NLP_RPI_REGISTERED = 31         /* nlp_rpi is valid */
}

// There are 4 different double linked lists nodelist entries can reside on.
// The Port Login (PLOGI) list and Address Discovery (ADISC) list are used
// when Link Up discovery or Registered State Change Notification (RSCN)
// processing is needed.  Each list holds the nodes that require a PLOGI or
// ADISC Extended Link Service (ELS) request.  These lists keep track of the
// nodes affected by an RSCN, or a Link Up (Typically, all nodes are effected
// by Link Up) event.  The unmapped_list contains all nodes that have
// successfully logged into at the Fibre Channel level.  The
// mapped_list will contain all nodes that are mapped FCP targets.
//
// The bind list is a list of undiscovered (potentially non-existent) nodes
// that we have saved binding information on. This information is used when
// nodes transition from the unmapped to the mapped list.
//
// Defines for nlp_state
pub const NLP_STE_UNUSED_NODE: c_uint = 0x0	/* node is just allocated */;
pub const NLP_STE_PLOGI_ISSUE: c_uint = 0x1	/* PLOGI was sent to NL_PORT */;
pub const NLP_STE_ADISC_ISSUE: c_uint = 0x2	/* ADISC was sent to NL_PORT */;
pub const NLP_STE_REG_LOGIN_ISSUE: c_uint = 0x3	/* REG_LOGIN was issued for NL_PORT */;
pub const NLP_STE_PRLI_ISSUE: c_uint = 0x4	/* PRLI was sent to NL_PORT */;
pub const NLP_STE_LOGO_ISSUE: c_uint = 0x5	/* LOGO was sent to NL_PORT */;
pub const NLP_STE_UNMAPPED_NODE: c_uint = 0x6	/* PRLI completed from NL_PORT */;
pub const NLP_STE_MAPPED_NODE: c_uint = 0x7	/* Identified as a FCP Target */;
pub const NLP_STE_NPR_NODE: c_uint = 0x8	/* NPort disappeared */;
pub const NLP_STE_MAX_STATE: c_uint = 0x9;
pub const NLP_STE_FREED_NODE: c_uint = 0xff	/* node entry was freed to MEM_NLP */;
// For UNUSED_NODE state, the node has just been allocated.
// For PLOGI_ISSUE and REG_LOGIN_ISSUE, the node is on
// the PLOGI list. For REG_LOGIN_COMPL, the node is taken off the PLOGI list
// and put on the unmapped list. For ADISC processing, the node is taken off
// the ADISC list and placed on either the mapped or unmapped list (depending
// on its previous state). Once on the unmapped list, a PRLI is issued and the
// state changed to PRLI_ISSUE. When the PRLI completion occurs, the state is
// changed to PRLI_COMPL. If the completion indicates a mapped
// node, the node is taken off the unmapped list. The binding list is checked
// for a valid binding, or a binding is automatically assigned. If binding
// assignment is unsuccessful, the node is left on the unmapped list. If
// binding assignment is successful, the associated binding list entry (if
// any) is removed, and the node is placed on the mapped list.
//
// For a Link Down, all nodes on the ADISC, PLOGI, unmapped or mapped
// lists will receive a DEVICE_RECOVERY event. If the linkdown or devloss timers
// expire, all effected nodes will receive a DEVICE_RM event.
//
// For a Link Up or RSCN, all nodes will move from the mapped / unmapped lists
// to either the ADISC or PLOGI list.  After a Nameserver query or ALPA loopmap
// check, additional nodes may be added (DEVICE_ADD) or removed (DEVICE_RM) to
// from the PLOGI or ADISC lists. Once the PLOGI and ADISC lists are populated,
// we will first process the ADISC list.  32 entries are processed initially and
// ADISC is initited for each one.  Completions / Events for each node are
// funnelled thru the state machine.  As each node finishes ADISC processing, it
// starts ADISC for any nodes waiting for ADISC processing. If no nodes are
// waiting, and the ADISC list count is identically 0, then we are done. For
// Link Up discovery, since all nodes on the PLOGI list are UNREG_LOGIN'ed, we
// can issue a CLEAR_LA and reenable Link Events. Next we will process the PLOGI
// list.  32 entries are processed initially and PLOGI is initited for each one.
// Completions / Events for each node are funnelled thru the state machine.  As
// each node finishes PLOGI processing, it starts PLOGI for any nodes waiting
// for PLOGI processing. If no nodes are waiting, and the PLOGI list count is
// identically 0, then we are done. We have now completed discovery / RSCN
// handling. Upon completion, ALL nodes should be on either the mapped or
// unmapped lists.
//
// Defines for Node List Entry Events that could happen
pub const NLP_EVT_RCV_PLOGI: c_uint = 0x0	/* Rcv'd an ELS PLOGI command */;
pub const NLP_EVT_RCV_PRLI: c_uint = 0x1	/* Rcv'd an ELS PRLI  command */;
pub const NLP_EVT_RCV_LOGO: c_uint = 0x2	/* Rcv'd an ELS LOGO  command */;
pub const NLP_EVT_RCV_ADISC: c_uint = 0x3	/* Rcv'd an ELS ADISC command */;
pub const NLP_EVT_RCV_PDISC: c_uint = 0x4	/* Rcv'd an ELS PDISC command */;
pub const NLP_EVT_RCV_PRLO: c_uint = 0x5	/* Rcv'd an ELS PRLO  command */;
pub const NLP_EVT_CMPL_PLOGI: c_uint = 0x6	/* Sent an ELS PLOGI command */;
pub const NLP_EVT_CMPL_PRLI: c_uint = 0x7	/* Sent an ELS PRLI  command */;
pub const NLP_EVT_CMPL_LOGO: c_uint = 0x8	/* Sent an ELS LOGO  command */;
pub const NLP_EVT_CMPL_ADISC: c_uint = 0x9	/* Sent an ELS ADISC command */;
pub const NLP_EVT_CMPL_REG_LOGIN: c_uint = 0xa	/* REG_LOGIN mbox cmd completed */;
pub const NLP_EVT_DEVICE_RM: c_uint = 0xb	/* Device not found in NS / ALPAmap */;
pub const NLP_EVT_DEVICE_RECOVERY: c_uint = 0xc	/* Device existence unknown */;
pub const NLP_EVT_MAX_EVENT: c_uint = 0xd;
pub const NLP_EVT_NOTHING_PENDING: c_uint = 0xff;
