//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_arena_globals2.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, ARENA_PAGES);

    __ulong(map_extra, (1ull << 32) | (~0u - __PAGE_SIZE * ARENA_PAGES + 1));

    __ulong(map_extra, (1ull << 44) | (~0u - __PAGE_SIZE * ARENA_PAGES + 1));

    } arena SEC(".maps");
//
// Fill the entire arena with global data.
// The offset into the arena should be 0.
//
    char __arena global_data[ARENA_PAGES][PAGE_SIZE];
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn check_reserve2(ctx: *mut c_void) -> c_int {
    int check_reserve2(void *ctx)
    {

    void __arena *guard;
    int ret;
    guard = (void __arena *)arena_base(&arena);
// Make sure the data at offset 0 case is properly handled.
    ret = bpf_arena_reserve_pages(&arena, guard, 1);
    if (!ret)
    return 1;

    return 0;
    }
    char _license[] SEC("license") = "GPL";
