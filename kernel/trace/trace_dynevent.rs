//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_dynevent.h
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
// Common header file for generic dynamic events.
//

//
// struct dyn_event_operations - Methods for each type of dynamic events
//
// These methods must be set for each type, since there is no default method.
// Before using this for dyn_event_init(), it must be registered by
// dyn_event_register().
//
// @create: Parse and create event method. This is invoked when user passes
// a event definition to dynamic_events interface. This must not destruct
// the arguments and return -ECANCELED if given arguments doesn't match its
// command prefix.
// @show: Showing method. This is invoked when user reads the event definitions
// via dynamic_events interface.
// @is_busy: Check whether given event is busy so that it can not be deleted.
// Return true if it is busy, otherwise false.
// @free: Delete the given event. Return 0 if success, otherwise error.
// @match: Check whether given event and system name match this event. The argc
// and argv is used for exact match. Return true if it matches, otherwise
// false.
//
// Except for @create, these methods are called under holding event_mutex.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_event_operations {
    pub list: list_head,
    pub raw_command): *const *const int (create)(char,
    pub ev): *mut *mut *mut int (show)(struct seq_file m, struct dyn_event,
    pub ev): *mut *mut bool (is_busy)(struct dyn_event,
    pub ev): *mut *mut int (free)(struct dyn_event,
    pub ev): *const *const *const int argc, char argv, struct dyn_event,
}

// Register new dyn_event type -- must be called at first
extern "C" {
    pub fn dyn_event_register(ops: *mut dyn_event_operations) -> c_int;
}
//
// struct dyn_event - Dynamic event list header
//
// The dyn_event structure encapsulates a list and a pointer to the operators
// for making a global list of dynamic events.
// User must includes this in each event structure, so that those events can
// be added/removed via dynamic_events interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_event {
    pub list: list_head,
    pub ops: *mut dyn_event_operations,
}

extern "C" {
    pub fn dyn_event_seq_stop(m: *mut seq_file, v: *mut c_void);
}
extern "C" {
    pub fn dyn_events_release_all(type: *mut dyn_event_operations) -> c_int;
}
extern "C" {
    pub fn dyn_event_release(raw_command: *const c_char, type: *mut dyn_event_operations) -> c_int;
}
extern "C" {
    pub fn dyn_event_create(raw_command: *const c_char, type: *mut dyn_event_operations) -> c_int;
}
//
// for_each_dyn_event	-	iterate over the dyn_event list
// @pos:	the struct dyn_event * to use as a loop cursor
//
// This is just a basement of for_each macro. Wrap this for
// each actual event structure with ops filtering.
//

//
// for_each_dyn_event	-	iterate over the dyn_event list safely
// @pos:	the struct dyn_event * to use as a loop cursor
// @n:		the struct dyn_event * to use as temporary storage
//

extern "C" {
    pub fn int(data: *mut *mut dynevent_check_arg_fn_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynevent_arg {
    pub str: *const c_char,
    pub /: *mut *mut char separator; / e.g. ';', ',', or nothing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynevent_arg_pair {
    pub lhs: *const c_char,
    pub rhs: *const c_char,
    pub /: *mut *mut char operator; / e.g. '=' or nothing,
    pub /: *mut *mut char separator; / e.g. ';', ',', or nothing,
}

extern "C" {
    pub fn dynevent_str_add(cmd: *mut dynevent_cmd, str: *const c_char) -> c_int;
}
