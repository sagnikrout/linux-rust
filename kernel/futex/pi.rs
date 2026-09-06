//! Automatically rewritten from C to Rust
//! Source: kernel/futex/pi.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-or-later

//
// PI code:
//
#[no_mangle]
pub unsafe extern "C" fn refill_pi_state_cache() -> c_int {
pub static mut pi_state: *mut c_void = core::ptr::null_mut();
    if (likely(current.futex.pi_state_cache)) {
    return 0;
    }
    pi_state = kzalloc_obj(*pi_state);
    if (!pi_state) {
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&pi_state.list);
// pi_mutex gets initialized later
    pi_state.owner = core::ptr::null_mut();
    refcount_set(&pi_state.refcount, 1);
    pi_state.key = FUTEX_KEY_INIT;
    current.futex.pi_state_cache = pi_state;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_pi_state() -> *mut c_void {
    let mut pi_state = current.futex.pi_state_cache;
    WARN_ON!(!pi_state);
    current.futex.pi_state_cache = core::ptr::null_mut();
    return pi_state;
    }
#[no_mangle]
pub unsafe extern "C" fn pi_state_update_owner(pi_state: *mut futex_pi_state, new_owner: *mut task_struct) {
    let mut old_owner = pi_state.owner;
    lockdep_assert_held(&pi_state.pi_mutex.wait_lock);
    if (old_owner) {
    raw_spin_lock(&old_owner.pi_lock);
    WARN_ON!(list_empty(&pi_state.list));
    list_del_init(&pi_state.list);
    raw_spin_unlock(&old_owner.pi_lock);
    }
    if (new_owner) {
    raw_spin_lock(&new_owner.pi_lock);
    WARN_ON!(!list_empty(&pi_state.list));
    list_add(&pi_state.list, &new_owner.futex.pi_state_list);
    pi_state.owner = new_owner;
    raw_spin_unlock(&new_owner.pi_lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_pi_state(pi_state: *mut futex_pi_state) {
    WARN_ON_ONCE!(!refcount_inc_not_zero(&pi_state.refcount));
    }
//
// Drops a reference to the pi_state object and frees or caches it
// when the last reference is gone.
//
#[no_mangle]
pub unsafe extern "C" fn put_pi_state(pi_state: *mut futex_pi_state) {
    if (!pi_state) {
    return;
    }
    if (!refcount_dec_and_test(&pi_state.refcount)) {
    return;
    }
//
// If pi_state->owner is NULL, the owner is most probably dying
// and has cleaned up the pi_state already
//
    if (pi_state.owner) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&pi_state.pi_mutex.wait_lock, flags);
    pi_state_update_owner(pi_state, core::ptr::null_mut());
    rt_mutex_proxy_unlock(&pi_state.pi_mutex);
    raw_spin_unlock_irqrestore(&pi_state.pi_mutex.wait_lock, flags);
    }
    if (current.futex.pi_state_cache) {
    kfree(pi_state);
    } else {
//
// pi_state->list is already empty.
// clear pi_state->owner.
// refcount is at 0 - put it back to 1.
//
    pi_state.owner = core::ptr::null_mut();
    refcount_set(&pi_state.refcount, 1);
    current.futex.pi_state_cache = pi_state;
    }
    }
//
// We need to check the following states:
//
// Waiter | pi_state | pi->owner | uTID      | uODIED | ?
//
// [1]  NULL   | ---      | ---       | 0         | 0/1    | Valid
// [2]  NULL   | ---      | ---       | >0        | 0/1    | Valid
//
// [3]  Found  | NULL     | --        | Any       | 0/1    | Invalid
//
// [4]  Found  | Found    | NULL      | 0         | 1      | Valid
// [5]  Found  | Found    | NULL      | >0        | 1      | Invalid
//
// [6]  Found  | Found    | task      | 0         | 1      | Valid
//
// [7]  Found  | Found    | NULL      | Any       | 0      | Invalid
//
// [8]  Found  | Found    | task      | ==taskTID | 0/1    | Valid
// [9]  Found  | Found    | task      | 0         | 0      | Invalid
// [10] Found  | Found    | task      | !=taskTID | 0/1    | Invalid
//
// [1]	Indicates that the kernel can acquire the futex atomically. We
// came here due to a stale FUTEX_WAITERS/FUTEX_OWNER_DIED bit.
//
// [2]	Valid, if TID does not belong to a kernel thread. If no matching
// thread is found then it indicates that the owner TID has died.
//
// [3]	Invalid. The waiter is queued on a non PI futex
//
// [4]	Valid state after exit_robust_list(), which sets the user space
// value to FUTEX_WAITERS | FUTEX_OWNER_DIED.
//
// [5]	The user space value got manipulated between exit_robust_list()
// and exit_pi_state_list()
//
// [6]	Valid state after exit_pi_state_list() which sets the new owner in
// the pi_state but cannot access the user space value.
//
// [7]	pi_state->owner can only be NULL when the OWNER_DIED bit is set.
//
// [8]	Owner and user space value match
//
// [9]	There is no transient state which sets the user space TID to 0
// except exit_robust_list(), but this is indicated by the
// FUTEX_OWNER_DIED bit. See [4]
//
// [10] There is no transient state which leaves owner and user space
// TID out of sync. Except one error case where the kernel is denied
// write access to the user address, see fixup_pi_state_owner().
//
// Serialization and lifetime rules:
//
// hb->lock:
//
// hb -> futex_q, relation
// futex_q -> pi_state, relation
//
// (cannot be raw because hb can contain arbitrary amount
// of futex_q's)
//
// pi_mutex->wait_lock:
//
// {uval, pi_state}
//
// (and pi_mutex 'obviously')
//
// p->pi_lock:
//
// p->futex.pi_state_list -> pi_state->list, relation
// pi_mutex->owner -> pi_state->owner, relation
//
// pi_state->refcount:
//
// pi_state lifetime
//
// Lock order:
//
// hb->lock
// pi_mutex->wait_lock
// p->pi_lock
//
// Futex kernel state:
//
// The kernel tracks the task state in p::futex::state to protect against exit()
// and exec(). The states are:
//
// - FUTEX_STATE_OK when the task is alive and waiters can be attached
//
// - FUTEX_STATE_EXITING when the task cleans up the robust list and PI
// state. Concurrent waiters cannot attach anymore and have to wait until the
// cleanup is finished to re-evaluate the potential changes caused by the
// robust list and PI state cleanups.
//
// - FUTEX_STATE_DEAD when the task has cleaned up the robust list. This state
// is set independent of exit() or exec(). In the exit() case the task is
// gone. In the exec() case this ensures that nothing can attach to the task
// after cleaning up the robust list and PI state before it has switched to
// the new mm. From a futex point of view the task is dead until it sets the
// state to FUTEX_STATE_OK again after switching to the new mm.
//
// The valid state transitions for exit():
//
// FUTEX_STATE_OK -> FUTEX_STATE_EXITING -> FUTEX_STATE_DEAD
//
// The valid state transitions for exec():
//
// FUTEX_STATE_OK -> FUTEX_STATE_EXITING -> FUTEX_STATE_DEAD -> FUTEX_STATE_OK
//
// The state has two related locks:
//
// 1) p::pi_lock
//
// p::pi_lock has to be taken by the waiter when evaluating the state to
// protect against a concurrent exit/exec cleanup by the owner. If the state
// is OK then the waiter can be attached to the owner while still holding
// pi_lock.
//
// The cleanup code has to hold it for all state transitions to ensure that
// the stores to the state cannot be reordered against previous stores on
// which the waiter correctness depends on.
//
// 2) p::futex::exit_mutex
//
// The mutex is acquired when the cleanup starts and released at the end. It
// obviously is not serializing the owner's cleanup against itself. It is
// used to avoid a live lock caused by a waiter preempting the owner's
// cleanup. Such a waiter would busy loop forever waiting for the owner to
// finish the cleanup.
//
// To prevent this, waiters have to drop all locks when observing
// FUTEX_STATE_EXITING and block on the mutex. When the owner releases the
// mutex after finishing the cleanup the waiters make progress and
// re-evaluate the situation.
//
// Validate that the existing waiter has a pi_state and sanity check
// the pi_state against the user space value. If correct, attach to
// it.
//
#[no_mangle]
pub unsafe extern "C" fn attach_to_pi_state(uaddr: *mut u32, uval: u32, pi_state: *mut futex_pi_state, ps: *mut *mut futex_pi_state) -> c_int {
pub static mut pid: pid_t = 0;
    let mut uval2 = 0;
    let mut ret = 0;
//
// Userspace might have messed up non-PI and PI futexes [3]
//
    if (unlikely(!pi_state)) {
    return -EINVAL;
    }
//
// We get here with hb->lock held, and having found a
// futex_top_waiter(). This means that futex_lock_pi() of said futex_q
// has dropped the hb->lock in between futex_queue() and futex_unqueue_pi(),
// which in turn means that futex_lock_pi() still has a reference on
// our pi_state.
//
// The waiter holding a reference on @pi_state also protects against
// the unlocked put_pi_state() in futex_unlock_pi(), futex_lock_pi()
// and futex_wait_requeue_pi() as it cannot go to 0 and consequently
// free pi_state before we can take a reference ourselves.
//
    WARN_ON!(!refcount_read(&pi_state.refcount));
//
// Now that we have a pi_state, we can acquire wait_lock
// and do the state validation.
//
    raw_spin_lock_irq(&pi_state.pi_mutex.wait_lock);
//
// Since {uval, pi_state} is serialized by wait_lock, and our current
// uval was read without holding it, it can have changed. Verify it
// still is what we expect it to be, otherwise retry the entire
// operation.
//
    if (futex_get_value_locked(&uval2, uaddr)) {
// goto;
    }
    if (uval != uval2) {
// goto;
    }
//
// Handle the owner died case:
//
    if (uval & FUTEX_OWNER_DIED) {
//
// exit_pi_state_list sets owner to NULL and wakes the
// topmost waiter. The task which acquires the
// pi_state->rt_mutex will fixup owner.
//
    if (!pi_state.owner) {
//
// No pi state owner, but the user space TID
// is not 0. Inconsistent state. [5]
//
    if (pid) {
// goto;
    }
//
// Take a ref on the state and return success. [4]
//
// goto;
    }
//
// If TID is 0, then either the dying owner has not
// yet executed exit_pi_state_list() or some waiter
// acquired the rtmutex in the pi state, but did not
// yet fixup the TID in user space.
//
// Take a ref on the state and return success. [6]
//
    if (!pid) {
// goto;
    }
    } else {
//
// If the owner died bit is not set, then the pi_state
// must have an owner. [7]
//
    if (!pi_state.owner) {
// goto;
    }
    }
//
// Bail out if user space manipulated the futex value. If pi
// state exists then the owner TID must be the same as the
// user space TID. [9/10]
//
    if (pid != task_pid_vnr(pi_state.owner)) {
// goto;
    }
// label;
    get_pi_state(pi_state);
    raw_spin_unlock_irq(&pi_state.pi_mutex.wait_lock);
// ps = pi_state;
    return 0;
// label;
    ret = -EINVAL;
// goto;
// label;
    ret = -EAGAIN;
// goto;
// label;
    ret = -EFAULT;
// goto;
// label;
    raw_spin_unlock_irq(&pi_state.pi_mutex.wait_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn handle_exit_race(uaddr: *mut u32 , uval: u32) -> c_int {
    let mut uval2 = 0;
//
// Reread the user space value to handle the following situation:
//
// CPU0				CPU1
//
// sys_exit()			sys_futex()
// do_exit()			 futex_lock_pi()
// futex_lock_pi_atomic()
// exit_signals(tsk)		    No waiters:
// tsk->flags |= PF_EXITING;	    *uaddr == 0x00000PID
// mm_release(tsk)		    Set waiter bit
// exit_robust_list(tsk) {	    *uaddr = 0x80000PID;
// Set owner died		    attach_to_pi_owner() {
// *uaddr = 0xC0000000;	     tsk = get_task(PID);
// }				     if (!tsk->flags & PF_EXITING) {
// ...				       attach();
// tsk->futex.state =               } else {
// FUTEX_STATE_DEAD;              if (tsk->futex.state !=
// FUTEX_STATE_DEAD)
// return -EAGAIN;
// return -ESRCH; <--- FAIL
// }
//
// Returning ESRCH unconditionally is wrong here because the
// user space value has been changed by the exiting task.
//
// The same logic applies to the case where the exiting task is
// already gone.
//
    if (futex_get_value_locked(&uval2, uaddr)) {
    return -EFAULT;
    }
// If the user space value has changed, try again.
    if (uval2 != uval) {
    return -EAGAIN;
    }
//
// The exiting task did not have a robust list, the robust list was
// corrupted or the user space value in *uaddr is simply bogus.
// Give up and tell user space.
//
    return -ESRCH;
    }
#[no_mangle]
pub unsafe extern "C" fn __attach_to_pi_owner(p: *mut task_struct, key: *mut union futex_key, ps: *mut *mut futex_pi_state) {
//
// No existing pi state. First waiter. [2]
//
// This creates pi_state, we have hb->lock held, this means nothing can
// observe this state, wait_lock is irrelevant.
//
    let mut pi_state = alloc_pi_state();
//
// Initialize the pi_mutex in locked state and make @p
// the owner of it:
//
    __assume_ctx_lock(&pi_state.pi_mutex.wait_lock);
    rt_mutex_init_proxy_locked(&pi_state.pi_mutex, p);
// Store the key for possible exit cleanups:
    pi_state.key = *key;
    WARN_ON!(!list_empty(&pi_state.list));
    list_add(&pi_state.list, &p.futex.pi_state_list);
//
// Assignment without holding pi_state->pi_mutex.wait_lock is safe
// because there is no concurrency as the object is not published yet.
//
    pi_state.owner = p;
// ps = pi_state;
    }
//
// Lookup the task for the TID provided from user space and attach to
// it after doing proper sanity checks.
//
#[no_mangle]
pub unsafe extern "C" fn attach_to_pi_owner(uaddr: *mut u32, uval: u32, key: *mut union futex_key, ps: *mut *mut futex_pi_state, exiting: *mut *mut task_struct) -> c_int {
pub static mut pid: pid_t = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
//
// We are the first waiter - try to look up the real owner and attach
// the new pi_state to it, but bail out when TID = 0 [1]
//
// The !pid check is paranoid. None of the call sites should end up
// with pid == 0, but better safe than sorry. Let the caller retry
//
    if (!pid) {
    return -EAGAIN;
    }
    p = find_get_task_by_vpid(pid);
    if (!p) {
    return handle_exit_race(uaddr, uval);
    }
    if (unlikely(p.flags & PF_KTHREAD)) {
    put_task_struct(p);
    return -EPERM;
    }
//
// We need to look at the task state to figure out whether the task is
// exiting. To protect against the change of the task state from
// FUTEX_STATE_OK to FUTEX_STATE_EXISTING in futex_cleanup_begin() it is
// required to do this protected by p->pi_lock, which prevents the owner
// from concurrently starting the exit cleanup.
//
// If the state is FUTEX_STATE_OK pi_lock must be held until the waiter
// is attached to protect against a concurrent exit()/exec().
//
    raw_spin_lock_irq(&p.pi_lock);
// Validate that the task is ready for futex operations.
    if (unlikely(p.futex.state != FUTEX_STATE_OK)) {
//
// The task is on the way out. When state is FUTEX_STATE_EXITING
// the cleanup is in progress. To avoid a live lock when the
// waiter preempted the owner, store the task pointer in
// @exiting and keep the reference on the task. The calling code
// will drop all locks, block on @p::futex::exit_mutex and wait
// for the owner to finish the cleanup. Once the owner released
// the mutex the waiter drops the reference count and
// re-evaluates the situation.
//
    if (p.futex.state == FUTEX_STATE_EXITING) {
    raw_spin_unlock_irq(&p.pi_lock);
// exiting = p;
    return -EBUSY;
    }
pub static mut ret: c_int = 0;
    raw_spin_unlock_irq(&p.pi_lock);
    put_task_struct(p);
    return ret;
    }
    if (IS_ENABLED!(CONFIG_MMU) && futex_key_is_private(key)) {
//
// A private futex key holds a pointer to the waiter's mm
// without holding a reference on it. So it must not be attached
// to an owner in a different address space. Otherwise that
// owner's exit cleanup could access the private hash after the
// key's mm is freed.
//
    if (unlikely(p.mm != key.private.mm)) {
    raw_spin_unlock_irq(&p.pi_lock);
    put_task_struct(p);
    return -EPERM;
    }
    }
    __attach_to_pi_owner(p, key, ps);
    raw_spin_unlock_irq(&p.pi_lock);
    put_task_struct(p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lock_pi_update_atomic(uaddr: *mut u32 , uval: u32, newval: u32) -> c_int {
    let mut err = 0;
    let mut curval = 0;
    if (unlikely(should_fail_futex(true))) {
    return -EFAULT;
    }
    err = futex_cmpxchg_value_locked(&curval, uaddr, uval, newval);
    if (unlikely(err)) {
    return err;
    }
// If user space value changed, let the caller retry
    return curval != uval ? -EAGAIN : 0;
    }
//
// futex_lock_pi_atomic() - Atomic work required to acquire a pi aware futex
// @uaddr:		the pi futex user address
// @hb:			the pi futex hash bucket
// @key:		the futex key associated with uaddr and hb
// @ps:			the pi_state pointer where we store the result of the
// lookup
// @task:		the task to perform the atomic lock work for.  This will
// be "current" except in the case of requeue pi.
// @exiting:		Pointer to store the task pointer of the owner task
// which is in the middle of exiting
// @set_waiters:	force setting the FUTEX_WAITERS bit (1) or not (0)
//
// Return:
// -  0 - ready to wait;
// -  1 - acquired the lock;
// - <0 - error
//
// The hb->lock must be held by the caller.
//
// @exiting is only set when the return value is -EBUSY. If so, this holds
// a refcount on the exiting task on return and the caller needs to drop it
// after waiting for the exit to complete.
//
#[no_mangle]
pub unsafe extern "C" fn futex_lock_pi_atomic(uaddr: *mut u32, hb: *mut futex_hash_bucket, key: *mut union futex_key, ps: *mut *mut futex_pi_state, task: *mut task_struct, exiting: *mut *mut task_struct, set_waiters: c_int) -> c_int {
    u32 uval, newval, vpid = task_pid_vnr(task);
pub static mut top_waiter: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
//
// Read the user space value first so we can validate a few
// things before proceeding further.
//
    if (futex_get_value_locked(&uval, uaddr)) {
    return -EFAULT;
    }
    if (unlikely(should_fail_futex(true))) {
    return -EFAULT;
    }
//
// Detect deadlocks.
//
    if ((unlikely((uval & FUTEX_TID_MASK) == vpid))) {
    return -EDEADLK;
    }
    if ((unlikely(should_fail_futex(true)))) {
    return -EDEADLK;
    }
//
// Lookup existing state first. If it exists, try to attach to
// its pi_state.
//
    top_waiter = futex_top_waiter(hb, key);
    if (top_waiter) {
    return attach_to_pi_state(uaddr, uval, top_waiter.pi_state, ps);
    }
//
// No waiter and user TID is 0. We are here because the
// waiters or the owner died bit is set or called from
// requeue_cmp_pi or for whatever reason something took the
// syscall.
//
    if (!(uval & FUTEX_TID_MASK)) {
//
// We take over the futex. No other waiters and the user space
// TID is 0. We preserve the owner died bit.
//
    newval = uval & FUTEX_OWNER_DIED;
    newval |= vpid;
// The futex requeue_pi code can enforce the waiters bit
    if (set_waiters) {
    newval |= FUTEX_WAITERS;
    }
    ret = lock_pi_update_atomic(uaddr, uval, newval);
    if (ret) {
    return ret;
    }
//
// If the waiter bit was requested the caller also needs PI
// state attached to the new owner of the user space futex.
//
// @task is guaranteed to be alive and it cannot be exiting
// because it is either sleeping or waiting in
// futex_requeue_pi_wakeup_sync().
//
// No need to do the full attach_to_pi_owner() exercise
// because @task is known and valid.
//
    if (set_waiters) {
    raw_spin_lock_irq(&task.pi_lock);
    __attach_to_pi_owner(task, key, ps);
    raw_spin_unlock_irq(&task.pi_lock);
    }
    return 1;
    }
//
// First waiter. Set the waiters bit before attaching ourself to
// the owner. If owner tries to unlock, it will be forced into
// the kernel and blocked on hb->lock.
//
    newval = uval | FUTEX_WAITERS;
    ret = lock_pi_update_atomic(uaddr, uval, newval);
    if (ret) {
    return ret;
    }
//
// If the update of the user space value succeeded, we try to
// attach to the owner. If that fails, no harm done, we only
// set the FUTEX_WAITERS bit in the user space variable.
//
    return attach_to_pi_owner(uaddr, newval, key, ps, exiting);
    }
//
// Caller must hold a reference on @pi_state.
//
#[no_mangle]
pub unsafe extern "C" fn wake_futex_pi(uaddr: *mut u32, uval: u32, pi_state: *mut futex_pi_state, wait_lock: *mut rt_mutex_waitertop_waiter)
    __must_hold(&pi_state.pi_mutex.wait_lock)
    __releases(&pi_state.pi_mutex.) -> c_int {
pub static mut new_owner: *mut c_void = core::ptr::null_mut();
pub static mut postunlock: bool = false;
pub static mut wqh: usize = 0;
    u32 curval, newval;
pub static mut ret: c_int = 0;
    new_owner = top_waiter.task;
//
// We pass it to the next owner. The WAITERS bit is always kept
// enabled while there is PI state around. We cleanup the owner
// died bit, because we are the owner.
//
    newval = FUTEX_WAITERS | task_pid_vnr(new_owner);
    if (unlikely(should_fail_futex(true))) {
    ret = -EFAULT;
// goto;
    }
    ret = futex_cmpxchg_value_locked(&curval, uaddr, uval, newval);
    if (!ret && (curval != uval)) {
//
// If a unconditional UNLOCK_PI operation (user space did not
// try the TID->0 transition) raced with a waiter setting the
// FUTEX_WAITERS flag between get_user() and locking the hash
// bucket lock, retry the operation.
//
    if ((FUTEX_TID_MASK & curval) == uval) {
    ret = -EAGAIN;
    }
    else {
    ret = -EINVAL;
    }
    }
    if (!ret) {
//
// This is a point of no return; once we modified the uval
// there is no going back and subsequent operations must
// not fail.
//
    pi_state_update_owner(pi_state, new_owner);
    postunlock = __rt_mutex_futex_unlock(&pi_state.pi_mutex, &wqh);
    }
// label;
    raw_spin_unlock_irq(&pi_state.pi_mutex.wait_lock);
    if (postunlock) {
    rt_mutex_postunlock(&wqh);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __fixup_pi_state_owner(uaddr: *mut u32, q: *mut futex_q, lock_ptr: *mut task_structargowner)
    __must_hold(&q.pi_state.pi_mutex.wait_lock)
    __must_hold(q.) -> c_int {
    let mut pi_state = q.pi_state;
    let mut oldowner = core::ptr::null_mut();
    let mut newowner = core::ptr::null_mut();
    u32 uval, curval, newval, newtid;
pub static mut err: c_int = 0;
    oldowner = pi_state.owner;
//
// We are here because either:
//
// - we stole the lock and pi_state->owner needs updating to reflect
// that (@argowner == current),
//
// or:
//
// - someone stole our lock and we need to fix things to point to the
// new owner (@argowner == NULL).
//
// Either way, we have to replace the TID in the user space variable.
// This must be atomic as we have to preserve the owner died bit here.
//
// Note: We write the user space value _before_ changing the pi_state
// because we can fault here. Imagine swapped out pages or a fork
// that marked all the anonymous memory readonly for cow.
//
// Modifying pi_state _before_ the user space value would leave the
// pi_state in an inconsistent state when we fault here, because we
// need to drop the locks to handle the fault. This might be observed
// in the PID checks when attaching to PI state .
//
// label;
    if (!argowner) {
    if (oldowner != current) {
//
// We raced against a concurrent self; things are
// already fixed up. Nothing to do.
//
    return 0;
    }
    if (__rt_mutex_futex_trylock(&pi_state.pi_mutex)) {
// We got the lock. pi_state is correct. Tell caller.
    return 1;
    }
//
// The trylock just failed, so either there is an owner or
// there is a higher priority waiter than this one.
//
    newowner = rt_mutex_owner(&pi_state.pi_mutex);
//
// If the higher priority waiter has not yet taken over the
// rtmutex then newowner is NULL. We can't return here with
// that state because it's inconsistent vs. the user space
// state. So drop the locks and try again. It's a valid
// situation and not any different from the other retry
// conditions.
//
    if (unlikely(!newowner)) {
    err = -EAGAIN;
// goto;
    }
    } else {
    WARN_ON_ONCE!(argowner != current);
    if (oldowner == current) {
//
// We raced against a concurrent self; things are
// already fixed up. Nothing to do.
//
    return 1;
    }
    newowner = argowner;
    }
    newtid = task_pid_vnr(newowner) | FUTEX_WAITERS;
// Owner died?
    if (!pi_state.owner) {
    newtid |= FUTEX_OWNER_DIED;
    }
    err = futex_get_value_locked(&uval, uaddr);
    if (err) {
// goto;
    }
    for (;;) {
    newval = (uval & FUTEX_OWNER_DIED) | newtid;
    err = futex_cmpxchg_value_locked(&curval, uaddr, uval, newval);
    if (err) {
// goto;
    }
    if (curval == uval) {
    break;
    }
    uval = curval;
    }
//
// We fixed up user space. Now we need to fix the pi_state
// itself.
//
    pi_state_update_owner(pi_state, newowner);
pub static mut argowner: return = 0;
//
// In order to reschedule or handle a page fault, we need to drop the
// locks here. In the case of a fault, this gives the other task
// (either the highest priority waiter itself or the task which stole
// the rtmutex) the chance to try the fixup of the pi_state. So once we
// are back from handling the fault we need to check the pi_state after
// reacquiring the locks and before trying to do another fixup. When
// the fixup has been done already we simply return.
//
// Note: we hold both hb->lock and pi_mutex->wait_lock. We can safely
// drop hb->lock since the caller owns the hb -> futex_q relation.
// Dropping the pi_mutex->wait_lock requires the state revalidate.
//
// label;
    raw_spin_unlock_irq(&pi_state.pi_mutex.wait_lock);
    spin_unlock(q.lock_ptr);
    match (err) {
    -EFAULT => {
    err = fault_in_user_writeable(uaddr);
    // break;
    }
    -EAGAIN => {
    cond_resched();
    err = 0;
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    // break;
    }
    }
    futex_q_lockptr_lock(q);
    raw_spin_lock_irq(&pi_state.pi_mutex.wait_lock);
//
// Check if someone else fixed it for us:
//
    if (pi_state.owner != oldowner) {
pub static mut argowner: return = 0;
    }
// Retry if err was -EAGAIN or the fault in succeeded
    if (!err) {
// goto;
    }
//
// fault_in_user_writeable() failed so user state is immutable. At
// best we can make the kernel state consistent but user state will
// be most likely hosed and any subsequent unlock operation will be
// rejected due to PI futex rule [10].
//
// Ensure that the rtmutex owner is also the pi_state owner despite
// the user space value claiming something different. There is no
// point in unlocking the rtmutex if current is the owner as it
// would need to wait until the next waiter has taken the rtmutex
// to guarantee consistent state. Keep it simple. Userspace asked
// for this wreckaged state.
//
// The rtmutex has an owner - either current or some other
// task. See the EAGAIN loop above.
//
    pi_state_update_owner(pi_state, rt_mutex_owner(&pi_state.pi_mutex));
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_pi_state_owner(uaddr: *mut u32, q: *mut futex_q, argowner: *mut task_struct) -> c_int {
    let mut pi_state = q.pi_state;
    let mut ret = 0;
    lockdep_assert_held(q.lock_ptr);
    raw_spin_lock_irq(&pi_state.pi_mutex.wait_lock);
    ret = __fixup_pi_state_owner(uaddr, q, argowner);
    raw_spin_unlock_irq(&pi_state.pi_mutex.wait_lock);
    return ret;
    }
//
// fixup_pi_owner() - Post lock pi_state and corner case management
// @uaddr:	user address of the futex
// @q:		futex_q (contains pi_state and access to the rt_mutex)
// @locked:	if the attempt to take the rt_mutex succeeded (1) or not (0)
//
// After attempting to lock an rt_mutex, this function is called to cleanup
// the pi_state owner as well as handle race conditions that may allow us to
// acquire the lock. Must be called with the hb lock held.
//
// Return:
// -  1 - success, lock taken;
// -  0 - success, lock not taken;
// - <0 - on error (-EFAULT)
//
#[no_mangle]
pub unsafe extern "C" fn fixup_pi_owner(uaddr: *mut u32 , q: *mut futex_q, locked: c_int) -> c_int {
    if (locked) {
//
// Got the lock. We might not be the anticipated owner if we
// did a lock-steal - fix up the PI-state in that case:
//
// Speculative pi_state->owner read (we don't hold wait_lock);
// since we own the lock pi_state->owner == current is the
// stable state, anything else needs more attention.
//
    if (q.pi_state.owner != current) {
    return fixup_pi_state_owner(uaddr, q, current);
    }
    return 1;
    }
//
// If we didn't get the lock; check if anybody stole it from us. In
// that case, we need to fix up the uval to point to them instead of
// us, otherwise bad things happen. [10]
//
// Another speculative read; pi_state->owner == current is unstable
// but needs our attention.
//
    if (q.pi_state.owner == current) {
    return fixup_pi_state_owner(uaddr, q, core::ptr::null_mut());
    }
//
// Paranoia check. If we did not take the lock, then we should not be
// the owner of the rt_mutex. Warn and establish consistent state.
//
    if (WARN_ON_ONCE!(rt_mutex_owner(&q.pi_state.pi_mutex) == current)) {
    return fixup_pi_state_owner(uaddr, q, current);
    }
    return 0;
    }
//
// Userspace tried a 0 -> TID atomic transition of the futex value
// and failed. The kernel side here does the whole locking operation:
// if there are waiters then it will block as a consequence of relying
// on rt-mutexes, it does PI, etc. (Due to races the kernel might see
// a 0 value of the futex too.).
//
// Also serves as futex trylock_pi()'ing, and due semantics.
//
#[no_mangle]
pub unsafe extern "C" fn futex_lock_pi(uaddr: *mut u32 , flags: c_uint, time: *mut ktime_t, trylock: c_int) -> c_int {
    struct hrtimer_sleeper timeout, *to;
pub static mut exiting: *mut c_void = core::ptr::null_mut();
pub static mut rt_waiter: usize = 0;
pub static mut q: futex_q = 0;
pub static mut wake_q: usize = 0;
    let mut res = 0;
    let mut ret = 0;
    if (!IS_ENABLED!(CONFIG_FUTEX_PI)) {
    return -ENOSYS;
    }
    if (refill_pi_state_cache()) {
    return -ENOMEM;
    }
    to = futex_setup_timer(time, &timeout, flags, 0);
// label;
    exiting = core::ptr::null_mut();
    ret = get_futex_key(uaddr, flags, &q.key, FUTEX_WRITE);
    if (unlikely(ret != 0)) {
// goto;
    }
// label;
    if (1) {
    CLASS(hbr, hbr)(&q.key);
pub static mut hb: auto = 0;
    futex_q_lock(&q, hb);
    ret = futex_lock_pi_atomic(uaddr, hb, &q.key, &q.pi_state, current,
    &exiting, 0);
    if (unlikely(ret)) {
//
// Atomic work succeeded and we got the lock,
// or failed. Either way, we do _not_ block.
//
    match (ret) {
    1 => {
// We got the lock.
    ret = 0;
// goto;
    }
    -EFAULT => {
// goto;
    }
    -EBUSY => {
    }
    -EAGAIN => {
//
// Two reasons for this:
// - EBUSY: Task is exiting and we just wait for the
// exit to complete.
// - EAGAIN: The user space value changed.
//
    futex_q_unlock(hb);
    __release(q.lock_ptr);
//
// Handle the case where the owner is in the middle of
// exiting. Wait for the exit to complete otherwise
// this task might loop forever, aka. live lock.
//
    wait_for_owner_exiting(ret, exiting);
    cond_resched();
// goto;
    }
    _ => {
// goto;
    }
    }
    }
    WARN_ON!(!q.pi_state);
//
// Only actually queue now that the atomic ops are done:
//
    __futex_queue(&q, hb, current);
    if (trylock) {
    ret = rt_mutex_futex_trylock(&q.pi_state.pi_mutex);
// Fixup the trylock return value:
    ret = ret ? 0 : -EWOULDBLOCK;
// goto;
    }
//
// Caution; releasing @hb in-scope. The hb->lock is still locked
// while the reference is dropped. The reference can not be dropped
// after the unlock because if a user initiated resize is in progress
// then we might need to wake him. This can not be done after the
// rt_mutex_pre_schedule() invocation. The hb will remain valid because
// the thread, performing resize, will block on hb->lock during
// the requeue.
//
    futex_private_hash_put(no_free_ptr(hbr.fph));
//
// Must be done before we enqueue the waiter, here is unfortunately
// under the hb lock, but that *should* work because it does nothing.
//
    rt_mutex_pre_schedule();
    rt_mutex_init_waiter(&rt_waiter);
//
// On PREEMPT_RT, when hb->lock becomes an rt_mutex, we must not
// hold it while doing rt_mutex_start_proxy(), because then it will
// include hb->lock in the blocking chain, even through we'll not in
// fact hold it while blocking. This will lead it to report -EDEADLK
// and BUG when futex_unlock_pi() interleaves with this.
//
// Therefore acquire wait_lock while holding hb->lock, but drop the
// latter before calling __rt_mutex_start_proxy_lock(). This
// interleaves with futex_unlock_pi() -- which does a similar lock
// handoff -- such that the latter can observe the futex_q::pi_state
// before __rt_mutex_start_proxy_lock() is done.
//
    raw_spin_lock_irq(&q.pi_state.pi_mutex.wait_lock);
    spin_unlock(q.lock_ptr);
//
// __rt_mutex_start_proxy_lock() unconditionally enqueues the @rt_waiter
// such that futex_unlock_pi() is guaranteed to observe the waiter when
// it sees the futex_q::pi_state.
//
    ret = __rt_mutex_start_proxy_lock(&q.pi_state.pi_mutex, &rt_waiter, current, &wake_q);
    raw_spin_unlock_irq_wake(&q.pi_state.pi_mutex.wait_lock, &wake_q);
    if (ret) {
    if (ret == 1) {
    ret = 0;
    }
// goto;
    }
    if (unlikely(to)) {
    hrtimer_sleeper_start_expires(to, HRTIMER_MODE_ABS);
    }
    ret = rt_mutex_wait_proxy_lock(&q.pi_state.pi_mutex, to, &rt_waiter);
// label;
//
// If we failed to acquire the lock (deadlock/signal/timeout), we must
// unwind the above, however we canont lock hb->lock because
// rt_mutex already has a waiter enqueued and hb->lock can itself try
// and enqueue an rt_waiter through rtlock.
//
// Doing the cleanup without holding hb->lock can cause inconsistent
// state between hb and pi_state, but only in the direction of not
// seeing a waiter that is leaving.
//
// See futex_unlock_pi(), it deals with this inconsistency.
//
// There be dragons here, since we must deal with the inconsistency on
// the way out (here), it is impossible to detect/warn about the race
// the other way around (missing an incoming waiter).
//
// What could possibly go wrong...
//
    if (ret && !rt_mutex_cleanup_proxy_lock(&q.pi_state.pi_mutex, &rt_waiter)) {
    ret = 0;
    }
//
// Now that the rt_waiter has been dequeued, it is safe to use
// spinlock/rtlock (which might enqueue its own rt_waiter) and fix up
// the
//
    futex_q_lockptr_lock(&q);
//
// Waiter is unqueued.
//
    rt_mutex_post_schedule();
// label;
//
// Fixup the pi_state owner and possibly acquire the lock if we
// haven't already.
//
    res = fixup_pi_owner(uaddr, &q, !ret);
//
// If fixup_pi_owner() returned an error, propagate that.  If it acquired
// the lock, clear our -ETIMEDOUT or -EINTR.
//
    if (res) {
    ret = (res < 0) ? res : 0;
    }
    __release(&hb.lock);
    futex_unqueue_pi(&q);
    spin_unlock(q.lock_ptr);
// Additional reference from futex_unlock_pi()
    futex_private_hash_put(q.drop_fph);
// goto;
// label;
    futex_q_unlock(hb);
    __release(q.lock_ptr);
// goto;
// label;
    futex_q_unlock(hb);
    __release(q.lock_ptr);
    ret = fault_in_user_writeable(uaddr);
    if (ret) {
// goto;
    }
    if (!(flags & FLAGS_SHARED)) {
// goto;
    }
// goto;
    }
// label;
    if (to) {
    hrtimer_cancel(&to.timer);
    destroy_hrtimer_on_stack(&to.timer);
    }
    return ret != -EINTR ? ret : -ERESTARTNOINTR;
    }
//
// Userspace attempted a TID -> 0 atomic transition, and failed.
// This is the in-kernel slowpath: we look up the PI state (if any),
// and do the rt-mutex unlock.
//
#[no_mangle]
unsafe extern "C" fn __futex_unlock_pi(uaddr: *mut u32 , flags: c_uint) -> c_int {
    u32 curval, uval, vpid = task_pid_vnr(current);
pub static mut key: union futex_key = 0;
pub static mut top_waiter: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!IS_ENABLED!(CONFIG_FUTEX_PI)) {
    return -ENOSYS;
    }
// label;
    if (get_user(uval, uaddr)) {
    return -EFAULT;
    }
//
// We release only a lock we actually own:
//
    if ((uval & FUTEX_TID_MASK) != vpid) {
    return -EPERM;
    }
    ret = get_futex_key(uaddr, flags, &key, FUTEX_WRITE);
    if (ret) {
    return ret;
    }
    CLASS(hbr, hbr)(&key);
pub static mut hb: auto = 0;
    spin_lock(&hb.lock);
// label;
//
// Check waiters first. We do not trust user space values at
// all and we at least want to know if user space fiddled
// with the futex value instead of blindly unlocking.
//
    top_waiter = futex_top_waiter(hb, &key);
    if (top_waiter) {
    let mut pi_state = top_waiter.pi_state;
pub static mut rt_waiter: *mut c_void = core::ptr::null_mut();
    ret = -EINVAL;
    if (!pi_state) {
// goto;
    }
//
// If current does not own the pi_state then the futex is
// inconsistent and user space fiddled with the futex value.
//
    if (pi_state.owner != current) {
// goto;
    }
//
// By taking wait_lock while still holding hb->lock, we ensure
// there is no point where we hold neither; and thereby
// wake_futex_pi() must observe any new waiters.
//
// Since the cleanup: case in futex_lock_pi() removes the
// rt_waiter without holding hb->lock, it is possible for
// wake_futex_pi() to not find a waiter while the above does,
// in this case the waiter is on the way out and it can be
// ignored.
//
// In particular; this forces __rt_mutex_start_proxy() to
// complete such that we're guaranteed to observe the
// rt_waiter.
//
    raw_spin_lock_irq(&pi_state.pi_mutex.wait_lock);
//
// Futex vs rt_mutex waiter state -- if there are no rt_mutex
// waiters even though futex thinks there are, then the waiter
// is leaving. The entry needs to be removed from the list so a
// new futex_lock_pi() is not using this stale PI-state while
// the futex is available in user space again.
// There can be more than one task on its way out so it needs
// to retry.
//
    rt_waiter = rt_mutex_top_waiter(&pi_state.pi_mutex);
    if (!rt_waiter) {
//
// Acquire a reference for the leaving waiter to ensure
// valid futex_q::lock_ptr.
//
    if (futex_key_is_private(&key)) {
    top_waiter.drop_fph = futex_private_hash(key.private.mm);
    }
    __futex_unqueue(top_waiter);
    raw_spin_unlock_irq(&pi_state.pi_mutex.wait_lock);
// goto;
    }
    get_pi_state(pi_state);
    spin_unlock(&hb.lock);
// drops pi_state->pi_mutex.wait_lock
    ret = wake_futex_pi(uaddr, uval, pi_state, rt_waiter);
    put_pi_state(pi_state);
//
// Success, we're done! No tricky corner cases.
//
    if (!ret) {
    return ret;
    }
//
// The atomic access to the futex value generated a
// pagefault, so retry the user-access and the wakeup:
//
    if (ret == -EFAULT) {
// goto;
    }
//
// A unconditional UNLOCK_PI op raced against a waiter
// setting the FUTEX_WAITERS bit. Try again.
//
    if (ret == -EAGAIN) {
// goto;
    }
//
// wake_futex_pi has detected invalid state. Tell user
// space.
//
    return ret;
    }
//
// We have no kernel internal state, i.e. no waiters in the
// kernel. Waiters which are about to queue themselves are stuck
// on hb->lock. So we can safely ignore them. We do neither
// preserve the WAITERS bit not the OWNER_DIED one. We are the
// owner.
//
    if ((ret = futex_cmpxchg_value_locked(&curval, uaddr, uval, 0))) {
    spin_unlock(&hb.lock);
    match (ret) {
    -EFAULT => {
// goto;
    }
    -EAGAIN => {
// goto;
    }
    _ => {
    WARN_ON_ONCE!(1);
    return ret;
    }
    }
    }
//
// If uval has changed, let user space handle it.
//
    ret = (curval == uval) ? 0 : -EAGAIN;
// label;
    spin_unlock(&hb.lock);
    return ret;
// label;
    cond_resched();
// goto;
// label;
    ret = fault_in_user_writeable(uaddr);
    if (!ret) {
// goto;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn futex_unlock_pi(uaddr: *mut u32 , flags: c_uint, pop: *mut c_void ) -> c_int {
pub static mut ret: c_int = 0;
    if (ret || !(flags & FLAGS_ROBUST_UNLOCK)) {
    return ret;
    }
    if (!futex_robust_list_clear_pending(pop, flags)) {
    return -EFAULT;
    }
    return 0;
    }
}
