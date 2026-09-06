//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_file.h
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
// Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
// Copyright (c) 2009-2010, Code Aurora Forum.
// All rights reserved.
//
// Author: Rickard E. (Rik) Faith <faith@valinux.com>
// Author: Gareth Hughes <gareth@valinux.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// FIXME: Not sure we want to have drm_minor here in the end, but to avoid
// header include loops we need it here for now.
//
// Note that the values of this enum are ABI (it determines
// /dev/dri/renderD* numbers).
//
// Setting DRM_MINOR_ACCEL to 32 gives enough space for more drm minors to
// be implemented before we hit any future
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_minor_type {
    DRM_MINOR_PRIMARY = 0,
    DRM_MINOR_CONTROL = 1,
    DRM_MINOR_RENDER = 2,
    DRM_MINOR_ACCEL = 32,
}

//
// struct drm_minor - DRM device minor structure
//
// This structure represents a DRM minor number for device nodes in /dev.
// Entirely opaque to drivers and should never be inspected directly by drivers.
// Drivers instead should only interact with &struct drm_file and of course
// &struct drm_device, which is also where driver-private data and resources can
// be attached to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_minor {
// private:
    pub /: *mut *mut int index; / Minor device number,
    pub /: *mut *mut int type; / Control or render or accel,
    pub /: *mut *mut *mut device kdev; / Linux device,
    pub dev: *mut drm_device,
    pub debugfs_symlink: *mut dentry,
    pub debugfs_root: *mut dentry,
}

//
// struct drm_pending_event - Event queued up for userspace to read
//
// This represents a DRM event. Drivers can use this as a generic completion
// mechanism, which supports kernel-internal &struct completion, &struct dma_fence
// and also the DRM-specific &struct drm_event delivery mechanism.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pending_event {
//
// @completion:
//
// Optional pointer to a kernel internal completion signalled when
// drm_send_event() is called, useful to internally synchronize with
// nonblocking operations.
//
    pub completion: *mut completion,
//
// @completion_release:
//
// Optional callback currently only used by the atomic modeset helpers
// to clean up the reference count for the structure @completion is
// stored in.
//
    pub completion): *mut *mut void (completion_release)(struct completion,
//
// @event:
//
// Pointer to the actual event that should be sent to userspace to be
// read using drm_read(). Can be optional, since nowadays events are
// also used to signal kernel internal threads with @completion or DMA
// transactions using @fence.
//
    pub event: *mut drm_event,
//
// @fence:
//
// Optional DMA fence to unblock other hardware transactions which
// depend upon the nonblocking DRM operation this event represents.
//
    pub fence: *mut dma_fence,
//
// @file_priv:
//
// &struct drm_file where @event should be delivered to. Only set when
// @event is set.
//
    pub file_priv: *mut drm_file,
//
// @link:
//
// Double-linked list to keep track of this event. Can be used by the
// driver up to the point when it calls drm_send_event(), after that
// this list entry is owned by the core for its own book-keeping.
//
    pub link: list_head,
//
// @pending_link:
//
// Entry on &drm_file.pending_event_list, to keep track of all pending
// events for @file_priv, to allow correct unwinding of them when
// userspace closes the file before the event is delivered.
//
    pub pending_link: list_head,
}

//
// struct drm_file - DRM file private data
//
// This structure tracks DRM state per open file descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_file {
//
// @authenticated:
//
// Whether the client is allowed to submit rendering, which for legacy
// nodes means it must be authenticated.
//
// See also the :ref:`section on primary nodes and authentication
// <drm_primary_node>`.
//
    pub authenticated: bool,
//
// @stereo_allowed:
//
// True when the client has asked us to expose stereo 3D mode flags.
//
    pub stereo_allowed: bool,
//
// @universal_planes:
//
// True if client understands CRTC primary planes and cursor planes
// in the plane list. Automatically set when @atomic is set.
//
    pub universal_planes: bool,
// @atomic: True if client understands atomic properties.
    pub atomic: bool,
//
// @aspect_ratio_allowed:
//
// True, if client can handle picture aspect ratios, and has requested
// to pass this information along with the mode.
//
    pub aspect_ratio_allowed: bool,
