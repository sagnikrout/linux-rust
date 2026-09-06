//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_btf_decl_tag.c
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
// Copyright (c) 2021 Facebook

    let mut __tag2: volatile bool skip_tests __tag1 = false;

// Macro flag: #define __tag1
// Macro flag: #define __tag2
    let mut skip_tests: volatile bool = true;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_t {
    pub a: c_int,
    pub __tag2: int b __tag1,
    pub c: c_int,
    pub __tag2: } __tag1,
    typedef struct {
    pub a: c_int,
    pub b: c_int,
    pub __tag2: } value_t __tag1,
    struct {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub 3): __uint(max_entries,,
    pub key_t): __type(key, struct,
    pub value_t): __type(value,,
    pub SEC(".maps"): } hashmap1,
#[no_mangle]
unsafe extern "C" fn foo(__tag2: int x __tag1) -> __noinline __tag1 __tag2 int {
    static __noinline __tag1 __tag2 int foo(int x __tag1 __tag2)
    {
    pub key: key_t,
    pub {}: value_t val =,
    pub x: key.a = key.b = key.c =,
    pub 0): bpf_map_update_elem(&hashmap1, &key, &val,,
    pub 0: return,
    }
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sub, x: c_int) -> c_int {
    int BPF_PROG(sub, int x)
    {
    pub foo(x): return,
    }
