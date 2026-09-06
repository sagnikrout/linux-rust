//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_synth.h
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

pub const SYNTH_FIELDS_MAX: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_field {
    pub type: *mut c_char,
    pub name: *mut c_char,
    pub size: usize,
    pub offset: c_uint,
    pub field_pos: c_uint,
    pub is_signed: bool,
    pub is_string: bool,
    pub is_dynamic: bool,
    pub is_stack: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_event {
    pub devent: dyn_event,
    pub ref: c_int,
    pub name: *mut c_char,
    pub fields: *mut synth_field,
    pub n_fields: c_uint,
    pub dynamic_fields: *mut synth_field,
    pub n_dynamic_fields: c_uint,
    pub n_u64: c_uint,
    pub class: trace_event_class,
    pub call: trace_event_call,
    pub tp: *mut tracepoint,
    pub mod: *mut module,
}
