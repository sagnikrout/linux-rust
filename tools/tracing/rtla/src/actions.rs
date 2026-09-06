//! Automatically rewritten from C Header to Rust Module
//! Source: tools/tracing/rtla/src/actions.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum action_type {
    ACTION_NONE = 0,
    ACTION_TRACE_OUTPUT,
    ACTION_SIGNAL,
    ACTION_SHELL,
    ACTION_CONTINUE,
    ACTION_FIELD_N
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action {
    pub type: action_type,
// For ACTION_TRACE_OUTPUT
    pub trace_output: *mut c_char,
}

// For ACTION_SIGNAL
// For ACTION_SHELL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct actions {
    pub list: *mut action,
    pub size: int len,,
    pub present: [bool; ACTION_FIELD_N],
    pub continue_flag: bool,
// External dependencies
    pub trace_output_inst: *mut tracefs_instance,
}

extern "C" {
    pub fn actions_init(self: *mut actions);
}
extern "C" {
    pub fn actions_destroy(self: *mut actions);
}
extern "C" {
    pub fn actions_add_trace_output(self: *mut actions, trace_output: *const c_char);
}
extern "C" {
    pub fn actions_add_signal(self: *mut actions, signal: c_int, pid: c_int);
}
extern "C" {
    pub fn actions_add_shell(self: *mut actions, command: *const c_char);
}
extern "C" {
    pub fn actions_add_continue(self: *mut actions);
}
extern "C" {
    pub fn actions_parse(self: *mut actions, trigger: *const c_char, tracefn: *const c_char) -> c_int;
}
extern "C" {
    pub fn actions_perform(self: *mut actions) -> c_int;
}
