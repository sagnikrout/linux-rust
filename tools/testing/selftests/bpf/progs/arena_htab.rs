//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/arena_htab.c
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
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, 100); /* number of pages */
    } arena SEC(".maps");

    void __arena *htab_for_user;
    let mut skip: bool = false;
    let mut zero: c_int = 0;
    char __arena arr1[100000];
    char arr2[1000];
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_htab_llvm(ctx: *mut c_void) -> c_int {
    int arena_htab_llvm(void *ctx)
    {

    struct htab __arena *htab;
    char __arena *arr = arr1;
    __u64 i;
    htab = bpf_alloc(sizeof(*htab));
    cast_kern(htab);
    htab_init(htab);
    cast_kern(arr);
// first run. No old elems in the table
    for (i = zero; i < 100000 && can_loop; i++) {
    htab_update_elem(htab, i, i);
    arr[i] = i;
    }
// should replace some elems with new ones
    for (i = zero; i < 1000 && can_loop; i++) {
    htab_update_elem(htab, i, i);
// Access mem to make the verifier use bounded loop logic
    arr2[i] = i;
    }
    cast_user(htab);
    htab_for_user = htab;

    skip = true;

    return 0;
    }
    char _license[] SEC("license") = "GPL";
