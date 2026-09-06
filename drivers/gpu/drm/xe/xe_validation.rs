//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_validation.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2024 Intel Corporation
//

//
// xe_validation_lockdep() - Assert that a drm_exec locking transaction can
// be initialized at this point.
//

//
// Various values of the drm_exec pointer where we've not (yet)
// implemented full ww locking.
//
// XE_VALIDATION_UNIMPLEMENTED means implementation is pending.
// A lockdep check is made to assure that a drm_exec locking
// transaction can actually take place where the macro is
// used. If this asserts, the exec pointer needs to be assigned
// higher up in the callchain and passed down.
//
// XE_VALIDATION_UNSUPPORTED is for dma-buf code only where
// the dma-buf layer doesn't support WW locking.
//
// XE_VALIDATION_OPT_OUT is for simplification of kunit tests where
// exhaustive eviction isn't necessary.
//

//
// struct xe_validation_device - The domain for exhaustive eviction
// @lock: The lock used to exclude other processes from allocating graphics memory
//
// The struct xe_validation_device represents the domain for which we want to use
// exhaustive eviction. The @lock is typically grabbed in read mode for allocations
// but when graphics memory allocation fails, it is retried with the write mode held.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_validation_device {
    pub lock: rw_semaphore,
}

//
// struct xe_val_flags - Flags for xe_validation_ctx_init().
// @exclusive: Start the validation transaction by locking out all other validators.
// @no_block:  Don't block on initialization.
// @interruptible: Block interruptible if blocking. Implies initializing the drm_exec
// context with the DRM_EXEC_INTERRUPTIBLE_WAIT flag.
// @exec_ignore_duplicates: Initialize the drm_exec context with the
// DRM_EXEC_IGNORE_DUPLICATES flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_val_flags {
    pub :1: u32 exclusive,
    pub :1: u32 no_block,
    pub :1: u32 interruptible,
    pub :1: u32 exec_ignore_duplicates,
}

//
// struct xe_validation_ctx - A struct drm_exec subclass with support for
// exhaustive eviction
// @exec: The drm_exec object base class. Note that we use a pointer instead of
// embedding to avoid diamond inheritance.
// @val: The exhaustive eviction domain.
// @val_flags: Copy of the struct xe_val_flags passed to xe_validation_ctx_init.
// @lock_held: Whether The domain lock is currently held.
// @lock_held_exclusive: Whether the domain lock is held in exclusive mode.
// @request_exclusive: Whether to lock exclusively (write mode) the next time
// the domain lock is locked.
// @exec_flags: The drm_exec flags used for drm_exec (re-)initialization.
// @nr: The drm_exec nr parameter used for drm_exec (re-)initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_validation_ctx {
    pub exec: *mut drm_exec,
    pub val: *mut xe_validation_device,
    pub val_flags: xe_val_flags,
    pub lock_held: bool,
    pub lock_held_exclusive: bool,
    pub request_exclusive: bool,
    pub exec_flags: u32,
    pub nr: c_uint,
}

extern "C" {
    pub fn xe_validation_ctx_fini(ctx: *mut xe_validation_ctx);
}
extern "C" {
    pub fn xe_validation_should_retry(ctx: *mut xe_validation_ctx, ret: *mut c_int) -> bool;
}
//
// xe_validation_retry_on_oom() - Retry on oom in an xe_validaton transaction
// @_ctx: Pointer to the xe_validation_ctx
// @_ret: The current error value possibly holding -ENOMEM
//
// Use this in way similar to drm_exec_retry_on_contention().
// If @_ret contains -ENOMEM the transaction is restarted once in a way that
// blocks other transactions and allows exhastive eviction. If the transaction
// was already restarted once, Just return the -ENOMEM. May also set
// _ret to -EINTR if not retrying and waits are interruptible.
// May only be used within a drm_exec_until_all_locked() loop.
//

//
// xe_validation_device_init - Initialize a struct xe_validation_device
// @val: The xe_validation_device to init.
//
// Make guard() and scoped_guard() work with xe_validation_ctx
// so that we can exit transactions without caring about the
// cleanup.
//
// _ret ? NULL : _ctx; }),

//
// xe_validation_guard() - An auto-cleanup xe_validation_ctx transaction
// @_ctx: The xe_validation_ctx.
// @_val: The xe_validation_device.
// @_exec: The struct drm_exec object
// @_flags: Flags for the xe_validation_ctx initialization.
// @_ret: Return in / out parameter. May be set by this macro. Typically 0 when called.
//
// This macro is will initiate a drm_exec transaction with additional support for
// exhaustive eviction.
//

