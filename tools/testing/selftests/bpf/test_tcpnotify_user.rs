//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/test_tcpnotify_user.c
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

    pthread_t tid;
    static bool exit_thread;
    int rx_callbacks;
#[no_mangle]
unsafe extern "C" fn dummyfn(ctx: *mut c_void, cpu: c_int, data: *mut c_void, size: __u32) {
    static void dummyfn(void *ctx, int cpu, void *data, __u32 size)
    {
    struct tcp_notifier *t = data;
    if (t.type != 0xde || t.subtype != 0xad ||
    t.source != 0xbe || t.hash != 0xef)
    return;
    rx_callbacks++;
    }
#[no_mangle]
pub unsafe extern "C" fn tcp_notifier_poller(pb: *mut perf_buffer) {
    void tcp_notifier_poller(struct perf_buffer *pb)
    {
    int err;
    while (!exit_thread) {
    err = perf_buffer__poll(pb, 100);
    if (err < 0 && err != -EINTR) {
    printf("failed perf_buffer__poll: %d\n", err);
    return;
    }
    }
    }
    static void *poller_thread(void *arg)
    {
    struct perf_buffer *pb = arg;
    tcp_notifier_poller(pb);
    return arg;
    }
#[no_mangle]
pub unsafe extern "C" fn verify_result(result: *const tcpnotify_globals) -> c_int {
    int verify_result(const struct tcpnotify_globals *result)
    {
    return (result.ncalls > 0 && result.ncalls == rx_callbacks ? 0 : 1);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *file = "test_tcpnotify_kern.bpf.o";
    struct bpf_map *perf_map, *global_map;
    let mut g: tcpnotify_globals = {0};
    struct perf_buffer *pb = core::ptr::null_mut();
    const char *cg_path = "/foo";
    int prog_fd, rv, cg_fd = -1;
    let mut error: c_int = EXIT_FAILURE;
    struct bpf_object *obj;
    char test_script[80];
    let mut key: __u32 = 0;
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    cg_fd = cgroup_setup_and_join(cg_path);
    if (cg_fd < 0)
    goto err;
    if (bpf_prog_test_load(file, BPF_PROG_TYPE_SOCK_OPS, &obj, &prog_fd)) {
    printf("FAILED: load_bpf_file failed for: %s\n", file);
    goto err;
    }
    rv = bpf_prog_attach(prog_fd, cg_fd, BPF_CGROUP_SOCK_OPS, 0);
    if (rv) {
    printf("FAILED: bpf_prog_attach: %d (%s)\n",
    error, strerror(errno));
    goto err;
    }
    perf_map = bpf_object__find_map_by_name(obj, "perf_event_map");
    if (!perf_map) {
    printf("FAIL:map '%s' not found\n", "perf_event_map");
    goto err;
    }
    global_map = bpf_object__find_map_by_name(obj, "global_map");
    if (!global_map) {
    printf("FAIL:map '%s' not found\n", "global_map");
    return -1;
    }
    pb = perf_buffer__new(bpf_map__fd(perf_map), 8, dummyfn, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (!pb)
    goto err;
    pthread_create(&tid, core::ptr::null_mut(), poller_thread, pb);
    sprintf(test_script,
    "iptables -A INPUT -p tcp --dport %d -j DROP",
    TESTPORT);
    if (system(test_script)) {
    printf("FAILED: execute command: %s, err %d\n", test_script, -errno);
    goto err;
    }
    sprintf(test_script,
    "nc 127.0.0.1 %d < /etc/passwd > /dev/null 2>&1 ",
    TESTPORT);
    if (system(test_script))
    printf("execute command: %s, err %d\n", test_script, -errno);
    sprintf(test_script,
    "iptables -D INPUT -p tcp --dport %d -j DROP",
    TESTPORT);
    if (system(test_script)) {
    printf("FAILED: execute command: %s, err %d\n", test_script, -errno);
    goto err;
    }
    rv = bpf_map_lookup_elem(bpf_map__fd(global_map), &key, &g);
    if (rv != 0) {
    printf("FAILED: bpf_map_lookup_elem returns %d\n", rv);
    goto err;
    }
    sleep(10);
    exit_thread = true;
    let mut ret: c_int = pthread_join(tid, core::ptr::null_mut());
    if (ret) {
    printf("FAILED: pthread_join\n");
    goto err;
    }
    if (verify_result(&g)) {
    printf("FAILED: Wrong stats Expected %d calls, got %d\n",
    g.ncalls, rx_callbacks);
    goto err;
    }
    printf("PASSED!\n");
    error = 0;
    err:
    bpf_prog_detach(cg_fd, BPF_CGROUP_SOCK_OPS);
    close(cg_fd);
    cleanup_cgroup_environment();
    perf_buffer__free(pb);
    return error;
    }
