//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/arena_atomics.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, 10); /* number of pages */

    __ulong(map_extra, 0x1ull << 32); /* start of mmap() region */

    __ulong(map_extra, 0x1ull << 44); /* start of mmap() region */

    } arena SEC(".maps");

    bool skip_all_tests __attribute((__section__(".data"))) = false;

    let mut skip_all_tests: bool = true;

    defined(__BPF_FEATURE_ADDR_SPACE_CAST) && \
    (defined(__TARGET_ARCH_arm64) || \
    defined(__TARGET_ARCH_x86) || \
    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
    defined(__TARGET_ARCH_s390))
    bool skip_lacq_srel_tests __attribute((__section__(".data"))) = false;

    let mut skip_lacq_srel_tests: bool = true;

    let mut pid: __u32 = 0;
    let mut add64_value: __u64 __arena_global = 1;
    let mut add64_result: __u64 __arena_global = 0;
    let mut add32_value: __u32 __arena_global = 1;
    let mut add32_result: __u32 __arena_global = 0;
    let mut add_stack_value_copy: __u64 __arena_global = 0;
    let mut add_stack_result: __u64 __arena_global = 0;
    let mut add_noreturn_value: __u64 __arena_global = 1;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn add(ctx: *const c_void) -> c_int {
    int add(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    let mut add_stack_value: __u64 = 1;
    add64_result = __sync_fetch_and_add(&add64_value, 2);
    add32_result = __sync_fetch_and_add(&add32_value, 2);
    add_stack_result = __sync_fetch_and_add(&add_stack_value, 2);
    add_stack_value_copy = add_stack_value;
    __sync_fetch_and_add(&add_noreturn_value, 2);

    return 0;
    }
    let mut sub64_value: __s64 __arena_global = 1;
    let mut sub64_result: __s64 __arena_global = 0;
    let mut sub32_value: __s32 __arena_global = 1;
    let mut sub32_result: __s32 __arena_global = 0;
    let mut sub_stack_value_copy: __s64 __arena_global = 0;
    let mut sub_stack_result: __s64 __arena_global = 0;
    let mut sub_noreturn_value: __s64 __arena_global = 1;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn sub(ctx: *const c_void) -> c_int {
    int sub(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    let mut sub_stack_value: __u64 = 1;
    sub64_result = __sync_fetch_and_sub(&sub64_value, 2);
    sub32_result = __sync_fetch_and_sub(&sub32_value, 2);
    sub_stack_result = __sync_fetch_and_sub(&sub_stack_value, 2);
    sub_stack_value_copy = sub_stack_value;
    __sync_fetch_and_sub(&sub_noreturn_value, 2);

    return 0;
    }

    let mut and64_value: _Atomic __u64 __arena_global = (0x110ull << 32);
    let mut and32_value: _Atomic __u32 __arena_global = 0x110;

    let mut and64_value: __u64 __arena_global = (0x110ull << 32);
    let mut and32_value: __u32 __arena_global = 0x110;

    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn and(ctx: *const c_void) -> c_int {
    int and(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    __c11_atomic_fetch_and(&and64_value, 0x011ull << 32, memory_order_relaxed);
    __c11_atomic_fetch_and(&and32_value, 0x011, memory_order_relaxed);

    __sync_fetch_and_and(&and64_value, 0x011ull << 32);
    __sync_fetch_and_and(&and32_value, 0x011);

    return 0;
    }

    let mut or32_value: _Atomic __u32 __arena_global = 0x110;
    let mut or64_value: _Atomic __u64 __arena_global = (0x110ull << 32);

    let mut or32_value: __u32 __arena_global = 0x110;
    let mut or64_value: __u64 __arena_global = (0x110ull << 32);

    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn or(ctx: *const c_void) -> c_int {
    int or(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    __c11_atomic_fetch_or(&or64_value, 0x011ull << 32, memory_order_relaxed);
    __c11_atomic_fetch_or(&or32_value, 0x011, memory_order_relaxed);

    __sync_fetch_and_or(&or64_value, 0x011ull << 32);
    __sync_fetch_and_or(&or32_value, 0x011);

    return 0;
    }

    let mut xor64_value: _Atomic __u64 __arena_global = (0x110ull << 32);
    let mut xor32_value: _Atomic __u32 __arena_global = 0x110;

    let mut xor64_value: __u64 __arena_global = (0x110ull << 32);
    let mut xor32_value: __u32 __arena_global = 0x110;

    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn xor(ctx: *const c_void) -> c_int {
    int xor(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    __c11_atomic_fetch_xor(&xor64_value, 0x011ull << 32, memory_order_relaxed);
    __c11_atomic_fetch_xor(&xor32_value, 0x011, memory_order_relaxed);

    __sync_fetch_and_xor(&xor64_value, 0x011ull << 32);
    __sync_fetch_and_xor(&xor32_value, 0x011);

    return 0;
    }
    let mut cmpxchg32_value: __u32 __arena_global = 1;
    let mut cmpxchg32_result_fail: __u32 __arena_global = 0;
    let mut cmpxchg32_result_succeed: __u32 __arena_global = 0;
    let mut cmpxchg64_value: __u64 __arena_global = 1;
    let mut cmpxchg64_result_fail: __u64 __arena_global = 0;
    let mut cmpxchg64_result_succeed: __u64 __arena_global = 0;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn cmpxchg(ctx: *const c_void) -> c_int {
    int cmpxchg(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    cmpxchg64_result_fail = __sync_val_compare_and_swap(&cmpxchg64_value, 0, 3);
    cmpxchg64_result_succeed = __sync_val_compare_and_swap(&cmpxchg64_value, 1, 2);
    cmpxchg32_result_fail = __sync_val_compare_and_swap(&cmpxchg32_value, 0, 3);
    cmpxchg32_result_succeed = __sync_val_compare_and_swap(&cmpxchg32_value, 1, 2);

    return 0;
    }
    let mut xchg64_value: __u64 __arena_global = 1;
    let mut xchg64_result: __u64 __arena_global = 0;
    let mut xchg32_value: __u32 __arena_global = 1;
    let mut xchg32_result: __u32 __arena_global = 0;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn xchg(ctx: *const c_void) -> c_int {
    int xchg(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    let mut val64: __u64 = 2;
    let mut val32: __u32 = 2;
    xchg64_result = __sync_lock_test_and_set(&xchg64_value, val64);
    xchg32_result = __sync_lock_test_and_set(&xchg32_value, val32);

    return 0;
    }
    __u64 __arena_global uaf_sink;
    volatile __u64 __arena_global uaf_recovery_fails;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn uaf(ctx: *const c_void) -> c_int {
    int uaf(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    !defined(__TARGET_ARCH_x86)
    __u32 __arena *page32;
    __u64 __arena *page64;
    void __arena *page;
    page = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    bpf_arena_free_pages(&arena, page, 1);
    uaf_recovery_fails = 24;
    page32 = (__u32 __arena *)page;
    uaf_sink += __sync_fetch_and_add(page32, 1);
    uaf_recovery_fails -= 1;
    __sync_add_and_fetch(page32, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_sub(page32, 1);
    uaf_recovery_fails -= 1;
    __sync_sub_and_fetch(page32, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_and(page32, 1);
    uaf_recovery_fails -= 1;
    __sync_and_and_fetch(page32, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_or(page32, 1);
    uaf_recovery_fails -= 1;
    __sync_or_and_fetch(page32, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_xor(page32, 1);
    uaf_recovery_fails -= 1;
    __sync_xor_and_fetch(page32, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_val_compare_and_swap(page32, 0, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_lock_test_and_set(page32, 1);
    uaf_recovery_fails -= 1;
    page64 = (__u64 __arena *)page;
    uaf_sink += __sync_fetch_and_add(page64, 1);
    uaf_recovery_fails -= 1;
    __sync_add_and_fetch(page64, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_sub(page64, 1);
    uaf_recovery_fails -= 1;
    __sync_sub_and_fetch(page64, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_and(page64, 1);
    uaf_recovery_fails -= 1;
    __sync_and_and_fetch(page64, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_or(page64, 1);
    uaf_recovery_fails -= 1;
    __sync_or_and_fetch(page64, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_fetch_and_xor(page64, 1);
    uaf_recovery_fails -= 1;
    __sync_xor_and_fetch(page64, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_val_compare_and_swap(page64, 0, 1);
    uaf_recovery_fails -= 1;
    uaf_sink += __sync_lock_test_and_set(page64, 1);
    uaf_recovery_fails -= 1;

    return 0;
    }

    let mut load_acquire8_value: __u8 __arena_global = 0x12;
    let mut load_acquire16_value: __u16 __arena_global = 0x1234;
    let mut load_acquire32_value: __u32 __arena_global = 0x12345678;
    let mut load_acquire64_value: __u64 __arena_global = 0x1234567890abcdef;
    let mut load_acquire8_result: __u8 __arena_global = 0;
    let mut load_acquire16_result: __u16 __arena_global = 0;
    let mut load_acquire32_result: __u32 __arena_global = 0;
    let mut load_acquire64_result: __u64 __arena_global = 0;

// clang-17 crashes if the .addr_space.1 ELF section has holes. Work around
// this issue by defining the below variables as 64-bit.
//
    __u64 __arena_global load_acquire8_value;
    __u64 __arena_global load_acquire16_value;
    __u64 __arena_global load_acquire32_value;
    __u64 __arena_global load_acquire64_value;
    __u64 __arena_global load_acquire8_result;
    __u64 __arena_global load_acquire16_result;
    __u64 __arena_global load_acquire32_result;
    __u64 __arena_global load_acquire64_result;

    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn load_acquire(ctx: *const c_void) -> c_int {
    int load_acquire(const void *ctx)
    {

    defined(__BPF_FEATURE_ADDR_SPACE_CAST) && \
    (defined(__TARGET_ARCH_arm64) || \
    defined(__TARGET_ARCH_x86) || \
    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
    defined(__TARGET_ARCH_s390))

    { asm volatile (				\
    "r1 = %[" #SRC "] ll;"				\
    "r1 = addr_space_cast(r1, 0x0, 0x1);"		\
    ".8byte %[load_acquire_insn];"			\
    "r3 = %[" #DST "] ll;"				\
    "r3 = addr_space_cast(r3, 0x0, 0x1);"		\
    "*(" #SIZE " *)(r3 + 0) = r2;"			\
    :						\
    : __imm_addr(SRC),				\
    __imm_insn(load_acquire_insn,			\
    BPF_ATOMIC_OP(BPF_##SIZEOP, BPF_LOAD_ACQ,	\
    BPF_REG_2, BPF_REG_1, 0)),	\
    __imm_addr(DST)				\
    : __clobber_all); }				\
    LOAD_ACQUIRE_ARENA(B, u8, load_acquire8_value, load_acquire8_result)
    LOAD_ACQUIRE_ARENA(H, u16, load_acquire16_value,
    load_acquire16_result)
    LOAD_ACQUIRE_ARENA(W, u32, load_acquire32_value,
    load_acquire32_result)
    LOAD_ACQUIRE_ARENA(DW, u64, load_acquire64_value,
    load_acquire64_result)

    return 0;
    }

    let mut store_release8_result: __u8 __arena_global = 0;
    let mut store_release16_result: __u16 __arena_global = 0;
    let mut store_release32_result: __u32 __arena_global = 0;
    let mut store_release64_result: __u64 __arena_global = 0;

// clang-17 crashes if the .addr_space.1 ELF section has holes. Work around
// this issue by defining the below variables as 64-bit.
//
    __u64 __arena_global store_release8_result;
    __u64 __arena_global store_release16_result;
    __u64 __arena_global store_release32_result;
    __u64 __arena_global store_release64_result;

    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn store_release(ctx: *const c_void) -> c_int {
    int store_release(const void *ctx)
    {

    defined(__BPF_FEATURE_ADDR_SPACE_CAST) && \
    (defined(__TARGET_ARCH_arm64) || \
    defined(__TARGET_ARCH_x86) || \
    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
    defined(__TARGET_ARCH_s390))

    { asm volatile (			\
    "r1 = " VAL ";"				\
    "r2 = %[" #DST "] ll;"			\
    "r2 = addr_space_cast(r2, 0x0, 0x1);"	\
    ".8byte %[store_release_insn];"		\
    :					\
    : __imm_addr(DST),			\
    __imm_insn(store_release_insn,	\
    BPF_ATOMIC_OP(BPF_##SIZEOP, BPF_STORE_REL,	\
    BPF_REG_2, BPF_REG_1, 0))	\
    : __clobber_all); }			\
    STORE_RELEASE_ARENA(B, store_release8_result, "0x12")
    STORE_RELEASE_ARENA(H, store_release16_result, "0x1234")
    STORE_RELEASE_ARENA(W, store_release32_result, "0x12345678")
    STORE_RELEASE_ARENA(DW, store_release64_result,
    "0x1234567890abcdef ll")

    return 0;
    }
    char _license[] SEC("license") = "GPL";
