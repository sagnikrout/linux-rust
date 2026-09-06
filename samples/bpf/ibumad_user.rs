//! Automatically rewritten from C to Rust
//! Source: samples/bpf/ibumad_user.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// ibumad BPF sample user side
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// Copyright(c) 2018 Ira Weiny, Intel Corporation
//

    static struct bpf_link *tp_links[3];
    static struct bpf_object *obj;
    static int map_fd[2];
    static int tp_cnt;
#[no_mangle]
unsafe extern "C" fn dump_counts(fd: c_int) {
    static void dump_counts(int fd)
    {
    __u32 key;
    __u64 value;
    for (key = 0; key < 256; key++) {
    if (bpf_map_lookup_elem(fd, &key, &value)) {
    printf("failed to read key %u\n", key);
    continue;
    }
    if (value)
    printf("0x%02x : %llu\n", key, value);
    }
    }
#[no_mangle]
unsafe extern "C" fn dump_all_counts() {
    static void dump_all_counts(void)
    {
    printf("Read 'Class : count'\n");
    dump_counts(map_fd[0]);
    printf("Write 'Class : count'\n");
    dump_counts(map_fd[1]);
    }
#[no_mangle]
unsafe extern "C" fn dump_exit(sig: c_int) {
    static void dump_exit(int sig)
    {
    dump_all_counts();
// Detach tracepoints
    while (tp_cnt)
    bpf_link__destroy(tp_links[--tp_cnt]);
    bpf_object__close(obj);
    exit(0);
    }
    static const struct option long_options[] = {
    {"help",      no_argument,       core::ptr::null_mut(), 'h'},
    {"delay",     required_argument, core::ptr::null_mut(), 'd'},
    };
#[no_mangle]
unsafe extern "C" fn usage(cmd: *mut c_char) {
    static void usage(char *cmd)
    {
    printf("eBPF test program to count packets from various IP addresses\n"
    "Usage: %s <options>\n"
    "       --help,   -h  this menu\n"
    "       --delay,  -d  <delay>  wait <delay> sec between prints [1 - 1000000]\n"
    , cmd
    );
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct bpf_program *prog;
    let mut delay: c_ulong = 5;
    char filename[256];
    let mut longindex: c_int = 0;
    int opt, err = -1;
    while ((opt = getopt_long(argc, argv, "hd:rSw",
    long_options, &longindex)) != -1) {
    switch (opt) {
    case 'd':
    delay = strtoul(optarg, core::ptr::null_mut(), 0);
    if (delay == ULONG_MAX || delay < 0 ||
    delay > 1000000) {
    fprintf(stderr, "ERROR: invalid delay : %s\n",
    optarg);
    usage(argv[0]);
    return 1;
    }
    break;
    default:
    case 'h':
    usage(argv[0]);
    return 1;
    }
    }
// Do one final dump when exiting
    signal(SIGINT, dump_exit);
    signal(SIGTERM, dump_exit);
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return err;
    }
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    map_fd[0] = bpf_object__find_map_fd_by_name(obj, "read_count");
    map_fd[1] = bpf_object__find_map_fd_by_name(obj, "write_count");
    if (map_fd[0] < 0 || map_fd[1] < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    bpf_object__for_each_program(prog, obj) {
    tp_links[tp_cnt] = bpf_program__attach(prog);
    if (libbpf_get_error(tp_links[tp_cnt])) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    tp_links[tp_cnt] = core::ptr::null_mut();
    goto cleanup;
    }
    tp_cnt++;
    }
    while (1) {
    sleep(delay);
    dump_all_counts();
    }
    err = 0;
    cleanup:
// Detach tracepoints
    while (tp_cnt)
    bpf_link__destroy(tp_links[--tp_cnt]);
    bpf_object__close(obj);
    return err;
    }
