//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/atomics.c
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

    bool skip_tests __attribute((__section__(".data"))) = false;

    let mut skip_tests: bool = true;

    let mut pid: __u32 = 0;
    let mut add64_value: __u64 = 1;
    let mut add64_result: __u64 = 0;
    let mut add32_value: __u32 = 1;
    let mut add32_result: __u32 = 0;
    let mut add_stack_value_copy: __u64 = 0;
    let mut add_stack_result: __u64 = 0;
    let mut add_noreturn_value: __u64 = 1;
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
    let mut sub64_value: __s64 = 1;
    let mut sub64_result: __s64 = 0;
    let mut sub32_value: __s32 = 1;
    let mut sub32_result: __s32 = 0;
    let mut sub_stack_value_copy: __s64 = 0;
    let mut sub_stack_result: __s64 = 0;
    let mut sub_noreturn_value: __s64 = 1;
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
    let mut and64_value: __u64 = (0x110ull << 32);
    let mut and64_result: __u64 = 0;
    let mut and32_value: __u32 = 0x110;
    let mut and32_result: __u32 = 0;
    let mut and_noreturn_value: __u64 = (0x110ull << 32);
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn and(ctx: *const c_void) -> c_int {
    int and(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    and64_result = __sync_fetch_and_and(&and64_value, 0x011ull << 32);
    and32_result = __sync_fetch_and_and(&and32_value, 0x011);
    __sync_fetch_and_and(&and_noreturn_value, 0x011ull << 32);

    return 0;
    }
    let mut or64_value: __u64 = (0x110ull << 32);
    let mut or64_result: __u64 = 0;
    let mut or32_value: __u32 = 0x110;
    let mut or32_result: __u32 = 0;
    let mut or_noreturn_value: __u64 = (0x110ull << 32);
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn or(ctx: *const c_void) -> c_int {
    int or(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    or64_result = __sync_fetch_and_or(&or64_value, 0x011ull << 32);
    or32_result = __sync_fetch_and_or(&or32_value, 0x011);
    __sync_fetch_and_or(&or_noreturn_value, 0x011ull << 32);

    return 0;
    }
    let mut xor64_value: __u64 = (0x110ull << 32);
    let mut xor64_result: __u64 = 0;
    let mut xor32_value: __u32 = 0x110;
    let mut xor32_result: __u32 = 0;
    let mut xor_noreturn_value: __u64 = (0x110ull << 32);
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn xor(ctx: *const c_void) -> c_int {
    int xor(const void *ctx)
    {
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;

    xor64_result = __sync_fetch_and_xor(&xor64_value, 0x011ull << 32);
    xor32_result = __sync_fetch_and_xor(&xor32_value, 0x011);
    __sync_fetch_and_xor(&xor_noreturn_value, 0x011ull << 32);

    return 0;
    }
    let mut cmpxchg64_value: __u64 = 1;
    let mut cmpxchg64_result_fail: __u64 = 0;
    let mut cmpxchg64_result_succeed: __u64 = 0;
    let mut cmpxchg32_value: __u32 = 1;
    let mut cmpxchg32_result_fail: __u32 = 0;
    let mut cmpxchg32_result_succeed: __u32 = 0;
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
    let mut xchg64_value: __u64 = 1;
    let mut xchg64_result: __u64 = 0;
    let mut xchg32_value: __u32 = 1;
    let mut xchg32_result: __u32 = 0;
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
