//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/refcount.h
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
// Variant of atomic_t specialized for reference counts.
//
// The interface matches the atomic_t interface (to aid in porting) but only
// provides the few functions one should use for reference counting.
//
// Saturation semantics
// ====================
//
// refcount_t differs from atomic_t in that the counter saturates at
// REFCOUNT_SATURATED and will not move once there. This avoids wrapping the
// counter and causing 'spurious' use-after-free issues. In order to avoid the
// cost associated with introducing cmpxchg() loops into all of the saturating
// operations, we temporarily allow the counter to take on an unchecked value
// and then explicitly set it to REFCOUNT_SATURATED on detecting that underflow
// or overflow has occurred. Although this is racy when multiple threads
// access the refcount concurrently, by placing REFCOUNT_SATURATED roughly
// equidistant from 0 and INT_MAX we minimise the scope for error:
//
// INT_MAX     REFCOUNT_SATURATED   UINT_MAX
// 0                          (0x7fff_ffff)    (0xc000_0000)    (0xffff_ffff)
// +--------------------------------+----------------+----------------+
// <---------- bad value! ---------->
//
// (in a signed view of the world, the "bad value" range corresponds to
// a negative counter value).
//
// As an example, consider a refcount_inc() operation that causes the counter
// to overflow:
//
// int old = atomic_fetch_add_relaxed(r);
// // old is INT_MAX, refcount now INT_MIN (0x8000_0000)
// if (old < 0)
// atomic_set(r, REFCOUNT_SATURATED);
//
// If another thread also performs a refcount_inc() operation between the two
// atomic operations, then the count will continue to edge closer to 0. If it
// reaches a value of 1 before /any/ of the threads reset it to the saturated
// value, then a concurrent refcount_dec_and_test() may erroneously free the
// underlying object.
// Linux limits the maximum number of tasks to PID_MAX_LIMIT, which is currently
// 0x400000 (and can't easily be raised in the future beyond FUTEX_TID_MASK).
// With the current PID limit, if no batched refcounting operations are used and
// the attacker can't repeatedly trigger kernel oopses in the middle of refcount
// operations, this makes it impossible for a saturated refcount to leave the
// saturation range, even if it is possible for multiple uses of the same
// refcount to nest in the context of a single task:
//
// (UINT_MAX+1-REFCOUNT_SATURATED) / PID_MAX_LIMIT =
// 0x40000000 / 0x400000 = 0x100 = 256
//
// If hundreds of references are added/removed with a single refcounting
// operation, it may potentially be possible to leave the saturation range; but
// given the precise timing details involved with the round-robin scheduling of
// each thread manipulating the refcount and the need to hit the race multiple
// times in succession, there doesn't appear to be a practical avenue of attack
// even if using refcount_add() operations with larger increments.
//
// Memory ordering
// ===============
//
// Memory ordering rules are slightly relaxed wrt regular atomic_t functions
// and provide only what is strictly required for refcounts.
//
// The increments are fully relaxed; these will not provide ordering. The
// rationale is that whatever is used to obtain the object we're increasing the
// reference count on will provide the ordering. For locked data structures,
// its the lock acquire, for RCU/lockless data structures its the dependent
// load.
//
// Do note that inc_not_zero() provides a control dependency which will order
// future stores against the inc, this ensures we'll never modify the object
// if we did not in fact acquire a reference.
//
// The decrements will provide release order, such that all the prior loads and
// stores will be issued before, it also provides a control dependency, which
// will order us against the subsequent free().
//
// The control dependency is against the load of the cmpxchg (ll/sc) that
// succeeded. This means the stores aren't fully ordered, but this is fine
// because the 1->0 transition indicates no concurrency.
//
// Note that the allocator is responsible for ordering things between free()
// and alloc().
//
// The decrements dec_and_test() and sub_and_test() also provide acquire
// ordering on success.
//
// refcount_{add|inc}_not_zero_acquire() and refcount_set_release() provide
// acquire and release ordering for cases when the memory occupied by the
// object might be reused to store another object. This is important for the
// cases where secondary validation is required to detect such reuse, e.g.
// SLAB_TYPESAFE_BY_RCU. The secondary validation checks have to happen after
// the refcount is taken, hence acquire order is necessary. Similarly, when the
// object is initialized, all stores to its attributes should be visible before
// the refcount is set, otherwise a stale attribute value might be used by
// another task which succeeds in taking a refcount to the new object.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum refcount_saturation_type {
    REFCOUNT_ADD_NOT_ZERO_OVF,
    REFCOUNT_ADD_OVF,
    REFCOUNT_ADD_UAF,
    REFCOUNT_SUB_UAF,
    REFCOUNT_DEC_LEAK,
}

