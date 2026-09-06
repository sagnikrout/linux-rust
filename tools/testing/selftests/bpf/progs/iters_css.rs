//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_css.c
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
// Copyright (C) 2023 Chuyi Zhou <zhouchuyi@bytedance.com>

    char _license[] SEC("license") = "GPL";
    pid_t target_pid;
    u64 root_cg_id, leaf_cg_id;
    u64 first_cg_id, last_cg_id;
    int pre_order_cnt, post_order_cnt, children_cnt, tree_high;
    struct cgroup *bpf_cgroup_from_id(u64 cgid) __ksym;
    void bpf_cgroup_release(struct cgroup *p) __ksym;
    void bpf_rcu_read_lock(void) __ksym;
    void bpf_rcu_read_unlock(void) __ksym;
    SEC("fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn iter_css_for_each(ctx: *const c_void) -> c_int {
    int iter_css_for_each(const void *ctx)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct cgroup_subsys_state *root_css, *leaf_css, *pos;
    struct cgroup *root_cgrp, *leaf_cgrp, *cur_cgrp;
    if (cur_task.pid != target_pid)
    return 0;
    root_cgrp = bpf_cgroup_from_id(root_cg_id);
    if (!root_cgrp)
    return 0;
    leaf_cgrp = bpf_cgroup_from_id(leaf_cg_id);
    if (!leaf_cgrp) {
    bpf_cgroup_release(root_cgrp);
    return 0;
    }
    root_css = &root_cgrp.self;
    leaf_css = &leaf_cgrp.self;
    pre_order_cnt = post_order_cnt = children_cnt = tree_high = 0;
    first_cg_id = last_cg_id = 0;
    bpf_rcu_read_lock();
    bpf_for_each(css, pos, root_css, BPF_CGROUP_ITER_DESCENDANTS_POST) {
    cur_cgrp = pos.cgroup;
    post_order_cnt++;
    last_cg_id = cur_cgrp.kn.id;
    }
    bpf_for_each(css, pos, root_css, BPF_CGROUP_ITER_DESCENDANTS_PRE) {
    cur_cgrp = pos.cgroup;
    pre_order_cnt++;
    if (!first_cg_id)
    first_cg_id = cur_cgrp.kn.id;
    }
    bpf_for_each(css, pos, root_css, BPF_CGROUP_ITER_CHILDREN) {
    children_cnt++;
    }
    bpf_for_each(css, pos, leaf_css, BPF_CGROUP_ITER_ANCESTORS_UP)
    tree_high++;
    bpf_for_each(css, pos, root_css, BPF_CGROUP_ITER_ANCESTORS_UP)
    tree_high--;
    bpf_rcu_read_unlock();
    bpf_cgroup_release(root_cgrp);
    bpf_cgroup_release(leaf_cgrp);
    return 0;
    }
