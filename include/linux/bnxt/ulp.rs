//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bnxt/ulp.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2016-2018 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

pub const BNXT_MIN_ROCE_CP_RINGS: c_int = 2;
pub const BNXT_MIN_ROCE_STAT_CTXS: c_int = 1;
pub const BNXT_MAX_ROCE_MSIX_VF: c_int = 2;
pub const BNXT_MAX_ROCE_MSIX_NPAR_PF: c_int = 5;
pub const BNXT_MAX_ROCE_MSIX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_auxdev_type {
    BNXT_AUXDEV_RDMA = 0,
    BNXT_AUXDEV_FWCTL,
    __BNXT_AUXDEV_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_aux_priv {
    pub aux_dev: auxiliary_device,
    pub edev: *mut bnxt_en_dev,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_msix_entry {
    pub vector: u32,
    pub ring_idx: u32,
    pub db_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ulp_ops {
// async_notifier() cannot sleep (in BH context)
    pub ): *mut *mut *mut void (ulp_async_notifier)(void , struct hwrm_async_event_cmpl,
    pub bool): *mut *mut *mut void (ulp_irq_stop)(void ,,
    pub ): *mut *mut *mut void (ulp_irq_restart)(void , struct bnxt_msix_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_fw_msg {
    pub msg: *mut c_void,
    pub msg_len: c_int,
    pub resp: *mut c_void,
    pub resp_max_len: c_int,
    pub timeout: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ulp {
    pub handle: *mut c_void,
    pub ulp_ops: *mut bnxt_ulp_ops __rcu,
    pub async_events_bmap: *mut c_ulong,
    pub max_async_event_id: u16,
    pub msix_requested: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_en_dev {
    pub net: *mut net_device,
    pub pdev: *mut pci_dev,
    pub msix_entries: [bnxt_msix_entry; BNXT_MAX_ROCE_MSIX],
    pub flags: u32,
pub const BNXT_EN_FLAG_ROCEV1_CAP: c_uint = 0x1;
pub const BNXT_EN_FLAG_ROCEV2_CAP: c_uint = 0x2;

pub const BNXT_EN_FLAG_ULP_STOPPED: c_uint = 0x8;
pub const BNXT_EN_FLAG_VF: c_uint = 0x10;

pub const BNXT_EN_FLAG_ROCE_VF_RES_MGMT: c_uint = 0x20;
pub const BNXT_EN_FLAG_SW_RES_LMT: c_uint = 0x40;

    pub ulp_tbl: *mut bnxt_ulp,
    pub in: *mut *mut int l2_db_size; / Doorbell BAR size,
// bytes mapped by L2
// driver.
//
    pub in: *mut *mut int l2_db_size_nc; / Doorbell BAR size,
// bytes mapped as non-
// cacheable.
//
    pub in: *mut *mut int l2_db_offset; / Doorbell offset,
// bytes within
// l2_db_size_nc.
//
    pub chip_num: u16,
    pub hw_ring_stats_size: u16,
    pub pf_port_id: u16,
    pub in: *mut *mut unsigned long en_state; / Could be checked,
// RoCE driver suspend
// mode only. Will be
// updated in resume.
//
    pub bar0: *mut void __iomem,
    pub ulp_num_msix_vec: u16,
    pub ulp_num_ctxs: u16,
// serialize ulp operations
    pub en_dev_lock: mutex,
}

extern "C" {
    pub fn bnxt_get_ulp_msix_num(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_get_ulp_msix_num_in_use(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_set_ulp_msix_num(bp: *mut bnxt, num: c_int);
}
extern "C" {
    pub fn bnxt_get_ulp_stat_ctxs(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_set_ulp_stat_ctxs(bp: *mut bnxt, num_ctxs: c_int);
}
extern "C" {
    pub fn bnxt_get_ulp_stat_ctxs_in_use(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_set_dflt_ulp_stat_ctxs(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_ulp_stop(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_ulp_start(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_ulp_sriov_cfg(bp: *mut bnxt, num_vfs: c_int);
}
extern "C" {
    pub fn bnxt_ulp_irq_stop(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_ulp_irq_restart(bp: *mut bnxt, err: c_int);
}
extern "C" {
    pub fn bnxt_ulp_async_events(bp: *mut bnxt, cmpl: *mut hwrm_async_event_cmpl);
}
extern "C" {
    pub fn bnxt_aux_devices_uninit(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_aux_devices_del(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_aux_devices_add(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_aux_devices_init(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_unregister_dev(edev: *mut bnxt_en_dev);
}
extern "C" {
    pub fn bnxt_send_msg(edev: *mut bnxt_en_dev, fw_msg: *mut bnxt_fw_msg) -> c_int;
}
extern "C" {
    pub fn bnxt_auxdev_id_alloc(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_auxdev_id_free(bp: *mut bnxt, id: c_int);
}
