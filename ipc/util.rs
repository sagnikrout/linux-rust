//! Automatically rewritten from C Header to Rust Module
//! Source: ipc/util.h
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
// linux/ipc/util.h
// Copyright (C) 1999 Christoph Rohland
//
// ipc helper functions (c) 1999 Manfred Spraul <manfred@colorfullife.com>
// namespaces support.      2006 OpenVZ, SWsoft Inc.
// Pavel Emelianov <xemul@openvz.org>
//

//
// The IPC ID contains 2 separate numbers - index and sequence number.
// By default,
// bits  0-14: index (32k, 15 bits)
// bits 15-30: sequence number (64k, 16 bits)
//
// When IPCMNI extension mode is turned on, the composition changes:
// bits  0-23: index (16M, 24 bits)
// bits 24-30: sequence number (128, 7 bits)
//
pub const IPCMNI_SHIFT: c_int = 15;
pub const IPCMNI_EXTEND_SHIFT: c_int = 24;

extern "C" {
    pub fn sem_init();
}
extern "C" {
    pub fn msg_init();
}
extern "C" {
    pub fn shm_init();
}

extern "C" {
    pub fn mq_clear_sbinfo(ns: *mut ipc_namespace);
}

extern "C" {
    pub fn sem_init_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn msg_init_ns(ns: *mut ipc_namespace) -> c_int;
}
extern "C" {
    pub fn shm_init_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn sem_exit_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn msg_exit_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn shm_exit_ns(ns: *mut ipc_namespace);
}

//
// Structure that holds the parameters needed by the ipc operations
// (see after)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params {
    pub key: key_t,
    pub flg: c_int,
    pub /: *mut *mut size_t size; / for shared memories,
    pub /: *mut *mut int nsems; / for semaphores,
    pub /: *mut *mut } u; / holds the getnew() specific param,
}

//
// Structure that holds some ipc operations. This structure is used to unify
// the calls to sys_msgget(), sys_semget(), sys_shmget()
// . routine to call to create a new ipc object. Can be one of newque,
// newary, newseg
// . routine to call to check permissions for a new ipc object.
// Can be one of security_msg_associate, security_sem_associate,
// security_shm_associate
// . routine to call for an extra check if needed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ops {
    pub ): *mut *mut *mut int (getnew)(struct ipc_namespace , struct ipc_params,
    pub int): *mut *mut *mut int (associate)(struct kern_ipc_perm ,,
    pub ): *mut *mut *mut int (more_checks)(struct kern_ipc_perm , struct ipc_params,
}

extern "C" {
    pub fn ipc_init_ids(ids: *mut ipc_ids);
}

pub const IPC_SEM_IDS: c_int = 0;
pub const IPC_MSG_IDS: c_int = 1;
pub const IPC_SHM_IDS: c_int = 2;

// must be called with ids->rwsem acquired for writing
extern "C" {
    pub fn ipc_addid(: *mut ipc_ids, : *mut kern_ipc_perm, _arg: c_int) -> c_int;
}
// must be called with both locks acquired.
extern "C" {
    pub fn ipc_rmid(: *mut ipc_ids, : *mut kern_ipc_perm);
}
// must be called with both locks acquired.
extern "C" {
    pub fn ipc_set_key_private(: *mut ipc_ids, : *mut kern_ipc_perm);
}
// must be called with ipcp locked
extern "C" {
    pub fn ipcperms(ns: *mut ipc_namespace, ipcp: *mut kern_ipc_perm, flg: c_short) -> c_int;
}
//
// ipc_get_maxidx - get the highest assigned index
// @ids: ipc identifier set
//
// The function returns the highest assigned index for @ids. The function
// doesn't scan the idr tree, it uses a cached value.
//
// Called with ipc_ids.rwsem held for reading.
//
// For allocation that need to be freed by RCU.
// Objects are reference counted, they start with reference count 1.
// getref increases the refcount, the putref call that reduces the recount
// to 0 schedules the rcu destruction. Caller must guarantee locking.
//
// refcount is initialized by ipc_addid(), before that point call_rcu()
// must be used.
//
extern "C" {
    pub fn ipc_rcu_getref(ptr: *mut kern_ipc_perm) -> bool;
}
extern "C" {
    pub fn kernel_to_ipc64_perm(in: *mut kern_ipc_perm, out: *mut ipc64_perm);
}
extern "C" {
    pub fn ipc64_perm_to_ipc_perm(in: *mut ipc64_perm, out: *mut ipc_perm);
}
extern "C" {
    pub fn ipc_update_perm(in: *mut ipc64_perm, out: *mut kern_ipc_perm) -> c_int;
}
// pos = get_pid(pid);

extern "C" {
    pub fn ipc_parse_version(cmd: *mut c_int) -> c_int;
}

extern "C" {
    pub fn free_msg(msg: *mut msg_msg);
}
extern "C" {
    pub fn store_msg(dest: *mut void __user, msg: *mut msg_msg, len: usize) -> c_int;
}
//
// ipc_valid_object() - helper to sort out IPC_RMID races for codepaths
// where the respective ipc_ids.rwsem is not being held down.
// Checks whether the ipc object is still around or if it's gone already, as
// ipc_rmid() may have already freed the ID while the ipc lock was spinning.
// Needs to be called with kern_ipc_perm.lock held -- exception made for one
// checkpoint case at sys_semtimedop() as noted in code commentary.
//
// Check semmni range [0, ipc_mni]
// semmni is the last element of sem_ctls[4] array
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm {
    pub key: key_t,
    pub uid: __compat_uid_t,
    pub gid: __compat_gid_t,
    pub cuid: __compat_uid_t,
    pub cgid: __compat_gid_t,
    pub mode: compat_mode_t,
    pub seq: c_ushort,
}

extern "C" {
    pub fn to_compat_ipc_perm(: *mut compat_ipc_perm, : *mut ipc64_perm);
}
extern "C" {
    pub fn to_compat_ipc64_perm(: *mut compat_ipc64_perm, : *mut ipc64_perm);
}
extern "C" {
    pub fn get_compat_ipc_perm(: *mut ipc64_perm, : *mut compat_ipc_perm __user) -> c_int;
}
// cmd &= ~IPC_64;
extern "C" {
    pub fn compat_ksys_old_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_int) -> c_long;
}
extern "C" {
    pub fn compat_ksys_old_msgctl(msqid: c_int, cmd: c_int, uptr: *mut void __user) -> c_long;
}
extern "C" {
    pub fn compat_ksys_old_shmctl(shmid: c_int, cmd: c_int, uptr: *mut void __user) -> c_long;
}

