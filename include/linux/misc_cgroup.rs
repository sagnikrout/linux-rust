//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/misc_cgroup.h
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
//
// Miscellaneous cgroup controller.
//
// Copyright 2020 Google LLC
// Author: Vipin Sharma <vipinsh@google.com>
//
// enum misc_res_type - Types of misc cgroup entries supported by the host.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum misc_res_type {

// @MISC_CG_RES_SEV: AMD SEV ASIDs resource
    MISC_CG_RES_SEV,
// @MISC_CG_RES_SEV_ES: AMD SEV-ES ASIDs resource
    MISC_CG_RES_SEV_ES,

// @MISC_CG_RES_TDX: Intel TDX HKIDs resource
    MISC_CG_RES_TDX,

// @MISC_CG_RES_TYPES: count of enum misc_res_type constants
    MISC_CG_RES_TYPES
}

//
// struct misc_res: Per cgroup per misc type resource
// @max: Maximum limit on the resource.
// @watermark: Historical maximum usage of the resource.
// @usage: Current usage of the resource.
// @events: Number of times, the resource limit exceeded.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct misc_res {
    pub max: u64,
    pub watermark: core::sync::atomic::AtomicI64,
    pub usage: core::sync::atomic::AtomicI64,
    pub events: core::sync::atomic::AtomicI64,
    pub events_local: core::sync::atomic::AtomicI64,
}

//
// struct misc_cg - Miscellaneous controller's cgroup structure.
// @css: cgroup subsys state object.
// @events_file: Handle for the misc resources events file.
// @res: Array of misc resources usage in the cgroup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct misc_cg {
    pub css: cgroup_subsys_state,
// misc.events
    pub events_file: cgroup_file,
// misc.events.local
    pub events_local_file: cgroup_file,
    pub res: [misc_res; MISC_CG_RES_TYPES],
}

extern "C" {
    pub fn misc_cg_set_capacity(type: misc_res_type, capacity: u64) -> c_int;
}
extern "C" {
    pub fn misc_cg_try_charge(type: misc_res_type, cg: *mut misc_cg, amount: u64) -> c_int;
}
extern "C" {
    pub fn misc_cg_uncharge(type: misc_res_type, cg: *mut misc_cg, amount: u64);
}
//
// css_misc() - Get misc cgroup from the css.
// @css: cgroup subsys state object.
//
// Context: Any context.
// Return:
// * %NULL - If @css is null.
// * struct misc_cg* - misc cgroup pointer of the passed css.
//
// get_current_misc_cg() - Find and get the misc cgroup of the current task.
//
// Returned cgroup has its ref count increased by 1. Caller must call
// put_misc_cg() to return the reference.
//
// Return: Misc cgroup to which the current task belongs to.
//
extern "C" {
    pub fn css_misc(_arg: task_get_css(current, _arg: misc_cgrp_id)) -> return;
}
//
// put_misc_cg() - Put the misc cgroup and reduce its ref count.
// @cg - cgroup to put.
//

