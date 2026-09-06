//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/test_user_ringbuf.h
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
pub const TEST_OP_64: c_int = 4;
pub const TEST_OP_32: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_msg_op {
    TEST_MSG_OP_INC64,
    TEST_MSG_OP_INC32,
    TEST_MSG_OP_MUL64,
    TEST_MSG_OP_MUL32,

// Must come last.
    TEST_MSG_OP_NUM_OPS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_msg {
    pub msg_op: test_msg_op,
    pub operand_64: __s64,
    pub operand_32: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample {
    pub pid: c_int,
    pub seq: c_int,
    pub value: c_long,
    pub comm: [c_char; 16],
}
