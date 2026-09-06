//! Automatically rewritten from C to Rust
//! Source: kernel/locking/spinlock_rt.c
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
// PREEMPT_RT substitution for spin/rw_locks
//
// spinlocks and rwlocks on RT are based on rtmutexes, with a few twists to
// resemble the non RT semantics:
//
// - Contrary to plain rtmutexes, spinlocks and rwlocks are state
// preserving. The task state is saved before blocking on the underlying
// rtmutex, and restored when the lock has been acquired. Regular wakeups
// during that time are redirected to the saved state so no wake up is
// missed.
//
// - Non RT spin/rwlocks disable preemption and eventually interrupts.
// Disabling preemption has the side effect of disabling migration and
// preventing RCU grace periods.
//
// The RT substitutions explicitly disable migration and take
// rcu_read_lock() across the lock held section.
//

// Macro flag: #define RT_MUTEX_BUILD_SPINLOCKS

//
// __might_resched() skips the state check as rtlocks are state
// preserving. Take RCU nesting into account as spin/read/write_lock() can
// legitimately nest into an RCU read side critical section.
//

    (rcu_preempt_depth() << MIGHT_RESCHED_RCU_SHIFT)

    __might_resched(__FILE__, __LINE__, RTLOCK_RESCHED_OFFSETS)
#[no_mangle]
unsafe extern "C" fn rtlock_lock(rtm: *mut rt_mutex_base) -> __always_inline void {
    lockdep_assert(!current.pi_blocked_on);
    if (unlikely(!rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current))) {
    rtlock_slowlock(rtm);
    }
    }
#[no_mangle]
unsafe extern "C" fn __rt_spin_lock(lock: *mut spinlock_t) -> __always_inline void {
    rtlock_might_resched();
    rtlock_lock(&lock.lock);
    rcu_read_lock();
    migrate_disable();
    }
