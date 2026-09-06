//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/media-request.h
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
// Media device request objects
//
// Copyright 2018 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
// Copyright (C) 2018 Intel Corporation
//
// Author: Hans Verkuil <hverkuil@kernel.org>
// Author: Sakari Ailus <sakari.ailus@linux.intel.com>
//

//
// enum media_request_state - media request state
//
// @MEDIA_REQUEST_STATE_IDLE:		Idle
// @MEDIA_REQUEST_STATE_VALIDATING:	Validating the request, no state changes
// allowed
// @MEDIA_REQUEST_STATE_QUEUED:		Queued
// @MEDIA_REQUEST_STATE_COMPLETE:	Completed, the request is done
// @MEDIA_REQUEST_STATE_CLEANING:	Cleaning, the request is being re-inited
// @MEDIA_REQUEST_STATE_UPDATING:	The request is being updated, i.e.
// request objects are being added,
// modified or removed
// @NR_OF_MEDIA_REQUEST_STATE:		The number of media request states, used
// internally for sanity check purposes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_request_state {
    MEDIA_REQUEST_STATE_IDLE,
    MEDIA_REQUEST_STATE_VALIDATING,
    MEDIA_REQUEST_STATE_QUEUED,
    MEDIA_REQUEST_STATE_COMPLETE,
    MEDIA_REQUEST_STATE_CLEANING,
    MEDIA_REQUEST_STATE_UPDATING,
    NR_OF_MEDIA_REQUEST_STATE,
}

//
// struct media_request - Media device request
// @mdev: Media device this request belongs to
// @kref: Reference count
// @debug_str: Prefix for debug messages (process name:fd)
// @state: The state of the request
// @updating_count: count the number of request updates that are in progress
// @access_count: count the number of request accesses that are in progress
// @objects: List of @struct media_request_object request objects
// @num_incomplete_objects: The number of incomplete objects in the request
// @manual_completion: if true, then the request won't be marked as completed
// when @num_incomplete_objects reaches 0. Call media_request_manual_complete()
// to complete the request after @num_incomplete_objects == 0.
// @poll_wait: Wait queue for poll
// @lock: Serializes access to this struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_request {
    pub mdev: *mut media_device,
    pub kref: kref,
    pub 11]: char debug_str[TASK_COMM_LEN +,
    pub state: media_request_state,
    pub updating_count: c_uint,
    pub access_count: c_uint,
    pub objects: list_head,
    pub num_incomplete_objects: c_uint,
    pub manual_completion: bool,
    pub poll_wait: wait_queue_head_t,
    pub lock: spinlock_t,
}

//
// media_request_lock_for_access - Lock the request to access its objects
//
// @req: The media request
//
// Use before accessing a completed request. A reference to the request must
// be held during the access. This usually takes place automatically through
// a file handle. Use @media_request_unlock_for_access when done.
//
// media_request_unlock_for_access - Unlock a request previously locked for
// access
//
// @req: The media request
//
// Unlock a request that has previously been locked using
// @media_request_lock_for_access.
//
// media_request_lock_for_update - Lock the request for updating its objects
//
// @req: The media request
//
// Use before updating a request, i.e. adding, modifying or removing a request
// object in it. A reference to the request must be held during the update. This
// usually takes place automatically through a file handle. Use
// @media_request_unlock_for_update when done.
//
// media_request_unlock_for_update - Unlock a request previously locked for
// update
//
// @req: The media request
//
// Unlock a request that has previously been locked using
// @media_request_lock_for_update.
//
// media_request_get - Get the media request
//
// @req: The media request
//
// Get the media request.
//
// media_request_put - Put the media request
//
// @req: The media request
//
// Put the media request. The media request will be released
// when the refcount reaches 0.
//
extern "C" {
    pub fn media_request_put(req: *mut media_request);
}
//
// media_request_get_by_fd - Get a media request by fd
//
// @mdev: Media device this request belongs to
// @request_fd: The file descriptor of the request
//
// Get the request represented by @request_fd that is owned
// by the media device.
//
// Return a -EBADR error pointer if requests are not supported
// by this driver. Return -EINVAL if the request was not found.
// Return the pointer to the request if found: the caller will
// have to call @media_request_put when it finished using the
// request.
//
// media_request_alloc - Allocate the media request
//
// @mdev: Media device this request belongs to
// @alloc_fd: Store the request's file descriptor in this int
//
// Allocated the media request and put the fd in @alloc_fd.
//
// media_request_mark_manual_completion - Enable manual completion
//
// @req: The request
//
// Mark that the request has to be manually completed by calling
// media_request_manual_complete().
//
// This function shall be called in the req_queue callback.
//
// media_request_manual_complete - Mark the request as completed
//
// @req: The request
//
// This function completes a request that was marked for manual completion by an
// earlier call to media_request_mark_manual_completion(). The request's
// @manual_completion field is reset to false.
//
// All objects contained in the request must have been completed previously. It
// is an error to call this function otherwise. If such an error occurred, the
// function will WARN and the object completion will be delayed until
// @num_incomplete_objects is 0.
//
extern "C" {
    pub fn media_request_manual_complete(req: *mut media_request);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EBADR) -> return;
}

