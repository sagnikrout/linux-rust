//! Automatically rewritten from C to Rust
//! Source: fs/jffs2/background.c
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


//
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

    static int jffs2_garbage_collect_thread(void *);
#[no_mangle]
pub unsafe extern "C" fn jffs2_garbage_collect_trigger(c: *mut jffs2_sb_info) {
    void jffs2_garbage_collect_trigger(struct jffs2_sb_info *c)
    {
    assert_spin_locked(&c.erase_completion_lock);
    if (c.gc_task && jffs2_thread_should_wake(c))
    send_sig(SIGHUP, c.gc_task, 1);
    }
// This must only ever be called when no GC thread is currently running
#[no_mangle]
pub unsafe extern "C" fn jffs2_start_garbage_collect_thread(c: *mut jffs2_sb_info) -> c_int {
    int jffs2_start_garbage_collect_thread(struct jffs2_sb_info *c)
    {
    struct task_struct *tsk;
    let mut ret: c_int = 0;
    BUG_ON(c.gc_task);
    init_completion(&c.gc_thread_start);
    init_completion(&c.gc_thread_exit);
    tsk = kthread_run(jffs2_garbage_collect_thread, c, "jffs2_gcd_mtd%d", c.mtd.index);
    if (IS_ERR(tsk)) {
    pr_warn("fork failed for JFFS2 garbage collect thread: %pe\n",
    tsk);
    complete(&c.gc_thread_exit);
    ret = PTR_ERR(tsk);
    } else {
// Wait for it...
    jffs2_dbg(1, "Garbage collect thread is pid %d\n", tsk.pid);
    wait_for_completion(&c.gc_thread_start);
    ret = tsk.pid;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_stop_garbage_collect_thread(c: *mut jffs2_sb_info) {
    void jffs2_stop_garbage_collect_thread(struct jffs2_sb_info *c)
    {
    let mut wait: c_int = 0;
    spin_lock(&c.erase_completion_lock);
    if (c.gc_task) {
    jffs2_dbg(1, "Killing GC task %d\n", c.gc_task.pid);
    send_sig(SIGKILL, c.gc_task, 1);
    wait = 1;
    }
    spin_unlock(&c.erase_completion_lock);
    if (wait)
    wait_for_completion(&c.gc_thread_exit);
    }
#[no_mangle]
unsafe extern "C" fn jffs2_garbage_collect_thread(_c: *mut c_void) -> c_int {
    static int jffs2_garbage_collect_thread(void *_c)
    {
    struct jffs2_sb_info *c = _c;
    sigset_t hupmask;
    siginitset(&hupmask, sigmask(SIGHUP));
    allow_signal(SIGKILL);
    allow_signal(SIGSTOP);
    allow_signal(SIGHUP);
    c.gc_task = current;
    complete(&c.gc_thread_start);
    set_user_nice(current, 10);
    set_freezable();
    for (;;) {
    sigprocmask(SIG_UNBLOCK, &hupmask, core::ptr::null_mut());
    again:
    spin_lock(&c.erase_completion_lock);
    if (!jffs2_thread_should_wake(c)) {
    set_current_state (TASK_INTERRUPTIBLE);
    spin_unlock(&c.erase_completion_lock);
    jffs2_dbg(1, "%s(): sleeping...\n", __func__);
    schedule();
    } else {
    spin_unlock(&c.erase_completion_lock);
    }
// Problem - immediately after bootup, the GCD spends a lot
// of time in places like jffs2_kill_fragtree(); so much so
// that userspace processes (like gdm and X) are starved
// despite plenty of cond_resched()s and renicing.  Yield()
// doesn't help, either (presumably because userspace and GCD
// are generally competing for a higher latency resource -
// disk).
// This forces the GCD to slow the hell down.   Pulling an
// inode in with read_inode() is much preferable to having
// the GC thread get there first.
    schedule_timeout_interruptible(msecs_to_jiffies(50));
    if (kthread_should_stop()) {
    jffs2_dbg(1, "%s(): kthread_stop() called\n", __func__);
    goto die;
    }
// Put_super will send a SIGKILL and then wait on the sem.
//
    while (signal_pending(current) || freezing(current)) {
    unsigned long signr;
    if (try_to_freeze())
    goto again;
    signr = kernel_dequeue_signal();
    switch(signr) {
    case SIGSTOP:
    jffs2_dbg(1, "%s(): SIGSTOP received\n",
    __func__);
    kernel_signal_stop();
    break;
    case SIGKILL:
    jffs2_dbg(1, "%s(): SIGKILL received\n",
    __func__);
    goto die;
    case SIGHUP:
    jffs2_dbg(1, "%s(): SIGHUP received\n",
    __func__);
    break;
    default:
    jffs2_dbg(1, "%s(): signal %ld received\n",
    __func__, signr);
    }
    }
// We don't want SIGHUP to interrupt us. STOP and KILL are OK though.
    sigprocmask(SIG_BLOCK, &hupmask, core::ptr::null_mut());
    jffs2_dbg(1, "%s(): pass\n", __func__);
    if (jffs2_garbage_collect_pass(c) == -ENOSPC) {
    pr_notice("No space for garbage collection. Aborting GC thread\n");
    goto die;
    }
    }
    die:
    spin_lock(&c.erase_completion_lock);
    c.gc_task = core::ptr::null_mut();
    spin_unlock(&c.erase_completion_lock);
    kthread_complete_and_exit(&c.gc_thread_exit, 0);
    }
