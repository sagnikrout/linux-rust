//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_port.h
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

extern "C" {
    pub fn void(dev: *mut *mut bfa_port_stats_cbfn_t) (void, status: bfa_status_t) -> typedef;
}
extern "C" {
    pub fn void(dev: *mut *mut bfa_port_endis_cbfn_t) (void, status: bfa_status_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_s {
    pub dev: *mut c_void,
    pub ioc: *mut bfa_ioc_s,
    pub trcmod: *mut bfa_trc_mod_s,
    pub msgtag: u32,
    pub stats_busy: bfa_boolean_t,
    pub stats_mb: bfa_mbox_cmd_s,
    pub stats_cbfn: bfa_port_stats_cbfn_t,
    pub stats_cbarg: *mut c_void,
    pub stats_status: bfa_status_t,
    pub stats_reset_time: time64_t,
    pub stats: *mut bfa_port_stats_u,
    pub stats_dma: bfa_dma_s,
    pub endis_pending: bfa_boolean_t,
    pub endis_mb: bfa_mbox_cmd_s,
    pub endis_cbfn: bfa_port_endis_cbfn_t,
    pub endis_cbarg: *mut c_void,
    pub endis_status: bfa_status_t,
    pub ioc_notify: bfa_ioc_notify_s,
    pub pbc_disabled: bfa_boolean_t,
    pub dport_enabled: bfa_boolean_t,
    pub port_dma: bfa_mem_dma_s,
}

extern "C" {
    pub fn bfa_port_notify(arg: *mut c_void, event: bfa_ioc_event_e);
}
extern "C" {
    pub fn bfa_port_meminfo() -> u32;
}
//
// CEE declaration
//
extern "C" {
    pub fn void(dev: *mut *mut bfa_cee_get_attr_cbfn_t) (void, status: bfa_status_t) -> typedef;
}
extern "C" {
    pub fn void(dev: *mut *mut bfa_cee_get_stats_cbfn_t) (void, status: bfa_status_t) -> typedef;
}
extern "C" {
    pub fn void(dev: *mut *mut bfa_cee_reset_stats_cbfn_t) (void, status: bfa_status_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_cbfn_s {
    pub get_attr_cbfn: bfa_cee_get_attr_cbfn_t,
    pub get_attr_cbarg: *mut c_void,
    pub get_stats_cbfn: bfa_cee_get_stats_cbfn_t,
    pub get_stats_cbarg: *mut c_void,
    pub reset_stats_cbfn: bfa_cee_reset_stats_cbfn_t,
    pub reset_stats_cbarg: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_s {
    pub dev: *mut c_void,
    pub get_attr_pending: bfa_boolean_t,
    pub get_stats_pending: bfa_boolean_t,
    pub reset_stats_pending: bfa_boolean_t,
    pub get_attr_status: bfa_status_t,
    pub get_stats_status: bfa_status_t,
    pub reset_stats_status: bfa_status_t,
    pub cbfn: bfa_cee_cbfn_s,
    pub ioc_notify: bfa_ioc_notify_s,
    pub trcmod: *mut bfa_trc_mod_s,
    pub attr: *mut bfa_cee_attr_s,
    pub stats: *mut bfa_cee_stats_s,
    pub attr_dma: bfa_dma_s,
    pub stats_dma: bfa_dma_s,
    pub ioc: *mut bfa_ioc_s,
    pub get_cfg_mb: bfa_mbox_cmd_s,
    pub get_stats_mb: bfa_mbox_cmd_s,
    pub reset_stats_mb: bfa_mbox_cmd_s,
    pub cee_dma: bfa_mem_dma_s,
}

extern "C" {
    pub fn bfa_cee_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_cee_mem_claim(cee: *mut bfa_cee_s, dma_kva: *mut u8, dma_pa: u64);
}
