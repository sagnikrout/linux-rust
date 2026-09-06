//! Automatically rewritten from C Header to Rust Module
//! Source: tools/tracing/rtla/src/osnoise.h
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
pub enum osnoise_mode {
    MODE_OSNOISE = 0,
    MODE_HWNOISE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_params {
    pub common: common_params,
    pub runtime: c_ulonglong,
    pub period: c_ulonglong,
    pub threshold: c_longlong,
    pub mode: osnoise_mode,
}

//
// *_INIT_VALs are also invalid values, they are used to
// communicate errors.
//

extern "C" {
    pub fn osnoise_get_context(context: *mut osnoise_context) -> c_int;
}
extern "C" {
    pub fn osnoise_put_context(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_restore_runtime_period(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_restore_stop_us(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_restore_stop_total_us(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_restore_timerlat_period_us(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_restore_tracing_thresh(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_restore_print_stack(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_restore_timerlat_align_us(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_set_timerlat_align(context: *mut osnoise_context, onoff: bool) -> c_int;
}
extern "C" {
    pub fn osnoise_set_irq_disable(context: *mut osnoise_context, onoff: bool) -> c_int;
}
extern "C" {
    pub fn osnoise_report_missed_events(tool: *mut osnoise_tool);
}
extern "C" {
    pub fn osnoise_apply_config(tool: *mut osnoise_tool, params: *mut osnoise_params) -> c_int;
}
extern "C" {
    pub fn osnoise_enable(tool: *mut osnoise_tool) -> c_int;
}
extern "C" {
    pub fn osnoise_main(argc: c_int, argv: *mut c_char) -> c_int;
}
extern "C" {
    pub fn hwnoise_main(argc: c_int, argv: *mut c_char) -> c_int;
}
extern "C" {
    pub fn run_tool(ops: *mut tool_ops, argc: c_int, argv[]: *mut c_char) -> c_int;
}
