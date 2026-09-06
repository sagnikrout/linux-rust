//! Automatically rewritten from C to Rust
//! Source: kernel/futex/requeue.c
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
// On PREEMPT_RT, the hash bucket lock is a 'sleeping' spinlock with an
// underlying rtmutex. The task which is about to be requeued could have
// just woken up (timeout, signal). After the wake up the task has to
// acquire hash bucket lock, which is held by the requeue code.  As a task
// can only be blocked on _ONE_ rtmutex at a time, the proxy lock blocking
// and the hash bucket lock blocking would collide and corrupt state.
//
// On !PREEMPT_RT this is not a problem and everything could be serialized
// on hash bucket lock, but aside of having the benefit of common code,
// this allows to avoid doing the requeue when the task is already on the
// way out and taking the hash bucket lock of the original uaddr1 when the
// requeue has been completed.
//
// The following state transitions are valid:
//
// On the waiter side:
// Q_REQUEUE_PI_NONE		-> Q_REQUEUE_PI_IGNORE
// Q_REQUEUE_PI_IN_PROGRESS	-> Q_REQUEUE_PI_WAIT
//
// On the requeue side:
// Q_REQUEUE_PI_NONE		-> Q_REQUEUE_PI_INPROGRESS
// Q_REQUEUE_PI_IN_PROGRESS	-> Q_REQUEUE_PI_DONE/LOCKED
// Q_REQUEUE_PI_IN_PROGRESS	-> Q_REQUEUE_PI_NONE (requeue failed)
// Q_REQUEUE_PI_WAIT		-> Q_REQUEUE_PI_DONE/LOCKED
// Q_REQUEUE_PI_WAIT		-> Q_REQUEUE_PI_IGNORE (requeue failed)
//
// The requeue side ignores a waiter with state Q_REQUEUE_PI_IGNORE as this
// signals that the waiter is already on the way out. It also means that
// the waiter is still on the 'wait' futex, i.e. uaddr1.
//
// The waiter side signals early wakeup to the requeue side either through
// setting state to Q_REQUEUE_PI_IGNORE or to Q_REQUEUE_PI_WAIT depending
// on the current state. In case of Q_REQUEUE_PI_IGNORE it can immediately
// proceed to take the hash bucket lock of uaddr1. If it set state to WAIT,
// which means the wakeup is interleaving with a requeue in progress it has
// to wait for the requeue side to change the state. Either to DONE/LOCKED
// or to IGNORE. DONE/LOCKED means the waiter q is now on the uaddr2 futex
// and either blocked (DONE) or has acquired it (LOCKED). IGNORE is set by
// the requeue side when the requeue attempt failed via deadlock detection
// and therefore the waiter q is still on the uaddr1 futex.
//
    enum {
    Q_REQUEUE_PI_NONE		=  0,
    Q_REQUEUE_PI_IGNORE,
    Q_REQUEUE_PI_IN_PROGRESS,
    Q_REQUEUE_PI_WAIT,
    Q_REQUEUE_PI_DONE,
    Q_REQUEUE_PI_LOCKED,
    };
pub static mut futex_q: usize = 0;
//
// requeue_futex() - Requeue a futex_q from one hb to another
// @q:		the futex_q to requeue
// @hb1:	the source hash_bucket
// @hb2:	the target hash_bucket
// @key2:	the new key for the requeued futex_q
//
#[no_mangle]
pub unsafe extern "C" fn requeue_futex(q: *mut futex_q, hb1: *mut futex_hash_bucket, hb2: *mut futex_hash_bucket, key2: *mut union futex_key) {
//
// If key1 and key2 hash to the same bucket, no need to
// requeue.
//
    if (likely(&hb1.chain != &hb2.chain)) {
    plist_del(&q.list, &hb1.chain);
    futex_hb_waiters_dec(hb1);
    futex_hb_waiters_inc(hb2);
    plist_add(&q.list, &hb2.chain);
    q.lock_ptr = &hb2.lock;
//
// hb1 and hb2 belong to the same futex_hash_bucket_private
// because if we managed get a reference on hb1 then it can't be
// replaced. Therefore we avoid put(hb1)+get(hb2) here.
//
    }
    q.key = *key2;
    }
