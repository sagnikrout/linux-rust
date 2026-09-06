//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/pkt_access.c
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

#[no_mangle]
pub unsafe extern "C" fn test_pkt_access() {
    void test_pkt_access(void)
    {
    const char *file = "./test_pkt_access.bpf.o";
    struct bpf_object *obj;
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 100000,
    );
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_SCHED_CLS, &obj, &prog_fd);
    if (CHECK_FAIL(err))
    return;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "ipv4 test_run_opts err");
    ASSERT_OK(topts.retval, "ipv4 test_run_opts retval");
    topts.data_in = &pkt_v6;
    topts.data_size_in = sizeof(pkt_v6);
    topts.data_size_out = 0; /* reset from last call */
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "ipv6 test_run_opts err");
    ASSERT_OK(topts.retval, "ipv6 test_run_opts retval");
    bpf_object__close(obj);
    }
