//! Automatically rewritten from C to Rust
//! Source: lib/atomic64.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Generic implementation of 64-bit atomics using spinlocks,
// useful on processors that don't have 64-bit atomic instructions.
//
// Copyright © 2009 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
//

//
// We use a hashed array of spinlocks to provide exclusive access
// to each atomic64_t variable.  Since this is expected to used on
// systems with small numbers of CPUs (<= 4 or so), we use a
// relatively small array of 16 spinlocks to avoid wasting too much
// memory on the spinlock array.
//
pub const NR_LOCKS: c_int = 16;
//
// Ensure each lock is in a separate cacheline.
//
    static union {
    arch_spinlock_t lock;
    char pad[L1_CACHE_BYTES];
    } atomic64_lock[NR_LOCKS] __cacheline_aligned_in_smp = {
    [0 ... (NR_LOCKS - 1)] = {
    .lock =  __ARCH_SPIN_LOCK_UNLOCKED,
    },
    };
    static inline arch_spinlock_t *lock_addr(const atomic64_t *v)
    {
    let mut addr: c_ulong = (unsigned long) v;
    addr >>= L1_CACHE_SHIFT;
    addr ^= (addr >> 8) ^ (addr >> 16);
    return &atomic64_lock[addr & (NR_LOCKS - 1)].lock;
    }
#[no_mangle]
pub unsafe extern "C" fn generic_atomic64_read(v: *const core::sync::atomic::AtomicI64) -> i64 {
    s64 generic_atomic64_read(const atomic64_t *v)
    {
    unsigned long flags;
    arch_spinlock_t *lock = lock_addr(v);
    s64 val;
    local_irq_save(flags);
    arch_spin_lock(lock);
    val = v.counter;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    return val;
    }
    EXPORT_SYMBOL(generic_atomic64_read);
#[no_mangle]
pub unsafe extern "C" fn generic_atomic64_set(v: *mut core::sync::atomic::AtomicI64, i: i64) {
    void generic_atomic64_set(atomic64_t *v, s64 i)
    {
    unsigned long flags;
    arch_spinlock_t *lock = lock_addr(v);
    local_irq_save(flags);
    arch_spin_lock(lock);
    v.counter = i;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(generic_atomic64_set);

    void generic_atomic64_##op(s64 a, atomic64_t *v)			\
    {									\
    unsigned long flags;						\
    arch_spinlock_t *lock = lock_addr(v);				\
    \
    local_irq_save(flags);						\
    arch_spin_lock(lock);						\
    v.counter c_op a;						\
    arch_spin_unlock(lock);						\
    local_irq_restore(flags);					\
    }									\
    EXPORT_SYMBOL(generic_atomic64_##op);

    s64 generic_atomic64_##op##_return(s64 a, atomic64_t *v)		\
    {									\
    unsigned long flags;						\
    arch_spinlock_t *lock = lock_addr(v);				\
    s64 val;							\
    \
    local_irq_save(flags);						\
    arch_spin_lock(lock);						\
    val = (v.counter c_op a);					\
    arch_spin_unlock(lock);						\
    local_irq_restore(flags);					\
    return val;							\
    }									\
    EXPORT_SYMBOL(generic_atomic64_##op##_return);

    s64 generic_atomic64_fetch_##op(s64 a, atomic64_t *v)			\
    {									\
    unsigned long flags;						\
    arch_spinlock_t *lock = lock_addr(v);				\
    s64 val;							\
    \
    local_irq_save(flags);						\
    arch_spin_lock(lock);						\
    val = v.counter;						\
    v.counter c_op a;						\
    arch_spin_unlock(lock);						\
    local_irq_restore(flags);					\
    return val;							\
    }									\
    EXPORT_SYMBOL(generic_atomic64_fetch_##op);

    ATOMIC64_OP(op, c_op)						\
    ATOMIC64_OP_RETURN(op, c_op)					\
    ATOMIC64_FETCH_OP(op, c_op)
    ATOMIC64_OPS(add, +=)
    ATOMIC64_OPS(sub, -=)

    ATOMIC64_OP(op, c_op)						\
    ATOMIC64_FETCH_OP(op, c_op)
    ATOMIC64_OPS(and, &=)
    ATOMIC64_OPS(or, |=)
    ATOMIC64_OPS(xor, ^=)

#[no_mangle]
pub unsafe extern "C" fn generic_atomic64_dec_if_positive(v: *mut core::sync::atomic::AtomicI64) -> i64 {
    s64 generic_atomic64_dec_if_positive(atomic64_t *v)
    {
    unsigned long flags;
    arch_spinlock_t *lock = lock_addr(v);
    s64 val;
    local_irq_save(flags);
    arch_spin_lock(lock);
    val = v.counter - 1;
    if (val >= 0)
    v.counter = val;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    return val;
    }
    EXPORT_SYMBOL(generic_atomic64_dec_if_positive);
#[no_mangle]
pub unsafe extern "C" fn generic_atomic64_cmpxchg(v: *mut core::sync::atomic::AtomicI64, o: i64, n: i64) -> i64 {
    s64 generic_atomic64_cmpxchg(atomic64_t *v, s64 o, s64 n)
    {
    unsigned long flags;
    arch_spinlock_t *lock = lock_addr(v);
    s64 val;
    local_irq_save(flags);
    arch_spin_lock(lock);
    val = v.counter;
    if (val == o)
    v.counter = n;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    return val;
    }
    EXPORT_SYMBOL(generic_atomic64_cmpxchg);
#[no_mangle]
pub unsafe extern "C" fn generic_atomic64_xchg(v: *mut core::sync::atomic::AtomicI64, new: i64) -> i64 {
    s64 generic_atomic64_xchg(atomic64_t *v, s64 new)
    {
    unsigned long flags;
    arch_spinlock_t *lock = lock_addr(v);
    s64 val;
    local_irq_save(flags);
    arch_spin_lock(lock);
    val = v.counter;
    v.counter = new;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    return val;
    }
    EXPORT_SYMBOL(generic_atomic64_xchg);
#[no_mangle]
pub unsafe extern "C" fn generic_atomic64_fetch_add_unless(v: *mut core::sync::atomic::AtomicI64, a: i64, u: i64) -> i64 {
    s64 generic_atomic64_fetch_add_unless(atomic64_t *v, s64 a, s64 u)
    {
    unsigned long flags;
    arch_spinlock_t *lock = lock_addr(v);
    s64 val;
    local_irq_save(flags);
    arch_spin_lock(lock);
    val = v.counter;
    if (val != u)
    v.counter += a;
    arch_spin_unlock(lock);
    local_irq_restore(flags);
    return val;
    }
    EXPORT_SYMBOL(generic_atomic64_fetch_add_unless);
