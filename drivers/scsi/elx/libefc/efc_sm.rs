//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/libefc/efc_sm.h
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
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//
// Generic state machine framework declarations.
//
// State Machine events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efc_sm_event {
// Common Events
    EFC_EVT_ENTER,
    EFC_EVT_REENTER,
    EFC_EVT_EXIT,
    EFC_EVT_SHUTDOWN,
    EFC_EVT_ALL_CHILD_NODES_FREE,
    EFC_EVT_RESUME,
    EFC_EVT_TIMER_EXPIRED,

// Domain Events
    EFC_EVT_RESPONSE,
    EFC_EVT_ERROR,

    EFC_EVT_DOMAIN_FOUND,
    EFC_EVT_DOMAIN_ALLOC_OK,
    EFC_EVT_DOMAIN_ALLOC_FAIL,
    EFC_EVT_DOMAIN_REQ_ATTACH,
    EFC_EVT_DOMAIN_ATTACH_OK,
    EFC_EVT_DOMAIN_ATTACH_FAIL,
    EFC_EVT_DOMAIN_LOST,
    EFC_EVT_DOMAIN_FREE_OK,
    EFC_EVT_DOMAIN_FREE_FAIL,
    EFC_EVT_HW_DOMAIN_REQ_ATTACH,
    EFC_EVT_HW_DOMAIN_REQ_FREE,

// Sport Events
    EFC_EVT_NPORT_ALLOC_OK,
    EFC_EVT_NPORT_ALLOC_FAIL,
    EFC_EVT_NPORT_ATTACH_OK,
    EFC_EVT_NPORT_ATTACH_FAIL,
    EFC_EVT_NPORT_FREE_OK,
    EFC_EVT_NPORT_FREE_FAIL,
    EFC_EVT_NPORT_TOPOLOGY_NOTIFY,
    EFC_EVT_HW_PORT_ALLOC_OK,
    EFC_EVT_HW_PORT_ALLOC_FAIL,
    EFC_EVT_HW_PORT_ATTACH_OK,
    EFC_EVT_HW_PORT_REQ_ATTACH,
    EFC_EVT_HW_PORT_REQ_FREE,
    EFC_EVT_HW_PORT_FREE_OK,

// Login Events
    EFC_EVT_SRRS_ELS_REQ_OK,
    EFC_EVT_SRRS_ELS_CMPL_OK,
    EFC_EVT_SRRS_ELS_REQ_FAIL,
    EFC_EVT_SRRS_ELS_CMPL_FAIL,
    EFC_EVT_SRRS_ELS_REQ_RJT,
    EFC_EVT_NODE_ATTACH_OK,
    EFC_EVT_NODE_ATTACH_FAIL,
    EFC_EVT_NODE_FREE_OK,
    EFC_EVT_NODE_FREE_FAIL,
    EFC_EVT_ELS_FRAME,
    EFC_EVT_ELS_REQ_TIMEOUT,
    EFC_EVT_ELS_REQ_ABORTED,
// request an ELS IO be aborted
    EFC_EVT_ABORT_ELS,
// ELS abort process complete
    EFC_EVT_ELS_ABORT_CMPL,

    EFC_EVT_ABTS_RCVD,

// node is not in the GID_PT payload
    EFC_EVT_NODE_MISSING,
// node is allocated and in the GID_PT payload
    EFC_EVT_NODE_REFOUND,
// node shutting down due to PLOGI recvd (implicit logo)
    EFC_EVT_SHUTDOWN_IMPLICIT_LOGO,
// node shutting down due to LOGO recvd/sent (explicit logo)
    EFC_EVT_SHUTDOWN_EXPLICIT_LOGO,

    EFC_EVT_PLOGI_RCVD,
    EFC_EVT_FLOGI_RCVD,
    EFC_EVT_LOGO_RCVD,
    EFC_EVT_PRLI_RCVD,
    EFC_EVT_PRLO_RCVD,
    EFC_EVT_PDISC_RCVD,
    EFC_EVT_FDISC_RCVD,
    EFC_EVT_ADISC_RCVD,
    EFC_EVT_RSCN_RCVD,
    EFC_EVT_SCR_RCVD,
    EFC_EVT_ELS_RCVD,

    EFC_EVT_FCP_CMD_RCVD,

    EFC_EVT_GIDPT_DELAY_EXPIRED,

// SCSI Target Server events
    EFC_EVT_NODE_ACTIVE_IO_LIST_EMPTY,
    EFC_EVT_NODE_DEL_INI_COMPLETE,
    EFC_EVT_NODE_DEL_TGT_COMPLETE,
    EFC_EVT_NODE_SESS_REG_OK,
    EFC_EVT_NODE_SESS_REG_FAIL,

// Must be last
    EFC_EVT_LAST
}

// State Machine event name lookup array

extern "C" {
    pub fn efc_sm_disable(ctx: *mut efc_sm_ctx);
}
