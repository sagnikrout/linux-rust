//! Automatically rewritten from C Header to Rust Module
//! Source: tools/tracing/rtla/src/trace.h
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
pub struct trace_events {
    pub next: *mut trace_events,
    pub system: *mut c_char,
    pub event: *mut c_char,
    pub filter: *mut c_char,
    pub trigger: *mut c_char,
    pub enabled: c_char,
    pub filter_enabled: c_char,
    pub trigger_enabled: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_instance {
    pub inst: *mut tracefs_instance,
    pub tep: *mut tep_handle,
    pub seq: *mut trace_seq,
    pub missed_events: c_ulonglong,
    pub processed_events: c_ulonglong,
}

extern "C" {
    pub fn trace_instance_init(trace: *mut trace_instance, tool_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn trace_instance_start(trace: *mut trace_instance) -> c_int;
}
extern "C" {
    pub fn trace_instance_stop(trace: *mut trace_instance) -> c_int;
}
extern "C" {
    pub fn trace_instance_destroy(trace: *mut trace_instance);
}
extern "C" {
    pub fn enable_tracer_by_name(inst: *mut tracefs_instance, tracer_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn disable_tracer(inst: *mut tracefs_instance);
}
extern "C" {
    pub fn destroy_instance(inst: *mut tracefs_instance);
}
extern "C" {
    pub fn save_trace_to_file(inst: *mut tracefs_instance, filename: *const c_char) -> c_int;
}
extern "C" {
    pub fn trace_event_add_filter(event: *mut trace_events, filter: *mut c_char);
}
extern "C" {
    pub fn trace_event_add_trigger(event: *mut trace_events, trigger: *mut c_char);
}
extern "C" {
    pub fn trace_set_buffer_size(trace: *mut trace_instance, size: c_int) -> c_int;
}
