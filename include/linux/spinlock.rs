//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spinlock.h
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
// include/linux/spinlock.h - generic spinlock/rwlock declarations
//
// here's the role of the various spinlock/rwlock related include files:
//
// on SMP builds:
//
// asm/spinlock_types.h: contains the arch_spinlock_t/arch_rwlock_t and the
// initializers
//
// linux/spinlock_types_raw:
// The raw types and initializers
// linux/spinlock_types.h:
// defines the generic type and initializers
//
// asm/spinlock.h:       contains the arch_spin_*()/etc. lowlevel
// implementations, mostly inline assembly code
//
// (also included on UP-debug builds:)
//
// linux/spinlock_api_smp.h:
// contains the prototypes for the _spin_*() APIs.
//
// linux/spinlock.h:     builds the final spin_*() APIs.
//
// on UP builds:
//
// linux/spinlock_type_up.h:
// contains the generic, simplified UP spinlock type.
// (which is an empty structure on non-debug builds)
//
// linux/spinlock_types_raw:
// The raw RT types and initializers
// linux/spinlock_types.h:
// defines the generic type and initializers
//
// linux/spinlock_up.h:
// contains the arch_spin_*()/etc. version of UP
// builds. (which are NOPs on non-debug, non-preempt
// builds)
//
// (included on UP-non-debug builds:)
//
// linux/spinlock_api_up.h:
// builds the _spin_*() APIs.
//
// linux/spinlock.h:     builds the final spin_*() APIs.
//

//
// Must define these before including other files, inline functions need them
//

//
// Pull the arch_spinlock_t and arch_rwlock_t definitions:
//

//
// Pull the arch_spin*() functions/declarations (UP-nondebug doesn't need them):
//

//
// smp_mb__after_spinlock() provides the equivalent of a full memory barrier
// between program-order earlier lock acquisitions and program-order later
// memory accesses.
//
// This guarantees that the following two properties hold:
//
// 1) Given the snippet:
//
// { X = 0;  Y = 0; }
//
// CPU0				CPU1
//
// WRITE_ONCE(X, 1);		WRITE_ONCE(Y, 1);
// spin_lock(S);			smp_mb();
// smp_mb__after_spinlock();	r1 = READ_ONCE(X);
// r0 = READ_ONCE(Y);
// spin_unlock(S);
//
// it is forbidden that CPU0 does not observe CPU1's store to Y (r0 = 0)
// and CPU1 does not observe CPU0's store to X (r1 = 0); see the comments
// preceding the call to smp_mb__after_spinlock() in __schedule() and in
// try_to_wake_up().
//
// 2) Given the snippet:
//
// { X = 0;  Y = 0; }
//
// CPU0		CPU1				CPU2
//
// spin_lock(S);	spin_lock(S);			r1 = READ_ONCE(Y);
// WRITE_ONCE(X, 1);	smp_mb__after_spinlock();	smp_rmb();
// spin_unlock(S);	r0 = READ_ONCE(X);		r2 = READ_ONCE(X);
// WRITE_ONCE(Y, 1);
// spin_unlock(S);
//
// it is forbidden that CPU0's critical section executes before CPU1's
// critical section (r0 = 1), CPU2 observes CPU1's store to Y (r1 = 1)
// and CPU2 does not observe CPU0's store to X (r2 = 0); see the comments
// preceding the calls to smp_rmb() in try_to_wake_up() for similar
// snippets but "projected" onto two CPUs.
//
// Property (2) upgrades the lock to an RCsc lock.
//
// Since most load-store architectures implement ACQUIRE with an smp_mb() after
// the LL/SC loop, they need no further barriers. Similarly all our TSO
// architectures imply an smp_mb() for each atomic instruction and equally don't
// need more.
//
// Architectures that can implement ACQUIRE better need to take care.
//

extern "C" {
    pub fn do_raw_spin_lock(__acquires(lock: *mut *mut raw_spinlock_t lock));
}
extern "C" {
    pub fn do_raw_spin_trylock(__cond_acquires(true: *mut *mut raw_spinlock_t lock), _arg: lock) -> c_int;
}
extern "C" {
    pub fn do_raw_spin_unlock(__releases(lock: *mut *mut raw_spinlock_t lock));
}

