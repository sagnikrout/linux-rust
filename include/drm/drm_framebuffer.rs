//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_framebuffer.h
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
// struct drm_framebuffer_funcs - framebuffer hooks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_framebuffer_funcs {
//
// @destroy:
//
// Clean up framebuffer resources, specifically also unreference the
// backing storage. The core guarantees to call this function for every
// framebuffer successfully created by calling
// &drm_mode_config_funcs.fb_create. Drivers must also call
// drm_framebuffer_cleanup() to release DRM core resources for this
// framebuffer.
//
    pub framebuffer): *mut *mut void (destroy)(struct drm_framebuffer,
//
// @create_handle:
//
// Create a buffer handle in the driver-specific buffer manager (either
// GEM or TTM) valid for the passed-in &struct drm_file. This is used by
// the core to implement the GETFB IOCTL, which returns (for
// sufficiently priviledged user) also a native buffer handle. This can
// be used for seamless transitions between modesetting clients by
// copying the current screen contents to a private buffer and blending
// between that and the new contents.
//
// GEM based drivers should call drm_gem_handle_create() to create the
// handle.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub handle): *mut c_uint,
//
// @dirty:
//
// Optional callback for the dirty fb IOCTL.
//
// Userspace can notify the driver via this callback that an area of the
// framebuffer has changed and should be flushed to the display
// hardware. This can also be used internally, e.g. by the fbdev
// emulation, though that's not the case currently.
//
// See documentation in drm_mode.h for the struct drm_mode_fb_dirty_cmd
// for more information as all the semantics and arguments have a one to
// one mapping on this function.
//
// Atomic drivers should use drm_atomic_helper_dirtyfb() to implement
// this hook.
//
// RETURNS:
//
// 0 on success or a negative error code on failure.
//
    pub num_clips): unsigned,
}

//
// struct drm_framebuffer - frame buffer object
//
// Note that the fb is refcounted for the benefit of driver internals,
// for example some hw, disabling a CRTC/plane is asynchronous, and
// scanout does not actually complete until the next vblank.  So some
// cleanup (like releasing the reference(s) on the backing GEM bo(s))
// should be deferred.  In cases like this, the driver would like to
// hold a ref to the fb even though it has already been removed from
// userspace perspective. See drm_framebuffer_get() and
// drm_framebuffer_put().
//
// The refcount is stored inside the mode object @base.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_framebuffer {
//
// @dev: DRM device this framebuffer belongs to
//
    pub dev: *mut drm_device,
//
// @head: Place on the &drm_mode_config.fb_list, access protected by
// &drm_mode_config.fb_lock.
//
    pub head: list_head,
//
// @base: base modeset object structure, contains the reference count.
//
    pub base: drm_mode_object,
//
// @comm: Name of the process allocating the fb, used for fb dumping.
//
    pub comm: [c_char; TASK_COMM_LEN],
//
// @format: framebuffer format information
//
    pub format: *const drm_format_info,
//
// @funcs: framebuffer vfunc table
//
    pub funcs: *const drm_framebuffer_funcs,
//
// @pitches: Line stride per buffer. For userspace created object this
// is copied from drm_mode_fb_cmd2.
//
    pub pitches: [c_uint; DRM_FORMAT_MAX_PLANES],
//
// @offsets: Offset from buffer start to the actual pixel data in bytes,
// per buffer. For userspace created object this is copied from
// drm_mode_fb_cmd2.
//
// Note that this is a linear offset and does not take into account
// tiling or buffer layout per @modifier. It is meant to be used when
// the actual pixel data for this framebuffer plane starts at an offset,
// e.g. when multiple planes are allocated within the same backing
// storage buffer object. For tiled layouts this generally means its
// @offsets must at least be tile-size aligned, but hardware often has
// stricter requirements.
//
// This should not be used to specifiy x/y pixel offsets into the buffer
// data (even for linear buffers). Specifying an x/y pixel offset is
// instead done through the source rectangle in &struct drm_plane_state.
//
    pub offsets: [c_uint; DRM_FORMAT_MAX_PLANES],
//
// @modifier: Data layout modifier. This is used to describe
// tiling, or also special layouts (like compression) of auxiliary
// buffers. For userspace created object this is copied from
// drm_mode_fb_cmd2.
//
    pub modifier: u64,
//
// @width: Logical width of the visible area of the framebuffer, in
// pixels.
//
    pub width: c_uint,
//
// @height: Logical height of the visible area of the framebuffer, in
// pixels.
//
    pub height: c_uint,
//
// @flags: Framebuffer flags like DRM_MODE_FB_INTERLACED or
// DRM_MODE_FB_MODIFIERS.
//
    pub flags: c_int,
//
// @internal_flags: Framebuffer flags like DRM_FRAMEBUFFER_HAS_HANDLE_REF.
//
    pub internal_flags: c_uint,
//
// @filp_head: Placed on &drm_file.fbs, protected by &drm_file.fbs_lock.
//
    pub filp_head: list_head,
//
// @obj: GEM objects backing the framebuffer, one per plane (optional).
//
// This is used by the GEM framebuffer helpers, see e.g.
// drm_gem_fb_create().
//
    pub obj: [*mut drm_gem_object; DRM_FORMAT_MAX_PLANES],
}

extern "C" {
    pub fn drm_framebuffer_remove(fb: *mut drm_framebuffer);
}
extern "C" {
    pub fn drm_framebuffer_cleanup(fb: *mut drm_framebuffer);
}
extern "C" {
    pub fn drm_framebuffer_unregister_private(fb: *mut drm_framebuffer);
}
//
// drm_framebuffer_get - acquire a framebuffer reference
// @fb: DRM framebuffer
//
// This function increments the framebuffer's reference count.
//
// drm_framebuffer_put - release a framebuffer reference
// @fb: DRM framebuffer
//
// This function decrements the framebuffer's reference count and frees the
// framebuffer if the reference count drops to zero.
//
// drm_framebuffer_read_refcount - read the framebuffer reference count.
// @fb: framebuffer
//
// This functions returns the framebuffer's reference count.
//
extern "C" {
    pub fn kref_read(_arg: &fb->base.refcount) -> return;
}
//
// drm_framebuffer_assign - store a reference to the fb
// @p: location to store framebuffer
// @fb: new framebuffer (maybe NULL)
//
// This functions sets the location to store a reference to the framebuffer,
// unreferencing the framebuffer that was previously stored in that location.
//
// p = fb;
//
// drm_for_each_fb - iterate over all framebuffers
// @fb: the loop cursor
// @dev: the DRM device
//
// Iterate over all framebuffers of @dev. User must hold
// &drm_mode_config.fb_lock.
//

//
// struct drm_afbc_framebuffer - a special afbc frame buffer object
//
// A derived class of struct drm_framebuffer, dedicated for afbc use cases.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_afbc_framebuffer {
//
// @base: base framebuffer structure.
//
    pub base: drm_framebuffer,
//
// @block_width: width of a single afbc block
//
    pub block_width: u32,
//
// @block_height: height of a single afbc block
//
    pub block_height: u32,
//
// @aligned_width: aligned frame buffer width
//
    pub aligned_width: u32,
//
// @aligned_height: aligned frame buffer height
//
    pub aligned_height: u32,
//
// @offset: offset of the first afbc header
//
    pub offset: u32,
//
// @afbc_size: minimum size of afbc buffer
//
    pub afbc_size: u32,
}

