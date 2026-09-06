//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_probe.h
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
// Common header file for probe-based Dynamic events.
//
// This code was copied from kernel/trace/trace_kprobe.h written by
// Masami Hiramatsu <masami.hiramatsu.pt@hitachi.com>
//
// Updates to make this generic:
// Copyright (C) IBM Corporation, 2010-2011
// Author:     Srikar Dronamraju
//

pub const MAX_TRACE_ARGS: c_int = 128;
pub const MAX_ARGSTR_LEN: c_int = 255;
pub const MAX_ARRAY_LEN: c_int = 64;
pub const MAX_ARG_NAME_LEN: c_int = 32;
pub const MAX_BTF_ARGS_LEN: c_int = 128;
pub const MAX_DENTRY_ARGS_LEN: c_int = 256;

pub const MAX_PROBE_EVENT_SIZE: c_int = 3072;
// Reserved field names

// Flags for trace_probe
pub const TP_FLAG_TRACE: c_int = 1;
pub const TP_FLAG_PROFILE: c_int = 2;
// data_loc: data location, compatible with u32

extern "C" {
    pub fn make_data_loc(consumed: maxlen -, consumed: offset +) -> return;
}
// Printing function type
extern "C" {
    pub fn int(: *mut *mut print_type_func_t)(struct trace_seq, : *mut c_void, : *mut c_void) -> typedef;
}

// Stage 1 (load) ops */					\
// Stage 2 (dereference) ops */					\
// Stage 3 (store) ops */					\
// Stage 4 (modify) op */					\
// Stage 5 (loop) op */						\
// End */							\
// Unresolved Symbol holder */					\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fetch_insn {
    pub op: fetch_op,
    pub param: c_uint,
    pub size: c_uint,
    pub offset: c_int,
}

// fetch + deref*N + store + mod + end <= 16, this allows N=12, enough
pub const FETCH_INSN_MAX: c_int = 16;

// Fetch type information table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fetch_type {
    pub /: *const *const *const char name; / Name of type,
    pub /: *mut *mut size_t size; / Byte size of type,
    pub /: *mut *mut bool is_signed; / Signed flag,
    pub /: *mut *mut bool is_string; / String flag,
    pub /: *mut *mut print_type_func_t print; / Print functions,
    pub /: *const *const *const char fmt; / Format string,
    pub /: *const *const *const char fmttype; / Name in format file,
}

// For defining macros, define string/string_size types
pub type string = u32;
pub type string_size = u32;

// Printing  in basic type function template

// Default (unsigned long) fetch type

// Non string types can use these macros

// If ptype is an alias of atype, use this macro (show atype in format)