//
// Define the various spin_lock methods.  Note we define these
// regardless of whether CONFIG_SMP or CONFIG_PREEMPTION are set. The
// various methods are defined as nops in the case they are not
// required.
//

//
// Always evaluate the 'subclass' argument to avoid that the compiler
// warns about set-but-not-used variables when building with
// CONFIG_DEBUG_LOCK_ALLOC=n and with W=1.
//

// Include rwlock functions for !RT

//
// Pull the _spin_*()/_read_*()/_write_*() functions/declarations:
//

// Non PREEMPT_RT kernel, map to raw spinlocks:

//
// Map the spin_lock functions to the raw variants for PREEMPT_RT=n
//

// (_lock) = __SPIN_LOCK_UNLOCKED(_lock);	\

extern "C" {
    pub fn raw_spin_trylock(_arg: &lock->rlock) -> return;
}

extern "C" {
    pub fn raw_spin_trylock_bh(_arg: &lock->rlock) -> return;
}
extern "C" {
    pub fn raw_spin_trylock_irq(_arg: &lock->rlock) -> return;
}
extern "C" {
    pub fn raw_spin_trylock_irqsave(_arg: spinlock_check(lock), _arg: *mut flags) -> return;
}

extern "C" {
    pub fn raw_spin_trylock_irq_disable(_arg: &lock->rlock) -> return;
}
//
// spin_is_locked() - Check whether a spinlock is locked.
// @lock: Pointer to the spinlock.
//
// This function is NOT required to provide any memory ordering
// guarantees; it could be used for debugging purposes or, when
// additional synchronization is needed, accompanied with other
// constructs (memory barriers) enforcing the synchronization.
//
// Returns: 1 if @lock is locked, 0 otherwise.
//
// Note that the function only tells you that the spinlock is
// seen to be locked, not that it is locked on your CPU.
//
// Further, on CONFIG_SMP=n builds with CONFIG_DEBUG_SPINLOCK=n,
// the return value is always 0 (see include/linux/spinlock_up.h).
// Therefore you should not rely heavily on the return value.
//
extern "C" {
    pub fn raw_spin_is_locked(_arg: &lock->rlock) -> return;
}
extern "C" {
    pub fn raw_spin_is_contended(_arg: &lock->rlock) -> return;
}

//
// Does a critical section need to be broken due to another
// task waiting?: (technically does not depend on CONFIG_PREEMPTION,
// but a general need for low latency)
//
extern "C" {
    pub fn spin_is_contended(_arg: lock) -> return;
}
//
// Check if a rwlock is contended.
// Returns non-zero if there is another task waiting on the rwlock.
// Returns zero if the lock is not contended or the system / underlying
// rwlock implementation does not support contention detection.
// Technically does not depend on CONFIG_PREEMPTION, but a general need
// for low latency.
//
extern "C" {
    pub fn rwlock_is_contended(_arg: lock) -> return;
}
//
// Pull the atomic_t declaration:
// (asm-mips/atomic.h needs above definitions)
//

//
// atomic_dec_and_lock - lock on reaching reference count zero
// @atomic: the atomic counter
// @lock: the spinlock in question
//
// Decrements @atomic by 1.  If the result is 0, returns true and locks
// @lock.  Returns false for all other cases.
//
extern "C" {
    pub fn atomic_dec_and_lock(atomic: *mut core::sync::atomic::AtomicI32, __cond_acquires(true: *mut *mut spinlock_t lock), _arg: lock) -> c_int;
}

extern "C" {
    pub fn atomic_dec_and_raw_lock(atomic: *mut core::sync::atomic::AtomicI32, __cond_acquires(true: *mut *mut raw_spinlock_t lock), _arg: lock) -> c_int;
}

extern "C" {
    pub fn free_bucket_spinlocks(locks: *mut spinlock_t);
}

