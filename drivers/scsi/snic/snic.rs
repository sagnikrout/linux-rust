//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/snic.h
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.

pub const DESC_CLEAN_LOW_WATERMARK: c_int = 8;

//
// Tag bits used for special requests.
//

//
// Command flags to identify the type of command and for other future use
//
pub const SNIC_NO_FLAGS: c_int = 0;

//
// These are protected by the hashed req_lock.
//

pub const SNIC_INVALID_CODE: c_uint = 0x100	/* Hdr Status val unused by firmware */;
pub const SNIC_MAX_TARGET: c_int = 256;

// snic module params
// snic debugging
pub const SNIC_MAIN_LOGGING: c_uint = 0x1;
pub const SNIC_SCSI_LOGGING: c_uint = 0x2;
pub const SNIC_ISR_LOGGING: c_uint = 0x8;
pub const SNIC_DESC_LOGGING: c_uint = 0x10;

// Soft assert

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_intx_intr_index {
    SNIC_INTX_WQ_RQ_COPYWQ,
    SNIC_INTX_ERR,
    SNIC_INTX_NOTIFY,
    SNIC_INTX_INTR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_msix_intr_index {
    SNIC_MSIX_WQ,
    SNIC_MSIX_IO_CMPL,
    SNIC_MSIX_ERR_NOTIFY,
    SNIC_MSIX_INTR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_msix_entry {
    pub requested: c_int,
    pub devname: [c_char; SNIC_INTRHDLR_NAMSZ],
    pub ): *mut *mut irqreturn_t (isr)(int, void,
    pub devid: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_state {
    SNIC_INIT = 0,
    SNIC_ERROR,
    SNIC_ONLINE,
    SNIC_OFFLINE,
    SNIC_FWRESET,
}

pub const SNIC_WQ_MAX: c_int = 1;
pub const SNIC_CQ_IO_CMPL_MAX: c_int = 1;

// firmware version information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_fw_info {
    pub fw_ver: u32,
    pub /: *mut *mut u32 hid; / u16 hid | u16 vnic id,
    pub /: *mut *mut u32 max_concur_ios; / max concurrent ios,
    pub /: *mut *mut u32 max_sgs_per_cmd; / max sgls per IO,
    pub /: *mut *mut u32 max_io_sz; / max io size supported,
    pub /: *mut *mut u32 hba_cap; / hba capabilities,
    pub /: *mut *mut u32 max_tgts; / max tgts supported,
    pub /: *mut *mut u16 io_tmo; / FW Extended timeout,
    pub lock*/: *mut *mut *mut completion wait; / protected by snic,
}

//
// snic_work item : defined to process asynchronous events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_work {
    pub work: work_struct,
    pub ev_id: u16,
    pub ev_data: *mut u64,
}

//
// snic structure to represent SCSI vNIC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic {
// snic specific members
    pub list: list_head,
    pub name: [c_char; IFNAMSIZ],
    pub state: core::sync::atomic::AtomicI32,
    pub snic_lock: spinlock_t,
    pub remove_wait: *mut completion,
    pub in_remove: bool,
    pub /: *mut *mut bool stop_link_events; / stop processing link events,
// discovery related
    pub disc: snic_disc,
// Scsi Host info
    pub shost: *mut Scsi_Host,
// vnic related structures
    pub bar0: vnic_dev_bar,
    pub stats: *mut vnic_stats,
    pub stats_time: c_ulong,
    pub stats_reset_time: c_ulong,
    pub vdev: *mut vnic_dev,
// hw resource info
    pub wq_count: c_uint,
    pub cq_count: c_uint,
    pub intr_count: c_uint,
    pub err_intr_offset: c_uint,
    pub /: *mut *mut int link_status; / retrieved from svnic_dev_link_status(),
    pub link_down_cnt: u32,
// pci related
    pub pdev: *mut pci_dev,
    pub msix: [snic_msix_entry; SNIC_MSIX_INTR_MAX],
// io related info
    pub /: *mut *mut *mut mempool_t req_pool[SNIC_REQ_MAX_CACHES]; / (??),
    pub io_req_lock: [____cacheline_aligned spinlock_t; SNIC_IO_LOCKS],
// Maintain snic specific commands, cmds with no tag in spl_cmd_list
    pub spl_cmd_lock: ____cacheline_aligned spinlock_t,
    pub spl_cmd_list: list_head,
    pub max_tag_id: c_uint,
    pub /: *mut *mut atomic_t ios_inflight; / io in flight counter,
    pub config: vnic_snic_config,
    pub link_work: work_struct,
// firmware information
    pub fwinfo: snic_fw_info,
// Work for processing Target related work
    pub tgt_work: work_struct,
// Work for processing Discovery
    pub disc_work: work_struct,
// stats related
    pub reset_stats: c_uint,
    pub io_cmpl_skip: core::sync::atomic::AtomicI64,
    pub /: *mut *mut snic_stats s_stats; / Per SNIC driver stats,
// platform specific

    pub /: *mut *mut *mut dentry stats_host; / Per snic debugfs root,
    pub /: *mut *mut *mut dentry stats_file; / Per snic debugfs file,
    pub /: *mut *mut *mut dentry reset_stats_file;/ Per snic reset stats file,

// completion queue cache line section
    pub cq: [____cacheline_aligned struct vnic_cq; SNIC_CQ_MAX],
// work queue cache line section
    pub wq: [____cacheline_aligned struct vnic_wq; SNIC_WQ_MAX],
    pub wq_lock: [spinlock_t; SNIC_WQ_MAX],
// interrupt resource cache line section
    pub intr: [____cacheline_aligned struct vnic_intr; SNIC_MSIX_INTR_MAX],
}

