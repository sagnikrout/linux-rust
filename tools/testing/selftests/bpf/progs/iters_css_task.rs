//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iters_css_task.c
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
    struct cgroup *bpf_cgroup_acquire(struct cgroup *p) __ksym;
    struct cgroup *bpf_cgroup_from_id(u64 cgid) __ksym;
    void bpf_cgroup_release(struct cgroup *p) __ksym;
    pid_t target_pid;
    int css_task_cnt;
    u64 cg_id;
    SEC("lsm/file_mprotect")
    int BPF_PROG(iter_css_task_for_each, struct vm_area_struct *vma,
    unsigned long reqprot, unsigned long prot, int ret)
    {
    struct task_struct *cur_task = bpf_get_current_task_btf();
    struct cgroup_subsys_state *css;
    struct task_struct *task;
    struct cgroup *cgrp;
    if (cur_task.pid != target_pid)
    return ret;
    cgrp = bpf_cgroup_from_id(cg_id);
    if (!cgrp)
    return -EPERM;
    css = &cgrp.self;
    css_task_cnt = 0;
    bpf_for_each(css_task, task, css, CSS_TASK_ITER_PROCS)
    if (task.pid == target_pid)
    css_task_cnt++;
    bpf_cgroup_release(cgrp);
    return -EPERM;
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_id(cgrp: *mut cgroup) -> u64 {
    static inline u64 cgroup_id(struct cgroup *cgrp)
    {
    return cgrp.kn.id;
    }
    SEC("?iter/cgroup")
#[no_mangle]
pub unsafe extern "C" fn cgroup_id_printer(ctx: *mut bpf_iter__cgroup) -> c_int {
    int cgroup_id_printer(struct bpf_iter__cgroup *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct cgroup *cgrp = ctx.cgroup;
    struct cgroup_subsys_state *css;
    struct task_struct *task;
// epilogue
    if (cgrp == core::ptr::null_mut()) {
    BPF_SEQ_PRINTF(seq, "epilogue\n");
    return 0;
    }
// prologue
    if (ctx.meta.seq_num == 0)
    BPF_SEQ_PRINTF(seq, "prologue\n");
    BPF_SEQ_PRINTF(seq, "%8llu\n", cgroup_id(cgrp));
    css = &cgrp.self;
    css_task_cnt = 0;
    bpf_for_each(css_task, task, css, CSS_TASK_ITER_PROCS) {
    if (task.pid == target_pid)
    css_task_cnt++;
    }
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: iter_css_task_for_each_sleep) -> c_int {
    int BPF_PROG(iter_css_task_for_each_sleep)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup *cgrp = bpf_cgroup_from_id(cgrp_id);
    struct cgroup_subsys_state *css;
    struct task_struct *task;
    if (cgrp == core::ptr::null_mut())
    return 0;
    css = &cgrp.self;
    bpf_for_each(css_task, task, css, CSS_TASK_ITER_PROCS) {
    }
    bpf_cgroup_release(cgrp);
    return 0;
    }
