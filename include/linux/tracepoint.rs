//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tracepoint.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Kernel Tracepoint API.
//
// See Documentation/trace/tracepoints.rst.
//
// Copyright (C) 2008-2014 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//
// Heavily inspired from the Linux Kernel Markers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_eval_map {
    pub system: *const c_char,
    pub eval_string: *const c_char,
    pub eval_value: c_ulong,
}

pub const TRACEPOINT_DEFAULT_PRIO: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_module {
    pub list: list_head,
    pub mod: *mut module,
}

extern "C" {
    pub fn trace_module_has_bad_taint(mod: *mut module) -> bool;
}
extern "C" {
    pub fn register_tracepoint_module_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_tracepoint_module_notifier(nb: *mut notifier_block) -> c_int;
}

//
// tracepoint_synchronize_unregister must be called between the last tracepoint
// probe unregistration and the end of module exit to make sure there is no
// caller executing a probe when it is freed.
//
// An alternative is to use the following for batch reclaim associated
// with a given tracepoint:
//
// - tracepoint_is_faultable() == false: call_srcu()
// - tracepoint_is_faultable() == true:  call_rcu_tasks_trace()
//

//
// Run RCU callback with the appropriate grace period wait for non-faultable
// tracepoints, e.g., those used in atomic context.
//
// Run RCU callback with the appropriate grace period wait for faultable
// tracepoints, e.g., those used in syscall context.
//

extern "C" {
    pub fn syscall_regfunc() -> c_int;
}
extern "C" {
    pub fn syscall_unregfunc();
}

// Macro flag: #define TRACE_DEFINE_ENUM(x)
// Macro flag: #define TRACE_DEFINE_SIZEOF(x)

extern "C" {
    pub fn offset_to_ptr(_arg: p) -> return;
}

//
// Note: we keep the TRACE_EVENT and DECLARE_TRACE outside the include
// file ifdef protection.
// This is due to the way trace events work. If a file includes two
// trace event headers under one "CREATE_TRACE_POINTS" the first include
// will override the TRACE_EVENT and break the second include.
//

//
// Individual subsystem may have a separate configuration to
// enable their tracepoints. By default, this file will create
// the tracepoints if CONFIG_TRACEPOINTS is defined. If a subsystem
// wants to be able to disable its tracepoints from being created
// it can define NOTRACE before including the tracepoint headers.
//

// Macro flag: #define TRACEPOINTS_ENABLED

//
// Declare an exported function that Rust code can call to trigger this
// tracepoint. This function does not include the static branch; that is done
// in Rust to avoid a function call when the tracepoint is disabled.
//

//
// When a tracepoint is used, it's name is added to the __tracepoint_check
// section. This section is only used at build time to make sure all
// defined tracepoints are used. It is discarded after the build.
//

//
// Make sure the alignment of the structure in the __tracepoints section will
// not add unwanted padding between the beginning of the section and the
// structure. Force alignment to the same alignment as the section start.
//
// When lockdep is enabled, we make sure to always test if RCU is
// "watching" regardless if the tracepoint is enabled or not. Tracepoints
// require RCU to be active, and it should always warn at the tracepoint
// site if it is not watching, as it will need to be active when the
// tracepoint is enabled.
//

//
// We have no guarantee that gcc and the linker won't up-align the tracepoint
// structures, so we create an array of pointers that will be used for iteration
// on the tracepoints.
//
// it_func[0] is never NULL because there is at least one element in the array
// when the array itself is non NULL.
//

// \
// Annotate the probestub 'CFI_NOSEAL' to stop objtool from	\
// requesting the kernel remove the ENDBR, because the only	\
// references to the function are in the __tracepoint section,	\
// that objtool doesn't scan.					\
// \

// Macro flag: #define EXPORT_TRACEPOINT_SYMBOL_GPL(name)
// Macro flag: #define EXPORT_TRACEPOINT_SYMBOL(name)

