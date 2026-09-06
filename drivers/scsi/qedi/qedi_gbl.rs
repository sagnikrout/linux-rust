//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedi/qedi_gbl.h
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
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//

extern "C" {
    pub fn qedi_alloc_sq(qedi: *mut qedi_ctx, ep: *mut qedi_endpoint) -> c_int;
}
extern "C" {
    pub fn qedi_free_sq(qedi: *mut qedi_ctx, ep: *mut qedi_endpoint);
}
extern "C" {
    pub fn qedi_send_iscsi_tmf(qedi_conn: *mut qedi_conn, mtask: *mut iscsi_task) -> c_int;
}
extern "C" {
    pub fn qedi_iscsi_send_ioreq(task: *mut iscsi_task) -> c_int;
}
extern "C" {
    pub fn qedi_get_task_idx(qedi: *mut qedi_ctx) -> c_int;
}
extern "C" {
    pub fn qedi_clear_task_idx(qedi: *mut qedi_ctx, idx: c_int);
}
extern "C" {
    pub fn qedi_iscsi_unmap_sg_list(cmd: *mut qedi_cmd);
}
extern "C" {
    pub fn qedi_get_task_tid(qedi: *mut qedi_ctx, itt: u32, tid: *mut i16);
}
extern "C" {
    pub fn qedi_mark_device_missing(cls_session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn qedi_mark_device_available(cls_session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn qedi_reset_host_mtu(qedi: *mut qedi_ctx, mtu: u16);
}
extern "C" {
    pub fn qedi_recover_all_conns(qedi: *mut qedi_ctx) -> c_int;
}
extern "C" {
    pub fn qedi_fp_process_cqes(work: *mut qedi_work);
}
extern "C" {
    pub fn qedi_alloc_id(id_tbl: *mut qedi_portid_tbl, id: u16) -> c_int;
}
extern "C" {
    pub fn qedi_alloc_new_id(id_tbl: *mut qedi_portid_tbl) -> u16;
}
extern "C" {
    pub fn qedi_free_id(id_tbl: *mut qedi_portid_tbl, id: u16);
}
