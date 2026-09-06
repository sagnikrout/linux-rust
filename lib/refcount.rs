//! Automatically rewritten from C to Rust
//! Source: lib/refcount.c
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
// Out-of-line refcount functions.
//

#[no_mangle]
pub unsafe extern "C" fn refcount_warn_saturate(r: *mut refcount_t, t: enum refcount_saturation_type) {
    void refcount_warn_saturate(refcount_t *r, enum refcount_saturation_type t)
    {
    refcount_set(r, REFCOUNT_SATURATED);
    switch (t) {
    case REFCOUNT_ADD_NOT_ZERO_OVF:
    REFCOUNT_WARN("saturated; leaking memory");
    break;
    case REFCOUNT_ADD_OVF:
    REFCOUNT_WARN("saturated; leaking memory");
    break;
    case REFCOUNT_ADD_UAF:
    REFCOUNT_WARN("addition on 0; use-after-free");
    break;
    case REFCOUNT_SUB_UAF:
    REFCOUNT_WARN("underflow; use-after-free");
    break;
    case REFCOUNT_DEC_LEAK:
    REFCOUNT_WARN("decrement hit 0; leaking memory");
    break;
    default:
    REFCOUNT_WARN("unknown saturation event!?");
    }
    }
    EXPORT_SYMBOL(refcount_warn_saturate);
//
// refcount_dec_if_one - decrement a refcount if it is 1
// @r: the refcount
//
// No atomic_t counterpart, it attempts a 1 -> 0 transition and returns the
// success thereof.
//
// Like all decrement operations, it provides release memory order and provides
// a control dependency.
//
// It can be used like a try-delete operator; this explicit case is provided
// and not cmpxchg in generic, because that would allow implementing unsafe
// operations.
//
// Return: true if the resulting refcount is 0, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_if_one(r: *mut refcount_t) -> bool {
    bool refcount_dec_if_one(refcount_t *r)
    {
    let mut val: c_int = 1;
    return atomic_try_cmpxchg_release(&r.refs, &val, 0);
    }
    EXPORT_SYMBOL(refcount_dec_if_one);
//
// refcount_dec_not_one - decrement a refcount if it is not 1
// @r: the refcount
//
// No atomic_t counterpart, it decrements unless the value is 1, in which case
// it will return false.
//
// Was often done like: atomic_add_unless(&var, -1, 1)
//
// Return: true if the decrement operation was successful, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_not_one(r: *mut refcount_t) -> bool {
    bool refcount_dec_not_one(refcount_t *r)
    {
    unsigned int new, val = atomic_read(&r.refs);
    do {
    if (unlikely(val == REFCOUNT_SATURATED))
    return true;
    if (val == 1)
    return false;
    new = val - 1;
    if (new > val) {
    WARN_ONCE(new > val, "refcount_t: underflow; use-after-free.\n");
    return true;
    }
    } while (!atomic_try_cmpxchg_release(&r.refs, &val, new));
    return true;
    }
    EXPORT_SYMBOL(refcount_dec_not_one);
//
// refcount_dec_and_mutex_lock - return holding mutex if able to decrement
// refcount to 0
// @r: the refcount
// @lock: the mutex to be locked
//
// Similar to atomic_dec_and_mutex_lock(), it will WARN on underflow and fail
// to decrement when saturated at REFCOUNT_SATURATED.
//
// Provides release memory ordering, such that prior loads and stores are done
// before, and provides a control dependency such that free() must come after.
// See the comment on top.
//
// Return: true and hold mutex if able to decrement refcount to 0, false
// otherwise
//
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_and_mutex_lock(r: *mut refcount_t, lock: *mut mutex) -> bool {
    bool refcount_dec_and_mutex_lock(refcount_t *r, struct mutex *lock)
    {
    if (refcount_dec_not_one(r))
    return false;
    mutex_lock(lock);
    if (!refcount_dec_and_test(r)) {
    mutex_unlock(lock);
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL(refcount_dec_and_mutex_lock);
//
// refcount_dec_and_lock - return holding spinlock if able to decrement
// refcount to 0
// @r: the refcount
// @lock: the spinlock to be locked
//
// Similar to atomic_dec_and_lock(), it will WARN on underflow and fail to
// decrement when saturated at REFCOUNT_SATURATED.
//
// Provides release memory ordering, such that prior loads and stores are done
// before, and provides a control dependency such that free() must come after.
// See the comment on top.
//
// Return: true and hold spinlock if able to decrement refcount to 0, false
// otherwise
//
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_and_lock(r: *mut refcount_t, lock: *mut spinlock_t) -> bool {
    bool refcount_dec_and_lock(refcount_t *r, spinlock_t *lock)
    {
    if (refcount_dec_not_one(r))
    return false;
    spin_lock(lock);
    if (!refcount_dec_and_test(r)) {
    spin_unlock(lock);
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL(refcount_dec_and_lock);
//
// refcount_dec_and_lock_irqsave - return holding spinlock with disabled
// interrupts if able to decrement refcount to 0
// @r: the refcount
// @lock: the spinlock to be locked
// @flags: saved IRQ-flags if the is acquired
//
// Same as refcount_dec_and_lock() above except that the spinlock is acquired
// with disabled interrupts.
//
// Return: true and hold spinlock if able to decrement refcount to 0, false
// otherwise
//
    bool refcount_dec_and_lock_irqsave(refcount_t *r, spinlock_t *lock,
    unsigned long *flags)
    {
    if (refcount_dec_not_one(r))
    return false;
    spin_lock_irqsave(lock, *flags);
    if (!refcount_dec_and_test(r)) {
    spin_unlock_irqrestore(lock, *flags);
    return false;
    }
    return true;
    }
    EXPORT_SYMBOL(refcount_dec_and_lock_irqsave);
