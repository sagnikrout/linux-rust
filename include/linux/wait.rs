//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wait.h
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
// Linux wait queue related types and methods
//

pub type wait_queue_entry_t = wait_queue_entry;
extern "C" {
    pub fn int(wq_entry: *mut *mut wait_queue_func_t)(struct wait_queue_entry, mode: unsigned, flags: c_int, key: *mut c_void) -> typedef;
}
extern "C" {
    pub fn default_wake_function(wq_entry: *mut wait_queue_entry, mode: unsigned, flags: c_int, key: *mut c_void) -> c_int;
}
// wait_queue_entry::flags
pub const WQ_FLAG_EXCLUSIVE: c_uint = 0x01;
pub const WQ_FLAG_WOKEN: c_uint = 0x02;
pub const WQ_FLAG_CUSTOM: c_uint = 0x04;
pub const WQ_FLAG_DONE: c_uint = 0x08;
pub const WQ_FLAG_PRIORITY: c_uint = 0x10;
//
// A single wait-queue entry structure:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_queue_entry {
    pub flags: c_uint,
    pub private: *mut c_void,
    pub func: wait_queue_func_t,
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_queue_head {
    pub lock: spinlock_t,
    pub head: list_head,
}

pub type wait_queue_head_t = wait_queue_head;
//
// Macros for declaration and initialisaton of the datatypes
//

extern "C" {
    pub fn __init_waitqueue_head(wq_head: *mut wait_queue_head, name: *const c_char, : *mut lock_class_key);
}

//
// waitqueue_active -- locklessly test for waiters on the queue
// @wq_head: the waitqueue to test for waiters
//
// returns true if the wait list is not empty
//
// NOTE: this function is lockless and requires care, incorrect usage _will_
// lead to sporadic and non-obvious failure.
//
// Use either while holding wait_queue_head::lock or when used for wakeups
// with an extra smp_mb() like::
//
// CPU0 - waker                    CPU1 - waiter
//
// for (;;) {
// @cond = true;                     prepare_to_wait(&wq_head, &wait, state);
// smp_mb();                         // smp_mb() from set_current_state()
// if (waitqueue_active(wq_head))         if (@cond)
// wake_up(wq_head);                      break;
// schedule();
// }
// finish_wait(&wq_head, &wait);
//
// Because without the explicit smp_mb() it's possible for the
// waitqueue_active() load to get hoisted over the @cond store such that we'll
// observe an empty wait list while the waiter might not observe @cond.
//
// Also note that this 'optimization' trades a spin_lock() for an smp_mb(),
// which (when the lock is uncontended) are of roughly equal cost.
//
// wq_has_single_sleeper - check if there is only one sleeper
// @wq_head: wait queue head
//
// Returns true of wq_head has only one sleeper on the list.
//
// Please refer to the comment for waitqueue_active.
//
extern "C" {
    pub fn list_is_singular(_arg: &wq_head->head) -> return;
}
//
// wq_has_sleeper - check if there are any waiting processes
// @wq_head: wait queue head
//
// Returns true if wq_head has waiting processes
//
// Please refer to the comment for waitqueue_active.
//
// We need to be sure we are in sync with the
// add_wait_queue modifications to the wait queue.
//
// This memory barrier should be paired with one on the
// waiting side.
//
extern "C" {
    pub fn waitqueue_active(_arg: wq_head) -> return;
}
extern "C" {
    pub fn add_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry);
}
extern "C" {
    pub fn add_wait_queue_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry);
}
extern "C" {
    pub fn add_wait_queue_priority(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry);
}
extern "C" {
    pub fn remove_wait_queue(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry);
}
//
// Used for wake-one threads:
//
extern "C" {
    pub fn __wake_up(wq_head: *mut wait_queue_head, mode: c_uint, nr: c_int, key: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __wake_up_on_current_cpu(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void);
}
extern "C" {
    pub fn __wake_up_locked_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void);
}
extern "C" {
    pub fn __wake_up_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void);
}
extern "C" {
    pub fn __wake_up_locked_sync_key(wq_head: *mut wait_queue_head, mode: c_uint, key: *mut c_void);
}
extern "C" {
    pub fn __wake_up_locked(wq_head: *mut wait_queue_head, mode: c_uint, nr: c_int);
}
extern "C" {
    pub fn __wake_up_sync(wq_head: *mut wait_queue_head, mode: c_uint);
}
extern "C" {
    pub fn __wake_up_pollfree(wq_head: *mut wait_queue_head);
}

//
// Wakeup macros to be used to report events to the targets.
//

