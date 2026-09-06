//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kprobe_multi.c
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

    char _license[] SEC("license") = "GPL";
    extern const void bpf_fentry_test1 __ksym;
    extern const void bpf_fentry_test2 __ksym;
    extern const void bpf_fentry_test3 __ksym;
    extern const void bpf_fentry_test4 __ksym;
    extern const void bpf_fentry_test5 __ksym;
    extern const void bpf_fentry_test6 __ksym;
    extern const void bpf_fentry_test7 __ksym;
    extern const void bpf_fentry_test8 __ksym;
    let mut pid: c_int = 0;
    let mut test_cookie: bool = false;
    let mut kprobe_test1_result: __u64 = 0;
    let mut kprobe_test2_result: __u64 = 0;
    let mut kprobe_test3_result: __u64 = 0;
    let mut kprobe_test4_result: __u64 = 0;
    let mut kprobe_test5_result: __u64 = 0;
    let mut kprobe_test6_result: __u64 = 0;
    let mut kprobe_test7_result: __u64 = 0;
    let mut kprobe_test8_result: __u64 = 0;
    let mut kretprobe_test1_result: __u64 = 0;
    let mut kretprobe_test2_result: __u64 = 0;
    let mut kretprobe_test3_result: __u64 = 0;
    let mut kretprobe_test4_result: __u64 = 0;
    let mut kretprobe_test5_result: __u64 = 0;
    let mut kretprobe_test6_result: __u64 = 0;
    let mut kretprobe_test7_result: __u64 = 0;
    let mut kretprobe_test8_result: __u64 = 0;
#[no_mangle]
unsafe extern "C" fn kprobe_multi_check(ctx: *mut c_void, is_return: bool) {
    static void kprobe_multi_check(void *ctx, bool is_return)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return;
    let mut cookie: __u64 = test_cookie ? bpf_get_attach_cookie(ctx) : 0;
    let mut addr: __u64 = bpf_get_func_ip(ctx);

    if (((const void *) addr == __addr) &&		\
    (!test_cookie || (cookie == __cookie)))	\
    __var = 1;				\
    })
    if (is_return) {
    SET(kretprobe_test1_result, &bpf_fentry_test1, 8);
    SET(kretprobe_test2_result, &bpf_fentry_test2, 2);
    SET(kretprobe_test3_result, &bpf_fentry_test3, 7);
    SET(kretprobe_test4_result, &bpf_fentry_test4, 6);
    SET(kretprobe_test5_result, &bpf_fentry_test5, 5);
    SET(kretprobe_test6_result, &bpf_fentry_test6, 4);
    SET(kretprobe_test7_result, &bpf_fentry_test7, 3);
    SET(kretprobe_test8_result, &bpf_fentry_test8, 1);
    } else {
    SET(kprobe_test1_result, &bpf_fentry_test1, 1);
    SET(kprobe_test2_result, &bpf_fentry_test2, 7);
    SET(kprobe_test3_result, &bpf_fentry_test3, 2);
    SET(kprobe_test4_result, &bpf_fentry_test4, 3);
    SET(kprobe_test5_result, &bpf_fentry_test5, 4);
    SET(kprobe_test6_result, &bpf_fentry_test6, 5);
    SET(kprobe_test7_result, &bpf_fentry_test7, 6);
    SET(kprobe_test8_result, &bpf_fentry_test8, 8);
    }

    }
//
// No tests in here, just to trigger 'bpf_fentry_test*'
// through tracing test_run
//
    SEC("fentry/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trigger) -> c_int {
    int BPF_PROG(trigger)
    {
    return 0;
    }
    SEC("kprobe.multi/bpf_fentry_tes??")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe(ctx: *mut pt_regs) -> c_int {
    int test_kprobe(struct pt_regs *ctx)
    {
    kprobe_multi_check(ctx, false);
    return 0;
    }
    SEC("kretprobe.multi/bpf_fentry_test*")
#[no_mangle]
pub unsafe extern "C" fn test_kretprobe(ctx: *mut pt_regs) -> c_int {
    int test_kretprobe(struct pt_regs *ctx)
    {
    kprobe_multi_check(ctx, true);
    return 0;
    }
    SEC("kprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe_manual(ctx: *mut pt_regs) -> c_int {
    int test_kprobe_manual(struct pt_regs *ctx)
    {
    kprobe_multi_check(ctx, false);
    return 0;
    }
    SEC("kretprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn test_kretprobe_manual(ctx: *mut pt_regs) -> c_int {
    int test_kretprobe_manual(struct pt_regs *ctx)
    {
    kprobe_multi_check(ctx, true);
    return 0;
    }
    extern const void bpf_testmod_fentry_test1 __ksym;
    extern const void bpf_testmod_fentry_test2 __ksym;
    extern const void bpf_testmod_fentry_test3 __ksym;
    let mut kprobe_testmod_test1_result: __u64 = 0;
    let mut kprobe_testmod_test2_result: __u64 = 0;
    let mut kprobe_testmod_test3_result: __u64 = 0;
    let mut kretprobe_testmod_test1_result: __u64 = 0;
    let mut kretprobe_testmod_test2_result: __u64 = 0;
    let mut kretprobe_testmod_test3_result: __u64 = 0;
#[no_mangle]
unsafe extern "C" fn kprobe_multi_testmod_check(ctx: *mut c_void, is_return: bool) {
    static void kprobe_multi_testmod_check(void *ctx, bool is_return)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return;
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    if (is_return) {
    if ((const void *) addr == &bpf_testmod_fentry_test1)
    kretprobe_testmod_test1_result = 1;
    if ((const void *) addr == &bpf_testmod_fentry_test2)
    kretprobe_testmod_test2_result = 1;
    if ((const void *) addr == &bpf_testmod_fentry_test3)
    kretprobe_testmod_test3_result = 1;
    } else {
    if ((const void *) addr == &bpf_testmod_fentry_test1)
    kprobe_testmod_test1_result = 1;
    if ((const void *) addr == &bpf_testmod_fentry_test2)
    kprobe_testmod_test2_result = 1;
    if ((const void *) addr == &bpf_testmod_fentry_test3)
    kprobe_testmod_test3_result = 1;
    }
    }
    SEC("kprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe_testmod(ctx: *mut pt_regs) -> c_int {
    int test_kprobe_testmod(struct pt_regs *ctx)
    {
    kprobe_multi_testmod_check(ctx, false);
    return 0;
    }
    SEC("kretprobe.multi")
#[no_mangle]
pub unsafe extern "C" fn test_kretprobe_testmod(ctx: *mut pt_regs) -> c_int {
    int test_kretprobe_testmod(struct pt_regs *ctx)
    {
    kprobe_multi_testmod_check(ctx, true);
    return 0;
    }