//
// @writeback_connectors:
//
// True if client understands writeback connectors
//
    pub writeback_connectors: bool,
//
// @plane_color_pipeline:
//
// True if client understands plane color pipelines
//
    pub plane_color_pipeline: bool,
//
// @was_master:
//
// This client has or had, master capability. Protected by struct
// &drm_device.master_mutex.
//
// This is used to ensure that CAP_SYS_ADMIN is not enforced, if the
// client is or was master in the past.
//
    pub was_master: bool,
//
// @is_master:
//
// This client is the creator of @master. Protected by struct
// &drm_device.master_mutex.
//
// See also the :ref:`section on primary nodes and authentication
// <drm_primary_node>`.
//
    pub is_master: bool,
//
// @supports_virtualized_cursor_plane:
//
// This client is capable of handling the cursor plane with the
// restrictions imposed on it by the virtualized drivers.
//
// This implies that the cursor plane has to behave like a cursor
// i.e. track cursor movement. It also requires setting of the
// hotspot properties by the client on the cursor plane.
//
    pub supports_virtualized_cursor_plane: bool,
//
// @master:
//
// Master this node is currently associated with. Protected by struct
// &drm_device.master_mutex, and serialized by @master_lookup_lock.
//
// Only relevant if drm_is_primary_client() returns true. Note that
// this only matches &drm_device.master if the master is the currently
// active one.
//
// To update @master, both &drm_device.master_mutex and
// @master_lookup_lock need to be held, therefore holding either of
// them is safe and enough for the read side.
//
// When dereferencing this pointer, either hold struct
// &drm_device.master_mutex for the duration of the pointer's use, or
// use drm_file_get_master() if struct &drm_device.master_mutex is not
// currently held and there is no other need to hold it. This prevents
// @master from being freed during use.
//
// See also @authentication and @is_master and the :ref:`section on
// primary nodes and authentication <drm_primary_node>`.
//
    pub master: *mut drm_master,
// @master_lookup_lock: Serializes @master.
    pub master_lookup_lock: spinlock_t,
//
// @pid: Process that is using this file.
//
// Must only be dereferenced under a rcu_read_lock or equivalent.
//
// Updates are guarded with dev->filelist_mutex and reference must be
// dropped after a RCU grace period to accommodate lockless readers.
//
    pub pid: *mut pid __rcu,
// @client_id: A unique id for fdinfo
    pub client_id: u64,
// @magic: Authentication magic, see @authenticated.
    pub magic: drm_magic_t,
//
// @lhead:
//
// List of all open files of a DRM device, linked into
// &drm_device.filelist. Protected by &drm_device.filelist_mutex.
//
    pub lhead: list_head,
// @minor: &struct drm_minor for this file.
    pub minor: *mut drm_minor,
//
// @object_idr:
//
// Mapping of mm object handles to object pointers. Used by the GEM
// subsystem. Protected by @table_lock.
//
// Note that allocated entries might be NULL as a transient state when
// creating or deleting a handle.
//
    pub object_idr: idr,
// @table_lock: Protects @object_idr.
    pub table_lock: spinlock_t,
// @syncobj_xa: Mapping of sync object handles to object pointers.
    pub syncobj_xa: xarray,
// @filp: Pointer to the core file structure.
    pub filp: *mut file,
//
// @driver_priv:
//
// Optional pointer for driver private data. Can be allocated in
// &drm_driver.open and should be freed in &drm_driver.postclose.
//
    pub driver_priv: *mut c_void,
//
// @fbs:
//
// List of &struct drm_framebuffer associated with this file, using the
// &drm_framebuffer.filp_head entry.
//
// Protected by @fbs_lock. Note that the @fbs list holds a reference on
// the framebuffer object to prevent it from untimely disappearing.
//
    pub fbs: list_head,
// @fbs_lock: Protects @fbs.
    pub fbs_lock: mutex,
//
// @blobs:
//
// User-created blob properties; this retains a reference on the
// property.
//
// Protected by @drm_mode_config.blob_lock;
//
    pub blobs: list_head,
