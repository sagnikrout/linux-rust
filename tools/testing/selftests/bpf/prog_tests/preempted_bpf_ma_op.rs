//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/preempted_bpf_ma_op.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd
// Macro flag: #define _GNU_SOURCE

pub const ALLOC_THREAD_NR: c_int = 4;
pub const ALLOC_LOOP_NR: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_ctx {
// output
    pub run_err: c_int,
// input
    pub fd: c_int,
    pub nomem_err: *mut bool,
}

    static void *run_alloc_prog(void *data)
    {
    struct alloc_ctx *ctx = data;
    cpu_set_t cpu_set;
    int i;
    CPU_ZERO(&cpu_set);
    CPU_SET(0, &cpu_set);
    pthread_setaffinity_np(pthread_self(), sizeof(cpu_set), &cpu_set);
    for (i = 0; i < ALLOC_LOOP_NR && !*ctx.nomem_err; i++) {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    int err;
    err = bpf_prog_test_run_opts(ctx.fd, &topts);
    ctx.run_err |= err | topts.retval;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_preempted_bpf_ma_op() {
    void test_preempted_bpf_ma_op(void)
    {
    struct alloc_ctx ctx[ALLOC_THREAD_NR];
    struct preempted_bpf_ma_op *skel;
    pthread_t tid[ALLOC_THREAD_NR];
    int i, err;
    skel = preempted_bpf_ma_op__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    err = preempted_bpf_ma_op__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto out;
    for (i = 0; i < ARRAY_SIZE(ctx); i++) {
    struct bpf_program *prog;
    char name[8];
    snprintf(name, sizeof(name), "test%d", i);
    prog = bpf_object__find_program_by_name(skel.obj, name);
    if (!ASSERT_OK_PTR(prog, "no test prog"))
    goto out;
    ctx[i].run_err = 0;
    ctx[i].fd = bpf_program__fd(prog);
    ctx[i].nomem_err = &skel.bss.nomem_err;
    }
    memset(tid, 0, sizeof(tid));
    for (i = 0; i < ARRAY_SIZE(tid); i++) {
    err = pthread_create(&tid[i], core::ptr::null_mut(), run_alloc_prog, &ctx[i]);
    if (!ASSERT_OK(err, "pthread_create"))
    break;
    }
    for (i = 0; i < ARRAY_SIZE(tid); i++) {
    if (!tid[i])
    break;
    pthread_join(tid[i], core::ptr::null_mut());
    ASSERT_EQ(ctx[i].run_err, 0, "run prog err");
    }
    ASSERT_FALSE(skel.bss.nomem_err, "ENOMEM");
    out:
    preempted_bpf_ma_op__destroy(skel);
    }
