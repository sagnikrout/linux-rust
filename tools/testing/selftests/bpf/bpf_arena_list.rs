//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bpf_arena_list.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

pub type arena_list_node_t = arena_list_node __arena;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_list_node {
    pub next: *mut arena_list_node_t,
    pub pprev: *mut *mut arena_list_node_t  __arena,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_list_head {
    pub first: *mut arena_list_node __arena,
}

pub type arena_list_head_t = arena_list_head __arena;

// Safely walk link list elements. Deletion of elements is allowed.

pub const POISON_POINTER_DELTA: c_int = 0;

