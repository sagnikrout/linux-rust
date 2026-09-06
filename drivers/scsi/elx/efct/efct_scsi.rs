//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/efct/efct_scsi.h
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

// efct_scsi_rcv_cmd() efct_scsi_rcv_tmf() flags

// efct_scsi_send_rd_data/recv_wr_data/send_resp flags

pub const EFCT_SCSI_WQ_STEERING_SHIFT: c_int = 16;

pub const EFCT_SCSI_WQ_CLASS_LOW_LATENCY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_scsi_cmd_resp {
    pub scsi_status: u8,
    pub scsi_status_qualifier: u16,
    pub response_data: *mut u8,
    pub response_data_length: u32,
    pub sense_data: *mut u8,
    pub sense_data_length: u32,
    pub residual: c_int,
    pub response_wire_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_vport {
    pub efct: *mut efct,
    pub is_vport: bool,
    pub fc_host_stats: fc_host_statistics,
    pub shost: *mut Scsi_Host,
    pub fc_vport: *mut fc_vport,
    pub npiv_wwpn: u64,
    pub npiv_wwnn: u64,
}

// Status values returned by IO callbacks
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_scsi_io_status {
    EFCT_SCSI_STATUS_GOOD = 0,
    EFCT_SCSI_STATUS_ABORTED,
    EFCT_SCSI_STATUS_ERROR,
    EFCT_SCSI_STATUS_DIF_GUARD_ERR,
    EFCT_SCSI_STATUS_DIF_REF_TAG_ERROR,
    EFCT_SCSI_STATUS_DIF_APP_TAG_ERROR,
    EFCT_SCSI_STATUS_DIF_UNKNOWN_ERROR,
    EFCT_SCSI_STATUS_PROTOCOL_CRC_ERROR,
    EFCT_SCSI_STATUS_NO_IO,
    EFCT_SCSI_STATUS_ABORT_IN_PROGRESS,
    EFCT_SCSI_STATUS_CHECK_RESPONSE,
    EFCT_SCSI_STATUS_COMMAND_TIMEOUT,
    EFCT_SCSI_STATUS_TIMEDOUT_AND_ABORTED,
    EFCT_SCSI_STATUS_SHUTDOWN,
    EFCT_SCSI_STATUS_NEXUS_LOST,
}

// Callback used by send_rd_data(), recv_wr_data(), send_resp()
// Callback used by send_rd_io(), send_wr_io()
// efct_scsi_cb_t flags

// IO completed, response sent

// efct_scsi_recv_tmf() request values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_scsi_tmf_cmd {
    EFCT_SCSI_TMF_ABORT_TASK = 1,
    EFCT_SCSI_TMF_QUERY_TASK_SET,
    EFCT_SCSI_TMF_ABORT_TASK_SET,
    EFCT_SCSI_TMF_CLEAR_TASK_SET,
    EFCT_SCSI_TMF_QUERY_ASYNCHRONOUS_EVENT,
    EFCT_SCSI_TMF_LOGICAL_UNIT_RESET,
    EFCT_SCSI_TMF_CLEAR_ACA,
    EFCT_SCSI_TMF_TARGET_RESET,
}

// efct_scsi_send_tmf_resp() response values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_scsi_tmf_resp {
    EFCT_SCSI_TMF_FUNCTION_COMPLETE = 1,
    EFCT_SCSI_TMF_FUNCTION_SUCCEEDED,
    EFCT_SCSI_TMF_FUNCTION_IO_NOT_FOUND,
    EFCT_SCSI_TMF_FUNCTION_REJECTED,
    EFCT_SCSI_TMF_INCORRECT_LOGICAL_UNIT_NUMBER,
    EFCT_SCSI_TMF_SERVICE_DELIVERY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_scsi_sgl {
    pub addr: uintptr_t,
    pub dif_addr: uintptr_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_scsi_io_role {
    EFCT_SCSI_IO_ROLE_ORIGINATOR,
    EFCT_SCSI_IO_ROLE_RESPONDER,
}

extern "C" {
    pub fn efct_scsi_io_free(io: *mut efct_io);
}
extern "C" {
    pub fn efct_scsi_tgt_driver_init() -> c_int;
}
extern "C" {
    pub fn efct_scsi_tgt_driver_exit() -> c_int;
}
extern "C" {
    pub fn efct_scsi_tgt_new_device(efct: *mut efct) -> c_int;
}
extern "C" {
    pub fn efct_scsi_tgt_del_device(efct: *mut efct) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_scsi_del_initiator_reason {
    EFCT_SCSI_INITIATOR_DELETED,
    EFCT_SCSI_INITIATOR_MISSING,
}

extern "C" {
    pub fn efct_scsi_io_complete(io: *mut efct_io);
}
extern "C" {
    pub fn efct_scsi_reg_fc_transport() -> c_int;
}
extern "C" {
    pub fn efct_scsi_release_fc_transport();
}
extern "C" {
    pub fn efct_scsi_new_device(efct: *mut efct) -> c_int;
}
extern "C" {
    pub fn efct_scsi_del_device(efct: *mut efct);
}
extern "C" {
    pub fn _efct_scsi_io_free(arg: *mut kref);
}
extern "C" {
    pub fn efct_scsi_io_dispatch(io: *mut efct_io, cb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn efct_scsi_io_dispatch_abort(io: *mut efct_io, cb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn efct_scsi_check_pending(efct: *mut efct);
}
