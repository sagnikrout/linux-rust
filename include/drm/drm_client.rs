//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_client.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT

//
// struct drm_client_funcs - DRM client callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_client_funcs {
//
// @owner: The module owner
//
    pub owner: *mut module,
//
// @free:
//
// Called when the client gets unregistered. Implementations should
// release all client-specific data and free the memory.
//
// This callback is optional.
//
    pub client): *mut *mut void (free)(struct drm_client_dev,
//
// @unregister:
//
// Called when &drm_device is unregistered. The client should respond by
// releasing its resources using drm_client_release().
//
// This callback is optional.
//
    pub client): *mut *mut void (unregister)(struct drm_client_dev,
//
// @restore:
//
// Called on drm_lastclose(). The first client instance in the list that
// returns zero gets the privilege to restore and no more clients are
// called. This callback is not called after @unregister has been called.
//
// Note that the core does not guarantee exclusion against concurrent
// drm_open(). Clients need to ensure this themselves, for example by
// using drm_master_internal_acquire() and drm_master_internal_release().
//
// If the caller passes force, the client should ignore any present DRM
// master and restore the display anyway.
//
// This callback is optional.
//
    pub force): *mut *mut *mut int (restore)(struct drm_client_dev client, bool,
//
// @hotplug:
//
// Called on drm_kms_helper_hotplug_event().
// This callback is not called after @unregister has been called.
//
// This callback is optional.
//
    pub client): *mut *mut int (hotplug)(struct drm_client_dev,
//
// @suspend:
//
// Called when suspending the device.
//
// This callback is optional.
//
    pub client): *mut *mut int (suspend)(struct drm_client_dev,
//
// @resume:
//
// Called when resuming the device from suspend.
//
// This callback is optional.
//
    pub client): *mut *mut int (resume)(struct drm_client_dev,
}

//
// struct drm_client_dev - DRM client instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_client_dev {
//
// @dev: DRM device
//
    pub dev: *mut drm_device,
//
// @name: Name of the client.
//
    pub name: *const c_char,
//
// @list:
//
// List of all clients of a DRM device, linked into
// &drm_device.clientlist. Protected by &drm_device.clientlist_mutex.
//
    pub list: list_head,
//
// @funcs: DRM client functions (optional)
//
    pub funcs: *const drm_client_funcs,
//
// @file: DRM file
//
    pub file: *mut drm_file,
//
// @modeset_mutex: Protects @modesets.
//
    pub modeset_mutex: mutex,
//
// @modesets: CRTC configurations
//
    pub modesets: *mut drm_mode_set,
//
// @suspended:
//
// The client has been suspended.
//
    pub suspended: bool,
//
// @hotplug_pending:
//
// A hotplug event has been received while the client was suspended.
// Try again on resume.
//
    pub hotplug_pending: bool,
//
// @hotplug_failed:
//
// Set by client hotplug helpers if the hotplugging failed
// before. It is usually not tried again.
//
    pub hotplug_failed: bool,
}

extern "C" {
    pub fn drm_client_release(client: *mut drm_client_dev);
}
extern "C" {
    pub fn drm_client_register(client: *mut drm_client_dev);
}
//
// struct drm_client_buffer - DRM client buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_client_buffer {
//
// @client: DRM client
//
    pub client: *mut drm_client_dev,
//
// @gem: GEM object backing this buffer
//
// FIXME: The DRM framebuffer holds a reference on its GEM
// buffer objects. Do not use this field in new code and
// update existing users.
//
    pub gem: *mut drm_gem_object,
//
// @map: Virtual address for the buffer
//
    pub map: iosys_map,
//
// @fb: DRM framebuffer
//
    pub fb: *mut drm_framebuffer,
}

extern "C" {
    pub fn drm_client_buffer_delete(buffer: *mut drm_client_buffer);
}
extern "C" {
    pub fn drm_client_buffer_flush(buffer: *mut drm_client_buffer, rect: *mut drm_rect) -> c_int;
}
extern "C" {
    pub fn drm_client_buffer_vunmap_local(buffer: *mut drm_client_buffer);
}
extern "C" {
    pub fn drm_client_buffer_vunmap(buffer: *mut drm_client_buffer);
}
extern "C" {
    pub fn drm_client_modeset_create(client: *mut drm_client_dev) -> c_int;
}
extern "C" {
    pub fn drm_client_modeset_free(client: *mut drm_client_dev);
}
extern "C" {
    pub fn drm_client_modeset_probe(client: *mut drm_client_dev, width: c_uint, height: c_uint) -> c_int;
}
extern "C" {
    pub fn drm_client_rotation(modeset: *mut drm_mode_set, rotation: *mut c_uint) -> bool;
}
extern "C" {
    pub fn drm_client_modeset_check(client: *mut drm_client_dev) -> c_int;
}
extern "C" {
    pub fn drm_client_modeset_commit_locked(client: *mut drm_client_dev) -> c_int;
}
extern "C" {
    pub fn drm_client_modeset_commit(client: *mut drm_client_dev) -> c_int;
}
extern "C" {
    pub fn drm_client_modeset_dpms(client: *mut drm_client_dev, mode: c_int) -> c_int;
}
extern "C" {
    pub fn drm_client_modeset_wait_for_vblank(client: *mut drm_client_dev, crtc_index: c_uint) -> c_int;
}
//
// drm_client_for_each_modeset() - Iterate over client modesets
// @modeset: &drm_mode_set loop cursor
// @client: DRM client
//

//
// drm_client_for_each_connector_iter - connector_list iterator macro
// @connector: &struct drm_connector pointer used as cursor
// @iter: &struct drm_connector_list_iter
//
// This iterates the connectors that are useable for internal clients (excludes
// writeback connectors).
//
// For more info see drm_for_each_connector_iter().
//

