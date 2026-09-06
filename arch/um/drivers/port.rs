//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/port.h
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
// Copyright (C) 2001 Jeff Dike (jdike@karaya.com)
//
extern "C" {
    pub fn port_wait(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn port_kern_close(d: *mut c_void);
}
extern "C" {
    pub fn port_connection(fd: c_int, socket_out: *mut c_int, pid_out: *mut c_int) -> c_int;
}
extern "C" {
    pub fn port_listen_fd(port: c_int) -> c_int;
}
extern "C" {
    pub fn port_read(fd: c_int, data: *mut c_void);
}
extern "C" {
    pub fn port_kern_free(d: *mut c_void);
}
extern "C" {
    pub fn port_rcv_fd(fd: c_int) -> c_int;
}
extern "C" {
    pub fn port_remove_dev(d: *mut c_void);
}
