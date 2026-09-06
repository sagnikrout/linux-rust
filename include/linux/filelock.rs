//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/filelock.h
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
// Special return value from posix_lock_file() and vfs_lock_file() for
// asynchronous locking.
//
pub const FILE_LOCK_DEFERRED: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_lock_operations {
    pub ): *mut *mut *mut void (fl_copy_lock)(struct file_lock , struct file_lock,
    pub ): *mut *mut void (fl_release_private)(struct file_lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_manager_operations {
    pub lm_mod_owner: *mut c_void,
    pub (*lm_get_owner)(fl_owner_t): *mut fl_owner_t,
    pub (*lm_put_owner)(fl_owner_t): *mut c_void,
    pub /: *mut *mut *mut *mut void (lm_notify)(struct file_lock ); / unblock callback,
    pub int): *mut *mut *mut int (lm_grant)(struct file_lock ,,
    pub cfl): *mut *mut bool (lm_lock_expirable)(struct file_lock,
    pub (*lm_expire_lock)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lease_manager_operations {
    pub ): *mut *mut bool (lm_break)(struct file_lease,
    pub ): *mut *mut *mut int (lm_change)(struct file_lease , int, struct list_head,
    pub ): *mut *mut *mut void (lm_setup)(struct file_lease , void,
    pub ): *mut *mut bool (lm_breaker_owns_lease)(struct file_lease,
    pub int): *mut *mut *mut int (lm_open_conflict)(struct file ,,
    pub fl): *mut *mut bool (lm_breaker_timedout)(struct file_lease,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_manager {
    pub list: list_head,
//
// NFSv4 and up also want opens blocked during the grace period;
// NLM doesn't care:
//
    pub block_opens: bool,
}

extern "C" {
    pub fn locks_start_grace(: *mut net, : *mut lock_manager);
}
extern "C" {
    pub fn locks_end_grace(: *mut lock_manager);
}
extern "C" {
    pub fn locks_in_grace(: *mut net) -> bool;
}
extern "C" {
    pub fn opens_in_grace(: *mut net) -> bool;
}
//
// struct file_lock has a union that some filesystems use to track
// their own private info. The NFS side of things is defined here:
//

//
// struct file_lock represents a generic "file lock". It's used to represent
// POSIX byte range locks, BSD (flock) locks, and leases. It's important to
// note that the same struct is used to represent both a request for a lock and
// the lock itself, but the same object is never used for both.
//
// FIXME: should we create a separate "struct lock_request" to help distinguish
// these two uses?
//
// The varous i_flctx lists are ordered by:
//
// 1) lock owner
// 2) lock range start
// 3) lock range end
//
// Obviously, the last two criteria only matter for POSIX locks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_lock_core {
    pub /: *mut *mut *mut file_lock_core flc_blocker; / The lock that is blocking us,
    pub /: *mut *mut list_head flc_list; / link into file_lock_context,
    pub /: *mut *mut hlist_node flc_link; / node in global lists,
    pub with: *mut *mut list_head flc_blocked_requests; / list of requests,
// ->fl_blocker pointing here
//
    pub in: *mut *mut list_head flc_blocked_member; / node,
// ->fl_blocker->fl_blocked_requests
//
    pub flc_owner: fl_owner_t,
    pub flc_flags: c_uint,
    pub flc_type: c_uchar,
    pub flc_pid: pid_t,
    pub /: *mut *mut int flc_link_cpu; / what cpu's list is this on?,
    pub flc_wait: wait_queue_head_t,
    pub flc_file: *mut file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_lock {
    pub c: file_lock_core,
    pub fl_start: loff_t,
    pub fl_end: loff_t,
    pub /: *const *const *const file_lock_operations fl_ops; / Callbacks for filesystems,
    pub /: *const *const *const lock_manager_operations fl_lmops; / Callbacks for lockmanagers,
    pub nfs_fl: nfs_lock_info,
    pub nfs4_fl: nfs4_lock_info,
    pub /: *mut *mut list_head link; / link in AFS vnode's pending_locks list,
    pub /: *mut *mut int state; / state of grant or error if -ve,
    pub debug_id: c_uint,
    pub afs: },
    pub inode: *mut inode,
    pub ceph: },
    pub fl_u: },
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_lease {
    pub c: file_lock_core,
    pub /: *mut *mut *mut fasync_ fl_fasync; / for lease break notifications,
// for lease breaks:
    pub fl_break_time: c_ulong,
    pub fl_downgrade_time: c_ulong,
    pub /: *const *const *const lease_manager_operations fl_lmops; / Callbacks for lease managers,
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_lock_context {
    pub flc_lock: spinlock_t,
    pub flc_flock: list_head,
    pub flc_posix: list_head,
    pub flc_lease: list_head,
}

extern "C" {
    pub fn fcntl_getlk(: *mut file, int: unsigned, : *mut flock) -> c_int;
}

extern "C" {
    pub fn fcntl_getlk64(: *mut file, int: unsigned, : *mut flock64) -> c_int;
}

extern "C" {
    pub fn fcntl_setlease(fd: c_uint, filp: *mut file, arg: c_int) -> c_int;
}
extern "C" {
    pub fn fcntl_getlease(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn fcntl_setdeleg(fd: c_uint, filp: *mut file, deleg: *mut delegation) -> c_int;
}
extern "C" {
    pub fn fcntl_getdeleg(filp: *mut file, deleg: *mut delegation) -> c_int;
}
// fs/locks.c
extern "C" {
    pub fn locks_free_lock_context(inode: *mut inode);
}
extern "C" {
    pub fn locks_free_lock(fl: *mut file_lock);
}
extern "C" {
    pub fn locks_init_lock(: *mut file_lock);
}
extern "C" {
    pub fn locks_copy_lock(: *mut file_lock, : *mut file_lock);
}
extern "C" {
    pub fn locks_copy_conflock(: *mut file_lock, : *mut file_lock);
}
extern "C" {
    pub fn locks_remove_posix(: *mut file, _arg: fl_owner_t);
}
extern "C" {
    pub fn locks_remove_file(: *mut file);
}
extern "C" {
    pub fn locks_release_private(: *mut file_lock);
}
extern "C" {
    pub fn posix_test_lock(: *mut file, : *mut file_lock);
}
extern "C" {
    pub fn posix_lock_file(: *mut file, : *mut file_lock, : *mut file_lock) -> c_int;
}
extern "C" {
    pub fn locks_delete_block(: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn vfs_test_lock(: *mut file, : *mut file_lock) -> c_int;
}
extern "C" {
    pub fn vfs_lock_file(: *mut file, int: unsigned, : *mut file_lock, : *mut file_lock) -> c_int;
}
extern "C" {
    pub fn vfs_cancel_lock(filp: *mut file, fl: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn vfs_inode_has_locks(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn locks_lock_inode_wait(inode: *mut inode, fl: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn locks_init_lease(: *mut file_lease);
}
extern "C" {
    pub fn locks_free_lease(fl: *mut file_lease);
}
extern "C" {
    pub fn __break_lease(inode: *mut inode, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn lease_get_mtime(: *mut inode, time: *mut timespec64);
}
extern "C" {
    pub fn generic_setlease(: *mut file, _arg: c_int, : *mut file_lease, priv: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kernel_setlease(: *mut file, _arg: c_int, : *mut file_lease, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn vfs_setlease(: *mut file, _arg: c_int, : *mut file_lease, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn lease_modify(: *mut file_lease, _arg: c_int, : *mut list_head) -> c_int;
}
extern "C" {
    pub fn inode_lease_ignore_mask(inode: *mut inode) -> u32;
}
extern "C" {
    pub fn lease_register_notifier(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn lease_unregister_notifier(: *mut notifier_block);
}
//
// Paired with smp_store_release in locks_get_lock_context().
//
// Ensures ->i_flctx will be visible if we spotted the flag.
//
extern "C" {
    pub fn READ_ONCE(_arg: inode->i_flctx) -> return;
}

// for walking lists of file_locks linked by fl_list

extern "C" {
    pub fn locks_lock_inode_wait(_arg: file_inode(filp), _arg: fl) -> return;
}

//
// Since this check is lockless, we must ensure that any refcounts
// taken are done before checking i_flctx->flc_lease. Otherwise, we
// could end up racing with tasks trying to set a new lease on this
// file.
//
extern "C" {
    pub fn __break_lease(_arg: inode, openmode_to_lease_flags(mode): LEASE_BREAK_LEASE |) -> return;
}
//
// Since this check is lockless, we must ensure that any refcounts
// taken are done before checking i_flctx->flc_lease. Otherwise, we
// could end up racing with tasks trying to set a new lease on this
// file.
//
extern "C" {
    pub fn __break_lease(_arg: inode, _arg: flags) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delegated_inode {
    pub di_inode: *mut inode,
}

//
// try_break_deleg - do a non-blocking delegation break
// @inode: inode that should have its delegations broken
// @flags: extra LEASE_BREAK_* flags to pass to break_deleg()
// @di: returns pointer to delegated inode (may be NULL)
//
// Break delegations in a non-blocking fashion. If there are
// outstanding delegations and @di is set, then an extra reference
// will be taken on @inode and @di->di_inode will be populated so
// that it may be waited upon.
//
// Returns 0 if there is no need to wait or an error. If -EWOULDBLOCK
// is returned, then @di will be populated (if non-NULL).
//
extern "C" {
    pub fn __break_lease(_arg: inode, _arg: flags) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delegated_inode {
    pub false: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,