//
// SNIC Driver's Global Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_global {
    pub snic_list: list_head,
    pub snic_list_lock: spinlock_t,
    pub req_cache: [*mut kmem_cache; SNIC_REQ_MAX_CACHES],
    pub event_q: *mut workqueue_struct,

// debugfs related global data
    pub trc_root: *mut dentry,
    pub stats_root: *mut dentry,
    pub ____cacheline_aligned: snic_trc trc,

}

extern "C" {
    pub fn snic_glob_init() -> c_int;
}
extern "C" {
    pub fn snic_glob_cleanup();
}
extern "C" {
    pub fn snic_abort_cmd(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn snic_device_reset(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn snic_host_reset(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn snic_reset(: *mut Scsi_Host, : *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn snic_shutdown_scsi_cleanup(: *mut snic);
}
extern "C" {
    pub fn snic_request_intr(: *mut snic) -> c_int;
}
extern "C" {
    pub fn snic_free_intr(: *mut snic);
}
extern "C" {
    pub fn snic_set_intr_mode(: *mut snic) -> c_int;
}
extern "C" {
    pub fn snic_clear_intr_mode(: *mut snic);
}
extern "C" {
    pub fn snic_fwcq_cmpl_handler(: *mut snic, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn snic_wq_cmpl_handler(: *mut snic, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn snic_free_wq_buf(: *mut vnic_wq, : *mut vnic_wq_buf);
}
extern "C" {
    pub fn snic_log_q_error(: *mut snic);
}
extern "C" {
    pub fn snic_handle_link_event(: *mut snic);
}
extern "C" {
    pub fn snic_handle_link(: *mut work_struct);
}
extern "C" {
    pub fn snic_queue_exch_ver_req(: *mut snic) -> c_int;
}
extern "C" {
    pub fn snic_io_exch_ver_cmpl_handler(: *mut snic, : *mut snic_fw_req);
}
extern "C" {
    pub fn snic_queue_wq_desc(: *mut snic, os_buf: *mut c_void, len: u16) -> c_int;
}
extern "C" {
    pub fn snic_handle_untagged_req(: *mut snic, : *mut snic_req_info);
}
extern "C" {
    pub fn snic_release_untagged_req(: *mut snic, : *mut snic_req_info);
}
extern "C" {
    pub fn snic_free_all_untagged_reqs(: *mut snic);
}
extern "C" {
    pub fn snic_get_conf(: *mut snic) -> c_int;
}
extern "C" {
    pub fn snic_set_state(: *mut snic, snic_state: enum);
}
extern "C" {
    pub fn snic_get_state(: *mut snic) -> c_int;
}
extern "C" {
    pub fn snic_hex_dump(: *mut c_char, : *mut c_char, _arg: c_int);
}
extern "C" {
    pub fn snic_print_desc(fn: *const c_char, os_buf: *mut c_char, len: c_int);
}
