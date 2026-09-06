//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_ct.h
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
pub struct tcf_ct_params {
    pub helper: *mut nf_conntrack_helper,
    pub tmpl: *mut nf_conn,
    pub zone: u16,
    pub action: c_int,
    pub mark: u32,
    pub mark_mask: u32,
    pub sizeof(u32)]: u32 labels[NF_CT_LABELS_MAX_SIZE /,
    pub sizeof(u32)]: u32 labels_mask[NF_CT_LABELS_MAX_SIZE /,
    pub range: nf_nat_range2,
    pub ipv4_range: bool,
    pub put_labels: bool,
    pub ct_action: u16,
    pub rcu: rcu_head,
    pub ct_ft: *mut tcf_ct_flow_table,
    pub nf_ft: *mut nf_flowtable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_ct {
    pub common: tc_action,
    pub params: *mut tcf_ct_params __rcu,
}

