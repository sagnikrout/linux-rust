//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_fb_helper.h
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
// Copyright (c) 2006-2009 Red Hat Inc.
// Copyright (c) 2006-2008 Intel Corporation
// Copyright (c) 2007 Dave Airlie <airlied@linux.ie>
//
// DRM framebuffer helper functions
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
// Authors:
// Dave Airlie <airlied@linux.ie>
// Jesse Barnes <jesse.barnes@intel.com>
//

//
// struct drm_fb_helper_surface_size - describes fbdev size and scanout surface size
// @fb_width: fbdev width
// @fb_height: fbdev height
// @surface_width: scanout buffer width
// @surface_height: scanout buffer height
// @surface_bpp: scanout buffer bpp
// @surface_depth: scanout buffer depth
//
// Note that the scanout surface width/height may be larger than the fbdev
// width/height.  In case of multiple displays, the scanout surface is sized
// according to the largest width/height (so it is large enough for all CRTCs
// to scanout).  But the fbdev width/height is sized to the minimum width
// height of all the displays.  This ensures that fbcon fits on the smallest
// of the attached displays. fb_width/fb_height is used by
// drm_fb_helper_fill_info() to fill out the &fb_info.var structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_fb_helper_surface_size {
    pub fb_width: u32,
    pub fb_height: u32,
    pub surface_width: u32,
    pub surface_height: u32,
    pub surface_bpp: u32,
    pub surface_depth: u32,
}

//
// struct drm_fb_helper_funcs - driver callbacks for the fbdev emulation library
//
// Driver callbacks used by the fbdev emulation helper library.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_fb_helper_funcs {
//
// @fb_dirty:
//
// Driver callback to update the framebuffer memory. If set, fbdev
// emulation will invoke this callback in regular intervals after
// the framebuffer has been written.
//
// This callback is optional.
//
// Returns:
// 0 on success, or an error code otherwise.
//
    pub clip): *mut *mut *mut int (fb_dirty)(struct drm_fb_helper helper, struct drm_clip_rect,
//
// @fb_restore:
//
// Driver callback to restore internal fbdev state. If set, fbdev
// emulation will invoke this callback after restoring the display
// mode.
//
// Only for i915. Do not use in new code.
//
// TODO: Fix i915 to not require this callback.
//
    pub helper): *mut *mut void (fb_restore)(struct drm_fb_helper,
//
// @fb_set_suspend:
//
// Driver callback to suspend or resume, if set, fbdev emulation will
// invoke this callback during suspend and resume. Driver should call
// fb_set_suspend() from their implementation. If not set, fbdev
// emulation will invoke fb_set_suspend() directly.
//
// Only for i915. Do not use in new code.
//
// TODO: Fix i915 to not require this callback.
//
    pub suspend): *mut *mut *mut void (fb_set_suspend)(struct drm_fb_helper helper, bool,
}

//
// struct drm_fb_helper - main structure to emulate fbdev on top of KMS
// @fb: Scanout framebuffer object
// @dev: DRM device
// @funcs: driver callbacks for fb helper
// @info: emulated fbdev device info struct
// @pseudo_palette: fake palette of 16 colors
// @damage_clip: clip rectangle used with deferred_io to accumulate damage to
// the screen buffer
// @damage_lock: spinlock protecting @damage_clip
// @damage_work: worker used to flush the framebuffer
// @resume_work: worker used during resume if the console lock is already taken
//
// This is the main structure used by the fbdev helpers. Drivers supporting
// fbdev emulation should embedded this into their overall driver structure.
// Drivers must also fill out a &struct drm_fb_helper_funcs with a few
// operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_fb_helper {
//
// @client:
//
// DRM client used by the generic fbdev emulation.
//
    pub client: drm_client_dev,
//
// @buffer:
//
// Framebuffer used by the generic fbdev emulation.
//
    pub buffer: *mut drm_client_buffer,
    pub fb: *mut drm_framebuffer,
    pub dev: *mut drm_device,
    pub funcs: *const drm_fb_helper_funcs,
    pub info: *mut fb_info,
    pub pseudo_palette: [u32; 17],
    pub damage_clip: drm_clip_rect,
    pub damage_lock: spinlock_t,
    pub damage_work: work_struct,
    pub resume_work: work_struct,
//
// @lock:
//
// Top-level FBDEV helper lock. This protects all internal data
// structures and lists, such as @connector_info and @crtc_info.
//
// FIXME: fbdev emulation locking is a mess and long term we want to
// protect all helper internal state with this lock as well as reduce
// core KMS locking as much as possible.
//
    pub lock: mutex,
//
// @delayed_hotplug:
//
// A hotplug was received while fbdev wasn't in control of the DRM
// device, i.e. another KMS master was active. The output configuration
// needs to be reprobe when fbdev is in control again.
//
    pub delayed_hotplug: bool,
//
// @deferred_setup:
//
// If no outputs are connected (disconnected or unknown) the FB helper
// code will defer setup until at least one of the outputs shows up.
// This field keeps track of the status so that setup can be retried
// at every hotplug event until it succeeds eventually.
//
// Protected by @lock.
//
    pub deferred_setup: bool,
//
// @preferred_bpp:
//
// Temporary storage for the driver's preferred BPP setting passed to
// FB helper initialization. This needs to be tracked so that deferred
// FB helper setup can pass this on.
//
// See also: @deferred_setup
//
    pub preferred_bpp: c_int,

//
// @fbdefio:
//
// Temporary storage for the driver's FB deferred I/O handler. If the
// driver uses the DRM fbdev emulation layer, this is set by the core
// to a generic deferred I/O handler if a driver is preferring to use
// a shadow buffer.
//
    pub fbdefio: fb_deferred_io,

}

extern "C" {
    pub fn container_of(_arg: client, drm_fb_helper: struct, _arg: client) -> return;
}
//
// define DRM_FB_HELPER_DEFAULT_OPS - helper define for drm drivers
//
// Helper define to register default implementations of drm_fb_helper
// functions. To be used in struct fb_ops of drm drivers.
//

extern "C" {
    pub fn drm_fb_helper_unprepare(fb_helper: *mut drm_fb_helper);
}
extern "C" {
    pub fn drm_fb_helper_init(dev: *mut drm_device, helper: *mut drm_fb_helper) -> c_int;
}
extern "C" {
    pub fn drm_fb_helper_fini(helper: *mut drm_fb_helper);
}
extern "C" {
    pub fn drm_fb_helper_blank(blank: c_int, info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn drm_fb_helper_set_par(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn drm_fb_helper_unregister_info(fb_helper: *mut drm_fb_helper);
}
extern "C" {
    pub fn drm_fb_helper_damage_range(info: *mut fb_info, off: off_t, len: usize);
}
extern "C" {
    pub fn drm_fb_helper_damage_area(info: *mut fb_info, x: u32, y: u32, width: u32, height: u32);
}

extern "C" {
    pub fn drm_fb_helper_deferred_io(info: *mut fb_info, pagereflist: *mut list_head);
}

extern "C" {
    pub fn drm_fb_helper_set_suspend(fb_helper: *mut drm_fb_helper, suspend: bool);
}
extern "C" {
    pub fn drm_fb_helper_setcmap(cmap: *mut fb_cmap, info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn drm_fb_helper_hotplug_event(fb_helper: *mut drm_fb_helper) -> c_int;
}
extern "C" {
    pub fn drm_fb_helper_initial_config(fb_helper: *mut drm_fb_helper) -> c_int;
}

