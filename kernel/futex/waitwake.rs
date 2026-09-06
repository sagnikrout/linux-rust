//! Automatically rewritten from C to Rust
//! Source: kernel/futex/waitwake.c
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
// READ this before attempting to hack on futexes!
//
// Basic futex operation and ordering guarantees
// =============================================
//
// The waiter reads the futex value in user space and calls
// futex_wait(). This function computes the hash bucket and acquires
// the hash bucket lock. After that it reads the futex user space value
// again and verifies that the data has not changed. If it has not changed
// it enqueues itself into the hash bucket, releases the hash bucket lock
// and schedules.
//
// The waker side modifies the user space value of the futex and calls
// futex_wake(). This function computes the hash bucket and acquires the
// hash bucket lock. Then it looks for waiters on that futex in the hash
// bucket and wakes them.
//
// In futex wake up scenarios where no tasks are blocked on a futex, taking
// the hb spinlock can be avoided and simply return. In order for this
// optimization to work, ordering guarantees must exist so that the waiter
// being added to the list is acknowledged when the list is concurrently being
// checked by the waker, avoiding scenarios like the following:
//
// CPU 0                               CPU 1
// val = *futex;
// sys_futex(WAIT, futex, val);
// futex_wait(futex, val);
// uval = *futex;
// *futex = newval;
// sys_futex(WAKE, futex);
// futex_wake(futex);
// if (queue_empty())
// return;
// if (uval == val)
// lock(hash_bucket(futex));
// queue();
// unlock(hash_bucket(futex));
// schedule();
//
// This would cause the waiter on CPU 0 to wait forever because it
// missed the transition of the user space value from val to newval
// and the waker did not find the waiter in the hash bucket queue.
//
// The correct serialization ensures that a waiter either observes
// the changed user space value before blocking or is woken by a
// concurrent waker:
//
// CPU 0                                 CPU 1
// val = *futex;
// sys_futex(WAIT, futex, val);
// futex_wait(futex, val);
//
// waiters += 1; (a)
// smp_mb(); (A) <-- paired with -.
// |
// lock(hash_bucket(futex));      |
// |
// uval = *futex;                 |
// |        *futex = newval;
// |        sys_futex(WAKE, futex);
// |          futex_wake(futex);
// |
// `--------> smp_mb(); (B)
// if (uval == val)
// queue();
// unlock(hash_bucket(futex));
// schedule();                         if (waiters)
// lock(hash_bucket(futex));
// else                                    wake_waiters(futex);
// waiters -= 1; (b)                        unlock(hash_bucket(futex));
//
// Where (A) orders the waiters increment and the futex value read through
// atomic operations (see futex_hb_waiters_inc) and where (B) orders the write
// to futex and the waiters read (see futex_hb_waiters_pending()).
//
// This yields the following case (where X:=waiters, Y:=futex):
//
// X = Y = 0
//
// w[X]=1		w[Y]=1
// MB		MB
// r[Y]=y		r[X]=x
//
// Which guarantees that x==0 && y==0 is impossible; which translates back into
// the guarantee that we cannot both miss the futex variable change and the
// enqueue.
//
// Note that a new waiter is accounted for in (a) even when it is possible that
// the wait call can return error, in which case we backtrack from it in (b).
// Refer to the comment in futex_q_lock().
//
// Similarly, in order to account for waiters being requeued on another
// address we always increment the waiters for the destination bucket before
// acquiring the lock. It then decrements them again  after releasing it -
// the code that actually moves the futex(es) between hash buckets (requeue_futex)
// will do the additional required waiter count housekeeping. This is done for
// double_lock_hb() and double_unlock_hb(), respectively.
//
#[no_mangle]
pub unsafe extern "C" fn __futex_wake_mark(q: *mut futex_q) -> bool {
    if (WARN(q.pi_state || q.rt_waiter, "refusing to wake PI futex\n")) {
    return false;
    }
    __futex_unqueue(q);
//
// The waiting task can free the futex_q as soon as q->lock_ptr = NULL
// is written, without taking any locks. This is possible in the event
// of a spurious wakeup, for example. A memory barrier is required here
// to prevent the following store to lock_ptr from getting ahead of the
// plist_del in __futex_unqueue().
//
    smp_store_release(&q.lock_ptr, core::ptr::null_mut());
    return true;
    }
