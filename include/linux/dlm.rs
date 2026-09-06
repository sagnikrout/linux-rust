//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dlm.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
// Copyright (C) 2004-2011 Red Hat, Inc.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_slot {
    pub /: *mut *mut int nodeid; / 1 to MAX_INT,
    pub /: *mut *mut int slot; / 1 to MAX_INT,
}

//
// recover_prep: called before the dlm begins lock recovery.
// Notfies lockspace user that locks from failed members will be granted.
// recover_slot: called after recover_prep and before recover_done.
// Identifies a failed lockspace member.
// recover_done: called after the dlm completes lock recovery.
// Identifies lockspace members and lockspace generation number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_lockspace_ops {
    pub ops_arg): *mut *mut void (recover_prep) (void,
    pub slot): *mut *mut *mut void (recover_slot) (void ops_arg, struct dlm_slot,
    pub generation): int num_slots, int our_slot, uint32_t,
}

// only relevant for kernel lockspaces, will be removed in future

//
// dlm_new_lockspace
//
// Create/join a lockspace.
//
// name: lockspace name, null terminated, up to DLM_LOCKSPACE_LEN (not
// including terminating null).
//
// cluster: cluster name, null terminated, up to DLM_LOCKSPACE_LEN (not
// including terminating null).  Optional.  When cluster is null, it
// is not used.  When set, dlm_new_lockspace() returns -EBADR if cluster
// is not equal to the dlm cluster name.
//
// flags:
// DLM_LSFL_NODIR
// The dlm should not use a resource directory, but statically assign
// resource mastery to nodes based on the name hash that is otherwise
// used to select the directory node.  Must be the same on all nodes.
// DLM_LSFL_NEWEXCL
// dlm_new_lockspace() should return -EEXIST if the lockspace exists.
// DLM_LSFL_SOFTIRQ
// dlm request callbacks (ast, bast) are softirq safe. Flag should be
// preferred by users. Will be default in some future. If set the
// strongest context for ast, bast callback is softirq as it avoids
// an additional context switch.
//
// lvblen: length of lvb in bytes.  Must be multiple of 8.
// dlm_new_lockspace() returns an error if this does not match
// what other nodes are using.
//
// ops: callbacks that indicate lockspace recovery points so the
// caller can coordinate its recovery and know lockspace members.
// This is only used by the initial dlm_new_lockspace() call.
// Optional.
//
// ops_arg: arg for ops callbacks.
//
// ops_result: tells caller if the ops callbacks (if provided) will
// be used or not.  0: will be used, -EXXX will not be used.
// -EOPNOTSUPP: the dlm does not have recovery_callbacks enabled.
//
// lockspace: handle for dlm functions
//
// dlm_release_lockspace() release_option values:
//
// DLM_RELEASE_NO_LOCKS returns -EBUSY if any locks (lkb's)
// exist in the local lockspace.
//
// DLM_RELEASE_UNUSED previous value that is no longer used.
//
// DLM_RELEASE_NORMAL releases the lockspace regardless of any
// locks managed in the local lockspace.
//
// DLM_RELEASE_NO_EVENT release the lockspace regardless of any
// locks managed in the local lockspace, and does not submit
// a leave event to the cluster manager, so other nodes will
// not be notified that the node should be removed from the
// list of lockspace members.
//
// DLM_RELEASE_RECOVER like DLM_RELEASE_NORMAL, but the remaining
// nodes will handle the removal of the node as if the node
// had failed, e.g. the recover_slot() callback would be used.
//
pub const DLM_RELEASE_NO_LOCKS: c_int = 0;
pub const DLM_RELEASE_UNUSED: c_int = 1;
pub const DLM_RELEASE_NORMAL: c_int = 2;
pub const DLM_RELEASE_NO_EVENT: c_int = 3;
pub const DLM_RELEASE_RECOVER: c_int = 4;

//
// dlm_release_lockspace
//
// Stop a lockspace.
//
// release_option: see DLM_RELEASE values above.
//
// dlm_lock
//
// Make an asynchronous request to acquire or convert a lock on a named
// resource.
//
// lockspace: context for the request
// mode: the requested mode of the lock (DLM_LOCK_)
// lksb: lock status block for input and async return values
// flags: input flags (DLM_LKF_)
// name: name of the resource to lock, can be binary
// namelen: the length in bytes of the resource name (MAX_RESNAME_LEN)
// parent: the lock ID of a parent lock or 0 if none
// lockast: function DLM executes when it completes processing the request
// astarg: argument passed to lockast and bast functions
// bast: function DLM executes when this lock later blocks another request
//
// Returns:
// 0 if request is successfully queued for processing
// -EINVAL if any input parameters are invalid
// -EAGAIN if request would block and is flagged DLM_LKF_NOQUEUE
// -ENOMEM if there is no memory to process request
// -ENOTCONN if there is a communication error
//
// If the call to dlm_lock returns an error then the operation has failed and
// the AST routine will not be called.  If dlm_lock returns 0 it is still
// possible that the lock operation will fail. The AST routine will be called
// when the locking is complete and the status is returned in the lksb.
//
// If the AST routines or parameter are passed to a conversion operation then
// they will overwrite those values that were passed to a previous dlm_lock
// call.
//
// AST routines should not block (at least not for long), but may make
// any locking calls they please. If DLM_LSFL_SOFTIRQ for kernel
// users of dlm_new_lockspace() is passed the ast and bast callbacks
// can be processed in softirq context. Also some of the callback
// contexts are in the same context as the DLM lock request API, users
// must not hold locks while calling dlm lock request API and trying
// to acquire this lock in the callback again, this will end in a
// lock recursion. For newer implementation the DLM_LSFL_SOFTIRQ
// should be used.
//
// dlm_unlock
//
// Asynchronously release a lock on a resource.  The AST routine is called
// when the resource is successfully unlocked.
//
// lockspace: context for the request
// lkid: the lock ID as returned in the lksb
// flags: input flags (DLM_LKF_)
// lksb: if NULL the lksb parameter passed to last lock request is used
// astarg: the arg used with the completion ast for the unlock
//
// Returns:
// 0 if request is successfully queued for processing
// -EINVAL if any input parameters are invalid
// -ENOTEMPTY if the lock still has sublocks
// -EBUSY if the lock is waiting for a remote lock operation
// -ENOTCONN if there is a communication error
//
