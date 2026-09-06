//! Automatically rewritten from C to Rust
//! Source: kernel/locking/spinlock_debug.c
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


//
// Copyright 2005, Red Hat, Inc., Ingo Molnar
// Released under the General Public License (GPL).
//
// This file contains the spinlock/rwlock implementations for
// DEBUG_SPINLOCK.
//

    void __raw_spin_lock_init(raw_spinlock_t *lock, const char *name,
    struct lock_class_key *key, short inner)
    {

//
// Make sure we are not reinitializing a held lock:
//
    debug_check_no_locks_freed((void *)lock, sizeof(*lock));
    lockdep_init_map_wait(&lock.dep_map, name, key, 0, inner);

    lock.raw_lock = (arch_spinlock_t)__ARCH_SPIN_LOCK_UNLOCKED;
    lock.magic = SPINLOCK_MAGIC;
    lock.owner = SPINLOCK_OWNER_INIT;
    lock.owner_cpu = -1;
    }
    EXPORT_SYMBOL(__raw_spin_lock_init);

    void __rwlock_init(rwlock_t *lock, const char *name,
    struct lock_class_key *key)
    {

//
// Make sure we are not reinitializing a held lock:
//
    debug_check_no_locks_freed((void *)lock, sizeof(*lock));
    lockdep_init_map_wait(&lock.dep_map, name, key, 0, LD_WAIT_CONFIG);

    lock.raw_lock = (arch_rwlock_t) __ARCH_RW_LOCK_UNLOCKED;
    lock.magic = RWLOCK_MAGIC;
    lock.owner = SPINLOCK_OWNER_INIT;
    lock.owner_cpu = -1;
    }
    EXPORT_SYMBOL(__rwlock_init);

