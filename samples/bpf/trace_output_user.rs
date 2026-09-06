//! Automatically rewritten from C to Rust
//! Source: samples/bpf/trace_output_user.c
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

#[no_mangle]
unsafe extern "C" fn time_get_ns() -> __u64 {
    static __u64 time_get_ns(void)
    {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000000000ull + ts.tv_nsec;
    }
    static __u64 start_time;
    static __u64 cnt;

#[no_mangle]
unsafe extern "C" fn print_bpf_output(ctx: *mut c_void, cpu: c_int, data: *mut c_void, size: __u32) {
    static void print_bpf_output(void *ctx, int cpu, void *data, __u32 size)
    {
    struct {
    __u64 pid;
    __u64 cookie;
    } *e = data;
    if (e.cookie != 0x12345678) {
    printf("BUG pid %llx cookie %llx sized %d\n",
    e.pid, e.cookie, size);
    return;
    }
    cnt++;
    if (cnt == MAX_CNT) {
    printf("recv %lld events per sec\n",
    MAX_CNT * 1000000000ll / (time_get_ns() - start_time));
    return;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct bpf_link *link = core::ptr::null_mut();
    struct bpf_program *prog;
    struct perf_buffer *pb;
    struct bpf_object *obj;
    int map_fd, ret = 0;
    char filename[256];
    FILE *f;
    snprintf(filename, sizeof(filename), "%s.bpf.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return 0;
    }
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    map_fd = bpf_object__find_map_fd_by_name(obj, "my_map");
    if (map_fd < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    prog = bpf_object__find_program_by_name(obj, "bpf_prog1");
    if (libbpf_get_error(prog)) {
    fprintf(stderr, "ERROR: finding a prog in obj file failed\n");
    goto cleanup;
    }
    link = bpf_program__attach(prog);
    if (libbpf_get_error(link)) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    link = core::ptr::null_mut();
    goto cleanup;
    }
    pb = perf_buffer__new(map_fd, 8, print_bpf_output, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    ret = libbpf_get_error(pb);
    if (ret) {
    printf("failed to setup perf_buffer: %d\n", ret);
    return 1;
    }
    f = popen("taskset 1 dd if=/dev/zero of=/dev/null", "r");
    (void) f;
    start_time = time_get_ns();
    while ((ret = perf_buffer__poll(pb, 1000)) >= 0 && cnt < MAX_CNT) {
    }
    kill(0, SIGINT);
    cleanup:
    bpf_link__destroy(link);
    bpf_object__close(obj);
    return ret;
    }
