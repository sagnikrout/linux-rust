//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/uverbs_types.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2017, Mellanox Technologies inc.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_lookup_mode {
    UVERBS_LOOKUP_READ,
    UVERBS_LOOKUP_WRITE,
//
// Destroy is like LOOKUP_WRITE, except that the uobject is not
// locked.  uobj_destroy is used to convert a LOOKUP_DESTROY lock into
// a LOOKUP_WRITE lock.
//
    UVERBS_LOOKUP_DESTROY,
}

//
// The following sequences are valid:
// Success flow:
// alloc_begin
// alloc_commit
// [..]
// Access flow:
// lookup_get(exclusive=false) & uverbs_try_lock_object
// lookup_put(exclusive=false) via rdma_lookup_put_uobject
// Destruction flow:
// lookup_get(exclusive=true) & uverbs_try_lock_object
// remove_commit
// remove_handle (optional)
// lookup_put(exclusive=true) via rdma_lookup_put_uobject
//
// Allocate Error flow #1
// alloc_begin
// alloc_abort
// Allocate Error flow #2
// alloc_begin
// remove_commit
// alloc_abort
// Allocate Error flow #3
// alloc_begin
// alloc_commit (fails)
// remove_commit
// alloc_abort
//
// In all cases the caller must hold the ufile kref until alloc_commit or
// alloc_abort returns.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_obj_type_class {
    pub attrs): *mut uverbs_attr_bundle,
// This consumes the kref on uobj
    pub uobj): *mut *mut void (alloc_commit)(struct ib_uobject,
// This does not consume the kref on uobj
    pub uobj): *mut *mut void (alloc_abort)(struct ib_uobject,
    pub mode): rdma_lookup_mode,
    pub mode): *mut *mut *mut void (lookup_put)(struct ib_uobject uobj, enum rdma_lookup_mode,
// This does not consume the kref on uobj
    pub attrs): *mut uverbs_attr_bundle,
    pub uobj): *mut *mut void (remove_handle)(struct ib_uobject,
    pub obj_new): *mut ib_uobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_obj_type {
    pub type_class: *const *const uverbs_obj_type_class,
    pub obj_size: usize,
}

//
// Objects type classes which support a detach state (object is still alive but
// it's not attached to any context need to make sure:
// (a) no call through to a driver after a detach is called
// (b) detach isn't called concurrently with context_cleanup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_obj_idr_type {
//
// In idr based objects, uverbs_obj_type_class points to a generic
// idr operations. In order to specialize the underlying types (e.g. CQ,
// QPs, etc.), we add destroy_object specific callbacks.
//
    pub type: uverbs_obj_type,
// Free driver resources from the uobject, make the driver uncallable,
// and move the uobject to the detached state. If the object was
// destroyed by the user's request, a failure should leave the uobject
// completely unchanged.
//
    pub attrs): *mut uverbs_attr_bundle,
}

//
// uverbs_uobject_get is called in order to increase the reference count on
// an uobject. This is useful when a handler wants to keep the uobject's memory
// alive, regardless if this uobject is still alive in the context's objects
// repository. Objects are put via uverbs_uobject_put.
//
extern "C" {
    pub fn uverbs_uobject_put(uobject: *mut ib_uobject);
}
extern "C" {
    pub fn uverbs_try_lock_object(uobj: *mut ib_uobject, mode: rdma_lookup_mode) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_obj_fd_type {
//
// In fd based objects, uverbs_obj_type_ops points to generic
// fd operations. In order to specialize the underlying types (e.g.
// completion_channel), we use fops, name and flags for fd creation.
// destroy_object is called when the uobject is to be destroyed,
// because the driver is removed or the FD is closed.
//
    pub type: uverbs_obj_type,
    pub why): rdma_remove_reason,
    pub uobj): *mut *mut void (release_cleanup)(struct ib_uobject,
    pub fops: *const file_operations,
    pub name: *const c_char,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_file {
    pub ref: kref,
    pub device: *mut ib_uverbs_device,
    pub ucontext_lock: mutex,
//
// ucontext must be accessed via ib_uverbs_get_ucontext() or with
// ucontext_lock held
//
    pub ucontext: *mut ib_ucontext,
    pub default_async_file: *mut ib_uverbs_async_event_file,
    pub list: list_head,
//
// To access the uobjects list hw_destroy_rwsem must be held for write
// OR hw_destroy_rwsem held for read AND uobjects_lock held.
// hw_destroy_rwsem should be called across any destruction of the HW
// object of an associated uobject.
//
    pub hw_destroy_rwsem: rw_semaphore,
    pub uobjects_lock: spinlock_t,
    pub uobjects: list_head,
    pub umap_lock: mutex,
    pub umaps: list_head,
    pub disassociate_page: *mut page,
    pub idr: xarray,
    pub disassociation_lock: mutex,
}

extern "C" {
    pub fn uverbs_uobject_fd_release(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn uverbs_uobject_release(uobj: *mut ib_uobject) -> c_int;
}

