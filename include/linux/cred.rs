//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cred.h
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
// Credentials management - see Documentation/security/credentials.rst
//
// Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// COW Supplementary groups list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_info {
    pub usage: refcount_t,
    pub ngroups: c_int,
    pub gid: [kgid_t; ],
    pub __randomize_layout: },
//
// get_group_info - Get a reference to a group info structure
// @gi: The group info to reference
//
// This gets a reference to a set of supplementary groups.
//
// If the caller is accessing a task's credentials, they must hold the RCU read
// lock when reading.
//
// Returns: @gi
//
    pub gi: return,
//
// put_group_info - Release a reference to a group info structure
// @group_info: The group info to release
//

    pub \: groups_free(group_info);,

    pub groups_alloc(int): *mut extern struct group_info,
    pub ): *mut extern void groups_free(struct group_info,
    pub in_group_p(kgid_t): extern int,
    pub in_egroup_p(kgid_t): extern int,
    pub kgid_t): *const *const extern int groups_search(struct group_info ,,
    pub ): *mut extern int set_current_groups(struct group_info,
    pub ): *mut *mut extern void set_groups(struct cred , struct group_info,
    pub may_setgroups(void): extern bool,
    pub ): *mut extern void groups_sort(struct group_info,

    pub 1: return,
    pub 1: return,
    pub 1: return,

//
// The security context of a task
//
// The parts of the context break down into two categories:
//
// (1) The objective context of a task.  These parts are used when some other
// task is attempting to affect this one.
//
// (2) The subjective context.  These details are used when the task is acting
// upon another object, be that a file, a task, a key or whatever.
//
// Note that some members of this structure belong to both categories - the
// LSM security pointer for instance.
//
// A task has two security pointers.  task->real_cred points to the objective
// context that defines that task's actual details.  The objective part of this
// context is used whenever that task is acted upon.
//
// task->cred points to the subjective context that defines the details of how
// that task is going to act upon another object.  This may be overridden
// temporarily to point to another security context, but normally points to the
// same context as task->real_cred.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred {
    pub usage: atomic_long_t,
    pub /: *mut *mut kuid_t uid; / real UID of the task,
    pub /: *mut *mut kgid_t gid; / real GID of the task,
    pub /: *mut *mut kuid_t suid; / saved UID of the task,
    pub /: *mut *mut kgid_t sgid; / saved GID of the task,
    pub /: *mut *mut kuid_t euid; / effective UID of the task,
    pub /: *mut *mut kgid_t egid; / effective GID of the task,
    pub /: *mut *mut kuid_t fsuid; / UID for VFS ops,
    pub /: *mut *mut kgid_t fsgid; / GID for VFS ops,
    pub /: *mut *mut unsigned securebits; / SUID-less security management,
    pub /: *mut *mut kernel_cap_t cap_inheritable; / caps our children can inherit,
    pub /: *mut *mut kernel_cap_t cap_permitted; / caps we're permitted,
    pub /: *mut *mut kernel_cap_t cap_effective; / caps we can actually use,
    pub /: *mut *mut kernel_cap_t cap_bset; / capability bounding set,
    pub /: *mut *mut kernel_cap_t cap_ambient; / Ambient capability set,

    pub requested: *mut *mut unsigned char jit_keyring; / default keyring to attach,
// keys to
    pub /: *mut *mut *mut key session_keyring; / keyring inherited over fork,
    pub /: *mut *mut *mut key process_keyring; / keyring private to this process,
    pub /: *mut *mut *mut key thread_keyring; / keyring private to this thread,
    pub /: *mut *mut *mut key request_key_auth; / assumed request_key authority,

    pub /: *mut *mut *mut void security; / LSM security,

    pub /: *mut *mut *mut user_user; / real user ID subscription,
    pub /: *mut *mut *mut user_namespace user_ns; / user_ns the caps and keyrings are relative to.,
    pub ucounts: *mut ucounts,
    pub /: *mut *mut *mut group_info group_info; / supplementary groups for euid/fsgid,
// RCU deletion
    pub /: *mut *mut int non_rcu; / Can we skip RCU deletion?,
    pub /: *mut *mut rcu_head rcu; / RCU deletion hook,
}