#[no_mangle]
pub unsafe extern "C" fn rt_spin_lock(__acquires(RCU: *mut *mut spinlock_t lock)) -> void __sched {
    void __sched rt_spin_lock(spinlock_t *lock) __acquires(RCU)
    {
    spin_acquire(&lock.dep_map, 0, 0, _RET_IP_);
    __rt_spin_lock(lock);
    }
    EXPORT_SYMBOL(rt_spin_lock);

#[no_mangle]
pub unsafe extern "C" fn rt_spin_lock_nested(lock: *mut spinlock_t, subclass: c_int) -> void __sched {
    spin_acquire(&lock.dep_map, subclass, 0, _RET_IP_);
    __rt_spin_lock(lock);
    }
    EXPORT_SYMBOL(rt_spin_lock_nested);
    void __sched rt_spin_lock_nest_lock(spinlock_t *lock, lockdep_map *nest_lock)
    {
    spin_acquire_nest(&lock.dep_map, 0, 0, nest_lock, _RET_IP_);
    __rt_spin_lock(lock);
    }
    EXPORT_SYMBOL(rt_spin_lock_nest_lock);

#[no_mangle]
pub unsafe extern "C" fn rt_spin_unlock(__releases(RCU: *mut *mut spinlock_t lock)) -> void __sched {
    void __sched rt_spin_unlock(spinlock_t *lock) __releases(RCU)
    {
    spin_release(&lock.dep_map, _RET_IP_);
    migrate_enable();
    if (unlikely(!rt_mutex_cmpxchg_release(&lock.lock, current, core::ptr::null_mut()))) {
    rt_mutex_slowunlock(&lock.lock);
    }
//
// This must be last to prevent the following UAF:
//
// T1					T2
// spin_lock(&p->lock);			rcu_read_lock();
// invalidate(p);			p = rcu_dereference(ptr);
// rcu_assign_pointer(ptr, NULL);	if (!p) return;
// spin_unlock(&p->lock);		spin_lock(&p->lock);
// kfree_rcu(p);			rcu_read_unlock();
// ....
// spin_unlock(&p->lock)
// rcu_read_unlock(); // Ends grace period
// rcu_do_batch()
// kfree(p);
// UAF ->	  rt_mutex_cmpxchg_release(&p->lock.lock...)
//
    rcu_read_unlock();
    }
    EXPORT_SYMBOL(rt_spin_unlock);
//
// Wait for the lock to get unlocked: instead of polling for an unlock
// (like raw spinlocks do), lock and unlock, to force the kernel to
// schedule if there's contention:
//
#[no_mangle]
pub unsafe extern "C" fn rt_spin_lock_unlock(lock: *mut spinlock_t) -> void __sched {
    spin_lock(lock);
    spin_unlock(lock);
    }
    EXPORT_SYMBOL(rt_spin_lock_unlock);
#[no_mangle]
unsafe extern "C" fn __rt_spin_trylock(lock: *mut spinlock_t) -> __always_inline int {
pub static mut ret: c_int = 1;
    if (unlikely(!rt_mutex_cmpxchg_acquire(&lock.lock, core::ptr::null_mut(), current))) {
    ret = rt_mutex_slowtrylock(&lock.lock);
    }
    if (ret) {
    spin_acquire(&lock.dep_map, 0, 1, _RET_IP_);
    rcu_read_lock();
    migrate_disable();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rt_spin_trylock(lock: *mut spinlock_t) -> int __sched {
    return __rt_spin_trylock(lock);
    }
    EXPORT_SYMBOL(rt_spin_trylock);
#[no_mangle]
pub unsafe extern "C" fn rt_spin_trylock_bh(lock: *mut spinlock_t) -> int __sched {
    let mut ret = 0;
    local_bh_disable();
    ret = __rt_spin_trylock(lock);
    if (!ret) {
    local_bh_enable();
    }
    return ret;
    }
    EXPORT_SYMBOL(rt_spin_trylock_bh);

#[no_mangle]
pub unsafe extern "C" fn __rt_spin_lock_init(lock: *mut spinlock_t, name: *mut c_char, key: *mut lock_class_key, percpu: bool) {
pub static mut type: u8 = 0;
    debug_check_no_locks_freed(lock, sizeof!(*lock));
    lockdep_init_map_type(&lock.dep_map, name, key, 0, LD_WAIT_CONFIG,
    LD_WAIT_INV, type);
    }
    EXPORT_SYMBOL(__rt_spin_lock_init);

//
// RT-specific reader/writer locks
//

    current_save_and_set_rtlock_wait_state()

    current_restore_rtlock_saved_state()
    static __always_inline int
    rwbase_rtmutex_lock_state(rt_mutex_base *rtm, unsigned int state)
    {
    if (unlikely(!rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current))) {
    rtlock_slowlock(rtm);
    }
    return 0;
    }
    static __always_inline int
    rwbase_rtmutex_slowlock_locked(rt_mutex_base *rtm, unsigned int state, wake_q_head *wake_q)
    {
    rtlock_slowlock_locked(rtm, wake_q);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rwbase_rtmutex_unlock(rtm: *mut rt_mutex_base) -> __always_inline void {
    if (likely(rt_mutex_cmpxchg_acquire(rtm, current, core::ptr::null_mut()))) {
    return;
    }
    rt_mutex_slowunlock(rtm);
    }
#[no_mangle]
unsafe extern "C" fn rwbase_rtmutex_trylock(rtm: *mut rt_mutex_base) -> __always_inline int {
    if (likely(rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current))) {
    return 1;
    }
    return rt_mutex_slowtrylock(rtm);
    }

// Macro flag: #define rwbase_pre_schedule()

    schedule_rtlock()
// Macro flag: #define rwbase_post_schedule()

//
// The common functions which get wrapped into the rwlock API.
//
#[no_mangle]
pub unsafe extern "C" fn rt_read_trylock(rwlock: *mut rwlock_t) -> int __sched {
    let mut ret = 0;
    ret = rwbase_read_trylock(&rwlock.rwbase);
    if (ret) {
    rwlock_acquire_read(&rwlock.dep_map, 0, 1, _RET_IP_);
    rcu_read_lock();
    migrate_disable();
    }
    return ret;
    }
    EXPORT_SYMBOL(rt_read_trylock);
#[no_mangle]
pub unsafe extern "C" fn rt_write_trylock(rwlock: *mut rwlock_t) -> int __sched {
    let mut ret = 0;
    ret = rwbase_write_trylock(&rwlock.rwbase);
    if (ret) {
    rwlock_acquire(&rwlock.dep_map, 0, 1, _RET_IP_);
    rcu_read_lock();
    migrate_disable();
    }
    return ret;
    }
    EXPORT_SYMBOL(rt_write_trylock);
#[no_mangle]
pub unsafe extern "C" fn rt_read_lock(__acquires(RCU: *mut *mut rwlock_t rwlock)) -> void __sched {
    void __sched rt_read_lock(rwlock_t *rwlock) __acquires(RCU)
    {
    rtlock_might_resched();
    rwlock_acquire_read(&rwlock.dep_map, 0, 0, _RET_IP_);
    rwbase_read_lock(&rwlock.rwbase, TASK_RTLOCK_WAIT);
    rcu_read_lock();
    migrate_disable();
    }
    EXPORT_SYMBOL(rt_read_lock);
#[no_mangle]
pub unsafe extern "C" fn rt_write_lock(__acquires(RCU: *mut *mut rwlock_t rwlock)) -> void __sched {
    void __sched rt_write_lock(rwlock_t *rwlock) __acquires(RCU)
    {
    rtlock_might_resched();
    rwlock_acquire(&rwlock.dep_map, 0, 0, _RET_IP_);
    rwbase_write_lock(&rwlock.rwbase, TASK_RTLOCK_WAIT);
    rcu_read_lock();
    migrate_disable();
    }
    EXPORT_SYMBOL(rt_write_lock);

#[no_mangle]
pub unsafe extern "C" fn rt_write_lock_nested(rwlock: *mut rwlock_t, __acquires(RCU: int subclass)) -> void __sched {
    void __sched rt_write_lock_nested(rwlock_t *rwlock, int subclass) __acquires(RCU)
    {
    rtlock_might_resched();
    rwlock_acquire(&rwlock.dep_map, subclass, 0, _RET_IP_);
    rwbase_write_lock(&rwlock.rwbase, TASK_RTLOCK_WAIT);
    rcu_read_lock();
    migrate_disable();
    }
    EXPORT_SYMBOL(rt_write_lock_nested);

#[no_mangle]
pub unsafe extern "C" fn rt_read_unlock(__releases(RCU: *mut *mut rwlock_t rwlock)) -> void __sched {
    void __sched rt_read_unlock(rwlock_t *rwlock) __releases(RCU)
    {
    rwlock_release(&rwlock.dep_map, _RET_IP_);
    migrate_enable();
    rwbase_read_unlock(&rwlock.rwbase, TASK_RTLOCK_WAIT);
// This must be last. See comment in rt_spin_unlock()
    rcu_read_unlock();
    }
    EXPORT_SYMBOL(rt_read_unlock);
#[no_mangle]
pub unsafe extern "C" fn rt_write_unlock(__releases(RCU: *mut *mut rwlock_t rwlock)) -> void __sched {
    void __sched rt_write_unlock(rwlock_t *rwlock) __releases(RCU)
    {
    rwlock_release(&rwlock.dep_map, _RET_IP_);
    migrate_enable();
    rwbase_write_unlock(&rwlock.rwbase);
// This must be last. See comment in rt_spin_unlock()
    rcu_read_unlock();
    }
    EXPORT_SYMBOL(rt_write_unlock);

#[no_mangle]
pub unsafe extern "C" fn __rt_rwlock_init(rwlock: *mut rwlock_t, name: *mut c_char, key: *mut lock_class_key) {
    debug_check_no_locks_freed(rwlock, sizeof!(*rwlock));
    lockdep_init_map_wait(&rwlock.dep_map, name, key, 0, LD_WAIT_CONFIG);
    }
    EXPORT_SYMBOL(__rt_rwlock_init);
}
}
}
}
}
}
}
