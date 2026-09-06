//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/trace_events.h
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

// Used for event string fields when they are NULL

extern "C" {
    pub fn trace_event_printf(iter: *mut trace_iterator, fmt: *const c_char, ...);
}
// Used to find the offset and length of dynamic fields in trace events
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_dynamic_info {

    pub len: u16,
    pub offset: u16,

    pub offset: u16,
    pub len: u16,

    pub __packed: },
//
// The trace entry - the most basic unit of tracing. This is what
// is printed in the end as a single line in the trace output, such as:
//
// bash-15816 [01]   235.197585: idle_cpu <- irq_enter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_entry {
    pub type: c_ushort,
    pub flags: c_uchar,
    pub preempt_count: c_uchar,
    pub pid: c_int,
}

//
// Trace iterator - used by printout routines who present trace
// results to users and which routines might sleep, etc:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_iterator {
    pub tr: *mut trace_array,
    pub trace: *mut tracer,
    pub array_buffer: *mut array_buffer,
    pub private: *mut c_void,
    pub cpu_file: c_int,
    pub mutex: mutex,
    pub buffer_iter: *mut ring_buffer_iter,
    pub iter_flags: c_ulong,
    pub /: *mut *mut *mut void temp; / temp holder,
    pub temp_size: c_uint,
    pub /: *mut *mut *mut char fmt; / modified format holder,
    pub fmt_size: c_uint,
    pub wait_index: core::sync::atomic::AtomicI32,
// trace_seq for __print_flags() and __print_symbolic() etc.
    pub tmp_seq: trace_seq,
    pub started: cpumask_var_t,
// Set when the file is closed to prevent new waiters
    pub closed: bool,
// it's true when current open file is snapshot
    pub snapshot: bool,
// The below is zeroed out in pipe_read
    pub seq: trace_seq,
    pub ent: *mut trace_entry,
    pub lost_events: c_ulong,
    pub leftover: c_int,
    pub ent_size: c_int,
    pub cpu: c_int,
    pub ts: u64,
    pub pos: loff_t,
    pub idx: c_long,
