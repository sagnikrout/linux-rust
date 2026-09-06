//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/snic_disc.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_disc_state {
    SNIC_DISC_NONE,
    SNIC_DISC_INIT,
    SNIC_DISC_PENDING,
    SNIC_DISC_DONE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_disc {
    pub tgt_list: list_head,
    pub state: snic_disc_state,
    pub mutex: mutex,
    pub disc_id: u16,
    pub req_cnt: u8,
    pub nxt_tgt_id: u32,
    pub rtgt_cnt: u32,
    pub rtgt_info: *mut u8,
    pub disc_timeout: delayed_work,
    pub ): *mut *mut void (cb)(struct snic,
}

pub const SNIC_TGT_NAM_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_tgt_state {
    SNIC_TGT_STAT_NONE,
    SNIC_TGT_STAT_INIT,
    SNIC_TGT_STAT_ONLINE,	/* Target is Online */
    SNIC_TGT_STAT_OFFLINE,	/* Target is Offline */
    SNIC_TGT_STAT_DEL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_tgt_priv {
    pub list: list_head,
    pub typ: snic_tgt_type,
    pub disc_id: u16,
    pub name: [*mut c_char; SNIC_TGT_NAM_LEN],
// DAS Target specific info
// SAN Target specific info
    pub dummmy: u8,
    pub u: },
}

// snic tgt flags
pub const SNIC_TGT_SCAN_PENDING: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_tgt {
    pub list: list_head,
    pub id: u16,
    pub channel: u16,
    pub flags: u32,
    pub scsi_tgt_id: u32,
    pub state: snic_tgt_state,
    pub dev: device,
    pub scan_work: work_struct,
    pub del_work: work_struct,
    pub tdata: snic_tgt_priv,
}

extern "C" {
    pub fn snic_disc_init(: *mut snic_disc);
}
extern "C" {
    pub fn snic_disc_start(: *mut snic) -> c_int;
}
extern "C" {
    pub fn snic_disc_term(: *mut snic);
}
extern "C" {
    pub fn snic_report_tgt_cmpl_handler(: *mut snic, : *mut snic_fw_req) -> c_int;
}
extern "C" {
    pub fn snic_tgtinfo_cmpl_handler(snic: *mut snic, fwreq: *mut snic_fw_req) -> c_int;
}
extern "C" {
    pub fn snic_process_report_tgts_rsp(: *mut work_struct);
}
extern "C" {
    pub fn snic_handle_tgt_disc(: *mut work_struct);
}
extern "C" {
    pub fn snic_handle_disc(: *mut work_struct);
}
extern "C" {
    pub fn snic_tgt_dev_release(: *mut device);
}
extern "C" {
    pub fn snic_tgt_del_all(: *mut snic);
}

extern "C" {
    pub fn snic_tgt_scsi_abort_io(: *mut snic_tgt) -> c_int;
}
