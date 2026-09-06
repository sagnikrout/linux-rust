//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fnic_nvme.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

pub const FNIC_TPORT_CLEANUP_WAIT_COUNT: c_int = 8;

pub const FNIC_LS_REQ_FLAGS_NONE: c_uint = 0x0;
pub const FNIC_LS_REQ_FLAGS_ABORTED: c_uint = 0x1;
pub const FNIC_LS_REQ_FLAGS_DONE: c_uint = 0x2;
pub const FNIC_LS_REQ_ABORT_COMPLETED: c_uint = 0x4;
pub const FNIC_STATUS_LS_REQ_ABORTED: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvfnic_lsreq_state_e {
    FNIC_LS_REQ_CMD_INIT = 0,
    FNIC_LS_REQ_CMD_PENDING,
    FNIC_LS_REQ_CMD_ABTS_PENDING,
    FNIC_LS_REQ_CMD_COMPLETE,
    FNIC_LS_REQ_ABTS_COMPLETE,
    FNIC_LS_REQ_CMD_ABTS_STARTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_nvme_tag {
    pub free_list: list_head,
    pub tag_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfnic_ls_req {
    pub list: list_head,
    pub ls_req: *mut nvmefc_ls_req,
    pub oxid: u16,
    pub ls_req_timer: timer_list,
    pub fnic: *mut fnic,
    pub tport: *mut fnic_tport_s,
    pub state: c_int,
    pub flags: c_uint,
}

extern "C" {
    pub fn nvfnic_nvme_io_done_handler(arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nvfnic_reset_fcpio_tag_pool(iport: *mut fnic_iport_s);
}
extern "C" {
    pub fn nvfnic_add_lport(fnic: *mut fnic) -> c_int;
}
extern "C" {
    pub fn nvfnic_remote_port_delete(rport: *mut nvme_fc_remote_port);
}
extern "C" {
    pub fn nvfnic_local_port_delete(lport: *mut nvme_fc_local_port);
}
extern "C" {
    pub fn nvfnic_dma_unmap_sgl(fnic: *mut fnic, io_req: *mut fnic_io_req);
}
extern "C" {
    pub fn nvfnic_get_sg_count(io_req: *mut fnic_io_req) -> c_int;
}
extern "C" {
    pub fn nvfnic_dump_nvcmd(io_req: *mut fnic_io_req, flags: u8);
}
extern "C" {
    pub fn _cleanup_tport_io(map: *mut sbitmap, tag: c_uint, data: *mut c_void) -> bool;
}
extern "C" {
    pub fn nvfnic_flush_nvme_io_list(fnic: *mut fnic);
}
extern "C" {
    pub fn nvfnic_fcpio_cmpl(io_req: *mut fnic_io_req);
}
extern "C" {
    pub fn nvfnic_queuecommand(io_req: *mut fnic_io_req) -> c_int;
}
extern "C" {
    pub fn nvfnic_delete_lport(iport: *mut fnic_iport_s);
}
extern "C" {
    pub fn nvfnic_cleanup_all_nvme_ios(fnic: *mut fnic);
}
extern "C" {
    pub fn nvfnic_delete_tport_work(work: *mut work_struct);
}
extern "C" {
    pub fn nvfnic_admin_io_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn nvfnic_nvme_zero_devloss_tports(fnic: *mut fnic);
}
extern "C" {
    pub fn nvfnic_ls_req_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn nvfnic_alloc_ls_req_oxid(iport: *mut fnic_iport_s) -> u16;
}
extern "C" {
    pub fn nvfnic_terminate_tport_ios(fnic: *mut fnic, tport: *mut fnic_tport_s);
}
extern "C" {
    pub fn _terminate_tport_ios(map: *mut sbitmap, tag: c_uint, data: *mut c_void) -> bool;
}
extern "C" {
    pub fn _cleanup_all_nvme_io(map: *mut sbitmap, tag: c_uint, data: *mut c_void) -> bool;
}
extern "C" {
    pub fn nvfnic_cleanup_all_nvme_ios(fnic: *mut fnic);
}
extern "C" {
    pub fn nvfnic_cleanup_tport_io(fnic: *mut fnic, tport: *mut fnic_tport_s);
}
extern "C" {
    pub fn nvfnic_nvme_unload(fnic: *mut fnic);
}
extern "C" {
    pub fn nvfnic_get_nvmef_info(fnic: *mut fnic, info: *mut fnic_nvmef_info) -> c_int;
}
extern "C" {
    pub fn nvfnic_exch_reset(iport: *mut fnic_iport_s, tport: *mut fnic_tport_s);
}
extern "C" {
    pub fn nvfnic_nvme_iodone_work(work: *mut work_struct);
}

