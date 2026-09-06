//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_ldsx_insn.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn test_map_val_and_probed_memory() {
    static void test_map_val_and_probed_memory(void)
    {
    struct test_ldsx_insn *skel;
    int err;
    skel = test_ldsx_insn__open();
    if (!ASSERT_OK_PTR(skel, "test_ldsx_insn__open"))
    return;
    if (skel.rodata.skip) {
    test__skip();
    goto out;
    }
    bpf_program__set_autoload(skel.progs.rdonly_map_prog, true);
    bpf_program__set_autoload(skel.progs.map_val_prog, true);
    bpf_program__set_autoload(skel.progs.test_ptr_struct_arg, true);
    err = test_ldsx_insn__load(skel);
    if (!ASSERT_OK(err, "test_ldsx_insn__load"))
    goto out;
    err = test_ldsx_insn__attach(skel);
    if (!ASSERT_OK(err, "test_ldsx_insn__attach"))
    goto out;
    ASSERT_OK(trigger_module_test_read(256), "trigger_read");
    ASSERT_EQ(skel.bss.done1, 1, "done1");
    ASSERT_EQ(skel.bss.ret1, 1, "ret1");
    ASSERT_EQ(skel.bss.done2, 1, "done2");
    ASSERT_EQ(skel.bss.ret2, 1, "ret2");
    ASSERT_EQ(skel.bss.int_member, -1, "int_member");
    out:
    test_ldsx_insn__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_ctx_member_sign_ext() {
    static void test_ctx_member_sign_ext(void)
    {
    struct test_ldsx_insn *skel;
    int err, fd, cgroup_fd;
    char buf[16] = {0};
    socklen_t optlen;
    cgroup_fd = test__join_cgroup("/ldsx_test");
    if (!ASSERT_GE(cgroup_fd, 0, "join_cgroup /ldsx_test"))
    return;
    skel = test_ldsx_insn__open();
    if (!ASSERT_OK_PTR(skel, "test_ldsx_insn__open"))
    goto close_cgroup_fd;
    if (skel.rodata.skip) {
    test__skip();
    goto destroy_skel;
    }
    bpf_program__set_autoload(skel.progs._getsockopt, true);
    err = test_ldsx_insn__load(skel);
    if (!ASSERT_OK(err, "test_ldsx_insn__load"))
    goto destroy_skel;
    skel.links._getsockopt =
    bpf_program__attach_cgroup(skel.progs._getsockopt, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links._getsockopt, "getsockopt_link"))
    goto destroy_skel;
    fd = socket(AF_INET, SOCK_STREAM, 0);
    if (!ASSERT_GE(fd, 0, "socket"))
    goto destroy_skel;
    optlen = sizeof(buf);
    (void)getsockopt(fd, SOL_IP, IP_TTL, buf, &optlen);
    ASSERT_EQ(skel.bss.set_optlen, -1, "optlen");
    ASSERT_EQ(skel.bss.set_retval, -1, "retval");
    close(fd);
    destroy_skel:
    test_ldsx_insn__destroy(skel);
    close_cgroup_fd:
    close(cgroup_fd);
    }
#[no_mangle]
unsafe extern "C" fn test_ctx_member_narrow_sign_ext() {
    static void test_ctx_member_narrow_sign_ext(void)
    {
    struct test_ldsx_insn *skel;
    let mut skb: __sk_buff = {};
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .ctx_in = &skb,
    .ctx_size_in = sizeof(skb),
    );
    int err, prog_fd;
    skel = test_ldsx_insn__open();
    if (!ASSERT_OK_PTR(skel, "test_ldsx_insn__open"))
    return;
    if (skel.rodata.skip) {
    test__skip();
    goto out;
    }
    bpf_program__set_autoload(skel.progs._tc, true);
    err = test_ldsx_insn__load(skel);
    if (!ASSERT_OK(err, "test_ldsx_insn__load"))
    goto out;
    prog_fd = bpf_program__fd(skel.progs._tc);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(skel.bss.set_mark, -2, "set_mark");
    out:
    test_ldsx_insn__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_ldsx_insn() {
    void test_ldsx_insn(void)
    {
    if (test__start_subtest("map_val and probed_memory"))
    test_map_val_and_probed_memory();
    if (test__start_subtest("ctx_member_sign_ext"))
    test_ctx_member_sign_ext();
    if (test__start_subtest("ctx_member_narrow_sign_ext"))
    test_ctx_member_narrow_sign_ext();
    }
