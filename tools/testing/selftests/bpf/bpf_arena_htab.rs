//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bpf_arena_htab.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab_bucket {
    pub head: arena_list_head,
}

pub type htab_bucket_t = htab_bucket __arena;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htab {
    pub buckets: *mut htab_bucket_t,
    pub n_buckets: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashtab_elem {
    pub hash: c_int,
    pub key: c_int,
    pub value: c_int,
    pub hash_node: arena_list_node,
}

pub type hashtab_elem_t = hashtab_elem __arena;
