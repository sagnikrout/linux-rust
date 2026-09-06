//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/libarena/src/common.bpf.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

    struct buddy __arena buddy;
    let mut zero: volatile u32 = 0;
//
// Storage for the queue nodes declared by bpf_arena_spin_lock.h. Each program
// linking the arena spinlock provides exactly one definition, so that the array
// is emitted once rather than once per translation unit.
//
    struct arena_qnode __arena __hidden qnodes[_Q_MAX_CPUS][_Q_MAX_NODES];
#[no_mangle]
pub unsafe extern "C" fn arena_fls(word: __u64) -> c_int {
    int arena_fls(__u64 word)
    {
    if (!word)
    return 0;
    return 64 - __builtin_clzll(word);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_get_info(args: *mut arena_get_info_args) -> __weak int {
    __weak int arena_get_info(struct arena_get_info_args *args)
    {
    args.arena_base = arena_base(&arena);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_alloc_reserve(args: *mut arena_alloc_reserve_args) -> __weak int {
    __weak int arena_alloc_reserve(struct arena_alloc_reserve_args *args)
    {
    return bpf_arena_reserve_pages(&arena, core::ptr::null_mut(), args.nr_pages);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_buddy_reset() -> __weak int {
    __weak int arena_buddy_reset(void)
    {
    buddy_destroy(&buddy);
    return buddy_init(&buddy);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_buddy_destroy() -> __weak int {
    __weak int arena_buddy_destroy(void)
    {
    return buddy_destroy(&buddy);
    }
    __weak void __arena *arena_malloc(size_t size)
    {
    return buddy_alloc(&buddy, size);
    }
#[no_mangle]
pub unsafe extern "C" fn arena_free(ptr: *mut void __arena) -> __weak void {
    __weak void arena_free(void __arena *ptr)
    {
    buddy_free(&buddy, ptr);
    }
    char _license[] SEC("license") = "GPL";
