//! Automatically rewritten from C to Rust
//! Source: net/core/netclassid_cgroup.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// net/core/netclassid_cgroup.c	Classid Cgroupfs Handling
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//

    static inline struct cgroup_cls_state *css_cls_state(struct cgroup_subsys_state *css)
    {
    return css ? container_of(css, struct cgroup_cls_state, css) : core::ptr::null_mut();
    }
    struct cgroup_cls_state *task_cls_state(struct task_struct *p)
    {
    return css_cls_state(task_css_check(p, net_cls_cgrp_id,
    rcu_read_lock_held() ||
    rcu_read_lock_bh_held() ||
    rcu_read_lock_trace_held()));
    }
    EXPORT_SYMBOL_GPL(task_cls_state);
    static struct cgroup_subsys_state *
    cgrp_css_alloc(struct cgroup_subsys_state *parent_css)
    {
    struct cgroup_cls_state *cs;
    cs = kzalloc_obj(*cs);
    if (!cs)
    return ERR_PTR(-ENOMEM);
    return &cs.css;
    }
#[no_mangle]
unsafe extern "C" fn cgrp_css_online(css: *mut cgroup_subsys_state) -> c_int {
    static int cgrp_css_online(struct cgroup_subsys_state *css)
    {
    struct cgroup_cls_state *cs = css_cls_state(css);
    struct cgroup_cls_state *parent = css_cls_state(css.parent);
    if (parent)
    cs.classid = parent.classid;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgrp_css_free(css: *mut cgroup_subsys_state) {
    static void cgrp_css_free(struct cgroup_subsys_state *css)
    {
    kfree(css_cls_state(css));
    }
//
// To avoid freezing of sockets creation for tasks with big number of threads
// and opened sockets lets release file_lock every 1000 iterated descriptors.
// New sockets will already have been created with new classid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_classid_context {
    pub classid: u32,
    pub batch: c_uint,
}

pub const UPDATE_CLASSID_BATCH: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn update_classid_sock(v: *const c_void, file: *mut file, n: c_uint) -> c_int {
    static int update_classid_sock(const void *v, struct file *file, unsigned int n)
    {
    struct update_classid_context *ctx = (void *)v;
    struct socket *sock = sock_from_file(file);
    if (sock)
    sock_cgroup_set_classid(&sock.sk.sk_cgrp_data, ctx.classid);
    if (--ctx.batch == 0) {
    ctx.batch = UPDATE_CLASSID_BATCH;
    return n + 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn update_classid_task(p: *mut task_struct, classid: u32) {
    static void update_classid_task(struct task_struct *p, u32 classid)
    {
    struct update_classid_context ctx = {
    .classid = classid,
    .batch = UPDATE_CLASSID_BATCH
    };
    let mut fd: c_uint = 0;
// Only update the leader task, when many threads in this task,
// so it can avoid the useless traversal.
//
    if (!thread_group_leader(p))
    return;
    do {
    task_lock(p);
    fd = iterate_fd(p.files, fd, update_classid_sock, &ctx);
    task_unlock(p);
    cond_resched();
    } while (fd);
    }
#[no_mangle]
unsafe extern "C" fn cgrp_attach(tset: *mut cgroup_taskset) {
    static void cgrp_attach(struct cgroup_taskset *tset)
    {
    struct cgroup_subsys_state *css;
    struct task_struct *p;
    cgroup_taskset_for_each(p, css, tset) {
    update_classid_task(p, css_cls_state(css).classid);
    }
    }
#[no_mangle]
unsafe extern "C" fn read_classid(css: *mut cgroup_subsys_state, cft: *mut cftype) -> u64 {
    static u64 read_classid(struct cgroup_subsys_state *css, struct cftype *cft)
    {
    return css_cls_state(css).classid;
    }
    static int write_classid(struct cgroup_subsys_state *css, struct cftype *cft,
    u64 value)
    {
    struct cgroup_cls_state *cs = css_cls_state(css);
    struct css_task_iter it;
    struct task_struct *p;
    cs.classid = (u32)value;
    css_task_iter_start(css, 0, &it);
    while ((p = css_task_iter_next(&it)))
    update_classid_task(p, cs.classid);
    css_task_iter_end(&it);
    return 0;
    }
    static struct cftype ss_files[] = {
    {
    .name		= "classid",
    .read_u64	= read_classid,
    .write_u64	= write_classid,
    },
    { }	/* terminate */
    };
    struct cgroup_subsys net_cls_cgrp_subsys = {
    .css_alloc		= cgrp_css_alloc,
    .css_online		= cgrp_css_online,
    .css_free		= cgrp_css_free,
    .attach			= cgrp_attach,
    .legacy_cftypes		= ss_files,
    };
