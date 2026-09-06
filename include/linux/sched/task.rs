//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/task.h
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
//
// Interface between the scheduler and various task lifetime (fork()/exit())
// functionality:
//

// All the bits taken by the old clone syscall.
pub const CLONE_LEGACY_FLAGS: c_uint = 0xffffffffULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_clone_args {
    pub flags: u64,
    pub pidfd: *mut int __user,
    pub child_tid: *mut int __user,
    pub parent_tid: *mut int __user,
    pub name: *const c_char,
    pub exit_signal: c_int,
    pub kthread:1: u32,
    pub io_thread:1: u32,
    pub user_worker:1: u32,
    pub no_files:1: u32,
    pub umh:1: u32,
    pub stack: c_ulong,
    pub stack_size: c_ulong,
    pub tls: c_ulong,
    pub set_tid: *mut pid_t,
// Number of elements in *set_tid
    pub set_tid_size: usize,
    pub cgroup: c_int,
    pub idle: c_int,
    pub ): *mut *mut int (fn)(void,
    pub fn_arg: *mut c_void,
    pub cgrp: *mut cgroup,
    pub cset: *mut css_set,
    pub kill_seq: c_uint,
}

//
// This serializes "schedule()" and also protects
// the run-queue from deletions/modifications (but
// _adding_ to the beginning of the run-queue has
// a separate lock).
//
extern "C" {
    pub fn lockdep_tasklist_lock_is_held() -> c_int;
}
extern "C" {
    pub fn schedule_tail(prev: *mut task_struct) -> asmlinkage void;
}
extern "C" {
    pub fn init_idle(idle: *mut task_struct, cpu: c_int);
}
extern "C" {
    pub fn sched_fork(clone_flags: u64, p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn sched_cgroup_fork(p: *mut task_struct, kargs: *mut kernel_clone_args) -> c_int;
}
extern "C" {
    pub fn sched_cancel_fork(p: *mut task_struct);
}
extern "C" {
    pub fn sched_post_fork(p: *mut task_struct);
}
extern "C" {
    pub fn sched_dead(p: *mut task_struct);
}
extern "C" {
    pub fn do_task_dead() -> void __noreturn;
}
extern "C" {
    pub fn make_task_dead(signr: c_int) -> void __noreturn;
}
extern "C" {
    pub fn mm_cache_init();
}
extern "C" {
    pub fn proc_caches_init();
}
extern "C" {
    pub fn fork_init();
}
extern "C" {
    pub fn release_task(p: *mut *mut task_struct);
}
extern "C" {
    pub fn copy_thread(: *mut task_struct, : *const kernel_clone_args) -> c_int;
}
extern "C" {
    pub fn flush_thread();
}

extern "C" {
    pub fn exit_thread(tsk: *mut task_struct);
}

extern "C" {
    pub fn do_group_exit(_arg: c_int) -> __noreturn void;
}
extern "C" {
    pub fn exit_files(: *mut task_struct);
}
extern "C" {
    pub fn exit_itimers(: *mut task_struct);
}
extern "C" {
    pub fn kernel_clone(kargs: *mut kernel_clone_args) -> pid_t;
}
extern "C" {
    pub fn user_mode_thread(): *mut *mut int (fn)(void, arg: *mut c_void, flags: c_ulong) -> pid_t;
}
extern "C" {
    pub fn kernel_wait4(_arg: pid_t, : *mut int __user, _arg: c_int, : *mut rusage) -> c_long;
}
extern "C" {
    pub fn kernel_wait(pid: pid_t, stat: *mut c_int) -> c_int;
}
extern "C" {
    pub fn free_task(tsk: *mut task_struct);
}
// sched_exec is called by processes performing an exec
extern "C" {
    pub fn sched_exec();
}
extern "C" {
    pub fn __put_task_struct(t: *mut task_struct);
}
extern "C" {
    pub fn __put_task_struct_rcu_cb(rhp: *mut rcu_head);
}
//
// Under PREEMPT_RT, we can't call __put_task_struct
// in atomic context because it will indirectly
// acquire sleeping locks. The same is true if the
// current process has a mutex enqueued (blocked on
// a PI chain).
//
// In !RT, it is always safe to call __put_task_struct().
// Though, in order to simplify the code, resort to the
// deferred call too.
//
// call_rcu() will schedule __put_task_struct_rcu_cb()
// to be called in process context.
//
// __put_task_struct() is called when
// refcount_dec_and_test(&t->usage) succeeds.
//
// This means that it can't "conflict" with
// put_task_struct_rcu_user() which abuses ->rcu the same
// way; rcu_users has a reference so task->usage can't be
// zero after rcu_users 1 -> 0 transition.
//
// delayed_free_task() also uses ->rcu, but it is only called
// when it fails to fork a process. Therefore, there is no
// way it can conflict with __put_task_struct().
//
extern "C" {
    pub fn put_task_struct_rcu_user(task: *mut task_struct);
}
// Free all architecture-specific resources held by a thread.
extern "C" {
    pub fn release_thread(dead_task: *mut task_struct);
}

//
// If an architecture has not declared a thread_struct whitelist we
// must assume something there may need to be copied to userspace.
//
// offset = 0;
// Handle dynamically sized thread_struct.
// size = arch_task_struct_size - offsetof(struct task_struct, thread);

//
// Protects ->fs, ->files, ->mm, ->group_info, ->comm, keyring
// subscriptions and synchronises with wait4().  Also used in procfs.  Also
// pins the final release of task.io_context.  Also protects ->cpuset and
// ->cgroup.subsys[]. And ->vfork_done. And ->sysvshm.shm_clist.
//
// Nests inside of read_lock(&tasklist_lock). It must not be nested with
// write_lock_irq(&tasklist_lock), neither inside nor outside.
//

