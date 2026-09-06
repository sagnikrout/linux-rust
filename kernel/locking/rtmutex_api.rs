//! Automatically rewritten from C to Rust
//! Source: kernel/locking/rtmutex_api.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// rtmutex API
//

// Macro flag: #define RT_MUTEX_BUILD_MUTEX

//
// Max number of times we'll walk the boosting chain:
//
pub static mut max_lock_depth: c_int = 1024;
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_rtmutex_sysctl() -> c_int {
    register_sysctl_init("kernel", rtmutex_sysctl_table);
    return 0;
    }
    subsys_initcall!(init_rtmutex_sysctl);
//
// Debug aware fast / slowpath lock,trylock,unlock
//
// The atomic acquire/release ops are compiled away, when either the
// architecture does not support cmpxchg or when debugging is enabled.
//
    static __always_inline int __rt_mutex_lock_common(rt_mutex *lock,
    unsigned int state, lockdep_map *nest_lock,
    unsigned int subclass)
    __cond_acquires(0, lock)
    {
    let mut ret = 0;
    might_sleep();
    mutex_acquire_nest(&lock.dep_map, subclass, 0, nest_lock, _RET_IP_);
    ret = __rt_mutex_lock(&lock.rtmutex, state);
    if (ret) {
    mutex_release(&lock.dep_map, _RET_IP_);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_base_init(rtb: *mut rt_mutex_base) {
    __rt_mutex_base_init(rtb);
    }
    EXPORT_SYMBOL(rt_mutex_base_init);

//
// rt_mutex_lock_nested - lock a rt_mutex
//
// @lock: the rt_mutex to be locked
// @subclass: the lockdep subclass
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_lock_nested(lock: *mut rt_mutex, subclass: c_uint) -> void __sched {
    if (__rt_mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, core::ptr::null_mut(), subclass) == 0) {
    return;
    }
//
// The code below is never reached because __rt_mutex_lock_common() only
// returns an error code if interrupted by a signal or upon a timeout.
//
    WARN_ON_ONCE!(true);
    __acquire(lock);
    }
    EXPORT_SYMBOL_GPL(rt_mutex_lock_nested);
#[no_mangle]
pub unsafe extern "C" fn _rt_mutex_lock_nest_lock(lock: *mut rt_mutex, nest_lock: *mut lockdep_map) -> void __sched {
    if (__rt_mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, nest_lock, 0) == 0) {
    return;
    }
//
// The code below is never reached because __rt_mutex_lock_common() only
// returns an error code if interrupted by a signal or upon a timeout.
//
    WARN_ON_ONCE!(true);
    __acquire(lock);
    }
    EXPORT_SYMBOL_GPL(_rt_mutex_lock_nest_lock);

//
// rt_mutex_lock - lock a rt_mutex
//
// @lock: the rt_mutex to be locked
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_lock(lock: *mut rt_mutex) -> void __sched {
    if (__rt_mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, core::ptr::null_mut(), 0) == 0) {
    return;
    }
//
// The code below is never reached because __rt_mutex_lock_common() only
// returns an error code if interrupted by a signal or upon a timeout.
//
    WARN_ON_ONCE!(true);
    __acquire(lock);
    }
    EXPORT_SYMBOL_GPL(rt_mutex_lock);

