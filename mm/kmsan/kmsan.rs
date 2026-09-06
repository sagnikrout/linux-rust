//! Automatically rewritten from C Header to Rust Module
//! Source: mm/kmsan/kmsan.h
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
// Functions used by the KMSAN runtime.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

pub const KMSAN_ALLOCA_MAGIC_ORIGIN: c_uint = 0xabcd0100;
pub const KMSAN_CHAIN_MAGIC_ORIGIN: c_uint = 0xabcd0200;
pub const KMSAN_POISON_NOCHECK: c_uint = 0x0;
pub const KMSAN_POISON_CHECK: c_uint = 0x1;
pub const KMSAN_POISON_FREE: c_uint = 0x2;
pub const KMSAN_ORIGIN_SIZE: c_int = 4;
pub const KMSAN_MAX_ORIGIN_DEPTH: c_int = 7;
pub const KMSAN_STACK_DEPTH: c_int = 64;

//
// A pair of metadata pointers to be returned by the instrumentation functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_origin_ptr {
    pub origin: *mut *mut void shadow,,
}

extern "C" {
    pub fn kmsan_init_alloc_meta_for_range(start: *mut c_void, end: *mut c_void) -> void __init;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kmsan_bug_reason {
    REASON_ANY,
    REASON_COPY_TO_USER,
    REASON_SUBMIT_URB,
}

extern "C" {
    pub fn kmsan_print_origin(origin: depot_stack_handle_t);
}
//
// kmsan_report() - Report a use of uninitialized value.
// @origin:    Stack ID of the uninitialized value.
// @address:   Address at which the memory access happens.
// @size:      Memory access size.
// @off_first: Offset (from @address) of the first byte to be reported.
// @off_last:  Offset (from @address) of the last byte to be reported.
// @user_addr: When non-NULL, denotes the userspace address to which the kernel
// is leaking data.
// @reason:    Error type from enum kmsan_bug_reason.
//
// kmsan_report() prints an error message for a consequent group of bytes
// sharing the same origin. If an uninitialized value is used in a comparison,
// this function is called once without specifying the addresses. When checking
// a memory range, KMSAN may call kmsan_report() multiple times with the same
// @address, @size, @user_addr and @reason, but different @off_first and
// @off_last corresponding to different @origin values.
//
extern "C" {
    pub fn in_task(raw_cpu_ptr(&kmsan_percpu_ctx: ) ? &current->kmsan_ctx :) -> return;
}
//
// When a compiler hook or KMSAN runtime function is invoked, it may make a
// call to instrumented code and eventually call itself recursively. To avoid
// that, we guard the runtime entry regions with
// kmsan_enter_runtime()/kmsan_leave_runtime() and exit the hook if
// kmsan_in_runtime() is true.
//
// Non-runtime code may occasionally get executed in nested IRQs from the
// runtime code (e.g. when called via smp_call_function_single()). Because some
// KMSAN routines may take locks (e.g. for memory allocation), we conservatively
// bail out instead of calling them. To minimize the effect of this (potentially
// missing initialization events) kmsan_in_runtime() is not checked in
// non-blocking runtime functions.
//
// Pack and unpack the origin chain depth and UAF flag to/from the extra bits
// provided by the stack depot.
// The UAF flag is stored in the lowest bit, followed by the depth in the upper
// bits.
// set_dsh_extra_bits() is responsible for clamping the value.
//
// kmsan_internal_ functions are supposed to be very simple and not require the
// kmsan_in_runtime() checks.
//
extern "C" {
    pub fn kmsan_internal_memmove_metadata(dst: *mut c_void, src: *mut c_void, n: usize);
}
extern "C" {
    pub fn kmsan_internal_unpoison_memory(address: *mut c_void, size: usize, checked: bool);
}
extern "C" {
    pub fn kmsan_internal_chain_origin(id: depot_stack_handle_t) -> depot_stack_handle_t;
}
extern "C" {
    pub fn kmsan_internal_task_create(task: *mut task_struct);
}
extern "C" {
    pub fn kmsan_metadata_is_contiguous(addr: *mut c_void, size: usize) -> bool;
}
//
// kmsan_internal_is_module_addr() and kmsan_internal_is_vmalloc_addr() are
// non-instrumented versions of is_module_address() and is_vmalloc_addr() that
// are safe to call from KMSAN runtime without recursion.
//