#[no_mangle]
pub unsafe extern "C" fn futex_requeue_pi_prepare(q: *mut futex_q, pi_state: *mut futex_pi_state) -> bool {
    let mut old = 0;
    let mut new = 0;
//
// Set state to Q_REQUEUE_PI_IN_PROGRESS unless an early wakeup has
// already set Q_REQUEUE_PI_IGNORE to signal that requeue should
// ignore the waiter.
//
    old = atomic_read_acquire(&q.requeue_state);
    do {
    if (old == Q_REQUEUE_PI_IGNORE) {
    return false;
    }
//
// futex_proxy_trylock_atomic() might have set it to
// IN_PROGRESS and a interleaved early wake to WAIT.
//
// It was considered to have an extra state for that
// trylock, but that would just add more conditionals
// all over the place for a dubious value.
//
    if (old != Q_REQUEUE_PI_NONE) {
    break;
    }
    new = Q_REQUEUE_PI_IN_PROGRESS;
    } while (!atomic_try_cmpxchg(&q.requeue_state, &old, new));
    q.pi_state = pi_state;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn futex_requeue_pi_complete(q: *mut futex_q, locked: c_int) {
    let mut old = 0;
    let mut new = 0;
    old = atomic_read_acquire(&q.requeue_state);
    do {
    if (old == Q_REQUEUE_PI_IGNORE) {
    return;
    }
    if (locked >= 0) {
// Requeue succeeded. Set DONE or LOCKED
    WARN_ON_ONCE!(old != Q_REQUEUE_PI_IN_PROGRESS &&
    old != Q_REQUEUE_PI_WAIT);
    new = Q_REQUEUE_PI_DONE + locked;
    } else if (old == Q_REQUEUE_PI_IN_PROGRESS) {
// Deadlock, no early wakeup interleave
    new = Q_REQUEUE_PI_NONE;
    } else {
// Deadlock, early wakeup interleave.
    WARN_ON_ONCE!(old != Q_REQUEUE_PI_WAIT);
    new = Q_REQUEUE_PI_IGNORE;
    }
    } while (!atomic_try_cmpxchg(&q.requeue_state, &old, new));

// If the waiter interleaved with the requeue let it know
    if (unlikely(old == Q_REQUEUE_PI_WAIT)) {
    rcuwait_wake_up(&q.requeue_wait);
    }

    }
#[no_mangle]
pub unsafe extern "C" fn futex_requeue_pi_wakeup_sync(q: *mut futex_q) -> c_int {
    let mut old = 0;
    let mut new = 0;
    old = atomic_read_acquire(&q.requeue_state);
    do {
// Is requeue done already?
    if (old >= Q_REQUEUE_PI_DONE) {
    return old;
    }
//
// If not done, then tell the requeue code to either ignore
// the waiter or to wake it up once the requeue is done.
//
    new = Q_REQUEUE_PI_WAIT;
    if (old == Q_REQUEUE_PI_NONE) {
    new = Q_REQUEUE_PI_IGNORE;
    }
    } while (!atomic_try_cmpxchg(&q.requeue_state, &old, new));
// If the requeue was in progress, wait for it to complete
    if (old == Q_REQUEUE_PI_IN_PROGRESS) {

    rcuwait_wait_event(&q.requeue_wait,
    atomic_read(&q.requeue_state) != Q_REQUEUE_PI_WAIT,
    TASK_UNINTERRUPTIBLE);

    (void)atomic_cond_read_relaxed(&q.requeue_state, VAL != Q_REQUEUE_PI_WAIT);

    }
//
// Requeue is now either prohibited or complete. Reread state
// because during the wait above it might have changed. Nothing
// will modify q->requeue_state after this point.
//
    return atomic_read(&q.requeue_state);
    }
//
// requeue_pi_wake_futex() - Wake a task that acquired the lock during requeue
// @q:		the futex_q
// @key:	the key of the requeue target futex
// @hb:		the hash_bucket of the requeue target futex
//
// During futex_requeue, with requeue_pi=1, it is possible to acquire the
// target futex if it is uncontended or via a lock steal.
//
// 1) Set @q::key to the requeue target futex key so the waiter can detect
// the wakeup on the right futex.
//
// 2) Dequeue @q from the hash bucket.
//
// 3) Set @q::rt_waiter to NULL so the woken up task can detect atomic lock
// acquisition.
//
// 4) Set the q->lock_ptr to the requeue target hb->lock for the case that
// the waiter has to fixup the pi state.
//
// 5) Complete the requeue state so the waiter can make progress. After
// this point the waiter task can return from the syscall immediately in
// case that the pi state does not have to be fixed up.
//
// 6) Wake the waiter task.
//
// Must be called with both q->lock_ptr and hb->lock held.
//
#[no_mangle]
pub unsafe extern "C" fn requeue_pi_wake_futex(q: *mut futex_q, key: *mut union futex_key, hb: *mut futex_hash_bucket) {
pub static mut task: *mut c_void = core::ptr::null_mut();
    q.key = *key;
    __futex_unqueue(q);
    WARN_ON!(!q.rt_waiter);
    q.rt_waiter = core::ptr::null_mut();
//
// Acquire a reference for the waiter to ensure valid
// futex_q::lock_ptr.
//
    if (futex_key_is_private(key)) {
    q.drop_fph = futex_private_hash(key.private.mm);
    }
    q.lock_ptr = &hb.lock;
    task = READ_ONCE(q.task);
// Signal locked state to the waiter
    futex_requeue_pi_complete(q, 1);
    wake_up_state(task, TASK_NORMAL);
    }
//
// futex_proxy_trylock_atomic() - Attempt an atomic lock for the top waiter
// @pifutex:		the user address of the to futex
// @hb1:		the from futex hash bucket, must be locked by the caller
// @hb2:		the to futex hash bucket, must be locked by the caller
// @key1:		the from futex key
// @key2:		the to futex key
// @ps:			address to store the pi_state pointer
// @exiting:		Pointer to store the task pointer of the owner task
// which is in the middle of exiting
// @set_waiters:	force setting the FUTEX_WAITERS bit (1) or not (0)
//
// Try and get the lock on behalf of the top waiter if we can do it atomically.
// Wake the top waiter if we succeed.  If the caller specified set_waiters,
// then direct futex_lock_pi_atomic() to force setting the FUTEX_WAITERS bit.
// hb1 and hb2 must be held by the caller.
//
// @exiting is only set when the return value is -EBUSY. If so, this holds
// a refcount on the exiting task on return and the caller needs to drop it
// after waiting for the exit to complete.
//
// Return:
// -  0 - failed to acquire the lock atomically;
// - >0 - acquired the lock, return value is vpid of the top_waiter
// - <0 - error
//
#[no_mangle]
pub unsafe extern "C" fn futex_proxy_trylock_atomic(pifutex: *mut u32, hb1: *mut futex_hash_bucket, hb2: *mut futex_hash_bucket, key1: *mut union futex_key, key2: *mut union futex_key, ps: *mut *mut futex_pi_state, exiting: *mut *mut task_struct, set_waiters: c_int) -> c_int {
pub static mut top_waiter: *mut c_void = core::ptr::null_mut();
    let mut curval = 0;
    let mut ret = 0;
    if (futex_get_value_locked(&curval, pifutex)) {
    return -EFAULT;
    }
    if (unlikely(should_fail_futex(true))) {
    return -EFAULT;
    }
//
// Find the top_waiter and determine if there are additional waiters.
// If the caller intends to requeue more than 1 waiter to pifutex,
// force futex_lock_pi_atomic() to set the FUTEX_WAITERS bit now,
// as we have means to handle the possible fault.  If not, don't set
// the bit unnecessarily as it will force the subsequent unlock to enter
// the kernel.
//
    top_waiter = futex_top_waiter(hb1, key1);
// There are no waiters, nothing for us to do.
    if (!top_waiter) {
    return 0;
    }
//
// Ensure that this is a waiter sitting in futex_wait_requeue_pi()
// and waiting on the 'waitqueue' futex which is always !PI.
//
    if (!top_waiter.rt_waiter || top_waiter.pi_state) {
    return -EINVAL;
    }
// Ensure we requeue to the expected futex.
    if (!futex_match(top_waiter.requeue_pi_key, key2)) {
    return -EINVAL;
    }
// Ensure that this does not race against an early wakeup
    if (!futex_requeue_pi_prepare(top_waiter, core::ptr::null_mut())) {
    plist_del(&top_waiter.list, &hb1.chain);
    futex_hb_waiters_dec(hb1);
    return -EAGAIN;
    }
//
// Try to take the lock for top_waiter and set the FUTEX_WAITERS bit
// in the contended case or if @set_waiters is true.
//
// In the contended case PI state is attached to the lock owner. If
// the user space lock can be acquired then PI state is attached to
// the new owner (@top_waiter->task) when @set_waiters is true.
//
    ret = futex_lock_pi_atomic(pifutex, hb2, key2, ps, top_waiter.task,
    exiting, set_waiters);
    if (ret == 1) {
//
// Lock was acquired in user space and PI state was
// attached to @top_waiter->task. That means state is fully
// consistent and the waiter can return to user space
// immediately after the wakeup.
//
    requeue_pi_wake_futex(top_waiter, key2, hb2);
    } else if (ret < 0) {
// Rewind top_waiter::requeue_state
    futex_requeue_pi_complete(top_waiter, ret);
    } else {
//
// futex_lock_pi_atomic() did not acquire the user space
// futex, but managed to establish the proxy lock and pi
// state. top_waiter::requeue_state cannot be fixed up here
// because the waiter is not enqueued on the rtmutex
// yet. This is handled at the callsite depending on the
// result of rt_mutex_start_proxy_lock() which is
// guaranteed to be reached with this function returning 0.
//
    }
    return ret;
    }
//
// futex_requeue() - Requeue waiters from uaddr1 to uaddr2
// @uaddr1:	source futex user address
// @flags1:	futex flags (FLAGS_SHARED, etc.)
// @uaddr2:	target futex user address
// @flags2:	futex flags (FLAGS_SHARED, etc.)
// @nr_wake:	number of waiters to wake (must be 1 for requeue_pi)
// @nr_requeue:	number of waiters to requeue (0-INT_MAX)
// @cmpval:	@uaddr1 expected value (or %NULL)
// @requeue_pi:	if we are attempting to requeue from a non-pi futex to a
// pi futex (pi to pi requeue is not supported)
//
// Requeue waiters on uaddr1 to uaddr2. In the requeue_pi case, try to acquire
// uaddr2 atomically on behalf of the top waiter.
//
// Return:
// - >=0 - on success, the number of tasks requeued or woken;
// -  <0 - on error
//
#[no_mangle]
pub unsafe extern "C" fn futex_requeue(uaddr1: *mut u32, flags1: c_uint, uaddr2: *mut u32, flags2: c_uint, nr_wake: c_int, nr_requeue: c_int, cmpval: *mut u32, requeue_pi: c_int) -> c_int {
pub static mut key1: union futex_key = 0;
pub static mut task_count: c_int = 0;
    let mut pi_state = core::ptr::null_mut();
    let mut this = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut wake_q: usize = 0;
    if (nr_wake < 0 || nr_requeue < 0) {
    return -EINVAL;
    }
//
// When PI not supported: return -ENOSYS if requeue_pi is true,
// consequently the compiler knows requeue_pi is always false past
// this point which will optimize away all the conditional code
// further down.
//
    if (!IS_ENABLED!(CONFIG_FUTEX_PI) && requeue_pi) {
    return -ENOSYS;
    }
    if (requeue_pi) {
//
// Requeue PI only works on two distinct uaddrs. This
// check is only valid for private futexes. See below.
//
    if (uaddr1 == uaddr2) {
    return -EINVAL;
    }
//
// futex_requeue() allows the caller to define the number
// of waiters to wake up via the @nr_wake argument. With
// REQUEUE_PI, waking up more than one waiter is creating
// more problems than it solves. Waking up a waiter makes
// only sense if the PI futex @uaddr2 is uncontended as
// this allows the requeue code to acquire the futex
// @uaddr2 before waking the waiter. The waiter can then
// return to user space without further action. A secondary
// wakeup would just make the futex_wait_requeue_pi()
// handling more complex, because that code would have to
// look up pi_state and do more or less all the handling
// which the requeue code has to do for the to be requeued
// waiters. So restrict the number of waiters to wake to
// one, and only wake it up when the PI futex is
// uncontended. Otherwise requeue it and let the unlock of
// the PI futex handle the wakeup.
//
// All REQUEUE_PI users, e.g. pthread_cond_signal() and
// pthread_cond_broadcast() must use nr_wake=1.
//
    if (nr_wake != 1) {
    return -EINVAL;
    }
//
// requeue_pi requires a pi_state, try to allocate it now
// without any locks in case it fails.
//
    if (refill_pi_state_cache()) {
    return -ENOMEM;
    }
    }
// label;
    ret = get_futex_key(uaddr1, flags1, &key1, FUTEX_READ);
    if (unlikely(ret != 0)) {
    return ret;
    }
    ret = get_futex_key(uaddr2, flags2, &key2,
    requeue_pi ? FUTEX_WRITE : FUTEX_READ);
    if (unlikely(ret != 0)) {
    return ret;
    }
//
// The check above which compares uaddrs is not sufficient for
// shared futexes. We need to compare the keys:
//
    if (requeue_pi && futex_match(&key1, &key2)) {
    return -EINVAL;
    }
// label;
    if (1) {
    CLASS(hbr, hbr1)(&key1);
    CLASS(hbr, hbr2)(&key2);
pub static mut hb1: auto = 0;
pub static mut hb2: auto = 0;
    futex_hb_waiters_inc(hb2);
    double_lock_hb(hb1, hb2);
    if (likely(cmpval != core::ptr::null_mut())) {
    let mut curval = 0;
    ret = futex_get_value_locked(&curval, uaddr1);
    if (unlikely(ret)) {
    futex_hb_waiters_dec(hb2);
    double_unlock_hb(hb1, hb2);
    ret = get_user(curval, uaddr1);
    if (ret) {
    return ret;
    }
    if (!(flags1 & FLAGS_SHARED)) {
// goto;
    }
// goto;
    }
    if (curval != *cmpval) {
    ret = -EAGAIN;
// goto;
    }
    }
    if (requeue_pi) {
    let mut exiting = core::ptr::null_mut();
//
// Attempt to acquire uaddr2 and wake the top waiter. If we
// intend to requeue waiters, force setting the FUTEX_WAITERS
// bit.  We force this here where we are able to easily handle
// faults rather in the requeue loop below.
//
// Updates topwaiter::requeue_state if a top waiter exists.
//
    ret = futex_proxy_trylock_atomic(uaddr2, hb1, hb2, &key1,
    &key2, &pi_state,
    &exiting, nr_requeue);
//
// At this point the top_waiter has either taken uaddr2 or
// is waiting on it. In both cases pi_state has been
// established and an initial refcount on it. In case of an
// error there's nothing.
//
// The top waiter's requeue_state is up to date:
//
// - If the lock was acquired atomically (ret == 1), then
// the state is Q_REQUEUE_PI_LOCKED.
//
// The top waiter has been dequeued and woken up and can
// return to user space immediately. The kernel/user
// space state is consistent. In case that there must be
// more waiters requeued the WAITERS bit in the user
// space futex is set so the top waiter task has to go
// into the syscall slowpath to unlock the futex. This
// will block until this requeue operation has been
// completed and the hash bucket locks have been
// dropped.
//
// - If the trylock failed with an error (ret < 0) then
// the state is either Q_REQUEUE_PI_NONE, i.e. "nothing
// happened", or Q_REQUEUE_PI_IGNORE when there was an
// interleaved early wakeup.
//
// - If the trylock did not succeed (ret == 0) then the
// state is either Q_REQUEUE_PI_IN_PROGRESS or
// Q_REQUEUE_PI_WAIT if an early wakeup interleaved.
// This will be cleaned up in the loop below, which
// cannot fail because futex_proxy_trylock_atomic() did
// the same sanity checks for requeue_pi as the loop
// below does.
//
    match (ret) {
    0 => {
// We hold a reference on the pi state.
    // break;
    }
    1 => {
//
// futex_proxy_trylock_atomic() acquired the user space
// futex. Adjust task_count.
//
    task_count += 1;
    ret = 0;
    // break;
//
// If the above failed, then pi_state is NULL and
// waiter::requeue_state is correct.
//
    }
    -EFAULT => {
    futex_hb_waiters_dec(hb2);
    double_unlock_hb(hb1, hb2);
    ret = fault_in_user_writeable(uaddr2);
    if (!ret) {
// goto;
    }
    return ret;
    }
    -EBUSY => {
    }
    -EAGAIN => {
//
// Two reasons for this:
// - EBUSY: Owner is exiting and we just wait for the
// exit to complete.
// - EAGAIN: The user space value changed.
//
    futex_hb_waiters_dec(hb2);
    double_unlock_hb(hb1, hb2);
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
    plist_for_each_entry_safe(this, next, &hb1.chain, list) {
    if (task_count - nr_wake >= nr_requeue) {
    break;
    }
    if (!futex_match(&this.key, &key1)) {
    continue;
    }
//
// FUTEX_WAIT_REQUEUE_PI and FUTEX_CMP_REQUEUE_PI should always
// be paired with each other and no other futex ops.
//
// We should never be requeueing a futex_q with a pi_state,
// which is awaiting a futex_unlock_pi().
//
    if ((requeue_pi && !this.rt_waiter) ||
    (!requeue_pi && this.rt_waiter) ||
    this.pi_state) {
    ret = -EINVAL;
    break;
    }
// Plain futexes just wake or requeue and are done
    if (!requeue_pi) {
    if (++task_count <= nr_wake) {
    this.wake(&wake_q, this);
    }
    else {
    requeue_futex(this, hb1, hb2, &key2);
    }
    continue;
    }
// Ensure we requeue to the expected futex for requeue_pi.
    if (!futex_match(this.requeue_pi_key, &key2)) {
    ret = -EINVAL;
    break;
    }
//
// Requeue nr_requeue waiters and possibly one more in the case
// of requeue_pi if we couldn't acquire the lock atomically.
//
// Prepare the waiter to take the rt_mutex. Take a refcount
// on the pi_state and store the pointer in the futex_q
// object of the waiter.
//
    get_pi_state(pi_state);
// Don't requeue when the waiter is already on the way out.
    if (!futex_requeue_pi_prepare(this, pi_state)) {
//
// Early woken waiter signaled that it is on the
// way out. Drop the pi_state reference and try the
// next waiter. @this->pi_state is still NULL.
//
    put_pi_state(pi_state);
    continue;
    }
    ret = rt_mutex_start_proxy_lock(&pi_state.pi_mutex,
    this.rt_waiter,
    this.task);
    if (ret == 1) {
//
// We got the lock. We do neither drop the refcount
// on pi_state nor clear this->pi_state because the
// waiter needs the pi_state for cleaning up the
// user space value. It will drop the refcount
// after doing so. this::requeue_state is updated
// in the wakeup as well.
//
    requeue_pi_wake_futex(this, &key2, hb2);
    task_count += 1;
    } else if (!ret) {
// Waiter is queued, move it to hb2
    requeue_futex(this, hb1, hb2, &key2);
    futex_requeue_pi_complete(this, 0);
    task_count += 1;
    } else {
//
// rt_mutex_start_proxy_lock() detected a potential
// deadlock when we tried to queue that waiter.
// Drop the pi_state reference which we took above
// and remove the pointer to the state from the
// waiters futex_q object.
//
    this.pi_state = core::ptr::null_mut();
    put_pi_state(pi_state);
    futex_requeue_pi_complete(this, ret);
//
// We stop queueing more waiters and let user space
// deal with the mess.
//
    break;
    }
    }
//
// We took an extra initial reference to the pi_state in
// futex_proxy_trylock_atomic(). We need to drop it here again.
//
    put_pi_state(pi_state);
// label;
    futex_hb_waiters_dec(hb2);
    double_unlock_hb(hb1, hb2);
    }
    wake_up_q(&wake_q);
    return ret ? ret : task_count;
    }
//
// handle_early_requeue_pi_wakeup() - Handle early wakeup on the initial futex
// @hb:		the hash_bucket futex_q was original enqueued on
// @q:		the futex_q woken while waiting to be requeued
// @timeout:	the timeout associated with the wait (NULL if none)
//
// Determine the cause for the early wakeup.
//
// Return:
// -EWOULDBLOCK or -ETIMEDOUT or -ERESTARTNOINTR
//
#[no_mangle]
pub unsafe extern "C" fn handle_early_requeue_pi_wakeup(hb: *mut futex_hash_bucket, q: *mut futex_q, timeout: *mut hrtimer_sleeper) -> c_int {
    let mut ret = 0;
//
// With the hb lock held, we avoid races while we process the wakeup.
// We only need to hold hb (and not hb2) to ensure atomicity as the
// wakeup code can't change q.key from uaddr to uaddr2 if we hold hb.
// It can't be requeued from uaddr2 to something else since we don't
// support a PI aware source futex for requeue.
//
    WARN_ON_ONCE!(&hb.lock != q.lock_ptr);
//
// We were woken prior to requeue by a timeout or a signal.
// Conditionally unqueue the futex_q and determine which it was.
//
    if (!plist_node_empty(&q.list)) {
    plist_del(&q.list, &hb.chain);
    futex_hb_waiters_dec(hb);
    }
// Handle spurious wakeups gracefully
    ret = -EWOULDBLOCK;
    if (timeout && !timeout.task) {
    ret = -ETIMEDOUT;
    }

    else if (signal_pending(current)) {
    ret = -ERESTARTNOINTR;
    }
    return ret;
    }
//
// futex_wait_requeue_pi() - Wait on uaddr and take uaddr2
// @uaddr:	the futex we initially wait on (non-pi)
// @flags:	futex flags (FLAGS_SHARED, FLAGS_CLOCKRT, etc.), they must be
// the same type, no requeueing from private to shared, etc.
// @val:	the expected value of uaddr
// @abs_time:	absolute timeout
// @bitset:	32 bit wakeup bitset set by userspace, defaults to all
// @uaddr2:	the pi futex we will take prior to returning to user-space
//
// The caller will wait on uaddr and will be requeued by futex_requeue() to
// uaddr2 which must be PI aware and unique from uaddr.  Normal wakeup will wake
// on uaddr2 and complete the acquisition of the rt_mutex prior to returning to
// userspace.  This ensures the rt_mutex maintains an owner when it has waiters;
// without one, the pi logic would not know which task to boost/deboost, if
// there was a need to.
//
// We call schedule in futex_wait_queue() when we enqueue and return there
// via the following--
// 1) wakeup on uaddr2 after an atomic lock acquisition by futex_requeue()
// 2) wakeup on uaddr2 after a requeue
// 3) signal
// 4) timeout
//
// If 3, cleanup and return -ERESTARTNOINTR.
//
// If 2, we may then block on trying to take the rt_mutex and return via:
// 5) successful lock
// 6) signal
// 7) timeout
// 8) other lock acquisition failure
//
// If 6, return -EWOULDBLOCK (restarting the syscall would do the same).
//
// If 4 or 7, we cleanup and return with -ETIMEDOUT.
//
// Return:
// -  0 - On success;
// - <0 - On error
//
#[no_mangle]
pub unsafe extern "C" fn futex_wait_requeue_pi(uaddr: *mut u32, flags: c_uint, val: u32, abs_time: *mut ktime_t, bitset: u32, uaddr2: *mut u32) -> c_int {
    struct hrtimer_sleeper timeout, *to;
pub static mut rt_waiter: usize = 0;
pub static mut key2: union futex_key = 0;
pub static mut q: futex_q = 0;
pub static mut pi_mutex: *mut c_void = core::ptr::null_mut();
    let mut res = 0;
    let mut ret = 0;
    if (!IS_ENABLED!(CONFIG_FUTEX_PI)) {
    return -ENOSYS;
    }
    if (uaddr == uaddr2) {
    return -EINVAL;
    }
    if (!bitset) {
    return -EINVAL;
    }
    to = futex_setup_timer(abs_time, &timeout, flags,
    current.timer_slack_ns);
//
// The waiter is allocated on our stack, manipulated by the requeue
// code while we sleep on uaddr.
//
    rt_mutex_init_waiter(&rt_waiter);
    ret = get_futex_key(uaddr2, flags, &key2, FUTEX_WRITE);
    if (unlikely(ret != 0)) {
// goto;
    }
    q.bitset = bitset;
    q.rt_waiter = &rt_waiter;
    q.requeue_pi_key = &key2;
//
// Prepare to wait on uaddr. On success, it holds hb->lock and q
// is initialized.
//
    ret = futex_wait_setup(uaddr, val, flags, &q, &key2, current);
    if (ret) {
// goto;
    }
// Queue the futex_q, drop the hb lock, wait for wakeup.
    futex_do_wait(&q, to);
    switch (futex_requeue_pi_wakeup_sync(&q)) {
    case Q_REQUEUE_PI_IGNORE:
    {
    CLASS(hbr, hbr)(&q.key);
pub static mut hb: auto = 0;
// The waiter is still on uaddr1
    spin_lock(&hb.lock);
    ret = handle_early_requeue_pi_wakeup(hb, &q, to);
    spin_unlock(&hb.lock);
    }
    break;
    case Q_REQUEUE_PI_LOCKED:
// The requeue acquired the lock
    if (q.pi_state && (q.pi_state.owner != current)) {
    futex_q_lockptr_lock(&q);
    ret = fixup_pi_owner(uaddr2, &q, true);
//
// Drop the reference to the pi state which the
// requeue_pi() code acquired for us.
//
    put_pi_state(q.pi_state);
    spin_unlock(q.lock_ptr);
//
// Adjust the return value. It's either -EFAULT or
// success (1) but the caller expects 0 for success.
//
    ret = ret < 0 ? ret : 0;
    }
    break;
    case Q_REQUEUE_PI_DONE:
// Requeue completed. Current is 'pi_blocked_on' the rtmutex
    pi_mutex = &q.pi_state.pi_mutex;
    ret = rt_mutex_wait_proxy_lock(pi_mutex, to, &rt_waiter);
//
// See futex_unlock_pi()'s cleanup: comment.
//
    if (ret && !rt_mutex_cleanup_proxy_lock(pi_mutex, &rt_waiter)) {
    ret = 0;
    }
    futex_q_lockptr_lock(&q);
    debug_rt_mutex_free_waiter(&rt_waiter);
//
// Fixup the pi_state owner and possibly acquire the lock if we
// haven't already.
//
    res = fixup_pi_owner(uaddr2, &q, !ret);
//
// If fixup_pi_owner() returned an error, propagate that.  If it
// acquired the lock, clear -ETIMEDOUT or -EINTR.
//
    if (res) {
    ret = (res < 0) ? res : 0;
    }
    futex_unqueue_pi(&q);
    spin_unlock(q.lock_ptr);
    if (ret == -EINTR) {
//
// We've already been requeued, but cannot restart
// by calling futex_lock_pi() directly. We could
// restart this syscall, but it would detect that
// the user space "val" changed and return
// -EWOULDBLOCK.  Save the overhead of the restart
// and return -EWOULDBLOCK directly.
//
    ret = -EWOULDBLOCK;
    }
    break;
// label;
    BUG();
    }
// Additional reference from requeue_pi_wake_futex()
    futex_private_hash_put(q.drop_fph);
// label;
    if (to) {
    hrtimer_cancel(&to.timer);
    destroy_hrtimer_on_stack(&to.timer);
    }
    return ret;
    }