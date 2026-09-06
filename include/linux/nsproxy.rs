//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nsproxy.h
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
// A structure to contain pointers to all per-process
// namespaces - fs (mount), uts, network, sysvipc, etc.
//
// The pid namespace is an exception -- it's accessed using
// task_active_pid_ns.  The pid namespace here is the
// namespace that children will use.
//
// 'count' is the number of tasks holding a reference.
// The count for each namespace, then, will be the number
// of nsproxies pointing to it, not the number of tasks.
//
// The nsproxy is shared by tasks which share all namespaces.
// As soon as a single namespace is cloned or unshared, the
// nsproxy is copied.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsproxy {
    pub count: refcount_t,
    pub uts_ns: *mut uts_namespace,
    pub ipc_ns: *mut ipc_namespace,
    pub mnt_ns: *mut mnt_namespace,
    pub pid_ns_for_children: *mut pid_namespace,
    pub net_ns: *mut net,
    pub time_ns: *mut time_namespace,
    pub time_ns_for_children: *mut time_namespace,
    pub cgroup_ns: *mut cgroup_namespace,
}

//
// A structure to encompass all bits needed to install
// a partial or complete new set of namespaces.
//
// If a new user namespace is requested cred will
// point to a modifiable set of credentials. If a pointer
// to a modifiable set is needed nsset_cred() must be
// used and tested.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsset {
    pub flags: unsigned,
    pub nsproxy: *mut nsproxy,
    pub fs: *mut fs_struct,
    pub cred: *const cred,
}

//
// the namespaces access rules are:
//
// 1. only current task is allowed to change tsk->nsproxy pointer or
// any pointer on the nsproxy itself.  Current must hold the task_lock
// when changing tsk->nsproxy.
//
// 2. when accessing (i.e. reading) current task's namespaces - no
// precautions should be taken - just dereference the pointers
//
// 3. the access to other task namespaces is performed like this
// task_lock(task);
// nsproxy = task->nsproxy;
// if (nsproxy != NULL) {
// /
// * work with the namespaces here
// * e.g. get the reference on one of them
// *
// } /
// * NULL task->nsproxy means that this task is
// * almost dead (zombie)
// *
// task_unlock(task);
//
extern "C" {
    pub fn copy_namespaces(flags: u64, tsk: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn switch_cred_namespaces(old: *const cred, new: *const cred);
}
extern "C" {
    pub fn exit_nsproxy_namespaces(tsk: *mut task_struct);
}
extern "C" {
    pub fn get_cred_namespaces(tsk: *mut task_struct);
}
extern "C" {
    pub fn exit_cred_namespaces(tsk: *mut task_struct);
}
extern "C" {
    pub fn switch_task_namespaces(tsk: *mut task_struct, new: *mut nsproxy);
}
extern "C" {
    pub fn exec_task_namespaces() -> c_int;
}
extern "C" {
    pub fn deactivate_nsproxy(ns: *mut nsproxy);
}
extern "C" {
    pub fn nsproxy_cache_init() -> int __init;
}