//
// wake_up_pollfree - signal that a polled waitqueue is going away
// @wq_head: the wait queue head
//
// In the very rare cases where a ->poll() implementation uses a waitqueue whose
// lifetime is tied to a task rather than to the 'struct file' being polled,
// this function must be called before the waitqueue is freed so that
// non-blocking polls (e.g. epoll) are notified that the queue is going away.
//
// The caller must also RCU-delay the freeing of the wait_queue_head, e.g. via
// an explicit synchronize_rcu() or call_rcu(), or via SLAB_TYPESAFE_BY_RCU.
//
// For performance reasons, we don't always take the queue lock here.
// Therefore, we might race with someone removing the last entry from
// the queue, and proceed while they still hold the queue lock.
// However, rcu_read_lock() is required to be held in such cases, so we
// can safely proceed with an RCU-delayed free.
//

extern "C" {
    pub fn init_wait_entry(wq_entry: *mut wait_queue_entry, flags: c_int);
}
//
// The below macro ___wait_event() has an explicit shadow of the __ret
// variable when used from the wait_event_*() macros.
//
// This is so that both can use the ___wait_cond_timeout() construct
// to wrap the condition.
//
// The type inconsistency of the wait_event_*() __ret variable is also
// on purpose; we use long where we can return timeout values and int
// otherwise.
//

//
// wait_event - sleep until a condition gets true
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_UNINTERRUPTIBLE) until the
// @condition evaluates to true. The @condition is checked each time
// the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//

//
// io_wait_event() -- like wait_event() but with io_schedule()
//

//
// wait_event_freezable - sleep (or freeze) until a condition gets true
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_INTERRUPTIBLE -- so as not to contribute
// to system load) until the @condition evaluates to true. The
// @condition is checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//

//
// wait_event_timeout - sleep until a condition gets true or a timeout elapses
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout, in jiffies
//
// The process is put to sleep (TASK_UNINTERRUPTIBLE) until the
// @condition evaluates to true. The @condition is checked each time
// the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// Returns:
// 0 if the @condition evaluated to %false after the @timeout elapsed,
// 1 if the @condition evaluated to %true after the @timeout elapsed,
// or the remaining jiffies (at least 1) if the @condition evaluated
// to %true before the @timeout elapsed.
//

//
// like wait_event_timeout() -- except it uses TASK_INTERRUPTIBLE to avoid
// increasing load and is freezable.
//

//
// Just like wait_event_cmd(), except it sets exclusive flag
//

//
// wait_event_cmd - sleep until a condition gets true
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @cmd1: the command will be executed before sleep
// @cmd2: the command will be executed after sleep
//
// The process is put to sleep (TASK_UNINTERRUPTIBLE) until the
// @condition evaluates to true. The @condition is checked each time
// the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//

//
// wait_event_interruptible - sleep until a condition gets true
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function will return -ERESTARTSYS if it was interrupted by a
// signal and 0 if @condition evaluated to true.
//

//
// wait_event_interruptible_timeout - sleep until a condition gets true or a timeout elapses
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout, in jiffies
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// Returns:
// 0 if the @condition evaluated to %false after the @timeout elapsed,
// 1 if the @condition evaluated to %true after the @timeout elapsed,
// the remaining jiffies (at least 1) if the @condition evaluated
// to %true before the @timeout elapsed, or -%ERESTARTSYS if it was
// interrupted by a signal.
//

//
// wait_event_hrtimeout - sleep until a condition gets true or a timeout elapses
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout, as a ktime_t
//
// The process is put to sleep (TASK_UNINTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function returns 0 if @condition became true, or -ETIME if the timeout
// elapsed.
//

//
// wait_event_interruptible_hrtimeout - sleep until a condition gets true or a timeout elapses
// @wq: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout, as a ktime_t
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function returns 0 if @condition became true, -ERESTARTSYS if it was
// interrupted by a signal, or -ETIME if the timeout elapsed.
//

//
// wait_event_idle - wait for a condition without contributing to system load
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_IDLE) until the
// @condition evaluates to true.
// The @condition is checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//

//
// wait_event_idle_exclusive - wait for a condition with contributing to system load
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_IDLE) until the
// @condition evaluates to true.
// The @condition is checked each time the waitqueue @wq_head is woken up.
//
// The process is put on the wait queue with an WQ_FLAG_EXCLUSIVE flag
// set thus if other processes wait on the same list, when this
// process is woken further processes are not considered.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//

//
// wait_event_idle_timeout - sleep without load until a condition becomes true or a timeout elapses
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout, in jiffies
//
// The process is put to sleep (TASK_IDLE) until the
// @condition evaluates to true. The @condition is checked each time
// the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// Returns:
// 0 if the @condition evaluated to %false after the @timeout elapsed,
// 1 if the @condition evaluated to %true after the @timeout elapsed,
// or the remaining jiffies (at least 1) if the @condition evaluated
// to %true before the @timeout elapsed.
//

