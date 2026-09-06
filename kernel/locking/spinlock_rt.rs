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
    static __always_inline void rtlock_lock(struct rt_mutex_base *rtm)
    {
    lockdep_assert(!current.pi_blocked_on);
    if (unlikely(!rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current)))
    rtlock_slowlock(rtm);
    }
#[no_mangle]
unsafe extern "C" fn __rt_spin_lock(lock: *mut spinlock_t) -> __always_inline void {
    static __always_inline void __rt_spin_lock(spinlock_t *lock)
    {
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
    void __sched rt_spin_lock_nested(spinlock_t *lock, int subclass)
    {
    spin_acquire(&lock.dep_map, subclass, 0, _RET_IP_);
    __rt_spin_lock(lock);
    }
    EXPORT_SYMBOL(rt_spin_lock_nested);
    void __sched rt_spin_lock_nest_lock(spinlock_t *lock,
    struct lockdep_map *nest_lock)
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
    if (unlikely(!rt_mutex_cmpxchg_release(&lock.lock, current, core::ptr::null_mut())))
    rt_mutex_slowunlock(&lock.lock);
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
    void __sched rt_spin_lock_unlock(spinlock_t *lock)
    {
    spin_lock(lock);
    spin_unlock(lock);
    }
    EXPORT_SYMBOL(rt_spin_lock_unlock);
#[no_mangle]
unsafe extern "C" fn __rt_spin_trylock(lock: *mut spinlock_t) -> __always_inline int {
    static __always_inline int __rt_spin_trylock(spinlock_t *lock)
    {
    let mut ret: c_int = 1;
    if (unlikely(!rt_mutex_cmpxchg_acquire(&lock.lock, core::ptr::null_mut(), current)))
    ret = rt_mutex_slowtrylock(&lock.lock);
    if (ret) {
    spin_acquire(&lock.dep_map, 0, 1, _RET_IP_);
    rcu_read_lock();
    migrate_disable();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rt_spin_trylock(lock: *mut spinlock_t) -> int __sched {
    int __sched rt_spin_trylock(spinlock_t *lock)
    {
    return __rt_spin_trylock(lock);
    }
    EXPORT_SYMBOL(rt_spin_trylock);
#[no_mangle]
pub unsafe extern "C" fn rt_spin_trylock_bh(lock: *mut spinlock_t) -> int __sched {
    int __sched rt_spin_trylock_bh(spinlock_t *lock)
    {
    int ret;
    local_bh_disable();
    ret = __rt_spin_trylock(lock);
    if (!ret)
    local_bh_enable();
    return ret;
    }
    EXPORT_SYMBOL(rt_spin_trylock_bh);

    void __rt_spin_lock_init(spinlock_t *lock, const char *name,
    struct lock_class_key *key, bool percpu)
    {
    let mut type: u8 = percpu ? LD_LOCK_PERCPU : LD_LOCK_NORMAL;
    debug_check_no_locks_freed((void *)lock, sizeof(*lock));
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
    rwbase_rtmutex_lock_state(struct rt_mutex_base *rtm, unsigned int state)
    {
    if (unlikely(!rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current)))
    rtlock_slowlock(rtm);
    return 0;
    }
    static __always_inline int
    rwbase_rtmutex_slowlock_locked(struct rt_mutex_base *rtm, unsigned int state,
    struct wake_q_head *wake_q)
    {
    rtlock_slowlock_locked(rtm, wake_q);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rwbase_rtmutex_unlock(rtm: *mut rt_mutex_base) -> __always_inline void {
    static __always_inline void rwbase_rtmutex_unlock(struct rt_mutex_base *rtm)
    {
    if (likely(rt_mutex_cmpxchg_acquire(rtm, current, core::ptr::null_mut())))
    return;
    rt_mutex_slowunlock(rtm);
    }
#[no_mangle]
unsafe extern "C" fn rwbase_rtmutex_trylock(rtm: *mut rt_mutex_base) -> __always_inline int {
    static __always_inline int  rwbase_rtmutex_trylock(struct rt_mutex_base *rtm)
    {
    if (likely(rt_mutex_cmpxchg_acquire(rtm, core::ptr::null_mut(), current)))
    return 1;
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
    int __sched rt_read_trylock(rwlock_t *rwlock)
    {
    int ret;
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
    int __sched rt_write_trylock(rwlock_t *rwlock)
    {
    int ret;
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

    void __rt_rwlock_init(rwlock_t *rwlock, const char *name,
    struct lock_class_key *key)
    {
    debug_check_no_locks_freed((void *)rwlock, sizeof(*rwlock));
    lockdep_init_map_wait(&rwlock.dep_map, name, key, 0, LD_WAIT_CONFIG);
    }
    EXPORT_SYMBOL(__rt_rwlock_init);
