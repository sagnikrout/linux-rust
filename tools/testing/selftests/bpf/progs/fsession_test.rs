//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fsession_test.c
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
// Copyright (c) 2025 ChinaTelecom

    char _license[] SEC("license") = "GPL";
    let mut test1_entry_result: __u64 = 0;
    let mut test1_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test1, a: c_int, ret: c_int) -> c_int {
    int BPF_PROG(test1, int a, int ret)
    {
    let mut is_exit: bool = bpf_session_is_return(ctx);
    if (!is_exit) {
    test1_entry_result = a == 1 && ret == 0;
    return 0;
    }
    test1_exit_result = a == 1 && ret == 2;
    return 0;
    }
    let mut test2_entry_result: __u64 = 0;
    let mut test2_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test2, a: c_char, b: c_int, c: __u64, ret: c_int) -> c_int {
    int BPF_PROG(test2, char a, int b, __u64 c, int ret)
    {
    let mut is_exit: bool = bpf_session_is_return(ctx);
    if (!is_exit) {
    test2_entry_result = a == 4 && b == 5 && c == 6 && ret == 0;
    return 0;
    }
    test2_exit_result = a == 4 && b == 5 && c == 6 && ret == 15;
    return 0;
    }
    let mut test3_entry_result: __u64 = 0;
    let mut test3_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test4")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test3, a: *mut c_void, b: c_char, c: c_int, d: __u64, ret: c_int) -> c_int {
    int BPF_PROG(test3, void *a, char b, int c, __u64 d, int ret)
    {
    let mut is_exit: bool = bpf_session_is_return(ctx);
    if (!is_exit) {
    test3_entry_result = a == (void *)7 && b == 8 && c == 9 && d == 10 && ret == 0;
    return 0;
    }
    test3_exit_result = a == (void *)7 && b == 8 && c == 9 && d == 10 && ret == 34;
    return 0;
    }
    let mut test4_entry_result: __u64 = 0;
    let mut test4_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test5")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test4, a: __u64, b: *mut c_void, c: c_short, d: c_int, e: __u64, ret: c_int) -> c_int {
    int BPF_PROG(test4, __u64 a, void *b, short c, int d, __u64 e, int ret)
    {
    let mut is_exit: bool = bpf_session_is_return(ctx);
    if (!is_exit) {
    test4_entry_result = a == 11 && b == (void *)12 && c == 13 && d == 14 &&
    e == 15 && ret == 0;
    return 0;
    }
    test4_exit_result = a == 11 && b == (void *)12 && c == 13 && d == 14 &&
    e == 15 && ret == 65;
    return 0;
    }
    let mut test5_entry_result: __u64 = 0;
    let mut test5_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test7")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test5, arg: *mut bpf_fentry_test_t, ret: c_int) -> c_int {
    int BPF_PROG(test5, struct bpf_fentry_test_t *arg, int ret)
    {
    let mut is_exit: bool = bpf_session_is_return(ctx);
    if (!is_exit) {
    if (!arg)
    test5_entry_result = ret == 0;
    return 0;
    }
    if (!arg)
    test5_exit_result = 1;
    return 0;
    }
    let mut test6_entry_result: __u64 = 0;
    let mut test6_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test6, a: c_int) -> c_int {
    int BPF_PROG(test6, int a)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    if (bpf_session_is_return(ctx))
    test6_exit_result = (const void *) addr == &bpf_fentry_test1;
    else
    test6_entry_result = (const void *) addr == &bpf_fentry_test1;
    return 0;
    }
    let mut test7_entry_ok: __u64 = 0;
    let mut test7_exit_ok: __u64 = 0;
    SEC("fsession/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test7, a: c_int) -> c_int {
    int BPF_PROG(test7, int a)
    {
    volatile __u64 *cookie = bpf_session_cookie(ctx);
    if (!bpf_session_is_return(ctx)) {
// cookie = 0xAAAABBBBCCCCDDDDull;
    test7_entry_ok = *cookie == 0xAAAABBBBCCCCDDDDull;
    return 0;
    }
    test7_exit_ok = *cookie == 0xAAAABBBBCCCCDDDDull;
    return 0;
    }
    let mut test8_entry_ok: __u64 = 0;
    let mut test8_exit_ok: __u64 = 0;
    SEC("fsession/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test8, a: c_int) -> c_int {
    int BPF_PROG(test8, int a)
    {
    volatile __u64 *cookie = bpf_session_cookie(ctx);
    if (!bpf_session_is_return(ctx)) {
// cookie = 0x1111222233334444ull;
    test8_entry_ok = *cookie == 0x1111222233334444ull;
    return 0;
    }
    test8_exit_ok = *cookie == 0x1111222233334444ull;
    return 0;
    }
    let mut test9_entry_result: __u64 = 0;
    let mut test9_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test9, a: c_int, ret: c_int) -> c_int {
    int BPF_PROG(test9, int a, int ret)
    {
    __u64 *cookie = bpf_session_cookie(ctx);
    if (!bpf_session_is_return(ctx)) {
    test9_entry_result = a == 1 && ret == 0;
// cookie = 0x123456ULL;
    return 0;
    }
    test9_exit_result = a == 1 && ret == 2 && *cookie == 0x123456ULL;
    return 0;
    }
    let mut test10_result: __u64 = 0;
    SEC("fexit/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test10, a: c_int, ret: c_int) -> c_int {
    int BPF_PROG(test10, int a, int ret)
    {
    test10_result = a == 1 && ret == 2;
    return 0;
    }
    let mut test11_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test11, a: c_int) -> c_int {
    int BPF_PROG(test11, int a)
    {
    test11_result = a == 1;
    return 0;
    }
