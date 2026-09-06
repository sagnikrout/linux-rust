//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/ext/ext.h
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
// BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
//
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2022 Tejun Heo <tj@kernel.org>
// Copyright (c) 2022 David Vernet <dvernet@meta.com>
//

extern "C" {
    pub fn scx_tick(rq: *mut rq);
}
extern "C" {
    pub fn init_scx_entity(scx: *mut sched_ext_entity);
}
extern "C" {
    pub fn scx_pre_fork(p: *mut task_struct);
}
extern "C" {
    pub fn scx_fork(p: *mut task_struct, kargs: *mut kernel_clone_args) -> c_int;
}
extern "C" {
    pub fn scx_post_fork(p: *mut task_struct);
}
extern "C" {
    pub fn scx_cancel_fork(p: *mut task_struct);
}
extern "C" {
    pub fn scx_can_stop_tick(rq: *mut rq) -> bool;
}
extern "C" {
    pub fn scx_rq_activate(rq: *mut rq);
}
extern "C" {
    pub fn scx_rq_deactivate(rq: *mut rq);
}
extern "C" {
    pub fn scx_check_setscheduler(p: *mut task_struct, policy: c_int) -> c_int;
}
extern "C" {
    pub fn task_should_scx(policy: c_int) -> bool;
}
extern "C" {
    pub fn scx_allow_ttwu_queue(p: *const task_struct) -> bool;
}
extern "C" {
    pub fn init_sched_ext_class();
}

extern "C" {
    pub fn __scx_update_idle(rq: *mut rq, idle: bool, do_notify: bool);
}

extern "C" {
    pub fn scx_tg_init(tg: *mut task_group);
}
extern "C" {
    pub fn scx_tg_online(tg: *mut task_group) -> c_int;
}
extern "C" {
    pub fn scx_tg_offline(tg: *mut task_group);
}
extern "C" {
    pub fn scx_cgroup_can_attach(tset: *mut cgroup_taskset) -> c_int;
}
extern "C" {
    pub fn scx_cgroup_move_task(p: *mut task_struct);
}
extern "C" {
    pub fn scx_cgroup_cancel_attach(tset: *mut cgroup_taskset);
}
extern "C" {
    pub fn scx_group_set_weight(tg: *mut task_group, cgrp_weight: c_ulong);
}
extern "C" {
    pub fn scx_group_set_idle(tg: *mut task_group, idle: bool);
}
extern "C" {
    pub fn scx_group_set_bandwidth(tg: *mut task_group, period_us: u64, quota_us: u64, burst_us: u64);
}

