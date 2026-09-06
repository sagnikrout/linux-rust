//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_pedit.h
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
pub struct tcf_pedit_key_ex {
    pub htype: pedit_header_type,
    pub cmd: pedit_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_pedit_parms {
    pub tcfp_keys: *mut tc_pedit_key,
    pub tcfp_keys_ex: *mut tcf_pedit_key_ex,
    pub action: c_int,
    pub tcfp_nkeys: c_uchar,
    pub tcfp_flags: c_uchar,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_pedit {
    pub common: tc_action,
    pub parms: *mut tcf_pedit_parms __rcu,
}

// Must be called with act->tcfa_lock held to ensure consistency of parallel
// reads of the same action's pedit keys (e.g. flow_offload count vs fill).
// Note, this is only used for pedit offload.
//
