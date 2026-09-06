//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/lib/locks.c
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
// Spin and read/write lock operations.
//
// Copyright (C) 2001-2004 Paul Mackerras <paulus@au.ibm.com>, IBM
// Copyright (C) 2001 Anton Blanchard <anton@au.ibm.com>, IBM
// Copyright (C) 2002 Dave Engebretsen <engebret@us.ibm.com>, IBM
// Rework to support virtual processors
//

// waiting for a spinlock...

#[no_mangle]
pub unsafe extern "C" fn splpar_spin_yield(lock: *mut arch_spinlock_t) {
    void splpar_spin_yield(arch_spinlock_t *lock)
    {
    unsigned int lock_value, holder_cpu, yield_count;
    lock_value = lock.slock;
    if (lock_value == 0)
    return;
    holder_cpu = lock_value & 0xffff;
    BUG_ON(holder_cpu >= NR_CPUS);
    yield_count = yield_count_of(holder_cpu);
    if ((yield_count & 1) == 0)
    return;		/* virtual cpu is currently running */
    rmb();
    if (lock.slock != lock_value)
    return;		/* something has changed */
    yield_to_preempted(holder_cpu, yield_count);
    }
    EXPORT_SYMBOL_GPL(splpar_spin_yield);
//
// Waiting for a read lock or a write lock on a rwlock...
// This turns out to be the same for read and write locks, since
// we only know the holder if it is write-locked.
//
#[no_mangle]
pub unsafe extern "C" fn splpar_rw_yield(rw: *mut arch_rwlock_t) {
    void splpar_rw_yield(arch_rwlock_t *rw)
    {
    int lock_value;
    unsigned int holder_cpu, yield_count;
    lock_value = rw.lock;
    if (lock_value >= 0)
    return;		/* no write lock at present */
    holder_cpu = lock_value & 0xffff;
    BUG_ON(holder_cpu >= NR_CPUS);
    yield_count = yield_count_of(holder_cpu);
    if ((yield_count & 1) == 0)
    return;		/* virtual cpu is currently running */
    rmb();
    if (rw.lock != lock_value)
    return;		/* something has changed */
    yield_to_preempted(holder_cpu, yield_count);
    }