//
// rt_mutex_lock_interruptible - lock a rt_mutex interruptible
//
// @lock:		the rt_mutex to be locked
//
// Returns:
// 0		on success
// -EINTR	when interrupted by a signal
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_lock_interruptible(lock: *mut rt_mutex) -> int __sched {
    return __rt_mutex_lock_common(lock, TASK_INTERRUPTIBLE, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL_GPL(rt_mutex_lock_interruptible);
//
// rt_mutex_lock_killable - lock a rt_mutex killable
//
// @lock:		the rt_mutex to be locked
//
// Returns:
// 0		on success
// -EINTR	when interrupted by a signal
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_lock_killable(lock: *mut rt_mutex) -> int __sched {
    return __rt_mutex_lock_common(lock, TASK_KILLABLE, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL_GPL(rt_mutex_lock_killable);
//
// rt_mutex_trylock - try to lock a rt_mutex
//
// @lock:	the rt_mutex to be locked
//
// This function can only be called in thread context. It's safe to call it
// from atomic regions, but not from hard or soft interrupt context.
//
// Returns:
// 1 on success
// 0 on contention
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_trylock(lock: *mut rt_mutex) -> int __sched {
    let mut ret = 0;
    if (IS_ENABLED!(CONFIG_DEBUG_RT_MUTEXES) && WARN_ON_ONCE!(!in_task())) {
    return 0;
    }
    ret = __rt_mutex_trylock(&lock.rtmutex);
    if (ret) {
    mutex_acquire(&lock.dep_map, 0, 1, _RET_IP_);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(rt_mutex_trylock);
//
// rt_mutex_unlock - unlock a rt_mutex
//
// @lock: the rt_mutex to be unlocked
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_unlock(lock: *mut rt_mutex) -> void __sched {
    mutex_release(&lock.dep_map, _RET_IP_);
    __rt_mutex_unlock(&lock.rtmutex);
    __release(lock);
    }
    EXPORT_SYMBOL_GPL(rt_mutex_unlock);
//
// Futex variants, must not use fastpath.
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_futex_trylock(lock: *mut rt_mutex_base) -> int __sched {
    return rt_mutex_slowtrylock(lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __rt_mutex_futex_trylock(lock: *mut rt_mutex_base) -> int __sched {
    return __rt_mutex_slowtrylock(lock);
    }
//
// __rt_mutex_futex_unlock - Futex variant, that since futex variants
// do not use the fast-path, can be simple and will not need to retry.
//
// @lock:	The rt_mutex to be unlocked
// @wqh:	The wake queue head from which to get the next lock waiter
//
    bool __sched __rt_mutex_futex_unlock(rt_mutex_base *lock, rt_wake_q_head *wqh)
    __must_hold(&lock.wait_lock)
    {
    lockdep_assert_held(&lock.wait_lock);
    debug_rt_mutex_unlock(lock);
    if (!rt_mutex_has_waiters(lock)) {
    lock.owner = core::ptr::null_mut();
    return false; /* done */
    }
//
// mark_wakeup_next_waiter() deboosts and retains preemption
// disabled when dropping the wait_lock, to avoid inversion prior
// to the wakeup.  preempt_disable() therein pairs with the
// preempt_enable() in rt_mutex_postunlock().
//
    mark_wakeup_next_waiter(wqh, lock);
    return true; /* call postunlock() */
    }
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_futex_unlock(lock: *mut rt_mutex_base) -> void __sched {
pub static mut wqh: usize = 0;
    let mut flags = 0;
    let mut postunlock = 0;
    raw_spin_lock_irqsave(&lock.wait_lock, flags);
    postunlock = __rt_mutex_futex_unlock(lock, &wqh);
    raw_spin_unlock_irqrestore(&lock.wait_lock, flags);
    if (postunlock) {
    rt_mutex_postunlock(&wqh);
    }
    }
//
// __rt_mutex_init - initialize the rt_mutex
//
// @lock:	The rt_mutex to be initialized
// @name:	The lock name used for debugging
// @key:	The lock class key used for debugging
//
// Initialize the rt_mutex to unlocked state.
//
// Initializing of a locked rt_mutex is not allowed
//
    void __sched __rt_mutex_init(rt_mutex *lock, const char *name, lock_class_key *key)
    {
    debug_check_no_locks_freed(lock, sizeof!(*lock));
    __rt_mutex_base_init(&lock.rtmutex);
    lockdep_init_map_wait(&lock.dep_map, name, key, 0, LD_WAIT_SLEEP);
    }
    EXPORT_SYMBOL_GPL(__rt_mutex_init);
//
// rt_mutex_init_proxy_locked - initialize and lock a rt_mutex on behalf of a
// proxy owner
//
// @lock:	the rt_mutex to be locked
// @proxy_owner:the task to set as owner
//
// No locking. Caller has to do serializing itself
//
// Special API call for PI-futex support. This initializes the rtmutex and
// assigns it to @proxy_owner. Concurrent operations on the rtmutex are not
// possible at this point because the pi_state which contains the rtmutex
// is not yet visible to other tasks.
//
    void __sched rt_mutex_init_proxy_locked(rt_mutex_base *lock, task_struct *proxy_owner)
    {
pub static mut pi_futex_key: usize = 0;
    __rt_mutex_base_init(lock);
//
// On PREEMPT_RT the futex hashbucket spinlock becomes 'sleeping'
// and rtmutex based. That causes a lockdep false positive, because
// some of the futex functions invoke spin_unlock(&hb->lock) with
// the wait_lock of the rtmutex associated to the pi_futex held.
// spin_unlock() in turn takes wait_lock of the rtmutex on which
// the spinlock is based, which makes lockdep notice a lock
// recursion. Give the futex/rtmutex wait_lock a separate key.
//
    lockdep_set_class(&lock.wait_lock, &pi_futex_key);
    rt_mutex_set_owner(lock, proxy_owner);
    }
//
// rt_mutex_proxy_unlock - release a lock on behalf of owner
//
// @lock:	the rt_mutex to be locked
//
// No locking. Caller has to do serializing itself
//
// Special API call for PI-futex support. This just cleans up the rtmutex
// (debugging) state. Concurrent operations on this rt_mutex are not
// possible because it belongs to the pi_state which is about to be freed
// and it is not longer visible to other tasks.
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_proxy_unlock(lock: *mut rt_mutex_base) -> void __sched {
    debug_rt_mutex_proxy_unlock(lock);
    rt_mutex_clear_owner(lock);
    }
//
// __rt_mutex_start_proxy_lock() - Start lock acquisition for another task
// @lock:		the rt_mutex to take
// @waiter:		the pre-initialized rt_mutex_waiter
// @task:		the task to prepare
// @wake_q:		the wake_q to wake tasks after we release the wait_lock
//
// Starts the rt_mutex acquire; it enqueues the @waiter and does deadlock
// detection. It does not wait, see rt_mutex_wait_proxy_lock() for that.
//
// NOTE: does _NOT_ remove the @waiter on failure; must either call
// rt_mutex_wait_proxy_lock() or rt_mutex_cleanup_proxy_lock() after this.
//
// Returns:
// 0 - task blocked on lock
// 1 - acquired the lock for task, caller should wake it up
// <0 - error
//
// Special API call for PI-futex support.
//
    int __sched __rt_mutex_start_proxy_lock(rt_mutex_base *lock, rt_mutex_waiter *waiter, task_struct *task, wake_q_head *wake_q)
    __must_hold(&lock.wait_lock)
    {
    let mut ret = 0;
    lockdep_assert_held(&lock.wait_lock);
    if (try_to_take_rt_mutex(lock, task, core::ptr::null_mut())) {
    return 1;
    }
// We enforce deadlock detection for futexes
    ret = task_blocks_on_rt_mutex(lock, waiter, task, core::ptr::null_mut(),
    RT_MUTEX_FULL_CHAINWALK, wake_q);
    if (ret && !rt_mutex_owner(lock)) {
//
// Reset the return value. We might have
// returned with -EDEADLK and the owner
// released the lock while we were walking the
// pi chain.  Let the waiter sort it out.
//
    ret = 0;
    }
    return ret;
    }
//
// rt_mutex_start_proxy_lock() - Start lock acquisition for another task
// @lock:		the rt_mutex to take
// @waiter:		the pre-initialized rt_mutex_waiter
// @task:		the task to prepare
//
// Starts the rt_mutex acquire; it enqueues the @waiter and does deadlock
// detection. It does not wait, see rt_mutex_wait_proxy_lock() for that.
//
// NOTE: unlike __rt_mutex_start_proxy_lock this _DOES_ remove the @waiter
// on failure.
//
// Returns:
// 0 - task blocked on lock
// 1 - acquired the lock for task, caller should wake it up
// <0 - error
//
// Special API call for PI-futex support.
//
    int __sched rt_mutex_start_proxy_lock(rt_mutex_base *lock, rt_mutex_waiter *waiter, task_struct *task)
    {
    let mut ret = 0;
pub static mut wake_q: usize = 0;
    raw_spin_lock_irq(&lock.wait_lock);
    ret = __rt_mutex_start_proxy_lock(lock, waiter, task, &wake_q);
    if (unlikely(ret < 0)) {
    remove_waiter(lock, waiter);
    }
    preempt_disable();
    raw_spin_unlock_irq(&lock.wait_lock);
    wake_up_q(&wake_q);
    preempt_enable();
    return ret;
    }
//
// rt_mutex_wait_proxy_lock() - Wait for lock acquisition
// @lock:		the rt_mutex we were woken on
// @to:			the timeout, null if none. hrtimer should already have
// been started.
// @waiter:		the pre-initialized rt_mutex_waiter
//
// Wait for the lock acquisition started on our behalf by
// rt_mutex_start_proxy_lock(). Upon failure, the caller must call
// rt_mutex_cleanup_proxy_lock().
//
// Returns:
// 0 - success
// <0 - error, one of -EINTR, -ETIMEDOUT
//
// Special API call for PI-futex support
//
    int __sched rt_mutex_wait_proxy_lock(rt_mutex_base *lock, hrtimer_sleeper *to, rt_mutex_waiter *waiter)
    {
    let mut ret = 0;
    raw_spin_lock_irq(&lock.wait_lock);
// sleep on the mutex
    set_current_state(TASK_INTERRUPTIBLE);
    ret = rt_mutex_slowlock_block(lock, core::ptr::null_mut(), TASK_INTERRUPTIBLE, to, waiter, core::ptr::null_mut());
//
// try_to_take_rt_mutex() sets the waiter bit unconditionally. We might
// have to fix that up.
//
    fixup_rt_mutex_waiters(lock, true);
    raw_spin_unlock_irq(&lock.wait_lock);
    return ret;
    }
//
// rt_mutex_cleanup_proxy_lock() - Cleanup failed lock acquisition
// @lock:		the rt_mutex we were woken on
// @waiter:		the pre-initialized rt_mutex_waiter
//
// Attempt to clean up after a failed __rt_mutex_start_proxy_lock() or
// rt_mutex_wait_proxy_lock().
//
// Unless we acquired the lock; we're still enqueued on the wait-list and can
// in fact still be granted ownership until we're removed. Therefore we can
// find we are in fact the owner and must disregard the
// rt_mutex_wait_proxy_lock() failure.
//
// Returns:
// true  - did the cleanup, we done.
// false - we acquired the lock after rt_mutex_wait_proxy_lock() returned,
// caller should disregards its return value.
//
// Special API call for PI-futex support
//
    bool __sched rt_mutex_cleanup_proxy_lock(rt_mutex_base *lock, rt_mutex_waiter *waiter)
    {
pub static mut cleanup: bool = false;
    raw_spin_lock_irq(&lock.wait_lock);
//
// Do an unconditional try-lock, this deals with the lock stealing
// state where __rt_mutex_futex_unlock() -> mark_wakeup_next_waiter()
// sets a NULL owner.
//
// We're not interested in the return value, because the subsequent
// test on rt_mutex_owner() will infer that. If the trylock succeeded,
// we will own the lock and it will have removed the waiter. If we
// failed the trylock, we're still not owner and we need to remove
// ourselves.
//
    try_to_take_rt_mutex(lock, current, waiter);
//
// Unless we're the owner; we're still enqueued on the wait_list.
// So check if we became owner, if not, take us off the wait_list.
//
    if (rt_mutex_owner(lock) != current) {
    remove_waiter(lock, waiter);
    cleanup = true;
    }
//
// try_to_take_rt_mutex() sets the waiter bit unconditionally. We might
// have to fix that up.
//
    fixup_rt_mutex_waiters(lock, false);
    raw_spin_unlock_irq(&lock.wait_lock);
    return cleanup;
    }
//
// Recheck the pi chain, in case we got a priority setting
//
// Called from sched_setscheduler
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_adjust_pi(task: *mut task_struct) -> void __sched {
pub static mut waiter: *mut c_void = core::ptr::null_mut();
pub static mut next_lock: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    raw_spin_lock_irqsave(&task.pi_lock, flags);
    waiter = task.pi_blocked_on;
    if (!waiter || rt_waiter_node_equal(&waiter.tree, task_to_waiter_node(task))) {
    raw_spin_unlock_irqrestore(&task.pi_lock, flags);
    return;
    }
    next_lock = waiter.lock;
    raw_spin_unlock_irqrestore(&task.pi_lock, flags);
// gets dropped in rt_mutex_adjust_prio_chain()!
    get_task_struct(task);
    rt_mutex_adjust_prio_chain(task, RT_MUTEX_MIN_CHAINWALK, core::ptr::null_mut(),
    next_lock, core::ptr::null_mut(), task);
    }
//
// Performs the wakeup of the top-waiter and re-enables preemption.
//
#[no_mangle]
pub unsafe extern "C" fn rt_mutex_postunlock(wqh: *mut rt_wake_q_head) -> void __sched {
    rt_mutex_wake_up_q(wqh);
    }

#[no_mangle]
pub unsafe extern "C" fn rt_mutex_debug_task_free(task: *mut task_struct) {
    DEBUG_LOCKS_WARN_ON(!RB_EMPTY_ROOT(&task.pi_waiters.rb_root));
    DEBUG_LOCKS_WARN_ON(task.pi_blocked_on);
    }

// Mutexes
#[no_mangle]
unsafe extern "C" fn __mutex_rt_init_generic(mutex: *mut mutex) {
    rt_mutex_base_init(&mutex.rtmutex);
    debug_check_no_locks_freed(mutex, sizeof!(*mutex));
    }
    static __always_inline int __mutex_lock_common(mutex *lock,
    unsigned int state,
    unsigned int subclass, lockdep_map *nest_lock,
    unsigned long ip)
    __acquires(lock) __no_context_analysis
    {
    let mut ret = 0;
    might_sleep();
    mutex_acquire_nest(&lock.dep_map, subclass, 0, nest_lock, ip);
    ret = __rt_mutex_lock(&lock.rtmutex, state);
    if (ret) {
    mutex_release(&lock.dep_map, ip);
    }
    else {
    lock_acquired(&lock.dep_map, ip);
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn mutex_rt_init_lockdep(mutex: *mut mutex, name: *const c_char, key: *mut lock_class_key) {
    __mutex_rt_init_generic(mutex);
    lockdep_init_map_wait(&mutex.dep_map, name, key, 0, LD_WAIT_SLEEP);
    }
    EXPORT_SYMBOL(mutex_rt_init_lockdep);
#[no_mangle]
pub unsafe extern "C" fn mutex_lock_nested(lock: *mut mutex, subclass: c_uint) -> void __sched {
    __mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, subclass, core::ptr::null_mut(), _RET_IP_);
    }
    EXPORT_SYMBOL_GPL(mutex_lock_nested);
    void __sched _mutex_lock_nest_lock(mutex *lock, lockdep_map *nest_lock)
    {
    __mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, 0, nest_lock, _RET_IP_);
    }
    EXPORT_SYMBOL_GPL(_mutex_lock_nest_lock);
    int __sched mutex_lock_interruptible_nested(mutex *lock,
    unsigned int subclass)
    {
    return __mutex_lock_common(lock, TASK_INTERRUPTIBLE, subclass, core::ptr::null_mut(), _RET_IP_);
    }
    EXPORT_SYMBOL_GPL(mutex_lock_interruptible_nested);
    int __sched _mutex_lock_killable(mutex *lock, unsigned int subclass, lockdep_map *nest_lock)
    {
    return __mutex_lock_common(lock, TASK_KILLABLE, subclass, nest_lock, _RET_IP_);
    }
    EXPORT_SYMBOL_GPL(_mutex_lock_killable);
#[no_mangle]
pub unsafe extern "C" fn mutex_lock_io_nested(lock: *mut mutex, subclass: c_uint) -> void __sched {
    let mut token = 0;
    might_sleep();
    token = io_schedule_prepare();
    __mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, subclass, core::ptr::null_mut(), _RET_IP_);
    io_schedule_finish(token);
    }
    EXPORT_SYMBOL_GPL(mutex_lock_io_nested);
    int __sched _mutex_trylock_nest_lock(mutex *lock, lockdep_map *nest_lock)
    {
    let mut ret = 0;
    if (IS_ENABLED!(CONFIG_DEBUG_RT_MUTEXES) && WARN_ON_ONCE!(!in_task())) {
    return 0;
    }
    ret = __rt_mutex_trylock(&lock.rtmutex);
    if (ret) {
    mutex_acquire_nest(&lock.dep_map, 0, 1, nest_lock, _RET_IP_);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(_mutex_trylock_nest_lock);

#[no_mangle]
pub unsafe extern "C" fn mutex_rt_init_generic(mutex: *mut mutex) {
    __mutex_rt_init_generic(mutex);
    }
    EXPORT_SYMBOL(mutex_rt_init_generic);
#[no_mangle]
pub unsafe extern "C" fn mutex_lock(lock: *mut mutex) -> void __sched {
    __mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, 0, core::ptr::null_mut(), _RET_IP_);
    }
    EXPORT_SYMBOL(mutex_lock);
#[no_mangle]
pub unsafe extern "C" fn mutex_lock_interruptible(lock: *mut mutex) -> int __sched {
    return __mutex_lock_common(lock, TASK_INTERRUPTIBLE, 0, core::ptr::null_mut(), _RET_IP_);
    }
    EXPORT_SYMBOL(mutex_lock_interruptible);
#[no_mangle]
pub unsafe extern "C" fn mutex_lock_killable(lock: *mut mutex) -> int __sched {
    return __mutex_lock_common(lock, TASK_KILLABLE, 0, core::ptr::null_mut(), _RET_IP_);
    }
    EXPORT_SYMBOL(mutex_lock_killable);
#[no_mangle]
pub unsafe extern "C" fn mutex_lock_io(lock: *mut mutex) -> void __sched {
pub static mut token: c_int = 0;
    __mutex_lock_common(lock, TASK_UNINTERRUPTIBLE, 0, core::ptr::null_mut(), _RET_IP_);
    io_schedule_finish(token);
    }
    EXPORT_SYMBOL(mutex_lock_io);
#[no_mangle]
pub unsafe extern "C" fn mutex_trylock(lock: *mut mutex) -> int __sched {
    if (IS_ENABLED!(CONFIG_DEBUG_RT_MUTEXES) && WARN_ON_ONCE!(!in_task())) {
    return 0;
    }
    return __rt_mutex_trylock(&lock.rtmutex);
    }
    EXPORT_SYMBOL(mutex_trylock);

#[no_mangle]
pub unsafe extern "C" fn mutex_unlock(lock: *mut mutex) -> void __sched {
    void __sched mutex_unlock(mutex *lock)
    __releases(lock) __no_context_analysis
    {
    mutex_release(&lock.dep_map, _RET_IP_);
    __rt_mutex_unlock(&lock.rtmutex);
    }
    EXPORT_SYMBOL(mutex_unlock);
}
