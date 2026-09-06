//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/locking/ww_mutex.h
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

//
// +--------+
// | first  |
// +--------+
// |
// v
// +----+     +----+     +----+
// | W3 | <-> | W1 | <-> | W2 |
// +----+     +----+     +----+
// ^                     ^
// +---------------------+
//
// for (cur = __ww_waiter_first(); cur; cur = __ww_waiter_next())
//
// Should iterate like: W1, W2, W3
//
// Terminate if the next entry is the first again, that has already
// been observed.
//
// for (cur = __ww_waiter_last(); cur; cur = __ww_waiter_prev())
//
// Should iterate like: W3, W2, W1
//
// Terminate at the first entry, the previous entry of first is the
// last and that has already been observed.
//
extern "C" {
    pub fn list_prev_entry(_arg: w, _arg: list) -> return;
}
extern "C" {
    pub fn __mutex_owner(_arg: lock) -> return;
}

extern "C" {
    pub fn rb_entry(_arg: n, rt_mutex_waiter: struct, _arg: tree.entry) -> return;
}
extern "C" {
    pub fn rb_entry(_arg: n, rt_mutex_waiter: struct, _arg: tree.entry) -> return;
}
extern "C" {
    pub fn rb_entry(_arg: n, rt_mutex_waiter: struct, _arg: tree.entry) -> return;
}
extern "C" {
    pub fn rb_entry(_arg: n, rt_mutex_waiter: struct, _arg: tree.entry) -> return;
}
// RT unconditionally adds the waiter first and then removes it on error
extern "C" {
    pub fn rt_mutex_owner(_arg: &lock->rtmutex) -> return;
}
extern "C" {
    pub fn rt_mutex_has_waiters(_arg: &lock->rtmutex) -> return;
}

//
// Wait-Die:
// The newer transactions are killed when:
// It (the new transaction) makes a request for a lock being held
// by an older transaction.
//
// Wound-Wait:
// The newer transactions are wounded when:
// An older transaction makes a request for a lock being held by
// the newer transaction.
//
// Associate the ww_mutex @ww with the context @ww_ctx under which we acquired
// it.
//

//
// If this WARN_ON triggers, you used ww_mutex_lock to acquire,
// but released with a normal mutex_unlock in this call.
//
// This should never happen, always use ww_mutex_unlock.
//
// Not quite done after calling ww_acquire_done() ?
//
// After -EDEADLK you tried to
// acquire a different ww_mutex? Bad!
//
// You called ww_mutex_lock after receiving -EDEADLK,
// but 'forgot' to unlock everything else first?
//
// Naughty, using a different class will lead to undefined behavior!
//

//
// Determine if @a is 'less' than @b. IOW, either @a is a lower priority task
// or, when of equal priority, a younger transaction than @b.
//
// Depending on the algorithm, @a will either need to wait for @b, or die.
//
// Can only do the RT prio for WW_RT, because task->prio isn't stable due to PI,
// so the wait_list ordering will go wobbly. rt_mutex re-queues the waiter and
// isn't affected by this.
//

// kernel prio; less is more
// equal static prio
// equal prio

// FIFO order tie break -- bigger is younger
//
// Wait-Die; wake a lesser waiter context (when locks held) such that it can
// die.
//
// Among waiters with context, only the first one can have other locks acquired
// already (ctx->acquired > 0), because __ww_mutex_add_waiter() and
// __ww_mutex_check_kill() wake any but the earliest context.
//

