//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/snprintf.c
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
// Copyright (c) 2021 Google LLC.

// The third specifier, %pB, depends on compiler inlining so don't check it

// The third specifier, %p, is a hashed pointer which changes on every reboot

pub const EXP_OVER_RET: c_int = 10;

pub const EXP_PAD_RET: c_int = 900007;

pub const EXP_NO_ARG_RET: c_int = 12;
pub const EXP_NO_BUF_RET: c_int = 29;
#[no_mangle]
unsafe extern "C" fn test_snprintf_positive() {
    static void test_snprintf_positive(void)
    {
    char exp_addr_out[] = EXP_ADDR_OUT;
    char exp_sym_out[]  = EXP_SYM_OUT;
    struct test_snprintf *skel;
    skel = test_snprintf__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.bss.pid = getpid();
    if (!ASSERT_OK(test_snprintf__attach(skel), "skel_attach"))
    goto cleanup;
// trigger tracepoint
    usleep(1);
    ASSERT_STREQ(skel.bss.num_out, EXP_NUM_OUT, "num_out");
    ASSERT_EQ(skel.bss.num_ret, EXP_NUM_RET, "num_ret");
    ASSERT_STREQ(skel.bss.ip_out, EXP_IP_OUT, "ip_out");
    ASSERT_EQ(skel.bss.ip_ret, EXP_IP_RET, "ip_ret");
    ASSERT_OK(memcmp(skel.bss.sym_out, exp_sym_out,
    sizeof(exp_sym_out) - 1), "sym_out");
    ASSERT_LT(MIN_SYM_RET, skel.bss.sym_ret, "sym_ret");
    ASSERT_OK(memcmp(skel.bss.addr_out, exp_addr_out,
    sizeof(exp_addr_out) - 1), "addr_out");
    ASSERT_EQ(skel.bss.addr_ret, EXP_ADDR_RET, "addr_ret");
    ASSERT_STREQ(skel.bss.str_out, EXP_STR_OUT, "str_out");
    ASSERT_EQ(skel.bss.str_ret, EXP_STR_RET, "str_ret");
    ASSERT_STREQ(skel.bss.over_out, EXP_OVER_OUT, "over_out");
    ASSERT_EQ(skel.bss.over_ret, EXP_OVER_RET, "over_ret");
    ASSERT_STREQ(skel.bss.pad_out, EXP_PAD_OUT, "pad_out");
    ASSERT_EQ(skel.bss.pad_ret, EXP_PAD_RET, "pad_ret");
    ASSERT_STREQ(skel.bss.noarg_out, EXP_NO_ARG_OUT, "no_arg_out");
    ASSERT_EQ(skel.bss.noarg_ret, EXP_NO_ARG_RET, "no_arg_ret");
    ASSERT_EQ(skel.bss.nobuf_ret, EXP_NO_BUF_RET, "no_buf_ret");
    cleanup:
    test_snprintf__destroy(skel);
    }
// Loads an eBPF object calling bpf_snprintf with up to 10 characters of fmt
#[no_mangle]
unsafe extern "C" fn load_single_snprintf(fmt: *mut c_char) -> c_int {
    static int load_single_snprintf(char *fmt)
    {
    struct test_snprintf_single *skel;
    int ret;
    skel = test_snprintf_single__open();
    if (!skel)
    return -EINVAL;
    memcpy(skel.rodata.fmt, fmt, MIN(strlen(fmt) + 1, 10));
    ret = test_snprintf_single__load(skel);
    test_snprintf_single__destroy(skel);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_snprintf_negative() {
    static void test_snprintf_negative(void)
    {
    ASSERT_OK(load_single_snprintf("valid %d"), "valid usage");
    ASSERT_ERR(load_single_snprintf("0123456789"), "no terminating zero");
    ASSERT_ERR(load_single_snprintf("%d %d"), "too many specifiers");
    ASSERT_ERR(load_single_snprintf("%pi5"), "invalid specifier 1");
    ASSERT_ERR(load_single_snprintf("%a"), "invalid specifier 2");
    ASSERT_ERR(load_single_snprintf("%"), "invalid specifier 3");
    ASSERT_ERR(load_single_snprintf("%12345678"), "invalid specifier 4");
    ASSERT_ERR(load_single_snprintf("%--------"), "invalid specifier 5");
    ASSERT_ERR(load_single_snprintf("%lc"), "invalid specifier 6");
    ASSERT_ERR(load_single_snprintf("%llc"), "invalid specifier 7");
    ASSERT_OK(load_single_snprintf("\x80"), "non ascii plain text");
    ASSERT_ERR(load_single_snprintf("%\x80"), "non ascii in specifier");
    ASSERT_ERR(load_single_snprintf("\x1"), "non printable character");
    ASSERT_ERR(load_single_snprintf("%p%"), "invalid specifier 8");
    ASSERT_ERR(load_single_snprintf("%s%"), "invalid specifier 9");
    }
#[no_mangle]
pub unsafe extern "C" fn test_snprintf() {
    void test_snprintf(void)
    {
    if (test__start_subtest("snprintf_positive"))
    test_snprintf_positive();
    if (test__start_subtest("snprintf_negative"))
    test_snprintf_negative();
    }
