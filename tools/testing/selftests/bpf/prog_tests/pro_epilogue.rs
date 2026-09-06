//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/pro_epilogue.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_ops_args {
    pub a: __u64,
}

#[no_mangle]
unsafe extern "C" fn test_tailcall() {
    static void test_tailcall(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    struct epilogue_tailcall *skel;
    struct st_ops_args args;
    int err, prog_fd;
    skel = epilogue_tailcall__open_and_load();
    if (!ASSERT_OK_PTR(skel, "epilogue_tailcall__open_and_load"))
    return;
    topts.ctx_in = &args;
    topts.ctx_size_in = sizeof(args);
    skel.links.epilogue_tailcall =
    bpf_map__attach_struct_ops(skel.maps.epilogue_tailcall);
    if (!ASSERT_OK_PTR(skel.links.epilogue_tailcall, "attach_struct_ops"))
    goto done;
// Both test_epilogue_tailcall and test_epilogue_subprog are
// patched with epilogue. When syscall_epilogue_tailcall()
// is run, test_epilogue_tailcall() is triggered.
// It executes a tail call and control is transferred to
// test_epilogue_subprog(). Only test_epilogue_subprog()
// does args->a += 1, thus final args.a value of 10001
// guarantees that only the epilogue of the
// test_epilogue_subprog is executed.
//
    memset(&args, 0, sizeof(args));
    prog_fd = bpf_program__fd(skel.progs.syscall_epilogue_tailcall);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_EQ(args.a, 10001, "args.a");
    ASSERT_EQ(topts.retval, 10001 * 2, "topts.retval");
    done:
    epilogue_tailcall__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_pro_epilogue() {
    void test_pro_epilogue(void)
    {
    RUN_TESTS(pro_epilogue);
    RUN_TESTS(pro_epilogue_goto_start);
    RUN_TESTS(epilogue_exit);
    RUN_TESTS(pro_epilogue_with_kfunc);
    if (test__start_subtest("tailcall"))
    test_tailcall();
    }
