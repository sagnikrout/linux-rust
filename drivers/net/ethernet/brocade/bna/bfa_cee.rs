//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfa_cee.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

extern "C" {
    pub fn void(dev: *mut *mut bfa_cee_get_attr_cbfn_t) (void, status: bfa_status) -> typedef;
}
extern "C" {
    pub fn void(dev: *mut *mut bfa_cee_get_stats_cbfn_t) (void, status: bfa_status) -> typedef;
}
extern "C" {
    pub fn void(dev: *mut *mut bfa_cee_reset_stats_cbfn_t) (void, status: bfa_status) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_cbfn {
    pub get_attr_cbfn: bfa_cee_get_attr_cbfn_t,
    pub get_attr_cbarg: *mut c_void,
    pub get_stats_cbfn: bfa_cee_get_stats_cbfn_t,
    pub get_stats_cbarg: *mut c_void,
    pub reset_stats_cbfn: bfa_cee_reset_stats_cbfn_t,
    pub reset_stats_cbarg: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee {
    pub dev: *mut c_void,
    pub get_attr_pending: bool,
    pub get_stats_pending: bool,
    pub reset_stats_pending: bool,
    pub get_attr_status: bfa_status,
    pub get_stats_status: bfa_status,
    pub reset_stats_status: bfa_status,
    pub cbfn: bfa_cee_cbfn,
    pub ioc_notify: bfa_ioc_notify,
    pub attr: *mut bfa_cee_attr,
    pub stats: *mut bfa_cee_stats,
    pub attr_dma: bfa_dma,
    pub stats_dma: bfa_dma,
    pub ioc: *mut bfa_ioc,
    pub get_cfg_mb: bfa_mbox_cmd,
    pub get_stats_mb: bfa_mbox_cmd,
    pub reset_stats_mb: bfa_mbox_cmd,
}

extern "C" {
    pub fn bfa_nw_cee_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_nw_cee_attach(cee: *mut bfa_cee, ioc: *mut bfa_ioc, dev: *mut c_void);
}