#[no_mangle]
unsafe extern "C" fn spin_dump(lock: *mut raw_spinlock_t, msg: *const c_char) {
    static void spin_dump(raw_spinlock_t *lock, const char *msg)
    {
    struct task_struct *owner = READ_ONCE(lock.owner);
    if (owner == SPINLOCK_OWNER_INIT)
    owner = core::ptr::null_mut();
    printk(KERN_EMERG "BUG: spinlock %s on CPU#%d, %s/%d\n",
    msg, raw_smp_processor_id(),
    current.comm, task_pid_nr(current));
    printk(KERN_EMERG " lock: %pS, .magic: %08x, .owner: %s/%d, "
    ".owner_cpu: %d\n",
    lock, READ_ONCE(lock.magic),
    owner ? owner.comm : "<none>",
    owner ? task_pid_nr(owner) : -1,
    READ_ONCE(lock.owner_cpu));
    dump_stack();
    }
#[no_mangle]
unsafe extern "C" fn spin_bug(lock: *mut raw_spinlock_t, msg: *const c_char) {
    static void spin_bug(raw_spinlock_t *lock, const char *msg)
    {
    if (!debug_locks_off())
    return;
    spin_dump(lock, msg);
    }

    static inline void
    debug_spin_lock_before(raw_spinlock_t *lock)
    {
    SPIN_BUG_ON(READ_ONCE(lock.magic) != SPINLOCK_MAGIC, lock, "bad magic");
    SPIN_BUG_ON(READ_ONCE(lock.owner) == current, lock, "recursion");
    SPIN_BUG_ON(READ_ONCE(lock.owner_cpu) == raw_smp_processor_id(),
    lock, "cpu recursion");
    }
#[no_mangle]
pub unsafe extern "C" fn debug_spin_lock_after(lock: *mut raw_spinlock_t) {
    static inline void debug_spin_lock_after(raw_spinlock_t *lock)
    {
    WRITE_ONCE(lock.owner_cpu, raw_smp_processor_id());
    WRITE_ONCE(lock.owner, current);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_spin_unlock(lock: *mut raw_spinlock_t) {
    static inline void debug_spin_unlock(raw_spinlock_t *lock)
    {
    SPIN_BUG_ON(lock.magic != SPINLOCK_MAGIC, lock, "bad magic");
    SPIN_BUG_ON(!raw_spin_is_locked(lock), lock, "already unlocked");
    SPIN_BUG_ON(lock.owner != current, lock, "wrong owner");
    SPIN_BUG_ON(lock.owner_cpu != raw_smp_processor_id(),
    lock, "wrong CPU");
    WRITE_ONCE(lock.owner, SPINLOCK_OWNER_INIT);
    WRITE_ONCE(lock.owner_cpu, -1);
    }
//
// We are now relying on the NMI watchdog to detect lockup instead of doing
// the detection here with an unfair lock which can cause problem of its own.
//
#[no_mangle]
pub unsafe extern "C" fn do_raw_spin_lock(lock: *mut raw_spinlock_t) {
    void do_raw_spin_lock(raw_spinlock_t *lock)
    {
    debug_spin_lock_before(lock);
    arch_spin_lock(&lock.raw_lock);
    mmiowb_spin_lock();
    debug_spin_lock_after(lock);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_spin_trylock(lock: *mut raw_spinlock_t) -> c_int {
    int do_raw_spin_trylock(raw_spinlock_t *lock)
    {
    let mut ret: c_int = arch_spin_trylock(&lock.raw_lock);
    if (ret) {
    mmiowb_spin_lock();
    debug_spin_lock_after(lock);
    }

//
// Must not happen on UP:
//
    SPIN_BUG_ON(!ret, lock, "trylock failure on UP");

    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_spin_unlock(lock: *mut raw_spinlock_t) {
    void do_raw_spin_unlock(raw_spinlock_t *lock)
    {
    mmiowb_spin_unlock();
    debug_spin_unlock(lock);
    arch_spin_unlock(&lock.raw_lock);
    }

#[no_mangle]
unsafe extern "C" fn rwlock_bug(lock: *mut rwlock_t, msg: *const c_char) {
    static void rwlock_bug(rwlock_t *lock, const char *msg)
    {
    if (!debug_locks_off())
    return;
    printk(KERN_EMERG "BUG: rwlock %s on CPU#%d, %s/%d, %p\n",
    msg, raw_smp_processor_id(), current.comm,
    task_pid_nr(current), lock);
    dump_stack();
    }

#[no_mangle]
pub unsafe extern "C" fn do_raw_read_lock(lock: *mut rwlock_t) {
    void do_raw_read_lock(rwlock_t *lock)
    {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    arch_read_lock(&lock.raw_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_read_trylock(lock: *mut rwlock_t) -> c_int {
    int do_raw_read_trylock(rwlock_t *lock)
    {
    let mut ret: c_int = arch_read_trylock(&lock.raw_lock);

//
// Must not happen on UP:
//
    RWLOCK_BUG_ON(!ret, lock, "trylock failure on UP");

    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_read_unlock(lock: *mut rwlock_t) {
    void do_raw_read_unlock(rwlock_t *lock)
    {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    arch_read_unlock(&lock.raw_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_write_lock_before(lock: *mut rwlock_t) {
    static inline void debug_write_lock_before(rwlock_t *lock)
    {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    RWLOCK_BUG_ON(READ_ONCE(lock.owner) == current, lock, "recursion");
    RWLOCK_BUG_ON(READ_ONCE(lock.owner_cpu) == raw_smp_processor_id(),
    lock, "cpu recursion");
    }
#[no_mangle]
pub unsafe extern "C" fn debug_write_lock_after(lock: *mut rwlock_t) {
    static inline void debug_write_lock_after(rwlock_t *lock)
    {
    WRITE_ONCE(lock.owner_cpu, raw_smp_processor_id());
    WRITE_ONCE(lock.owner, current);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_write_unlock(lock: *mut rwlock_t) {
    static inline void debug_write_unlock(rwlock_t *lock)
    {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    RWLOCK_BUG_ON(lock.owner != current, lock, "wrong owner");
    RWLOCK_BUG_ON(lock.owner_cpu != raw_smp_processor_id(),
    lock, "wrong CPU");
    WRITE_ONCE(lock.owner, SPINLOCK_OWNER_INIT);
    WRITE_ONCE(lock.owner_cpu, -1);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_write_lock(lock: *mut rwlock_t) {
    void do_raw_write_lock(rwlock_t *lock)
    {
    debug_write_lock_before(lock);
    arch_write_lock(&lock.raw_lock);
    debug_write_lock_after(lock);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_write_trylock(lock: *mut rwlock_t) -> c_int {
    int do_raw_write_trylock(rwlock_t *lock)
    {
    let mut ret: c_int = arch_write_trylock(&lock.raw_lock);
    if (ret)
    debug_write_lock_after(lock);

//
// Must not happen on UP:
//
    RWLOCK_BUG_ON(!ret, lock, "trylock failure on UP");

    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_write_unlock(lock: *mut rwlock_t) {
    void do_raw_write_unlock(rwlock_t *lock)
    {
    debug_write_unlock(lock);
    arch_write_unlock(&lock.raw_lock);
    }
