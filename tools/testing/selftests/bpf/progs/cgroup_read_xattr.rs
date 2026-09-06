//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_read_xattr.c
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

    char _license[] SEC("license") = "GPL";
    char value[16];
#[no_mangle]
unsafe extern "C" fn read_xattr(cgroup: *mut cgroup) -> __always_inline void {
    static __always_inline void read_xattr(struct cgroup *cgroup)
    {
    struct bpf_dynptr value_ptr;
    bpf_dynptr_from_mem(value, sizeof(value), 0, &value_ptr);
    bpf_cgroup_read_xattr(cgroup, "user.bpf_test",
    &value_ptr);
    }
    SEC("lsm.s/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trusted_cgroup_ptr_sleepable) -> c_int {
    int BPF_PROG(trusted_cgroup_ptr_sleepable)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp;
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp)
    return 0;
    read_xattr(cgrp);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("lsm/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trusted_cgroup_ptr_non_sleepable) -> c_int {
    int BPF_PROG(trusted_cgroup_ptr_non_sleepable)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp;
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp)
    return 0;
    read_xattr(cgrp);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("lsm/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: use_css_iter_non_sleepable) -> c_int {
    int BPF_PROG(use_css_iter_non_sleepable)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup_subsys_state *css;
    struct cgroup *cgrp;
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp)
    return 0;
    bpf_for_each(css, css, &cgrp.self, BPF_CGROUP_ITER_ANCESTORS_UP)
    read_xattr(css.cgroup);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("lsm.s/socket_connect")
#[no_mangle]
pub unsafe extern "C" fn __msg(protection": "kernel func bpf_iter_css_new requires RCU critical section) -> __failure {
    __failure __msg("kernel func bpf_iter_css_new requires RCU critical section protection")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: use_css_iter_sleepable_missing_rcu_lock) -> c_int {
    int BPF_PROG(use_css_iter_sleepable_missing_rcu_lock)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup_subsys_state *css;
    struct cgroup *cgrp;
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp)
    return 0;
    bpf_for_each(css, css, &cgrp.self, BPF_CGROUP_ITER_ANCESTORS_UP)
    read_xattr(css.cgroup);
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("lsm.s/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: use_css_iter_sleepable_with_rcu_lock) -> c_int {
    int BPF_PROG(use_css_iter_sleepable_with_rcu_lock)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup_subsys_state *css;
    struct cgroup *cgrp;
    bpf_rcu_read_lock();
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp)
    goto out;
    bpf_for_each(css, css, &cgrp.self, BPF_CGROUP_ITER_ANCESTORS_UP)
    read_xattr(css.cgroup);
    bpf_cgroup_release(cgrp);
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("lsm/socket_connect")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: use_bpf_cgroup_ancestor) -> c_int {
    int BPF_PROG(use_bpf_cgroup_ancestor)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp, *ancestor;
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp)
    return 0;
    ancestor = bpf_cgroup_ancestor(cgrp, 1);
    if (!ancestor)
    goto out;
    read_xattr(cgrp);
    bpf_cgroup_release(ancestor);
    out:
    bpf_cgroup_release(cgrp);
    return 0;
    }
    SEC("cgroup/sendmsg4")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cgroup_skb) -> c_int {
    int BPF_PROG(cgroup_skb)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp, *ancestor;
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp)
    return 0;
    ancestor = bpf_cgroup_ancestor(cgrp, 1);
    if (!ancestor)
    goto out;
    read_xattr(cgrp);
    bpf_cgroup_release(ancestor);
    out:
    bpf_cgroup_release(cgrp);
    return 0;
    }
