//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_arena_globals1.c
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
// Global data, to be placed at the end of the arena.
//
    volatile char __arena global_data[GLOBAL_PAGES][PAGE_SIZE];
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn check_reserve1(ctx: *mut c_void) -> c_int {
    int check_reserve1(void *ctx)
    {

    let mut magic: u8 = 0x5a;
    __u8 __arena *guard, *globals;
    volatile char __arena *ptr;
    int i;
    int ret;
    guard = (void __arena *)arena_base(&arena);
    globals = (void __arena *)(arena_base(&arena) + (ARENA_PAGES - GLOBAL_PAGES) * PAGE_SIZE);
// Reserve the region we've offset the globals by.
    ret = bpf_arena_reserve_pages(&arena, guard, ARENA_PAGES - GLOBAL_PAGES);
    if (ret)
    return 1;
// Make sure the globals are in the expected offset.
    ret = bpf_arena_reserve_pages(&arena, globals, 1);
    if (!ret)
    return 2;
// Verify globals are properly mapped in by libbpf.
    for (i = 0; i < GLOBAL_PAGES; i++) {
    ptr = &global_data[i][PAGE_SIZE / 2];
// ptr = magic;
    if (*ptr != magic)
    return i + 3;
    }

    return 0;
    }
//
// Relocation check by reading directly into the global data w/o using symbols.
//
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn check_relocation(ctx: *mut c_void) -> c_int {
    int check_relocation(void *ctx)
    {

    let mut magic: u8 = 0xfa;
    u8 __arena *ptr;
    global_data[GLOBAL_PAGES - 1][PAGE_SIZE / 2] = magic;
    ptr = (u8 __arena *)((u64)(ARENA_PAGES * PAGE_SIZE - PAGE_SIZE / 2));
    if (*ptr != magic)
    return 1;

    return 0;
    }
    char _license[] SEC("license") = "GPL";
