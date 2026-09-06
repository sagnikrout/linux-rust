//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/chan.h
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
// Copyright (C) 2000, 2001 Jeff Dike (jdike@karaya.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan {
    pub list: list_head,
    pub free_list: list_head,
    pub line: *mut line,
    pub dev: *mut c_char,
    pub primary:1: c_uint,
    pub input:1: c_uint,
    pub output:1: c_uint,
    pub opened:1: c_uint,
    pub enabled:1: c_uint,
    pub fd_in: c_int,
    pub /: *mut *mut int fd_out; / only different to fd_in if blocking output is needed,
    pub ops: *const chan_ops,
    pub data: *mut c_void,
}

extern "C" {
    pub fn chan_interrupt(line: *mut line, irq: c_int);
}
extern "C" {
    pub fn console_open_chan(line: *mut line, co: *mut console) -> c_int;
}
extern "C" {
    pub fn deactivate_chan(chan: *mut chan, irq: c_int);
}
extern "C" {
    pub fn chan_enable_winch(chan: *mut chan, port: *mut tty_port);
}
extern "C" {
    pub fn enable_chan(line: *mut line) -> c_int;
}
extern "C" {
    pub fn close_chan(line: *mut line);
}
