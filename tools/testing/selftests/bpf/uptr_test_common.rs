//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/uptr_test_common.h
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
pub const MAGIC_VALUE: c_uint = 0xabcd1234;
pub const PAGE_SIZE: c_int = 4096;

// Avoid fwd btf type being generated for the following struct

// Macro flag: #define __uptr
// Macro flag: #define __kptr

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_data {
    pub a: c_int,
    pub b: c_int,
    pub result: c_int,
    pub nested_result: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_udata {
    pub udata: *mut user_data __uptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct value_type {
    pub udata: *mut user_data __uptr,
    pub cgrp: *mut cgroup __kptr,
    pub nested: nested_udata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct value_lock_type {
    pub udata: *mut user_data __uptr,
    pub lock: bpf_spin_lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct large_data {
    pub one_page: [__u8; PAGE_SIZE],
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct large_uptr {
    pub udata: *mut large_data __uptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct empty_data {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct empty_uptr {
    pub udata: *mut empty_data __uptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kstruct_uptr {
    pub cgrp: *mut cgroup __uptr,
}
