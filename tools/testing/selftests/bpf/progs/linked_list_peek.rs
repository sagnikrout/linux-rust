//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/linked_list_peek.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data {
    pub l: bpf_list_node,
    pub key: c_int,
}

    private(A) struct bpf_spin_lock glock;
    private(A) struct bpf_list_head ghead __contains(node_data, l);

pub const NR_NODES: c_int = 16;
    let mut zero: c_int = 0;
    SEC("syscall")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn list_peek(ctx: *mut c_void) -> c_long {
    long list_peek(void *ctx)
    {
    struct bpf_list_node *l_n;
    struct node_data *n;
    int i, err = 0;
    bpf_spin_lock(&glock);
    l_n = bpf_list_front(&ghead);
    bpf_spin_unlock(&glock);
    if (l_n)
    return __LINE__;
    bpf_spin_lock(&glock);
    l_n = bpf_list_back(&ghead);
    bpf_spin_unlock(&glock);
    if (l_n)
    return __LINE__;
    for (i = zero; i < NR_NODES && can_loop; i++) {
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return __LINE__;
    n.key = i;
    bpf_spin_lock(&glock);
    bpf_list_push_back(&ghead, &n.l);
    bpf_spin_unlock(&glock);
    }
    bpf_spin_lock(&glock);
    l_n = bpf_list_front(&ghead);
    if (!l_n) {
    err = __LINE__;
    goto done;
    }
    n = list_entry(l_n, struct node_data, l);
    if (n.key != 0) {
    err = __LINE__;
    goto done;
    }
    l_n = bpf_list_back(&ghead);
    if (!l_n) {
    err = __LINE__;
    goto done;
    }
    n = list_entry(l_n, struct node_data, l);
    if (n.key != NR_NODES - 1) {
    err = __LINE__;
    goto done;
    }
    done:
    bpf_spin_unlock(&glock);
    return err;
    }

    SEC("syscall")							\
    __failure __msg(MSG)						\
    long test_##op##_spinlock_##dolock(void *ctx)			\
    {								\
    struct bpf_list_node *l_n;				\
    __u64 jiffies = 0;					\
    \
    if (dolock)						\
    bpf_spin_lock(&glock);				\
    l_n = bpf_list_##op(&ghead);				\
    if (l_n)						\
    jiffies = bpf_jiffies64();			\
    if (dolock)						\
    bpf_spin_unlock(&glock);			\
    \
    return !!jiffies;					\
    }

    TEST_FB(front, true)
    TEST_FB(back, true)

    TEST_FB(front, false)
    TEST_FB(back, false)

    char _license[] SEC("license") = "GPL";
