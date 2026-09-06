//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/debugfs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (c) 2024, Broadcom. All rights reserved.  The term
// Broadcom refers to Broadcom Limited and/or its subsidiaries.
//
// Description: Debugfs header
//
extern "C" {
    pub fn bnxt_re_debug_add_qpinfo(rdev: *mut bnxt_re_dev, qp: *mut bnxt_re_qp);
}
extern "C" {
    pub fn bnxt_re_debug_rem_qpinfo(rdev: *mut bnxt_re_dev, qp: *mut bnxt_re_qp);
}
extern "C" {
    pub fn bnxt_re_debugfs_add_pdev(rdev: *mut bnxt_re_dev);
}
extern "C" {
    pub fn bnxt_re_debugfs_rem_pdev(rdev: *mut bnxt_re_dev);
}
extern "C" {
    pub fn bnxt_re_register_debugfs();
}
extern "C" {
    pub fn bnxt_re_unregister_debugfs();
}

pub const BNXT_RE_CC_PARAM_GEN0: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_cc_param {
    pub rdev: *mut bnxt_re_dev,
    pub dentry: *mut dentry,
    pub offset: u32,
    pub cc_gen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_dbg_cc_config_params {
    pub gen0_parms: [bnxt_re_cc_param; BNXT_RE_CC_PARAM_GEN0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_cq_coal_param {
    pub rdev: *mut bnxt_re_dev,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_cq_coal_types {
    BNXT_RE_COAL_CQ_BUF_MAXTIME,
    BNXT_RE_COAL_CQ_NORMAL_MAXBUF,
    BNXT_RE_COAL_CQ_DURING_MAXBUF,
    BNXT_RE_COAL_CQ_EN_RING_IDLE_MODE,
    BNXT_RE_COAL_CQ_ENABLE,
    BNXT_RE_COAL_CQ_MAX

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_dbg_cq_coal_params {
    pub params: [bnxt_re_cq_coal_param; BNXT_RE_COAL_CQ_MAX],
}
