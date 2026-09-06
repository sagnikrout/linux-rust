//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fault-inject.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fault_flags {
    FAULT_NOWARN =	1 << 0,
}

//
// For explanation of the elements of this struct, see
// Documentation/fault-injection/fault-injection.rst
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fault_attr {
    pub probability: c_ulong,
    pub interval: c_ulong,
    pub times: core::sync::atomic::AtomicI32,
    pub space: core::sync::atomic::AtomicI32,
    pub verbose: c_ulong,
    pub task_filter: bool,
    pub stacktrace_depth: c_ulong,
    pub require_start: c_ulong,
    pub require_end: c_ulong,
    pub reject_start: c_ulong,
    pub reject_end: c_ulong,
    pub count: c_ulong,
    pub ratelimit_state: ratelimit_state,
    pub dname: *mut dentry,
}

extern "C" {
    pub fn setup_fault_attr(attr: *mut fault_attr, str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn should_fail_ex(attr: *mut fault_attr, size: isize, flags: c_int) -> bool;
}
extern "C" {
    pub fn should_fail(attr: *mut fault_attr, size: isize) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fault_attr {
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fault_config {
    pub attr: fault_attr,
    pub group: config_group,
}

extern "C" {
    pub fn fault_config_init(config: *mut fault_config, name: *const c_char);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fault_config {
}

extern "C" {
    pub fn should_fail_alloc_page(gfp_mask: gfp_t, order: c_uint) -> bool;
}

extern "C" {
    pub fn should_failslab(s: *mut kmem_cache, gfpflags: gfp_t) -> c_int;
}

