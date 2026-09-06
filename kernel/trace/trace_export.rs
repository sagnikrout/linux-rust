//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_export.c
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
// trace_export.c - export basic ftrace utilities to user space
//
// Copyright (C) 2009 Steven Rostedt <srostedt@redhat.com>
//

// Stub function for events with triggers
    static int ftrace_event_register(struct trace_event_call *call,
    enum trace_reg type, void *data)
    {
    return 0;
    }

//
// The FTRACE_ENTRY_REG macro allows ftrace entry to define register
// function and thus become accessible via perf.
//

    FTRACE_ENTRY(name, struct_name, id, PARAMS(tstruct), PARAMS(print))
// not needed for this file

    struct ____ftrace_##name {						\
    tstruct								\
    };									\
    static void __always_unused ____ftrace_check_##name(void)		\
    {									\
    struct ____ftrace_##name *__entry = core::ptr::null_mut();			\
    \
// force compile-time check on F_printk() */			\
    printk(print);							\
    }

    FTRACE_ENTRY(name, struct_name, id, PARAMS(tstruct), PARAMS(print))

    .type = #_type, .name = #_item,					\
    .size = sizeof(_type), .align = __alignof__(_type),		\
    is_signed_type(_type), .filter_type = _filter_type },

    .type = #_type, .name = #_item,				\
    .size = sizeof(_type), .align = 1,			\
    is_signed_type(_type), .filter_type = _filter_type },

    .type = #_type"["__stringify(_len)"]", .name = #_item,		\
    .size = sizeof(_type[_len]), .align = __alignof__(_type),	\
    is_signed_type(_type), .filter_type = FILTER_OTHER,			\
    .len = _len },

    .type = #_type "[]", .name = #_item,				\
    .size = 0, .align = __alignof__(_type),				\
    is_signed_type(_type), .filter_type = FILTER_OTHER },

    static struct trace_event_fields ftrace_event_fields_##name[] = {	\
    tstruct								\
    {} };

    static struct trace_event_class __refdata event_class_ftrace_##call = {	\
    .system			= __stringify(TRACE_SYSTEM),		\
    .fields_array		= ftrace_event_fields_##call,		\
    .fields			= LIST_HEAD_INIT(event_class_ftrace_##call.fields),\
    .reg			= regfn,				\
    };									\
    \
    struct trace_event_call __used event_##call = {				\
    .class			= &event_class_ftrace_##call,		\
    {								\
    .name			= #call,			\
    },								\
    .event.type		= etype,				\
    .print_fmt		= print,				\
    .flags			= TRACE_EVENT_FL_IGNORE_ENABLE,		\
    };									\
    static struct trace_event_call __used						\
    __section("_ftrace_events") *__event_##call = &event_##call;

    FTRACE_ENTRY_REG(call, struct_name, etype,			\
    PARAMS(tstruct), PARAMS(print), core::ptr::null_mut())
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_is_function(call: *mut trace_event_call) -> bool {
    bool ftrace_event_is_function(struct trace_event_call *call)
    {
    let mut call: return = = &event_function;
    }
