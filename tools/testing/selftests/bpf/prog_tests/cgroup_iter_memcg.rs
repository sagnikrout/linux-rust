//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_iter_memcg.c
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

//
// memcg stats are cached per-cpu and only become visible once the periodic
// flusher runs (FLUSH_TIME, 2s), or once pending updates cross
// MEMCG_CHARGE_BATCH * num_online_cpus(). That threshold grows with the CPU
// count, so on a large machine a single pass does not reach it and
// bpf_mem_cgroup_flush_stats() returns without flushing anything. Retry for
// long enough to cover a flusher cycle.
//
pub const MEMCG_STAT_RETRIES: c_int = 16;

#[no_mangle]
unsafe extern "C" fn read_stats(link: *mut bpf_link) -> c_int {
    static int read_stats(struct bpf_link *link)
    {
    int fd, ret = 0;
    ssize_t bytes;
    fd = bpf_iter_create(bpf_link__fd(link));
    if (!ASSERT_OK_FD(fd, "bpf_iter_create"))
    return 1;
//
// Invoke iter program by reading from its fd. We're not expecting any
// data to be written by the bpf program so the result should be zero.
// Results will be read directly through the custom data section
// accessible through skel->data_query.memcg_query.
//
    bytes = read(fd, core::ptr::null_mut(), 0);
    if (!ASSERT_EQ(bytes, 0, "read fd"))
    ret = 1;
    close(fd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_anon(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    static void test_anon(struct bpf_link *link, struct memcg_query *memcg_query)
    {
    let mut retries: c_int = 0;
    void *map;
    size_t len;
    len = sysconf(_SC_PAGESIZE) * 1024;
    retry:
//
// Increase memcg anon usage by mapping and writing
// to a new anon region.
//
    map = mmap(core::ptr::null_mut(), len, PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (!ASSERT_NEQ(map, MAP_FAILED, "mmap anon"))
    return;
    memset(map, 1, len);
    if (!ASSERT_OK(read_stats(link), "read stats"))
    goto cleanup;
    if (!memcg_query.nr_anon_mapped && ++retries < MEMCG_STAT_RETRIES) {
    usleep(MEMCG_STAT_RETRY_DELAY_US);
    munmap(map, len);
    goto retry;
    }
    ASSERT_GT(memcg_query.nr_anon_mapped, 0, "final anon mapped val");
    cleanup:
    munmap(map, len);
    }
#[no_mangle]
unsafe extern "C" fn test_file(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    static void test_file(struct bpf_link *link, struct memcg_query *memcg_query)
    {
    let mut retries: c_int = 0;
    void *map;
    size_t len;
    char *path;
    int fd;
    len = sysconf(_SC_PAGESIZE) * 1024;
    path = "/tmp/test_cgroup_iter_memcg";
//
// Increase memcg file usage by creating and writing
// to a mapped file.
//
    fd = open(path, O_CREAT | O_RDWR, 0644);
    if (!ASSERT_OK_FD(fd, "open fd"))
    return;
    retry:
    if (!ASSERT_OK(ftruncate(fd, len), "ftruncate"))
    goto cleanup_fd;
    map = mmap(core::ptr::null_mut(), len, PROT_WRITE, MAP_SHARED, fd, 0);
    if (!ASSERT_NEQ(map, MAP_FAILED, "mmap file"))
    goto cleanup_fd;
    memset(map, 1, len);
    if (!ASSERT_OK(read_stats(link), "read stats"))
    goto cleanup_map;
    if ((!memcg_query.nr_file_pages || !memcg_query.nr_file_mapped) &&
    ++retries < MEMCG_STAT_RETRIES) {
    usleep(MEMCG_STAT_RETRY_DELAY_US);
    munmap(map, len);
    goto retry;
    }
    ASSERT_GT(memcg_query.nr_file_pages, 0, "final file value");
    ASSERT_GT(memcg_query.nr_file_mapped, 0, "final file mapped value");
    cleanup_map:
    munmap(map, len);
    cleanup_fd:
    close(fd);
    unlink(path);
    }
#[no_mangle]
unsafe extern "C" fn test_shmem(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    static void test_shmem(struct bpf_link *link, struct memcg_query *memcg_query)
    {
    let mut retries: c_int = 0;
    size_t len;
    int fd;
    len = sysconf(_SC_PAGESIZE) * 1024;
//
// Increase memcg shmem usage by creating and writing
// to a memfd backed by shmem/tmpfs.
//
    fd = memfd_create("tmp_shmem", 0);
    if (!ASSERT_OK_FD(fd, "memfd_create"))
    return;
    retry:
    if (!ASSERT_OK(fallocate(fd, 0, 0, len), "fallocate"))
    goto cleanup;
    if (!ASSERT_OK(read_stats(link), "read stats"))
    goto cleanup;
    if (!memcg_query.nr_shmem && ++retries < MEMCG_STAT_RETRIES) {
    usleep(MEMCG_STAT_RETRY_DELAY_US);
    goto retry;
    }
    ASSERT_GT(memcg_query.nr_shmem, 0, "final shmem value");
    cleanup:
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn test_pgfault(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    static void test_pgfault(struct bpf_link *link, struct memcg_query *memcg_query)
    {
    let mut retries: c_int = 0;
    void *map;
    size_t len;
    len = sysconf(_SC_PAGESIZE) * 1024;
    retry:
// Create region to use for triggering a page fault.
    map = mmap(core::ptr::null_mut(), len, PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (!ASSERT_NEQ(map, MAP_FAILED, "mmap anon"))
    return;
// Trigger page fault.
    memset(map, 1, len);
    if (!ASSERT_OK(read_stats(link), "read stats"))
    goto cleanup;
    if (!memcg_query.pgfault && ++retries < MEMCG_STAT_RETRIES) {
    usleep(MEMCG_STAT_RETRY_DELAY_US);
    munmap(map, len);
    goto retry;
    }
    ASSERT_GT(memcg_query.pgfault, 0, "final pgfault val");
    cleanup:
    munmap(map, len);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_iter_memcg() {
    void test_cgroup_iter_memcg(void)
    {
    char *cgroup_rel_path = "/cgroup_iter_memcg_test";
    struct cgroup_iter_memcg *skel;
    struct bpf_link *link;
    int cgroup_fd;
    cgroup_fd = cgroup_setup_and_join(cgroup_rel_path);
    if (!ASSERT_OK_FD(cgroup_fd, "cgroup_setup_and_join"))
    return;
    skel = cgroup_iter_memcg__open_and_load();
    if (!ASSERT_OK_PTR(skel, "cgroup_iter_memcg__open_and_load"))
    goto cleanup_cgroup_fd;
    DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    union bpf_iter_link_info linfo = {
    .cgroup.cgroup_fd = cgroup_fd,
    .cgroup.order = BPF_CGROUP_ITER_SELF_ONLY,
    };
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    link = bpf_program__attach_iter(skel.progs.cgroup_memcg_query, &opts);
    if (!ASSERT_OK_PTR(link, "bpf_program__attach_iter"))
    goto cleanup_skel;
    if (test__start_subtest("cgroup_iter_memcg__anon"))
    test_anon(link, &skel.data_query.memcg_query);
    if (test__start_subtest("cgroup_iter_memcg__shmem"))
    test_shmem(link, &skel.data_query.memcg_query);
    if (test__start_subtest("cgroup_iter_memcg__file"))
    test_file(link, &skel.data_query.memcg_query);
    if (test__start_subtest("cgroup_iter_memcg__pgfault"))
    test_pgfault(link, &skel.data_query.memcg_query);
    bpf_link__destroy(link);
    cleanup_skel:
    cgroup_iter_memcg__destroy(skel);
    cleanup_cgroup_fd:
    close(cgroup_fd);
    cleanup_cgroup_environment();
    }