//
// The hash bucket lock must be held when this is called.
// Afterwards, the futex_q must not be accessed. Callers
// must ensure to later call wake_up_q() for the actual
// wakeups to occur.
//
#[no_mangle]
pub unsafe extern "C" fn futex_wake_mark(wake_q: *mut wake_q_head, q: *mut futex_q) {
    let mut p = q.task;
    get_task_struct(p);
    if (!__futex_wake_mark(q)) {
    put_task_struct(p);
    return;
    }
//
// Queue the task for later wakeup for after we've released
// the hb->lock.
//
    wake_q_add_safe(wake_q, p);
    }
//
// If requested, clear the robust list pending op and unlock the futex
//
#[no_mangle]
unsafe extern "C" fn futex_robust_unlock(uaddr: *mut u32 , flags: c_uint, pop: *mut c_void ) -> bool {
    if (!(flags & FLAGS_ROBUST_UNLOCK)) {
    return true;
    }
// First unlock the futex, which requires release semantics.
    scoped_user_write_access(uaddr, efault)
    unsafe_atomic_store_release_user(0, uaddr, efault);
//
// Clear the pending list op now. If that fails, then the task is in
// deeper trouble as the robust list head is usually part of the TLS.
// The chance of survival is close to zero.
//
    return futex_robust_list_clear_pending(pop, flags);
// label;
    return false;
    }
