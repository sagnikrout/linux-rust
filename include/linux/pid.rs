//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pid.h
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
// What is struct pid?
//
// A struct pid is the kernel's internal notion of a process identifier.
// It refers to individual tasks, process groups, and sessions.  While
// there are processes attached to it the struct pid lives in a hash
// table, so it and then the processes that it refers to can be found
// quickly from the numeric pid value.  The attached processes may be
// quickly accessed by following pointers from struct pid.
//
// Storing pid_t values in the kernel and referring to them later has a
// problem.  The process originally with that pid may have exited and the
// pid allocator wrapped, and another process could have come along
// and been assigned that pid.
//
// Referring to user space processes by holding a reference to struct
// task_struct has a problem.  When the user space process exits
// the now useless task_struct is still kept.  A task_struct plus a
// stack consumes around 10K of low kernel memory.  More precisely
// this is THREAD_SIZE + sizeof(struct task_struct).  By comparison
// a struct pid is about 64 bytes.
//
// Holding a reference to struct pid solves both of these problems.
// It is small so holding a reference does not consume a lot of
// resources, and since a new struct pid is allocated when the numeric pid
// value is reused (when pids wrap around) we don't mistakenly refer to new
// processes.
//
// struct upid is used to get the id of the struct pid, as it is
// seen in particular namespace. Later the struct pid is found with
// find_pid_ns() using the int nr and struct pid_namespace *ns.
//
pub const RESERVED_PIDS: c_int = 300;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upid {
    pub nr: c_int,
    pub ns: *mut pid_namespace,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pid {
    pub count: refcount_t,
    pub level: c_uint,
    pub lock: spinlock_t,
    pub ino: u64,
    pub pidfs_hash: rhash_head,
    pub stashed: *mut dentry,
    pub attr: *mut pidfs_attr,
}

// lists of tasks that use this pid
// wait queue for pidfd notifications
extern "C" {
    pub fn pidfd_prepare(pid: *mut pid, flags: c_uint, ret_file: *mut file) -> c_int;
}
extern "C" {
    pub fn do_notify_pidfd(task: *mut task_struct);
}
extern "C" {
    pub fn put_pid(pid: *mut pid);
}
//
// these helpers must be called with the tasklist_lock write-held.
//
extern "C" {
    pub fn attach_pid(task: *mut task_struct, pid_type: enum);
}
extern "C" {
    pub fn detach_pid(pids: *mut pid, task: *mut task_struct, pid_type: enum);
}
extern "C" {
    pub fn exchange_tids(task: *mut task_struct, old: *mut task_struct);
}
//
// look up a PID in the hash table. Must be called with the tasklist_lock
// or rcu_read_lock() held.
//
// find_pid_ns() finds the pid in the namespace specified
// find_vpid() finds the pid by its virtual id, i.e. in the current namespace
//
// see also find_task_by_vpid() set in include/linux/sched.h
//
// Lookup a PID in the hash table, and return with it's count elevated.
//
extern "C" {
    pub fn free_pid(pid: *mut pid);
}
extern "C" {
    pub fn free_pids(pids: *mut pid);
}
extern "C" {
    pub fn disable_pid_allocation(ns: *mut pid_namespace);
}
//
// ns_of_pid() returns the pid namespace in which the specified pid was
// allocated.
//
// NOTE:
// ns_of_pid() is expected to be called for a process (task) that has
// an attached 'struct pid' (see attach_pid(), detach_pid()) i.e @pid
// is expected to be non-NULL. If @pid is NULL, caller should handle
// the resulting NULL pid-ns.
//
// is_child_reaper returns true if the pid is the init process
// of the current namespace. As this one could be checked before
// pid_ns->child_reaper is assigned in copy_process, we check
// with the pid number.
//
// the helpers to get the pid's id seen from different namespaces
//
// pid_nr()    : global id, i.e. the id seen from the init namespace;
// pid_vnr()   : virtual id, i.e. the id seen from the pid namespace of
// current.
// pid_nr_ns() : id seen from the ns specified.
//
// see also task_xid_nr() etc in include/linux/sched.h
//
extern "C" {
    pub fn pid_nr_ns(pid: *mut pid, ns: *mut pid_namespace) -> pid_t;
}
extern "C" {
    pub fn pid_vnr(pid: *mut pid) -> pid_t;
}

//
// Both old and new leaders may be attached to
// the same pid in the middle of de_thread().
//

//
// the helpers to get the task's different pids as they are seen
// from various namespaces
//
// task_xid_nr()     : global id, i.e. the id seen from the init namespace;
// task_xid_vnr()    : virtual id, i.e. the id seen from the pid namespace of
// current.
// task_xid_nr_ns()  : id seen from the ns specified;
//
// see also pid_nr() etc in include/linux/pid.h
//
extern "C" {
    pub fn __task_pid_nr_ns(task: *mut task_struct, type: pid_type, ns: *mut pid_namespace) -> pid_t;
}
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_PID, _arg: ns) -> return;
}
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_PID, _arg: NULL) -> return;
}
//
// pid_alive - check that a task structure is not stale
// @p: Task structure to be checked.
//
// Test if a process is not yet dead (at most zombie state)
// If pid_alive fails, then pointers within the task structure
// can be stale and must not be dereferenced.
//
// Return: 1 if the process is alive. 0 otherwise.
//
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_PGID, _arg: ns) -> return;
}
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_PGID, _arg: NULL) -> return;
}
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_SID, _arg: ns) -> return;
}
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_SID, _arg: NULL) -> return;
}
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_TGID, _arg: ns) -> return;
}
extern "C" {
    pub fn __task_pid_nr_ns(_arg: tsk, _arg: PIDTYPE_TGID, _arg: NULL) -> return;
}
extern "C" {
    pub fn task_ppid_nr_ns(_arg: tsk, _arg: NULL) -> return;
}
extern "C" {
    pub fn task_ppid_nr_ns(_arg: tsk, _arg: &init_pid_ns) -> return;
}
// Obsolete, do not use:
extern "C" {
    pub fn task_pgrp_nr_ns(_arg: tsk, _arg: &init_pid_ns) -> return;
}
//
// is_global_init - check if a task structure is init. Since init
// is free to have sub-threads we need to check tgid.
// @tsk: Task structure to be checked.
//
// Check if a task structure is the first user space task the kernel created.
//
// Return: 1 if the task structure is init. 0 otherwise.
//