//
// wait_event_idle_exclusive_timeout - sleep without load until a condition becomes true or a timeout elapses
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout, in jiffies
//
// The process is put to sleep (TASK_IDLE) until the
// @condition evaluates to true. The @condition is checked each time
// the waitqueue @wq_head is woken up.
//
// The process is put on the wait queue with an WQ_FLAG_EXCLUSIVE flag
// set thus if other processes wait on the same list, when this
// process is woken further processes are not considered.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// Returns:
// 0 if the @condition evaluated to %false after the @timeout elapsed,
// 1 if the @condition evaluated to %true after the @timeout elapsed,
// or the remaining jiffies (at least 1) if the @condition evaluated
// to %true before the @timeout elapsed.
//

extern "C" {
    pub fn do_wait_intr(: *mut wait_queue_head_t, : *mut wait_queue_entry_t) -> c_int;
}
extern "C" {
    pub fn do_wait_intr_irq(: *mut wait_queue_head_t, : *mut wait_queue_entry_t) -> c_int;
}

//
// wait_event_interruptible_locked - sleep until a condition gets true
// @wq: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq is woken up.
//
// It must be called with wq.lock being held.  This spinlock is
// unlocked while sleeping but @condition testing is done while lock
// is held and when this macro exits the lock is held.
//
// The lock is locked/unlocked using spin_lock()/spin_unlock()
// functions which must match the way they are locked/unlocked outside
// of this macro.
//
// wake_up_locked() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function will return -ERESTARTSYS if it was interrupted by a
// signal and 0 if @condition evaluated to true.
//

//
// wait_event_interruptible_locked_irq - sleep until a condition gets true
// @wq: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq is woken up.
//
// It must be called with wq.lock being held.  This spinlock is
// unlocked while sleeping but @condition testing is done while lock
// is held and when this macro exits the lock is held.
//
// The lock is locked/unlocked using spin_lock_irq()/spin_unlock_irq()
// functions which must match the way they are locked/unlocked outside
// of this macro.
//
// wake_up_locked() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function will return -ERESTARTSYS if it was interrupted by a
// signal and 0 if @condition evaluated to true.
//

//
// wait_event_interruptible_exclusive_locked - sleep exclusively until a condition gets true
// @wq: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq is woken up.
//
// It must be called with wq.lock being held.  This spinlock is
// unlocked while sleeping but @condition testing is done while lock
// is held and when this macro exits the lock is held.
//
// The lock is locked/unlocked using spin_lock()/spin_unlock()
// functions which must match the way they are locked/unlocked outside
// of this macro.
//
// The process is put on the wait queue with an WQ_FLAG_EXCLUSIVE flag
// set thus when other process waits process on the list if this
// process is awaken further processes are not considered.
//
// wake_up_locked() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function will return -ERESTARTSYS if it was interrupted by a
// signal and 0 if @condition evaluated to true.
//

//
// wait_event_interruptible_exclusive_locked_irq - sleep until a condition gets true
// @wq: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq is woken up.
//
// It must be called with wq.lock being held.  This spinlock is
// unlocked while sleeping but @condition testing is done while lock
// is held and when this macro exits the lock is held.
//
// The lock is locked/unlocked using spin_lock_irq()/spin_unlock_irq()
// functions which must match the way they are locked/unlocked outside
// of this macro.
//
// The process is put on the wait queue with an WQ_FLAG_EXCLUSIVE flag
// set thus when other process waits process on the list if this
// process is awaken further processes are not considered.
//
// wake_up_locked() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function will return -ERESTARTSYS if it was interrupted by a
// signal and 0 if @condition evaluated to true.
//

//
// wait_event_killable - sleep until a condition gets true
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
//
// The process is put to sleep (TASK_KILLABLE) until the
// @condition evaluates to true or a signal is received.
// The @condition is checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function will return -ERESTARTSYS if it was interrupted by a
// signal and 0 if @condition evaluated to true.
//

//
// wait_event_killable() - link wait_event_killable but with io_schedule()
//

//
// wait_event_state - sleep until a condition gets true
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @state: state to sleep in
//
// The process is put to sleep (@state) until the @condition evaluates to true
// or a signal is received (when allowed by @state).  The @condition is checked
// each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// The function will return -ERESTARTSYS if it was interrupted by a signal
// (when allowed by @state) and 0 if @condition evaluated to true.
//

//
// wait_event_killable_timeout - sleep until a condition gets true or a timeout elapses
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @timeout: timeout, in jiffies
//
// The process is put to sleep (TASK_KILLABLE) until the
// @condition evaluates to true or a kill signal is received.
// The @condition is checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// Returns:
// 0 if the @condition evaluated to %false after the @timeout elapsed,
// 1 if the @condition evaluated to %true after the @timeout elapsed,
// the remaining jiffies (at least 1) if the @condition evaluated
// to %true before the @timeout elapsed, or -%ERESTARTSYS if it was
// interrupted by a kill signal.
//
// Only kill signals interrupt this process.
//

