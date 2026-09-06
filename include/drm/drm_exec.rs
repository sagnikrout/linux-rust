//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_exec.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT

//
// Dummy value used to initially enter the retry loop.
// internal use only.
//

//
// struct drm_exec - Execution context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exec {
//
// @flags: Flags to control locking behavior
//
    pub flags: u32,
//
// @ticket: WW ticket used for acquiring locks
//
    pub ticket: ww_acquire_ctx,
//
// @num_objects: number of objects locked
//
    pub num_objects: c_uint,
//
// @max_objects: maximum objects in array
//
    pub max_objects: c_uint,
//
// @objects: array of the locked objects
//
    pub objects: *mut drm_gem_object,
//
// @contended: contended GEM object we backed off for
//
    pub contended: *mut drm_gem_object,
//
// @prelocked: already locked GEM object due to contention
//
    pub prelocked: *mut drm_gem_object,
}

//
// drm_exec_obj() - Return the object for a give drm_exec index
// @exec: Pointer to the drm_exec context
// @index: The index.
//
// Return: Pointer to the locked object corresponding to @index if
// index is within the number of locked objects. NULL otherwise.
//
// Helper for drm_exec_for_each_locked_object(). Internal use only.

//
// drm_exec_for_each_locked_object - iterate over all the locked objects
// @exec: drm_exec object
// @obj: the current GEM object
//
// Iterate over all the locked GEM objects inside the drm_exec object.
//

// Helper for drm_exec_for_each_locked_object_reverse(). Internal use only.

//
// drm_exec_for_each_locked_object_reverse - iterate over all the locked
// objects in reverse locking order
// @exec: drm_exec object
// @obj: the current GEM object
//
// Iterate over all the locked GEM objects inside the drm_exec object in
// reverse locking order. Note that the internal index may wrap around,
// but that will be caught by drm_exec_obj(), returning a NULL object.
//

//
// drm_exec_until_all_locked - loop until all GEM objects are locked
// @exec: drm_exec object
//
// Core functionality of the drm_exec object. Loops until all GEM objects are
// locked and no more contention exists. At the beginning of the loop it is
// guaranteed that no GEM object is locked.
//
// A global label name drm_exec_retry is used, if you need to use more than one
// instance of this macro in the same function the label needs to be made local
// to the block with the __label__ keyword.
//

//
// drm_exec_retry_on_contention - restart the loop to grap all locks
// @exec: drm_exec object
//
// Control flow helper to continue when a contention was detected and we need to
// clean up and re-start the loop to prepare all GEM objects.
// The __drm_exec_loop check exists to prevent usage outside of an
// drm_exec_until_all_locked() loop.
//

//
// drm_exec_is_contended - check for contention
// @exec: drm_exec object
//
// Returns true if the drm_exec object has run into some contention while
// locking a GEM object and needs to clean up.
//
// drm_exec_retry() - Unconditionally restart the loop to grab all locks.
// @exec: drm_exec object
//
// Unconditionally retry the loop to lock all objects. For consistency,
// the exec object needs to be newly initialized.
// The __drm_exec_loop check exists to prevent usage outside of an
// drm_exec_until_all_locked() loop.
//

//
// drm_exec_ticket - return the ww_acquire_ctx for this exec context
// @exec: drm_exec object
//
// Return: Pointer to the ww_acquire_ctx embedded in @exec.
//
extern "C" {
    pub fn drm_exec_init(exec: *mut drm_exec, flags: u32, nr: unsigned);
}
extern "C" {
    pub fn drm_exec_fini(exec: *mut drm_exec);
}
extern "C" {
    pub fn drm_exec_cleanup(exec: *mut drm_exec) -> bool;
}
extern "C" {
    pub fn drm_exec_lock_obj(exec: *mut drm_exec, obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn drm_exec_unlock_obj(exec: *mut drm_exec, obj: *mut drm_gem_object);
}
