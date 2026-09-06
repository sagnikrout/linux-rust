//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/dlfilter.h
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
// dlfilter.h: Interface to perf script --dlfilter shared object
// Copyright (c) 2021, Intel Corporation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlfilter {
    pub file: *mut c_char,
    pub handle: *mut c_void,
    pub data: *mut c_void,
    pub session: *mut perf_session,
    pub ctx_valid: bool,
    pub in_start: bool,
    pub in_stop: bool,
    pub dlargc: c_int,
    pub dlargv: *mut c_char,
    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub evsel: *mut evsel,
    pub machine: *mut machine,
    pub al: *mut addr_location,
    pub addr_al: *mut addr_location,
    pub d_sample: *mut perf_dlfilter_sample,
    pub d_ip_al: *mut perf_dlfilter_al,
    pub d_addr_al: *mut perf_dlfilter_al,
    pub ctx): *mut *mut *mut *mut int (start)(void data, void,
    pub ctx): *mut *mut *mut int (stop)(void data, void,
    pub ctx): *mut c_void,
    pub ctx): *mut c_void,
    pub fns: *mut perf_dlfilter_fns,
}

extern "C" {
    pub fn dlfilter__start(d: *mut dlfilter, session: *mut perf_session) -> c_int;
}
extern "C" {
    pub fn dlfilter__cleanup(d: *mut dlfilter);
}
extern "C" {
    pub fn dlfilter__do_filter_event(_arg: d, _arg: event, _arg: sample, _arg: evsel, _arg: machine, _arg: al, _arg: addr_al, _arg: false) -> return;
}
extern "C" {
    pub fn dlfilter__do_filter_event(_arg: d, _arg: event, _arg: sample, _arg: evsel, _arg: machine, _arg: al, _arg: addr_al, _arg: true) -> return;
}
extern "C" {
    pub fn list_available_dlfilters(opt: *const option, s: *const c_char, unset: c_int) -> c_int;
}
