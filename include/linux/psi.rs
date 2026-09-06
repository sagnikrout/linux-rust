//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/psi.h
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

extern "C" {
    pub fn psi_init();
}
extern "C" {
    pub fn psi_memstall_enter(flags: *mut c_ulong);
}
extern "C" {
    pub fn psi_memstall_leave(flags: *mut c_ulong);
}
extern "C" {
    pub fn psi_show(s: *mut seq_file, group: *mut psi_group, res: psi_res) -> c_int;
}
extern "C" {
    pub fn psi_trigger_create_rtpoll_worker(group: *mut psi_group) -> c_int;
}
extern "C" {
    pub fn psi_trigger_destroy(t: *mut psi_trigger);
}

extern "C" {
    pub fn psi_cgroup_alloc(cgrp: *mut cgroup) -> c_int;
}
extern "C" {
    pub fn psi_cgroup_free(cgrp: *mut cgroup);
}
extern "C" {
    pub fn cgroup_move_task(p: *mut task_struct, to: *mut css_set);
}
extern "C" {
    pub fn psi_cgroup_restart(group: *mut psi_group);
}

