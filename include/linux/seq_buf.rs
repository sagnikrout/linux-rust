//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/seq_buf.h
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
// struct seq_buf - seq buffer structure
// @buffer:	pointer to the buffer
// @size:	size of the buffer
// @len:	the amount of data inside the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_buf {
    pub buffer: *mut c_char,
    pub size: usize,
    pub len: usize,
}

//
// seq_buf have a buffer that might overflow. When this happens
// len is set to be greater than size.
//
// How much buffer is left on the seq_buf?
//
// How much buffer was written?
extern "C" {
    pub fn min(_arg: s->len, _arg: s->size) -> return;
}
//
// seq_buf_str - get NUL-terminated C string from seq_buf
// @s: the seq_buf handle
//
// This makes sure that the buffer in @s is NUL-terminated and
// safe to read as a string.
//
// Note, if this is called when the buffer has overflowed, then
// the last byte of the buffer is zeroed, and the len will still
// point passed it.
//
// After this function is called, s->buffer is safe to use
// in string operations.
//
// Returns: @s->buf after making sure it is terminated.
//
// seq_buf_get_buf - get buffer to write arbitrary data to
// @s: the seq_buf handle
// @bufp: the beginning of the buffer is stored here
//
// Returns: the number of bytes available in the buffer, or zero if
// there's no space.
//
// bufp = s->buffer + s->len;
// bufp = NULL;
//
// seq_buf_commit - commit data to the buffer
// @s: the seq_buf handle
// @num: the number of bytes to commit
//
// Commit @num bytes of data written to a buffer previously acquired
// by seq_buf_get_buf(). To signal an error condition, or that the data
// didn't fit in the available space, pass a negative @num value.
//
// num must be negative on overflow
//
// seq_buf_pop - pop off the last written character
// @s: the seq_buf handle
//
// Removes the last written character to the seq_buf @s.
//
// Returns the last character or -1 if it is empty.
//
extern "C" {
    pub fn seq_buf_printf(s: *mut seq_buf, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn seq_buf_vprintf(s: *mut seq_buf, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn seq_buf_print_seq(m: *mut seq_file, s: *mut seq_buf) -> c_int;
}
extern "C" {
    pub fn seq_buf_puts(s: *mut seq_buf, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn seq_buf_putc(s: *mut seq_buf, c: c_uchar) -> c_int;
}
extern "C" {
    pub fn seq_buf_putmem(s: *mut seq_buf, mem: *const c_void, len: c_uint) -> c_int;
}
extern "C" {
    pub fn seq_buf_path(s: *mut seq_buf, path: *const path, esc: *const c_char) -> c_int;
}

extern "C" {
    pub fn seq_buf_bprintf(s: *mut seq_buf, fmt: *const c_char, binary: *const u32) -> c_int;
}

extern "C" {
    pub fn seq_buf_do_printk(s: *mut seq_buf, lvl: *const c_char);
}