//
// tracepoint_string - register constant persistent string to trace system
// @str - a constant persistent string that will be referenced in tracepoints
//
// If constant strings are being used in tracepoints, it is faster and
// more efficient to just save the pointer to the string and reference
// that with a printf "%s" instead of saving the string in the ring buffer
// and wasting space and time.
//
// The problem with the above approach is that userspace tools that read
// the binary output of the trace buffers do not have access to the string.
// Instead they just show the address of the string which is not very
// useful to users.
//
// With tracepoint_string(), the string will be registered to the tracing
// system and exported to userspace via the debugfs/tracing/printk_formats
// file that maps the string address to the string text. This way userspace
// tools that read the binary buffers have a way to map the pointers to
// the ASCII strings they represent.
//
// The @str used must be a constant string and persistent as it would not
// make sense to show a string that no longer exists. But it is still fine
// to be used with modules, because when modules are unloaded, if they
// had tracepoints, the ring buffers are cleared too. As long as the string
// does not change during the life of the module, it is fine to use
// tracepoint_string() within a module.
//

//
// tracepoint_string() is used to save the string address for userspace
// tracing tools. When tracing isn't configured, there's no need to save
// anything.
//

//
// For use with the TRACE_EVENT macro:
//
// We define a tracepoint, its arguments, its printk format
// and its 'fast binary record' layout.
//
// Firstly, name your tracepoint via TRACE_EVENT(name : the
// 'subsystem_event' notation is fine.
//
// Think about this whole construct as the
// 'trace_sched_switch() function' from now on.
//
// TRACE_EVENT(sched_switch,
//
// * A function has a regular function arguments
// * prototype, declare it via TP_PROTO():
//
// TP_PROTO(struct rq *rq, struct task_struct *prev,
// struct task_struct *next),
//
// * Define the call signature of the 'function'.
// * (Design sidenote: we use this instead of a
// *  TP_PROTO1/TP_PROTO2/TP_PROTO3 ugliness.)
//
// TP_ARGS(rq, prev, next),
//
// * Fast binary tracing: define the trace record via
// * TP_STRUCT__entry(). You can think about it like a
// * regular C structure local variable definition.
//
// * This is how the trace record is structured and will
// * be saved into the ring buffer. These are the fields
// * that will be exposed to user-space in
// * /sys/kernel/tracing/events/<*>/format.
//
// * The declared 'local variable' is called '__entry'
//
// * __field(pid_t, prev_pid) is equivalent to a standard declaration:
//
// *	pid_t	prev_pid;
//
// * __array(char, prev_comm, TASK_COMM_LEN) is equivalent to:
//
// *	char	prev_comm[TASK_COMM_LEN];
//
// TP_STRUCT__entry(
// __array(	char,	prev_comm,	TASK_COMM_LEN	)
// __field(	pid_t,	prev_pid			)
// __field(	int,	prev_prio			)
// __array(	char,	next_comm,	TASK_COMM_LEN	)
// __field(	pid_t,	next_pid			)
// __field(	int,	next_prio			)
// ),
//
// * Assign the entry into the trace record, by embedding
// * a full C statement block into TP_fast_assign(). You
// * can refer to the trace record as '__entry' -
// * otherwise you can put arbitrary C code in here.
//
// * Note: this C code will execute every time a trace event
// * happens, on an active tracepoint.
//
// TP_fast_assign(
// memcpy(__entry->next_comm, next->comm, TASK_COMM_LEN);
// __entry->prev_pid	= prev->pid;
// __entry->prev_prio	= prev->prio;
// memcpy(__entry->prev_comm, prev->comm, TASK_COMM_LEN);
// __entry->next_pid	= next->pid;
// __entry->next_prio	= next->prio;
// ),
//
// * Formatted output of a trace record via TP_printk().
// * This is how the tracepoint will appear under ftrace
// * plugins that make use of this tracepoint.
//
// * (raw-binary tracing wont actually perform this step.)
//
// TP_printk("task %s:%d [%d] ==> %s:%d [%d]",
// __entry->prev_comm, __entry->prev_pid, __entry->prev_prio,
// __entry->next_comm, __entry->next_pid, __entry->next_prio),
//
// );
//
// This macro construct is thus used for the regular printk format
// tracing setup, it is used to construct a function pointer based
// tracepoint callback (this is used by programmatic plugins and
// can also by used by generic instrumentation like SystemTap), and
// it is also used to expose a structured trace record in
// /sys/kernel/tracing/events/.
//
// A set of (un)registration functions can be passed to the variant
// TRACE_EVENT_FN to perform any (un)registration work.
//