extern "C" {
    pub fn __put_cred(: *mut cred);
}
extern "C" {
    pub fn exit_creds(: *mut task_struct);
}
extern "C" {
    pub fn copy_creds(: *mut task_struct, _arg: u64) -> c_int;
}
extern "C" {
    pub fn commit_creds(: *mut cred) -> c_int;
}
extern "C" {
    pub fn abort_creds(: *mut cred);
}
// shut up sparse
extern "C" {
    pub fn rcu_dereference_raw(_arg: init_task.cred) -> return;
}
extern "C" {
    pub fn set_security_override(: *mut cred, _arg: u32) -> c_int;
}
extern "C" {
    pub fn set_create_files_as(: *mut cred, : *mut inode) -> c_int;
}
extern "C" {
    pub fn cred_fscmp(: *const cred, : *const cred) -> c_int;
}
extern "C" {
    pub fn cred_init() -> void __init;
}
extern "C" {
    pub fn set_cred_ucounts(: *mut cred) -> c_int;
}
extern "C" {
    pub fn rcu_replace_pointer(_arg: current->cred, _arg: override_cred, _arg: 1) -> return;
}
extern "C" {
    pub fn rcu_replace_pointer(_arg: current->cred, _arg: revert_cred, _arg: 1) -> return;
}

//
// get_cred_many - Get references on a set of credentials
// @cred: The credentials to reference
// @nr: Number of references to acquire
//
// Get references on the specified set of credentials.  The caller must release
// all acquired reference.  If %NULL is passed, it is returned with no action.
//
// This is used to deal with a committed set of credentials.  Although the
// pointer is const, this will temporarily discard the const and increment the
// usage count.  The purpose of this is to attempt to catch at compile time the
// accidental alteration of a set of credentials that should be considered
// immutable.
//
// Returns: @cred when the references are acquired, NULL otherwise.
//
// get_cred - Get a reference on a set of credentials
// @cred: The credentials to reference
//
// Get a reference on the specified set of credentials.  The caller must
// release the reference.  If %NULL is passed, it is returned with no action.
//
// This is used to deal with a committed set of credentials.
//
extern "C" {
    pub fn get_cred_many(_arg: cred, _arg: 1) -> return;
}
//
// put_cred_many - Release a reference to a set of credentials
// @_cred: The credentials to release
// @nr: Number of references to release
//
// Release a reference to a set of credentials, deleting them when the last ref
// is released.  If %NULL is passed, nothing is done.
//
// This takes a const pointer to a set of credentials because the credentials
// on task_struct are attached by const pointers to prevent accidental
// alteration of otherwise immutable credential sets.
//
// put_cred - Release a reference to a set of credentials
// @cred: The credentials to release
//
// Release a reference to a set of credentials, deleting them when the last ref
// is released.  If %NULL is passed, nothing is done.
//
// current_cred - Access the current task's subjective credentials
//
// Access the subjective credentials of the current task.  RCU-safe,
// since nobody else can modify it.
//

//
// current_real_cred - Access the current task's objective credentials
//
// Access the objective credentials of the current task.  RCU-safe,
// since nobody else can modify it.
//

//
// __task_cred - Access a task's objective credentials
// @task: The task to query
//
// Access the objective credentials of a task.  The caller must hold the RCU
// readlock.
//
// The result of this function should not be passed directly to get_cred();
// rather get_task_cred() should be used instead.
//

//
// get_current_cred - Get the current task's subjective credentials
//
// Get the subjective credentials of the current task, pinning them so that
// they can't go away.  Accessing the current task's credentials directly is
// not permitted.
//

//
// get_current_user - Get the current task's user_struct
//
// Get the user record of the current task, pinning it so that it can't go
// away.
//

//
// get_current_groups - Get the current task's supplementary group list
//
// Get the supplementary group list of the current task, pinning it so that it
// can't go away.
//

// (_uid) = __cred->uid;			\
// (_gid) = __cred->gid;			\

// (_euid) = __cred->euid;		\
// (_egid) = __cred->egid;		\

// (_fsuid) = __cred->fsuid;		\
// (_fsgid) = __cred->fsgid;		\
