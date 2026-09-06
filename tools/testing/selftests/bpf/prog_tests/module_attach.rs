//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/module_attach.c
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
// Copyright (c) 2020 Facebook

    static const char * const read_tests[] = {
    "handle_raw_tp",
    "handle_tp_btf",
    "handle_fentry",
    "handle_fentry_explicit",
    "handle_fmod_ret",
    };
    static const char * const detach_tests[] = {
    "handle_fentry",
    "handle_fexit",
    "kprobe_multi",
    };
    let mut READ_SZ: static int = 456;
    let mut WRITE_SZ: static int = 457;
#[no_mangle]
unsafe extern "C" fn trigger_module_test_writable(val: *mut c_int) -> c_int {
    static int trigger_module_test_writable(int *val)
    {
    int fd, err;
    char buf[65];
    ssize_t rd;
    fd = open(BPF_TESTMOD_TEST_FILE, O_RDONLY);
    err = -errno;
    if (!ASSERT_GE(fd, 0, "testmode_file_open"))
    return err;
    rd = read(fd, buf, sizeof(buf) - 1);
    err = -errno;
    if (!ASSERT_GT(rd, 0, "testmod_file_rd_val")) {
    close(fd);
    return err;
    }
    buf[rd] = '\0';
// val = strtol(buf, NULL, 0);
    close(fd);
    return 0;
    }
    static void test_module_attach_prog(const char *prog_name, int sz,
    const char *attach_target, int ret)
    {
    struct test_module_attach *skel;
    struct bpf_program *prog;
    int err;
    skel = test_module_attach__open();
    if (!ASSERT_OK_PTR(skel, "module_attach open"))
    return;
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!ASSERT_OK_PTR(prog, "module_attach find_program"))
    goto cleanup;
    bpf_program__set_autoload(prog, true);
    if (attach_target) {
    err = bpf_program__set_attach_target(prog, 0, attach_target);
    if (!ASSERT_OK(err, attach_target))
    goto cleanup;
    }
    err = test_module_attach__load(skel);
    if (!ASSERT_OK(err, "module_attach load"))
    goto cleanup;
    err = test_module_attach__attach(skel);
    if (!ASSERT_OK(err, "module_attach attach"))
    goto cleanup;
    if (sz) {
// trigger both read and write though each test uses only one
    ASSERT_OK(trigger_module_test_read(sz), "trigger_read");
    ASSERT_OK(trigger_module_test_write(sz), "trigger_write");
    ASSERT_EQ(skel.bss.sz, sz, prog_name);
    }
    if (ret)
    ASSERT_EQ(skel.bss.retval, ret, "ret");
    cleanup:
    test_module_attach__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_module_attach_writable() {
    static void test_module_attach_writable(void)
    {
    struct test_module_attach__bss *bss;
    struct test_module_attach *skel;
    let mut writable_val: c_int = 0;
    int err;
    skel = test_module_attach__open();
    if (!ASSERT_OK_PTR(skel, "module_attach open"))
    return;
    bpf_program__set_autoload(skel.progs.handle_raw_tp_writable_bare, true);
    err = test_module_attach__load(skel);
    if (!ASSERT_OK(err, "module_attach load"))
    goto cleanup;
    bss = skel.bss;
    err = test_module_attach__attach(skel);
    if (!ASSERT_OK(err, "module_attach attach"))
    goto cleanup;
    bss.raw_tp_writable_bare_early_ret = true;
    bss.raw_tp_writable_bare_out_val = 0xf1f2f3f4;
    ASSERT_OK(trigger_module_test_writable(&writable_val),
    "trigger_writable");
    ASSERT_EQ(bss.raw_tp_writable_bare_in_val, 1024, "writable_test_in");
    ASSERT_EQ(bss.raw_tp_writable_bare_out_val, writable_val,
    "writable_test_out");
    cleanup:
    test_module_attach__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_module_attach_detach(prog_name: *const c_char) {
    static void test_module_attach_detach(const char *prog_name)
    {
    struct test_module_attach *skel;
    struct bpf_program *prog;
    struct bpf_link *link;
    int err;
    skel = test_module_attach__open();
    if (!ASSERT_OK_PTR(skel, "module_attach open"))
    return;
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!ASSERT_OK_PTR(prog, "module_attach find_program"))
    goto cleanup;
    bpf_program__set_autoload(prog, true);
    err = test_module_attach__load(skel);
    if (!ASSERT_OK(err, "module_attach load"))
    goto cleanup;
// attach and make sure it gets module reference
    link = bpf_program__attach(prog);
    if (!ASSERT_OK_PTR(link, "module_attach attach"))
    goto cleanup;
    ASSERT_ERR(try_unload_module("bpf_testmod", 1, false), "try_unload_module");
    bpf_link__destroy(link);
    cleanup:
    test_module_attach__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_module_attach() {
    void test_module_attach(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(read_tests); i++) {
    if (!test__start_subtest(read_tests[i]))
    continue;
    test_module_attach_prog(read_tests[i], READ_SZ, core::ptr::null_mut(), 0);
    }
    if (test__start_subtest("handle_raw_tp_bare"))
    test_module_attach_prog("handle_raw_tp_bare", WRITE_SZ, core::ptr::null_mut(), 0);
    if (test__start_subtest("handle_raw_tp_writable_bare"))
    test_module_attach_writable();
    if (test__start_subtest("handle_fentry_manual")) {
    test_module_attach_prog("handle_fentry_manual", READ_SZ,
    "bpf_testmod_test_read", 0);
    }
    if (test__start_subtest("handle_fentry_explicit_manual")) {
    test_module_attach_prog("handle_fentry_explicit_manual",
    READ_SZ,
    "bpf_testmod:bpf_testmod_test_read", 0);
    }
    if (test__start_subtest("handle_fexit"))
    test_module_attach_prog("handle_fexit", READ_SZ, core::ptr::null_mut(), -EIO);
    if (test__start_subtest("handle_fexit_ret"))
    test_module_attach_prog("handle_fexit_ret", 0, core::ptr::null_mut(), 0);
    for (i = 0; i < ARRAY_SIZE(detach_tests); i++) {
    char test_name[50];
    snprintf(test_name, sizeof(test_name), "%s_detach", detach_tests[i]);
    if (!test__start_subtest(test_name))
    continue;
    test_module_attach_detach(detach_tests[i]);
    }
    }
