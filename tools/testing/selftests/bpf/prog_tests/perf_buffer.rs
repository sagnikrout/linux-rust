//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/perf_buffer.c
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

    static int duration;
// AddressSanitizer sometimes crashes due to data dereference below, due to
// this being mmap()'ed memory. Disable instrumentation with
// no_sanitize_address attribute
//
    __attribute__((no_sanitize_address))
#[no_mangle]
unsafe extern "C" fn on_sample(ctx: *mut c_void, cpu: c_int, data: *mut c_void, size: __u32) {
    static void on_sample(void *ctx, int cpu, void *data, __u32 size)
    {
    let mut cpu_data: c_int = *(int *)data, duration = 0;
    cpu_set_t *cpu_seen = ctx;
    if (cpu_data != cpu)
    CHECK(cpu_data != cpu, "check_cpu_data",
    "cpu_data %d != cpu %d\n", cpu_data, cpu);
    CPU_SET(cpu, cpu_seen);
    }
#[no_mangle]
pub unsafe extern "C" fn trigger_on_cpu(cpu: c_int) -> c_int {
    int trigger_on_cpu(int cpu)
    {
    cpu_set_t cpu_set;
    int err;
    CPU_ZERO(&cpu_set);
    CPU_SET(cpu, &cpu_set);
    err = pthread_setaffinity_np(pthread_self(), sizeof(cpu_set), &cpu_set);
    if (err && CHECK(err, "set_affinity", "cpu #%d, err %d\n", cpu, err))
    return err;
    usleep(1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_perf_buffer() {
    void serial_test_perf_buffer(void)
    {
    int err, on_len, nr_on_cpus = 0, nr_cpus, i, j;
    let mut zero: c_int = 0, my_pid = getpid();
    struct test_perf_buffer *skel;
    cpu_set_t cpu_seen;
    struct perf_buffer *pb;
    let mut last_fd: c_int = -1, fd;
    bool *online;
    nr_cpus = libbpf_num_possible_cpus();
    if (CHECK(nr_cpus < 0, "nr_cpus", "err %d\n", nr_cpus))
    return;
    err = parse_cpu_mask_file("/sys/devices/system/cpu/online",
    &online, &on_len);
    if (CHECK(err, "nr_on_cpus", "err %d\n", err))
    return;
    for (i = 0; i < on_len; i++)
    if (online[i])
    nr_on_cpus++;
// load program
    skel = test_perf_buffer__open_and_load();
    if (CHECK(!skel, "skel_load", "skeleton open/load failed\n"))
    goto out_close;
    err = bpf_map_update_elem(bpf_map__fd(skel.maps.my_pid_map), &zero, &my_pid, 0);
    if (!ASSERT_OK(err, "my_pid_update"))
    goto out_close;
// attach probe
    err = test_perf_buffer__attach(skel);
    if (CHECK(err, "attach_kprobe", "err %d\n", err))
    goto out_close;
// set up perf buffer
    pb = perf_buffer__new(bpf_map__fd(skel.maps.perf_buf_map), 1,
    on_sample, core::ptr::null_mut(), &cpu_seen, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(pb, "perf_buf__new"))
    goto out_close;
    CHECK(perf_buffer__epoll_fd(pb) < 0, "epoll_fd",
    "bad fd: %d\n", perf_buffer__epoll_fd(pb));
// trigger kprobe on every CPU
    CPU_ZERO(&cpu_seen);
    for (i = 0; i < nr_cpus; i++) {
    if (i >= on_len || !online[i]) {
    printf("skipping offline CPU #%d\n", i);
    continue;
    }
    if (trigger_on_cpu(i))
    goto out_close;
    }
// read perf buffer
    err = perf_buffer__poll(pb, 100);
    if (CHECK(err < 0, "perf_buffer__poll", "err %d\n", err))
    goto out_free_pb;
    if (CHECK(CPU_COUNT(&cpu_seen) != nr_on_cpus, "seen_cpu_cnt",
    "expect %d, seen %d\n", nr_on_cpus, CPU_COUNT(&cpu_seen)))
    goto out_free_pb;
    if (CHECK(perf_buffer__buffer_cnt(pb) != nr_on_cpus, "buf_cnt",
    "got %zu, expected %d\n", perf_buffer__buffer_cnt(pb), nr_on_cpus))
    goto out_close;
    for (i = 0, j = 0; i < nr_cpus; i++) {
    if (i >= on_len || !online[i])
    continue;
    fd = perf_buffer__buffer_fd(pb, j);
    CHECK(fd < 0 || last_fd == fd, "fd_check", "last fd %d == fd %d\n", last_fd, fd);
    last_fd = fd;
    err = perf_buffer__consume_buffer(pb, j);
    if (CHECK(err, "drain_buf", "cpu %d, err %d\n", i, err))
    goto out_close;
    CPU_CLR(i, &cpu_seen);
    if (trigger_on_cpu(i))
    goto out_close;
    err = perf_buffer__consume_buffer(pb, j);
    if (CHECK(err, "consume_buf", "cpu %d, err %d\n", j, err))
    goto out_close;
    if (CHECK(!CPU_ISSET(i, &cpu_seen), "cpu_seen", "cpu %d not seen\n", i))
    goto out_close;
    j++;
    }
    out_free_pb:
    perf_buffer__free(pb);
    out_close:
    test_perf_buffer__destroy(skel);
    free(online);
    }
