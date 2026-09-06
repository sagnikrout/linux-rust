//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/find_vma.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
unsafe extern "C" fn test_and_reset_skel(skel: *mut find_vma, expected_find_zero_ret: c_int, need_test: bool) {
    static void test_and_reset_skel(struct find_vma *skel, int expected_find_zero_ret, bool need_test)
    {
    if (need_test) {
    ASSERT_EQ(skel.bss.found_vm_exec, 1, "found_vm_exec");
    ASSERT_EQ(skel.data.find_addr_ret, 0, "find_addr_ret");
    ASSERT_EQ(skel.data.find_zero_ret, expected_find_zero_ret, "find_zero_ret");
    ASSERT_OK_PTR(strstr(skel.bss.d_iname, "test_progs"), "find_test_progs");
    }
    skel.bss.found_vm_exec = 0;
    skel.data.find_addr_ret = -1;
    skel.data.find_zero_ret = -1;
    skel.bss.d_iname[0] = 0;
    }
#[no_mangle]
unsafe extern "C" fn open_pe() -> c_int {
    static int open_pe(void)
    {
    let mut attr: perf_event_attr = {0};
    int pfd;
// create perf event
    attr.size = sizeof(attr);
    attr.type = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_CPU_CLOCK;
    attr.freq = 1;
    attr.sample_freq = 1000;
    pfd = syscall(__NR_perf_event_open, &attr, 0, -1, -1, PERF_FLAG_FD_CLOEXEC);
    return pfd >= 0 ? pfd : -errno;
    }
#[no_mangle]
unsafe extern "C" fn find_vma_pe_condition(skel: *mut find_vma) -> bool {
    static bool find_vma_pe_condition(struct find_vma *skel)
    {
    return skel.bss.found_vm_exec == 0 ||
    skel.data.find_addr_ret != 0 ||
    skel.data.find_zero_ret == -1 ||
    strcmp(skel.bss.d_iname, "test_progs") != 0;
    }
#[no_mangle]
unsafe extern "C" fn test_find_vma_pe(skel: *mut find_vma) {
    static void test_find_vma_pe(struct find_vma *skel)
    {
    struct bpf_link *link = core::ptr::null_mut();
    let mut j: volatile int = 0;
    int pfd, i;
    let mut one_bn: c_int = 1000000000;
    pfd = open_pe();
    if (pfd < 0) {
    if (pfd == -ENOENT || pfd == -EOPNOTSUPP) {
    printf("%s:SKIP:no PERF_COUNT_HW_CPU_CYCLES\n", __func__);
    test__skip();
    goto cleanup;
    }
    if (!ASSERT_GE(pfd, 0, "perf_event_open"))
    goto cleanup;
    }
    link = bpf_program__attach_perf_event(skel.progs.handle_pe, pfd);
    if (!ASSERT_OK_PTR(link, "attach_perf_event"))
    goto cleanup;
    for (i = 0; i < one_bn && find_vma_pe_condition(skel); ++i)
    ++j;
    test_and_reset_skel(skel, -EBUSY /* in nmi, irq_work is busy */, i == one_bn);
    cleanup:
    bpf_link__destroy(link);
    close(pfd);
    }
#[no_mangle]
unsafe extern "C" fn test_find_vma_kprobe(skel: *mut find_vma) {
    static void test_find_vma_kprobe(struct find_vma *skel)
    {
    int err;
    err = find_vma__attach(skel);
    if (!ASSERT_OK(err, "get_branch_snapshot__attach"))
    return;
    getpgid(skel.bss.target_pid);
    test_and_reset_skel(skel, -ENOENT /* could not find vma for ptr 0 */, true);
    }
#[no_mangle]
unsafe extern "C" fn test_illegal_write_vma() {
    static void test_illegal_write_vma(void)
    {
    struct find_vma_fail1 *skel;
    skel = find_vma_fail1__open_and_load();
    if (!ASSERT_ERR_PTR(skel, "find_vma_fail1__open_and_load"))
    find_vma_fail1__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_illegal_write_task() {
    static void test_illegal_write_task(void)
    {
    struct find_vma_fail2 *skel;
    skel = find_vma_fail2__open_and_load();
    if (!ASSERT_ERR_PTR(skel, "find_vma_fail2__open_and_load"))
    find_vma_fail2__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_find_vma() {
    void serial_test_find_vma(void)
    {
    struct find_vma *skel;
    skel = find_vma__open_and_load();
    if (!ASSERT_OK_PTR(skel, "find_vma__open_and_load"))
    return;
    skel.bss.target_pid = getpid();
    skel.bss.addr = (__u64)(uintptr_t)test_find_vma_pe;
    test_find_vma_pe(skel);
    test_find_vma_kprobe(skel);
    find_vma__destroy(skel);
    test_illegal_write_vma();
    test_illegal_write_task();
    }
