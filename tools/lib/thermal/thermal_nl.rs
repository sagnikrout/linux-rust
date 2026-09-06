//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/thermal/thermal_nl.h
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


// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_handler {
    pub done: c_int,
    pub error: c_int,
    pub ops: *mut thermal_ops,
    pub msg: *mut nl_msg,
    pub sk_event: *mut nl_sock,
    pub sk_sampling: *mut nl_sock,
    pub sk_cmd: *mut nl_sock,
    pub cb_cmd: *mut nl_cb,
    pub cb_event: *mut nl_cb,
    pub cb_sampling: *mut nl_cb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_handler_param {
    pub th: *mut thermal_handler,
    pub arg: *mut c_void,
}

//
// Low level netlink
//
extern "C" {
    pub fn nl_thermal_connect(nl_sock: *mut nl_sock, nl_cb: *mut nl_cb) -> c_int;
}
extern "C" {
    pub fn nl_thermal_disconnect(nl_sock: *mut nl_sock, nl_cb: *mut nl_cb);
}
