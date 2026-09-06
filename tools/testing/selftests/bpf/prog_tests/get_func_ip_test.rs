//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/get_func_ip_test.c
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

#[no_mangle]
unsafe extern "C" fn uprobe_trigger() -> noinline void {
    static noinline void uprobe_trigger(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn test_function_entry() {
    static void test_function_entry(void)
    {
    struct get_func_ip_test *skel = core::ptr::null_mut();
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    skel = get_func_ip_test__open();
    if (!ASSERT_OK_PTR(skel, "get_func_ip_test__open"))
    return;
    err = get_func_ip_test__load(skel);
    if (!ASSERT_OK(err, "get_func_ip_test__load"))
    goto cleanup;
    err = get_func_ip_test__attach(skel);
    if (!ASSERT_OK(err, "get_func_ip_test__attach"))
    goto cleanup;
    skel.bss.uprobe_trigger = (unsigned long) uprobe_trigger;
    prog_fd = bpf_program__fd(skel.progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    prog_fd = bpf_program__fd(skel.progs.test5);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    uprobe_trigger();
    ASSERT_EQ(skel.bss.test1_result, 1, "test1_result");
    ASSERT_EQ(skel.bss.test2_result, 1, "test2_result");
    ASSERT_EQ(skel.bss.test3_result, 1, "test3_result");
    ASSERT_EQ(skel.bss.test4_result, 1, "test4_result");
    ASSERT_EQ(skel.bss.test5_result, 1, "test5_result");
    ASSERT_EQ(skel.bss.test7_result, 1, "test7_result");
    ASSERT_EQ(skel.bss.test8_result, 1, "test8_result");
    cleanup:
    get_func_ip_test__destroy(skel);
    }

    extern void uprobe_trigger_body(void);
    asm(
    ".globl uprobe_trigger_body\n"
    ".type uprobe_trigger_body, @function\n"
    "uprobe_trigger_body:\n"
    "	nop\n"
    "	ret\n"
    );
#[no_mangle]
unsafe extern "C" fn test_function_body_kprobe() {
    static void test_function_body_kprobe(void)
    {
    struct get_func_ip_test *skel = core::ptr::null_mut();
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    LIBBPF_OPTS(bpf_kprobe_opts, kopts);
    struct bpf_link *link6 = core::ptr::null_mut();
    int err, prog_fd;
    skel = get_func_ip_test__open();
    if (!ASSERT_OK_PTR(skel, "get_func_ip_test__open"))
    return;
// test6 is x86_64 specific and is disabled by default,
// enable it for body test.
//
    bpf_program__set_autoload(skel.progs.test6, true);
    err = get_func_ip_test__load(skel);
    if (!ASSERT_OK(err, "get_func_ip_test__load"))
    goto cleanup;
    kopts.offset = skel.kconfig.CONFIG_X86_KERNEL_IBT ? 9 : 5;
    link6 = bpf_program__attach_kprobe_opts(skel.progs.test6, "bpf_fentry_test6", &kopts);
    if (!ASSERT_OK_PTR(link6, "link6"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    ASSERT_EQ(skel.bss.test6_result, 1, "test6_result");
    cleanup:
    bpf_link__destroy(link6);
    get_func_ip_test__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_function_body_uprobe() {
    static void test_function_body_uprobe(void)
    {
    struct get_func_ip_uprobe_test *skel = core::ptr::null_mut();
    int err;
    skel = get_func_ip_uprobe_test__open_and_load();
    if (!ASSERT_OK_PTR(skel, "get_func_ip_uprobe_test__open_and_load"))
    return;
    err = get_func_ip_uprobe_test__attach(skel);
    if (!ASSERT_OK(err, "get_func_ip_test__attach"))
    goto cleanup;
    skel.bss.uprobe_trigger_body = (unsigned long) uprobe_trigger_body;
    uprobe_trigger_body();
    ASSERT_EQ(skel.bss.test1_result, 1, "test1_result");
    cleanup:
    get_func_ip_uprobe_test__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_function_body() {
    static void test_function_body(void)
    {
    test_function_body_kprobe();
    test_function_body_uprobe();
    }

// Macro flag: #define test_function_body()

#[no_mangle]
pub unsafe extern "C" fn test_get_func_ip_test() {
    void test_get_func_ip_test(void)
    {
    test_function_entry();
    test_function_body();
    }
#[no_mangle]
pub unsafe extern "C" fn test_get_func_ip_fsession_test() {
    void test_get_func_ip_fsession_test(void)
    {
    struct get_func_ip_fsession_test *skel = core::ptr::null_mut();
    int err;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    skel = get_func_ip_fsession_test__open_and_load();
    if (!ASSERT_OK_PTR(skel, "get_func_ip_fsession_test__open_and_load"))
    return;
    err = get_func_ip_fsession_test__attach(skel);
    if (!ASSERT_OK(err, "get_func_ip_fsession_test__attach"))
    goto cleanup;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.test1), &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    ASSERT_EQ(skel.bss.test1_entry_result, 1, "test1_entry_result");
    ASSERT_EQ(skel.bss.test1_exit_result, 1, "test1_exit_result");
    cleanup:
    get_func_ip_fsession_test__destroy(skel);
    }
