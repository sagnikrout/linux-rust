//! Automatically rewritten from C to Rust
//! Source: lib/is_single_threaded.c
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
// Function to determine if a thread group is single threaded or not
//
// Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
// - Derived from security/selinux/hooks.c
//

//
// Returns true if the task does not share ->mm with another thread/process.
//
#[no_mangle]
pub unsafe extern "C" fn current_is_single_threaded() -> bool {
    bool current_is_single_threaded(void)
    {
    struct task_struct *task = current;
    struct mm_struct *mm = task.mm;
    struct task_struct *p, *t;
    bool ret;
    if (atomic_read(&task.signal.live) != 1)
    return false;
    if (atomic_read(&mm.mm_users) == 1)
    return true;
    ret = false;
    rcu_read_lock();
    for_each_process(p) {
    if (unlikely(p.flags & PF_KTHREAD))
    continue;
    if (unlikely(p == task.group_leader))
    continue;
    for_each_thread(p, t) {
    if (unlikely(t.mm == mm))
    goto found;
    if (likely(t.mm))
    break;
//
// t->mm == NULL. Make sure next_thread/next_task
// will see other CLONE_VM tasks which might be
// forked before exiting.
//
    smp_rmb();
    }
    }
    ret = true;
    found:
    rcu_read_unlock();
    return ret;
    }
