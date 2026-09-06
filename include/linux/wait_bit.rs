//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wait_bit.h
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
// Linux wait-bit related types and methods:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_bit_key {
    pub flags: *mut c_ulong,
    pub bit_nr: c_int,
    pub timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_bit_queue_entry {
    pub key: wait_bit_key,
    pub wq_entry: wait_queue_entry,
}

extern "C" {
    pub fn wait_bit_action_f(key: *mut wait_bit_key, mode: c_int) -> typedef int;
}
extern "C" {
    pub fn __wake_up_bit(wq_head: *mut wait_queue_head, word: *mut c_ulong, bit: c_int);
}
extern "C" {
    pub fn __wait_on_bit(wq_head: *mut wait_queue_head, wbq_entry: *mut wait_bit_queue_entry, action: *mut wait_bit_action_f, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn __wait_on_bit_lock(wq_head: *mut wait_queue_head, wbq_entry: *mut wait_bit_queue_entry, action: *mut wait_bit_action_f, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn wake_up_bit(word: *mut c_ulong, bit: c_int);
}
extern "C" {
    pub fn out_of_line_wait_on_bit(word: *mut c_ulong, _arg: c_int, action: *mut wait_bit_action_f, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn out_of_line_wait_on_bit_timeout(word: *mut c_ulong, _arg: c_int, action: *mut wait_bit_action_f, mode: c_uint, timeout: c_ulong) -> c_int;
}
extern "C" {
    pub fn out_of_line_wait_on_bit_lock(word: *mut c_ulong, _arg: c_int, action: *mut wait_bit_action_f, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn wait_bit_init() -> void __init;
}
extern "C" {
    pub fn wake_bit_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, key: *mut c_void) -> c_int;
}

extern "C" {
    pub fn bit_wait(key: *mut wait_bit_key, mode: c_int) -> c_int;
}
extern "C" {
    pub fn bit_wait_io(key: *mut wait_bit_key, mode: c_int) -> c_int;
}
extern "C" {
    pub fn bit_wait_timeout(key: *mut wait_bit_key, mode: c_int) -> c_int;
}
//
// wait_on_bit - wait for a bit to be cleared
// @word: the address containing the bit being waited on
// @bit: the bit at that address being waited on
// @mode: the task state to sleep in
//
// Wait for the given bit in an unsigned long or bitmap (see DECLARE_BITMAP())
// to be cleared.  The clearing of the bit must be signalled with
// wake_up_bit(), often as clear_and_wake_up_bit().
//
// The process will wait on a waitqueue selected by hash from a shared
// pool.  It will only be woken on a wake_up for the target bit, even
// if other processes on the same queue are waiting for other bits.
//
// Returned value will be zero if the bit was cleared in which case the
// call has ACQUIRE semantics, or %-EINTR if the process received a
// signal and the mode permitted wake up on that signal.
//
// wait_on_bit_io - wait for a bit to be cleared
// @word: the address containing the bit being waited on
// @bit: the bit at that address being waited on
// @mode: the task state to sleep in
//
// Wait for the given bit in an unsigned long or bitmap (see DECLARE_BITMAP())
// to be cleared.  The clearing of the bit must be signalled with
// wake_up_bit(), often as clear_and_wake_up_bit().
//
// This is similar to wait_on_bit(), but calls io_schedule() instead of
// schedule() for the actual waiting.
//
// Returned value will be zero if the bit was cleared in which case the
// call has ACQUIRE semantics, or %-EINTR if the process received a
// signal and the mode permitted wake up on that signal.
//
// wait_on_bit_timeout - wait for a bit to be cleared or a timeout to elapse
// @word: the address containing the bit being waited on
// @bit: the bit at that address being waited on
// @mode: the task state to sleep in
// @timeout: timeout, in jiffies
//
// Wait for the given bit in an unsigned long or bitmap (see
// DECLARE_BITMAP()) to be cleared, or for a timeout to expire.  The
// clearing of the bit must be signalled with wake_up_bit(), often as
// clear_and_wake_up_bit().
//
// This is similar to wait_on_bit(), except it also takes a timeout
// parameter.
//
// Returned value will be zero if the bit was cleared in which case the
// call has ACQUIRE semantics, or %-EINTR if the process received a
// signal and the mode permitted wake up on that signal, or %-EAGAIN if the
// timeout elapsed.
//
// wait_on_bit_action - wait for a bit to be cleared
// @word: the address containing the bit waited on
// @bit: the bit at that address being waited on
// @action: the function used to sleep, which may take special actions
// @mode: the task state to sleep in
//
// Wait for the given bit in an unsigned long or bitmap (see DECLARE_BITMAP())
// to be cleared.  The clearing of the bit must be signalled with
// wake_up_bit(), often as clear_and_wake_up_bit().
//
// This is similar to wait_on_bit(), but calls @action() instead of
// schedule() for the actual waiting.
//
// Returned value will be zero if the bit was cleared in which case the
// call has ACQUIRE semantics, or the error code returned by @action if
// that call returned non-zero.
//
extern "C" {
    pub fn out_of_line_wait_on_bit(_arg: word, _arg: bit, _arg: action, _arg: mode) -> return;
}
//
// wait_on_bit_lock - wait for a bit to be cleared, then set it
// @word: the address containing the bit being waited on
// @bit: the bit of the word being waited on and set
// @mode: the task state to sleep in
//
// Wait for the given bit in an unsigned long or bitmap (see
// DECLARE_BITMAP()) to be cleared.  The clearing of the bit must be
// signalled with wake_up_bit(), often as clear_and_wake_up_bit().  As
// soon as it is clear, atomically set it and return.
//
// This is similar to wait_on_bit(), but sets the bit before returning.
//
// Returned value will be zero if the bit was successfully set in which
// case the call has the same memory sequencing semantics as
// test_and_clear_bit(), or %-EINTR if the process received a signal and
// the mode permitted wake up on that signal.
//
extern "C" {
    pub fn out_of_line_wait_on_bit_lock(_arg: word, _arg: bit, _arg: bit_wait, _arg: mode) -> return;
}
//
// wait_on_bit_lock_io - wait for a bit to be cleared, then set it
// @word: the address containing the bit being waited on
// @bit: the bit of the word being waited on and set
// @mode: the task state to sleep in
//
// Wait for the given bit in an unsigned long or bitmap (see
// DECLARE_BITMAP()) to be cleared.  The clearing of the bit must be
// signalled with wake_up_bit(), often as clear_and_wake_up_bit().  As
// soon as it is clear, atomically set it and return.
//
// This is similar to wait_on_bit_lock(), but calls io_schedule() instead
// of schedule().
//
// Returns zero if the bit was (eventually) found to be clear and was
// set.  Returns non-zero if a signal was delivered to the process and
// the @mode allows that signal to wake the process.
//
extern "C" {
    pub fn out_of_line_wait_on_bit_lock(_arg: word, _arg: bit, _arg: bit_wait_io, _arg: mode) -> return;
}
//
// wait_on_bit_lock_action - wait for a bit to be cleared, then set it
// @word: the address containing the bit being waited on
// @bit: the bit of the word being waited on and set
// @action: the function used to sleep, which may take special actions
// @mode: the task state to sleep in
//
// This is similar to wait_on_bit_lock(), but calls @action() instead of
// schedule() for the actual waiting.
//
// Returned value will be zero if the bit was successfully set in which
// case the call has the same memory sequencing semantics as
// test_and_clear_bit(), or the error code returned by @action if that
// call returned non-zero.
//
extern "C" {
    pub fn out_of_line_wait_on_bit_lock(_arg: word, _arg: bit, _arg: action, _arg: mode) -> return;
}
extern "C" {
    pub fn init_wait_var_entry(wbq_entry: *mut wait_bit_queue_entry, var: *mut c_void, flags: c_int);
}
extern "C" {
    pub fn wake_up_var(var: *mut c_void);
}

//
// wait_var_event - wait for a variable to be updated and notified
// @var: the address of variable being waited on
// @condition: the condition to wait for
//
// Wait for a @condition to be true, only re-checking when a wake up is
// received for the given @var (an arbitrary kernel address which need
// not be directly related to the given condition, but usually is).
//
// The process will wait on a waitqueue selected by hash from a shared
// pool.  It will only be woken on a wake_up for the given address.
//
// The condition should normally use smp_load_acquire() or a similarly
// ordered access to ensure that any changes to memory made before the
// condition became true will be visible after the wait completes.
//

//
// wait_var_event_io - wait for a variable to be updated and notified
// @var: the address of variable being waited on
// @condition: the condition to wait for
//
// Wait for an IO related @condition to be true, only re-checking when a
// wake up is received for the given @var (an arbitrary kernel address
// which need not be directly related to the given condition, but
// usually is).
//
// The process will wait on a waitqueue selected by hash from a shared
// pool.  It will only be woken on a wake_up for the given address.
//
// This is similar to wait_var_event(), but calls io_schedule() instead
// of schedule().
//
// The condition should normally use smp_load_acquire() or a similarly
// ordered access to ensure that any changes to memory made before the
// condition became true will be visible after the wait completes.
//

//
// wait_var_event_killable - wait for a variable to be updated and notified
// @var: the address of variable being waited on
// @condition: the condition to wait for
//
// Wait for a @condition to be true or a fatal signal to be received,
// only re-checking the condition when a wake up is received for the given
// @var (an arbitrary kernel address which need not be directly related
// to the given condition, but usually is).
//
// This is similar to wait_var_event() but returns a value which is
// 0 if the condition became true, or %-ERESTARTSYS if a fatal signal
// was received.
//
// The condition should normally use smp_load_acquire() or a similarly
// ordered access to ensure that any changes to memory made before the
// condition became true will be visible after the wait completes.
//

//
// wait_var_event_timeout - wait for a variable to be updated or a timeout to expire
// @var: the address of variable being waited on
// @condition: the condition to wait for
// @timeout: maximum time to wait in jiffies
//
// Wait for a @condition to be true or a timeout to expire, only
// re-checking the condition when a wake up is received for the given
// @var (an arbitrary kernel address which need not be directly related
// to the given condition, but usually is).
//
// This is similar to wait_var_event() but returns a value which is 0 if
// the timeout expired and the condition was still false, or the
// remaining time left in the timeout (but at least 1) if the condition
// was found to be true.
//
// The condition should normally use smp_load_acquire() or a similarly
// ordered access to ensure that any changes to memory made before the
// condition became true will be visible after the wait completes.
//

//
// wait_var_event_interruptible - wait for a variable to be updated and notified
// @var: the address of variable being waited on
// @condition: the condition to wait for
//
// Wait for a @condition to be true or a signal to be received, only
// re-checking the condition when a wake up is received for the given
// @var (an arbitrary kernel address which need not be directly related
// to the given condition, but usually is).
//
// This is similar to wait_var_event() but returns a value which is 0 if
// the condition became true, or %-ERESTARTSYS if a signal was received.
//
// The condition should normally use smp_load_acquire() or a similarly
// ordered access to ensure that any changes to memory made before the
// condition became true will be visible after the wait completes.
//

//
// wait_var_event_any_lock - wait for a variable to be updated under a lock
// @var: the address of the variable being waited on
// @condition: condition to wait for
// @lock: the object that is locked to protect updates to the variable
// @type: prefix on lock and unlock operations
// @state: waiting state, %TASK_UNINTERRUPTIBLE etc.
//
// Wait for a condition which can only be reliably tested while holding
// a lock.  The variables assessed in the condition will normal be updated
// under the same lock, and the wake up should be signalled with
// wake_up_var_locked() under the same lock.
//
// This is similar to wait_var_event(), but assumes a lock is held
// while calling this function and while updating the variable.
//
// This must be called while the given lock is held and the lock will be
// dropped when schedule() is called to wait for a wake up, and will be
// reclaimed before testing the condition again.  The functions used to
// unlock and lock the object are constructed by appending _unlock and _lock
// to @type.
//
// Return %-ERESTARTSYS if a signal arrives which is allowed to interrupt
// the wait according to @state.
//

//
// wait_var_event_spinlock - wait for a variable to be updated under a spinlock
// @var: the address of the variable being waited on
// @condition: condition to wait for
// @lock: the spinlock which protects updates to the variable
//
// Wait for a condition which can only be reliably tested while holding
// a spinlock.  The variables assessed in the condition will normal be updated
// under the same spinlock, and the wake up should be signalled with
// wake_up_var_locked() under the same spinlock.
//
// This is similar to wait_var_event(), but assumes a spinlock is held
// while calling this function and while updating the variable.
//
// This must be called while the given lock is held and the lock will be
// dropped when schedule() is called to wait for a wake up, and will be
// reclaimed before testing the condition again.
//

//
// wait_var_event_mutex - wait for a variable to be updated under a mutex
// @var: the address of the variable being waited on
// @condition: condition to wait for
// @lock: the mutex which protects updates to the variable
//
// Wait for a condition which can only be reliably tested while holding
// a mutex.  The variables assessed in the condition will normal be
// updated under the same mutex, and the wake up should be signalled
// with wake_up_var_locked() under the same mutex.
//
// This is similar to wait_var_event(), but assumes a mutex is held
// while calling this function and while updating the variable.
//
// This must be called while the given mutex is held and the mutex will be
// dropped when schedule() is called to wait for a wake up, and will be
// reclaimed before testing the condition again.
//

//
// wake_up_var_protected - wake up waiters for a variable asserting that it is safe
// @var: the address of the variable being waited on
// @cond: the condition which afirms this is safe
//
// When waking waiters which use wait_var_event_any_lock() the waker must be
// holding the reelvant lock to avoid races.  This version of wake_up_var()
// asserts that the relevant lock is held and so no barrier is needed.
// The @cond is only tested when CONFIG_LOCKDEP is enabled.
//

//
// wake_up_var_locked - wake up waiters for a variable while holding a spinlock or mutex
// @var: the address of the variable being waited on
// @lock: The spinlock or mutex what protects the variable
//
// Send a wake up for the given variable which should be waited for with
// wait_var_event_spinlock() or wait_var_event_mutex().  Unlike wake_up_var(),
// no extra barriers are needed as the locking provides sufficient sequencing.
//

//
// clear_and_wake_up_bit - clear a bit and wake up anyone waiting on that bit
// @bit: the bit of the word being waited on
// @word: the address containing the bit being waited on
//
// The designated bit is cleared and any tasks waiting in wait_on_bit()
// or similar will be woken.  This call has RELEASE semantics so that
// any changes to memory made before this call are guaranteed to be visible
// after the corresponding wait_on_bit() completes.
//
// See wake_up_bit() for which memory barrier you need to use.
//
// test_and_clear_wake_up_bit - clear a bit if it was set: wake up anyone waiting on that bit
// @bit: the bit of the word being waited on
// @word: the address of memory containing that bit
//
// If the bit is set and can be atomically cleared, any tasks waiting in
// wait_on_bit() or similar will be woken.  This call has the same
// complete ordering semantics as test_and_clear_bit().  Any changes to
// memory made before this call are guaranteed to be visible after the
// corresponding wait_on_bit() completes.
//
// Returns %true if the bit was successfully set and the wake up was sent.
//
// no extra barrier required
//
// atomic_dec_and_wake_up - decrement an atomic_t and if zero, wake up waiters
// @var: the variable to dec and test
//
// Decrements the atomic variable and if it reaches zero, send a wake_up to any
// processes waiting on the variable.
//
// This function has the same complete ordering semantics as atomic_dec_and_test.
//
// Returns %true is the variable reaches zero and the wake up was sent.
//
// No extra barrier required
//
// store_release_wake_up - update a variable and send a wake_up
// @var: the address of the variable to be updated and woken
// @val: the value to store in the variable.
//
// Store the given value in the variable send a wake up to any tasks
// waiting on the variable.  All necessary barriers are included to ensure
// the task calling wait_var_event() sees the new value and all values
// written to memory before this call.
//

