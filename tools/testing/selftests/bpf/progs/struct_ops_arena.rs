//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_arena.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
// page 0 hosts the arena globals, page 1 is for allocations
    __uint(max_entries, 2);
    } arena SEC(".maps");
// also associates the callbacks with the arena
    u64 __arena arena_touch;
// raw value of the last __arena ctx argument, captured by test_arena_cb
    u64 __arena cb_ptr_val;
    SEC("struct_ops/test_arena")
#[no_mangle]
pub unsafe extern "C" fn test_arena_cb(ctx: *mut c_ulonglong) -> c_int {
    int test_arena_cb(unsigned long long *ctx)
    {
    u64 __arena *ptr = (u64 __arena *)ctx[0];
    arena_touch++;
    cb_ptr_val = ctx[0];
// ptr += 1;
    return 0;
    }
    SEC("struct_ops/test_arena_nullable")
#[no_mangle]
pub unsafe extern "C" fn test_arena_nullable_cb(ctx: *mut c_ulonglong) -> c_int {
    int test_arena_nullable_cb(unsigned long long *ctx)
    {
    u64 __arena *ptr = (u64 __arena *)ctx[0];
    arena_touch++;
    if (!ptr)
    return 0xbee;
// ptr += 1;
    return 0;
    }
    SEC("struct_ops/test_arena_stack")
#[no_mangle]
pub unsafe extern "C" fn test_arena_stack_cb(ctx: *mut c_ulonglong) -> c_int {
    int test_arena_stack_cb(unsigned long long *ctx)
    {
    u64 __arena *ptr = (u64 __arena *)ctx[8];
    arena_touch++;
// pin the slot layout: the leading args fill ctx[0]..ctx[7]
    if (ctx[0] != 1 || ctx[7] != 8)
    return 0xbad;
// ptr += 1;
    return 0;
    }
    SEC("struct_ops/test_arena_multislot")
#[no_mangle]
pub unsafe extern "C" fn test_arena_multislot_cb(ctx: *mut c_ulonglong) -> c_int {
    int test_arena_multislot_cb(unsigned long long *ctx)
    {
    u64 __arena *ptr = (u64 __arena *)ctx[2];
    arena_touch++;
//
// The 16-byte struct occupies ctx[0] and ctx[1], so @ptr is argument
// one but slot two. Getting that wrong hands the callback a scalar.
//
    if (ctx[0] != 11 || ctx[1] != 22)
    return 0xbad;
// ptr += 1;
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops3 testmod_arena = {
    .test_arena = (void *)test_arena_cb,
    .test_arena_nullable = (void *)test_arena_nullable_cb,
    .test_arena_stack = (void *)test_arena_stack_cb,
    .test_arena_multislot = (void *)test_arena_multislot_cb,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn trigger(ctx: *mut c_void) -> c_int {
    int trigger(void *ctx)
    {

    u64 __arena *val;
    int ret;
    val = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!val)
    return 1;
// val = 41;
    ret = bpf_testmod_ops3_call_test_arena((u64 *)val);
    if (ret)
    return 2;
    if (*val != 42)
    return 3;
//
// The callback must have seen exactly (u32)(kaddr - kern_vm_start),
// which is the arena offset of val with the upper 32 bits clear.
//
    if (cb_ptr_val != (u32)(u64)val)
    return 4;
    ret = bpf_testmod_ops3_call_test_arena_nullable((u64 *)val);
    if (ret)
    return 5;
    if (*val != 43)
    return 6;
// NULL survives the nullable kfunc and the trampoline as NULL
    ret = bpf_testmod_ops3_call_test_arena_nullable(core::ptr::null_mut());
    if (ret != 0xbee)
    return 7;
// the arena pointer is stack-passed into the trampoline here
    ret = bpf_testmod_ops3_call_test_arena_stack((u64 *)val);
    if (ret)
    return 8;
    if (*val != 44)
    return 9;
// a multi-slot arg precedes the arena pointer here
    ret = bpf_testmod_ops3_call_test_arena_multislot((u64 *)val);
    if (ret)
    return 10;
    if (*val != 45)
    return 11;
    bpf_arena_free_pages(&arena, (void __arena *)val, 1);

    return 0;
    }