extern "C" {
    pub fn trace_kprobe_on_func_entry(call: *mut trace_event_call) -> bool;
}
extern "C" {
    pub fn trace_kprobe_error_injectable(call: *mut trace_event_call) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_arg {
    pub code: *mut fetch_insn,
    pub /: *mut *mut bool dynamic;/ Dynamic array (string) is used,
    pub /: *mut *mut unsigned int offset; / Offset from argument entry,
    pub /: *mut *mut unsigned int count; / Array count,
    pub /: *const *const *const char name; / Name of this argument,
    pub /: *const *const *const char comm; / Command of this argument,
    pub /: *mut *mut *mut char fmt; / Format string if needed,
    pub /: *const *const *const fetch_type type; / Type of this argument,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_entry_arg {
    pub /: *mut *mut unsigned int size; / The entry data size,
    pub __counted_by(size): fetch_insn code[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_uprobe_filter {
    pub rwlock: rwlock_t,
    pub nr_systemwide: c_int,
    pub perf_events: list_head,
}

// Event call and class holder
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_probe_event {
    pub /: *mut *mut *mut unsigned int flags; / For TP_FLAG_,
    pub class: trace_event_class,
    pub call: trace_event_call,
    pub files: list_head,
    pub probes: list_head,
    pub field_strings: *mut c_char,
    pub nr_field_strings: c_int,
    pub filter: [trace_uprobe_filter; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_probe {
    pub list: list_head,
    pub event: *mut trace_probe_event,
    pub /: *mut *mut ssize_t size; / trace entry size,
    pub nr_args: c_uint,
    pub /: *mut *mut *mut probe_entry_arg entry_arg; / This is only for return probe,
    pub args: [probe_arg; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_file_link {
    pub file: *mut trace_event_file,
    pub list: list_head,
}

extern "C" {
    pub fn smp_load_acquire(_arg: &tp->event->flags) -> return;
}
extern "C" {
    pub fn trace_probe_test_flag(_arg: tp, TP_FLAG_PROFILE: TP_FLAG_TRACE |) -> return;
}
extern "C" {
    pub fn trace_event_name(_arg: &tp->event->call) -> return;
}
extern "C" {
    pub fn container_of(_arg: event_call, trace_probe_event: struct, _arg: call) -> return;
}
extern "C" {
    pub fn list_first_entry_or_null(_arg: &tpe->probes, trace_probe: struct, _arg: list) -> return;
}
// tp->event is unregistered in trace_remove_event_call()
extern "C" {
    pub fn trace_remove_event_call(_arg: &tp->event->call) -> return;
}
extern "C" {
    pub fn list_is_singular(_arg: &tp->event->files) -> return;
}
extern "C" {
    pub fn trace_probe_cleanup(tp: *mut trace_probe);
}
extern "C" {
    pub fn trace_probe_append(tp: *mut trace_probe, to: *mut trace_probe) -> c_int;
}
extern "C" {
    pub fn trace_probe_unlink(tp: *mut trace_probe);
}
extern "C" {
    pub fn trace_probe_register_event_call(tp: *mut trace_probe) -> c_int;
}
extern "C" {
    pub fn trace_probe_add_file(tp: *mut trace_probe, file: *mut trace_event_file) -> c_int;
}
extern "C" {
    pub fn trace_probe_compare_arg_type(a: *mut trace_probe, b: *mut trace_probe) -> c_int;
}
extern "C" {
    pub fn trace_probe_create(raw_command: *const c_char, (*createfn)(int: *mut c_int, ): *const c_char) -> c_int;
}

extern "C" {
    pub fn trace_probe_dump_args(m: *mut seq_file, tp: *mut trace_probe);
}

extern "C" {
    pub fn traceprobe_get_entry_data_size(tp: *mut trace_probe) -> c_int;
}
// This is a runtime function to store entry data
extern "C" {
    pub fn store_trace_entry_data(edata: *mut c_void, tp: *mut trace_probe, regs: *mut pt_regs);
}

//
// The flags used for parsing trace_probe arguments.
// TPARG_FL_RETURN, TPARG_FL_FENTRY and TPARG_FL_TEVENT are mutually exclusive.
// TPARG_FL_KERNEL and TPARG_FL_USER are also mutually exclusive.
// TPARG_FL_FPROBE and TPARG_FL_TPOINT are optional but it should be with
// TPARG_FL_KERNEL.
//

// Each typecast consumes nested level. So the max number of typecast is 8.
pub const TRACEPROBE_MAX_NESTED_LEVEL: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum parse_state_type {
    STATE_DEREF,
    STATE_TYPECAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_state {
    pub type: c_int,
    pub deref: c_int,
    pub offset: c_long,
    pub cur_offs: c_int,
    pub inner_arg: *mut c_char,
    pub is_cpu_read: bool,
    pub deref: },
    pub casttype: *mut c_char,
    pub fieldname: *mut c_char,
    pub orig_offset: c_int,
    pub field_offset_diff: c_int,
    pub inner_arg: *mut c_char,
    pub typecast: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct traceprobe_parse_context {
    pub event: *mut trace_event_call,
// BTF related parameters
    pub /: *const *const *const char funcname; / Function name in BTF,
    pub /: *const *const *const btf_type proto; / Prototype of the function,
    pub /: *const *const *const btf_param params; / Parameter of the function,
    pub /: *mut *mut s32 nr_params; / The number of the parameters,
    pub /: *mut *mut *mut btf btf; / The BTF to be used,
    pub /: *mut *mut *mut btf struct_btf; / The BTF to be used for structs,
    pub /: *const *const *const btf_type last_type; / Saved type,
    pub /: *const *const *const btf_type last_struct; / Saved structure,
    pub /: *mut *mut u32 last_bitoffs; / Saved bitoffs,
    pub /: *mut *mut u32 last_bitsize; / Saved bitsize,
    pub tp: *mut trace_probe,
    pub flags: c_uint,
    pub offset: c_int,
    pub /: *mut *mut int prefix_byteoffs; / The byte offset of the prefix field of typecast,
    pub 1]: parse_state stack[TRACEPROBE_MAX_NESTED_LEVEL +,
    pub depth: c_int,
}

extern "C" {
    pub fn traceprobe_expand_dentry_args(argc: c_int, argv[]: *const c_char, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn traceprobe_update_arg(arg: *mut probe_arg) -> c_int;
}
extern "C" {
    pub fn traceprobe_free_probe_arg(arg: *mut probe_arg);
}
//
// If either traceprobe_parse_probe_arg() or traceprobe_expand_meta_args() is called,
// this MUST be called for clean up the context and return a resource.
//
extern "C" {
    pub fn traceprobe_finish_parse(ctx: *mut traceprobe_parse_context);
}
extern "C" {
    pub fn traceprobe_split_symbol_offset(symbol: *mut c_char, offset: *mut c_long) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum probe_print_type {
    PROBE_PRINT_NORMAL,
    PROBE_PRINT_RETURN,
    PROBE_PRINT_EVENT,
}

extern "C" {
    pub fn traceprobe_set_print_fmt(tp: *mut trace_probe, ptype: probe_print_type) -> c_int;
}

extern "C" {
    pub fn destroy_local_trace_kprobe(event_call: *mut trace_event_call);
}
extern "C" {
    pub fn destroy_local_trace_uprobe(event_call: *mut trace_event_call);
}

// Define TP_ERR_
// Error text is defined in trace_probe.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_probe_log {
    pub subsystem: *const c_char,
    pub argv: *const c_char,
    pub argc: c_int,
    pub index: c_int,
}

extern "C" {
    pub fn trace_probe_log_set_index(index: c_int);
}
extern "C" {
    pub fn trace_probe_log_clear();
}
extern "C" {
    pub fn __trace_probe_log_err(offset: c_int, err: c_int);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uprobe_dispatch_data {
    pub tu: *mut trace_uprobe,
    pub bp_addr: c_ulong,
}
