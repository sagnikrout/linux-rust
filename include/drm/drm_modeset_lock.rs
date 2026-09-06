//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_modeset_lock.h
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


//
// Copyright (C) 2014 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// struct drm_modeset_acquire_ctx - locking context (see ww_acquire_ctx)
// @ww_ctx: base acquire ctx
// @contended: used internally for -EDEADLK handling
// @stack_depot: used internally for contention debugging
// @locked: list of held locks
// @trylock_only: trylock mode used in atomic contexts/panic notifiers
// @interruptible: whether interruptible locking should be used.
//
// Each thread competing for a set of locks must use one acquire
// ctx.  And if any lock fxn returns -EDEADLK, it must backoff and
// retry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_modeset_acquire_ctx {
    pub ww_ctx: ww_acquire_ctx,
//
// Contended lock: if a lock is contended you should only call
// drm_modeset_backoff() which drops locks and slow-locks the
// contended lock.
//
    pub contended: *mut drm_modeset_lock,
//
// Stack depot for debugging when a contended lock was not backed off
// from.
//
    pub stack_depot: depot_stack_handle_t,
//
// list of held locks (drm_modeset_lock)
//
    pub locked: list_head,
//
// Trylock mode, use only for panic handlers!
//
    pub trylock_only: bool,
// Perform interruptible waits on this context.
    pub interruptible: bool,
}

//
// struct drm_modeset_lock - used for locking modeset resources.
// @mutex: resource locking
// @head: used to hold its place on &drm_atomi_state.locked list when
// part of an atomic update
//
// Used for locking CRTCs and other modeset resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_modeset_lock {
//
// modeset lock
//
    pub mutex: ww_mutex,
//
// Resources that are locked as part of an atomic update are added
// to a list (so we know what to unlock at the end).
//
    pub head: list_head,
}

extern "C" {
    pub fn drm_modeset_acquire_fini(ctx: *mut drm_modeset_acquire_ctx);
}
extern "C" {
    pub fn drm_modeset_drop_locks(ctx: *mut drm_modeset_acquire_ctx);
}
extern "C" {
    pub fn drm_modeset_backoff(ctx: *mut drm_modeset_acquire_ctx) -> c_int;
}
extern "C" {
    pub fn drm_modeset_lock_init(lock: *mut drm_modeset_lock);
}
//
// drm_modeset_lock_fini - cleanup lock
// @lock: lock to cleanup
//
// drm_modeset_is_locked - equivalent to mutex_is_locked()
// @lock: lock to check
//
extern "C" {
    pub fn ww_mutex_is_locked(_arg: &lock->mutex) -> return;
}
//
// drm_modeset_lock_assert_held - equivalent to lockdep_assert_held()
// @lock: lock to check
//
extern "C" {
    pub fn drm_modeset_lock_single_interruptible(lock: *mut drm_modeset_lock) -> int __must_check;
}
extern "C" {
    pub fn drm_modeset_unlock(lock: *mut drm_modeset_lock);
}
extern "C" {
    pub fn drm_modeset_lock_all(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_modeset_unlock_all(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_warn_on_modeset_not_all_locked(dev: *mut drm_device);
}
//
// DRM_MODESET_LOCK_ALL_BEGIN - Helper to acquire modeset locks
// @dev: drm device
// @ctx: local modeset acquire context, will be dereferenced
// @flags: DRM_MODESET_ACQUIRE_* flags to pass to drm_modeset_acquire_init()
// @ret: local ret/err/etc variable to track error status
//
// Use these macros to simplify grabbing all modeset locks using a local
// context. This has the advantage of reducing boilerplate, but also properly
// checking return values where appropriate.
//
// Any code run between BEGIN and END will be holding the modeset locks.
//
// This must be paired with DRM_MODESET_LOCK_ALL_END(). We will jump back and
// forth between the labels on deadlock and error conditions.
//
// Drivers can acquire additional modeset locks. If any lock acquisition
// fails, the control flow needs to jump to DRM_MODESET_LOCK_ALL_END() with
// the @ret parameter containing the return value of drm_modeset_lock().
//
// Returns:
// The only possible value of ret immediately after DRM_MODESET_LOCK_ALL_BEGIN()
// is 0, so no error checking is necessary
//

//
// DRM_MODESET_LOCK_ALL_END - Helper to release and cleanup modeset locks
// @dev: drm device
// @ctx: local modeset acquire context, will be dereferenced
// @ret: local ret/err/etc variable to track error status
//
// The other side of DRM_MODESET_LOCK_ALL_BEGIN(). It will bounce back to BEGIN
// if ret is -EDEADLK.
//
// It's important that you use the same ret variable for begin and end so
// deadlock conditions are properly handled.
//
// Returns:
// ret will be untouched unless it is -EDEADLK on entry. That means that if you
// successfully acquire the locks, ret will be whatever your code sets it to. If
// there is a deadlock or other failure with acquire or backoff, ret will be set
// to that failure. In both of these cases the code between BEGIN/END will not
// be run, so the failure will reflect the inability to grab the locks.
//

