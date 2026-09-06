//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tc_act/tc_gate.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright 2020 NXP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_gate_entry {
    pub gate_state: u8,
    pub interval: u32,
    pub ipv: i32,
    pub maxoctets: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcfg_gate_entry {
    pub index: c_int,
    pub gate_state: u8,
    pub interval: u32,
    pub ipv: i32,
    pub maxoctets: i32,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_gate_params {
    pub tcfg_priority: i32,
    pub tcfg_basetime: u64,
    pub tcfg_cycletime: u64,
    pub tcfg_cycletime_ext: u64,
    pub tcfg_flags: u32,
    pub tcfg_clockid: i32,
    pub num_entries: usize,
    pub entries: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_gate {
    pub common: tc_action,
    pub param: *mut tcf_gate_params __rcu,
    pub current_gate_status: u8,
    pub current_close_time: ktime_t,
    pub current_entry_octets: u32,
    pub current_max_octets: i32,
    pub next_entry: *mut tcfg_gate_entry,
    pub hitimer: hrtimer,
    pub tk_offset: tk_offsets,
}

// tcf_gate_get_list(const struct tc_action *a)
