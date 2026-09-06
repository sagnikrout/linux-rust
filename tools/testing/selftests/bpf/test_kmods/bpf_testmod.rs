//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/test_kmods/bpf_testmod.h
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
// Copyright (c) 2020 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_test_read_ctx {
    pub buf: *mut c_char,
    pub off: loff_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_test_write_ctx {
    pub buf: *mut c_char,
    pub off: loff_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_test_writable_ctx {
    pub early_ret: bool,
    pub val: c_int,
}

// BPF iter that returns *value* *n* times in a row
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_testmod_seq {
    pub value: i64,
    pub cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops {
    pub (*test_1)(void): *mut c_int,
    pub b): *mut *mut void (test_2)(int a, int,
// Used to test nullable arguments.
    pub task): *mut *mut int (test_maybe_null)(int dummy, struct task_struct,
    pub (*unsupported_ops)(void): *mut c_int,
// Used to test ref_acquired arguments.
    pub task): *mut *mut int (test_refcounted)(int dummy, struct task_struct,
// Used to test checking of __ref arguments when it not the first argument.
    pub task2): *mut task_struct,
// Used to test returning referenced kptr.
    pub cgrp): *mut cgroup,
// The following fields are used to test shadow copies.
    pub onebyte: c_char,
    pub a: c_int,
    pub b: c_int,
    pub unsupported: },
    pub data: c_int,
// The following pointers are used to test the maps having multiple
// pages of trampolines.
//
    pub value): *mut *mut int (tramp_1)(int,
    pub value): *mut *mut int (tramp_2)(int,
    pub value): *mut *mut int (tramp_3)(int,
    pub value): *mut *mut int (tramp_4)(int,
    pub value): *mut *mut int (tramp_5)(int,
    pub value): *mut *mut int (tramp_6)(int,
    pub value): *mut *mut int (tramp_7)(int,
    pub value): *mut *mut int (tramp_8)(int,
    pub value): *mut *mut int (tramp_9)(int,
    pub value): *mut *mut int (tramp_10)(int,
    pub value): *mut *mut int (tramp_11)(int,
    pub value): *mut *mut int (tramp_12)(int,
    pub value): *mut *mut int (tramp_13)(int,
    pub value): *mut *mut int (tramp_14)(int,
    pub value): *mut *mut int (tramp_15)(int,
    pub value): *mut *mut int (tramp_16)(int,
    pub value): *mut *mut int (tramp_17)(int,
    pub value): *mut *mut int (tramp_18)(int,
    pub value): *mut *mut int (tramp_19)(int,
    pub value): *mut *mut int (tramp_20)(int,
    pub value): *mut *mut int (tramp_21)(int,
    pub value): *mut *mut int (tramp_22)(int,
    pub value): *mut *mut int (tramp_23)(int,
    pub value): *mut *mut int (tramp_24)(int,
    pub value): *mut *mut int (tramp_25)(int,
    pub value): *mut *mut int (tramp_26)(int,
    pub value): *mut *mut int (tramp_27)(int,
    pub value): *mut *mut int (tramp_28)(int,
    pub value): *mut *mut int (tramp_29)(int,
    pub value): *mut *mut int (tramp_30)(int,
    pub value): *mut *mut int (tramp_31)(int,
    pub value): *mut *mut int (tramp_32)(int,
    pub value): *mut *mut int (tramp_33)(int,
    pub value): *mut *mut int (tramp_34)(int,
    pub value): *mut *mut int (tramp_35)(int,
    pub value): *mut *mut int (tramp_36)(int,
    pub value): *mut *mut int (tramp_37)(int,
    pub value): *mut *mut int (tramp_38)(int,
    pub value): *mut *mut int (tramp_39)(int,
    pub value): *mut *mut int (tramp_40)(int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops2 {
    pub (*test_1)(void): *mut c_int,
}

// 16 bytes, so it takes two argument slots when passed by value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_arena_pair {
    pub a: u64,
    pub b: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops3 {
    pub (*test_1)(void): *mut c_int,
    pub (*test_2)(void): *mut c_int,
// Used to test arena pointer arguments.
    pub ptr): *mut *mut int (test_arena)(u64,
    pub ptr): *mut *mut int (test_arena_nullable)(u64,
// enough leading args to force @ptr onto the stack on x86 and arm64
    pub ptr): *mut u64 g, u64 h, u64,
// a multi-slot leading arg, so @ptr is not at the slot its arg index suggests
    pub ptr): *mut *mut int (test_arena_multislot)(struct bpf_testmod_arena_pair p, u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_ops_args {
    pub a: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_st_ops {
    pub args): *mut *mut int (test_prologue)(struct st_ops_args,
    pub args): *mut *mut int (test_epilogue)(struct st_ops_args,
    pub args): *mut *mut int (test_pro_epilogue)(struct st_ops_args,
    pub owner: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_multi_st_ops {
    pub args): *mut *mut int (test_1)(struct st_ops_args,
    pub node: hlist_node,
    pub id: c_int,
}
