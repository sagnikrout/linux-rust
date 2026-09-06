//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_skb_pkt_end.c
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
// Copyright (c) 2020 Facebook

#[no_mangle]
unsafe extern "C" fn sanity_run(prog: *mut bpf_program) -> c_int {
    static int sanity_run(struct bpf_program *prog)
    {
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    .flags = BPF_F_TEST_SKB_CHECKSUM_COMPLETE,
    );
    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (!ASSERT_OK(err, "test_run"))
    return -1;
    if (!ASSERT_EQ(topts.retval, 123, "test_run retval"))
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_skb_pkt_end() {
    void test_test_skb_pkt_end(void)
    {
    struct skb_pkt_end *skb_pkt_end_skel = core::ptr::null_mut();
    let mut duration: __u32 = 0;
    int err;
    skb_pkt_end_skel = skb_pkt_end__open_and_load();
    if (CHECK(!skb_pkt_end_skel, "skb_pkt_end_skel_load", "skb_pkt_end skeleton failed\n"))
    goto cleanup;
    err = skb_pkt_end__attach(skb_pkt_end_skel);
    if (CHECK(err, "skb_pkt_end_attach", "skb_pkt_end attach failed: %d\n", err))
    goto cleanup;
    if (sanity_run(skb_pkt_end_skel.progs.main_prog))
    goto cleanup;
    cleanup:
    skb_pkt_end__destroy(skb_pkt_end_skel);
    }
