//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/tp_pmu.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

extern "C" {
    pub fn int(state: *mut *mut tp_sys_callback)(void, sys_name: *const c_char) -> typedef;
}
extern "C" {
    pub fn int(state: *mut *mut tp_event_callback)(void, sys_name: *const c_char, evt_name: *const c_char) -> typedef;
}
extern "C" {
    pub fn tp_pmu__id(sys: *const c_char, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn tp_pmu__for_each_tp_event(sys: *const c_char, state: *mut c_void, cb: tp_event_callback) -> c_int;
}
extern "C" {
    pub fn tp_pmu__for_each_tp_sys(state: *mut c_void, cb: tp_sys_callback) -> c_int;
}
extern "C" {
    pub fn perf_pmu__is_tracepoint(pmu: *const perf_pmu) -> bool;
}
extern "C" {
    pub fn tp_pmu__for_each_event(pmu: *mut perf_pmu, state: *mut c_void, cb: pmu_event_callback) -> c_int;
}
extern "C" {
    pub fn tp_pmu__num_events(pmu: *mut perf_pmu) -> usize;
}
extern "C" {
    pub fn tp_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
}