// @event_wait: Waitqueue for new events added to @event_list.
    pub event_wait: wait_queue_head_t,
//
// @pending_event_list:
//
// List of pending &struct drm_pending_event, used to clean up pending
// events in case this file gets closed before the event is signalled.
// Uses the &drm_pending_event.pending_link entry.
//
// Protect by &drm_device.event_lock.
//
    pub pending_event_list: list_head,
//
// @event_list:
//
// List of &struct drm_pending_event, ready for delivery to userspace
// through drm_read(). Uses the &drm_pending_event.link entry.
//
// Protect by &drm_device.event_lock.
//
    pub event_list: list_head,
//
// @event_space:
//
// Available event space to prevent userspace from
// exhausting kernel memory. Currently limited to the fairly arbitrary
// value of 4KB.
//
    pub event_space: c_int,
// @event_read_lock: Serializes drm_read().
    pub event_read_lock: mutex,
//
// @prime:
//
// Per-file buffer caches used by the PRIME buffer sharing code.
//
    pub prime: drm_prime_file_private,
//
// @client_name:
//
// Userspace-provided name; useful for accounting and debugging.
//
    pub client_name: *const c_char,
//
// @client_name_lock: Protects @client_name.
//
    pub client_name_lock: mutex,
//
// @debugfs_client:
//
// debugfs directory for each client under a drm node.
//
    pub debugfs_client: *mut dentry,
}

//
// drm_is_primary_client - is this an open file of the primary node
// @file_priv: DRM file
//
// Returns true if this is an open file of the primary node, i.e.
// &drm_file.minor of @file_priv is a primary minor.
//
// See also the :ref:`section on primary nodes and authentication
// <drm_primary_node>`.
//
// drm_is_render_client - is this an open file of the render node
// @file_priv: DRM file
//
// Returns true if this is an open file of the render node, i.e.
// &drm_file.minor of @file_priv is a render minor.
//
// See also the :ref:`section on render nodes <drm_render_node>`.
//
// drm_is_accel_client - is this an open file of the compute acceleration node
// @file_priv: DRM file
//
// Returns true if this is an open file of the compute acceleration node, i.e.
// &drm_file.minor of @file_priv is a accel minor.
//
// See also :doc:`Introduction to compute accelerators subsystem
// </accel/introduction>`.
//
extern "C" {
    pub fn drm_file_err(file_priv: *mut drm_file, fmt: *const c_char, ...);
}
extern "C" {
    pub fn drm_file_update_pid(: *mut drm_file);
}
extern "C" {
    pub fn drm_minor_release(minor: *mut drm_minor);
}
extern "C" {
    pub fn drm_open(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn drm_open_helper(filp: *mut file, minor: *mut drm_minor) -> c_int;
}
extern "C" {
    pub fn drm_release(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn drm_release_noglobal(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn drm_poll(filp: *mut file, wait: *mut poll_table_struct) -> __poll_t;
}
extern "C" {
    pub fn drm_send_event_locked(dev: *mut drm_device, e: *mut drm_pending_event);
}
extern "C" {
    pub fn drm_send_event(dev: *mut drm_device, e: *mut drm_pending_event);
}
//
// struct drm_memory_stats - GEM object stats associated
// @shared: Total size of GEM objects shared between processes
// @private: Total size of GEM objects
// @resident: Total size of GEM objects backing pages
// @purgeable: Total size of GEM objects that can be purged (resident and not active)
// @active: Total size of GEM objects active on one or more engines
//
// Used by drm_print_memory_stats()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_memory_stats {
    pub shared: u64,
    pub private: u64,
    pub resident: u64,
    pub purgeable: u64,
    pub active: u64,
}

extern "C" {
    pub fn drm_memory_stats_is_zero(stats: *const drm_memory_stats) -> c_int;
}
extern "C" {
    pub fn drm_show_memory_stats(p: *mut drm_printer, file: *mut drm_file);
}
extern "C" {
    pub fn drm_show_fdinfo(m: *mut seq_file, f: *mut file);
}
