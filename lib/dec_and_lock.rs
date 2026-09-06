//! Automatically rewritten from C to Rust
//! Source: lib/dec_and_lock.c
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
// This is an implementation of the notion of "decrement a
// reference count, and return locked if it decremented to zero".
//
// NOTE NOTE NOTE! This is _not_ equivalent to
//
// if (atomic_dec_and_test(&atomic)) {
// spin_lock(&lock);
// return 1;
// }
// return 0;
//
// because the spin-lock and the decrement must be
// "atomic".
//
#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_lock(atomic: *mut core::sync::atomic::AtomicI32, lock: *mut spinlock_t) -> c_int {
    int atomic_dec_and_lock(atomic_t *atomic, spinlock_t *lock)
    {
// Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if (atomic_add_unless(atomic, -1, 1))
    return 0;
// Otherwise do it the slow way
    spin_lock(lock);
    if (atomic_dec_and_test(atomic))
    return 1;
    spin_unlock(lock);
    return 0;
    }
    EXPORT_SYMBOL(atomic_dec_and_lock);
    int _atomic_dec_and_lock_irqsave(atomic_t *atomic, spinlock_t *lock,
    unsigned long *flags)
    {
// Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if (atomic_add_unless(atomic, -1, 1))
    return 0;
// Otherwise do it the slow way
    spin_lock_irqsave(lock, *flags);
    if (atomic_dec_and_test(atomic))
    return 1;
    spin_unlock_irqrestore(lock, *flags);
    return 0;
    }
    EXPORT_SYMBOL(_atomic_dec_and_lock_irqsave);
#[no_mangle]
pub unsafe extern "C" fn atomic_dec_and_raw_lock(atomic: *mut core::sync::atomic::AtomicI32, lock: *mut raw_spinlock_t) -> c_int {
    int atomic_dec_and_raw_lock(atomic_t *atomic, raw_spinlock_t *lock)
    {
// Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if (atomic_add_unless(atomic, -1, 1))
    return 0;
// Otherwise do it the slow way
    raw_spin_lock(lock);
    if (atomic_dec_and_test(atomic))
    return 1;
    raw_spin_unlock(lock);
    return 0;
    }
    EXPORT_SYMBOL(atomic_dec_and_raw_lock);
    int _atomic_dec_and_raw_lock_irqsave(atomic_t *atomic, raw_spinlock_t *lock,
    unsigned long *flags)
    {
// Subtract 1 from counter unless that drops it to 0 (ie. it was 1)
    if (atomic_add_unless(atomic, -1, 1))
    return 0;
// Otherwise do it the slow way
    raw_spin_lock_irqsave(lock, *flags);
    if (atomic_dec_and_test(atomic))
    return 1;
    raw_spin_unlock_irqrestore(lock, *flags);
    return 0;
    }
    EXPORT_SYMBOL(_atomic_dec_and_raw_lock_irqsave);
