//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/ttm_object.h
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
// Copyright (c) 2006-2023 VMware, Inc., Palo Alto, CA., USA
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Thomas Hellstrom <thellstrom-at-vmware-dot-com>
//
// @file ttm_object.h
//
// Base- and reference object implementation for the various
// ttm objects. Implements reference counting, minimal security checks
// and release on file close.
//

//
// enum ttm_object_type
//
// One entry per ttm object type.
// Device-specific types should use the
// ttm_driver_typex types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttm_object_type {
    ttm_fence_type,
    ttm_lock_type,
    ttm_prime_type,
    ttm_driver_type0 = 256,
    ttm_driver_type1,
    ttm_driver_type2,
    ttm_driver_type3,
    ttm_driver_type4,
    ttm_driver_type5
}

//
// struct ttm_base_object
//
// @hash: hash entry for the per-device object hash.
// @type: derived type this object is base class for.
// @shareable: Other ttm_object_files can access this object.
//
// @tfile: Pointer to ttm_object_file of the creator.
// NULL if the object was not created by a user request.
// (kernel object).
//
// @refcount: Number of references to this object, not
// including the hash entry. A reference to a base object can
// only be held by a ref object.
//
// @refcount_release: A function to be called when there are
// no more references to this object. This function should
// destroy the object (or make sure destruction eventually happens),
// and when it is called, the object has
// already been taken out of the per-device hash. The parameter
// "base" should be set to NULL by the function.
//
// @ref_obj_release: A function to be called when a reference object
// with another ttm_ref_type than TTM_REF_USAGE is deleted.
// This function may, for example, release a lock held by a user-space
// process.
//
// This struct is intended to be used as a base struct for objects that
// are visible to user-space. It provides a global name, race-safe
// access and refcounting, minimal access control and hooks for unref actions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_base_object {
    pub rhead: rcu_head,
    pub tfile: *mut ttm_object_file,
    pub refcount: kref,
    pub base): *mut *mut void (refcount_release) (struct ttm_base_object,
    pub handle: u64,
    pub object_type: ttm_object_type,
    pub shareable: u32,
}

//
// struct ttm_prime_object - Modified base object that is prime-aware
//
// @base: struct ttm_base_object that we derive from
// @mutex: Mutex protecting the @dma_buf member.
// @size: Size of the dma_buf associated with this object
// @real_type: Type of the underlying object. Needed since we're setting
// the value of @base::object_type to ttm_prime_type
// @dma_buf: Non ref-coutned pointer to a struct dma_buf created from this
// object.
// @refcount_release: The underlying object's release method. Needed since
// we set @base::refcount_release to our own release method.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_prime_object {
    pub base: ttm_base_object,
    pub mutex: mutex,
    pub size: usize,
    pub real_type: ttm_object_type,
    pub dma_buf: *mut dma_buf,
    pub ): *mut *mut void (refcount_release) (struct ttm_base_object,
}

//
// ttm_base_object_init
//
// @tfile: Pointer to a struct ttm_object_file.
// @base: The struct ttm_base_object to initialize.
// @shareable: This object is shareable with other applications.
// (different @tfile pointers.)
// @type: The object type.
// @refcount_release: See the struct ttm_base_object description.
// @ref_obj_release: See the struct ttm_base_object description.
//
// Initializes a struct ttm_base_object.
//
// ));
//
// ttm_base_object_lookup
//
// @tfile: Pointer to a struct ttm_object_file.
// @key: Hash key
//
// Looks up a struct ttm_base_object with the key @key.
//
// tfile, uint64_t key);
//
// ttm_base_object_lookup_for_ref
//
// @tdev: Pointer to a struct ttm_object_device.
// @key: Hash key
//
// Looks up a struct ttm_base_object with the key @key.
// This function should only be used when the struct tfile associated with the
// caller doesn't yet have a reference to the base object.
//
// ttm_base_object_unref
//
// @p_base: Pointer to a pointer referencing a struct ttm_base_object.
//
// Decrements the base object refcount and clears the pointer pointed to by
// p_base.
//
extern "C" {
    pub fn ttm_base_object_unref(p_base: *mut ttm_base_object);
}
//
// ttm_ref_object_add.
//
// @tfile: A struct ttm_object_file representing the application owning the
// ref_object.
// @base: The base object to reference.
// @ref_type: The type of reference.
// @existed: Upon completion, indicates that an identical reference object
// already existed, and the refcount was upped on that object instead.
// @require_existed: Fail with -EPERM if an identical ref object didn't
// already exist.
//
// Checks that the base object is shareable and adds a ref object to it.
//
// Adding a ref object to a base object is basically like referencing the
// base object, but a user-space application holds the reference. When the
// file corresponding to @tfile is closed, all its reference objects are
// deleted. A reference object can have different types depending on what
// it's intended for. It can be refcounting to prevent object destruction,
// When user-space takes a lock, it can add a ref object to that lock to
// make sure the lock is released if the application dies. A ref object
// will hold a single reference on a base object.
//
// ttm_ref_object_base_unref
//
// @key: Key representing the base object.
// @ref_type: Ref type of the ref object to be dereferenced.
//
// Unreference a ref object with type @ref_type
// on the base object identified by @key. If there are no duplicate
// references, the ref object will be destroyed and the base object
// will be unreferenced.
//
// ttm_object_file_init - initialize a struct ttm_object file
//
// @tdev: A struct ttm_object device this file is initialized on.
//
// This is typically called by the file_ops::open function.
//
// tdev);
//
// ttm_object_file_release - release data held by a ttm_object_file
//
// @p_tfile: Pointer to pointer to the ttm_object_file object to release.
// *p_tfile will be set to NULL by this function.
//
// Releases all data associated by a ttm_object_file.
// Typically called from file_ops::release. The caller must
// ensure that there are no concurrent users of tfile.
//
extern "C" {
    pub fn ttm_object_file_release(p_tfile: *mut ttm_object_file);
}
//
// ttm_object device init - initialize a struct ttm_object_device
//
// @ops: DMA buf ops for prime objects of this device.
//
// This function is typically called on device initialization to prepare
// data structures needed for ttm base and ref objects.
//
// ttm_object_device_release - release data held by a ttm_object_device
//
// @p_tdev: Pointer to pointer to the ttm_object_device object to release.
// *p_tdev will be set to NULL by this function.
//
// Releases all data associated by a ttm_object_device.
// Typically called from driver::unload before the destruction of the
// device private data structure.
//
extern "C" {
    pub fn ttm_object_device_release(p_tdev: *mut ttm_object_device);
}

extern "C" {
    pub fn ttm_bo_wait_ctx(_arg: bo, _arg: &ctx) -> return;
}
