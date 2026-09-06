//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/percpu-refcount.h
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
// Percpu refcounts:
// (C) 2012 Google, Inc.
// Author: Kent Overstreet <koverstreet@google.com>
//
// This implements a refcount with similar semantics to atomic_t - atomic_inc(),
// atomic_dec_and_test() - but percpu.
//
// There's one important difference between percpu refs and normal atomic_t
// refcounts; you have to keep track of your initial refcount, and then when you
// start shutting down you call percpu_ref_kill() _before_ dropping the initial
// refcount.
//
// The refcount will have a range of 0 to LONG_MAX, i.e. one bit less
// than an atomic_long_t - this is because of the way shutdown works, see
// percpu_ref_kill()/PERCPU_COUNT_BIAS.
//
// Before you call percpu_ref_kill(), percpu_ref_put() does not check for the
// refcount hitting 0 - it can't, if it was in percpu mode. percpu_ref_kill()
// puts the ref back in single atomic_t mode, collecting the per cpu refs and
// issuing the appropriate barriers, and then marks the ref as shutting down so
// that percpu_ref_put() will check for the ref hitting 0.  After it returns,
// it's safe to drop the initial ref.
//
// USAGE:
//
// See fs/aio.c for some example usage; it's used there for struct kioctx, which
// is created when userspaces calls io_setup(), and destroyed when userspace
// calls io_destroy() or the process exits.
//
// In the aio code, kill_ioctx() is called when we wish to destroy a kioctx; it
// removes the kioctx from the proccess's table of kioctxs and kills percpu_ref.
// After that, there can't be any new users of the kioctx (from lookup_ioctx())
// and it's then safe to drop the initial ref with percpu_ref_put().
//
// Note that the free path, free_ioctx(), needs to go through explicit call_rcu()
// to synchronize with RCU protected lookup_ioctx().  percpu_ref operations don't
// imply RCU grace periods of any kind and if a user wants to combine percpu_ref
// with RCU protection, it must be done explicitly.
//
// Code that does a two stage shutdown like this often needs some kind of
// explicit synchronization to ensure the initial refcount can only be dropped
// once - percpu_ref_kill() does this for you, it returns true once and false if
// someone else already called it. The aio code uses it this way, but it's not
// necessary if the code has some other mechanism to synchronize teardown.
// around.
//

extern "C" {
    pub fn void(: *mut percpu_ref_func_t)(struct percpu_ref) -> typedef;
}
// flags set in the lower bits of percpu_ref->percpu_count_ptr
// @flags for percpu_ref_init()
//
// Start w/ ref == 1 in atomic mode.  Can be switched to percpu
// operation using percpu_ref_switch_to_percpu().  If initialized
// with this flag, the ref will stay in atomic mode until
// percpu_ref_switch_to_percpu() is invoked on it.
// Implies ALLOW_REINIT.
//
// Start dead w/ ref == 0 in atomic mode.  Must be revived with
// percpu_ref_reinit() before used.  Implies INIT_ATOMIC and
// ALLOW_REINIT.
//
// Allow switching from atomic mode to percpu mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_ref_data {
    pub count: atomic_long_t,
    pub release: *mut percpu_ref_func_t,
    pub confirm_switch: *mut percpu_ref_func_t,
    pub force_atomic:1: bool,
    pub allow_reinit:1: bool,
    pub rcu: rcu_head,
    pub ref: *mut percpu_ref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_ref {
//
// The low bit of the pointer indicates whether the ref is in percpu
// mode; if set, then get/put will manipulate the atomic_t.
//
    pub percpu_count_ptr: c_ulong,
//
// 'percpu_ref' is often embedded into user structure, and only
// 'percpu_count_ptr' is required in fast path, move other fields
// into 'percpu_ref_data', so we can reduce memory footprint in
// fast path.
//
    pub data: *mut percpu_ref_data,
}

