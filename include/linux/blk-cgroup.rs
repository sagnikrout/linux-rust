//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk-cgroup.h
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
// Common Block IO controller cgroup interface
//
// Based on ideas and code from CFQ, CFS and BFQ:
// Copyright (C) 2003 Jens Axboe <axboe@kernel.dk>
//
// Copyright (C) 2008 Fabio Checconi <fabio@gandalf.sssup.it>
// Paolo Valente <paolo.valente@unimore.it>
//
// Copyright (C) 2009 Vivek Goyal <vgoyal@redhat.com>
// Nauman Rafique <nauman@google.com>
//

pub const FC_APPID_LEN: c_int = 129;

extern "C" {
    pub fn blkcg_schedule_throttle(disk: *mut gendisk, use_memdelay: bool);
}
extern "C" {
    pub fn blkcg_maybe_throttle_current();
}
extern "C" {
    pub fn __blk_cgroup_congested() -> bool;
}
//
// blk_cgroup_congested - is the current task in a throttled blkcg?
//
// Called from mm hot paths where the answer is almost always false, so keep
// that case to a load and a branch and only walk the hierarchy out of line
// when something in the system really is throttled.
//
// Return: %true if the current task's blkcg or any of its ancestors is
// throttled, %false otherwise.
//
extern "C" {
    pub fn __blk_cgroup_congested() -> return;
}
extern "C" {
    pub fn blkcg_pin_online(blkcg_css: *mut cgroup_subsys_state);
}
extern "C" {
    pub fn blkcg_unpin_online(blkcg_css: *mut cgroup_subsys_state);
}

extern "C" {
    pub fn blkcg_set_fc_appid(app_id: *mut c_char, cgrp_id: u64, app_id_len: usize) -> c_int;
}
