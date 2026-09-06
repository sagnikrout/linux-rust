//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/get_branch_snapshot.c
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

    static int *pfd_array;
    static int cpu_cnt;
#[no_mangle]
unsafe extern "C" fn is_hypervisor() -> bool {
    static bool is_hypervisor(void)
    {
    char *line = core::ptr::null_mut();
    let mut ret: bool = false;
    size_t len;
    FILE *fp;
    fp = fopen("/proc/cpuinfo", "r");
    if (!fp)
    return false;
    while (getline(&line, &len, fp) != -1) {
    if (!strncmp(line, "flags", 5)) {
    if (strstr(line, "hypervisor") != core::ptr::null_mut())
    ret = true;
    break;
    }
    }
    free(line);
    fclose(fp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn create_perf_events() -> c_int {
    static int create_perf_events(void)
    {
    let mut attr: perf_event_attr = {0};
    int cpu;
// create perf event
    attr.size = sizeof(attr);
    attr.type = PERF_TYPE_HARDWARE;
    attr.config = PERF_COUNT_HW_CPU_CYCLES;
    attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    attr.branch_sample_type = PERF_SAMPLE_BRANCH_KERNEL |
    PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_ANY;
    cpu_cnt = libbpf_num_possible_cpus();
    pfd_array = malloc(sizeof(int) * cpu_cnt);
    if (!pfd_array) {
    cpu_cnt = 0;
    return 1;
    }
    for (cpu = 0; cpu < cpu_cnt; cpu++) {
    pfd_array[cpu] = syscall(__NR_perf_event_open, &attr,
    -1, cpu, -1, PERF_FLAG_FD_CLOEXEC);
    if (pfd_array[cpu] < 0)
    break;
    }
    let mut cpu: return = = 0;
    }
#[no_mangle]
unsafe extern "C" fn close_perf_events() {
    static void close_perf_events(void)
    {
    int cpu, fd;
    for (cpu = 0; cpu < cpu_cnt; cpu++) {
    fd = pfd_array[cpu];
    if (fd < 0)
    break;
    close(fd);
    }
    free(pfd_array);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_get_branch_snapshot() {
    void serial_test_get_branch_snapshot(void)
    {
    struct get_branch_snapshot *skel = core::ptr::null_mut();
    int err;
// Skip the test before we fix LBR snapshot for hypervisor.
    if (is_hypervisor()) {
    test__skip();
    return;
    }
    if (create_perf_events()) {
    test__skip();  /* system doesn't support LBR */
    goto cleanup;
    }
    skel = get_branch_snapshot__open_and_load();
    if (!ASSERT_OK_PTR(skel, "get_branch_snapshot__open_and_load"))
    goto cleanup;
    err = kallsyms_find("bpf_testmod_loop_test", &skel.bss.address_low);
    if (!ASSERT_OK(err, "kallsyms_find"))
    goto cleanup;
// Just a guess for the end of this function, as module functions
// in /proc/kallsyms could come in any order.
//
    skel.bss.address_high = skel.bss.address_low + 128;
    err = get_branch_snapshot__attach(skel);
    if (!ASSERT_OK(err, "get_branch_snapshot__attach"))
    goto cleanup;
    trigger_module_test_read(100);
    if (skel.bss.total_entries < 16) {
// too few entries for the hit/waste test
    test__skip();
    goto cleanup;
    }
    ASSERT_GT(skel.bss.test1_hits, 6, "find_looptest_in_lbr");
// Given we stop LBR in software, we will waste a few entries.
// But we should try to waste as few as possible entries. We are at
// about 7 on x86_64 systems.
// Add a check for < 10 so that we get heads-up when something
// changes and wastes too many entries.
//
    ASSERT_LT(skel.bss.wasted_entries, 10, "check_wasted_entries");
    cleanup:
    get_branch_snapshot__destroy(skel);
    close_perf_events();
    }