// All new field here will be zeroed out in pipe_read
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_iter_flags {
    TRACE_FILE_LAT_FMT	= 1,
    TRACE_FILE_ANNOTATE	= 2,
    TRACE_FILE_TIME_IN_NS	= 4,
    TRACE_FILE_PAUSE	= 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_functions {
    pub trace: trace_print_func,
    pub raw: trace_print_func,
    pub hex: trace_print_func,
    pub binary: trace_print_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event {
    pub node: hlist_node,
    pub type: c_int,
    pub funcs: *mut trace_event_functions,
}

extern "C" {
    pub fn register_trace_event(event: *mut trace_event) -> c_int;
}
extern "C" {
    pub fn unregister_trace_event(event: *mut trace_event) -> c_int;
}
// Return values for print_line callback
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum print_line_t {
    TRACE_TYPE_PARTIAL_LINE	= 0,	/* Retry after flushing the seq */
    TRACE_TYPE_HANDLED	= 1,
    TRACE_TYPE_UNHANDLED	= 2,	/* Relay to other output functions */
    TRACE_TYPE_NO_CONSUME	= 3	/* Handled but ask to not consume */
}

extern "C" {
    pub fn trace_handle_return(s: *mut trace_seq) -> print_line_t;
}
extern "C" {
    pub fn tracing_gen_ctx_irq_test(irqs_status: c_uint) -> c_uint;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_flag_type {
    TRACE_FLAG_IRQS_OFF		= 0x01,
    TRACE_FLAG_NEED_RESCHED_LAZY	= 0x02,
    TRACE_FLAG_NEED_RESCHED		= 0x04,
    TRACE_FLAG_HARDIRQ		= 0x08,
    TRACE_FLAG_SOFTIRQ		= 0x10,
    TRACE_FLAG_PREEMPT_RESCHED	= 0x20,
    TRACE_FLAG_NMI			= 0x40,
    TRACE_FLAG_BH_OFF		= 0x80,
}

extern "C" {
    pub fn tracing_gen_ctx_irq_test(_arg: irq_status) -> return;
}
extern "C" {
    pub fn tracing_gen_ctx_flags(_arg: irqflags) -> return;
}
//
// Subtract one from the preemption counter if preemption is enabled,
// see trace_event_buffer_reserve()for details.
//

extern "C" {
    pub fn tracing_record_taskinfo(task: *mut task_struct, flags: c_int);
}
extern "C" {
    pub fn tracing_record_cmdline(task: *mut task_struct);
}
extern "C" {
    pub fn tracing_record_tgid(task: *mut task_struct);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_reg {
    TRACE_REG_REGISTER,
    TRACE_REG_UNREGISTER,

    TRACE_REG_PERF_REGISTER,
    TRACE_REG_PERF_UNREGISTER,
    TRACE_REG_PERF_OPEN,
    TRACE_REG_PERF_CLOSE,
//
// These (ADD/DEL) use a 'boolean' return value, where 1 (true) means a
// custom action was taken and the default action is not to be
// performed.
//
    TRACE_REG_PERF_ADD,
    TRACE_REG_PERF_DEL,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_fields {
    pub type: *const c_char,
    pub name: *const c_char,
    pub size: c_int,
    pub align: c_int,
    pub is_signed:1: c_uint,
    pub needs_test:1: c_uint,
    pub filter_type: c_int,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_class {
    pub system: *const c_char,
    pub probe: *mut c_void,

    pub perf_probe: *mut c_void,

    pub data): *mut trace_reg type, void,
    pub fields_array: *mut trace_event_fields,
    pub ): *mut *mut *mut list_head (get_fields)(trace_event_call,
    pub fields: list_head,
    pub ): *mut *mut int (raw_init)(struct trace_event_call,

//
// Per-template BTF ids set by DECLARE_EVENT_CLASS via BTF_ID() and
// patched by resolve_btfids at link time. NULL for handcrafted classes.
// [0] FUNC   __bpf_trace_<template>
// [1] STRUCT trace_event_raw_<template>
//
    pub btf_ids: *const u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_buffer {
    pub buffer: *mut trace_buffer,
    pub event: *mut ring_buffer_event,
    pub trace_file: *mut trace_event_file,
    pub entry: *mut c_void,
    pub trace_ctx: c_uint,
    pub regs: *mut pt_regs,
}

extern "C" {
    pub fn trace_event_buffer_commit(fbuffer: *mut trace_event_buffer);
}
//
// Event flags:
// CAP_ANY	  - Any user can enable for perf
// NO_SET_FILTER - Set when filter has error and is to be ignored
// IGNORE_ENABLE - For trace internal events, do not enable with debugfs file
// TRACEPOINT    - Event is a tracepoint
// DYNAMIC       - Event is a dynamic event (created at run time)
// KPROBE        - Event is a kprobe
// UPROBE        - Event is a uprobe
// EPROBE        - Event is an event probe
// FPROBE        - Event is an function probe
// CUSTOM        - Event is a custom event (to be attached to an exsiting tracepoint)
// This is set when the custom event has not been attached
// to a tracepoint yet, then it is cleared when it is.
// TEST_STR      - The event has a "%s" that points to a string outside the event
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_call {
    pub list: list_head,
    pub class: *mut trace_event_class,
    pub name: *const c_char,
// Set TRACE_EVENT_FL_TRACEPOINT flag when using "tp"
    pub tp: *mut tracepoint,
}

//
// Static events can disappear with modules,
// where as dynamic ones need their own ref count.
//
// See the TRACE_EVENT_FL_* flags above

extern "C" {
    pub fn trace_event_dyn_try_get_ref(call: *mut trace_event_call) -> bool;
}
extern "C" {
    pub fn trace_event_dyn_put_ref(call: *mut trace_event_call);
}
extern "C" {
    pub fn trace_event_dyn_busy(call: *mut trace_event_call) -> bool;
}

// Without DYNAMIC_EVENTS configured, nothing should be calling this
// Nothing should call this without DYNAIMIC_EVENTS configured.

extern "C" {
    pub fn trace_event_dyn_try_get_ref(_arg: call) -> return;
}
extern "C" {
    pub fn try_module_get(_arg: call->module) -> return;
}

//
// This inline function checks whether call->prog_array
// is valid or not. The function is called in various places,
// outside rcu_read_lock/unlock, as a heuristic to speed up execution.
//
// If this function returns true, and later call->prog_array
// becomes false inside rcu_read_lock/unlock region,
// we bail out then. If this function return false,
// there is a risk that we might miss a few events if the checking
// were delayed until inside rcu_read_lock/unlock region and
// call->prog_array happened to become non-NULL then.
//
// Here, READ_ONCE() is used instead of rcu_access_pointer().
// rcu_access_pointer() requires the actual definition of
// "struct bpf_prog_array" while READ_ONCE() only needs
// a declaration of the same type.
//

extern "C" {
    pub fn trace_put_event_file(file: *mut trace_event_file);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dynevent_type {
    DYNEVENT_TYPE_SYNTH = 1,
    DYNEVENT_TYPE_KPROBE,
    DYNEVENT_TYPE_NONE,
}

extern "C" {
    pub fn int(cmd: *mut *mut dynevent_create_fn_t)(struct dynevent_cmd) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynevent_cmd {
    pub seq: seq_buf,
    pub event_name: *const c_char,
    pub n_fields: c_uint,
    pub type: dynevent_type,
    pub run_command: dynevent_create_fn_t,
    pub private_data: *mut c_void,
}

extern "C" {
    pub fn dynevent_create(cmd: *mut dynevent_cmd) -> c_int;
}
extern "C" {
    pub fn synth_event_delete(name: *const c_char) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_field_desc {
    pub type: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_event_trace_state {
    pub fbuffer: trace_event_buffer,
    pub entry: *mut synth_trace_event,
    pub buffer: *mut trace_buffer,
    pub event: *mut synth_event,
    pub cur_field: c_uint,
    pub n_u64: c_uint,
    pub disabled: bool,
    pub add_next: bool,
    pub add_name: bool,
}

extern "C" {
    pub fn synth_event_trace_end(trace_state: *mut synth_event_trace_state) -> c_int;
}
extern "C" {
    pub fn kprobe_event_delete(name: *const c_char) -> c_int;
}

extern "C" {
    pub fn __kprobe_event_add_fields(cmd: *mut dynevent_cmd, ...) -> c_int;
}

//
// Event file flags:
// ENABLED	  - The event is enabled
// RECORDED_CMD  - The comms should be recorded at sched_switch
// RECORDED_TGID - The tgids should be recorded at sched_switch
// FILTERED	  - The event has a filter attached
// NO_SET_FILTER - Set when filter has error and is to be ignored
// SOFT_DISABLED - When set, do not trace the event (even though its
// tracepoint may be enabled)
// TRIGGER_MODE  - When set, invoke the triggers associated with the event
// TRIGGER_COND  - When set, one or more triggers has an associated filter
// PID_FILTER    - When set, the event is filtered based on pid
// WAS_ENABLED   - Set when enabled to know to clear trace on module removal
// FREED         - File descriptor is freed, all fields should be considered invalid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_event_file {
    pub list: list_head,
    pub event_call: *mut trace_event_call,
    pub filter: *mut event_filter __rcu,
    pub ei: *mut eventfs_inode,
    pub tr: *mut trace_array,
    pub system: *mut trace_subsystem_dir,
    pub triggers: list_head,
//
// 32 bit flags:
// bit 0:		enabled
// bit 1:		enabled cmd record
// bit 2:		enable/disable with the soft disable bit
// bit 3:		soft disabled
// bit 4:		trigger enabled
//
// Note: The bits must be set atomically to prevent races
// from other writers. Reads of flags do not need to be in
// sync as they occur in critical sections. But the way flags
// is currently used, these changes do not affect the code
// except that when a change is made, it may have a slight
// delay in propagating the changes to other CPUs due to
// caching and such. Which is mostly OK ;-)
//
    pub flags: c_ulong,
    pub /: *mut *mut refcount_t ref; / ref count for opened files,
    pub /: *mut *mut atomic_t sm_ref; / soft-mode reference counter,
    pub /: *mut *mut atomic_t tm_ref; / trigger-mode reference counter,
}

pub const PERF_MAX_TRACE_SIZE: c_int = 8192;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum event_trigger_type {
    ETT_NONE		= (0),
    ETT_TRACE_ONOFF		= (1 << 0),
    ETT_SNAPSHOT		= (1 << 1),
    ETT_STACKTRACE		= (1 << 2),
    ETT_EVENT_ENABLE	= (1 << 3),
    ETT_EVENT_HIST		= (1 << 4),
    ETT_HIST_ENABLE		= (1 << 5),
    ETT_EVENT_EPROBE	= (1 << 6),
}

extern "C" {
    pub fn filter_match_preds(filter: *mut event_filter, rec: *mut c_void) -> c_int;
}
extern "C" {
    pub fn trace_event_ignore_this_pid(trace_file: *mut trace_event_file) -> bool;
}
extern "C" {
    pub fn __trace_trigger_soft_disabled(file: *mut trace_event_file) -> bool;
}
//
// trace_trigger_soft_disabled - do triggers and test if soft disabled
// @file: The file pointer of the event to test
//
// If any triggers without filters are attached to this event, they
// will be called here. If the event is soft disabled and has no
// triggers that require testing the fields, it will return true,
// otherwise false.
//
extern "C" {
    pub fn __trace_trigger_soft_disabled(_arg: file) -> return;
}

extern "C" {
    pub fn trace_call_bpf(call: *mut trace_event_call, ctx: *mut c_void) -> c_uint;
}
extern "C" {
    pub fn trace_call_bpf_faultable(call: *mut trace_event_call, ctx: *mut c_void) -> c_uint;
}
extern "C" {
    pub fn perf_event_attach_bpf_prog(event: *mut perf_event, prog: *mut bpf_prog, bpf_cookie: u64) -> c_int;
}
extern "C" {
    pub fn perf_event_detach_bpf_prog(event: *mut perf_event);
}
extern "C" {
    pub fn perf_event_query_prog_array(event: *mut perf_event, info: *mut void __user) -> c_int;
}
extern "C" {
    pub fn bpf_probe_register(btp: *mut bpf_raw_event_map, link: *mut bpf_raw_tp_link) -> c_int;
}
extern "C" {
    pub fn bpf_probe_unregister(btp: *mut bpf_raw_event_map, link: *mut bpf_raw_tp_link) -> c_int;
}
extern "C" {
    pub fn bpf_put_raw_tracepoint(btp: *mut bpf_raw_event_map);
}
extern "C" {
    pub fn bpf_kprobe_multi_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_uprobe_multi_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn bpf_tracing_multi_attach(prog: *mut bpf_prog, attr: *const bpf_attr) -> c_int;
}

extern "C" {
    pub fn trace_event_raw_init(call: *mut trace_event_call) -> c_int;
}
extern "C" {
    pub fn trace_add_event_call(call: *mut trace_event_call) -> c_int;
}
extern "C" {
    pub fn trace_remove_event_call(call: *mut trace_event_call) -> c_int;
}
extern "C" {
    pub fn trace_event_get_offsets(call: *mut trace_event_call) -> c_int;
}
extern "C" {
    pub fn ftrace_set_clr_event(tr: *mut trace_array, buf: *mut c_char, set: c_int) -> c_int;
}
extern "C" {
    pub fn trace_set_clr_event(system: *const c_char, event: *const c_char, set: c_int) -> c_int;
}

extern "C" {
    pub fn perf_trace_init(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn perf_trace_destroy(event: *mut perf_event);
}
extern "C" {
    pub fn perf_trace_add(event: *mut perf_event, flags: c_int) -> c_int;
}
extern "C" {
    pub fn perf_trace_del(event: *mut perf_event, flags: c_int);
}

extern "C" {
    pub fn perf_kprobe_init(event: *mut perf_event, is_retprobe: bool) -> c_int;
}
extern "C" {
    pub fn perf_kprobe_destroy(event: *mut perf_event);
}

extern "C" {
    pub fn perf_uprobe_destroy(event: *mut perf_event);
}

extern "C" {
    pub fn ftrace_profile_free_filter(event: *mut perf_event);
}
extern "C" {
    pub fn perf_trace_buf_update(record: *mut c_void, type: u16);
}
extern "C" {
    pub fn perf_event_set_bpf_prog(event: *mut perf_event, prog: *mut bpf_prog, bpf_cookie: u64) -> c_int;
}
extern "C" {
    pub fn perf_event_free_bpf_prog(event: *mut perf_event);
}
extern "C" {
    pub fn bpf_trace_run1(link: *mut bpf_raw_tp_link, arg1: u64);
}
extern "C" {
    pub fn bpf_trace_run2(link: *mut bpf_raw_tp_link, arg1: u64, arg2: u64);
}

pub const TRACE_EVENT_STR_MAX: c_int = 512;
//
// gcc warns that you can not use a va_list in an inlined
// function. But lets me make it into a macro :-
//

//
// Note: we keep the TRACE_CUSTOM_EVENT outside the include file ifdef protection.
// This is due to the way trace custom events work. If a file includes two
// trace event headers under one "CREATE_CUSTOM_TRACE_EVENTS" the first include
// will override the TRACE_CUSTOM_EVENT and break the second include.
//

