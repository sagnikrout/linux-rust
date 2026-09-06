//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_file_private.h
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
// Copyright © 2021 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_file_private {
    pub i915: *mut drm_i915_private,
    pub file: *mut drm_file,
    pub rcu: rcu_head,
}

// @proto_context_lock: Guards all struct i915_gem_proto_context
// operations
//
// This not only guards @proto_context_xa, but is always held
// whenever we manipulate any struct i915_gem_proto_context,
// including finalizing it on first actual use of the GEM context.
//
// See i915_gem_proto_context.
//
// @proto_context_xa: xarray of struct i915_gem_proto_context
//
// Historically, the context uAPI allowed for two methods of
// setting context parameters: SET_CONTEXT_PARAM and
// CONTEXT_CREATE_EXT_SETPARAM.  The former is allowed to be called
// at any time while the later happens as part of
// GEM_CONTEXT_CREATE.  Everything settable via one was settable
// via the other.  While some params are fairly simple and setting
// them on a live context is harmless such as the context priority,
// others are far trickier such as the VM or the set of engines.
// In order to swap out the VM, for instance, we have to delay
// until all current in-flight work is complete, swap in the new
// VM, and then continue.  This leads to a plethora of potential
// race conditions we'd really rather avoid.
//
// We have since disallowed setting these more complex parameters
// on active contexts.  This works by delaying the creation of the
// actual context until after the client is done configuring it
// with SET_CONTEXT_PARAM.  From the perspective of the client, it
// has the same u32 context ID the whole time.  From the
// perspective of i915, however, it's a struct i915_gem_proto_context
// right up until the point where we attempt to do something which
// the proto-context can't handle.  Then the struct i915_gem_context
// gets created.
//
// This is accomplished via a little xarray dance.  When
// GEM_CONTEXT_CREATE is called, we create a struct
// i915_gem_proto_context, reserve a slot in @context_xa but leave
// it NULL, and place the proto-context in the corresponding slot
// in @proto_context_xa.  Then, in i915_gem_context_lookup(), we
// first check @context_xa.  If it's there, we return the struct
// i915_gem_context and we're done.  If it's not, we look in
// @proto_context_xa and, if we find it there, we create the actual
// context and kill the proto-context.
//
// In order for this dance to work properly, everything which ever
// touches a struct i915_gem_proto_context is guarded by
// @proto_context_lock, including context creation.  Yes, this
// means context creation now takes a giant global lock but it
// can't really be helped and that should never be on any driver's
// fast-path anyway.
//
// @context_xa: xarray of fully created i915_gem_context
//
// Write access to this xarray is guarded by @proto_context_lock.
// Otherwise, writers may race with finalize_create_context_locked().
//
// See @proto_context_xa.
//
// Every context ban increments per client ban score. Also
// hangs in short succession increments ban score. If ban threshold
// is reached, client is considered banned and submitting more work
// will fail. This is a stop gap measure to limit the badly behaving
// clients access to gpu. Note that unbannable contexts never increment
// the client ban score.
//
pub const I915_CLIENT_SCORE_HANG_FAST: c_int = 1;

pub const I915_CLIENT_SCORE_CONTEXT_BAN: c_int = 3;
pub const I915_CLIENT_SCORE_BANNED: c_int = 9;
// ban_score: Accumulated score of all ctx bans and fast hangs.