//
// When waking up the task to die, be sure to set the
// blocked_on to PROXY_WAKING. Otherwise we can see
// circular blocked_on relationships that can't resolve.
//
// Wound-Wait; wound a lesser @hold_ctx if it holds the lock.
//
// Wound the lock holder if there are waiters with more important transactions
// than the lock holders. Even if multiple waiters may wound the lock holder,
// it's sufficient that only one does.
//
// Possible through __ww_mutex_add_waiter() when we race with
// ww_mutex_set_context_fastpath(). In that case we'll get here again
// through __ww_mutex_check_waiters().
//
// Can have !owner because of __mutex_unlock_slowpath(), but if owner,
// it cannot go away because we'll have FLAG_WAITERS set and hold
// wait_lock.
//
// wake_up_process() paired with set_current_state()
// inserts sufficient barriers to make sure @owner either sees
// it's wounded in __ww_mutex_check_kill() or has a
// wakeup pending to re-read the wounded state.
//
// When waking up the task to wound, be sure to set the
// blocked_on to PROXY_WAKING. Otherwise we can see
// circular blocked_on relationships that can't resolve.
//
// NOTE: We pass NULL here instead of lock, because we
// are waking the mutex owner, who may be currently
// blocked on a different mutex.
//
// We just acquired @lock under @ww_ctx, if there are more important contexts
// waiting behind us on the wait-list, check if they need to die, or wound us.
//
// See __ww_mutex_add_waiter() for the list-order construction; basically the
// list is ordered by stamp, smallest (oldest) first.
//
// This relies on never mixing wait-die/wound-wait on the same wait-list;
// which is currently ensured by that being a ww_class property.
//
// The current task must not be on the wait list.
//
// After acquiring lock with fastpath, where we do not hold wait_lock, set ctx
// and wake up any waiters so they can recheck.
//
// The lock->ctx update should be visible on all cores before
// the WAITERS check is done, otherwise contended waiters might be
// missed. The contended waiters will either see ww_ctx == NULL
// and keep spinning, or it will acquire wait_lock, add itself
// to waiter list and sleep.
//
// [W] ww->ctx = ctx	    [W] MUTEX_FLAG_WAITERS
// MB		        MB
// [R] MUTEX_FLAG_WAITERS   [R] ww->ctx
//
// The memory barrier above pairs with the memory barrier in
// __ww_mutex_add_waiter() and makes sure we either observe ww->ctx
// and/or !empty list.
//
// Uh oh, we raced in fastpath, check if any of the waiters need to
// die or wound us.
//

//
// Check the wound condition for the current lock acquire.
//
// Wound-Wait: If we're wounded, kill ourself.
//
// Wait-Die: If we're trying to acquire a lock already held by an older
// context, kill ourselves.
//
// Since __ww_mutex_add_waiter() orders the wait-list on stamp, we only have to
// look at waiters before us in the wait-list.
//
extern "C" {
    pub fn __ww_mutex_kill(_arg: lock, _arg: ctx) -> return;
}
extern "C" {
    pub fn __ww_mutex_kill(_arg: lock, _arg: ctx) -> return;
}
//
// If there is a waiter in front of us that has a context, then its
// stamp is earlier than ours and we must kill ourself.
//
extern "C" {
    pub fn __ww_mutex_kill(_arg: lock, _arg: ctx) -> return;
}
//
// Add @waiter to the wait-list, keep the wait-list ordered by stamp, smallest
// first. Such that older contexts are preferred to acquire the lock over
// younger contexts.
//
// Waiters without context are interspersed in FIFO order.
//
// Furthermore, for Wait-Die kill ourself immediately when possible (there are
// older contexts already waiting) to avoid unnecessary waiting and for
// Wound-Wait ensure we wound the owning context when it is younger.
//
// Add the waiter before the first waiter with a higher stamp.
// Waiters without a context are skipped to avoid starving
// them. Wait-Die waiters may die here. Wound-Wait waiters
// never die here, but they are sorted in stamp order and
// may wound the lock holder.
//
// Wait-Die: if we find an older context waiting, there
// is no point in queueing behind it, as we'd have to
// die the moment it would acquire the lock.
//
// Wait-Die: ensure younger waiters die.
//
// Wound-Wait: if we're blocking on a mutex owned by a younger context,
// wound that such that we might proceed.
//
// See ww_mutex_set_context_fastpath(). Orders setting
// MUTEX_FLAG_WAITERS vs the ww->ctx load,
// such that either we or the fastpath will wound @ww->ctx.
//

