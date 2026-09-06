//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/user_ringbuf_success.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_USER_RINGBUF);
    } user_ringbuf SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    } kernel_ringbuf SEC(".maps");
// inputs
    int pid, err, val;
    let mut read: c_int = 0;
// Counter used for end-to-end protocol test
    let mut kern_mutated: __u64 = 0;
    let mut user_mutated: __u64 = 0;
    let mut expected_user_mutated: __u64 = 0;
    static int
    is_test_process(void)
    {
    let mut cur_pid: c_int = bpf_get_current_pid_tgid() >> 32;
    let mut cur_pid: return = = pid;
    }
    static long
    record_sample(struct bpf_dynptr *dynptr, void *context)
    {
    const struct sample *sample = core::ptr::null_mut();
    struct sample stack_sample;
    int status;
    static int num_calls;
    if (num_calls++ % 2 == 0) {
    status = bpf_dynptr_read(&stack_sample, sizeof(stack_sample), dynptr, 0, 0);
    if (status) {
    bpf_printk("bpf_dynptr_read() failed: %d\n", status);
    err = 1;
    return 1;
    }
    } else {
    sample = bpf_dynptr_data(dynptr, 0, sizeof(*sample));
    if (!sample) {
    bpf_printk("Unexpectedly failed to get sample\n");
    err = 2;
    return 1;
    }
    stack_sample = *sample;
    }
    __sync_fetch_and_add(&read, 1);
    return 0;
    }
    static void
    handle_sample_msg(const struct test_msg *msg)
    {
    switch (msg.msg_op) {
    case TEST_MSG_OP_INC64:
    kern_mutated += msg.operand_64;
    break;
    case TEST_MSG_OP_INC32:
    kern_mutated += msg.operand_32;
    break;
    case TEST_MSG_OP_MUL64:
    kern_mutated *= msg.operand_64;
    break;
    case TEST_MSG_OP_MUL32:
    kern_mutated *= msg.operand_32;
    break;
    default:
    bpf_printk("Unrecognized op %d\n", msg.msg_op);
    err = 2;
    }
    }
    static long
    read_protocol_msg(struct bpf_dynptr *dynptr, void *context)
    {
    const struct test_msg *msg = core::ptr::null_mut();
    msg = bpf_dynptr_data(dynptr, 0, sizeof(*msg));
    if (!msg) {
    err = 1;
    bpf_printk("Unexpectedly failed to get msg\n");
    return 0;
    }
    handle_sample_msg(msg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn publish_next_kern_msg(index: __u32, context: *mut c_void) -> c_int {
    static int publish_next_kern_msg(__u32 index, void *context)
    {
    struct test_msg *msg = core::ptr::null_mut();
    let mut operand_64: c_int = TEST_OP_64;
    let mut operand_32: c_int = TEST_OP_32;
    msg = bpf_ringbuf_reserve(&kernel_ringbuf, sizeof(*msg), 0);
    if (!msg) {
    err = 4;
    return 1;
    }
    switch (index % TEST_MSG_OP_NUM_OPS) {
    case TEST_MSG_OP_INC64:
    msg.operand_64 = operand_64;
    msg.msg_op = TEST_MSG_OP_INC64;
    expected_user_mutated += operand_64;
    break;
    case TEST_MSG_OP_INC32:
    msg.operand_32 = operand_32;
    msg.msg_op = TEST_MSG_OP_INC32;
    expected_user_mutated += operand_32;
    break;
    case TEST_MSG_OP_MUL64:
    msg.operand_64 = operand_64;
    msg.msg_op = TEST_MSG_OP_MUL64;
    expected_user_mutated *= operand_64;
    break;
    case TEST_MSG_OP_MUL32:
    msg.operand_32 = operand_32;
    msg.msg_op = TEST_MSG_OP_MUL32;
    expected_user_mutated *= operand_32;
    break;
    default:
    bpf_ringbuf_discard(msg, 0);
    err = 5;
    return 1;
    }
    bpf_ringbuf_submit(msg, 0);
    return 0;
    }
    static void
    publish_kern_messages(void)
    {
    if (expected_user_mutated != user_mutated) {
    bpf_printk("%lu != %lu\n", expected_user_mutated, user_mutated);
    err = 3;
    return;
    }
    bpf_loop(8, publish_next_kern_msg, core::ptr::null_mut(), 0);
    }
    SEC("fentry/" SYS_PREFIX "sys_prctl")
#[no_mangle]
pub unsafe extern "C" fn test_user_ringbuf_protocol(ctx: *mut c_void) -> c_int {
    int test_user_ringbuf_protocol(void *ctx)
    {
    let mut status: c_long = 0;
    if (!is_test_process())
    return 0;
    status = bpf_user_ringbuf_drain(&user_ringbuf, read_protocol_msg, core::ptr::null_mut(), 0);
    if (status < 0) {
    bpf_printk("Drain returned: %ld\n", status);
    err = 1;
    return 0;
    }
    publish_kern_messages();
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn test_user_ringbuf(ctx: *mut c_void) -> c_int {
    int test_user_ringbuf(void *ctx)
    {
    if (!is_test_process())
    return 0;
    err = bpf_user_ringbuf_drain(&user_ringbuf, record_sample, core::ptr::null_mut(), 0);
    return 0;
    }
    static long
    do_nothing_cb(struct bpf_dynptr *dynptr, void *context)
    {
    __sync_fetch_and_add(&read, 1);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_prlimit64")
#[no_mangle]
pub unsafe extern "C" fn test_user_ringbuf_epoll(ctx: *mut c_void) -> c_int {
    int test_user_ringbuf_epoll(void *ctx)
    {
    long num_samples;
    if (!is_test_process())
    return 0;
    num_samples = bpf_user_ringbuf_drain(&user_ringbuf, do_nothing_cb, core::ptr::null_mut(), 0);
    if (num_samples <= 0)
    err = 1;
    return 0;
    }
