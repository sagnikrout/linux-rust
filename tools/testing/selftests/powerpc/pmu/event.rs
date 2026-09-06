//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/pmu/event.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2013, Michael Ellerman, IBM Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub attr: perf_event_attr,
    pub name: *mut c_char,
    pub fd: c_int,
// This must match the read_format we use
    pub value: u64,
    pub running: u64,
    pub enabled: u64,
    pub result: },
//
// mmap buffer used while recording sample.
// Accessed as "struct perf_event_mmap_page"
//
    pub mmap_buffer: *mut c_void,
}

extern "C" {
    pub fn event_init(e: *mut event, config: u64);
}
extern "C" {
    pub fn event_init_named(e: *mut event, config: u64, name: *mut c_char);
}
extern "C" {
    pub fn event_init_opts(e: *mut event, config: u64, type: c_int, name: *mut c_char);
}
extern "C" {
    pub fn event_init_sampling(e: *mut event, config: u64);
}
extern "C" {
    pub fn event_open_with_options(e: *mut event, pid: pid_t, cpu: c_int, group_fd: c_int) -> c_int;
}
extern "C" {
    pub fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
}
extern "C" {
    pub fn event_open_with_pid(e: *mut event, pid: pid_t) -> c_int;
}
extern "C" {
    pub fn event_open_with_cpu(e: *mut event, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn event_open(e: *mut event) -> c_int;
}
extern "C" {
    pub fn event_close(e: *mut event);
}
extern "C" {
    pub fn event_enable(e: *mut event) -> c_int;
}
extern "C" {
    pub fn event_disable(e: *mut event) -> c_int;
}
extern "C" {
    pub fn event_reset(e: *mut event) -> c_int;
}
extern "C" {
    pub fn event_read(e: *mut event) -> c_int;
}
extern "C" {
    pub fn event_report_justified(e: *mut event, name_width: c_int, result_width: c_int);
}
extern "C" {
    pub fn event_report(e: *mut event);
}
