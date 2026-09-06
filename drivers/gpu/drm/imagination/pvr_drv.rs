//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_drv.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// Driver interface version:
// - 1.0: Initial interface
//
pub const PVR_DRIVER_MAJOR: c_int = 1;
pub const PVR_DRIVER_MINOR: c_int = 0;
pub const PVR_DRIVER_PATCHLEVEL: c_int = 0;
extern "C" {
    pub fn pvr_get_uobj(usr_ptr: u64, usr_size: u32, min_size: u32, obj_size: u32, out: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pvr_set_uobj(usr_ptr: u64, usr_size: u32, min_size: u32, obj_size: u32, in: *const c_void) -> c_int;
}

// NOLINTBEGIN(bugprone-macro-parentheses)

// NOLINTEND(bugprone-macro-parentheses)
//
// DOC: PVR user objects.
//
// Macros used to aid copying structured and array data to and from
// userspace. Objects can differ in size, provided the minimum size
// allowed is specified (using the last mandatory field in the struct).
// All types used with PVR_UOBJ_GET/SET macros must be listed here under
// PVR_UOBJ_MIN_SIZE, with the last mandatory struct field specified.
//
// PVR_UOBJ_MIN_SIZE() - Fetch the minimum copy size of a compatible type object.
// @_obj_name: The name of the object. Cannot be a typename - this is deduced.
//
// This cannot fail. Using the macro with an incompatible type will result in a
// compiler error.
//
// To add compatibility for a type, list it within the macro in an orderly
// fashion. The second argument is the name of the last mandatory field of the
// struct type, which is used to calculate the size. See also PVR_UOBJ_DECL().
//
// Return: The minimum copy size.
//

//
// PVR_UOBJ_GET() - Copies from _src_usr_ptr to &_dest_obj.
// @_dest_obj: The destination container object in kernel space.
// @_usr_size: The size of the source container in user space.
// @_src_usr_ptr: __u64 raw pointer to the source container in user space.
//
// Return: Error code. See pvr_get_uobj().
//

//
// PVR_UOBJ_SET() - Copies from &_src_obj to _dest_usr_ptr.
// @_dest_usr_ptr: __u64 raw pointer to the destination container in user space.
// @_usr_size: The size of the destination container in user space.
// @_src_obj: The source container object in kernel space.
//
// Return: Error code. See pvr_set_uobj().
//

//
// PVR_UOBJ_GET_ARRAY() - Copies from @_src_drm_pvr_obj_array.array to
// alloced memory and returns a pointer in _dest_array.
// @_dest_array: The destination C array object in kernel space.
// @_src_drm_pvr_obj_array: The &struct drm_pvr_obj_array containing a __u64 raw
// pointer to the source C array in user space and the size of each array
// element in user space (the 'stride').
//
// Return: Error code. See pvr_get_uobj_array().
//

//
// PVR_UOBJ_SET_ARRAY() - Copies from _src_array to @_dest_drm_pvr_obj_array.array.
// @_dest_drm_pvr_obj_array: The &struct drm_pvr_obj_array containing a __u64 raw
// pointer to the destination C array in user space and the size of each array
// element in user space (the 'stride').
// @_src_array: The source C array object in kernel space.
//
// Return: Error code. See pvr_set_uobj_array().
//

