//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cdx/mcdi.h
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
// Copyright 2008-2013 Solarflare Communications Inc.
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

//
// enum cdx_mcdi_mode - MCDI transaction mode
// @MCDI_MODE_EVENTS: wait for an mcdi response callback.
// @MCDI_MODE_FAIL: we think MCDI is dead, so fail-fast all calls
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdx_mcdi_mode {
    MCDI_MODE_EVENTS,
    MCDI_MODE_FAIL,
}

//
// enum cdx_mcdi_cmd_state - State for an individual MCDI command
// @MCDI_STATE_QUEUED: Command not started and is waiting to run.
// @MCDI_STATE_RETRY: Command was submitted and MC rejected with no resources,
// as MC have too many outstanding commands. Command will be retried once
// another command returns.
// @MCDI_STATE_RUNNING: Command was accepted and is running.
// @MCDI_STATE_RUNNING_CANCELLED: Command is running but the issuer cancelled
// the command.
// @MCDI_STATE_FINISHED: Processing of this command has completed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdx_mcdi_cmd_state {
    MCDI_STATE_QUEUED,
    MCDI_STATE_RETRY,
    MCDI_STATE_RUNNING,
    MCDI_STATE_RUNNING_CANCELLED,
    MCDI_STATE_FINISHED,
}

//
// struct cdx_mcdi - CDX MCDI Firmware interface, to interact
// with CDX controller.
// @mcdi: MCDI interface
// @mcdi_ops: MCDI operations
// @r5_rproc : R5 Remoteproc device handle
// @rpdev: RPMsg device
// @ept: RPMsg endpoint
// @work: Post probe work
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_mcdi {
// MCDI interface
    pub mcdi: *mut cdx_mcdi_data,
    pub mcdi_ops: *const cdx_mcdi_ops,
    pub r5_rproc: *mut rproc,
    pub rpdev: *mut rpmsg_device,
    pub ept: *mut rpmsg_endpoint,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_mcdi_ops {
    pub sdu_len): *const *const cdx_dword sdu, size_t,
    pub cmd): *mut *mut *mut unsigned int (mcdi_rpc_timeout)(struct cdx_mcdi cdx, unsigned int,
}

//
// struct cdx_mcdi_cmd - An outstanding MCDI command
// @ref: Reference count. There will be one reference if the command is
// in the mcdi_iface cmd_list, another if it's on a cleanup list,
// and a third if it's queued in the work queue.
// @list: The data for this entry in mcdi->cmd_list
// @cleanup_list: The data for this entry in a cleanup list
// @work: The work item for this command, queued in mcdi->workqueue
// @mcdi: The mcdi_iface for this command
// @state: The state of this command
// @inlen: inbuf length
// @inbuf: Input buffer
// @quiet: Whether to silence errors
// @reboot_seen: Whether a reboot has been seen during this command,
// to prevent duplicates
// @seq: Sequence number
// @started: Jiffies this command was started at
// @cookie: Context for completion function
// @completer: Completion function
// @handle: Command handle
// @cmd: Command number
// @rc: Return code
// @outlen: Length of output buffer
// @outbuf: Output buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_mcdi_cmd {
    pub ref: kref,
    pub list: list_head,
    pub cleanup_list: list_head,
    pub work: work_struct,
    pub mcdi: *mut cdx_mcdi_iface,
    pub state: cdx_mcdi_cmd_state,
    pub inlen: usize,
    pub inbuf: *const cdx_dword,
    pub quiet: bool,
    pub reboot_seen: bool,
    pub seq: u8,
    pub started: c_ulong,
    pub cookie: c_ulong,
    pub completer: *mut cdx_mcdi_async_completer,
    pub handle: c_uint,
    pub cmd: c_uint,
    pub rc: c_int,
    pub outlen: usize,
    pub outbuf: *mut cdx_dword,
// followed by inbuf data if necessary
}

//
// struct cdx_mcdi_iface - MCDI protocol context
// @cdx: The associated NIC
// @iface_lock: Serialise access to this structure
// @outstanding_cleanups: Count of cleanups
// @cmd_list: List of outstanding and running commands
// @workqueue: Workqueue used for delayed processing
// @cmd_complete_wq: Waitqueue for command completion
// @db_held_by: Command the MC doorbell is in use by
// @seq_held_by: Command each sequence number is in use by
// @prev_handle: The last used command handle
// @mode: Poll for mcdi completion, or wait for an mcdi_event
// @prev_seq: The last used sequence number
// @new_epoch: Indicates start of day or start of MC reboot recovery
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_mcdi_iface {
    pub cdx: *mut cdx_mcdi,
// Serialise access
    pub iface_lock: mutex,
    pub outstanding_cleanups: c_uint,
    pub cmd_list: list_head,
    pub workqueue: *mut workqueue_struct,
    pub cmd_complete_wq: wait_queue_head_t,
    pub db_held_by: *mut cdx_mcdi_cmd,
    pub seq_held_by: [*mut cdx_mcdi_cmd; 16],
    pub prev_handle: c_uint,
    pub mode: cdx_mcdi_mode,
    pub prev_seq: u8,
    pub new_epoch: bool,
}

//
// struct cdx_mcdi_data - extra state for NICs that implement MCDI
// @iface: Interface/protocol state
// @fn_flags: Flags for this function, as returned by %MC_CMD_DRV_ATTACH.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_mcdi_data {
    pub iface: cdx_mcdi_iface,
    pub fn_flags: u32,
}

extern "C" {
    pub fn cdx_mcdi_finish(cdx: *mut cdx_mcdi);
}
extern "C" {
    pub fn cdx_mcdi_init(cdx: *mut cdx_mcdi) -> c_int;
}
extern "C" {
    pub fn cdx_mcdi_process_cmd(cdx: *mut cdx_mcdi, outbuf: *mut cdx_dword, len: c_int);
}
//
// We expect that 16- and 32-bit fields in MCDI requests and responses
// are appropriately aligned, but 64-bit fields are only
// 32-bit-aligned.
//