//
// Wake up waiters matching bitset queued on this futex (uaddr).
//
#[no_mangle]
pub unsafe extern "C" fn futex_wake(uaddr: *mut u32 , flags: c_uint, pop: *mut c_void , nr_wake: c_int, bitset: u32) -> c_int {
pub static mut key: union futex_key = 0;
    let mut this = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut wake_q: usize = 0;
    let mut ret = 0;
    if (!bitset) {
    return -EINVAL;
    }
    ret = get_futex_key(uaddr, flags, &key, FUTEX_READ);
    if (unlikely(ret != 0)) {
    return ret;
    }
    if (!futex_robust_unlock(uaddr, flags, pop)) {
    return -EFAULT;
    }
    if ((flags & FLAGS_STRICT) && !nr_wake) {
    return 0;
    }
    CLASS(hbr, hbr)(&key);
pub static mut hb: auto = 0;
// Make sure we really have tasks to wakeup
    if (!futex_hb_waiters_pending(hb)) {
    return ret;
    }
    spin_lock(&hb.lock);
    plist_for_each_entry_safe(this, next, &hb.chain, list) {
    if (futex_match (&this.key, &key)) {
    if (this.pi_state || this.rt_waiter) {
    ret = -EINVAL;
    break;
    }
// Check if one of the bits is set in both bitsets
    if (!(this.bitset & bitset)) {
    continue;
    }
    this.wake(&wake_q, this);
    if (++ret >= nr_wake) {
    break;
    }
    }
    }
    spin_unlock(&hb.lock);
    wake_up_q(&wake_q);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn futex_atomic_op_inuser(encoded_op: c_uint, uaddr: *mut u32 ) -> c_int {
pub static mut op: c_uint = 0;
pub static mut cmp: c_uint = 0;
pub static mut oparg: c_int = 0;
pub static mut cmparg: c_int = 0;
    let mut oldval = 0;
    let mut ret = 0;
    if (encoded_op & (FUTEX_OP_OPARG_SHIFT << 28)) {
    if (oparg < 0 || oparg > 31) {
//
// kill this print and return -EINVAL when userspace
// is sane again
//
    pr_info_ratelimited("futex_wake_op: %s tries to shift op by %d; fix this program\n",
    current.comm, oparg);
    oparg &= 31;
    }
    oparg = 1 << oparg;
    }
    pagefault_disable();
    ret = arch_futex_atomic_op_inuser(op, oparg, &oldval, uaddr);
    pagefault_enable();
    if (ret) {
    return ret;
    }
    match (cmp) {
    FUTEX_OP_CMP_EQ => {
pub static mut oldval: return = 0;
    }
    FUTEX_OP_CMP_NE => {
    return oldval != cmparg;
    }
    FUTEX_OP_CMP_LT => {
    return oldval < cmparg;
    }
    FUTEX_OP_CMP_GE => {
    return oldval >= cmparg;
    }
    FUTEX_OP_CMP_LE => {
    return oldval <= cmparg;
    }
    FUTEX_OP_CMP_GT => {
    return oldval > cmparg;
    }
    _ => {
    return -ENOSYS;
    }
    }
    }
//
// Wake up all waiters hashed on the physical page that is mapped
// to this virtual address:
//
#[no_mangle]
pub unsafe extern "C" fn futex_wake_op(uaddr1: *mut u32, flags: c_uint, uaddr2: *mut u32, nr_wake: c_int, nr_wake2: c_int, op: c_int) -> c_int {
pub static mut key1: union futex_key = 0;
    let mut this = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut ret = 0;
    let mut op_ret = 0;
pub static mut wake_q: usize = 0;
// label;
    ret = get_futex_key(uaddr1, flags, &key1, FUTEX_READ);
    if (unlikely(ret != 0)) {
    return ret;
    }
    ret = get_futex_key(uaddr2, flags, &key2, FUTEX_WRITE);
    if (unlikely(ret != 0)) {
    return ret;
    }
// label;
    if (1) {
    CLASS(hbr, hbr1)(&key1);
    CLASS(hbr, hbr2)(&key2);
pub static mut hb1: auto = 0;
pub static mut hb2: auto = 0;
    double_lock_hb(hb1, hb2);
    op_ret = futex_atomic_op_inuser(op, uaddr2);
    if (unlikely(op_ret < 0)) {
    double_unlock_hb(hb1, hb2);
    if (!IS_ENABLED!(CONFIG_MMU) ||
    unlikely(op_ret != -EFAULT && op_ret != -EAGAIN)) {
//
// we don't get EFAULT from MMU faults if we don't have
// an MMU, but we might get them from range checking
//
    ret = op_ret;
    return ret;
    }
    if (op_ret == -EFAULT) {
    ret = fault_in_user_writeable(uaddr2);
    if (ret) {
    return ret;
    }
    }
    cond_resched();
    if (!(flags & FLAGS_SHARED)) {
// goto;
    }
// goto;
    }
    plist_for_each_entry_safe(this, next, &hb1.chain, list) {
    if (futex_match(&this.key, &key1)) {
    if (this.pi_state || this.rt_waiter) {
    ret = -EINVAL;
// goto;
    }
    this.wake(&wake_q, this);
    if (++ret >= nr_wake) {
    break;
    }
    }
    }
    if (op_ret > 0) {
    op_ret = 0;
    plist_for_each_entry_safe(this, next, &hb2.chain, list) {
    if (futex_match(&this.key, &key2)) {
    if (this.pi_state || this.rt_waiter) {
    ret = -EINVAL;
// goto;
    }
    this.wake(&wake_q, this);
    if (++op_ret >= nr_wake2) {
    break;
    }
    }
    }
    ret += op_ret;
    }
// label;
    double_unlock_hb(hb1, hb2);
    }
    wake_up_q(&wake_q);
    return ret;
    }
// forward_decl: futex_wait_restart;
//
// futex_do_wait() - wait for wakeup, timeout, or signal
// @q:		the futex_q to queue up on
// @timeout:	the prepared hrtimer_sleeper, or null for no timeout
//
#[no_mangle]
pub unsafe extern "C" fn futex_do_wait(q: *mut futex_q, timeout: *mut hrtimer_sleeper) {
// Arm the timer
    if (timeout) {
    hrtimer_sleeper_start_expires(timeout, HRTIMER_MODE_ABS);
    }
//
// If we have been removed from the hash list, then another task
// has tried to wake us, and we can skip the call to schedule().
//
    if (likely(!plist_node_empty(&q.list))) {
//
// If the timer has already expired, current will already be
// flagged for rescheduling. Only call schedule if there
// is no timeout, or if it has yet to expire.
//
    if (!timeout || timeout.task) {
    schedule();
    }
    }
    __set_current_state(TASK_RUNNING);
    }
//
// futex_unqueue_multiple - Remove various futexes from their hash bucket
// @v:	   The list of futexes to unqueue
// @count: Number of futexes in the list
//
// Helper to unqueue a list of futexes. This can't fail.
//
// Return:
// - >=0 - Index of the last futex that was awoken;
// - -1  - No futex was awoken
//
#[no_mangle]
pub unsafe extern "C" fn futex_unqueue_multiple(v: *mut futex_vector, count: c_int) -> c_int {
pub static mut ret: c_int = 0;
    while (i < count) {
    if (!futex_unqueue(&v[i].q)) {
    ret = i;
    }
    }
    return ret;
    }
//
// futex_wait_multiple_setup - Prepare to wait and enqueue multiple futexes
// @vs:		The futex list to wait on
// @count:	The size of the list
// @woken:	Index of the last woken futex, if any. Used to notify the
// caller that it can return this index to userspace (return parameter)
//
// Prepare multiple futexes in a single step and enqueue them. This may fail if
// the futex list is invalid or if any futex was already awoken. On success the
// task is ready to interruptible sleep.
//
// Return:
// -  1 - One of the futexes was woken by another thread
// -  0 - Success
// - <0 - -EFAULT, -EWOULDBLOCK or -EINVAL
//
#[no_mangle]
pub unsafe extern "C" fn futex_wait_multiple_setup(vs: *mut futex_vector, count: c_int, woken: *mut c_int) -> c_int {
pub static mut retry: bool = false;
    let mut ret = 0;
    let mut i = 0;
    let mut uval = 0;
//
// Make sure to have a reference on the private_hash such that we
// don't block on rehash after changing the task state below.
//
    guard(private_hash)(current.mm);
//
// Enqueuing multiple futexes is tricky, because we need to enqueue
// each futex on the list before dealing with the next one to avoid
// deadlocking on the hash bucket. But, before enqueuing, we need to
// make sure that current->state is TASK_INTERRUPTIBLE, so we don't
// lose any wake events, which cannot be done before the get_futex_key
// of the next key, because it calls get_user_pages, which can sleep.
// Thus, we fetch the list of futexes keys in two steps, by first
// pinning all the memory keys in the futex key, and only then we read
// each key and queue the corresponding futex.
//
// Private futexes doesn't need to recalculate hash in retry, so skip
// get_futex_key() when retrying.
//
// label;
    while (i < count) {
    if (!(vs[i].w.flags & FLAGS_SHARED) && retry) {
    continue;
    }
    ret = get_futex_key(u64_to_user_ptr(vs[i].w.uaddr),
    vs[i].w.flags,
    &vs[i].q.key, FUTEX_READ);
    if (unlikely(ret)) {
    return ret;
    }
    }
    set_current_state(TASK_INTERRUPTIBLE|TASK_FREEZABLE);
    while (i < count) {
    let mut uaddr = (unsigned long)vs[i].w.uaddr;
    let mut q = &vs[i].q;
pub static mut val: u32 = 0;
    if (1) {
    CLASS(hbr, hbr)(&q.key);
pub static mut hb: auto = 0;
    futex_q_lock(q, hb);
    ret = futex_get_value_locked(&uval, uaddr);
    if (!ret && uval == val) {
//
// The bucket lock can't be held while dealing with the
// next futex. Queue each futex at this moment so hb can
// be unlocked.
//
    futex_queue(q, hb, current);
    continue;
    }
    futex_q_unlock(hb);
    __release(q.lock_ptr);
    }
    __set_current_state(TASK_RUNNING);
//
// Even if something went wrong, if we find out that a futex
// was woken, we don't return error and return this index to
// userspace
//
// woken = futex_unqueue_multiple(vs, i);
    if (*woken >= 0) {
    return 1;
    }
    if (ret) {
//
// If we need to handle a page fault, we need to do so
// without any lock and any enqueued futex (otherwise
// we could lose some wakeup). So we do it here, after
// undoing all the work done so far. In success, we
// retry all the work.
//
    if (get_user(uval, uaddr)) {
    return -EFAULT;
    }
    retry = true;
// goto;
    }
    if (uval != val) {
    return -EWOULDBLOCK;
    }
    }
    return 0;
    }
//
// futex_sleep_multiple - Check sleeping conditions and sleep
// @vs:    List of futexes to wait for
// @count: Length of vs
// @to:    Timeout
//
// Sleep if and only if the timeout hasn't expired and no futex on the list has
// been woken up.
//
#[no_mangle]
pub unsafe extern "C" fn futex_sleep_multiple(vs: *mut futex_vector, count: c_uint, to: *mut hrtimer_sleeper) {
    if (to && !to.task) {
    return;
    }
    while (count) {
    if (!READ_ONCE(vs.q.lock_ptr)) {
    return;
    }
    }
    schedule();
    }
//
// futex_wait_multiple - Prepare to wait on and enqueue several futexes
// @vs:		The list of futexes to wait on
// @count:	The number of objects
// @to:		Timeout before giving up and returning to userspace
//
// Entry point for the FUTEX_WAIT_MULTIPLE futex operation, this function
// sleeps on a group of futexes and returns on the first futex that is
// wake, or after the timeout has elapsed.
//
// Return:
// - >=0 - Hint to the futex that was awoken
// - <0  - On error
//
#[no_mangle]
pub unsafe extern "C" fn futex_wait_multiple(vs: *mut futex_vector, count: c_uint, to: *mut hrtimer_sleeper) -> c_int {
    int ret, hint = 0;
    if (to) {
    hrtimer_sleeper_start_expires(to, HRTIMER_MODE_ABS);
    }
    while (1) {
    ret = futex_wait_multiple_setup(vs, count, &hint);
    if (ret) {
    if (ret > 0) {
// A futex was woken during setup
    ret = hint;
    }
    return ret;
    }
    futex_sleep_multiple(vs, count, to);
    __set_current_state(TASK_RUNNING);
    ret = futex_unqueue_multiple(vs, count);
    if (ret >= 0) {
    return ret;
    }
    if (to && !to.task) {
    return -ETIMEDOUT;
    }

    else if (signal_pending(current)) {
    return -ERESTARTSYS;
    }
//
// The final case is a spurious wakeup, for
// which just retry.
//
    }
    }
//
// futex_wait_setup() - Prepare to wait on a futex
// @uaddr:	the futex userspace address
// @val:	the expected value
// @flags:	futex flags (FLAGS_SHARED, etc.)
// @q:		the associated futex_q
// @key2:	the second futex_key if used for requeue PI
// @task:	Task queueing this futex
//
// Setup the futex_q and locate the hash_bucket.  Get the futex value and
// compare it with the expected value.  Handle atomic faults internally.
// Return with the hb lock held on success, and unlocked on failure.
//
// Return:
// -  0 - uaddr contains val and hb has been locked;
// - <0 - On error and the hb is unlocked. A possible reason: the uaddr can not
// be read, does not contain the expected value or is not properly aligned.
//
#[no_mangle]
pub unsafe extern "C" fn futex_wait_setup(uaddr: *mut u32, val: u32, flags: c_uint, q: *mut futex_q, key2: *mut union futex_key, task: *mut task_struct) -> c_int {
    let mut uval = 0;
    let mut ret = 0;
//
// Access the page AFTER the hash-bucket is locked.
// Order is important:
//
// Userspace waiter: val = var; if (cond(val)) futex_wait(&var, val);
// Userspace waker:  if (cond(var)) { var = new; futex_wake(&var); }
//
// The basic logical guarantee of a futex is that it blocks ONLY
// if cond(var) is known to be true at the time of blocking, for
// any cond.  If we locked the hash-bucket after testing *uaddr, that
// would open a race condition where we could block indefinitely with
// cond(var) false, which would violate the guarantee.
//
// On the other hand, we insert q and release the hash-bucket only
// after testing *uaddr.  This guarantees that futex_wait() will NOT
// absorb a wakeup if *uaddr does not match the desired values
// while the syscall executes.
//
// label;
    ret = get_futex_key(uaddr, flags, &q.key, FUTEX_READ);
    if (unlikely(ret != 0)) {
    return ret;
    }
// label;
    if (1) {
    CLASS(hbr, hbr)(&q.key);
pub static mut hb: auto = 0;
    futex_q_lock(q, hb);
    ret = futex_get_value_locked(&uval, uaddr);
    if (ret) {
    futex_q_unlock(hb);
    __release(q.lock_ptr);
    ret = get_user(uval, uaddr);
    if (ret) {
    return ret;
    }
    if (!(flags & FLAGS_SHARED)) {
// goto;
    }
// goto;
    }
    if (uval != val) {
    futex_q_unlock(hb);
    __release(q.lock_ptr);
    return -EWOULDBLOCK;
    }
    if (key2 && futex_match(&q.key, key2)) {
    futex_q_unlock(hb);
    __release(q.lock_ptr);
    return -EINVAL;
    }
//
// The task state is guaranteed to be set before another task can
// wake it. set_current_state() is implemented using smp_store_mb() and
// futex_queue() calls spin_unlock() upon completion, both serializing
// access to the hash list and forcing another memory barrier.
//
    if (task == current) {
    set_current_state(TASK_INTERRUPTIBLE|TASK_FREEZABLE);
    }
    futex_queue(q, hb, task);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __futex_wait(uaddr: *mut u32, flags: c_uint, val: u32, to: *mut hrtimer_sleeper, bitset: u32) -> c_int {
pub static mut q: futex_q = 0;
    let mut ret = 0;
    if (!bitset) {
    return -EINVAL;
    }
    q.bitset = bitset;
// label;
//
// Prepare to wait on uaddr. On success, it holds hb->lock and q
// is initialized.
//
    ret = futex_wait_setup(uaddr, val, flags, &q, core::ptr::null_mut(), current);
    if (ret) {
    return ret;
    }
// futex_queue and wait for wakeup, timeout, or a signal.
    futex_do_wait(&q, to);
// If we were woken (and unqueued), we succeeded, whatever.
    if (!futex_unqueue(&q)) {
    return 0;
    }
    if (to && !to.task) {
    return -ETIMEDOUT;
    }
//
// We expect signal_pending(current), but we might be the
// victim of a spurious wakeup as well.
//
    if (!signal_pending(current)) {
// goto;
    }
    return -ERESTARTSYS;
    }
#[no_mangle]
pub unsafe extern "C" fn futex_wait(uaddr: *mut u32 , flags: c_uint, val: u32, abs_time: *mut ktime_t, bitset: u32) -> c_int {
    struct hrtimer_sleeper timeout, *to;
pub static mut restart: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    to = futex_setup_timer(abs_time, &timeout, flags,
    current.timer_slack_ns);
    ret = __futex_wait(uaddr, flags, val, to, bitset);
// No timeout, nothing to clean up.
    if (!to) {
    return ret;
    }
    hrtimer_cancel(&to.timer);
    destroy_hrtimer_on_stack(&to.timer);
    if (ret == -ERESTARTSYS) {
    restart = &current.restart_block;
    restart.futex.uaddr = uaddr;
    restart.futex.val = val;
    restart.futex.time = *abs_time;
    restart.futex.bitset = bitset;
    restart.futex.flags = flags | FLAGS_HAS_TIMEOUT;
    return set_restart_fn(restart, futex_wait_restart);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn futex_wait_restart(restart: *mut restart_block) -> c_long {
    let mut uaddr = restart.futex.uaddr;
    let mut tp = core::ptr::null_mut();
    if (restart.futex.flags & FLAGS_HAS_TIMEOUT) {
    tp = &restart.futex.time;
    }
    restart.fn = do_no_restart_syscall;
    return (long)futex_wait(uaddr, restart.futex.flags,
    restart.futex.val, tp, restart.futex.bitset);
    }