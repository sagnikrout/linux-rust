//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/seq_file.h
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
#[derive(Copy, Clone)]
pub struct seq_file {
    pub buf: *mut c_char,
    pub size: usize,
    pub from: usize,
    pub count: usize,
    pub pad_until: usize,
    pub index: loff_t,
    pub read_pos: loff_t,
    pub lock: mutex,
    pub op: *const seq_operations,
    pub poll_event: c_int,
    pub file: *const file,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_operations {
    pub pos): *mut *mut *mut *mut void  (start) (struct seq_file m, loff_t,
    pub v): *mut *mut *mut void (stop) (struct seq_file m, void,
    pub pos): *mut *mut *mut *mut *mut void  (next) (struct seq_file m, void v, loff_t,
    pub v): *mut *mut *mut int (show) (struct seq_file m, void,
}

pub const SEQ_SKIP: c_int = 1;
//
// seq_has_overflowed - check if the buffer has overflowed
// @m: the seq_file handle
//
// seq_files have a buffer which may overflow. When this happens a larger
// buffer is reallocated and all the data will be printed again.
// The overflow state is true when m->count == m->size.
//
// Returns true if the buffer received more than it can hold.
//
// seq_get_buf - get buffer to write arbitrary data to
// @m: the seq_file handle
// @bufp: the beginning of the buffer is stored here
//
// Return the number of bytes available in the buffer, or zero if
// there's no space.
//
// bufp = m->buf + m->count;
// bufp = NULL;
//
// seq_commit - commit data to the buffer
// @m: the seq_file handle
// @num: the number of bytes to commit
//
// Commit @num bytes of data written to a buffer previously acquired
// by seq_buf_get.  To signal an error condition, or that the data
// didn't fit in the available space, pass a negative @num value.
//
// seq_setwidth - set padding width
// @m: the seq_file handle
// @size: the max number of bytes to pad.
//
// Call seq_setwidth() for setting max width, then call seq_printf() etc. and
// finally call seq_pad() to pad the remaining bytes.
//
extern "C" {
    pub fn seq_pad(m: *mut seq_file, c: c_char);
}
extern "C" {
    pub fn seq_open(: *mut file, : *const seq_operations) -> c_int;
}
extern "C" {
    pub fn seq_read(: *mut file, : *mut char __user, _arg: usize, : *mut loff_t) -> isize;
}
extern "C" {
    pub fn seq_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
}
extern "C" {
    pub fn seq_lseek(: *mut file, _arg: loff_t, _arg: c_int) -> loff_t;
}
extern "C" {
    pub fn seq_release(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn seq_write(seq: *mut seq_file, data: *const c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn seq_vprintf(m: *mut seq_file, fmt: *const c_char, args: va_list);
}
extern "C" {
    pub fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
}
extern "C" {
    pub fn seq_putc(m: *mut seq_file, c: c_char);
}
extern "C" {
    pub fn __seq_puts(m: *mut seq_file, s: *const c_char);
}
extern "C" {
    pub fn seq_put_decimal_ll(m: *mut seq_file, delimiter: *const c_char, num: c_longlong);
}
//
// seq_escape - print string into buffer, escaping some characters
// @m: target buffer
// @s: NULL-terminated string
// @esc: set of characters that need escaping
//
// Puts string into buffer, replacing each occurrence of character from
// @esc with usual octal escape.
//
// Use seq_has_overflowed() to check for errors.
//
extern "C" {
    pub fn seq_path(: *mut seq_file, : *const path, : *const c_char) -> c_int;
}
extern "C" {
    pub fn seq_file_path(: *mut seq_file, : *mut file, : *const c_char) -> c_int;
}
extern "C" {
    pub fn seq_dentry(: *mut seq_file, : *mut dentry, : *const c_char) -> c_int;
}
extern "C" {
    pub fn single_open(: *mut file, : *mut *mut int ()(struct seq_file, ): *mut c_void, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn single_open_size(: *mut file, : *mut *mut int ()(struct seq_file, ): *mut c_void, : *mut c_void, _arg: usize) -> c_int;
}
extern "C" {
    pub fn single_release(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn seq_open_private(: *mut file, : *const seq_operations, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn seq_release_private(: *mut inode, : *mut file) -> c_int;
}

extern "C" {
    pub fn seq_bprintf(m: *mut seq_file, f: *const c_char, binary: *const u32);
}

//
// seq_show_options - display mount options with appropriate escapes.
// @m: the seq_file handle
// @name: the mount option name
// @value: the mount option name's value, can be NULL
//
// seq_show_option_n - display mount options with appropriate escapes
// where @value must be a specific length (i.e.
// not NUL-terminated).
// @m: the seq_file handle
// @name: the mount option name
// @value: the mount option name's value, cannot be NULL
// @length: the exact length of @value to display, must be constant expression
//
// This is a macro since this uses "length" to define the size of the
// stack buffer.
//

//
// Helpers for iteration over list_head-s in seq_files
//
// Helpers for iteration over hlist_head-s in seq_files
//
// Helpers for iterating over per-cpu hlist_head-s in seq_files
extern "C" {
    pub fn seq_file_init();
}
