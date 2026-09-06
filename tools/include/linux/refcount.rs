//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/refcount.h
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
// It differs in that the counter saturates at UINT_MAX and will not move once
// there. This avoids wrapping the counter and causing 'spurious'
// use-after-free issues.
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

// Macro flag: #define __refcount_check

extern "C" {
    pub fn atomic_read(_arg: &r->refs) -> return;
}
//
// Similar to atomic_inc_not_zero(), will saturate at UINT_MAX and WARN.
//
// Provides no memory ordering, it is assumed the caller has guaranteed the
// object memory to be stable (RCU, etc.). It does provide a control dependency
// and thereby orders future stores. See the comment on top.
//
// Similar to atomic_inc(), will saturate at UINT_MAX and WARN.
//
// Provides no memory ordering, it is assumed the caller already has a
// reference on the object, will WARN when this is not so.
//
// Similar to atomic_dec_and_test(), it will WARN on underflow and fail to
// decrement when saturated at UINT_MAX.
//
// Provides release memory ordering, such that prior loads and stores are done
// before, and provides a control dependency such that free() must come after.
// See the comment on top.
//
extern "C" {
    pub fn refcount_sub_and_test(_arg: 1, _arg: r) -> return;
}
