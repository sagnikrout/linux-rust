//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/rpmsg/qcom_glink_native.h
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
// Copyright (c) 2016-2017, Linaro Ltd
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_glink_pipe {
    pub length: usize,
    pub glink_pipe): *mut *mut size_t (avail)(struct qcom_glink_pipe,
    pub count): unsigned int offset, size_t,
    pub count): *mut *mut *mut void (advance)(struct qcom_glink_pipe glink_pipe, size_t,
    pub dlen): *const *const void data, size_t,
    pub glink_pipe): *mut *mut void (kick)(struct qcom_glink_pipe,
}

extern "C" {
    pub fn qcom_glink_native_remove(glink: *mut qcom_glink);
}
extern "C" {
    pub fn qcom_glink_native_rx(glink: *mut qcom_glink);
}
