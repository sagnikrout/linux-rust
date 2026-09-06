//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/pyperf.h
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
// Copyright (c) 2019 Facebook

pub const FUNCTION_NAME_LEN: c_int = 64;
pub const FILE_NAME_LEN: c_int = 128;
pub const TASK_COMM_LEN: c_int = 16;
pub type pid_t = c_int;

// read data from PyFrameObject
// read data from PyCodeObject
// read actual names into symbol
extern "C" {
    pub fn __get_frame_data(_arg: (long)frame_ptr, _arg: pidData, _arg: frame, _arg: symbol) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct process_frame_ctx {
    pub cur_cpu: c_int,
    pub symbol_counter: *mut i32,
    pub frame_ptr: *mut c_void,
    pub frame: *mut FrameData,
    pub pidData: *mut PidData,
    pub sym: *mut Symbol,
    pub event: *mut Event,
    pub done: bool,
}

// no for loop, no unrolling

// Unwind python stack

