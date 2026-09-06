//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/type_cast.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn test_xdp() {
    static void test_xdp(void)
    {
    struct type_cast *skel;
    int err, prog_fd;
    char buf[128];
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .data_out = buf,
    .data_size_out = sizeof(buf),
    .repeat = 1,
    );
    skel = type_cast__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    bpf_program__set_autoload(skel.progs.md_xdp, true);
    err = type_cast__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto out;
    prog_fd = bpf_program__fd(skel.progs.md_xdp);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, XDP_PASS, "xdp test_run retval");
    ASSERT_EQ(skel.bss.ifindex, 1, "xdp_md ifindex");
    ASSERT_EQ(skel.bss.ifindex, skel.bss.ingress_ifindex, "xdp_md ingress_ifindex");
    ASSERT_STREQ(skel.bss.name, "lo", "xdp_md name");
    ASSERT_NEQ(skel.bss.inum, 0, "xdp_md inum");
    out:
    type_cast__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_tc() {
    static void test_tc(void)
    {
    struct type_cast *skel;
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    skel = type_cast__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    bpf_program__set_autoload(skel.progs.md_skb, true);
    err = type_cast__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto out;
    prog_fd = bpf_program__fd(skel.progs.md_skb);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "tc test_run retval");
    ASSERT_EQ(skel.bss.meta_len, 0, "skb meta_len");
    ASSERT_EQ(skel.bss.frag0_len, 0, "skb frag0_len");
    ASSERT_NEQ(skel.bss.kskb_len, 0, "skb len");
    ASSERT_NEQ(skel.bss.kskb2_len, 0, "skb2 len");
    ASSERT_EQ(skel.bss.kskb_len, skel.bss.kskb2_len, "skb len compare");
    out:
    type_cast__destroy(skel);
    }
    static const char * const negative_tests[] = {
    "untrusted_ptr",
    "kctx_u64",
    };
#[no_mangle]
unsafe extern "C" fn test_negative() {
    static void test_negative(void)
    {
    struct bpf_program *prog;
    struct type_cast *skel;
    int i, err;
    for (i = 0; i < ARRAY_SIZE(negative_tests); i++) {
    skel = type_cast__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    prog = bpf_object__find_program_by_name(skel.obj, negative_tests[i]);
    if (!ASSERT_OK_PTR(prog, "bpf_object__find_program_by_name"))
    goto out;
    bpf_program__set_autoload(prog, true);
    err = type_cast__load(skel);
    ASSERT_ERR(err, "skel_load");
    out:
    type_cast__destroy(skel);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_type_cast() {
    void test_type_cast(void)
    {
    if (test__start_subtest("xdp"))
    test_xdp();
    if (test__start_subtest("tc"))
    test_tc();
    if (test__start_subtest("negative"))
    test_negative();
    }
