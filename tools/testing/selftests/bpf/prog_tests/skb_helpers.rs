//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/skb_helpers.c
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
pub unsafe extern "C" fn test_skb_helpers() {
    void test_skb_helpers(void)
    {
    struct __sk_buff skb = {
    .wire_len = 100,
    .gso_segs = 8,
    .gso_size = 10,
    };
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .ctx_in = &skb,
    .ctx_size_in = sizeof(skb),
    .ctx_out = &skb,
    .ctx_size_out = sizeof(skb),
    );
    struct bpf_object *obj;
    int err, prog_fd;
    err = bpf_prog_test_load("./test_skb_helpers.bpf.o",
    BPF_PROG_TYPE_SCHED_CLS, &obj, &prog_fd);
    if (!ASSERT_OK(err, "load"))
    return;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    bpf_object__close(obj);
    }
