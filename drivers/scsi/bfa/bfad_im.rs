//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfad_im.h
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

pub const KOBJ_NAME_LEN: c_int = 20;

extern "C" {
    pub fn bfad_im_module_init() -> bfa_status_t;
}
extern "C" {
    pub fn bfad_im_module_exit();
}
extern "C" {
    pub fn bfad_im_probe(bfad: *mut bfad_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfad_im_probe_undo(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_im_port_new(bfad: *mut bfad_s, port: *mut bfad_port_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfad_im_port_delete(bfad: *mut bfad_s, port: *mut bfad_port_s);
}
extern "C" {
    pub fn bfad_im_port_clean(im_port: *mut bfad_im_port_s);
}
extern "C" {
    pub fn bfad_im_supported_speeds(bfa: *mut bfa_s) -> u32;
}
pub const MAX_FCP_TARGET: c_int = 1024;
pub const MAX_FCP_LUN: c_int = 16384;
pub const BFAD_TARGET_RESET_TMO: c_int = 60;
pub const BFAD_LUN_RESET_TMO: c_int = 60;
pub const BFA_QUEUE_FULL_RAMP_UP_TIME: c_int = 120;
//
// itnim flags
//
pub const IO_DONE_BIT: c_int = 0;
//
// struct bfad_cmd_priv - private data per SCSI command.
// @status: Lowest bit represents IO_DONE. The next seven bits hold a value of
// type enum bfi_tskim_status.
// @wq: Wait queue used to wait for completion of an operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_cmd_priv {
    pub status: c_ulong,
    pub wq: *mut wait_queue_head_t,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_itnim_data_s {
    pub itnim: *mut bfad_itnim_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_im_port_s {
    pub bfad: *mut bfad_s,
    pub port: *mut bfad_port_s,
    pub port_delete_work: work_struct,
    pub idr_id: c_int,
    pub cur_scsi_id: u16,
    pub flags: u16,
    pub binding_list: list_head,
    pub shost: *mut Scsi_Host,
    pub itnim_mapped_list: list_head,
    pub fc_vport: *mut fc_vport,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_im_port_pointer {
    pub p: *mut bfad_im_port_s,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfad_itnim_state {
    ITNIM_STATE_NONE,
    ITNIM_STATE_ONLINE,
    ITNIM_STATE_OFFLINE_PENDING,
    ITNIM_STATE_OFFLINE,
    ITNIM_STATE_TIMEOUT,
    ITNIM_STATE_FREE,
}

//
// Per itnim data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_itnim_s {
    pub list_entry: list_head,
    pub fcs_itnim: bfa_fcs_itnim_s,
    pub itnim_work: work_struct,
    pub flags: u32,
    pub state: bfad_itnim_state,
    pub im: *mut bfad_im_s,
    pub im_port: *mut bfad_im_port_s,
    pub drv_rport: *mut bfad_rport_s,
    pub fc_rport: *mut fc_rport,
    pub bfa_itnim: *mut bfa_itnim_s,
    pub scsi_tgt_id: u16,
    pub channel: u16,
    pub queue_work: u16,
    pub last_ramp_up_time: c_ulong,
    pub last_queue_full_time: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfad_binding_type {
    FCP_PWWN_BINDING = 0x1,
    FCP_NWWN_BINDING = 0x2,
    FCP_FCID_BINDING = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_fcp_binding {
    pub list_entry: list_head,
    pub binding_type: bfad_binding_type,
    pub scsi_target_id: u16,
    pub fc_id: u32,
    pub nwwn: wwn_t,
    pub pwwn: wwn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_im_s {
    pub bfad: *mut bfad_s,
    pub drv_workq: *mut workqueue_struct,
    pub aen_im_notify_work: work_struct,
}

// post fc_host vendor event
//
// 'unsigned long aen_tv_sec' overflows in y2106 on 32-bit
// architectures, or in 2038 if user space interprets it
// as 'signed'.
//
extern "C" {
    pub fn bfad_thread_workq(bfad: *mut bfad_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfad_destroy_workq(im: *mut bfad_im_s);
}
extern "C" {
    pub fn bfad_fc_host_init(im_port: *mut bfad_im_port_s);
}
extern "C" {
    pub fn bfad_handle_qfull(itnim: *mut bfad_itnim_s, sdev: *mut scsi_device);
}
extern "C" {
    pub fn bfad_intx(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn bfad_im_bsg_request(job: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn bfad_im_bsg_timeout(job: *mut bsg_job) -> c_int;
}