extern "C" {
    pub fn refcount_warn_saturate(r: *mut refcount_t, t: refcount_saturation_type);
}
//
// refcount_set - set a refcount's value
// @r: the refcount
// @n: value to which the refcount will be set
//
// refcount_set_release - set a refcount's value with release ordering
// @r: the refcount
// @n: value to which the refcount will be set
//
// This function should be used when memory occupied by the object might be
// reused to store another object -- consider SLAB_TYPESAFE_BY_RCU.
//
// Provides release memory ordering which will order previous memory operations
// against this store. This ensures all updates to this object are visible
// once the refcount is set and stale values from the object previously
// occupying this memory are overwritten with new ones.
//
// This function should be called only after new object is fully initialized.
// After this call the object should be considered visible to other tasks even
// if it was not yet added into an object collection normally used to discover
// it. This is because other tasks might have discovered the object previously
// occupying the same memory and after memory reuse they can succeed in taking
// refcount to the new object and start using it.
//
// refcount_read - get a refcount's value
// @r: the refcount
//
// Return: the refcount's value
//
extern "C" {
    pub fn atomic_read(_arg: &r->refs) -> return;
}
// oldp = old;
//
// refcount_add_not_zero - add a value to a refcount unless it is 0
// @i: the value to add to the refcount
// @r: the refcount
//
// Will saturate at REFCOUNT_SATURATED and WARN.
//
// Provides no memory ordering, it is assumed the caller has guaranteed the
// object memory to be stable (RCU, etc.). It does provide a control dependency
// and thereby orders future stores. See the comment on top.
//
// Use of this function is not recommended for the normal reference counting
// use case in which references are taken and released one at a time.  In these
// cases, refcount_inc(), or one of its variants, should instead be used to
// increment a reference count.
//
// Return: false if the passed refcount is 0, true otherwise
//
extern "C" {
    pub fn __refcount_add_not_zero(_arg: i, _arg: r, _arg: NULL) -> return;
}
// oldp = old;
extern "C" {
    pub fn __refcount_add_not_zero_limited_acquire(_arg: 1, _arg: r, _arg: oldp, _arg: limit) -> return;
}
extern "C" {
    pub fn __refcount_add_not_zero_limited_acquire(_arg: i, _arg: r, _arg: oldp, _arg: INT_MAX) -> return;
}
//
// refcount_add_not_zero_acquire - add a value to a refcount with acquire ordering unless it is 0
//
// @i: the value to add to the refcount
// @r: the refcount
//
// Will saturate at REFCOUNT_SATURATED and WARN.
//
// This function should be used when memory occupied by the object might be
// reused to store another object -- consider SLAB_TYPESAFE_BY_RCU.
//
// Provides acquire memory ordering on success, it is assumed the caller has
// guaranteed the object memory to be stable (RCU, etc.). It does provide a
// control dependency and thereby orders future stores. See the comment on top.
//
// Use of this function is not recommended for the normal reference counting
// use case in which references are taken and released one at a time.  In these
// cases, refcount_inc_not_zero_acquire() should instead be used to increment a
// reference count.
//
// Return: false if the passed refcount is 0, true otherwise
//
extern "C" {
    pub fn __refcount_add_not_zero_acquire(_arg: i, _arg: r, _arg: NULL) -> return;
}
// oldp = old;
//
// refcount_add - add a value to a refcount
// @i: the value to add to the refcount
// @r: the refcount
//
// Similar to atomic_add(), but will saturate at REFCOUNT_SATURATED and WARN.
//
// Provides no memory ordering, it is assumed the caller has guaranteed the
// object memory to be stable (RCU, etc.). It does provide a control dependency
// and thereby orders future stores. See the comment on top.
//
// Use of this function is not recommended for the normal reference counting
// use case in which references are taken and released one at a time.  In these
// cases, refcount_inc(), or one of its variants, should instead be used to
// increment a reference count.
//
extern "C" {
    pub fn __refcount_add_not_zero(_arg: 1, _arg: r, _arg: oldp) -> return;
}
//
// refcount_inc_not_zero - increment a refcount unless it is 0
// @r: the refcount to increment
//
// Similar to atomic_inc_not_zero(), but will saturate at REFCOUNT_SATURATED
// and WARN.
//
// Provides no memory ordering, it is assumed the caller has guaranteed the
// object memory to be stable (RCU, etc.). It does provide a control dependency
// and thereby orders future stores. See the comment on top.
//
// Return: true if the increment was successful, false otherwise
//
extern "C" {
    pub fn __refcount_inc_not_zero(_arg: r, _arg: NULL) -> return;
}
extern "C" {
    pub fn __refcount_add_not_zero_acquire(_arg: 1, _arg: r, _arg: oldp) -> return;
}
//
// refcount_inc_not_zero_acquire - increment a refcount with acquire ordering unless it is 0
// @r: the refcount to increment
//
// Similar to refcount_inc_not_zero(), but provides acquire memory ordering on
// success.
//
// This function should be used when memory occupied by the object might be
// reused to store another object -- consider SLAB_TYPESAFE_BY_RCU.
//
// Provides acquire memory ordering on success, it is assumed the caller has
// guaranteed the object memory to be stable (RCU, etc.). It does provide a
// control dependency and thereby orders future stores. See the comment on top.
//
// Return: true if the increment was successful, false otherwise
//
extern "C" {
    pub fn __refcount_inc_not_zero_acquire(_arg: r, _arg: NULL) -> return;
}
//
// refcount_inc - increment a refcount
// @r: the refcount to increment
//
// Similar to atomic_inc(), but will saturate at REFCOUNT_SATURATED and WARN.
//
// Provides no memory ordering, it is assumed the caller already has a
// reference on the object.
//
// Will WARN if the refcount is 0, as this represents a possible use-after-free
// condition.
//
// oldp = old;
//
// refcount_sub_and_test - subtract from a refcount and test if it is 0
// @i: amount to subtract from the refcount
// @r: the refcount
//
// Similar to atomic_dec_and_test(), but it will WARN, return false and
// ultimately leak on underflow and will fail to decrement when saturated
// at REFCOUNT_SATURATED.
//
// Provides release memory ordering, such that prior loads and stores are done
// before, and provides an acquire ordering on success such that free()
// must come after.
//
// Use of this function is not recommended for the normal reference counting
// use case in which references are taken and released one at a time.  In these
// cases, refcount_dec(), or one of its variants, should instead be used to
// decrement a reference count.
//
// Return: true if the resulting refcount is 0, false otherwise
//
extern "C" {
    pub fn __refcount_sub_and_test(_arg: i, _arg: r, _arg: NULL) -> return;
}
extern "C" {
    pub fn __refcount_sub_and_test(_arg: 1, _arg: r, _arg: oldp) -> return;
}
//
// refcount_dec_and_test - decrement a refcount and test if it is 0
// @r: the refcount
//
// Similar to atomic_dec_and_test(), it will WARN on underflow and fail to
// decrement when saturated at REFCOUNT_SATURATED.
//
// Provides release memory ordering, such that prior loads and stores are done
// before, and provides an acquire ordering on success such that free()
// must come after.
//
// Return: true if the resulting refcount is 0, false otherwise
//
extern "C" {
    pub fn __refcount_dec_and_test(_arg: r, _arg: NULL) -> return;
}
// oldp = old;
//
// refcount_dec - decrement a refcount
// @r: the refcount
//
// Similar to atomic_dec(), it will WARN on underflow and fail to decrement
// when saturated at REFCOUNT_SATURATED.
//
// Provides release memory ordering, such that prior loads and stores are done
// before.
//
extern "C" {
    pub fn refcount_dec_if_one(r: *mut refcount_t) -> __must_check bool;
}
extern "C" {
    pub fn refcount_dec_not_one(r: *mut refcount_t) -> __must_check bool;
}
extern "C" {
    pub fn refcount_dec_and_mutex_lock(r: *mut refcount_t, __cond_acquires(true: *mut *mut mutex lock), _arg: lock) -> __must_check bool;
}
extern "C" {
    pub fn refcount_dec_and_lock(r: *mut refcount_t, __cond_acquires(true: *mut *mut spinlock_t lock), _arg: lock) -> __must_check bool;
}
