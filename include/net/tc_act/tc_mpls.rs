//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_mpls.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2019 Netronome Systems, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_mpls_params {
    pub tcfm_action: c_int,
    pub tcfm_label: u32,
    pub /: *mut *mut int action; / tcf_action,
    pub tcfm_tc: u8,
    pub tcfm_ttl: u8,
    pub tcfm_bos: u8,
    pub tcfm_proto: __be16,
    pub rcu: rcu_head,
}

pub const ACT_MPLS_TC_NOT_SET: c_uint = 0xff;
pub const ACT_MPLS_BOS_NOT_SET: c_uint = 0xff;
pub const ACT_MPLS_LABEL_NOT_SET: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_mpls {
    pub common: tc_action,
    pub mpls_p: *mut tcf_mpls_params __rcu,
}

