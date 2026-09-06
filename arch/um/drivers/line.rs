//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/line.h
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
// Copyright (C) 2001, 2002 Jeff Dike (jdike@karaya.com)
//

// There's only two modifiable fields in this - .mc.list and .driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_driver {
    pub name: *const c_char,
    pub device_name: *const c_char,
    pub major: c_short,
    pub minor_start: c_short,
    pub type: c_short,
    pub subtype: c_short,
    pub read_irq_name: *const c_char,
    pub write_irq_name: *const c_char,
    pub mc: mc_device,
    pub driver: *mut tty_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line {
    pub port: tty_port,
    pub valid: c_int,
    pub write_irq: int read_irq,,
    pub init_str: *mut c_char,
    pub chan_list: list_head,
    pub chan_out: *mut *mut chan chan_in,,
// This lock is actually, mostly, local to
    pub lock: spinlock_t,
    pub throttled: c_int,
// Yes, this is a real circular buffer.
// XXX: And this should become a struct kfifo!
//
// buffer points to a buffer allocated on demand, of length
// LINE_BUFSIZE, head to the start of the ring, tail to the end.
    pub buffer: *mut u8,
    pub head: *mut u8,
    pub tail: *mut u8,
    pub sigio: c_int,
    pub task: delayed_work,
    pub driver: *const line_driver,
}

extern "C" {
    pub fn line_close(tty: *mut tty_struct, filp: *mut *mut file);
}
extern "C" {
    pub fn line_open(tty: *mut tty_struct, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn line_cleanup(tty: *mut tty_struct);
}
extern "C" {
    pub fn line_hangup(tty: *mut tty_struct);
}
extern "C" {
    pub fn line_write(tty: *mut tty_struct, buf: *const u8, len: usize) -> isize;
}
extern "C" {
    pub fn line_chars_in_buffer(tty: *mut tty_struct) -> c_uint;
}
extern "C" {
    pub fn line_flush_buffer(tty: *mut tty_struct);
}
extern "C" {
    pub fn line_flush_chars(tty: *mut tty_struct);
}
extern "C" {
    pub fn line_write_room(tty: *mut tty_struct) -> c_uint;
}
extern "C" {
    pub fn line_throttle(tty: *mut tty_struct);
}
extern "C" {
    pub fn line_unthrottle(tty: *mut tty_struct);
}
extern "C" {
    pub fn line_close_chan(line: *mut line);
}
extern "C" {
    pub fn close_lines(lines: *mut line, nlines: c_int);
}
extern "C" {
    pub fn line_id(str: *mut c_char, start_out: *mut c_int, end_out: *mut c_int) -> c_int;
}
