//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fexit_stress.c
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
// Copyright (c) 2019 Facebook

#[no_mangle]
pub unsafe extern "C" fn serial_test_fexit_stress() {
    void serial_test_fexit_stress(void)
    {
    int bpf_max_tramp_links, err, i;
    int *fd, *fexit_fd, *link_fd;
    bpf_max_tramp_links = get_bpf_max_tramp_links();
    if (!ASSERT_GE(bpf_max_tramp_links, 1, "bpf_max_tramp_links"))
    return;
    fd = calloc(bpf_max_tramp_links * 2, sizeof(*fd));
    if (!ASSERT_OK_PTR(fd, "fd"))
    return;
    fexit_fd = fd;
    link_fd = fd + bpf_max_tramp_links;
    const struct bpf_insn trace_program[] = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    LIBBPF_OPTS(bpf_prog_load_opts, trace_opts,
    .expected_attach_type = BPF_TRACE_FEXIT,
    );
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    err = libbpf_find_vmlinux_btf_id("bpf_fentry_test1",
    trace_opts.expected_attach_type);
    if (!ASSERT_GT(err, 0, "find_vmlinux_btf_id"))
    goto out;
    trace_opts.attach_btf_id = err;
    for (i = 0; i < bpf_max_tramp_links; i++) {
    fexit_fd[i] = bpf_prog_load(BPF_PROG_TYPE_TRACING, core::ptr::null_mut(), "GPL",
    trace_program,
    ARRAY_SIZE(trace_program),
    &trace_opts);
    if (!ASSERT_GE(fexit_fd[i], 0, "fexit load"))
    goto out;
    link_fd[i] = bpf_link_create(fexit_fd[i], 0, BPF_TRACE_FEXIT, core::ptr::null_mut());
    if (!ASSERT_GE(link_fd[i], 0, "fexit attach"))
    goto out;
    }
    err = bpf_prog_test_run_opts(fexit_fd[0], &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    out:
    for (i = 0; i < bpf_max_tramp_links; i++) {
    if (link_fd[i] > 0)
    close(link_fd[i]);
    if (fexit_fd[i] > 0)
    close(fexit_fd[i]);
    }
    free(fd);
    }
