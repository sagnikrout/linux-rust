//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_mode_object.h
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
// Copyright (c) 2016 Intel Corporation
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

//
// struct drm_mode_object - base structure for modeset objects
// @id: userspace visible identifier
// @type: type of the object, one of DRM_MODE_OBJECT\_\
// @properties: properties attached to this object, including values
// @refcount: reference count for objects with dynamic lifetime
// @free_cb: free function callback, only set for objects with dynamic lifetime
//
// Base structure for modeset objects visible to userspace. Objects can be
// looked up using drm_mode_object_find(). Besides basic uapi interface
// properties like @id and @type it provides two services:
//
// - It tracks attached properties and their values. This is used by &drm_crtc,
// &drm_plane and &drm_connector. Properties are attached by calling
// drm_object_attach_property() before the object is visible to userspace.
//
// - For objects with dynamic lifetimes (as indicated by a non-NULL @free_cb) it
// provides reference counting through drm_mode_object_get() and
// drm_mode_object_put(). This is used by &drm_framebuffer, &drm_connector
// and &drm_property_blob. These objects provide specialized reference
// counting wrappers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_object {
    pub id: u32,
    pub type: u32,
    pub properties: *mut drm_object_properties,
    pub refcount: kref,
    pub kref): *mut *mut void (free_cb)(struct kref,
}

pub const DRM_OBJECT_MAX_PROPERTY: c_int = 64;
//
// struct drm_object_properties - property tracking for &drm_mode_object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_object_properties {
//
// @count: number of valid properties, must be less than or equal to
// DRM_OBJECT_MAX_PROPERTY.
//
    pub count: c_int,
//
// @properties: Array of pointers to &drm_property.
//
// NOTE: if we ever start dynamically destroying properties (ie.
// not at drm_mode_config_cleanup() time), then we'd have to do
// a better job of detaching property from mode objects to avoid
// dangling property pointers:
//
    pub properties: [*mut drm_property; DRM_OBJECT_MAX_PROPERTY],
//
// @values: Array to store the property values, matching @properties. Do
// not read/write values directly, but use
// drm_object_property_get_value() and drm_object_property_set_value().
//
// Note that atomic drivers do not store mutable properties in this
// array, but only the decoded values in the corresponding state
// structure. The decoding is done using the &drm_crtc.atomic_get_property and
// &drm_crtc.atomic_set_property hooks for &struct drm_crtc. For
// &struct drm_plane the hooks are &drm_plane_funcs.atomic_get_property and
// &drm_plane_funcs.atomic_set_property. And for &struct drm_connector
// the hooks are &drm_connector_funcs.atomic_get_property and
// &drm_connector_funcs.atomic_set_property .
//
// Hence atomic drivers should not use drm_object_property_set_value()
// and drm_object_property_get_value() on mutable objects, i.e. those
// without the DRM_MODE_PROP_IMMUTABLE flag set.
//
// For atomic drivers the default value of properties is stored in this
// array, so drm_object_property_get_default_value can be used to
// retrieve it.
//
    pub values: [u64; DRM_OBJECT_MAX_PROPERTY],
}

// Avoid boilerplate.  I'm tired of typing.

extern "C" {
    pub fn drm_mode_object_get(obj: *mut drm_mode_object);
}
extern "C" {
    pub fn drm_mode_object_put(obj: *mut drm_mode_object);
}
extern "C" {
    pub fn drm_mode_object_lease_required(type: u32) -> bool;
}
