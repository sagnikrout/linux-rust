//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/chan_user.h
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
pub struct chan_opts {
    pub dev): *const *const *const void (announce)(char dev_name, int,
    pub xterm_title: *mut c_char,
    pub raw: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_ops {
    pub type: *mut c_char,
    pub ): *const *const *const *const void (init)(char , int, struct chan_opts,
    pub ): *mut *mut *mut int (open)(int, int, int, void , char,
    pub ): *mut *mut void (close)(int, void,
    pub ): *mut *mut *mut int (read)(int, __u8 , void,
    pub ): *const *const *const int (write)(int, __u8 , size_t, void,
    pub int): *const *const *const int (console_write)(int, char ,,
    pub ): *mut *mut *mut *mut int (window_size)(int, void , unsigned short , unsigned short,
    pub ): *mut *mut void (free)(void,
    pub winch: c_int,
}

extern "C" {
    pub fn generic_close(fd: c_int, unused: *mut c_void);
}
extern "C" {
    pub fn generic_read(fd: c_int, c_out: *mut __u8, unused: *mut c_void) -> c_int;
}
extern "C" {
    pub fn generic_write(fd: c_int, buf: *const __u8, n: usize, unused: *mut c_void) -> c_int;
}
extern "C" {
    pub fn generic_console_write(fd: c_int, buf: *const c_char, n: c_int) -> c_int;
}
extern "C" {
    pub fn generic_free(data: *mut c_void);
}
extern "C" {
    pub fn register_winch(fd: c_int, port: *mut tty_port);
}

