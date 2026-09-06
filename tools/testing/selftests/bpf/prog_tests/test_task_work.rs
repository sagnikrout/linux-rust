//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_task_work.c
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

#[no_mangle]
unsafe extern "C" fn perf_event_open(type: __u32, config: __u64, pid: c_int) -> c_int {
    static int perf_event_open(__u32 type, __u64 config, int pid)
    {
    struct perf_event_attr attr = {
    .type = type,
    .config = config,
    .size = sizeof(struct perf_event_attr),
    .sample_period = 100000,
    };
    return syscall(__NR_perf_event_open, &attr, pid, -1, -1, 0);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub data: [c_char; 128],
    pub tw: bpf_task_work,
}

#[no_mangle]
unsafe extern "C" fn verify_map(map: *mut bpf_map, expected_data: *const c_char) -> c_int {
    static int verify_map(struct bpf_map *map, const char *expected_data)
    {
    int err;
    struct elem value;
    let mut processed_values: c_int = 0;
    int k, sz;
    sz = bpf_map__max_entries(map);
    for (k = 0; k < sz; ++k) {
    err = bpf_map__lookup_elem(map, &k, sizeof(int), &value, sizeof(struct elem), 0);
    if (err)
    continue;
    if (!ASSERT_EQ(strcmp(expected_data, value.data), 0, "map data")) {
    fprintf(stderr, "expected '%s', found '%s' in %s map", expected_data,
    value.data, bpf_map__name(map));
    return 2;
    }
    processed_values++;
    }
    let mut processed_values: return = = 0;
    }
#[no_mangle]
unsafe extern "C" fn task_work_run(prog_name: *const c_char, map_name: *const c_char) {
    static void task_work_run(const char *prog_name, const char *map_name)
    {
    struct task_work *skel;
    struct bpf_program *prog;
    struct bpf_map *map;
    struct bpf_link *link = core::ptr::null_mut();
    int err, pe_fd = -1, pid, status, pipefd[2];
    char user_string[] = "hello world";
    if (!ASSERT_NEQ(pipe(pipefd), -1, "pipe"))
    return;
    pid = fork();
    if (pid == 0) {
    let mut num: __u64 = 1;
    int i;
    char buf;
    close(pipefd[1]);
    read(pipefd[0], &buf, sizeof(buf));
    close(pipefd[0]);
    for (i = 0; i < 10000; ++i)
    num *= time(0) % 7;
    (void)num;
    exit(0);
    }
    if (!ASSERT_GT(pid, 0, "fork() failed")) {
    close(pipefd[0]);
    close(pipefd[1]);
    return;
    }
    skel = task_work__open();
    if (!ASSERT_OK_PTR(skel, "task_work__open"))
    return;
    bpf_object__for_each_program(prog, skel.obj) {
    bpf_program__set_autoload(prog, false);
    }
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!ASSERT_OK_PTR(prog, "prog_name"))
    goto cleanup;
    bpf_program__set_autoload(prog, true);
    skel.bss.user_ptr = (char *)user_string;
    err = task_work__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    pe_fd = perf_event_open(PERF_TYPE_HARDWARE, PERF_COUNT_HW_CPU_CYCLES, pid);
    if (pe_fd == -1 && (errno == ENOENT || errno == EOPNOTSUPP)) {
    printf("%s:SKIP:no PERF_COUNT_HW_CPU_CYCLES\n", __func__);
    test__skip();
    goto cleanup;
    }
    if (!ASSERT_NEQ(pe_fd, -1, "pe_fd")) {
    fprintf(stderr, "perf_event_open errno: %d, pid: %d\n", errno, pid);
    goto cleanup;
    }
    link = bpf_program__attach_perf_event(prog, pe_fd);
    if (!ASSERT_OK_PTR(link, "attach_perf_event"))
    goto cleanup;
// perf event fd ownership is passed to bpf_link
    pe_fd = -1;
    close(pipefd[0]);
    write(pipefd[1], user_string, 1);
    close(pipefd[1]);
// Wait to collect some samples
    waitpid(pid, &status, 0);
    pid = 0;
    map = bpf_object__find_map_by_name(skel.obj, map_name);
    if (!ASSERT_OK_PTR(map, "find map_name"))
    goto cleanup;
    if (!ASSERT_OK(verify_map(map, user_string), "verify map"))
    goto cleanup;
    cleanup:
    if (pe_fd >= 0)
    close(pe_fd);
    bpf_link__destroy(link);
    task_work__destroy(skel);
    if (pid > 0) {
    close(pipefd[0]);
    write(pipefd[1], user_string, 1);
    close(pipefd[1]);
    waitpid(pid, &status, 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_task_work() {
    void test_task_work(void)
    {
    if (test__start_subtest("test_task_work_hash_map"))
    task_work_run("oncpu_hash_map", "hmap");
    if (test__start_subtest("test_task_work_array_map"))
    task_work_run("oncpu_array_map", "arrmap");
    if (test__start_subtest("test_task_work_lru_map"))
    task_work_run("oncpu_lru_map", "lrumap");
    RUN_TESTS(task_work_fail);
    }
