//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ibmvscsi/ibmvscsi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// ------------------------------------------------------------
// ibmvscsi.h
// (C) Copyright IBM Corporation 1994, 2003
// Authors: Colin DeVilbiss (devilbis@us.ibm.com)
// Santiago Leon (santil@us.ibm.com)
// Dave Boutcher (sleddog@us.ibm.com)
//
// ------------------------------------------------------------
// Emulation of a SCSI host adapter for Virtual I/O devices
//
// This driver allows the Linux SCSI peripheral drivers to directly
// access devices in the hosting partition, either on an iSeries
// hypervisor system or a converged hypervisor system.
//

// Number of indirect bufs...the list of these has to fit in the
// additional data of the srp_cmd struct along with the indirect
// descriptor
//
pub const MAX_INDIRECT_BUFS: c_int = 10;
pub const IBMVSCSI_MAX_REQUESTS_DEFAULT: c_int = 100;
pub const IBMVSCSI_CMDS_PER_LUN_DEFAULT: c_int = 16;

pub const IBMVSCSI_MAX_CMDS_PER_LUN: c_int = 64;
pub const IBMVSCSI_MAX_LUN: c_int = 32;
// ------------------------------------------------------------
// Data Structures
//
// an RPA command/response transport queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crq_queue {
    pub msgs: *mut viosrp_crq,
    pub cur: int size,,
    pub msg_token: dma_addr_t,
    pub lock: spinlock_t,
}

// a unit of work for the hosting partition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_event_struct {
    pub xfer_iu: *mut viosrp_iu,
    pub cmnd: *mut scsi_cmnd,
    pub list: list_head,
    pub ): *mut *mut void (done) (struct srp_event_struct,
    pub crq: viosrp_crq,
    pub hostdata: *mut ibmvscsi_host_data,
    pub free: core::sync::atomic::AtomicI32,
    pub iu: viosrp_iu,
    pub ): *mut *mut void (cmnd_done) (struct scsi_cmnd,
    pub comp: completion,
    pub timer: timer_list,
    pub sync_srp: *mut viosrp_iu,
    pub ext_list: *mut srp_direct_buf,
    pub ext_list_token: dma_addr_t,
}

// a pool of event structs for use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_pool {
    pub events: *mut srp_event_struct,
    pub size: u32,
    pub next: c_int,
    pub iu_storage: *mut viosrp_iu,
    pub iu_token: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvscsi_host_action {
    IBMVSCSI_HOST_ACTION_NONE = 0,
    IBMVSCSI_HOST_ACTION_RESET,
    IBMVSCSI_HOST_ACTION_REENABLE,
    IBMVSCSI_HOST_ACTION_UNBLOCK,
}

// all driver data associated with a host adapter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvscsi_host_data {
    pub host_list: list_head,
    pub request_limit: core::sync::atomic::AtomicI32,
    pub client_migrated: c_int,
    pub action: ibmvscsi_host_action,
    pub dev: *mut device,
    pub pool: event_pool,
    pub queue: crq_queue,
    pub srp_task: tasklet_struct,
    pub sent: list_head,
    pub host: *mut Scsi_Host,
    pub work_thread: *mut task_struct,
    pub work_wait_q: wait_queue_head_t,
    pub madapter_info: mad_adapter_info_data,
    pub caps: capabilities,
    pub caps_addr: dma_addr_t,
    pub adapter_info_addr: dma_addr_t,
}
