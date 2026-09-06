//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_gact.h
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
pub struct tcf_gact {
    pub common: tc_action,

    pub tcfg_ptype: u16,
    pub tcfg_pval: u16,
    pub tcfg_paction: c_int,
    pub packets: core::sync::atomic::AtomicI32,

}

extern "C" {
    pub fn __is_tcf_gact_act(_arg: a, _arg: TC_ACT_OK, _arg: false) -> return;
}
extern "C" {
    pub fn __is_tcf_gact_act(_arg: a, _arg: TC_ACT_SHOT, _arg: false) -> return;
}
extern "C" {
    pub fn __is_tcf_gact_act(_arg: a, _arg: TC_ACT_TRAP, _arg: false) -> return;
}
extern "C" {
    pub fn __is_tcf_gact_act(_arg: a, _arg: TC_ACT_GOTO_CHAIN, _arg: true) -> return;
}
extern "C" {
    pub fn __is_tcf_gact_act(_arg: a, _arg: TC_ACT_UNSPEC, _arg: false) -> return;
}
extern "C" {
    pub fn __is_tcf_gact_act(_arg: a, _arg: TC_ACT_RECLASSIFY, _arg: false) -> return;
}
extern "C" {
    pub fn __is_tcf_gact_act(_arg: a, _arg: TC_ACT_PIPE, _arg: false) -> return;
}
