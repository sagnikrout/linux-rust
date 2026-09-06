//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/arena_kfunc.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
// page 0 hosts the arena global, page 1 is for allocations
    __uint(max_entries, 2);
    } arena SEC(".maps");
//
// Occupies page 0 so no allocation lands at arena offset 0, which the
// nullable tests below must be able to tell apart from NULL.
//
    u64 __arena arena_pad;
// volatile to force the scalar reloads below
    volatile u64 stash;
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_arg_forms(ctx: *mut c_void) -> c_int {
    int arena_arg_forms(void *ctx)
    {

    u64 __arena *val;
    u64 ret;
    val = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!val)
    return 1;
// PTR_TO_ARENA argument
// val = 41;
    ret = bpf_kfunc_arena_arg_test((u64 *)val);
    if (ret != 41 || *val != 42)
    return 2;
// the low 32 bits as a scalar
    stash = (u32)(u64)val;
    ret = bpf_kfunc_arena_arg_test((u64 *)stash);
    if (ret != 42 || *val != 43)
    return 3;
// the full user address as a scalar
    stash = (u64)val;
    bpf_addr_space_cast(stash, 1, 0);
    ret = bpf_kfunc_arena_arg_test((u64 *)stash);
    if (ret != 43 || *val != 44)
    return 4;
    bpf_arena_free_pages(&arena, (void __arena *)val, 1);

    return 0;
    }
//
// Pin the rebase semantics using the capture kfuncs, which return the raw
// argument value: __arena rebases unconditionally, so zero low 32 bits
// arrive as the arena kernel base, while __arena__nullable turns them into
// NULL.
//
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_arg_rebase(ctx: *mut c_void) -> c_int {
    int arena_arg_rebase(void *ctx)
    {

    u64 __arena *val;
    u64 base, off;
    val = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!val)
    return 1;
    base = bpf_kfunc_arena_cap_test(core::ptr::null_mut());
    if (!base)
    return 2;
// only the low 32 bits contribute
    stash = 0xbadc0ffe00000000;
    if (bpf_kfunc_arena_cap_test((u64 *)stash) != base)
    return 3;
    off = (u32)(u64)val;
    if (bpf_kfunc_arena_cap_test((u64 *)val) != base + off)
    return 4;
    if (bpf_kfunc_arena_cap_nullable_test(core::ptr::null_mut()) != 0)
    return 5;
    stash = 0xbadc0ffe00000000;
    if (bpf_kfunc_arena_cap_nullable_test((u64 *)stash) != 0)
    return 6;
    if (bpf_kfunc_arena_cap_nullable_test((u64 *)val) != base + off)
    return 7;
    bpf_arena_free_pages(&arena, (void __arena *)val, 1);

    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_args5(ctx: *mut c_void) -> c_int {
    int arena_args5(void *ctx)
    {

    u64 __arena *val;
    val = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!val)
    return 1;
    val[0] = 1;
    val[1] = 2;
    val[2] = 4;
    val[3] = 8;
    val[4] = 16;
    if (bpf_kfunc_arena_args5_test((u64 *)&val[0], (u64 *)&val[1],
    (u64 *)&val[2], (u64 *)&val[3],
    (u64 *)&val[4]) != 31)
    return 2;
    if (bpf_kfunc_arena_args5_test((u64 *)&val[0], (u64 *)&val[1],
    (u64 *)&val[2], (u64 *)&val[3], core::ptr::null_mut()) != 15)
    return 3;
    bpf_arena_free_pages(&arena, (void __arena *)val, 1);

    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_arg_mixed(ctx: *mut c_void) -> c_int {
    int arena_arg_mixed(void *ctx)
    {

    u64 __arena *val;
    val = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!val)
    return 1;
    val[0] = 7;
    val[1] = 5;
    if (bpf_kfunc_arena_mixed_test((u64 *)&val[0], core::ptr::null_mut()) != 7)
    return 2;
    if (bpf_kfunc_arena_mixed_test((u64 *)&val[0], (u64 *)&val[1]) != 12)
    return 3;
    bpf_arena_free_pages(&arena, (void __arena *)val, 1);

    return 0;
    }
// kernel-side faults on unpopulated pages recover via the scratch page
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_arg_unpopulated(ctx: *mut c_void) -> c_int {
    int arena_arg_unpopulated(void *ctx)
    {

    u64 __arena *val;
    val = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!val)
    return 1;
    stash = (u64)val + PAGE_SIZE;
    bpf_kfunc_arena_arg_test((u64 *)stash);
    bpf_arena_free_pages(&arena, (void __arena *)val, 1);

    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __msg(arena": "arena pointer requires a program with an associated) -> __failure {
    __failure __msg("arena pointer requires a program with an associated arena")
#[no_mangle]
pub unsafe extern "C" fn arena_arg_no_arena(ctx: *mut c_void) -> c_int {
    int arena_arg_no_arena(void *ctx)
    {
    bpf_kfunc_arena_arg_test((u64 *)1);
    return 0;
    }
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar": "is not a pointer to arena or) -> __failure {
    __failure __msg("is not a pointer to arena or scalar")
#[no_mangle]
pub unsafe extern "C" fn arena_arg_bad_reg(ctx: *mut c_void) -> c_int {
    int arena_arg_bad_reg(void *ctx)
    {
    let mut buf: u64 = 0;
// use the arena so the program passes the arena presence check
    bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    bpf_kfunc_arena_arg_test(&buf);
    return 0;
    }

    defined(__BPF_FEATURE_STACK_ARGUMENT)
    SEC("syscall")
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn __msg(argument": "arena pointer cannot be a stack) -> __failure {
    __failure __msg("arena pointer cannot be a stack argument")
#[no_mangle]
pub unsafe extern "C" fn arena_arg_stack(ctx: *mut c_void) -> c_int {
    int arena_arg_stack(void *ctx)
    {
    bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    bpf_kfunc_arena_stack_arg_test(1, 2, 3, 4, 5, (u64 *)1);
    return 0;
    }

    SEC("syscall")
    __arch_x86_64
    __arch_arm64
    __description("arena_arg_stack: not supported, dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn arena_arg_stack(ctx: *mut c_void) -> c_int {
    int arena_arg_stack(void *ctx)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
