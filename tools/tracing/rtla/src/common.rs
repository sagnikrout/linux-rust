//! Automatically rewritten from C Header to Rust Module
//! Source: tools/tracing/rtla/src/common.h
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
// osnoise_context - read, store, write, restore osnoise configs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_context {
    pub flags: c_int,
    pub ref: c_int,
    pub curr_cpus: *mut c_char,
    pub orig_cpus: *mut c_char,
// 0 as init value
    pub orig_runtime_us: c_ulonglong,
    pub runtime_us: c_ulonglong,
// 0 as init value
    pub orig_period_us: c_ulonglong,
    pub period_us: c_ulonglong,
// 0 as init value
    pub orig_timerlat_period_us: c_longlong,
    pub timerlat_period_us: c_longlong,
// 0 as init value
    pub orig_tracing_thresh: c_longlong,
    pub tracing_thresh: c_longlong,
// -1 as init value because 0 is disabled
    pub orig_stop_us: c_longlong,
    pub stop_us: c_longlong,
// -1 as init value because 0 is disabled
    pub orig_stop_total_us: c_longlong,
    pub stop_total_us: c_longlong,
// -1 as init value because 0 is disabled
    pub orig_print_stack: c_longlong,
    pub print_stack: c_longlong,
// -1 as init value because 0 is off
    pub orig_opt_irq_disable: c_int,
    pub opt_irq_disable: c_int,
// -1 as init value because 0 is off
    pub orig_opt_workload: c_int,
    pub opt_workload: c_int,
// -1 as init value because 0 is off
    pub orig_opt_timerlat_align: c_int,
    pub opt_timerlat_align: c_int,
// 0 as init value
    pub orig_timerlat_align_us: c_ulonglong,
    pub timerlat_align_us: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_params {
    pub no_irq: bool,
    pub no_thread: bool,
    pub no_header: bool,
    pub no_summary: bool,
    pub no_index: bool,
    pub with_zeros: bool,
    pub bucket_size: c_int,
    pub entries: c_int,
}

//
// common_params - Parameters shared between timerlat_params and osnoise_params
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_params {
// trace configuration
    pub cpus: *mut c_char,
    pub monitored_cpus: cpu_set_t,
    pub events: *mut trace_events,
    pub buffer_size: c_int,
// Timing parameters
    pub warmup: c_int,
    pub stop_us: c_longlong,
    pub stop_total_us: c_longlong,
    pub sleep_time: c_int,
    pub duration: c_int,
// Scheduling parameters
    pub set_sched: c_int,
    pub sched_param: sched_attr,
    pub cgroup: c_int,
    pub cgroup_name: *mut c_char,
    pub hk_cpus: c_int,
    pub hk_cpu_set: cpu_set_t,
// Other parameters
    pub hist: hist_params,
    pub output_divisor: c_int,
    pub pretty_output: bool,
    pub quiet: bool,
    pub user_workload: bool,
    pub kernel_workload: bool,
    pub user_data: bool,
    pub aa_only: bool,
    pub threshold_actions: actions,
    pub end_actions: actions,
    pub user: timerlat_u_params,
}

//
// osnoise_tool -  osnoise based tool definition.
//
// Only the "trace" and "context" fields are used for
// the additional trace instances (record and aa).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_tool {
    pub ops: *mut tool_ops,
    pub trace: trace_instance,
    pub context: *mut osnoise_context,
    pub data: *mut c_void,
    pub params: *mut common_params,
    pub start_time: time_t,
    pub record: *mut osnoise_tool,
    pub aa: *mut osnoise_tool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tool_ops {
    pub tracer: *const c_char,
    pub comm_prefix: *const c_char,
    pub argv[]): *mut *mut *mut common_params (parse_args)(int argc, char,
    pub params): *mut *mut *mut osnoise_tool (init_tool)(common_params,
    pub tool): *mut *mut int (apply_config)(struct osnoise_tool,
    pub tool): *mut *mut int (enable)(struct osnoise_tool,
    pub tool): *mut *mut int (main)(struct osnoise_tool,
    pub tool): *mut *mut void (print_stats)(struct osnoise_tool,
    pub stopped): *mut *mut *mut void (analyze)(struct osnoise_tool tool, bool,
    pub tool): *mut *mut void (free)(struct osnoise_tool,
}

//
// should_continue_tracing - check if tracing should continue after threshold
// @params: pointer to the common parameters structure
//
// Returns true if the continue action was configured (--on-threshold continue),
// indicating that tracing should be restarted after handling the threshold event.
//
// Return: 1 if tracing should continue, 0 otherwise.
//
extern "C" {
    pub fn osnoise_set_cpus(context: *mut osnoise_context, cpus: *mut c_char) -> c_int;
}
extern "C" {
    pub fn osnoise_restore_cpus(context: *mut osnoise_context);
}
extern "C" {
    pub fn osnoise_set_workload(context: *mut osnoise_context, onoff: bool) -> c_int;
}
extern "C" {
    pub fn osnoise_destroy_tool(top: *mut osnoise_tool);
}
extern "C" {
    pub fn osnoise_trace_is_off(tool: *mut osnoise_tool, record: *mut osnoise_tool) -> bool;
}
extern "C" {
    pub fn osnoise_set_stop_us(context: *mut osnoise_context, stop_us: c_longlong) -> c_int;
}
extern "C" {
    pub fn common_apply_config(tool: *mut osnoise_tool, params: *mut common_params) -> c_int;
}
extern "C" {
    pub fn top_main_loop(tool: *mut osnoise_tool) -> c_int;
}
extern "C" {
    pub fn hist_main_loop(tool: *mut osnoise_tool) -> c_int;
}
extern "C" {
    pub fn osn_set_stop(tool: *mut osnoise_tool) -> c_int;
}
