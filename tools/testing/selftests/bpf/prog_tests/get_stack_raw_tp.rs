//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/get_stack_raw_tp.c
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

pub const MAX_STACK_RAWTP: c_int = 100;
    let mut duration: static int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_stack_trace_t {
    pub pid: c_int,
    pub kern_stack_size: c_int,
    pub user_stack_size: c_int,
    pub user_stack_buildid_size: c_int,
    pub kern_stack: [__u64; MAX_STACK_RAWTP],
    pub user_stack: [__u64; MAX_STACK_RAWTP],
    pub user_stack_buildid: [bpf_stack_build_id; MAX_STACK_RAWTP],
}

#[no_mangle]
unsafe extern "C" fn get_stack_print_output(ctx: *mut c_void, cpu: c_int, data: *mut c_void, size: __u32) {
    static void get_stack_print_output(void *ctx, int cpu, void *data, __u32 size)
    {
    let mut good_kern_stack: bool = false, good_user_stack = false;
    const char *nonjit_func = "___bpf_prog_run";
// perfbuf-submitted data is 4-byte aligned, but we need 8-byte
// alignment, so copy data into a local variable, for simplicity
//
    struct get_stack_trace_t e;
    int i, num_stack;
    struct ksym *ks;
    memset(&e, 0, sizeof(e));
    memcpy(&e, data, size <= sizeof(e) ? size : sizeof(e));
    if (size < sizeof(struct get_stack_trace_t)) {
    __u64 *raw_data = data;
    let mut found: bool = false;
    num_stack = size / sizeof(__u64);
// If jit is enabled, we do not have a good way to
// verify the sanity of the kernel stack. So we
// just assume it is good if the stack is not empty.
// This could be improved in the future.
//
    if (env.jit_enabled) {
    found = num_stack > 0;
    } else {
    for (i = 0; i < num_stack; i++) {
    ks = ksym_search(raw_data[i]);
    if (ks && (strcmp(ks.name, nonjit_func) == 0)) {
    found = true;
    break;
    }
    }
    }
    if (found) {
    good_kern_stack = true;
    good_user_stack = true;
    }
    } else {
    num_stack = e.kern_stack_size / sizeof(__u64);
    if (env.jit_enabled) {
    good_kern_stack = num_stack > 0;
    } else {
    for (i = 0; i < num_stack; i++) {
    ks = ksym_search(e.kern_stack[i]);
    if (ks && (strcmp(ks.name, nonjit_func) == 0)) {
    good_kern_stack = true;
    break;
    }
    }
    }
    if (e.user_stack_size > 0 && e.user_stack_buildid_size > 0)
    good_user_stack = true;
    }
    if (!good_kern_stack)
    CHECK(!good_kern_stack, "kern_stack", "corrupted kernel stack\n");
    if (!good_user_stack)
    CHECK(!good_user_stack, "user_stack", "corrupted user stack\n");
    }
#[no_mangle]
pub unsafe extern "C" fn test_get_stack_raw_tp() {
    void test_get_stack_raw_tp(void)
    {
    const char *file = "./test_get_stack_rawtp.bpf.o";
    const char *file_err = "./test_get_stack_rawtp_err.bpf.o";
    const char *prog_name = "bpf_prog1";
    int i, err, prog_fd, exp_cnt = MAX_CNT_RAWTP;
    struct perf_buffer *pb = core::ptr::null_mut();
    struct bpf_link *link = core::ptr::null_mut();
    let mut tv: timespec = {0, 10};
    struct bpf_program *prog;
    struct bpf_object *obj;
    struct bpf_map *map;
    cpu_set_t cpu_set;
    err = bpf_prog_test_load(file_err, BPF_PROG_TYPE_RAW_TRACEPOINT, &obj, &prog_fd);
    if (CHECK(err >= 0, "prog_load raw tp", "err %d errno %d\n", err, errno))
    return;
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_RAW_TRACEPOINT, &obj, &prog_fd);
    if (CHECK(err, "prog_load raw tp", "err %d errno %d\n", err, errno))
    return;
    prog = bpf_object__find_program_by_name(obj, prog_name);
    if (CHECK(!prog, "find_probe", "prog '%s' not found\n", prog_name))
    goto close_prog;
    map = bpf_object__find_map_by_name(obj, "perfmap");
    if (CHECK(!map, "bpf_find_map", "not found\n"))
    goto close_prog;
    err = load_kallsyms();
    if (CHECK(err < 0, "load_kallsyms", "err %d errno %d\n", err, errno))
    goto close_prog;
    CPU_ZERO(&cpu_set);
    CPU_SET(0, &cpu_set);
    err = pthread_setaffinity_np(pthread_self(), sizeof(cpu_set), &cpu_set);
    if (CHECK(err, "set_affinity", "err %d, errno %d\n", err, errno))
    goto close_prog;
    link = bpf_program__attach_raw_tracepoint(prog, "sys_enter");
    if (!ASSERT_OK_PTR(link, "attach_raw_tp"))
    goto close_prog;
    pb = perf_buffer__new(bpf_map__fd(map), 8, get_stack_print_output,
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_OK_PTR(pb, "perf_buf__new"))
    goto close_prog;
// trigger some syscall action
    for (i = 0; i < MAX_CNT_RAWTP; i++)
    nanosleep(&tv, core::ptr::null_mut());
    while (exp_cnt > 0) {
    err = perf_buffer__poll(pb, 100);
    if (err < 0 && CHECK(err < 0, "pb__poll", "err %d\n", err))
    goto close_prog;
    exp_cnt -= err;
    }
    close_prog:
    bpf_link__destroy(link);
    perf_buffer__free(pb);
    bpf_object__close(obj);
    }
