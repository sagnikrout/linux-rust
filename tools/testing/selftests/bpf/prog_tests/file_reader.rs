//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/file_reader.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    const char *user_ptr = "hello world";
    char file_contents[256000];
    void *addr;
    void *get_executable_base_addr(void)
    {
    Dl_info info;
    if (!dladdr((void *)&get_executable_base_addr, &info)) {
    fprintf(stderr, "dladdr failed\n");
    return core::ptr::null_mut();
    }
    return info.dli_fbase;
    }
#[no_mangle]
unsafe extern "C" fn initialize_file_contents() -> c_int {
    static int initialize_file_contents(void)
    {
    int fd, page_sz = sysconf(_SC_PAGESIZE);
    let mut n: isize = 0, cur;
    fd = open("/proc/self/exe", O_RDONLY);
    if (!ASSERT_OK_FD(fd, "Open /proc/self/exe\n"))
    return 1;
    do {
    cur = read(fd, file_contents + n, sizeof(file_contents) - n);
    if (!ASSERT_GT(cur, 0, "read success"))
    break;
    n += cur;
    } while (n < sizeof(file_contents));
    close(fd);
    if (!ASSERT_EQ(n, sizeof(file_contents), "Read /proc/self/exe\n"))
    return 1;
    addr = get_executable_base_addr();
    if (!ASSERT_NEQ(addr, core::ptr::null_mut(), "get executable address"))
    return 1;
// page-align base file address
    addr = (void *)((unsigned long)addr & ~(page_sz - 1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_test(prog_name: *const c_char) {
    static void run_test(const char *prog_name)
    {
    struct file_reader *skel;
    struct bpf_program *prog;
    int err, fd;
    err = initialize_file_contents();
    if (!ASSERT_OK(err, "initialize file contents"))
    return;
    skel = file_reader__open();
    if (!ASSERT_OK_PTR(skel, "file_reader__open"))
    return;
    bpf_object__for_each_program(prog, skel.obj) {
    bpf_program__set_autoload(prog, strcmp(bpf_program__name(prog), prog_name) == 0);
    }
    memcpy(skel.bss.user_buf, file_contents, sizeof(file_contents));
    skel.bss.pid = getpid();
    err = file_reader__load(skel);
    if (!ASSERT_OK(err, "file_reader__load"))
    goto cleanup;
//
// Page out range 0..512K, use 0..256K for positive tests and
// 256K..512K for negative tests expecting page faults
//
    if (!ASSERT_OK(madvise(addr, sizeof(file_contents) * 2, MADV_PAGEOUT),
    "madvise pageout"))
    goto cleanup;
    err = file_reader__attach(skel);
    if (!ASSERT_OK(err, "file_reader__attach"))
    goto cleanup;
    fd = open("/proc/self/exe", O_RDONLY);
    if (fd >= 0)
    close(fd);
    ASSERT_EQ(skel.bss.err, 0, "err");
    ASSERT_EQ(skel.bss.run_success, 1, "run_success");
    cleanup:
    file_reader__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_file_reader() {
    void test_file_reader(void)
    {
    if (test__start_subtest("on_open_expect_fault"))
    run_test("on_open_expect_fault");
    if (test__start_subtest("on_open_validate_file_read"))
    run_test("on_open_validate_file_read");
    if (test__start_subtest("negative"))
    RUN_TESTS(file_reader_fail);
    }
