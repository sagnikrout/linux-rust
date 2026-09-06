//! Automatically rewritten from C to Rust
//! Source: lib/lockref.c
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
// Note that the "cmpxchg()" reloads the "old" value for the
// failure case.
//

    int retry = 100;							\
    struct lockref old;							\
    BUILD_BUG_ON(sizeof(old) != 8);						\
    old.lock_count = READ_ONCE(lockref.lock_count);			\
    while (likely(arch_spin_value_unlocked(old.lock.rlock.raw_lock))) {  	\
    struct lockref new = old;					\
    CODE								\
    if (likely(try_cmpxchg64_relaxed(&lockref.lock_count,		\
    &old.lock_count,		\
    new.lock_count))) {		\
    SUCCESS;						\
    }								\
    if (!--retry)							\
    break;							\
    }									\
    } while (0)

//
// lockref_get - Increments reference count unconditionally
// @lockref: pointer to lockref structure
//
// This operation is only valid if you already hold a reference
// to the object, so you know the count cannot be zero.
//
#[no_mangle]
pub unsafe extern "C" fn lockref_get(lockref: *mut lockref) {
    void lockref_get(struct lockref *lockref)
    {
    CMPXCHG_LOOP(
    new.count++;
    ,
    return;
    );
    spin_lock(&lockref.lock);
    lockref.count++;
    spin_unlock(&lockref.lock);
    }
    EXPORT_SYMBOL(lockref_get);
//
// lockref_get_not_zero - Increments count unless the count is 0 or dead
// @lockref: pointer to lockref structure
// Return: 1 if count updated successfully or 0 if count was zero
//
#[no_mangle]
pub unsafe extern "C" fn lockref_get_not_zero(lockref: *mut lockref) -> bool {
    bool lockref_get_not_zero(struct lockref *lockref)
    {
    let mut retval: bool = false;
    CMPXCHG_LOOP(
    new.count++;
    if (old.count <= 0)
    return false;
    ,
    return true;
    );
    spin_lock(&lockref.lock);
    if (lockref.count > 0) {
    lockref.count++;
    retval = true;
    }
    spin_unlock(&lockref.lock);
    return retval;
    }
    EXPORT_SYMBOL(lockref_get_not_zero);
//
// lockref_put_return - Decrement reference count if possible
// @lockref: pointer to lockref structure
//
// Decrement the reference count and return the new value.
// If the lockref was dead or locked, return -1.
//
#[no_mangle]
pub unsafe extern "C" fn lockref_put_return(lockref: *mut lockref) -> c_int {
    int lockref_put_return(struct lockref *lockref)
    {
    CMPXCHG_LOOP(
    new.count--;
    if (old.count <= 0)
    return -1;
    ,
    return new.count;
    );
    return -1;
    }
    EXPORT_SYMBOL(lockref_put_return);
//
// lockref_put_or_lock - decrements count unless count <= 1 before decrement
// @lockref: pointer to lockref structure
// Return: 1 if count updated successfully or 0 if count <= 1 and lock taken
//
#[no_mangle]
pub unsafe extern "C" fn lockref_put_or_lock(lockref: *mut lockref) -> bool {
    bool lockref_put_or_lock(struct lockref *lockref)
    {
    CMPXCHG_LOOP(
    new.count--;
    if (old.count <= 1)
    break;
    ,
    return true;
    );
    spin_lock(&lockref.lock);
    if (lockref.count <= 1)
    return false;
    lockref.count--;
    spin_unlock(&lockref.lock);
    return true;
    }
    EXPORT_SYMBOL(lockref_put_or_lock);
//
// lockref_mark_dead - mark lockref dead
// @lockref: pointer to lockref structure
//
#[no_mangle]
pub unsafe extern "C" fn lockref_mark_dead(lockref: *mut lockref) {
    void lockref_mark_dead(struct lockref *lockref)
    {
    assert_spin_locked(&lockref.lock);
    lockref.count = __LOCKREF_DEAD_VAL;
    }
    EXPORT_SYMBOL(lockref_mark_dead);
//
// lockref_get_not_dead - Increments count unless the ref is dead
// @lockref: pointer to lockref structure
// Return: 1 if count updated successfully or 0 if lockref was dead
//
#[no_mangle]
pub unsafe extern "C" fn lockref_get_not_dead(lockref: *mut lockref) -> bool {
    bool lockref_get_not_dead(struct lockref *lockref)
    {
    let mut retval: bool = false;
    CMPXCHG_LOOP(
    new.count++;
    if (old.count < 0)
    return false;
    ,
    return true;
    );
    spin_lock(&lockref.lock);
    if (lockref.count >= 0) {
    lockref.count++;
    retval = true;
    }
    spin_unlock(&lockref.lock);
    return retval;
    }
    EXPORT_SYMBOL(lockref_get_not_dead);
