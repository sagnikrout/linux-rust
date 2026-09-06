//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/trace-event.h
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
pub struct trace_event {
    pub pevent: *mut tep_handle,
    pub plugin_list: *mut tep_plugin_list,
}

// Computes a version number comparable with LIBTRACEEVENT_VERSION from Makefile.config.

extern "C" {
    pub fn have_tracepoints(evlist: *mut list_head) -> bool;
}
extern "C" {
    pub fn trace_event__init(t: *mut trace_event) -> c_int;
}
extern "C" {
    pub fn trace_event__cleanup(t: *mut trace_event);
}
extern "C" {
    pub fn parse_ftrace_file(pevent: *mut tep_handle, buf: *mut c_char, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn parse_proc_kallsyms(pevent: *mut tep_handle, file: *mut c_char, size: c_uint);
}
extern "C" {
    pub fn parse_ftrace_printk(pevent: *mut tep_handle, file: *mut c_char, size: c_uint);
}
extern "C" {
    pub fn parse_saved_cmdline(pevent: *mut tep_handle, file: *mut c_char, size: c_uint);
}
extern "C" {
    pub fn trace_report(fd: c_int, tevent: *mut trace_event, repipe: bool) -> isize;
}
extern "C" {
    pub fn read_size(event: *mut tep_event, ptr: *mut c_void, size: c_int) -> c_ulonglong;
}
extern "C" {
    pub fn eval_flag(flag: *const c_char) -> c_ulonglong;
}
extern "C" {
    pub fn read_tracing_data(fd: c_int, pattrs: *mut list_head) -> c_int;
}
//
// Return the tracepoint name in the format "subsystem:event_name",
// callers should free the returned string.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_data {
// size is only valid if temp is 'true'
    pub size: isize,
    pub temp: bool,
    pub temp_file: [c_char; 50],
}

extern "C" {
    pub fn tracing_data_put(tdata: *mut tracing_data) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scripting_ops {
    pub name: *const c_char,
    pub /: *const *const *const char dirname; / For script path .../scripts/<dirname>/...,
    pub session): *mut perf_session,
    pub (void): *mut *mut int (flush_script),
    pub (void): *mut *mut int (stop_script),
    pub addr_al): *mut addr_location,
    pub machine): *mut machine,
    pub event): *mut perf_event,
    pub tstamp): *mut *mut evsel evsel, u64,
    pub tstamp): *mut *mut void (process_stat_interval)(u64,
    pub machine): *mut machine,
    pub outfile): *const *const *const int (generate_script) (struct tep_handle pevent, char,
}

extern "C" {
    pub fn script_spec__for_each(ops: *mut *mut int (cb)(struct scripting_ops, spec): *const c_char) -> c_int;
}
extern "C" {
    pub fn setup_perl_scripting();
}
extern "C" {
    pub fn setup_python_scripting();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scripting_context {
    pub pevent: *mut tep_handle,
    pub event_data: *mut c_void,
    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub al: *mut addr_location,
    pub addr_al: *mut addr_location,
    pub session: *mut perf_session,
}

extern "C" {
    pub fn common_pc(context: *mut scripting_context) -> c_int;
}
extern "C" {
    pub fn common_flags(context: *mut scripting_context) -> c_int;
}
extern "C" {
    pub fn common_lock_depth(context: *mut scripting_context) -> c_int;
}
pub const SAMPLE_FLAGS_BUF_SIZE: c_int = 64;
pub const SAMPLE_FLAGS_STR_ALIGNED_SIZE: c_int = 21;
extern "C" {
    pub fn perf_sample__sprintf_flags(flags: u32, str: *mut c_char, sz: usize) -> c_int;
}

