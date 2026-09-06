//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_wrapper.h
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

// Macro flag: #define TC_INDIRECT_SCOPE
// TC Actions

extern "C" {
    pub fn tcf_gact_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_mirred_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_pedit_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_skbedit_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_skbmod_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_police_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_bpf_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_connmark_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_csum_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_ct_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_ctinfo_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_gate_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_mpls_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_nat_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tunnel_key_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_vlan_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_ife_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_simp_act(_arg: skb, _arg: a, _arg: res) -> return;
}

extern "C" {
    pub fn tcf_sample_act(_arg: skb, _arg: a, _arg: res) -> return;
}

// TC Filters

extern "C" {
    pub fn cls_bpf_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn u32_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn fl_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn fw_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn mall_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn basic_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn cls_cgroup_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn flow_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

extern "C" {
    pub fn route4_classify(_arg: skb, _arg: tp, _arg: res) -> return;
}

