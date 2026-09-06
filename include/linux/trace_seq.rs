//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/trace_seq.h
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
// Trace sequences are used to allow a function to call several other functions
// to create a string of data to use.
//
// Have the trace seq to be 8K which is typically PAGE_SIZE * 2 on
// most architectures. The TRACE_SEQ_BUFFER_SIZE (which is
// TRACE_SEQ_SIZE minus the other fields of trace_seq), is the
// max size the output of a trace event may be.
//
pub const TRACE_SEQ_SIZE: c_int = 8192;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_seq {
    pub seq: seq_buf,
    pub readpos: usize,
    pub full: c_int,
    pub buffer: [c_char; TRACE_SEQ_BUFFER_SIZE],
}

//
// trace_seq_used - amount of actual data written to buffer
// @s: trace sequence descriptor
//
// Returns the amount of data written to the buffer.
//
// IMPORTANT!
//
// Use this instead of @s->seq.len if you need to pass the amount
// of data from the buffer to another buffer (userspace, or what not).
// The @s->seq.len on overflow is bigger than the buffer size and
// using it can cause access to undefined memory.
//
extern "C" {
    pub fn seq_buf_used(_arg: &s->seq) -> return;
}
//
// trace_seq_buffer_ptr - return pointer to next location in buffer
// @s: trace sequence descriptor
//
// Returns the pointer to the buffer where the next write to
// the buffer will happen. This is useful to save the location
// that is about to be written to and then return the result
// of that write.
//
// trace_seq_has_overflowed - return true if the trace_seq took too much
// @s: trace sequence descriptor
//
// Returns true if too much data was added to the trace_seq and it is
// now full and will not take anymore.
//
// trace_seq_pop - pop off the last written character
// @s: trace sequence descriptor
//
// Removes the last written character to the trace_seq @s.
//
// Returns the last character or -1 if it is empty.
//
extern "C" {
    pub fn seq_buf_pop(_arg: &s->seq) -> return;
}
//
// Currently only defined when tracing is enabled.
//

extern "C" {
    pub fn trace_seq_printf(s: *mut trace_seq, fmt: *const c_char, ...);
}
extern "C" {
    pub fn trace_seq_vprintf(s: *mut trace_seq, fmt: *const c_char, args: va_list);
}
extern "C" {
    pub fn trace_seq_bprintf(s: *mut trace_seq, fmt: *const c_char, binary: *const u32);
}
extern "C" {
    pub fn trace_print_seq(m: *mut seq_file, s: *mut trace_seq) -> c_int;
}
extern "C" {
    pub fn trace_seq_puts(s: *mut trace_seq, str: *const c_char);
}
extern "C" {
    pub fn trace_seq_putc(s: *mut trace_seq, c: c_uchar);
}
extern "C" {
    pub fn trace_seq_putmem(s: *mut trace_seq, mem: *const c_void, len: c_uint);
}
extern "C" {
    pub fn trace_seq_path(s: *mut trace_seq, path: *const path) -> c_int;
}

