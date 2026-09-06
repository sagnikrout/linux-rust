//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tracing_multi_check.c
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
    let mut pid: c_int = 0;
    let mut test_cookies: bool = false;
// bpf_fentry_test1 is exported as kfunc via vmlinux.h
    extern const void bpf_fentry_test2 __ksym;
    extern const void bpf_fentry_test3 __ksym;
    extern const void bpf_fentry_test4 __ksym;
    extern const void bpf_fentry_test5 __ksym;
    extern const void bpf_fentry_test6 __ksym;
    extern const void bpf_fentry_test7 __ksym;
    extern const void bpf_fentry_test8 __ksym;
    extern const void bpf_fentry_test9 __ksym;
    extern const void bpf_fentry_test10 __ksym;
    extern const void bpf_testmod_fentry_test1 __ksym;
    extern const void bpf_testmod_fentry_test2 __ksym;
    extern const void bpf_testmod_fentry_test3 __ksym;
    extern const void bpf_testmod_fentry_test7 __ksym;
    extern const void bpf_testmod_fentry_test11 __ksym;
#[no_mangle]
pub unsafe extern "C" fn tracing_multi_arg_check(ctx: *mut __u64, test_result: *mut __u64, is_return: bool) -> c_int {
    int tracing_multi_arg_check(__u64 *ctx, __u64 *test_result, bool is_return)
    {
    void *ip = (void *) bpf_get_func_ip(ctx);
    let mut value: __u64 = 0, ret = 0, cookie = 0;
    let mut err: c_long = 0;
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 1;
    if (is_return)
    err |= bpf_get_func_ret(ctx, &ret);
    if (test_cookies)
    cookie = bpf_get_attach_cookie(ctx);
    if (ip == &bpf_fentry_test1) {
    int a;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = (int) value;
    err |= is_return ? ret != 2 : 0;
    err |= test_cookies ? cookie != 8 : 0;
// test_result += err == 0 && a == 1;
    } else if (ip == &bpf_fentry_test2) {
    __u64 b;
    int a;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = (int) value;
    err |= bpf_get_func_arg(ctx, 1, &value);
    b = value;
    err |= is_return ? ret != 5 : 0;
    err |= test_cookies ? cookie != 9 : 0;
// test_result += err == 0 && a == 2 && b == 3;
    } else if (ip == &bpf_fentry_test3) {
    __u64 c;
    char a;
    int b;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = (char) value;
    err |= bpf_get_func_arg(ctx, 1, &value);
    b = (int) value;
    err |= bpf_get_func_arg(ctx, 2, &value);
    c = value;
    err |= is_return ? ret != 15 : 0;
    err |= test_cookies ? cookie != 7 : 0;
// test_result += err == 0 && a == 4 && b == 5 && c == 6;
    } else if (ip == &bpf_fentry_test4) {
    void *a;
    char b;
    int c;
    __u64 d;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = (void *) value;
    err |= bpf_get_func_arg(ctx, 1, &value);
    b = (char) value;
    err |= bpf_get_func_arg(ctx, 2, &value);
    c = (int) value;
    err |= bpf_get_func_arg(ctx, 3, &value);
    d = value;
    err |= is_return ? ret != 34 : 0;
    err |= test_cookies ? cookie != 5 : 0;
// test_result += err == 0 && a == (void *) 7 && b == 8 && c == 9 && d == 10;
    } else if (ip == &bpf_fentry_test5) {
    __u64 a;
    void *b;
    short c;
    int d;
    __u64 e;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = value;
    err |= bpf_get_func_arg(ctx, 1, &value);
    b = (void *) value;
    err |= bpf_get_func_arg(ctx, 2, &value);
    c = (short) value;
    err |= bpf_get_func_arg(ctx, 3, &value);
    d = (int) value;
    err |= bpf_get_func_arg(ctx, 4, &value);
    e = value;
    err |= is_return ? ret != 65 : 0;
    err |= test_cookies ? cookie != 4 : 0;
// test_result += err == 0 && a == 11 && b == (void *) 12 && c == 13 && d == 14 && e == 15;
    } else if (ip == &bpf_fentry_test6) {
    __u64 a;
    void *b;
    short c;
    int d;
    void *e;
    __u64 f;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = value;
    err |= bpf_get_func_arg(ctx, 1, &value);
    b = (void *) value;
    err |= bpf_get_func_arg(ctx, 2, &value);
    c = (short) value;
    err |= bpf_get_func_arg(ctx, 3, &value);
    d = (int) value;
    err |= bpf_get_func_arg(ctx, 4, &value);
    e = (void *) value;
    err |= bpf_get_func_arg(ctx, 5, &value);
    f = value;
    err |= is_return ? ret != 111 : 0;
    err |= test_cookies ? cookie != 2 : 0;
// test_result += err == 0 && a == 16 && b == (void *) 17 && c == 18 && d == 19 && e == (void *) 20 && f == 21;
    } else if (ip == &bpf_fentry_test7) {
    err |= is_return ? ret != 0 : 0;
    err |= test_cookies ? cookie != 3 : 0;
// test_result += err == 0 ? 1 : 0;
    } else if (ip == &bpf_fentry_test8) {
    err |= is_return ? ret != 0 : 0;
    err |= test_cookies ? cookie != 1 : 0;
// test_result += err == 0 ? 1 : 0;
    } else if (ip == &bpf_fentry_test9) {
    err |= is_return ? ret != 0 : 0;
    err |= test_cookies ? cookie != 10 : 0;
// test_result += err == 0 ? 1 : 0;
    } else if (ip == &bpf_fentry_test10) {
    err |= is_return ? ret != 0 : 0;
    err |= test_cookies ? cookie != 6 : 0;
// test_result += err == 0 ? 1 : 0;
    } else if (ip == &bpf_testmod_fentry_test1) {
    int a;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = (int) value;
    err |= is_return ? ret != 2 : 0;
// test_result += err == 0 && a == 1;
    } else if (ip == &bpf_testmod_fentry_test2) {
    int a;
    __u64 b;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = (int) value;
    err |= bpf_get_func_arg(ctx, 1, &value);
    b = (__u64) value;
    err |= is_return ? ret != 5 : 0;
// test_result += err == 0 && a == 2 && b == 3;
    } else if (ip == &bpf_testmod_fentry_test3) {
    char a;
    int b;
    __u64 c;
    err |= bpf_get_func_arg(ctx, 0, &value);
    a = (char) value;
    err |= bpf_get_func_arg(ctx, 1, &value);
    b = (int) value;
    err |= bpf_get_func_arg(ctx, 2, &value);
    c = (__u64) value;
    err |= is_return ? ret != 15 : 0;
// test_result += err == 0 && a == 4 && b == 5 && c == 6;
    } else if (ip == &bpf_testmod_fentry_test7) {
    err |= is_return ? ret != 133 : 0;
// test_result += err == 0;
    } else if (ip == &bpf_testmod_fentry_test11) {
    err |= is_return ? ret != 231 : 0;
// test_result += err == 0;
    }
    return 0;
    }
