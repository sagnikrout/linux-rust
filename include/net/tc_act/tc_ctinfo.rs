//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_ctinfo.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ctinfo_params {
    pub rcu: rcu_head,
    pub net: *mut net,
    pub action: c_int,
    pub dscpmask: u32,
    pub dscpstatemask: u32,
    pub cpmarkmask: u32,
    pub zone: u16,
    pub mode: u8,
    pub dscpmaskshift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ctinfo {
    pub common: tc_action,
    pub params: *mut tcf_ctinfo_params __rcu,
    pub stats_dscp_set: core::sync::atomic::AtomicI64,
    pub stats_dscp_error: core::sync::atomic::AtomicI64,
    pub stats_cpmark_set: core::sync::atomic::AtomicI64,
}

