//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/preempt_lock.c
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

    extern int bpf_copy_from_user_str(void *dst, u32 dst__sz, const void *unsafe_ptr__ign, u64 flags) __weak __ksym;
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn preempt_lock_missing_1(ctx: *mut __sk_buff) -> c_int {
    int preempt_lock_missing_1(struct __sk_buff *ctx)
    {
    bpf_preempt_disable();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn preempt_lock_missing_2(ctx: *mut __sk_buff) -> c_int {
    int preempt_lock_missing_2(struct __sk_buff *ctx)
    {
    bpf_preempt_disable();
    bpf_preempt_disable();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn preempt_lock_missing_3(ctx: *mut __sk_buff) -> c_int {
    int preempt_lock_missing_3(struct __sk_buff *ctx)
    {
    bpf_preempt_disable();
    bpf_preempt_disable();
    bpf_preempt_disable();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn preempt_lock_missing_3_minus_2(ctx: *mut __sk_buff) -> c_int {
    int preempt_lock_missing_3_minus_2(struct __sk_buff *ctx)
    {
    bpf_preempt_disable();
    bpf_preempt_disable();
    bpf_preempt_disable();
    bpf_preempt_enable();
    bpf_preempt_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn preempt_disable() -> __noinline void {
    static __noinline void preempt_disable(void)
    {
    bpf_preempt_disable();
    }
#[no_mangle]
unsafe extern "C" fn preempt_enable() -> __noinline void {
    static __noinline void preempt_enable(void)
    {
    bpf_preempt_enable();
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn preempt_lock_missing_1_subprog(ctx: *mut __sk_buff) -> c_int {
    int preempt_lock_missing_1_subprog(struct __sk_buff *ctx)
    {
    preempt_disable();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn preempt_lock_missing_2_subprog(ctx: *mut __sk_buff) -> c_int {
    int preempt_lock_missing_2_subprog(struct __sk_buff *ctx)
    {
    preempt_disable();
    preempt_disable();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn preempt_lock_missing_2_minus_1_subprog(ctx: *mut __sk_buff) -> c_int {
    int preempt_lock_missing_2_minus_1_subprog(struct __sk_buff *ctx)
    {
    preempt_disable();
    preempt_disable();
    preempt_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn preempt_balance_subprog() -> __noinline void {
    static __noinline void preempt_balance_subprog(void)
    {
    preempt_disable();
    preempt_enable();
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn preempt_balance(ctx: *mut __sk_buff) -> __success int {
    __success int preempt_balance(struct __sk_buff *ctx)
    {
    bpf_guard_preempt();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn preempt_balance_subprog_test(ctx: *mut __sk_buff) -> __success int {
    __success int preempt_balance_subprog_test(struct __sk_buff *ctx)
    {
    preempt_balance_subprog();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_copy_from_user#": "sleepable helper) -> __failure {
    __failure __msg("sleepable helper bpf_copy_from_user#")
#[no_mangle]
pub unsafe extern "C" fn preempt_sleepable_helper(ctx: *mut c_void) -> c_int {
    int preempt_sleepable_helper(void *ctx)
    {
    u32 data;
    bpf_preempt_disable();
    bpf_copy_from_user(&data, sizeof(data), core::ptr::null_mut());
    bpf_preempt_enable();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "kernel func bpf_copy_from_user_str is sleepable within non-preemptible) -> __failure {
    __failure __msg("kernel func bpf_copy_from_user_str is sleepable within non-preemptible region")
#[no_mangle]
pub unsafe extern "C" fn preempt_sleepable_kfunc(ctx: *mut c_void) -> c_int {
    int preempt_sleepable_kfunc(void *ctx)
    {
    u32 data;
    bpf_preempt_disable();
    bpf_copy_from_user_str(&data, sizeof(data), core::ptr::null_mut(), 0);
    bpf_preempt_enable();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn preempt_global_subprog() -> int __noinline {
    int __noinline preempt_global_subprog(void)
    {
    preempt_balance_subprog();
    return 0;
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn preempt_global_subprog_test(ctx: *mut __sk_buff) -> c_int {
    int preempt_global_subprog_test(struct __sk_buff *ctx)
    {
    preempt_disable();
    preempt_global_subprog();
    preempt_enable();
    return 0;
    }
    int __noinline
    global_subprog(int i)
    {
    if (i)
    bpf_printk("%p", &i);
    return i;
    }
    int __noinline
    global_sleepable_helper_subprog(int i)
    {
    if (i)
    bpf_copy_from_user(&i, sizeof(i), core::ptr::null_mut());
    return i;
    }
    int __noinline
    global_sleepable_kfunc_subprog(int i)
    {
    if (i)
    bpf_copy_from_user_str(&i, sizeof(i), core::ptr::null_mut(), 0);
    global_subprog(i);
    return i;
    }
    int __noinline
    global_subprog_calling_sleepable_global(int i)
    {
    if (!i)
    global_sleepable_kfunc_subprog(i);
    return i;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(function": "sleepable global) -> __failure {
    __failure __msg("sleepable global function")
#[no_mangle]
pub unsafe extern "C" fn preempt_global_sleepable_helper_subprog(ctx: *mut __sk_buff) -> c_int {
    int preempt_global_sleepable_helper_subprog(struct __sk_buff *ctx)
    {
    preempt_disable();
    if (ctx.mark)
    global_sleepable_helper_subprog(ctx.mark);
    preempt_enable();
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(function": "sleepable global) -> __failure {
    __failure __msg("sleepable global function")
#[no_mangle]
pub unsafe extern "C" fn preempt_global_sleepable_kfunc_subprog(ctx: *mut __sk_buff) -> c_int {
    int preempt_global_sleepable_kfunc_subprog(struct __sk_buff *ctx)
    {
    preempt_disable();
    if (ctx.mark)
    global_sleepable_kfunc_subprog(ctx.mark);
    preempt_enable();
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(function": "sleepable global) -> __failure {
    __failure __msg("sleepable global function")
#[no_mangle]
pub unsafe extern "C" fn preempt_global_sleepable_subprog_indirect(ctx: *mut __sk_buff) -> c_int {
    int preempt_global_sleepable_subprog_indirect(struct __sk_buff *ctx)
    {
    preempt_disable();
    if (ctx.mark)
    global_subprog_calling_sleepable_global(ctx.mark);
    preempt_enable();
    return 0;
    }
    char _license[] SEC("license") = "GPL";
