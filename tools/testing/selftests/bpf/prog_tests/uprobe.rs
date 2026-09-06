//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/uprobe.c
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
// Copyright (c) 2023 Hengqi Chen

    static FILE *urand_spawn(int *pid)
    {
    FILE *f;
// urandom_read's stdout is wired into f
    f = popen("./urandom_read 1 report-pid", "r");
    if (!f)
    return core::ptr::null_mut();
    if (fscanf(f, "%d", pid) != 1) {
    pclose(f);
    errno = EINVAL;
    return core::ptr::null_mut();
    }
    return f;
    }
#[no_mangle]
unsafe extern "C" fn urand_trigger(urand_pipe: *mut FILE) -> c_int {
    static int urand_trigger(FILE **urand_pipe)
    {
    int exit_code;
// pclose() waits for child process to exit and returns their exit code
    exit_code = pclose(*urand_pipe);
// urand_pipe = NULL;
    return exit_code;
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_attach() {
    static void test_uprobe_attach(void)
    {
    LIBBPF_OPTS(bpf_uprobe_opts, uprobe_opts);
    struct test_uprobe *skel;
    FILE *urand_pipe = core::ptr::null_mut();
    let mut urand_pid: c_int = 0, err;
    skel = test_uprobe__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    urand_pipe = urand_spawn(&urand_pid);
    if (!ASSERT_OK_PTR(urand_pipe, "urand_spawn"))
    goto cleanup;
    skel.bss.my_pid = urand_pid;
// Manual attach uprobe to urandlib_api
// There are two `urandlib_api` symbols in .dynsym section:
// - urandlib_api@LIBURANDOM_READ_1.0.0
// - urandlib_api@@LIBURANDOM_READ_2.0.0
// Both are global bind and would cause a conflict if user
// specify the symbol name without a version suffix
//
    uprobe_opts.func_name = "urandlib_api";
    skel.links.test4 = bpf_program__attach_uprobe_opts(skel.progs.test4,
    urand_pid,
    "./liburandom_read.so",
    0 /* offset */,
    &uprobe_opts);
    if (!ASSERT_ERR_PTR(skel.links.test4, "urandlib_api_attach_conflict"))
    goto cleanup;
    uprobe_opts.func_name = "urandlib_api@LIBURANDOM_READ_1.0.0";
    skel.links.test4 = bpf_program__attach_uprobe_opts(skel.progs.test4,
    urand_pid,
    "./liburandom_read.so",
    0 /* offset */,
    &uprobe_opts);
    if (!ASSERT_OK_PTR(skel.links.test4, "urandlib_api_attach_ok"))
    goto cleanup;
// Auto attach 3 u[ret]probes to urandlib_api_sameoffset
    err = test_uprobe__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
// trigger urandom_read
    ASSERT_OK(urand_trigger(&urand_pipe), "urand_exit_code");
    ASSERT_EQ(skel.bss.test1_result, 1, "urandlib_api_sameoffset");
    ASSERT_EQ(skel.bss.test2_result, 1, "urandlib_api_sameoffset@v1");
    ASSERT_EQ(skel.bss.test3_result, 3, "urandlib_api_sameoffset@@v2");
    ASSERT_EQ(skel.bss.test4_result, 1, "urandlib_api");
    cleanup:
    if (urand_pipe)
    pclose(urand_pipe);
    test_uprobe__destroy(skel);
    }

#[no_mangle]
pub unsafe extern "C" fn uprobe_regs_change_trigger() -> __naked __maybe_unused unsigned long {
    __naked __maybe_unused unsigned long uprobe_regs_change_trigger(void)
    {
    asm volatile (
    "ret\n"
    );
    }
#[no_mangle]
unsafe extern "C" fn uprobe_regs_change(before: *mut pt_regs, after: *mut pt_regs) -> __naked void {
    static __naked void uprobe_regs_change(struct pt_regs *before, struct pt_regs *after)
    {
    asm volatile (
    "movq %r11,  48(%rdi)\n"
    "movq %r10,  56(%rdi)\n"
    "movq  %r9,  64(%rdi)\n"
    "movq  %r8,  72(%rdi)\n"
    "movq %rax,  80(%rdi)\n"
    "movq %rcx,  88(%rdi)\n"
    "movq %rdx,  96(%rdi)\n"
    "movq %rsi, 104(%rdi)\n"
    "movq %rdi, 112(%rdi)\n"
// save 2nd argument
    "pushq %rsi\n"
    "call uprobe_regs_change_trigger\n"
// save  return value and load 2nd argument pointer to rax
    "pushq %rax\n"
    "movq 8(%rsp), %rax\n"
    "movq %r11,  48(%rax)\n"
    "movq %r10,  56(%rax)\n"
    "movq  %r9,  64(%rax)\n"
    "movq  %r8,  72(%rax)\n"
    "movq %rcx,  88(%rax)\n"
    "movq %rdx,  96(%rax)\n"
    "movq %rsi, 104(%rax)\n"
    "movq %rdi, 112(%rax)\n"
// restore return value and 2nd argument
    "pop %rax\n"
    "pop %rsi\n"
    "movq %rax,  80(%rsi)\n"
    "ret\n"
    );
    }
#[no_mangle]
unsafe extern "C" fn regs_common() {
    static void regs_common(void)
    {
    struct pt_regs before = {}, after = {}, expected = {
    .rax = 0xc0ffe,
    .rcx = 0xbad,
    .rdx = 0xdead,
    .r8  = 0x8,
    .r9  = 0x9,
    .r10 = 0x10,
    .r11 = 0x11,
    .rdi = 0x12,
    .rsi = 0x13,
    };
    LIBBPF_OPTS(bpf_uprobe_opts, uprobe_opts);
    struct test_uprobe *skel;
    skel = test_uprobe__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.bss.my_pid = getpid();
    skel.bss.regs = expected;
    uprobe_opts.func_name = "uprobe_regs_change_trigger";
    skel.links.test_regs_change = bpf_program__attach_uprobe_opts(skel.progs.test_regs_change,
    -1,
    "/proc/self/exe",
    0 /* offset */,
    &uprobe_opts);
    if (!ASSERT_OK_PTR(skel.links.test_regs_change, "bpf_program__attach_uprobe_opts"))
    goto cleanup;
    uprobe_regs_change(&before, &after);
    ASSERT_EQ(after.rax, expected.rax, "ax");
    ASSERT_EQ(after.rcx, expected.rcx, "cx");
    ASSERT_EQ(after.rdx, expected.rdx, "dx");
    ASSERT_EQ(after.r8,  expected.r8,  "r8");
    ASSERT_EQ(after.r9,  expected.r9,  "r9");
    ASSERT_EQ(after.r10, expected.r10, "r10");
    ASSERT_EQ(after.r11, expected.r11, "r11");
    ASSERT_EQ(after.rdi, expected.rdi, "rdi");
    ASSERT_EQ(after.rsi, expected.rsi, "rsi");
    cleanup:
    test_uprobe__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_regs_change_ip_1() -> noinline unsigned long {
    static noinline unsigned long uprobe_regs_change_ip_1(void)
    {
    return 0xc0ffee;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_regs_change_ip_2() -> noinline unsigned long {
    static noinline unsigned long uprobe_regs_change_ip_2(void)
    {
    return 0xdeadbeef;
    }
#[no_mangle]
unsafe extern "C" fn regs_ip() {
    static void regs_ip(void)
    {
    LIBBPF_OPTS(bpf_uprobe_opts, uprobe_opts);
    struct test_uprobe *skel;
    unsigned long ret;
    skel = test_uprobe__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.bss.my_pid = getpid();
    skel.bss.ip = (unsigned long) uprobe_regs_change_ip_2;
    uprobe_opts.func_name = "uprobe_regs_change_ip_1";
    skel.links.test_regs_change_ip = bpf_program__attach_uprobe_opts(
    skel.progs.test_regs_change_ip,
    -1,
    "/proc/self/exe",
    0 /* offset */,
    &uprobe_opts);
    if (!ASSERT_OK_PTR(skel.links.test_regs_change_ip, "bpf_program__attach_uprobe_opts"))
    goto cleanup;
    ret = uprobe_regs_change_ip_1();
    ASSERT_EQ(ret, 0xdeadbeef, "ret");
    cleanup:
    test_uprobe__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_uprobe_regs_change() {
    static void test_uprobe_regs_change(void)
    {
    if (test__start_subtest("regs_change_common"))
    regs_common();
    if (test__start_subtest("regs_change_ip"))
    regs_ip();
    }

    static void test_uprobe_regs_change(void) { }

#[no_mangle]
pub unsafe extern "C" fn test_uprobe() {
    void test_uprobe(void)
    {
    if (test__start_subtest("attach"))
    test_uprobe_attach();
    test_uprobe_regs_change();
    }