//
// struct media_request_object_ops - Media request object operations
// @prepare: Validate and prepare the request object, optional.
// @unprepare: Unprepare the request object, optional.
// @queue: Queue the request object, optional.
// @unbind: Unbind the request object, optional.
// @release: Release the request object, required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_request_object_ops {
    pub object): *mut *mut int (prepare)(struct media_request_object,
    pub object): *mut *mut void (unprepare)(struct media_request_object,
    pub object): *mut *mut void (queue)(struct media_request_object,
    pub object): *mut *mut void (unbind)(struct media_request_object,
    pub object): *mut *mut void (release)(struct media_request_object,
}

//
// struct media_request_object - An opaque object that belongs to a media
// request
//
// @mdev: Media device this object belongs to
// @ops: object's operations
// @priv: object's priv pointer
// @req: the request this object belongs to (can be NULL)
// @list: List entry of the object for @struct media_request
// @kref: Reference count of the object, acquire before releasing req->lock
// @completed: If true, then this object was completed.
//
// An object related to the request. This struct is always embedded in
// another struct that contains the actual data for this request object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_request_object {
    pub mdev: *mut media_device,
    pub ops: *const media_request_object_ops,
    pub priv: *mut c_void,
    pub req: *mut media_request,
    pub list: list_head,
    pub kref: kref,
    pub completed: bool,
}

//
// media_request_object_get - Get a media request object
//
// @obj: The object
//
// Get a media request object.
//
// media_request_object_put - Put a media request object
//
// @obj: The object
//
// Put a media request object. Once all references are gone, the
// object's memory is released.
//
extern "C" {
    pub fn media_request_object_put(obj: *mut media_request_object);
}
//
// media_request_object_find - Find an object in a request
//
// @req: The media request
// @ops: Find an object with this ops value
// @priv: Find an object with this priv value
//
// Both @ops and @priv must be non-NULL.
//
// Returns the object pointer or NULL if not found. The caller must
// call media_request_object_put() once it finished using the object.
//
// Since this function needs to walk the list of objects it takes
// the @req->lock spin lock to make this safe.
//
// media_request_object_init - Initialise a media request object
//
// @obj: The object
//
// Initialise a media request object. The object will be released using the
// release callback of the ops once it has no references (this function
// initialises references to one).
//
extern "C" {
    pub fn media_request_object_init(obj: *mut media_request_object);
}
//
// media_request_object_bind - Bind a media request object to a request
//
// @req: The media request
// @ops: The object ops for this object
// @priv: A driver-specific priv pointer associated with this object
// @is_buffer: Set to true if the object is a buffer object.
// @obj: The object
//
// Bind this object to the request and set the ops and priv values of
// the object so it can be found later with media_request_object_find().
//
// Every bound object must be unbound or completed by the kernel at some
// point in time, otherwise the request will never complete. When the
// request is released all completed objects will be unbound by the
// request core code.
//
// Buffer objects will be added to the end of the request's object
// list, non-buffer objects will be added to the front of the list.
// This ensures that all buffer objects are at the end of the list
// and that all non-buffer objects that they depend on are processed
// first.
//
// media_request_object_unbind - Unbind a media request object
//
// @obj: The object
//
// Unbind the media request object from the request.
//
extern "C" {
    pub fn media_request_object_unbind(obj: *mut media_request_object);
}
//
// media_request_object_complete - Mark the media request object as complete
//
// @obj: The object
//
// Mark the media request object as complete. Only bound objects can
// be completed.
//
extern "C" {
    pub fn media_request_object_complete(obj: *mut media_request_object);
}

