//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/probe-event.h
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

// Probe related configurations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_conf {
    pub show_ext_vars: bool,
    pub show_location_range: bool,
    pub force_add: bool,
    pub no_inlines: bool,
    pub cache: bool,
    pub bootconfig: bool,
    pub max_probes: c_int,
    pub magic_num: c_ulong,
}

pub const DEFAULT_PROBE_MAGIC_NUM: c_uint = 0xdeade12d	/* u32: 3735937325 */;
// kprobe-tracer and uprobe-tracer tracing point
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_trace_point {
    pub /: *mut *mut *mut char realname; / function real name (if needed),
    pub /: *mut *mut *mut char symbol; / Base symbol,
    pub /: *mut *mut *mut char module; / Module name,
    pub /: *mut *mut unsigned long offset; / Offset from symbol,
    pub /: *mut *mut unsigned long ref_ctr_offset; / SDT reference counter offset,
    pub /: *mut *mut u64 address; / Actual address of the trace point,
    pub /: *mut *mut bool retprobe; / Return probe flag,
}

// probe-tracer tracing argument referencing offset
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_trace_arg_ref {
    pub /: *mut *mut *mut probe_trace_arg_ref next; / Next reference,
    pub /: *mut *mut long offset; / Offset value,
    pub /: *mut *mut bool user_access; / User-memory access,
}

// kprobe-tracer and uprobe-tracer tracing argument
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_trace_arg {
    pub /: *mut *mut *mut char name; / Argument name,
    pub /: *mut *mut *mut char value; / Base value,
    pub /: *mut *mut *mut char type; / Type name,
    pub /: *mut *mut *mut probe_trace_arg_ref ref; / Referencing offset,
}

// kprobe-tracer and uprobe-tracer tracing event (point + arg)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_trace_event {
    pub /: *mut *mut *mut char event; / Event name,
    pub /: *mut *mut *mut char group; / Group name,
    pub /: *mut *mut probe_trace_point point; / Trace point,
    pub /: *mut *mut int nargs; / Number of args,
    pub /: *mut *mut int lang; / Dwarf language code,
    pub /: *mut *mut bool uprobes; / uprobes only,
    pub /: *mut *mut *mut probe_trace_arg args; / Arguments,
}

// Perf probe probing point
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_probe_point {
    pub /: *mut *mut *mut char file; / File path,
    pub /: *mut *mut *mut char function; / Function name,
    pub /: *mut *mut int line; / Line number,
    pub /: *mut *mut bool retprobe; / Return probe flag,
    pub /: *mut *mut *mut char lazy_line; / Lazy matching pattern,
    pub /: *mut *mut unsigned long offset; / Offset from function entry,
    pub /: *mut *mut u64 abs_address; / Absolute address of the point,
}

// Perf probe probing argument field chain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_probe_arg_field {
    pub /: *mut *mut *mut perf_probe_arg_field next; / Next field,
    pub /: *mut *mut *mut char name; / Name of the field,
    pub /: *mut *mut long index; / Array index number,
    pub /: *mut *mut bool ref; / Referencing flag,
}

// Perf probe probing argument
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_probe_arg {
    pub /: *mut *mut *mut char name; / Argument name,
    pub /: *mut *mut *mut char var; / Variable name,
    pub /: *mut *mut *mut char type; / Type name,
    pub /: *mut *mut *mut perf_probe_arg_field field; / Structure fields,
    pub /: *mut *mut bool user_access; / User-memory access,
}

// Perf probe probing event (point + arg)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_probe_event {
    pub /: *mut *mut *mut char event; / Event name,
    pub /: *mut *mut *mut char group; / Group name,
    pub /: *mut *mut perf_probe_point point; / Probe point,
    pub /: *mut *mut int nargs; / Number of arguments,
    pub /: *mut *mut bool sdt; / SDT/cached event flag,
    pub /: *mut *mut bool uprobes; / Uprobe event flag,
    pub /: *mut *mut *mut char target; / Target binary,
    pub /: *mut *mut *mut perf_probe_arg args; / Arguments,
    pub tevs: *mut probe_trace_event,
    pub ntevs: c_int,
    pub /: *mut *mut *mut nsinfo nsi; / Target namespace,
}

// Line range
#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_range {
    pub /: *mut *mut *mut char file; / File name,
    pub /: *mut *mut *mut char function; / Function name,
    pub /: *mut *mut int start; / Start line number,
    pub /: *mut *mut int end; / End line number,
    pub /: *mut *mut int offset; / Start line offset,
    pub /: *mut *mut *mut char path; / Real path name,
    pub /: *mut *mut *mut char comp_dir; / Compile directory,
    pub /: *mut *mut *mut intlist line_list; / Visible lines,
}

// List of variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct variable_list {
    pub /: *mut *mut probe_trace_point point; / Actual probepoint,
    pub /: *mut *mut *mut strlist vars; / Available variables,
}

extern "C" {
    pub fn init_probe_symbol_maps(user_only: bool) -> c_int;
}
extern "C" {
    pub fn exit_probe_symbol_maps();
}
// Command string to events
extern "C" {
    pub fn parse_perf_probe_command(cmd: *const c_char, pev: *mut perf_probe_event) -> c_int;
}
extern "C" {
    pub fn parse_probe_trace_command(cmd: *const c_char, tev: *mut probe_trace_event) -> c_int;
}
// Events to command string
extern "C" {
    pub fn perf_probe_with_var(pev: *mut perf_probe_event) -> bool;
}
// Check the perf_probe_event needs debuginfo
extern "C" {
    pub fn perf_probe_event_need_dwarf(pev: *mut perf_probe_event) -> bool;
}
// Release event contents
extern "C" {
    pub fn clear_perf_probe_event(pev: *mut perf_probe_event);
}
extern "C" {
    pub fn clear_probe_trace_event(tev: *mut probe_trace_event);
}
// Command string to line-range
extern "C" {
    pub fn parse_line_range_desc(cmd: *const c_char, lr: *mut line_range) -> c_int;
}
// Release line range members
extern "C" {
    pub fn line_range__clear(lr: *mut line_range);
}
// Initialize line range
extern "C" {
    pub fn line_range__init(lr: *mut line_range) -> c_int;
}
extern "C" {
    pub fn convert_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
}
extern "C" {
    pub fn apply_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
}
extern "C" {
    pub fn show_probe_trace_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
}
extern "C" {
    pub fn show_bootconfig_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int;
}
extern "C" {
    pub fn cleanup_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int);
}
extern "C" {
    pub fn show_perf_probe_events(filter: *mut strfilter) -> c_int;
}
// If there is no space to write, returns -E2BIG.
extern "C" {
    pub fn e_snprintf(str: *mut c_char, size: usize, format: *const c_char, __printf(3: ...), _arg: 4) -> c_int;
}
// Maximum index number of event-name postfix
pub const MAX_EVENT_INDEX: c_int = 1024;
