//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/prestera/prestera_flow.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2020 Marvell International Ltd. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_flow_block_binding {
    pub list: list_head,
    pub port: *mut prestera_port,
    pub span_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_flow_block {
    pub binding_list: list_head,
    pub sw: *mut prestera_switch,
    pub net: *mut net,
    pub ruleset_zero: *mut prestera_acl_ruleset,
    pub block_cb: *mut flow_block_cb,
    pub template_list: list_head,
    pub prio_min: u32,
    pub prio_max: u32,
    pub bound: bool,
    pub mall: },
    pub rule_count: c_uint,
    pub ingress: bool,
}
