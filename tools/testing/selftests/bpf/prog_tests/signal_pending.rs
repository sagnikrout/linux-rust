//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/signal_pending.c
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

    static void sigalrm_handler(int s) {}
    static struct sigaction sigalrm_action = {
    .sa_handler = sigalrm_handler,
    };
#[no_mangle]
unsafe extern "C" fn test_signal_pending_by_type(prog_type: enum bpf_prog_type) {
    static void test_signal_pending_by_type(enum bpf_prog_type prog_type)
    {
    struct bpf_insn prog[4096];
    struct itimerval timeo = {
    .it_value.tv_usec = 100000, /* 100ms */
    };
    int prog_fd;
    int err;
    int i;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 0xffffffff,
    );
    for (i = 0; i < ARRAY_SIZE(prog); i++)
    prog[i] = BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 0);
    prog[ARRAY_SIZE(prog) - 1] = BPF_EXIT_INSN();
    prog_fd = bpf_test_load_program(prog_type, prog, ARRAY_SIZE(prog),
    "GPL", 0, core::ptr::null_mut(), 0);
    ASSERT_GE(prog_fd, 0, "test-run load");
    err = sigaction(SIGALRM, &sigalrm_action, core::ptr::null_mut());
    ASSERT_OK(err, "test-run-signal-sigaction");
    err = setitimer(ITIMER_REAL, &timeo, core::ptr::null_mut());
    ASSERT_OK(err, "test-run-signal-timer");
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_LE(topts.duration, 500000000 /* 500ms */,
    "test-run-signal-duration");
    signal(SIGALRM, SIG_DFL);
    }
#[no_mangle]
pub unsafe extern "C" fn test_signal_pending() {
    void test_signal_pending(void)
    {
    test_signal_pending_by_type(BPF_PROG_TYPE_SOCKET_FILTER);
    test_signal_pending_by_type(BPF_PROG_TYPE_FLOW_DISSECTOR);
    }