//
// wait_event_lock_irq_cmd - sleep until a condition gets true. The
// condition is checked under the lock. This
// is expected to be called with the lock
// taken.
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @lock: a locked spinlock_t, which will be released before cmd
// and schedule() and reacquired afterwards.
// @cmd: a command which is invoked outside the critical section before
// sleep
//
// The process is put to sleep (TASK_UNINTERRUPTIBLE) until the
// @condition evaluates to true. The @condition is checked each time
// the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// This is supposed to be called while holding the lock. The lock is
// dropped before invoking the cmd and going to sleep and is reacquired
// afterwards.
//

//
// wait_event_lock_irq - sleep until a condition gets true. The
// condition is checked under the lock. This
// is expected to be called with the lock
// taken.
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @lock: a locked spinlock_t, which will be released before schedule()
// and reacquired afterwards.
//
// The process is put to sleep (TASK_UNINTERRUPTIBLE) until the
// @condition evaluates to true. The @condition is checked each time
// the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// This is supposed to be called while holding the lock. The lock is
// dropped before going to sleep and is reacquired afterwards.
//

//
// wait_event_interruptible_lock_irq_cmd - sleep until a condition gets true.
// The condition is checked under the lock. This is expected to
// be called with the lock taken.
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @lock: a locked spinlock_t, which will be released before cmd and
// schedule() and reacquired afterwards.
// @cmd: a command which is invoked outside the critical section before
// sleep
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or a signal is received. The @condition is
// checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// This is supposed to be called while holding the lock. The lock is
// dropped before invoking the cmd and going to sleep and is reacquired
// afterwards.
//
// The macro will return -ERESTARTSYS if it was interrupted by a signal
// and 0 if @condition evaluated to true.
//

//
// wait_event_interruptible_lock_irq - sleep until a condition gets true.
// The condition is checked under the lock. This is expected
// to be called with the lock taken.
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @lock: a locked spinlock_t, which will be released before schedule()
// and reacquired afterwards.
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or signal is received. The @condition is
// checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// This is supposed to be called while holding the lock. The lock is
// dropped before going to sleep and is reacquired afterwards.
//
// The macro will return -ERESTARTSYS if it was interrupted by a signal
// and 0 if @condition evaluated to true.
//

//
// wait_event_interruptible_lock_irq_timeout - sleep until a condition gets
// true or a timeout elapses. The condition is checked under
// the lock. This is expected to be called with the lock taken.
// @wq_head: the waitqueue to wait on
// @condition: a C expression for the event to wait for
// @lock: a locked spinlock_t, which will be released before schedule()
// and reacquired afterwards.
// @timeout: timeout, in jiffies
//
// The process is put to sleep (TASK_INTERRUPTIBLE) until the
// @condition evaluates to true or signal is received. The @condition is
// checked each time the waitqueue @wq_head is woken up.
//
// wake_up() has to be called after changing any variable that could
// change the result of the wait condition.
//
// This is supposed to be called while holding the lock. The lock is
// dropped before going to sleep and is reacquired afterwards.
//
// The function returns 0 if the @timeout elapsed, -ERESTARTSYS if it
// was interrupted by a signal, and the remaining jiffies otherwise
// if the condition evaluated to true before the timeout elapsed.
//

//
// Waitqueues which are removed from the waitqueue_head at wakeup time
//
extern "C" {
    pub fn prepare_to_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int);
}
extern "C" {
    pub fn prepare_to_wait_exclusive(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> bool;
}
extern "C" {
    pub fn prepare_to_wait_event(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry, state: c_int) -> c_long;
}
extern "C" {
    pub fn finish_wait(wq_head: *mut wait_queue_head, wq_entry: *mut wait_queue_entry);
}
extern "C" {
    pub fn wait_woken(wq_entry: *mut wait_queue_entry, mode: unsigned, timeout: c_long) -> c_long;
}
extern "C" {
    pub fn woken_wake_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, key: *mut c_void) -> c_int;
}
extern "C" {
    pub fn woken_wake_bit_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, key: *mut c_void) -> c_int;
}
extern "C" {
    pub fn autoremove_wake_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, key: *mut c_void) -> c_int;
}

extern "C" {
    pub fn int(p: *mut *mut task_call_f)(struct task_struct, arg: *mut c_void) -> typedef;
}
extern "C" {
    pub fn task_call_func(p: *mut task_struct, func: task_call_f, arg: *mut c_void) -> c_int;
}
