//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/map_lock.c
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

    static void *spin_lock_thread(void *arg)
    {
    int err, prog_fd = *(u32 *) arg;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 10000,
    );
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run_opts err");
    ASSERT_OK(topts.retval, "test_run_opts retval");
    pthread_exit(arg);
    }
    static void *parallel_map_access(void *arg)
    {
    int err, map_fd = *(u32 *) arg;
    int vars[17], i, j, rnd, key = 0;
    for (i = 0; i < 10000; i++) {
    err = bpf_map_lookup_elem_flags(map_fd, &key, vars, BPF_F_LOCK);
    if (CHECK_FAIL(err)) {
    printf("lookup failed\n");
    goto out;
    }
    if (CHECK_FAIL(vars[0] != 0)) {
    printf("lookup #%d var[0]=%d\n", i, vars[0]);
    goto out;
    }
    rnd = vars[1];
    for (j = 2; j < 17; j++) {
    if (vars[j] == rnd)
    continue;
    printf("lookup #%d var[1]=%d var[%d]=%d\n",
    i, rnd, j, vars[j]);
    CHECK_FAIL(vars[j] != rnd);
    goto out;
    }
    }
    out:
    pthread_exit(arg);
    }
#[no_mangle]
pub unsafe extern "C" fn test_map_lock() {
    void test_map_lock(void)
    {
    const char *file = "./test_map_lock.bpf.o";
    int prog_fd, map_fd[2], vars[17] = {};
    pthread_t thread_id[6];
    struct bpf_object *obj = core::ptr::null_mut();
    let mut err: c_int = 0, key = 0, i;
    void *ret;
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_CGROUP_SKB, &obj, &prog_fd);
    if (CHECK_FAIL(err)) {
    printf("test_map_lock:bpf_prog_test_load errno %d\n", errno);
    goto close_prog;
    }
    map_fd[0] = bpf_find_map(__func__, obj, "hash_map");
    if (CHECK_FAIL(map_fd[0] < 0))
    goto close_prog;
    map_fd[1] = bpf_find_map(__func__, obj, "array_map");
    if (CHECK_FAIL(map_fd[1] < 0))
    goto close_prog;
    bpf_map_update_elem(map_fd[0], &key, vars, BPF_F_LOCK);
    for (i = 0; i < 4; i++)
    if (CHECK_FAIL(pthread_create(&thread_id[i], core::ptr::null_mut(),
    &spin_lock_thread, &prog_fd)))
    goto close_prog;
    for (i = 4; i < 6; i++)
    if (CHECK_FAIL(pthread_create(&thread_id[i], core::ptr::null_mut(),
    &parallel_map_access,
    &map_fd[i - 4])))
    goto close_prog;
    for (i = 0; i < 4; i++)
    if (CHECK_FAIL(pthread_join(thread_id[i], &ret) ||
    ret != (void *)&prog_fd))
    goto close_prog;
    for (i = 4; i < 6; i++)
    if (CHECK_FAIL(pthread_join(thread_id[i], &ret) ||
    ret != (void *)&map_fd[i - 4]))
    goto close_prog;
    close_prog:
    bpf_object__close(obj);
    }
