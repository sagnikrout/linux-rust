//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/paravirt-spinlock.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_lock_ops {
    pub val): *mut *mut *mut void (wait)(u8 ptr, u8,
    pub cpu): *mut *mut void (kick)(int,
    pub vcpu_is_preempted: paravirt_callee_save,
    pub __no_randomize_layout: },
    pub pv_ops_lock: extern struct pv_lock_ops,

    pub val): *mut *mut extern void native_queued_spin_lock_slowpath(struct qspinlock lock, u32,
    pub __pv_init_lock_hash(void): extern void,
    pub val): *mut *mut extern void __pv_queued_spin_lock_slowpath(struct qspinlock lock, u32,
    pub lock): *mut extern void __raw_callee_save___native_queued_spin_unlock(struct qspinlock,
    pub lock): *mut extern void __raw_callee_save___pv_queued_spin_unlock(struct qspinlock,
    pub nopvspin: extern bool,
    pub native_queued_spin_lock_slowpath): DECLARE_STATIC_CALL(queued_spin_lock_slowpath,,
    pub __raw_callee_save___native_queued_spin_unlock): DECLARE_STATIC_CALL(queued_spin_unlock,,
    pub val): static_call_mod(queued_spin_lock_slowpath)(lock,,
    pub "cc"): : "memory",,

//
// queued_spin_unlock - release a queued spinlock
// @lock : Pointer to queued spinlock structure
//
// A smp_store_release() on the least-significant byte.
//
    pub 0): smp_store_release(&lock->locked,,
    pub val): pv_queued_spin_lock_slowpath(lock,,

    pub pv_vcpu_is_preempted(cpu): return,
    pub val): PVOP_VCALL2(pv_ops_lock, wait, ptr,,
    pub cpu): PVOP_VCALL1(pv_ops_lock, kick,,
    pub lock): *mut void __raw_callee_save___native_queued_spin_unlock(struct qspinlock,
    pub cpu): bool __raw_callee_save___native_vcpu_is_preempted(long,

    pub native_pv_lock_init(void): void __init,
    pub lock): *mut __visible void __native_queued_spin_unlock(struct qspinlock,
    pub lock): *mut __visible void native_queued_spin_unlock_traced(struct qspinlock,
    pub lock): *mut __visible void pv_queued_spin_unlock_traced(struct qspinlock,
    pub pv_is_native_spin_unlock(void): bool,
    pub cpu): __visible bool __native_vcpu_is_preempted(long,
    pub pv_is_native_vcpu_is_preempted(void): bool,
//
// virt_spin_lock_key - disables by default the virt_spin_lock() hijack.
//
// Native (and PV wanting native due to vCPU pinning) should keep this key
// disabled. Native does not touch the key.
//
// When in a guest then native_pv_lock_init() enables the key first and
// KVM/XEN might conditionally disable it later in the boot process again.
//
// Shortcut for the queued_spin_lock_slowpath() function that allows
// virt to hijack it.
//
// Returns:
// true - lock has been negotiated, all done;
// false - queued_spin_lock_slowpath() will do its thing.
//

    pub val: c_int,
    pub false: return,
//
// On hypervisors without PARAVIRT_SPINLOCKS support we fall
// back to a Test-and-Set spinlock, because fair locks have
// horrible lock 'holder' preemption issues.
//
    pub atomic_read(&lock->val): val =,
    pub __retry: goto,
    pub true: return,
