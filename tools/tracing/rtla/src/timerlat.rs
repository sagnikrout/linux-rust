//! Automatically rewritten from C Header to Rust Module
//! Source: tools/tracing/rtla/src/timerlat.h
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
// Define timerlat tracing mode.
//
// There are three tracing modes:
// - tracefs-only, used when BPF is unavailable.
// - BPF-only, used when BPF is available and neither trace saving nor
// auto-analysis are enabled.
// - mixed mode, used when BPF is available and either trace saving or
// auto-analysis is enabled (which rely on sample collection through
// tracefs).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum timerlat_tracing_mode {
    TRACING_MODE_BPF,
    TRACING_MODE_TRACEFS,
    TRACING_MODE_MIXED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timerlat_params {
    pub common: common_params,
    pub timerlat_period_us: c_longlong,
    pub print_stack: c_longlong,
    pub dma_latency: c_int,
    pub no_aa: bool,
    pub dump_tasks: bool,
    pub deepest_idle_state: c_int,
    pub mode: timerlat_tracing_mode,
    pub bpf_action_program: *const c_char,
    pub stack_format: stack_format,
    pub timerlat_align: bool,
    pub timerlat_align_us: c_ulonglong,
}

extern "C" {
    pub fn timerlat_apply_config(tool: *mut osnoise_tool, params: *mut timerlat_params) -> c_int;
}
extern "C" {
    pub fn timerlat_main(argc: c_int, argv[]: *mut c_char) -> c_int;
}
extern "C" {
    pub fn timerlat_enable(tool: *mut osnoise_tool) -> c_int;
}
extern "C" {
    pub fn timerlat_analyze(tool: *mut osnoise_tool, stopped: bool);
}
extern "C" {
    pub fn timerlat_free(tool: *mut osnoise_tool);
}
