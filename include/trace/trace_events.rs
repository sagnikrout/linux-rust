//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/trace_events.h
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
// Stage 1 of the trace events.
//
// Override the macros in the event tracepoint header <trace/events/XXX.h>
// to include the following:
//
// struct trace_event_raw_<call> {
// struct trace_entry		ent;
// <type>				<item>;
// <type2>				<item2>[<len>];
// [...]
// };
//
// The <type> <item> is created by the __field(type, item) macro or
// the __array(type2, item2, len) macro.
// We simply do "type item;", and that will create the fields
// in the structure.
//

//
// DECLARE_EVENT_CLASS can be used to add a generic function
// handlers for events. That is, if all events have the same
// parameters and just have distinct trace points.
// Each tracepoint can be defined with DEFINE_EVENT and that
// will map the DECLARE_EVENT_CLASS to the tracepoint.
//
// TRACE_EVENT is a one to one mapping between tracepoint and template.
//

// Callbacks are meaningless to ftrace.

//
// Stage 2 of the trace events.
//
// Include the following:
//
// struct trace_event_data_offsets_<call> {
// u32				<item1>;
// u32				<item2>;
// [...]
// };
//
// The __dynamic_array() macro will create each u32 <item>, this is
// to keep the offset of each array from the beginning of the event.
// The size of an array is also encoded, in the higher 16 bits of <item>.
//

//
// Stage 3 of the trace events.
//
// Override the macros in the event tracepoint header <trace/events/XXX.h>
// to include the following:
//
// enum print_line_t
// trace_raw_output_<call>(struct trace_iterator *iter, int flags)
// {
// struct trace_seq *s = &iter->seq;
// struct trace_event_raw_<call> *field; <-- defined in stage 1
// struct trace_seq *p = &iter->tmp_seq;
//
// -------(for event)-------
//
// struct trace_entry *entry;
//
// entry = iter->ent;
//
// if (entry->type != event_<call>->event.type) {
// WARN_ON_ONCE(1);
// return TRACE_TYPE_UNHANDLED;
// }
//
// field = (typeof(field))entry;
//
// trace_seq_init(p);
// return trace_output_call(iter, <call>, <TP_printk> "\n");
//
// ------(or, for event class)------
//
// int ret;
//
// field = (typeof(field))iter->ent;
//
// ret = trace_raw_output_prep(iter, trace_event);
// if (ret != TRACE_TYPE_HANDLED)
// return ret;
//
// trace_event_printf(iter, <TP_printk> "\n");
//
// return trace_handle_return(s);
// -------
// }
//
// This is the method used to print the raw event to the trace
// output format. Note, this is not needed if the data is read
// in binary.
//

//
// Stage 4 of the trace events.
//
// Override the macros in the event tracepoint header <trace/events/XXX.h>
// to include the following:
//
// For those macros defined with TRACE_EVENT:
//
// static struct trace_event_call event_<call>;
//
// static void trace_event_raw_event_<call>(void *__data, proto)
// {
// struct trace_event_file *trace_file = __data;
// struct trace_event_call *event_call = trace_file->event_call;
// struct trace_event_data_offsets_<call> __maybe_unused __data_offsets;
// unsigned long eflags = trace_file->flags;
// enum event_trigger_type __tt = ETT_NONE;
// struct ring_buffer_event *event;
// struct trace_event_raw_<call> *entry; <-- defined in stage 1
// struct trace_buffer *buffer;
// unsigned long irq_flags;
// int __data_size;
// int pc;
//
// if (!(eflags & EVENT_FILE_FL_TRIGGER_COND)) {
// if (eflags & EVENT_FILE_FL_TRIGGER_MODE)
// event_triggers_call(trace_file, NULL);
// if (eflags & EVENT_FILE_FL_SOFT_DISABLED)
// return;
// }
//
// local_save_flags(irq_flags);
// pc = preempt_count();
//
// __data_size = trace_event_get_offsets_<call>(&__data_offsets, args);
//
// event = trace_event_buffer_lock_reserve(&buffer, trace_file,
// event_<call>->event.type,
// sizeof(*entry) + __data_size,
// irq_flags, pc);
// if (!event)
// return;
// entry	= ring_buffer_event_data(event);
//
// { <assign>; }  <-- Here we assign the entries by the __field and
// __array macros.
//
// if (eflags & EVENT_FILE_FL_TRIGGER_COND)
// __tt = event_triggers_call(trace_file, entry);
//
// if (test_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT,
// &trace_file->flags))
// ring_buffer_discard_commit(buffer, event);
// else if (!filter_check_discard(trace_file, entry, buffer, event))
// trace_buffer_unlock_commit(buffer, event, irq_flags, pc);
//
// if (__tt)
// event_triggers_post_call(trace_file, __tt);
// }
//
// static struct trace_event ftrace_event_type_<call> = {
// .trace			= trace_raw_output_<call>, <-- stage 2
// };
//
// static char print_fmt_<call>[] = <TP_printk>;
//
// static struct trace_event_class __used event_class_<template> = {
// .system			= "<system>",
// .fields_array		= trace_event_fields_<call>,
// .fields			= LIST_HEAD_INIT(event_class_##call.fields),
// .raw_init		= trace_event_raw_init,
// .probe			= trace_event_raw_event_##call,
// .reg			= trace_event_reg,
// };
//
// static struct trace_event_call event_<call> = {
// .class			= event_class_<template>,
// {
// .tp			= &__tracepoint_<call>,
// },
// .event			= &ftrace_event_type_<call>,
// .print_fmt		= print_fmt_<call>,
// .flags			= TRACE_EVENT_FL_TRACEPOINT,
// };
// // its only safe to use pointers when doing linker tricks to
// // create an array.
// static struct trace_event_call __used
// __section("_ftrace_events") *__event_<call> = &event_<call>;
//

// Macro flag: #define _TRACE_PERF_INIT(call)

//
// Per-template BTF id list, populated at link time by resolve_btfids:
// [0] FUNC   __bpf_trace_<call>     (the BPF dispatcher)
// [1] STRUCT trace_event_raw_<call> (the ring-buffer record)
// Exposed via the events/<sys>/<name>/btf_ids tracefs file.
//

// Macro flag: #define _TRACE_BTF_IDS_DECLARE(call)
// Macro flag: #define _TRACE_BTF_IDS_INIT(call)

//
// The ftrace_test_probe is compiled out, it is only here as a build time check
// to make sure that if the tracepoint handling changes, the ftrace probe will
// fail to compile unless it too is updated.
//

