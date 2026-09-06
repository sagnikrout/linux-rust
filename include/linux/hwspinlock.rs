//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hwspinlock.h
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
// Hardware spinlock public header
//
// Copyright (C) 2010 Texas Instruments Incorporated - http://www.ti.com
//
// Contact: Ohad Ben-Cohen <ohad@wizery.com>
//

// hwspinlock mode argument
pub const HWLOCK_IRQSTATE: c_uint = 0x01 /* Disable interrupts, save state */;
pub const HWLOCK_IRQ: c_uint = 0x02 /* Disable interrupts, don't save state */;
pub const HWLOCK_RAW: c_uint = 0x03;
pub const HWLOCK_IN_ATOMIC: c_uint = 0x04 /* Called while in atomic context */;

extern "C" {
    pub fn hwspin_lock_unregister(bank: *mut hwspinlock_device) -> c_int;
}
extern "C" {
    pub fn hwspin_lock_free(hwlock: *mut hwspinlock) -> c_int;
}
extern "C" {
    pub fn of_hwspin_lock_get_id(np: *mut device_node, index: c_int) -> c_int;
}
extern "C" {
    pub fn __hwspin_trylock(: *mut hwspinlock, _arg: c_int, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn __hwspin_unlock(: *mut hwspinlock, _arg: c_int, : *mut c_ulong);
}
extern "C" {
    pub fn of_hwspin_lock_get_id_byname(np: *mut device_node, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn hwspin_lock_bust(hwlock: *mut hwspinlock, id: c_uint) -> c_int;
}
extern "C" {
    pub fn devm_hwspin_lock_free(dev: *mut device, hwlock: *mut hwspinlock) -> c_int;
}

//
// We don't want these functions to fail if CONFIG_HWSPINLOCK is not
// enabled. We prefer to silently succeed in this case, and let the
// code path get compiled away. This way, if CONFIG_HWSPINLOCK is not
// required on a given setup, users will still work.
//
// The only exception is hwspin_lock_register/hwspin_lock_unregister, with which
// we _do_ want users to fail (no point in registering hwspinlock instances if
// the framework is not available).
//
// Note: ERR_PTR(-ENODEV) will still be considered a success for NULL-checking
// users. Others, which care, can still check this with IS_ERR.
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

//
// hwspin_trylock_irqsave() - try to lock an hwspinlock, disable interrupts
// @hwlock: an hwspinlock which we want to trylock
// @flags: a pointer to where the caller's interrupt state will be saved at
//
// This function attempts to lock the underlying hwspinlock, and will
// immediately fail if the hwspinlock is already locked.
//
// Upon a successful return from this function, preemption and local
// interrupts are disabled (previous interrupts state is saved at @flags),
// so the caller must not sleep, and is advised to release the hwspinlock
// as soon as possible.
//
// Returns 0 if we successfully locked the hwspinlock, -EBUSY if
// the hwspinlock was already taken, and -EINVAL if @hwlock is invalid.
//
extern "C" {
    pub fn __hwspin_trylock(_arg: hwlock, _arg: HWLOCK_IRQSTATE, _arg: flags) -> return;
}
//
// hwspin_trylock_irq() - try to lock an hwspinlock, disable interrupts
// @hwlock: an hwspinlock which we want to trylock
//
// This function attempts to lock the underlying hwspinlock, and will
// immediately fail if the hwspinlock is already locked.
//
// Upon a successful return from this function, preemption and local
// interrupts are disabled, so the caller must not sleep, and is advised
// to release the hwspinlock as soon as possible.
//
// Returns 0 if we successfully locked the hwspinlock, -EBUSY if
// the hwspinlock was already taken, and -EINVAL if @hwlock is invalid.
//
extern "C" {
    pub fn __hwspin_trylock(_arg: hwlock, _arg: HWLOCK_IRQ, _arg: NULL) -> return;
}
//
// hwspin_trylock_raw() - attempt to lock a specific hwspinlock
// @hwlock: an hwspinlock which we want to trylock
//
// This function attempts to lock an hwspinlock, and will immediately fail
// if the hwspinlock is already taken.
//
// Caution: User must protect the routine of getting hardware lock with mutex
// or spinlock to avoid dead-lock, that will let user can do some time-consuming
// or sleepable operations under the hardware lock.
//
// Returns 0 if we successfully locked the hwspinlock, -EBUSY if
// the hwspinlock was already taken, and -EINVAL if @hwlock is invalid.
//
extern "C" {
    pub fn __hwspin_trylock(_arg: hwlock, _arg: HWLOCK_RAW, _arg: NULL) -> return;
}
//
// hwspin_trylock_in_atomic() - attempt to lock a specific hwspinlock
// @hwlock: an hwspinlock which we want to trylock
//
// This function attempts to lock an hwspinlock, and will immediately fail
// if the hwspinlock is already taken.
//
// This function shall be called only from an atomic context.
//
// Returns 0 if we successfully locked the hwspinlock, -EBUSY if
// the hwspinlock was already taken, and -EINVAL if @hwlock is invalid.
//
extern "C" {
    pub fn __hwspin_trylock(_arg: hwlock, _arg: HWLOCK_IN_ATOMIC, _arg: NULL) -> return;
}
//
// hwspin_trylock() - attempt to lock a specific hwspinlock
// @hwlock: an hwspinlock which we want to trylock
//
// This function attempts to lock an hwspinlock, and will immediately fail
// if the hwspinlock is already taken.
//
// Upon a successful return from this function, preemption is disabled,
// so the caller must not sleep, and is advised to release the hwspinlock
// as soon as possible. This is required in order to minimize remote cores
// polling on the hardware interconnect.
//
// Returns 0 if we successfully locked the hwspinlock, -EBUSY if
// the hwspinlock was already taken, and -EINVAL if @hwlock is invalid.
//
extern "C" {
    pub fn __hwspin_trylock(_arg: hwlock, _arg: 0, _arg: NULL) -> return;
}
//
// hwspin_lock_timeout_irqsave() - lock hwspinlock, with timeout, disable irqs
// @hwlock: the hwspinlock to be locked
// @to: timeout value in msecs
// @flags: a pointer to where the caller's interrupt state will be saved at
//
// This function locks the underlying @hwlock. If the @hwlock
// is already taken, the function will busy loop waiting for it to
// be released, but give up when @timeout msecs have elapsed.
//
// Upon a successful return from this function, preemption and local interrupts
// are disabled (plus previous interrupt state is saved), so the caller must
// not sleep, and is advised to release the hwspinlock as soon as possible.
//
// Returns 0 when the @hwlock was successfully taken, and an appropriate
// error code otherwise (most notably an -ETIMEDOUT if the @hwlock is still
// busy after @timeout msecs). The function will never sleep.
//
extern "C" {
    pub fn __hwspin_lock_timeout(_arg: hwlock, _arg: to, _arg: HWLOCK_IRQSTATE, _arg: flags) -> return;
}
//
// hwspin_lock_timeout_irq() - lock hwspinlock, with timeout, disable irqs
// @hwlock: the hwspinlock to be locked
// @to: timeout value in msecs
//
// This function locks the underlying @hwlock. If the @hwlock
// is already taken, the function will busy loop waiting for it to
// be released, but give up when @timeout msecs have elapsed.
//
// Upon a successful return from this function, preemption and local interrupts
// are disabled so the caller must not sleep, and is advised to release the
// hwspinlock as soon as possible.
//
// Returns 0 when the @hwlock was successfully taken, and an appropriate
// error code otherwise (most notably an -ETIMEDOUT if the @hwlock is still
// busy after @timeout msecs). The function will never sleep.
//
extern "C" {
    pub fn __hwspin_lock_timeout(_arg: hwlock, _arg: to, _arg: HWLOCK_IRQ, _arg: NULL) -> return;
}
//
// hwspin_lock_timeout_raw() - lock an hwspinlock with timeout limit
// @hwlock: the hwspinlock to be locked
// @to: timeout value in msecs
//
// This function locks the underlying @hwlock. If the @hwlock
// is already taken, the function will busy loop waiting for it to
// be released, but give up when @timeout msecs have elapsed.
//
// Caution: User must protect the routine of getting hardware lock with mutex
// or spinlock to avoid dead-lock, that will let user can do some time-consuming
// or sleepable operations under the hardware lock.
//
// Returns 0 when the @hwlock was successfully taken, and an appropriate
// error code otherwise (most notably an -ETIMEDOUT if the @hwlock is still
// busy after @timeout msecs). The function will never sleep.
//
extern "C" {
    pub fn __hwspin_lock_timeout(_arg: hwlock, _arg: to, _arg: HWLOCK_RAW, _arg: NULL) -> return;
}
//
// hwspin_lock_timeout_in_atomic() - lock an hwspinlock with timeout limit
// @hwlock: the hwspinlock to be locked
// @to: timeout value in msecs
//
// This function locks the underlying @hwlock. If the @hwlock
// is already taken, the function will busy loop waiting for it to
// be released, but give up when @timeout msecs have elapsed.
//
// This function shall be called only from an atomic context and the timeout
// value shall not exceed a few msecs.
//
// Returns 0 when the @hwlock was successfully taken, and an appropriate
// error code otherwise (most notably an -ETIMEDOUT if the @hwlock is still
// busy after @timeout msecs). The function will never sleep.
//
extern "C" {
    pub fn __hwspin_lock_timeout(_arg: hwlock, _arg: to, _arg: HWLOCK_IN_ATOMIC, _arg: NULL) -> return;
}
//
// hwspin_lock_timeout() - lock an hwspinlock with timeout limit
// @hwlock: the hwspinlock to be locked
// @to: timeout value in msecs
//
// This function locks the underlying @hwlock. If the @hwlock
// is already taken, the function will busy loop waiting for it to
// be released, but give up when @timeout msecs have elapsed.
//
// Upon a successful return from this function, preemption is disabled
// so the caller must not sleep, and is advised to release the hwspinlock
// as soon as possible.
// This is required in order to minimize remote cores polling on the
// hardware interconnect.
//
// Returns 0 when the @hwlock was successfully taken, and an appropriate
// error code otherwise (most notably an -ETIMEDOUT if the @hwlock is still
// busy after @timeout msecs). The function will never sleep.
//
extern "C" {
    pub fn __hwspin_lock_timeout(_arg: hwlock, _arg: to, _arg: 0, _arg: NULL) -> return;
}
//
// hwspin_unlock_irqrestore() - unlock hwspinlock, restore irq state
// @hwlock: a previously-acquired hwspinlock which we want to unlock
// @flags: previous caller's interrupt state to restore
//
// This function will unlock a specific hwspinlock, enable preemption and
// restore the previous state of the local interrupts. It should be used
// to undo, e.g., hwspin_trylock_irqsave().
//
// @hwlock must be already locked before calling this function: it is a bug
// to call unlock on a @hwlock that is already unlocked.
//
// hwspin_unlock_irq() - unlock hwspinlock, enable interrupts
// @hwlock: a previously-acquired hwspinlock which we want to unlock
//
// This function will unlock a specific hwspinlock, enable preemption and
// enable local interrupts. Should be used to undo hwspin_lock_irq().
//
// @hwlock must be already locked (e.g. by hwspin_trylock_irq()) before
// calling this function: it is a bug to call unlock on a @hwlock that is
// already unlocked.
//
// hwspin_unlock_raw() - unlock hwspinlock
// @hwlock: a previously-acquired hwspinlock which we want to unlock
//
// This function will unlock a specific hwspinlock.
//
// @hwlock must be already locked (e.g. by hwspin_trylock()) before calling
// this function: it is a bug to call unlock on a @hwlock that is already
// unlocked.
//
// hwspin_unlock_in_atomic() - unlock hwspinlock
// @hwlock: a previously-acquired hwspinlock which we want to unlock
//
// This function will unlock a specific hwspinlock.
//
// @hwlock must be already locked (e.g. by hwspin_trylock()) before calling
// this function: it is a bug to call unlock on a @hwlock that is already
// unlocked.
//
// hwspin_unlock() - unlock hwspinlock
// @hwlock: a previously-acquired hwspinlock which we want to unlock
//
// This function will unlock a specific hwspinlock and enable preemption
// back.
//
// @hwlock must be already locked (e.g. by hwspin_trylock()) before calling
// this function: it is a bug to call unlock on a @hwlock that is already
// unlocked.
//
