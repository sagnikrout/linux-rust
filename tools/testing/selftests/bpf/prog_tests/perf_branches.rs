//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/perf_branches.c
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
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn check_good_sample(skel: *mut test_perf_branches) {
    static void check_good_sample(struct test_perf_branches *skel)
    {
    let mut written_global: c_int = skel.bss.written_global_out;
    let mut required_size: c_int = skel.bss.required_size_out;
    let mut written_stack: c_int = skel.bss.written_stack_out;
    let mut pbe_size: c_int = sizeof(struct perf_branch_entry);
    let mut duration: c_int = 0;
    if (CHECK(!skel.bss.run_cnt, "invalid run_cnt",
    "checked sample validity before prog run"))
    return;
    if (CHECK(!skel.bss.valid, "output not valid",
    "no valid sample from prog"))
    return;
//
// It's hard to validate the contents of the branch entries b/c it
// would require some kind of disassembler and also encoding the
// valid jump instructions for supported architectures. So just check
// the easy stuff for now.
//
    CHECK(required_size <= 0, "read_branches_size", "err %d\n", required_size);
    CHECK(written_stack < 0, "read_branches_stack", "err %d\n", written_stack);
    CHECK(written_stack % pbe_size != 0, "read_branches_stack",
    "stack bytes written=%d not multiple of struct size=%d\n",
    written_stack, pbe_size);
    CHECK(written_global < 0, "read_branches_global", "err %d\n", written_global);
    CHECK(written_global % pbe_size != 0, "read_branches_global",
    "global bytes written=%d not multiple of struct size=%d\n",
    written_global, pbe_size);
    CHECK(written_global < written_stack, "read_branches_size",
    "written_global=%d < written_stack=%d\n", written_global, written_stack);
    }
#[no_mangle]
unsafe extern "C" fn check_bad_sample(skel: *mut test_perf_branches) {
    static void check_bad_sample(struct test_perf_branches *skel)
    {
    let mut written_global: c_int = skel.bss.written_global_out;
    let mut required_size: c_int = skel.bss.required_size_out;
    let mut written_stack: c_int = skel.bss.written_stack_out;
    let mut duration: c_int = 0;
    if (CHECK(!skel.bss.run_cnt, "invalid run_cnt",
    "checked sample validity before prog run"))
    return;
    if (CHECK(!skel.bss.valid, "output not valid",
    "no valid sample from prog"))
    return;
    CHECK((required_size != -EINVAL && required_size != -ENOENT),
    "read_branches_size", "err %d\n", required_size);
    CHECK((written_stack != -EINVAL && written_stack != -ENOENT),
    "read_branches_stack", "written %d\n", written_stack);
    CHECK((written_global != -EINVAL && written_global != -ENOENT),
    "read_branches_global", "written %d\n", written_global);
    }
    static void test_perf_branches_common(int perf_fd,
    void (*cb)(struct test_perf_branches *))
    {
    struct test_perf_branches *skel;
    int err, i, duration = 0;
    let mut detached: bool = false;
    struct bpf_link *link;
    let mut j: volatile int = 0;
    cpu_set_t cpu_set;
    skel = test_perf_branches__open_and_load();
    if (CHECK(!skel, "test_perf_branches_load",
    "perf_branches skeleton failed\n"))
    return;
// attach perf_event
    link = bpf_program__attach_perf_event(skel.progs.perf_branches, perf_fd);
    if (!ASSERT_OK_PTR(link, "attach_perf_event"))
    goto out_destroy_skel;
// generate some branches on cpu 0
    CPU_ZERO(&cpu_set);
    CPU_SET(0, &cpu_set);
    err = pthread_setaffinity_np(pthread_self(), sizeof(cpu_set), &cpu_set);
    if (CHECK(err, "set_affinity", "cpu #0, err %d\n", err))
    goto out_destroy;
// Spin the loop for a while by using a high iteration count, and by
// checking whether the specific run count marker has been explicitly
// incremented at least once by the backing perf_event BPF program.
//
    for (i = 0; i < 100000000 && !*(volatile int *)&skel.bss.run_cnt; ++i)
    ++j;
    test_perf_branches__detach(skel);
    detached = true;
    cb(skel);
    out_destroy:
    bpf_link__destroy(link);
    out_destroy_skel:
    if (!detached)
    test_perf_branches__detach(skel);
    test_perf_branches__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_perf_branches_hw() {
    static void test_perf_branches_hw(void)
    {
    let mut attr: perf_event_attr = {0};
    let mut duration: c_int = 0;
    int pfd;
// create perf event
    attr.size = sizeof(attr);
    attr.type = PERF_TYPE_HARDWARE;
    attr.config = PERF_COUNT_HW_CPU_CYCLES;
    attr.freq = 1;
    attr.sample_freq = 1000;
    attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    attr.branch_sample_type = PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_ANY;
    pfd = syscall(__NR_perf_event_open, &attr, -1, 0, -1, PERF_FLAG_FD_CLOEXEC);
//
// Some setups don't support LBR (virtual machines, !x86, AMD Milan Zen
// 3 which only supports BRS), so skip test in this case.
//
    if (pfd < 0) {
    if (errno == ENOENT || errno == EOPNOTSUPP || errno == EINVAL) {
    printf("%s:SKIP:no PERF_SAMPLE_BRANCH_STACK\n",
    __func__);
    test__skip();
    return;
    }
    if (CHECK(pfd < 0, "perf_event_open", "err %d errno %d\n",
    pfd, errno))
    return;
    }
    test_perf_branches_common(pfd, check_good_sample);
    close(pfd);
    }
//
// Tests negative case -- run bpf_read_branch_records() on improperly configured
// perf event.
//
#[no_mangle]
unsafe extern "C" fn test_perf_branches_no_hw() {
    static void test_perf_branches_no_hw(void)
    {
    let mut attr: perf_event_attr = {0};
    let mut duration: c_int = 0;
    int pfd;
// create perf event
    attr.size = sizeof(attr);
    attr.type = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_CPU_CLOCK;
    attr.freq = 1;
    attr.sample_freq = 1000;
    pfd = syscall(__NR_perf_event_open, &attr, -1, 0, -1, PERF_FLAG_FD_CLOEXEC);
    if (CHECK(pfd < 0, "perf_event_open", "err %d\n", pfd))
    return;
    test_perf_branches_common(pfd, check_bad_sample);
    close(pfd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_perf_branches() {
    void test_perf_branches(void)
    {
    if (test__start_subtest("perf_branches_hw"))
    test_perf_branches_hw();
    if (test__start_subtest("perf_branches_no_hw"))
    test_perf_branches_no_hw();
    }
