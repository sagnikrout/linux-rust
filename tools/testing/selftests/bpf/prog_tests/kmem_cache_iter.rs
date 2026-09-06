//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kmem_cache_iter.c
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
// Copyright (c) 2024 Google

pub const SLAB_NAME_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache_result {
    pub name: [c_char; SLAB_NAME_MAX],
    pub obj_size: c_long,
}

#[no_mangle]
unsafe extern "C" fn subtest_kmem_cache_iter_check_task_struct(skel: *mut kmem_cache_iter) {
    static void subtest_kmem_cache_iter_check_task_struct(struct kmem_cache_iter *skel)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .flags = 0,  /* Run it with the current task */
    );
    let mut prog_fd: c_int = bpf_program__fd(skel.progs.check_task_struct);
// Get task_struct and check it if's from a slab cache
    ASSERT_OK(bpf_prog_test_run_opts(prog_fd, &opts), "prog_test_run");
// The BPF program should set 'found' variable
    ASSERT_EQ(skel.bss.task_struct_found, 1, "task_struct_found");
    }
#[no_mangle]
unsafe extern "C" fn subtest_kmem_cache_iter_check_slabinfo(skel: *mut kmem_cache_iter) {
    static void subtest_kmem_cache_iter_check_slabinfo(struct kmem_cache_iter *skel)
    {
    FILE *fp;
    int map_fd;
    char name[SLAB_NAME_MAX];
    unsigned long objsize;
    char rest_of_line[1000];
    struct kmem_cache_result r;
    let mut seen: c_int = 0;
    fp = fopen("/proc/slabinfo", "r");
    if (fp == core::ptr::null_mut()) {
// CONFIG_SLUB_DEBUG is not enabled
    return;
    }
    map_fd = bpf_map__fd(skel.maps.slab_result);
// Ignore first two lines for header
    fscanf(fp, "slabinfo - version: %*d.%*d\n");
    fscanf(fp, "# %*s %*s %*s %*s %*s %*s : %[^\n]\n", rest_of_line);
// Compare name and objsize only - others can be changes frequently
    while (fscanf(fp, "%s %*u %*u %lu %*u %*u : %[^\n]\n",
    name, &objsize, rest_of_line) == 3) {
    let mut ret: c_int = bpf_map_lookup_elem(map_fd, &seen, &r);
    if (!ASSERT_OK(ret, "kmem_cache_lookup"))
    break;
    ASSERT_STRNEQ(r.name, name, sizeof(r.name) - 1,
    "kmem_cache_name");
    ASSERT_EQ(r.obj_size, objsize, "kmem_cache_objsize");
    seen++;
    }
    ASSERT_EQ(skel.bss.kmem_cache_seen, seen, "kmem_cache_seen_eq");
    fclose(fp);
    }
#[no_mangle]
unsafe extern "C" fn subtest_kmem_cache_iter_open_coded(skel: *mut kmem_cache_iter) {
    static void subtest_kmem_cache_iter_open_coded(struct kmem_cache_iter *skel)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    int err, fd;
// No need to attach it, just run it directly
    fd = bpf_program__fd(skel.progs.open_coded_iter);
    err = bpf_prog_test_run_opts(fd, &topts);
    if (!ASSERT_OK(err, "test_run_opts err"))
    return;
    if (!ASSERT_OK(topts.retval, "test_run_opts retval"))
    return;
// It should be same as we've seen from the explicit iterator
    ASSERT_EQ(skel.bss.open_coded_seen, skel.bss.kmem_cache_seen, "open_code_seen_eq");
    }
#[no_mangle]
pub unsafe extern "C" fn test_kmem_cache_iter() {
    void test_kmem_cache_iter(void)
    {
    struct kmem_cache_iter *skel = core::ptr::null_mut();
    char buf[256];
    int iter_fd;
    skel = kmem_cache_iter__open_and_load();
    if (!ASSERT_OK_PTR(skel, "kmem_cache_iter__open_and_load"))
    return;
    if (!ASSERT_OK(kmem_cache_iter__attach(skel), "skel_attach"))
    goto destroy;
    iter_fd = bpf_iter_create(bpf_link__fd(skel.links.slab_info_collector));
    if (!ASSERT_GE(iter_fd, 0, "iter_create"))
    goto destroy;
    while (read(iter_fd, buf, sizeof(buf)) > 0)
    ; /* Read out all contents */
// Next reads should return 0
    ASSERT_EQ(read(iter_fd, buf, sizeof(buf)), 0, "read");
    if (test__start_subtest("check_task_struct"))
    subtest_kmem_cache_iter_check_task_struct(skel);
    if (test__start_subtest("check_slabinfo"))
    subtest_kmem_cache_iter_check_slabinfo(skel);
    if (test__start_subtest("open_coded_iter"))
    subtest_kmem_cache_iter_open_coded(skel);
    close(iter_fd);
    destroy:
    kmem_cache_iter__destroy(skel);
    }
