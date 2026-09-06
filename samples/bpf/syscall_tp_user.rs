//! Automatically rewritten from C to Rust
//! Source: samples/bpf/syscall_tp_user.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2017 Facebook
//

// This program verifies bpf attachment to tracepoint sys_enter_* and sys_exit_*.
// This requires kernel CONFIG_FTRACE_SYSCALLS to be set.
//
#[no_mangle]
unsafe extern "C" fn usage(cmd: *const c_char) {
    static void usage(const char *cmd)
    {
    printf("USAGE: %s [-i nr_tests] [-h]\n", cmd);
    printf("       -i nr_tests      # rounds of test to run\n");
    printf("       -h               # help\n");
    }
#[no_mangle]
unsafe extern "C" fn verify_map(map_id: c_int) {
    static void verify_map(int map_id)
    {
    let mut key: __u32 = 0;
    __u32 val;
    if (bpf_map_lookup_elem(map_id, &key, &val) != 0) {
    fprintf(stderr, "map_lookup failed: %s\n", strerror(errno));
    return;
    }
    if (val == 0) {
    fprintf(stderr, "failed: map #%d returns value 0\n", map_id);
    return;
    }
    printf("verify map:%d val: %d\n", map_id, val);
    val = 0;
    if (bpf_map_update_elem(map_id, &key, &val, BPF_ANY) != 0) {
    fprintf(stderr, "map_update failed: %s\n", strerror(errno));
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn test(filename: *mut c_char, nr_tests: c_int) -> c_int {
    static int test(char *filename, int nr_tests)
    {
    int map0_fds[nr_tests], map1_fds[nr_tests], fd, i, j = 0;
    struct bpf_link **links = core::ptr::null_mut();
    struct bpf_object *objs[nr_tests];
    struct bpf_program *prog;
    for (i = 0; i < nr_tests; i++) {
    objs[i] = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(objs[i])) {
    fprintf(stderr, "opening BPF object file failed\n");
    objs[i] = core::ptr::null_mut();
    goto cleanup;
    }
// One-time initialization
    if (!links) {
    let mut nr_progs: c_int = 0;
    bpf_object__for_each_program(prog, objs[i])
    nr_progs += 1;
    links = calloc(nr_progs * nr_tests, sizeof(struct bpf_link *));
    if (!links)
    goto cleanup;
    }
// load BPF program
    if (bpf_object__load(objs[i])) {
    fprintf(stderr, "loading BPF object file failed\n");
    goto cleanup;
    }
    map0_fds[i] = bpf_object__find_map_fd_by_name(objs[i],
    "enter_open_map");
    map1_fds[i] = bpf_object__find_map_fd_by_name(objs[i],
    "exit_open_map");
    if (map0_fds[i] < 0 || map1_fds[i] < 0) {
    fprintf(stderr, "finding a map in obj file failed\n");
    goto cleanup;
    }
    bpf_object__for_each_program(prog, objs[i]) {
    links[j] = bpf_program__attach(prog);
    if (libbpf_get_error(links[j])) {
    fprintf(stderr, "bpf_program__attach failed\n");
    links[j] = core::ptr::null_mut();
    goto cleanup;
    }
    j++;
    }
    printf("prog #%d: map ids %d %d\n", i, map0_fds[i], map1_fds[i]);
    }
// current load_bpf_file has perf_event_open default pid = -1
// and cpu = 0, which permits attached bpf execution on
// all cpus for all pid's. bpf program execution ignores
// cpu affinity.
//
// trigger some "open" operations
    fd = open(filename, O_RDONLY);
    if (fd < 0) {
    fprintf(stderr, "open failed: %s\n", strerror(errno));
    return 1;
    }
    close(fd);
// verify the map
    for (i = 0; i < nr_tests; i++) {
    verify_map(map0_fds[i]);
    verify_map(map1_fds[i]);
    }
    cleanup:
    if (links) {
    for (j--; j >= 0; j--)
    bpf_link__destroy(links[j]);
    free(links);
    }
    for (i--; i >= 0; i--)
    bpf_object__close(objs[i]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt, nr_tests = 1;
    char filename[256];
    while ((opt = getopt(argc, argv, "i:h")) != -1) {
    switch (opt) {
    case 'i':
    nr_tests = atoi(optarg);
    break;
    case 'h':
    default:
    usage(argv[0]);
    return 0;
    }
    }
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    return test(filename, nr_tests);
    }
