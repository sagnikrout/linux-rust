//! Automatically rewritten from C to Rust
//! Source: kernel/exec_state.c
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
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>

    static struct kmem_cache *task_exec_state_cachep;
#[no_mangle]
unsafe extern "C" fn __free_task_exec_state(rcu: *mut rcu_head) {
    static void __free_task_exec_state(struct rcu_head *rcu)
    {
    struct task_exec_state *exec_state = container_of(rcu, struct task_exec_state, rcu);
    put_user_ns(exec_state.user_ns);
    kmem_cache_free(task_exec_state_cachep, exec_state);
    }
#[no_mangle]
pub unsafe extern "C" fn put_task_exec_state(exec_state: *mut task_exec_state) {
    void put_task_exec_state(struct task_exec_state *exec_state)
    {
    if (exec_state && refcount_dec_and_test(&exec_state.count))
    call_rcu(&exec_state.rcu, __free_task_exec_state);
    }
    struct task_exec_state *alloc_task_exec_state(struct user_namespace *user_ns)
    {
    struct task_exec_state *exec_state;
    exec_state = kmem_cache_alloc(task_exec_state_cachep, GFP_KERNEL);
    if (!exec_state)
    return core::ptr::null_mut();
    refcount_set(&exec_state.count, 1);
    exec_state.dumpable = TASK_DUMPABLE_OFF;
    exec_state.user_ns = get_user_ns(user_ns);
    return exec_state;
    }
    struct task_exec_state *task_exec_state_rcu(const struct task_struct *tsk)
    {
    struct task_exec_state *exec_state;
    exec_state = rcu_dereference_check(tsk.exec_state,
    lockdep_is_held(&tsk.alloc_lock));
    WARN_ON_ONCE(!exec_state);
    return exec_state;
    }
    struct task_exec_state *task_exec_state_replace(struct task_struct *tsk,
    struct task_exec_state *exec_state)
    {
//
// Updates must hold both locks so callers needing a consistent
// snapshot of mm + dumpability are covered.
//
    lockdep_assert_held(&tsk.alloc_lock);
    lockdep_assert_held_write(&tsk.signal.exec_update_lock);
    return rcu_replace_pointer(tsk.exec_state, exec_state, true);
    }
//
// The non-CLONE_VM clone path: allocate a fresh exec_state and
// inherit the parent's dumpable mode and user_ns reference.  CLONE_VM
// siblings refcount-share via copy_exec_state() in fork.c; only this
// path and execve() ever allocate.
//
#[no_mangle]
pub unsafe extern "C" fn task_exec_state_copy(tsk: *mut task_struct) -> c_int {
    int task_exec_state_copy(struct task_struct *tsk)
    {
    struct task_exec_state *src, *dst;
    src = rcu_dereference_protected(current.exec_state, true);
    dst = alloc_task_exec_state(src.user_ns);
    if (!dst)
    return -ENOMEM;
    dst.dumpable = READ_ONCE(src.dumpable);
    rcu_assign_pointer(tsk.exec_state, dst);
    return 0;
    }
//
// Store TASK_DUMPABLE_* on current->exec_state.  All callers
// (commit_creds, begin_new_exec, prctl(PR_SET_DUMPABLE)) act on the
// running task, which guarantees ->exec_state is allocated and cannot
// be replaced under us.
//
#[no_mangle]
pub unsafe extern "C" fn task_exec_state_set_dumpable(value: enum task_dumpable) {
    void task_exec_state_set_dumpable(enum task_dumpable value)
    {
    struct task_exec_state *exec_state;
    if (WARN_ON_ONCE(value > TASK_DUMPABLE_ROOT))
    value = TASK_DUMPABLE_OFF;
    exec_state = rcu_dereference_protected(current.exec_state, true);
// mm-less tasks share init_task's exec_state; never mutate it
    if (WARN_ON_ONCE(exec_state == &init_task_exec_state))
    return;
    WRITE_ONCE(exec_state.dumpable, value);
    }
#[no_mangle]
pub unsafe extern "C" fn task_exec_state_get_dumpable(task: *mut task_struct) -> enum task_dumpable {
    enum task_dumpable task_exec_state_get_dumpable(struct task_struct *task)
    {
    struct task_exec_state *exec_state;
    guard(rcu)();
    exec_state = rcu_dereference(task.exec_state);
    return READ_ONCE(exec_state.dumpable);
    }
#[no_mangle]
pub unsafe extern "C" fn exec_state_init() -> void __init {
    void __init exec_state_init(void)
    {
    task_exec_state_cachep = kmem_cache_create("task_exec_state",
    sizeof(struct task_exec_state), 0,
    SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT,
    core::ptr::null_mut());
    }
