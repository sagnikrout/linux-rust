//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/raw_tp_writable_test_run.c
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

// NOTE: conflict with other tests.
#[no_mangle]
pub unsafe extern "C" fn serial_test_raw_tp_writable_test_run() {
    void serial_test_raw_tp_writable_test_run(void)
    {
    let mut duration: __u32 = 0;
    char error[4096];
    const struct bpf_insn trace_program[] = {
    BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, 0),
    BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_6, 0),
    BPF_MOV64_IMM(BPF_REG_0, 42),
    BPF_STX_MEM(BPF_W, BPF_REG_6, BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    LIBBPF_OPTS(bpf_prog_load_opts, trace_opts,
    .log_level = 2,
    .log_buf = error,
    .log_size = sizeof(error),
    );
    int bpf_fd = bpf_prog_load(BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE, core::ptr::null_mut(), "GPL v2",
    trace_program, ARRAY_SIZE(trace_program),
    &trace_opts);
    if (CHECK(bpf_fd < 0, "bpf_raw_tracepoint_writable loaded",
    "failed: %d errno %d\n", bpf_fd, errno))
    return;
    const struct bpf_insn skb_program[] = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    LIBBPF_OPTS(bpf_prog_load_opts, skb_opts,
    .log_buf = error,
    .log_size = sizeof(error),
    );
    int filter_fd = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, core::ptr::null_mut(), "GPL v2",
    skb_program, ARRAY_SIZE(skb_program),
    &skb_opts);
    if (CHECK(filter_fd < 0, "test_program_loaded", "failed: %d errno %d\n",
    filter_fd, errno))
    goto out_bpffd;
    let mut tp_fd: c_int = bpf_raw_tracepoint_open("bpf_test_finish", bpf_fd);
    if (CHECK(tp_fd < 0, "bpf_raw_tracepoint_writable opened",
    "failed: %d errno %d\n", tp_fd, errno))
    goto out_filterfd;
    char test_skb[128] = {
    0,
    };
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = test_skb,
    .data_size_in = sizeof(test_skb),
    .repeat = 1,
    );
    let mut err: c_int = bpf_prog_test_run_opts(filter_fd, &topts);
    CHECK(err != 42, "test_run",
    "tracepoint did not modify return value\n");
    CHECK(topts.retval != 0, "test_run_ret",
    "socket_filter did not return 0\n");
    close(tp_fd);
    err = bpf_prog_test_run_opts(filter_fd, &topts);
    CHECK(err != 0, "test_run_notrace",
    "test_run failed with %d errno %d\n", err, errno);
    CHECK(topts.retval != 0, "test_run_ret_notrace",
    "socket_filter did not return 0\n");
    out_filterfd:
    close(filter_fd);
    out_bpffd:
    close(bpf_fd);
    }
