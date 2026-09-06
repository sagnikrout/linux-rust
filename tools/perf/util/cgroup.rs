//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/cgroup.h
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
pub struct cgroup {
    pub node: rb_node,
    pub id: u64,
    pub name: *mut c_char,
    pub fd: c_int,
    pub refcnt: refcount_t,
}

extern "C" {
    pub fn cgroup__put(cgroup: *mut cgroup);
}
extern "C" {
    pub fn evlist__expand_cgroup(evlist: *mut evlist, cgroups: *const c_char, open_cgroup: bool) -> c_int;
}
extern "C" {
    pub fn evlist__set_default_cgroup(evlist: *mut evlist, cgroup: *mut cgroup);
}
extern "C" {
    pub fn parse_cgroups(opt: *const option, str: *const c_char, unset: c_int) -> c_int;
}
extern "C" {
    pub fn perf_env__purge_cgroups(env: *mut perf_env);
}

extern "C" {
    pub fn read_cgroup_id(cgrp: *mut cgroup) -> c_int;
}

// read all cgroups in the system and save them in the rbtree
extern "C" {
    pub fn read_all_cgroups(root: *mut rb_root);
}
extern "C" {
    pub fn cgroup_is_v2(subsys: *const c_char) -> c_int;
}