extern "C" {
    pub fn percpu_ref_exit(ref: *mut percpu_ref);
}
extern "C" {
    pub fn percpu_ref_switch_to_atomic_sync(ref: *mut percpu_ref);
}
extern "C" {
    pub fn percpu_ref_switch_to_percpu(ref: *mut percpu_ref);
}
extern "C" {
    pub fn percpu_ref_resurrect(ref: *mut percpu_ref);
}
extern "C" {
    pub fn percpu_ref_reinit(ref: *mut percpu_ref);
}
extern "C" {
    pub fn percpu_ref_is_zero(ref: *mut percpu_ref) -> bool;
}
//
// percpu_ref_kill - drop the initial ref
// @ref: percpu_ref to kill
//
// Must be used to drop the initial ref on a percpu refcount; must be called
// precisely once before shutdown.
//
// Switches @ref into atomic mode before gathering up the percpu counters
// and dropping the initial ref.
//
// There are no implied RCU grace periods between kill and release.
//
// Internal helper.  Don't use outside percpu-refcount proper.  The
// function doesn't return the pointer and let the caller test it for NULL
// because doing so forces the compiler to generate two conditional
// branches as it can't assume that @ref->percpu_count is not NULL.
//
// The value of @ref->percpu_count_ptr is tested for
// !__PERCPU_REF_ATOMIC, which may be set asynchronously, and then
// used as a pointer.  If the compiler generates a separate fetch
// when using it as a pointer, __PERCPU_REF_ATOMIC may be set in
// between contaminating the pointer value, meaning that
// READ_ONCE() is required when fetching it.
//
// The dependency ordering from the READ_ONCE() pairs
// with smp_store_release() in __percpu_ref_switch_to_percpu().
//
// Theoretically, the following could test just ATOMIC; however,
// then we'd have to mask off DEAD separately as DEAD may be
// visible without ATOMIC if we race with percpu_ref_kill().  DEAD
// implies ATOMIC anyway.  Test them together.
//
// percpu_countp = (unsigned long __percpu *)percpu_ptr;
//
// percpu_ref_get_many - increment a percpu refcount
// @ref: percpu_ref to get
// @nr: number of references to get
//
// Analogous to atomic_long_add().
//
// This function is safe to call as long as @ref is between init and exit.
//
// percpu_ref_get - increment a percpu refcount
// @ref: percpu_ref to get
//
// Analogous to atomic_long_inc().
//
// This function is safe to call as long as @ref is between init and exit.
//
// percpu_ref_tryget_many - try to increment a percpu refcount
// @ref: percpu_ref to try-get
// @nr: number of references to get
//
// Increment a percpu refcount  by @nr unless its count already reached zero.
// Returns %true on success; %false on failure.
//
// This function is safe to call as long as @ref is between init and exit.
//
// percpu_ref_tryget - try to increment a percpu refcount
// @ref: percpu_ref to try-get
//
// Increment a percpu refcount unless its count already reached zero.
// Returns %true on success; %false on failure.
//
// This function is safe to call as long as @ref is between init and exit.
//
extern "C" {
    pub fn percpu_ref_tryget_many(_arg: ref, _arg: 1) -> return;
}
//
// percpu_ref_tryget_live_rcu - same as percpu_ref_tryget_live() but the
// caller is responsible for taking RCU.
// @ref: percpu_ref to try-get
//
// This function is safe to call as long as @ref is between init and exit.
//
// percpu_ref_tryget_live - try to increment a live percpu refcount
// @ref: percpu_ref to try-get
//
// Increment a percpu refcount unless it has already been killed.  Returns
// %true on success; %false on failure.
//
// Completion of percpu_ref_kill() in itself doesn't guarantee that this
// function will fail.  For such guarantee, percpu_ref_kill_and_confirm()
// should be used.  After the confirm_kill callback is invoked, it's
// guaranteed that no new reference will be given out by
// percpu_ref_tryget_live().
//
// This function is safe to call as long as @ref is between init and exit.
//
// percpu_ref_put_many - decrement a percpu refcount
// @ref: percpu_ref to put
// @nr: number of references to put
//
// Decrement the refcount, and if 0, call the release function (which was passed
// to percpu_ref_init())
//
// This function is safe to call as long as @ref is between init and exit.
//
// percpu_ref_put - decrement a percpu refcount
// @ref: percpu_ref to put
//
// Decrement the refcount, and if 0, call the release function (which was passed
// to percpu_ref_init())
//
// This function is safe to call as long as @ref is between init and exit.
//
// percpu_ref_is_dying - test whether a percpu refcount is dying or dead
// @ref: percpu_ref to test
//
// Returns %true if @ref is dying or dead.
//
// This function is safe to call as long as @ref is between init and exit
// and the caller is responsible for synchronizing against state changes.
//
