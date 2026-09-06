//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/crypto_common.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __crypto_ctx_value {
    pub ctx: *mut *mut bpf_crypto_ctx __kptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub __crypto_ctx_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } __crypto_ctx_map,
    pub 0: u32 key =,
    pub &key): return bpf_map_lookup_elem(&__crypto_ctx_map,,
    pub v: *mut __crypto_ctx_value local,,
    pub old: *mut bpf_crypto_ctx,
    pub 0: u32 key =,
    pub err: c_int,
    pub NULL: local.ctx =,
    pub 0): err = bpf_map_update_elem(&__crypto_ctx_map, &key, &local,,
    pub err: return,
    pub &key): v = bpf_map_lookup_elem(&__crypto_ctx_map,,
    pub -ENOENT: return,
    pub ctx): old = bpf_kptr_xchg(&v->ctx,,
    pub -EEXIST: return,
    pub 0: return,
